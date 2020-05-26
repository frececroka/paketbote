use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;

use crate::db;
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
    pub repo_id: i32
}

impl From<db::models::Package> for Package {
    fn from(package: db::models::Package) -> Self {
        let created = DateTime::<Utc>::from_utc(package.created, Utc);
        let created_fmt = created
            .format("%Y-%m-%d")
            .to_string();
        let archive_file = format_pkg_filename(&package);
        let signature_file = format!("{}.sig", archive_file);
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
            repo_id: package.repo_id,
        }
    }
}

