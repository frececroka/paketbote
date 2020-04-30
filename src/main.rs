#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use fehler::throws;
use log::{info, warn};
use multipart::server::{Multipart, MultipartData};
use rocket::{Data, Request};
use rocket::data::DataStream;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use tar::Archive;
use uuid::Uuid;
use xz2::read::XzDecoder;

use error::Error;

use crate::db::{create_package, get_account_for_token, get_repo_by_account_and_name};
use crate::db::models::NewPackage;

mod error;
mod db;

#[database("main")]
struct Db(diesel::PgConnection);

#[derive(Debug)]
struct Boundary(String);

impl<'a, 'r> FromRequest<'a, 'r> for Boundary {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let ct = request.headers().get_one("Content-Type").expect("no content-type");
        let idx = ct.find("boundary=").expect("no boundary");
        Outcome::Success(Boundary(ct[(idx + "boundary=".len())..].to_string()))
    }
}

#[derive(Debug)]
struct Principal(db::models::Account);

impl<'a, 'r> FromRequest<'a, 'r> for Principal {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // We need an authorization header.
        let authorization = if let Some(authorization) = request.headers().get_one("Authorization") {
            authorization
        } else {
            return Outcome::Failure((Status::Unauthorized, ()));
        };

        // We need a bearer token.
        let token = if authorization.starts_with("Bearer ") {
            &authorization["Bearer ".len()..]
        } else {
            return Outcome::Failure((Status::Unauthorized, ()));
        };

        // The token must belong to a user account.
        let db = Db::from_request(request).unwrap();
        if let Ok(account) = get_account_for_token(&*db, token) {
            Outcome::Success(Principal(account))
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

#[throws(Status)]
#[post("/<account>/<repo>", data = "<data>")]
fn upload(db: Db, principal: Principal, account: String, repo: String, boundary: Boundary, data: Data) {
    let account = if account == principal.0.name {
        principal.0
    } else {
        warn!("The principal {} does not match the account {} provided in the URL.", principal.0.name, account);
        Err(Status::Unauthorized)?
    };

    let repo = get_repo_by_account_and_name(&*db, account.id, &repo)
        .map_err(|_| Status::NotFound)?;

    info!("Saving uploaded files to disk...");
    let ((package_file, package_size), (signature_file, signature_size)) =
        save_uploaded_files(data, &boundary.0)?;
    info!("Received package of size {} and signature of size {}.",
          package_size, signature_size);

    let total_size: i32 = (package_size + signature_size)
        .try_into().map_err(|_| Status::BadRequest)?;
    info!("The total size of uploaded files is {}.", total_size);

    info!("Loading PKGINFO from package...");
    let pkginfo = load_pkginfo(&package_file)?;
    let pkgname = pkginfo.get("pkgname").ok_or(Status::BadRequest)?;
    let pkgver = pkginfo.get("pkgver").ok_or(Status::BadRequest)?;
    info!("Package has name {} and version {}", pkgname, pkgver);

    let package = NewPackage {
        name: pkgname.to_string(),
        version: pkgver.to_string(),
        size: total_size,
        archive: package_file,
        signature: signature_file,
        repo_id: repo.id,
    };

    info!("Adding package to database: {:?}", package);
    create_package(&*db, &package)
        .map_err(|_| Status::InternalServerError)?;
}

#[throws(Status)]
fn save_uploaded_files(data: Data, boundary: &str) -> ((String, u64), (String, u64)) {
    let mut package: Option<(String, u64)> = None;
    let mut signature: Option<(String, u64)> = None;

    let mut multipart = Multipart::with_body(data.open(), boundary);
    multipart.foreach_entry(|entry| {
        let name = entry.headers.name.borrow();
        let target = match name {
            "package" => &mut package,
            "signature" => &mut signature,
            _ => return
        };
        let filename = Uuid::new_v4().to_string();
        let filesize = save_file(&filename, entry.data)
            .expect("failed to save uploaded file");
        *target = Some((filename, filesize));
    }).unwrap();

    (package.ok_or(Status::BadRequest)?, signature.ok_or(Status::BadRequest)?)
}

#[throws]
fn save_file(filename: &str, mut data: MultipartData<&mut Multipart<DataStream>>) -> u64 {
    let path = PathBuf::new()
        .join("packages")
        .join(filename);
    data.save()
        .size_limit(1024 * 1024 * 1024)
        .write_to(File::create(path)?)
        .into_result()?
}

#[throws(Status)]
fn load_pkginfo(package_file: &str) -> HashMap<String, String> {
    let package_path = PathBuf::new()
        .join("packages")
        .join(package_file);
    let compressed_reader = std::fs::File::open(package_path)
        .map_err(|_| Status::InternalServerError)?;
    extract_pkginfo(compressed_reader)?
}

#[throws(Status)]
fn extract_pkginfo(reader: impl Read) -> HashMap<String, String> {
    let reader = XzDecoder::new(reader);
    let mut archive = Archive::new(reader);
    let pkginfo_entry = archive.entries()
        .map_err(|_| Status::InternalServerError)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path().unwrap();
            path.as_os_str() == ".PKGINFO"
        })
        .next().ok_or(Status::BadRequest)?;

    let mut contents = String::new();
    pkginfo_entry.take(100_000)
        .read_to_string(&mut contents)
        .map_err(|_| Status::BadRequest)?;

    parse_pkginfo(contents)
        .map_err(|_| Status::BadRequest)?
}

#[throws]
fn parse_pkginfo(pkginfo: String) -> HashMap<String, String> {
    pkginfo.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with("#") {
                None
            } else {
                let components: Vec<_> = line
                    .splitn(2, "=")
                    .collect();
                if components.len() == 2 {
                    Some(Ok((
                        components[0].trim().to_string(),
                        components[1].trim().to_string())))
                } else {
                    Some(Err(Error))
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .collect()
}

fn main() {
    rocket::ignite()
        .attach(Db::fairing())
        .mount("/", routes![upload])
        .launch();
}
