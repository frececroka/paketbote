use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::AurVersion;
use crate::db::models::NewAurVersion;

use super::schema;

#[throws]
pub fn create_aur_version(conn: &PgConnection, package: String, version: String) {
    use schema::aur_version::dsl as av;
    let aur_version = NewAurVersion { package, version };
    diesel::insert_into(av::aur_version)
        .values(&aur_version)
        .on_conflict(av::package)
        .do_update()
        .set(av::version.eq(&aur_version.version))
        .execute(conn)?;
}

#[throws]
pub fn get_aur_version(conn: &PgConnection, package: &str) -> Option<String> {
    use schema::aur_version::dsl as av;
    av::aur_version
        .filter(av::package.eq(package))
        .first::<AurVersion>(conn)
        .optional()?
        .map(|av| av.version)
}
