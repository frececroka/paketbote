use chrono::{DateTime, Utc};
use colored::*;
use diesel::PgConnection;
use fehler::throws;
use prettytable::{Cell, Row, Table};
use prettytable::format::consts::FORMAT_CLEAN;
use rocket::http::Status;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db;
use crate::db::{get_account_by_name, get_packages_by_repo, get_repo_by_account_and_name};
use crate::db::models::{Account, Repo};
use crate::web::ctx_base::BaseContext;
use crate::web::db::Db;
use crate::web::props::Props;

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
    packages: Vec<Package>
}

#[derive(Serialize)]
struct Package {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub arch: String,
    pub size: i32,
    pub archive: String,
    pub signature: String,
    pub created: String,
    pub repo_id: i32
}

impl From<db::models::Package> for Package {
    fn from(package: db::models::Package) -> Self {
        let created = DateTime::<Utc>::from_utc(package.created, Utc);
        let created_fmt = created
            .format("%Y-%m-%d")
            .to_string();
        let archive_file = format!("{}-{}-{}.pkg.tar.xz",
            package.name, package.version, package.arch);
        let signature_file = format!("{}.sig", archive_file);
        Package {
            id: package.id,
            name: package.name,
            version: package.version,
            arch: package.arch,
            size: package.size,
            archive: archive_file,
            signature: signature_file,
            created: created_fmt,
            repo_id: package.repo_id,
        }
    }
}

impl RepoContext {
    fn new(props: &Props, account: Account, repo: Repo, packages: Vec<Package>) -> RepoContext {
        let base = BaseContext::new(&props.account);
        RepoContext { base, account, repo, packages }
    }
}

#[throws(Status)]
#[get("/<account>/<repo>", format = "text/html", rank = 4)]
pub fn route_repo_html(props: Props, account: String, repo: String) -> Template {
    let (account, repo, packages) = get_packages(&*props.db, &account, &repo)?;
    let context = RepoContext::new(&props, account, repo, packages);
    Template::render("repo", context)
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
