use std::cmp::Ordering;

use alpm::vercmp;
use chrono::DateTime;
use chrono::Utc;
use diesel::PgConnection;
use fehler::throws;
use serde::Serialize;

use crate::db;
use crate::db::get_aur_version;
use crate::error::Error;
use crate::format_pkg_filename;

#[derive(Debug, Serialize)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub arch: String,
    pub size: i32,
    pub archive: String,
    pub signature: String,
    pub created: String,
    pub active: bool,
    pub aur_version: Option<String>,
    pub repo_id: i32
}

#[throws]
pub fn augment_package(conn: &PgConnection, package: db::models::Package) -> Package {
    let created = DateTime::<Utc>::from_utc(package.created, Utc);
    let created_fmt = created
        .format("%Y-%m-%d")
        .to_string();
    let archive_file = format_pkg_filename(&package);
    let signature_file = format!("{}.sig", archive_file);
    let aur_version = if package.active {
        get_aur_version(conn, &package.name)?
            .filter(|v| vercmp(v, &package.version) == Ordering::Greater)
    } else { None };
    Package {
        id: package.id,
        name: package.name,
        version: package.version,
        arch: package.arch,
        size: package.size,
        archive: archive_file,
        signature: signature_file,
        created: created_fmt,
        active: package.active,
        aur_version: aur_version,
        repo_id: package.repo_id,
    }
}
