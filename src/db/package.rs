use diesel::PgConnection;
use diesel::prelude::*;
use fehler::throws;
use diesel::result::Error;

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

