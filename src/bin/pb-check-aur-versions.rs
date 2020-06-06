use anyhow::Error;
use fehler::throws;
use itertools::Itertools;

use pacman::aur::check_aur_version;
use pacman::connect_db;
use pacman::db::create_aur_version;
use pacman::db::get_packages;

#[throws]
fn main() {
    let conn = &connect_db()?;

    let packages = get_packages(conn)?.into_iter()
        .map(|p| p.name)
        .dedup()
        .collect::<Vec<_>>();

    for packages in packages.chunks(200) {
        let packages = &packages.iter()
            .map(|p| p as &str)
            .collect::<Vec<_>>();
        let results = check_aur_version(packages)?;
        for (package, version) in results {
            println!("{}... {}", package, version);
            create_aur_version(conn, package, version)?;
        }
    }
}
