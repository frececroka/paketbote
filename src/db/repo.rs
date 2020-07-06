use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::Repo;

use super::schema;

#[throws]
pub fn get_repos_by_account(conn: &PgConnection, account_id: i32) -> Vec<Repo> {
    use schema::repo::dsl as r;
    r::repo
        .filter(r::owner_id.eq(account_id))
        .load(conn)?
}

#[throws]
pub fn get_repo_by_account_and_name(conn: &PgConnection, account_id: i32, name: &str) -> Option<Repo> {
    use schema::repo::dsl as r;
    r::repo
        .filter(r::owner_id.eq(account_id))
        .filter(r::name.eq(name))
        .first(conn)
        .optional()?
}

