use fehler::throws;
use log::warn;
use rocket::http::Status;
use rocket::response::Redirect;

use crate::db::{create_repo_action, get_package_by_repo, get_repo_by_account_and_name, set_package_deleted};
use crate::db::models::Account;
use crate::parse_pkg_filename;
use crate::web::db::Db;

#[throws(Status)]
#[delete("/<account>/<repo>/<package>")]
pub fn route_remove(
    db: Db,
    active_account: Account,
    account: String,
    repo: String,
    package: String,
) -> Redirect
{
    let account = if account == active_account.name {
        active_account
    } else {
        warn!("The principal {} does not match the account {} provided in the URL.", active_account.name, account);
        Err(Status::Unauthorized)?
    };

    let repo = get_repo_by_account_and_name(&*db, account.id, &repo)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    let (name, version, arch, _) = parse_pkg_filename(&package)
        .map_err(|_| Status::BadRequest)?;
    let package = get_package_by_repo(&*db, repo.id, &name, &version, &arch)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    set_package_deleted(&*db, package.id, true)
        .map_err(|_| Status::InternalServerError)?;
    create_repo_action(&*db, package.id, "remove".to_string())
        .map_err(|_| Status::InternalServerError)?;

    Redirect::to(format!("/{}/{}", account.name, repo.name))
}
