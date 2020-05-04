use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sha3::{Digest, Sha3_256};

use crate::{db, format_pkg_filename};

pub mod home;
pub mod create_account;
pub mod login;
pub mod logout;
pub mod account;
pub mod access_tokens;
pub mod repo;
pub mod getfile;
pub mod remove;
pub mod upload;
pub mod search;

fn no_context() -> HashMap<String, String> {
    HashMap::new()
}

fn hash_password(salt: &str, password: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(salt);
    hasher.input(password);
    base64::encode(hasher.result())
}

#[derive(Debug, Serialize)]
struct Package {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub arch: String,
    pub size: i32,
    pub archive: String,
    pub signature: String,
    pub created: String,
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
            repo_id: package.repo_id,
        }
    }
}
