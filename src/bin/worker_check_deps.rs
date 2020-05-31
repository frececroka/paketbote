#![feature(never_type)]

use std::thread;
use std::time::Duration;

use anyhow::Error;

use pacman::connect_db;
use pacman::db::delete_job;
use pacman::db::replace_missing_deps;
use pacman::jobs::get_check_deps;
use pacman::missing::missing_dependencies;

fn main() -> Result<!, Error> {
    let conn = &connect_db()?;

    loop {
        if let Some((id, check_deps)) = get_check_deps(conn, "worker")? {
            let repo_id = check_deps.repo_id;
            let missing_deps = missing_dependencies(conn, repo_id)?.into_iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>();
            println!("repo {} is missing these dependencies: {:?}", repo_id, missing_deps);
            replace_missing_deps(conn, repo_id, missing_deps)?;
            delete_job(conn, id)?;
        } else {
            thread::sleep(Duration::from_secs(10));
        }
    }
}
