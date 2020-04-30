use std::env::set_current_dir;
use std::fs::{copy, remove_file, rename};
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use diesel::{Connection, PgConnection};
use fehler::throws;

use pacman::db::{delete_repo_add, get_package, get_repo_add};
use pacman::db::models::Package;
use pacman::error::Error;
use pacman::get_config;

fn main() {
    let config = get_config();
    let database = config
        .get_table("databases").unwrap()
        .get("main").unwrap()
        .get("url").unwrap()
        .as_str().unwrap();
    let conn = &PgConnection::establish(database).unwrap();

    set_current_dir("worker").unwrap();

    loop {
        if let Ok(repo_add) = get_repo_add(conn) {
            let package = get_package(conn, repo_add.package_id).unwrap();
            perform_repo_add(package).unwrap();
            delete_repo_add(conn, repo_add.id).unwrap();
        } else {
            thread::sleep(Duration::from_secs(10));
        }
    }
}

#[throws]
fn perform_repo_add(package: Package) {
    remove_file("database.db").ok();
    remove_file("database.db.tar.gz").ok();
    remove_file("database.files").ok();
    remove_file("database.files.tar.gz").ok();

    let package_file = format!("{}-{}-{}.pkg.tar.xz",
        package.name, package.version, package.arch);
    remove_file(&package_file).ok();

    let signature_file = format!("{}.sig", package_file);
    remove_file(&signature_file).ok();

    let source = format!("../packages/{}", package.archive);
    symlink(&source, &package_file)?;

    let source = format!("../packages/{}", package.signature);
    symlink(&source, &signature_file)?;

    let repo_source = format!("../repos/{}.db.tar.gz", package.repo_id);
    if Path::new(&repo_source).exists() {
        symlink(&repo_source, "database.db.tar.gz").ok();
    }

    let output = Command::new("repo-add")
        .arg("database.db.tar.gz")
        .arg(package_file)
        .output()?;

    if !output.status.success() {
        println!("{}", String::from_utf8(output.stderr)?);
        Err(Error)?
    }

    println!("{}", String::from_utf8(output.stdout)?);

    let repo_source_tmp = format!("{}.new", repo_source);
    copy("database.db.tar.gz", &repo_source_tmp)?;
    rename(&repo_source_tmp, &repo_source)?;
}
