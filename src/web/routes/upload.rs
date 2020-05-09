use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use fehler::throws;
use libflate::gzip;
use log::info;
use multipart::server::{Multipart, MultipartData};
use rocket::Data;
use rocket::data::DataStream;
use rocket::http::Status;
use tar::Archive;
use uuid::Uuid;
use xz2::read::XzDecoder;

use crate::db::{create_package, create_repo_action, get_package_by_repo, get_repo_by_account_and_name};
use crate::db::ExpectConflict;
use crate::db::models::{Account, Compression, NewPackage};
use crate::error::Error;
use crate::parse_pkg_filename;
use crate::web::boundary::Boundary;
use crate::web::db::Db;
use crate::web::routes::validate_access;

#[throws(Status)]
#[post("/<account>/<repo>/<package>", data = "<data>", rank = 5)]
pub fn upload(db: Db, active_account: Account, account: String, repo: String, package: String, boundary: Boundary, data: Data) {
    let account = validate_access(active_account, account)?;

    let repo = get_repo_by_account_and_name(&*db, account.id, &repo)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    let (name, version, arch, compression) = parse_pkg_filename(&package)
        .map_err(|_| Status::BadRequest)?;
    let existing_package = get_package_by_repo(&*db, repo.id, &name, &version, &arch)
        .map_err(|_| Status::InternalServerError)?;
    if existing_package.is_some() {
        info!("Aborting upload early, because package already exists in this version.");
        Err(Status::Conflict)?
    }

    info!("Saving uploaded files to disk...");
    let ((package_file, package_size), (signature_file, signature_size)) =
        save_uploaded_files(data, &boundary.0)?;
    info!("Received package of size {} and signature of size {}.",
          package_size, signature_size);

    let total_size: i32 = (package_size + signature_size)
        .try_into().map_err(|_| Status::BadRequest)?;
    info!("The total size of uploaded files is {}.", total_size);

    info!("Loading PKGINFO from package...");
    let pkginfo = load_pkginfo(compression, &package_file)?;
    let pkgname = pkginfo.get("pkgname").ok_or(Status::BadRequest)?;
    let pkgver = pkginfo.get("pkgver").ok_or(Status::BadRequest)?;
    let arch = pkginfo.get("arch").ok_or(Status::BadRequest)?;
    info!("Package has name {}, version {}, and is for architecture {}",
          pkgname, pkgver, arch);

    let package = NewPackage {
        name: pkgname.to_string(),
        version: pkgver.to_string(),
        arch: arch.to_string(),
        size: total_size,
        archive: package_file,
        signature: signature_file,
        compression: compression,
        repo_id: repo.id,
    };

    info!("Adding package to database: {:?}", package);
    let package = create_package(&*db, &package)
        .expect_conflict()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::Conflict)?;
    create_repo_action(&*db, package.id, "add".to_string())
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
fn load_pkginfo(compression: Compression, package_file: &str) -> HashMap<String, String> {
    let package_path = PathBuf::new()
        .join("packages")
        .join(package_file);
    let compressed_reader = std::fs::File::open(package_path)
        .map_err(|_| Status::InternalServerError)?;
    let decompressed_reader = decompress(compression, compressed_reader)
        .map_err(|_| Status::BadRequest)?;
    extract_pkginfo(decompressed_reader)?
}

#[throws(io::Error)]
fn decompress(compression: Compression, reader: impl Read + 'static) -> Box<dyn Read + 'static> {
    use Compression::*;
    match compression {
        Lzma => Box::new(XzDecoder::new(reader)) as Box<dyn Read>,
        Zstd => Box::new(zstd::Decoder::new(reader)?) as Box<dyn Read>,
        Gzip => Box::new(gzip::Decoder::new(reader)?) as Box<dyn Read>
    }
}

#[throws(Status)]
fn extract_pkginfo(reader: impl Read) -> HashMap<String, String> {
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
