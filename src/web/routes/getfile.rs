use std::fs::File;
use std::path::PathBuf;

use diesel::PgConnection;
use fehler::throws;
use rocket::http::Status;
use rocket::response::Stream;

use crate::db::{get_account_by_name, get_package_by_repo, get_repo_by_account_and_name};
use crate::db::models::Repo;
use crate::error::Error;
use crate::web::db::Db;

#[throws(Status)]
#[get("/<account>/<repo>/<file>")]
pub fn getfile(db: Db, account: String, repo: String, file: String) -> Stream<File> {
    let account = get_account_by_name(&*db, &account)
        .map_err(|_| Status::NotFound)?;
    let repo = get_repo_by_account_and_name(&*db, account.id, &repo)
        .map_err(|_| Status::NotFound)?;

    if file.ends_with(".db") {
        serve_db(&repo)
            .map_err(|_| Status::InternalServerError)?
    } else {
        serve_package(&*db, &repo, &file)?
    }
}

#[throws]
fn serve_db(repo: &Repo) -> Stream<File> {
    let filename = format!("{}.db.tar.gz", repo.id);
    let path = PathBuf::new()
        .join("repos")
        .join(filename);
    let f = File::open(path)?;
    Stream::from(f)
}

#[throws(Status)]
fn serve_package(conn: &PgConnection, repo: &Repo, package: &str) -> Stream<File> {
    // linux-mainline-5.7rc3-1-x86_64.pkg.tar.xz
    let parts: Vec<_> = package.rsplitn(4, "-").collect();
    if parts.len() != 4 { Err(Status::NotFound)? }

    let name = parts[3];
    let version = format!("{}-{}", parts[2], parts[1]);

    let parts: Vec<_> = parts[0].splitn(2, ".").collect();
    if parts.len() != 2 { Err(Status::NotFound)? }

    let arch = parts[0];
    let ext = parts[1];

    let package = get_package_by_repo(conn, repo.id, &name, &version, &arch)
        .map_err(|_| Status::NotFound)?;

    let filename = match ext {
        "pkg.tar.xz" => package.archive,
        "pkg.tar.xz.sig" => package.signature,
        _ => Err(Status::NotFound)?
    };

    let path = PathBuf::new()
        .join("packages")
        .join(filename);
    let f = File::open(path)
        .map_err(|_| Status::InternalServerError)?;
    Stream::from(f)
}
