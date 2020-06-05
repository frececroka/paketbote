#![feature(proc_macro_hygiene, decl_macro)]
#![feature(backtrace)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::fs::File;
use std::path::PathBuf;

use diesel::Connection;
use diesel::PgConnection;
use fehler::throws;
use multipart::server::Multipart;
use multipart::server::MultipartData;
use rocket::Config;
use rocket::data::DataStream;
use rocket::Rocket;

use crate::db::models::{Compression, Package};
use crate::error::Error;

pub mod alpm;
pub mod aur;
pub mod db;
pub mod error;
pub mod jobs;
pub mod missing;
pub mod obsolete;
pub mod pkginfo;
pub mod spec;
pub mod web;

pub fn get_config() -> Config {
    Rocket::ignite().config().clone()
}

#[throws(diesel::result::ConnectionError)]
pub fn connect_db() -> PgConnection {
    let config = get_config();
    let database = config
        .get_table("databases").unwrap()
        .get("main").unwrap()
        .get("url").unwrap()
        .as_str().unwrap();
    PgConnection::establish(database)?
}

pub fn format_pkg_filename(package: &Package) -> String {
    format!("{}-{}-{}.pkg.tar.{}", package.name, package.version, package.arch, package.compression)
}

#[throws]
pub fn parse_pkg_filename(package: &str) -> (String, String, String, Compression) {
    // linux-mainline-5.7rc3-1-x86_64.tar.zst
    let parts: Vec<_> = package.rsplitn(2, "-").collect();
    if parts.len() != 2 {
        Err(format!("Cannot parse string {} as package name.", package))?
    }

    let (name, version) = parse_pkg_name(parts[1])?;

    // x86_64.pkg.tar.zst
    let parts: Vec<_> = parts[0].split(".").collect();
    if parts.len() != 4 {
        Err(format!("Cannot parse string {} as package name.", package))?
    }

    let arch = parts[0].to_string();
    let compression = parts[3].parse()?;

    (name, version, arch, compression)
}

#[throws]
pub fn parse_pkg_name(package: &str) -> (String, String) {
    let parts: Vec<_> = package.rsplitn(3, "-").collect();
    if parts.len() != 3 {
        Err(format!("Cannot parse string {} as package name.", package))?
    }

    let name = parts[2].to_string();
    let version = format!("{}-{}", parts[1], parts[0]);
    (name, version)
}

#[throws]
fn serve_db(repo_id: u32) -> File {
    let filename = format!("{}.db.tar.gz", repo_id);
    let path = PathBuf::new()
        .join("repos")
        .join(filename);
    File::open(path)?
}

#[throws]
fn serve_archive(archive: &str) -> File {
    let path = PathBuf::new()
        .join("packages")
        .join(archive);
    File::open(path)?
}

#[throws]
fn save_archive(filename: &str, mut data: MultipartData<&mut Multipart<DataStream>>) -> u64 {
    let path = PathBuf::new()
        .join("packages")
        .join(filename);
    data.save()
        .size_limit(1024 * 1024 * 1024)
        .write_to(File::create(path)?)
        .into_result()?
}
