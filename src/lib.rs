#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use fehler::throws;
use rocket::{Config, Rocket};

use crate::db::models::{Compression, Package};
use crate::error::Error;

pub mod error;
pub mod db;
pub mod web;

pub fn get_config() -> Config {
    Rocket::ignite().config().clone()
}

pub fn format_pkg_filename(package: &Package) -> String {
    format!("{}-{}-{}.pkg.tar.{}", package.name, package.version, package.arch, package.compression)
}

#[throws]
pub fn parse_pkg_filename(package: &str) -> (String, String, String, Compression) {
    // linux-mainline-5.7rc3-x86_64.tar.zst
    let parts: Vec<_> = package.rsplitn(4, "-").collect();
    if parts.len() != 4 { Err(Error)? }

    let name = parts[3].to_string();
    let version = format!("{}-{}", parts[2], parts[1]);

    // x86_64.tar.zst
    let parts: Vec<_> = parts[0].split(".").collect();
    if parts.len() != 3 { Err(Error)? }
    let arch = parts[0].to_string();
    let compression = parts[1].parse()
        .map_err(|_| Error)?;

    (name, version, arch, compression)
}
