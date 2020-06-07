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
    pub aur_package: Option<AurPackage>,
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
    let aur_package = load_aur_package(conn, &package)?;
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
        aur_package: aur_package,
        repo_id: package.repo_id,
    }
}

#[throws]
fn load_aur_package(conn: &PgConnection, package: &db::models::Package) -> Option<AurPackage> {
    if let Some(version) = get_aur_version(conn, &package.name)? {
        let is_newer = vercmp(&version, &package.version) == Ordering::Greater;
        Some(AurPackage::new(&package.name, &version, is_newer))
    } else {
        None
    }
}

#[derive(Debug, Serialize)]
pub struct AurPackage {
    name: String,
    version: String,
    is_newer: bool,
    url: String
}

impl AurPackage {
    fn new(name: &str, version: &str, is_newer: bool) -> AurPackage {
        let name = name.to_owned();
        let version = version.to_owned();
        let url = format!("https://aur.archlinux.org/packages/{}", name);
        AurPackage { name, version, is_newer, url }
    }
}
