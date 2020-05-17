use diesel::{Connection, PgConnection};
use fehler::throws;

use pacman::db::{create_package_depends, create_package_provides, get_package_provides, get_packages};
use pacman::db::models::Package;
use pacman::error::Error;
use pacman::get_config;
use pacman::pkginfo::load_pkginfo;

fn main() {
    let config = get_config();
    let database = config
        .get_table("databases").unwrap()
        .get("main").unwrap()
        .get("url").unwrap()
        .as_str().unwrap();
    let conn = &PgConnection::establish(database).unwrap();

    let packages = get_packages(conn).unwrap();
    for package in &packages {
        println!("{:?}", package);
        process_package(conn, package).ok();
    }
}

#[throws]
fn process_package(conn: &PgConnection, package: &Package) {
    let provides = get_package_provides(conn, package.id)?;
    if provides.len() > 0 {
        return;
    }

    let pkginfo = load_pkginfo(package.compression, &package.archive)?;

    let no_depends = vec![];
    let depends = pkginfo.get("depend")
        .unwrap_or(&no_depends);
    for depends in depends {
        create_package_depends(conn, package.id, depends.clone())?;
    }

    create_package_provides(conn, package.id, package.name.clone())?;

    let no_provides = vec![];
    let provides = pkginfo.get("provides")
        .unwrap_or(&no_provides);
    for provides in provides {
        create_package_provides(conn, package.id, provides.clone())?;
    }
}
