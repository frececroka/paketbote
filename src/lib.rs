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

use fehler::throws;
use multipart::server::{Multipart, MultipartData};
use rocket::{Config, Rocket};
use rocket::data::DataStream;

use crate::db::models::{Compression, Package};
use crate::error::Error;

pub mod error;
pub mod db;
pub mod web;
pub mod pkginfo;
pub mod obsolete;

pub fn get_config() -> Config {
    Rocket::ignite().config().clone()
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
