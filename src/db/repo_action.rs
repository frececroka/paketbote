use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::{NewRepoAction, RepoAction, RepoActionOp};

use super::schema;

#[throws]
pub fn create_repo_action(conn: &PgConnection, package_id: i32, action: RepoActionOp) {
    use schema::repo_action::dsl as ra;
    diesel::insert_into(ra::repo_action)
        .values(NewRepoAction { package_id, action })
        .execute(conn)?;
}

#[throws]
pub fn get_repo_action(conn: &PgConnection) -> Option<RepoAction> {
    use schema::repo_action::dsl as ra;
    ra::repo_action
        .filter(ra::worker.is_null())
        .order_by(ra::id.asc())
        .first(conn)
        .optional()?
}

#[throws]
pub fn delete_repo_action(conn: &PgConnection, id: i32) {
    use schema::repo_action::dsl as ra;
    diesel::delete(ra::repo_action)
        .filter(ra::id.eq(id))
        .execute(conn)?;
}

#[throws]
pub fn delete_repo_action_by_package(conn: &PgConnection, package_id: i32) {
    use schema::repo_action::dsl as ra;
    diesel::delete(ra::repo_action)
        .filter(ra::package_id.eq(package_id))
        .execute(conn)?;
}
