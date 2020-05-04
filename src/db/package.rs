use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::delete_repo_action_by_package;
use crate::db::models::{NewPackage, Package};

use super::schema;

#[throws]
pub fn create_package(conn: &PgConnection, package: &NewPackage) -> Package {
    use schema::package::dsl as p;
    diesel::insert_into(p::package)
        .values(package)
        .get_result(conn)?
}

#[throws]
pub fn get_package(conn: &PgConnection, id: i32) -> Package {
    use schema::package::dsl as p;
    p::package
        .filter(p::id.eq(id))
        .first(conn)?
}

#[throws]
pub fn get_packages_by_repo(conn: &PgConnection, repo_id: i32) -> Vec<Package> {
    use schema::package::dsl as p;
    p::package
        .filter(p::repo_id.eq(repo_id))
        .filter(p::deleted.eq(false))
        .load(conn)?
}

#[throws]
pub fn get_package_by_repo(conn: &PgConnection, repo_id: i32, name: &str, version: &str, arch: &str) -> Option<Package> {
    use schema::package::dsl as p;
    p::package
        .filter(p::repo_id.eq(repo_id))
        .filter(p::name.eq(name))
        .filter(p::version.eq(version))
        .filter(p::arch.eq(arch))
        .first(conn)
        .optional()?
}

#[throws]
pub fn get_packages_by_query(conn: &PgConnection, query: &str) -> Vec<Package> {
    use schema::package::dsl as p;
    p::package
        .filter(p::name.like(&format!("%{}%", query)))
        .filter(p::deleted.eq(false))
        .load(conn)?
}

#[throws]
pub fn remove_package(conn: &PgConnection, id: i32) {
    use schema::package::dsl as p;
    delete_repo_action_by_package(conn, id)?;
    diesel::delete(p::package)
        .filter(p::id.eq(id))
        .execute(conn)?;
}

#[throws]
pub fn set_package_deleted(conn: &PgConnection, id: i32, deleted: bool) {
    use schema::package::dsl as p;
    diesel::update(p::package)
        .filter(p::id.eq(id))
        .set(p::deleted.eq(deleted))
        .execute(conn)?;
}
