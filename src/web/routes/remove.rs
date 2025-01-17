use fehler::throws;
use rocket::response::Redirect;

use crate::db::get_package_by_repo;
use crate::db::get_repo_by_account_and_name;
use crate::db::models::Account;
use crate::db::set_package_deleted;
use crate::jobs::create_repo_action;
use crate::jobs::RepoActionOp;
use crate::parse_pkg_filename;
use crate::web::db::Db;
use crate::web::Error;
use crate::web::Error::*;
use crate::web::routes::validate_access;

#[throws]
#[delete("/<account>/<repo>/<package>")]
pub fn route_remove(
    db: Db,
    active_account: Account,
    account: String,
    repo: String,
    package: String,
) -> Redirect
{
    let account = validate_access(active_account, account)?;

    let repo = get_repo_by_account_and_name(&*db, account.id, &repo)?
        .ok_or(NotFound)?;

    let (name, version, arch, _) = parse_pkg_filename(&package)
        .map_err(|_| NotFound)?;
    let package = get_package_by_repo(&*db, repo.id, &name, &version, &arch)?
        .ok_or(NotFound)?;

    set_package_deleted(&*db, package.id, true)?;
    create_repo_action(&*db, package.id, RepoActionOp::Remove)?;

    Redirect::to(format!("/{}/{}", account.name, repo.name))
}
