use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::{NewPackageDepends, PackageDepends};

use super::schema;

#[throws]
pub fn create_package_depends(conn: &PgConnection, package_id: i32, depends: String) {
    use schema::package_depends::dsl as pv;
    let package_depends = NewPackageDepends { package_id, depends };
    diesel::insert_into(pv::package_depends)
        .values(package_depends)
        .execute(conn)?;
}

#[throws]
pub fn get_package_depends(conn: &PgConnection, package_id: i32) -> Vec<PackageDepends> {
    use schema::package_depends::dsl as pv;
    pv::package_depends
        .filter(pv::package_id.eq(package_id))
        .load(conn)?
}

#[throws]
pub fn delete_package_depends(conn: &PgConnection, package_id: i32) {
    use schema::package_depends::dsl as pv;
    diesel::delete(pv::package_depends)
        .filter(pv::package_id.eq(package_id))
        .execute(conn)?;
}
