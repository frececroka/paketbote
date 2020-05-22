use colored::*;
use diesel::PgConnection;
use fehler::throws;
use prettytable::{Cell, Row, Table};
use prettytable::format::consts::FORMAT_CLEAN;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db::{create_repo, create_repo_action, get_account_by_name, get_all_packages_by_repo, get_packages_by_repo, get_repo_by_account_and_name, Paginated, set_package_deleted};
use crate::db::ExpectConflict;
use crate::db::models::{Account, NewRepo, Repo, RepoActionOp};
use crate::obsolete::determine_obsolete;
use crate::web::ctx_base::BaseContext;
use crate::web::db::Db;
use crate::web::props::Props;
use crate::web::routes::{Package, validate_access};

#[throws(Status)]
#[get("/<account>/<repo>", format = "text/plain", rank = 5)]
pub fn route_repo_text(db: Db, account: String, repo: String) -> String {
    let (_, _, packages) = get_packages(&*db, &account, &repo, 0)?;

    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);

    for package in packages.items {
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
    packages: Paginated<Package>,
    pages: Vec<usize>,
    can_edit: bool
}

impl RepoContext {
    fn new(props: &Props, account: Account, repo: Repo, packages: Paginated<Package>) -> RepoContext {
        let base = BaseContext::new(&props.account);
        let pages = (0..packages.total_pages).collect();
        let can_edit = if let Some(active_account) = &props.account {
            active_account.name == account.name
        } else { false };
        RepoContext { base, account, repo, packages, pages, can_edit }
    }
}

#[throws(Status)]
#[get("/<account>/<repo>?<p>", format = "text/html", rank = 4)]
pub fn route_repo_html(props: Props, account: String, repo: String, p: Option<usize>) -> Template {
    let (account, repo, packages) = get_packages(&*props.db, &account, &repo, p.unwrap_or(0))?;
    let context = RepoContext::new(&props, account, repo, packages);
    Template::render("repo", context)
}

#[throws(Status)]
fn get_packages(db: &PgConnection, account: &str, repo: &str, page: usize) -> (Account, Repo, Paginated<Package>) {
    let account = load_account(db, account)?;
    let repo = load_repo(db, repo, &account)?;
    let mut packages = get_packages_by_repo(db, repo.id, page)
        .map_err(|_| Status::InternalServerError)?;
    packages.items.sort_by_key(|p| p.name.clone());
    let packages = packages.map(|p| p.into());
    (account, repo, packages)
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
    let account = load_account(&*props.db, &account)?;
    let repo = load_repo(&*props.db, &repo, &account)?;
    let packages = get_all_packages_by_repo(&*props.db, repo.id)
        .map_err(|_| Status::InternalServerError)?;
    let packages: Vec<&crate::db::models::Package> = packages.iter().collect();
    let obsoletes = determine_obsolete(packages)
        .map_err(|_| Status::InternalServerError)?;

    for obsolete in obsoletes {
        set_package_deleted(&*props.db, obsolete.id, true)
            .map_err(|_| Status::InternalServerError)?;
        create_repo_action(&*props.db, obsolete.id, RepoActionOp::Remove)
            .map_err(|_| Status::InternalServerError)?;
    }

    Redirect::to(format!("/{}/{}", account.name, repo.name))
}

#[throws(Status)]
fn load_account(db: &PgConnection, account: &str) -> Account {
    get_account_by_name(db, account)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?
}

#[throws(Status)]
fn load_repo(db: &PgConnection, repo: &str, account: &Account) -> Repo {
    get_repo_by_account_and_name(db, account.id, repo)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?
}
