use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::{NewRepoAdd, RepoAdd};

use super::schema;

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

#[throws]
pub fn delete_repo_add(conn: &PgConnection, id: i32) {
    use schema::repo_add::dsl as ra;
    diesel::delete(ra::repo_add)
        .filter(ra::id.eq(id))
        .execute(conn)?;
}
