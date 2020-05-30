use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::NewRepo;
use crate::db::models::Repo;

use super::schema;

#[throws]
pub fn get_repo(conn: &PgConnection, id: i32) -> Repo {
    use schema::repo::dsl as r;
    r::repo
        .filter(r::id.eq(id))
        .first(conn)?
}

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

#[throws]
pub fn create_repo(conn: &PgConnection, repo: &NewRepo) -> Repo {
    use schema::repo::dsl as r;
    diesel::insert_into(r::repo)
        .values(repo)
        .get_result(conn)?
}

#[throws]
pub fn get_depends_by_repo(conn: &PgConnection, repo_id: i32) -> Vec<String> {
    use schema::repo::dsl as r;
    use schema::package::dsl as p;
    use schema::package_depends::dsl as pd;
    r::repo
        .inner_join(p::package.inner_join(pd::package_depends))
        .filter(r::id.eq(repo_id))
        .select(pd::depends)
        .distinct()
        .load(conn)?
}

#[throws]
pub fn get_provides_by_repo(conn: &PgConnection, repo_id: i32) -> Vec<(String, String)> {
    use schema::repo::dsl as r;
    use schema::package::dsl as p;
    use schema::package_provides::dsl as pp;
    r::repo
        .inner_join(p::package.inner_join(pp::package_provides))
        .filter(r::id.eq(repo_id))
        .select((pp::provides, p::version))
        .distinct()
        .load(conn)?
}
