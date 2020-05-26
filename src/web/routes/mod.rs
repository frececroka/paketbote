use diesel::PgConnection;
use fehler::throws;
use log::warn;
use sha3::Digest;
use sha3::Sha3_256;

use crate::db::get_account_by_name;
use crate::db::get_package_by_repo;
use crate::db::get_repo_by_account_and_name;
use crate::db::models::Account;
use crate::db::models::Package;
use crate::db::models::Repo;
use crate::parse_pkg_filename;
use crate::web::Error;
use crate::web::Error::*;

pub mod home;
pub mod create_account;
pub mod login;
pub mod logout;
pub mod account;
pub mod access_tokens;
pub mod repo;
pub mod package;
pub mod remove;
pub mod upload;
pub mod search;

fn hash_password(salt: &str, password: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(salt);
    hasher.input(password);
    base64::encode(hasher.result())
}

#[throws]
fn validate_access(active_account: Account, claimed_account: String) -> Account {
    if active_account.name == claimed_account {
        active_account
    } else {
        warn!("The principal {} does not match the account {} provided in the URL.",
            active_account.name, claimed_account);
        Err(Unauthorized)?
    }
}

#[throws]
fn load_account(db: &PgConnection, account: &str) -> Account {
    get_account_by_name(db, account)?
        .ok_or(NotFound)?
}

#[throws]
fn load_repo(db: &PgConnection, account_id: i32, repo: &str) -> Repo {
    get_repo_by_account_and_name(db, account_id, repo)?
        .ok_or(NotFound)?
}

#[throws]
fn load_package(conn: &PgConnection, repo_id: i32, package: &str) -> Package {
    let (name, version, arch, _) = parse_pkg_filename(package)
        .map_err(|_| NotFound)?;
    get_package_by_repo(conn, repo_id, &name, &version, &arch)?
        .ok_or(NotFound)?
}

fn create_random_token() -> String {
    base64::encode(rand::random::<[u8; 20]>())
}
