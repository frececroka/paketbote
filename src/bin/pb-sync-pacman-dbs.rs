use anyhow::Error;
use fehler::throws;

use pacman::alpm;
use pacman::connect_db;
use pacman::db::get_repos;
use pacman::jobs::create_check_deps;

#[throws]
fn main() {
    let conn = &connect_db()?;

    alpm::sync("x86_64", x86_64_mirror)?;
    println!("Synced x86_64 repositories.");

    for arch in &["aarch64", "arm", "armv6h", "armv7h"] {
        alpm::sync(arch, arm_mirror)?;
        println!("Synced {} repositories.", arch);
    }

    let repos = get_repos(conn)?;
    for repo in repos {
        create_check_deps(conn, repo.id)?;
    }
}

fn x86_64_mirror(repo: &str, arch: &str) -> String {
    format!(
        "http://archlinux.honkgong.info/{}/os/{}",
        repo, arch)
}

fn arm_mirror(repo: &str, arch: &str) -> String {
    format!(
        "http://mirror.archlinuxarm.org/{}/{}",
        arch, repo)
}
