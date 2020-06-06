#![feature(never_type)]

use std::env::set_current_dir;
use std::fs::copy;
use std::fs::File;
use std::fs::remove_file;
use std::fs::rename;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use diesel::PgConnection;
use fehler::throws;
use libflate::gzip;
use tar::Archive;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Error;
use pacman::connect_db;
use pacman::db::delete_job;
use pacman::db::get_package;
use pacman::db::models::Package;
use pacman::db::remove_package;
use pacman::db::set_package_active;
use pacman::format_pkg_filename;
use pacman::jobs::create_check_deps;
use pacman::jobs::get_repo_action;
use pacman::jobs::RepoActionOp;
use pacman::parse_pkg_name;

fn main() -> Result<!, Error> {
    let conn = &connect_db()?;

    set_current_dir("worker")
        .with_context(|| "Failed to switch to 'worker' directory")?;

    loop {
        if let Some((id, repo_action)) = get_repo_action(conn, "worker")? {
            let package = get_package(conn, repo_action.package_id)?;
            match repo_action.operation {
                RepoActionOp::Add => {
                    println!("Adding {:?}", package);
                    perform_repo_add(conn, &package)?;
                }
                RepoActionOp::Remove => {
                    println!("Removing {:?}", package);
                    perform_repo_rm(conn, &package)?;
                }
            };
            delete_job(conn, id)?;
            create_check_deps(conn, package.repo_id)?;
        } else {
            thread::sleep(Duration::from_secs(10));
        }
    }
}

#[throws]
fn perform_repo_add(conn: &PgConnection, package: &Package) {
    let package_file = format_pkg_filename(&package);
    remove_file(&package_file).ok();

    let signature_file = format!("{}.sig", package_file);
    remove_file(&signature_file).ok();

    let source = format!("../packages/{}", package.archive);
    symlink(&source, &package_file)?;

    let source = format!("../packages/{}", package.signature);
    symlink(&source, &signature_file)?;

    let source_db = link_source_db(package.repo_id)?;

    let output = Command::new("repo-add")
        .arg("database.db.tar.gz")
        .arg(package_file)
        .output()?;
    if !output.status.success() {
        Err(anyhow!("Invocation of repo-add failed with exit code {:?}",
            output.status.code()))?
    }

    update_source_db(&source_db)?;
    set_package_active(conn, package.id)?;
}

#[throws]
fn perform_repo_rm(conn: &PgConnection, package: &Package) {
    let source_db = link_source_db(package.repo_id)?;

    let packages = read_pkgs_from_db(&source_db)?;
    let needle = (package.name.clone(), package.version.clone());
    if packages.contains(&needle) {
        let output = Command::new("repo-remove")
            .arg("database.db.tar.gz")
            .arg(&package.name)
            .output()?;
        if !output.status.success() {
            Err(anyhow!("Invocation of repo-remove failed with exit code {:?}",
                output.status.code()))?
        }
    }

    remove_package(conn, package.id)?;
    remove_file(format!("../packages/{}", package.archive))?;
    remove_file(format!("../packages/{}", package.signature))?;

    update_source_db(&source_db)?;
}

#[throws]
fn read_pkgs_from_db(db: impl AsRef<Path>) -> Vec<(String, String)> {
    let compressed_file = File::open(db)?;
    let uncompressed_file = gzip::Decoder::new(compressed_file)?;
    let mut archive = Archive::new(uncompressed_file);
    archive.entries()?
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|entry| entry.path())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter_map(|path| {
            let components = path.components().collect::<Vec<_>>();
            if components.len() == 1 {
                components[0].as_os_str().to_str()
            } else {
                None
            }
        })
        .map(parse_pkg_name)
        .collect::<Result<Vec<_>, _>>()?
}

#[throws]
fn link_source_db(repo_id: i32) -> String {
    remove_file("database.db").ok();
    remove_file("database.db.tar.gz").ok();
    remove_file("database.files").ok();
    remove_file("database.files.tar.gz").ok();

    let repo_source = format!("../repos/{}.db.tar.gz", repo_id);
    if Path::new(&repo_source).exists() {
        symlink(&repo_source, "database.db.tar.gz")?;
    }

    repo_source
}

#[throws]
fn update_source_db(source_db: &String) {
    let repo_source_tmp = format!("{}.new", source_db);
    copy("database.db.tar.gz", &repo_source_tmp)?;
    rename(&repo_source_tmp, &source_db)?;
}
