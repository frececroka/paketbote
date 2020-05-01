use fehler::throws;
use prettytable::{Cell, Row, Table};
use prettytable::format::consts::FORMAT_CLEAN;
use rocket::http::Status;
use colored::*;
use crate::db::{get_account_by_name, get_packages_by_repo, get_repo_by_account_and_name};
use crate::web::db::Db;

#[throws(Status)]
#[get("/<account>/<repo>")]
pub fn list(db: Db, account: String, repo: String) -> String {
    let account = get_account_by_name(&*db, &account)
        .map_err(|_| Status::NotFound)?;
    let repo = get_repo_by_account_and_name(&*db, account.id, &repo)
        .map_err(|_| Status::NotFound)?;
    let mut packages = get_packages_by_repo(&*db, repo.id)
        .map_err(|_| Status::InternalServerError)?;
    packages.sort_by_key(|p| p.name.clone());

    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);

    for package in packages {
        table.add_row(Row::new(vec![
            Cell::new(&package.name),
            Cell::new(&package.version.bright_blue().italic().to_string())]));
    }
    table.to_string()
}
