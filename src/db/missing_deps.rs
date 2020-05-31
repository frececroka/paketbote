use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::NewMissingDep;

use super::schema;

#[throws]
pub fn replace_missing_deps(conn: &PgConnection, repo_id: i32, missing_deps: Vec<String>) {
    use schema::missing_dep::dsl as md;
    let missing_deps = missing_deps.into_iter()
        .map(|dependency| NewMissingDep { repo_id, dependency })
        .collect::<Vec<_>>();
    diesel::delete(md::missing_dep)
        .filter(md::repo_id.eq(repo_id))
        .execute(conn)?;
    diesel::insert_into(md::missing_dep)
        .values(missing_deps)
        .execute(conn)?;
}

#[throws]
pub fn get_missing_deps(conn: &PgConnection, repo_id: i32) -> Vec<String> {
    use schema::missing_dep::dsl as md;
    md::missing_dep
        .filter(md::repo_id.eq(repo_id))
        .select(md::dependency)
        .load(conn)?
}
