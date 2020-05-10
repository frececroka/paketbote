use std::cmp::Ordering;
use std::panic;
use std::process::Command;

use colored::*;
use diesel::PgConnection;
use fehler::throws;
use itertools::Itertools;
use prettytable::{Cell, Row, Table};
use prettytable::format::consts::FORMAT_CLEAN;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db::{create_repo, get_account_by_name, get_packages_by_repo, get_repo_by_account_and_name, remove_package};
use crate::db::ExpectConflict;
use crate::db::models::{Account, NewRepo, Repo};
use crate::error::Error;
use crate::web::ctx_base::BaseContext;
use crate::web::db::Db;
use crate::web::props::Props;
use crate::web::routes::{Package, validate_access};

#[throws(Status)]
#[get("/<account>/<repo>", format = "text/plain", rank = 5)]
pub fn route_repo_text(db: Db, account: String, repo: String) -> String {
    let (_, _, packages) = get_packages(&*db, &account, &repo)?;

    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);

    for package in packages {
        table.add_row(Row::new(vec![
            Cell::new(&package.name),
            Cell::new(&package.version.bright_blue().italic().to_string())]));
    }
    table.to_string()
}

#[derive(Serialize)]
struct RepoContext {
    base: BaseContext,
    account: Account,
    repo: Repo,
    packages: Vec<Package>,
    can_edit: bool
}

impl RepoContext {
    fn new(props: &Props, account: Account, repo: Repo, packages: Vec<Package>) -> RepoContext {
        let base = BaseContext::new(&props.account);
        let can_edit = if let Some(active_account) = &props.account {
            active_account.name == account.name
        } else { false };
        RepoContext { base, account, repo, packages, can_edit }
    }
}

#[throws(Status)]
#[get("/<account>/<repo>", format = "text/html", rank = 4)]
pub fn route_repo_html(props: Props, account: String, repo: String) -> Template {
    let (account, repo, packages) = get_packages(&*props.db, &account, &repo)?;
    let context = RepoContext::new(&props, account, repo, packages);
    Template::render("repo", context)
}

#[derive(FromForm)]
pub struct CreateRepo {
    name: String
}

#[throws(Status)]
#[post("/<account>", data = "<data>")]
pub fn route_repo_create(
    db: Db,
    active_account: Account,
    account: String,
    data: Form<CreateRepo>,
) -> Redirect
{
    let account = validate_access(active_account, account)?;

    if data.name.len() == 0 {
        Err(Status::BadRequest)?
    }

    let repo = NewRepo { name: data.name.clone(), owner_id: account.id };
    let repo = create_repo(&*db, &repo)
        .expect_conflict()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::Conflict)?;

    Redirect::to(format!("/{}/{}", account.name, repo.name))
}

#[throws(Status)]
#[post("/<account>/<repo>/delete-obsolete", rank = 4)]
pub fn route_delete_obsolete(props: Props, account: String, repo: String) -> Redirect {
    let (account, repo, mut packages) = get_packages(&*props.db, &account, &repo)?;
    packages.sort_by_key(|p| p.name.clone());
    let groups = packages.into_iter()
        .group_by(|p| p.name.clone()).into_iter()
        .map(|(_, g)| g.collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for group in groups {
        let group = group.iter().collect::<Vec<_>>();
        let obsolete = determine_obsolete(group)
            .map_err(|_| Status::InternalServerError)?;
        for package in obsolete {
            remove_package(&*props.db, package.id)
                .map_err(|_| Status::InternalServerError)?;
            create_repo_action(&*props.db, package.id, "remove".to_string())
                .map_err(|_| Status::InternalServerError)?;
        }
    }
    Redirect::to(format!("/{}/{}", account.name, repo.name))
}

#[throws]
fn determine_obsolete(packages: Vec<&Package>) -> Vec<&Package> {
    let packages = sort_by_version(packages)?;
    packages.into_iter()
        .skip_while(|p| !p.active)
        .skip(1)
        .collect()
}

#[throws]
fn sort_by_version(mut packages: Vec<&Package>) -> Vec<&Package> {
    panic::set_hook(Box::new(|_| {}));
    panic::catch_unwind(move || {
        packages.sort_by(|p, q|
            package_vercmp(&p.version, &q.version)
                .unwrap().reverse());
        packages
    }).map_err(|_| Error)?
}

#[throws]
fn package_vercmp(v: &str, w: &str) -> Ordering {
    let output = Command::new("vercmp")
        .arg(v).arg(w)
        .output()?;
    if !output.status.success() {
        Err(Error)?
    }
    let result: i32 = String::from_utf8(output.stdout)
        .map_err(|_| Error)?
        .trim().parse()
        .map_err(|_| Error)?;
    if result < 0 {
        Ordering::Less
    } else if result == 0 {
        Ordering::Equal
    } else if result > 0 {
        Ordering::Greater
    } else {
        unreachable!()
    }
}

#[throws(Status)]
fn get_packages(db: &PgConnection, account: &str, repo: &str) -> (Account, Repo, Vec<Package>) {
    let account = get_account_by_name(&*db, account)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
    let repo = get_repo_by_account_and_name(&*db, account.id, repo)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
    let mut packages = get_packages_by_repo(&*db, repo.id)
        .map_err(|_| Status::InternalServerError)?;
    packages.sort_by_key(|p| p.name.clone());
    let packages = packages.into_iter()
        .map(|p| p.into())
        .collect();
    (account, repo, packages)
}
