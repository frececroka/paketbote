use std::fs::File;

use diesel::PgConnection;
use fehler::throws;
use log::info;
use regex::Regex;
use rocket::http::ContentType;
use rocket::response::Content;

use crate::{parse_pkg_filename, serve_archive, serve_db};
use crate::db::{get_account_by_name, get_package_by_repo, get_repo_by_account_and_name};
use crate::db::models::Repo;
use crate::web::db::Db;
use crate::web::Error;
use crate::web::Error::*;

#[throws]
#[get("/<account>/<repo>/<file>")]
pub fn getfile(db: Db, account: String, repo: String, file: String) -> Content<File> {
    let account = get_account_by_name(&*db, &account)?
        .ok_or(NotFound)?;
    let repo = get_repo_by_account_and_name(&*db, account.id, &repo)?
        .ok_or(NotFound)?;

    let archive_ext = Regex::new(r#"\.tar\.[a-z]+$"#).unwrap();
    let file = if file.ends_with(".db") {
        info!("Serving database for repo {:?}.", repo);
        serve_db(repo.id as u32)?
    } else if archive_ext.is_match(&file) {
        info!("Serving package from repo {:?}.", repo);
        serve_package(&*db, &repo, &file)?
    } else {
        Err(NotFound)?
    };

    Content(ContentType::Binary, file)
}

#[throws]
fn serve_package(conn: &PgConnection, repo: &Repo, package: &str) -> File {
    let (name, version, arch, _) = parse_pkg_filename(package)
        .map_err(|_| NotFound)?;

    let package = get_package_by_repo(conn, repo.id, &name, &version, &arch)?
        .ok_or(NotFound)?;

    serve_archive(&package.archive)?
}
