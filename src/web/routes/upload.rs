use std::borrow::Borrow;
use std::convert::TryInto;

use fehler::throws;
use log::info;
use multipart::server::Multipart;
use rocket::Data;
use uuid::Uuid;

use crate::db::create_package;
use crate::db::create_package_depends;
use crate::db::create_package_provides;
use crate::db::ExpectConflict;
use crate::db::get_package_by_repo;
use crate::db::get_repo_by_account_and_name;
use crate::db::models::Account;
use crate::db::models::NewPackage;
use crate::jobs::create_repo_action;
use crate::jobs::RepoActionOp;
use crate::parse_pkg_filename;
use crate::pkginfo::load_pkginfo;
use crate::save_archive;
use crate::web::boundary::Boundary;
use crate::web::db::Db;
use crate::web::Error;
use crate::web::Error::*;
use crate::web::routes::validate_access;

#[throws]
#[post("/<account>/<repo>/<package>", data = "<data>", rank = 5)]
pub fn upload(db: Db, active_account: Account, account: String, repo: String, package: String, boundary: Boundary, data: Data) {
    let account = validate_access(active_account, account)?;

    let repo = get_repo_by_account_and_name(&*db, account.id, &repo)?
        .ok_or(NotFound)?;

    let (name, version, arch, compression) = parse_pkg_filename(&package)
        .map_err(|_| BadRequest)?;
    let existing_package = get_package_by_repo(&*db, repo.id, &name, &version, &arch)?;
    if existing_package.is_some() {
        info!("Aborting upload early, because package already exists in this version.");
        Err(Conflict)?
    }

    info!("Saving uploaded files to disk...");
    let ((package_file, package_size), (signature_file, signature_size)) =
        save_uploaded_files(data, &boundary.0)?;
    info!("Received package of size {} and signature of size {}.",
          package_size, signature_size);

    let total_size: i32 = (package_size + signature_size)
        .try_into().map_err(|_| BadRequest)?;
    info!("The total size of uploaded files is {}.", total_size);

    info!("Loading PKGINFO from package...");
    let pkginfo = load_pkginfo(compression, &package_file)?;
    let pkgname = pkginfo
        .get("pkgname").ok_or(BadRequest)?
        .first().ok_or(BadRequest)?;
    let pkgver = pkginfo
        .get("pkgver").ok_or(BadRequest)?
        .first().ok_or(BadRequest)?;
    let arch = pkginfo
        .get("arch").ok_or(BadRequest)?
        .first().ok_or(BadRequest)?;
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
        .expect_conflict()?
        .ok_or(Conflict)?;

    let no_depends = vec![];
    let depends = pkginfo.get("depend")
        .unwrap_or(&no_depends);
    for depends in depends {
        create_package_depends(&*db, package.id, depends.clone())?;
    }

    create_package_provides(&*db, package.id, package.name.clone())?;

    let no_provides = vec![];
    let provides = pkginfo.get("provides")
        .unwrap_or(&no_provides);
    for provides in provides {
        create_package_provides(&*db, package.id, provides.clone())?;
    }

    create_repo_action(&*db, package.id, RepoActionOp::Add)?;
}

#[throws]
fn save_uploaded_files(data: Data, boundary: &str) -> ((String, u64), (String, u64)) {
    let mut package: Option<(String, u64)> = None;
    let mut signature: Option<(String, u64)> = None;

    let mut multipart = Multipart::with_body(data.open(), boundary);
    while let Some(entry) = multipart.read_entry()? {
        let name = entry.headers.name.borrow();
        let target = match name {
            "package" => &mut package,
            "signature" => &mut signature,
            _ => continue
        };
        let filename = Uuid::new_v4().to_string();
        let filesize = save_archive(&filename, entry.data)?;
        *target = Some((filename, filesize));
    }

    (package.ok_or(BadRequest)?, signature.ok_or(BadRequest)?)
}
