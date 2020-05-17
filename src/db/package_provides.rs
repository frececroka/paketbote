use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::{NewPackageProvides, PackageProvides};

use super::schema;

#[throws]
pub fn create_package_provides(conn: &PgConnection, package_id: i32, provides: String) {
    use schema::package_provides::dsl as pv;
    let package_provides = NewPackageProvides { package_id, provides };
    diesel::insert_into(pv::package_provides)
        .values(package_provides)
        .execute(conn)?;
}

#[throws]
pub fn get_package_provides(conn: &PgConnection, package_id: i32) -> Vec<PackageProvides> {
    use schema::package_provides::dsl as pv;
    pv::package_provides
        .filter(pv::package_id.eq(package_id))
        .load(conn)?
}

#[throws]
pub fn delete_package_provides(conn: &PgConnection, package_id: i32) {
    use schema::package_provides::dsl as pv;
    diesel::delete(pv::package_provides)
        .filter(pv::package_id.eq(package_id))
        .execute(conn)?;
}
