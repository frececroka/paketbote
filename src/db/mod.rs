use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use models::*;

use crate::db::models::Token;

mod schema;
pub mod models;

#[throws]
pub fn get_account(conn: &PgConnection, account_id: i32) -> Account {
    use schema::account::dsl as a;
    a::account
        .filter(a::id.eq(account_id))
        .first(conn)?
}

#[throws]
pub fn get_account_by_name(conn: &PgConnection, name: &str) -> Account {
    use schema::account::dsl as a;
    a::account
        .filter(a::name.eq(name))
        .first(conn)?
}

#[throws]
pub fn get_account_for_token(conn: &PgConnection, token: &str) -> Account {
    use schema::token::dsl as t;
    let token: Token = t::token
        .filter(t::the_token.eq(token))
        .first(conn)?;
    get_account(conn, token.account_id)?
}

#[throws]
pub fn get_repo_by_account_and_name(conn: &PgConnection, account_id: i32, name: &str) -> Repo {
    use schema::repo::dsl as r;
    r::repo
        .filter(r::owner_id.eq(account_id))
        .filter(r::name.eq(name))
        .first(conn)?
}

#[throws]
pub fn create_package(conn: &PgConnection, package: &NewPackage) -> Package {
    use schema::package::dsl as p;
    diesel::insert_into(p::package)
        .values(package)
        .get_result(conn)?
}

#[throws]
pub fn get_package_by_repo(conn: &PgConnection, repo_id: i32, name: &str, version: &str, arch: &str) -> Package {
    use schema::package::dsl as p;
    p::package
        .filter(p::repo_id.eq(repo_id))
        .filter(p::name.eq(name))
        .filter(p::version.eq(version))
        .filter(p::arch.eq(arch))
        .first(conn)?
}

#[throws]
pub fn create_repo_add(conn: &PgConnection, package_id: i32) {
    use schema::repo_add::dsl as ra;
    diesel::insert_into(ra::repo_add)
        .values(NewRepoAdd { package_id })
        .execute(conn)?;
}

#[throws]
pub fn get_repo_add(conn: &PgConnection) -> RepoAdd {
    use schema::repo_add::dsl as ra;
    ra::repo_add
        .filter(ra::worker.is_null())
        .first(conn)?
}
