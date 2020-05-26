use std::fs::File;

use fehler::throws;
use log::info;
use regex::Regex;
use rocket::http::ContentType;
use rocket::response::Content;
use rocket::response::Redirect;

use crate::db::create_repo_action;
use crate::db::models::RepoActionOp;
use crate::serve_archive;
use crate::serve_db;
use crate::web::db::Db;
use crate::web::Error;
use crate::web::Error::*;
use crate::web::referer::Referer;
use crate::web::routes::load_account;
use crate::web::routes::load_package;
use crate::web::routes::load_repo;

#[throws]
#[get("/<account>/<repo>/<file>")]
pub fn route_get_package(db: Db, account: String, repo: String, file: String) -> Content<File> {
    let account = load_account(&*db, &account)?;
    let repo = load_repo(&*db, account.id, &repo)?;

    let archive_ext = Regex::new(r#"\.tar\.[a-z]+$"#).unwrap();
    let file = if file.ends_with(".db") {
        info!("Serving database for repo {:?}.", repo);
        serve_db(repo.id as u32)?
    } else if archive_ext.is_match(&file) {
        info!("Serving package from repo {:?}.", repo);
        let package = load_package(&*db, repo.id, &file)?;
        serve_archive(&package.archive)?
    } else {
        Err(NotFound)?
    };

    Content(ContentType::Binary, file)
}

#[throws]
#[post("/<account>/<repo>/<package>/activate")]
pub fn route_activate_package(db: Db, referrer: Referer, account: String, repo: String, package: String) -> Redirect {
    let account = load_account(&*db, &account)?;
    let repo = load_repo(&*db, account.id, &repo)?;
    let package = load_package(&*db, repo.id, &package)?;
    create_repo_action(&*db, package.id, RepoActionOp::Add)?;
    Redirect::to(referrer.0)
}
