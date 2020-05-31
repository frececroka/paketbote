#![feature(never_type)]

use std::thread;
use std::time::Duration;

use diesel::Connection;
use diesel::PgConnection;

use anyhow::Error;
use pacman::db::delete_job;
use pacman::get_config;
use pacman::jobs::get_check_deps;
use pacman::missing::missing_dependencies;

fn main() -> Result<!, Error> {
    let config = get_config();
    let database = config
        .get_table("databases").unwrap()
        .get("main").unwrap()
        .get("url").unwrap()
        .as_str().unwrap();
    let conn = &PgConnection::establish(database)?;

    loop {
        if let Some((id, check_deps)) = get_check_deps(conn, "worker")? {
            let repo_id = check_deps.repo_id;
            let missing = missing_dependencies(conn, repo_id)?;
            println!("repo {} is missing these dependencies: {:?}", repo_id, missing);
            delete_job(conn, id)?;
        } else {
            thread::sleep(Duration::from_secs(10));
        }
    }
}
