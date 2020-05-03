use std::fs::File;
use std::path::PathBuf;

use diesel::PgConnection;
use fehler::throws;
use regex::Regex;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Content;

use crate::db::{get_account_by_name, get_package_by_repo, get_repo_by_account_and_name};
use crate::db::models::Repo;
use crate::error::Error;
use crate::web::db::Db;

#[throws(Status)]
#[get("/<account>/<repo>/<file>")]
pub fn getfile(db: Db, account: String, repo: String, file: String) -> Content<File> {
    let account = get_account_by_name(&*db, &account)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
    let repo = get_repo_by_account_and_name(&*db, account.id, &repo)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    let archive_ext = Regex::new(r#"\.tar\.[a-z]+$"#).unwrap();
    let file = if file.ends_with(".db") {
        serve_db(&repo)
            .map_err(|_| Status::InternalServerError)?
    } else if archive_ext.is_match(&file) {
        serve_package(&*db, &repo, &file)?
    } else {
        Err(Status::NotFound)?
    };

    Content(ContentType::Binary, file)
}

#[throws]
fn serve_db(repo: &Repo) -> File {
    let filename = format!("{}.db.tar.gz", repo.id);
    let path = PathBuf::new()
        .join("repos")
        .join(filename);
    File::open(path)?
}

#[throws(Status)]
fn serve_package(conn: &PgConnection, repo: &Repo, package: &str) -> File {
    let parts: Vec<_> = package.rsplitn(4, "-").collect();
    if parts.len() != 4 { Err(Status::NotFound)? }

    let name = &parts[3];
    let version = format!("{}-{}", parts[2], parts[1]);

    let parts: Vec<_> = parts[0].splitn(2, ".").collect();
    if parts.len() != 2 { Err(Status::NotFound)? }

    let arch = &parts[0];

    let package = get_package_by_repo(conn, repo.id, &name, &version, &arch)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    let path = PathBuf::new()
        .join("packages")
        .join(package.archive);

    File::open(path)
        .map_err(|_| Status::InternalServerError)?
}
