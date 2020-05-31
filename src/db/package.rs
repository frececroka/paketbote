use diesel::{PgConnection, sql_query};
use diesel::dsl::sum;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_types::Integer;
use fehler::throws;

use crate::db::models::NewPackage;
use crate::db::models::Package;
use crate::db::package_depends::delete_package_depends;
use crate::db::package_provides::delete_package_provides;
use crate::db::Paginated;

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
pub fn get_packages(conn: &PgConnection) -> Vec<Package> {
    use schema::package::dsl as p;
    p::package.load(conn)?
}

#[throws]
pub fn get_all_packages_by_repo(conn: &PgConnection, repo_id: i32) -> Vec<Package> {
    use schema::package::dsl as p;
    p::package
        .filter(p::repo_id.eq(repo_id))
        .filter(p::deleted.eq(false))
        .load(conn)?
}

#[throws]
pub fn get_packages_by_repo(conn: &PgConnection, repo_id: i32, page: usize) -> Paginated<Package> {
    use schema::package::dsl as p;
    let query = p::package
        .filter(p::repo_id.eq(repo_id))
        .filter(p::deleted.eq(false));
    let total_items = query.count().first::<i64>(conn)? as usize;
    let limit = 100;
    let offset = page * limit;
    let items = query
        .order_by(p::name.asc())
        .then_order_by(p::id.asc())
        .offset(offset as i64)
        .limit(limit as i64)
        .load(conn)?;
    Paginated::new(items, total_items, page, limit)
}

#[throws]
pub fn get_package_count_by_repo(conn: &PgConnection, repo_id: i32) -> usize {
    use schema::package::dsl as p;
    p::package
        .filter(p::repo_id.eq(repo_id))
        .filter(p::deleted.eq(false))
        .count().first::<i64>(conn)? as usize
}

#[throws]
pub fn get_total_package_size_by_repo(conn: &PgConnection, repo_id: i32) -> usize {
    use schema::package::dsl as p;
    p::package
        .filter(p::repo_id.eq(repo_id))
        .filter(p::deleted.eq(false))
        .select(sum(p::size))
        .first::<Option<i64>>(conn)?
        .unwrap_or(0) as usize
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
        .filter(p::active.eq(true))
        .load(conn)?
}

#[throws]
pub fn remove_package(conn: &PgConnection, id: i32) {
    use schema::package::dsl as p;
    delete_package_depends(conn, id)?;
    delete_package_provides(conn, id)?;
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

#[throws]
pub fn set_package_active(conn: &PgConnection, id: i32) {
    let query = "\
        Update package As p \
        Set active = (p.id = q.id) \
        From package as q \
        Where \
              q.id = $1 And \
              p.repo_id = q.repo_id And \
              p.name = q.name";
    sql_query(query)
        .bind::<Integer, _>(id)
        .execute(conn)?;
}
