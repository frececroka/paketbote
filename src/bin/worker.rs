use pacman::get_config;
use diesel::{PgConnection, Connection};
use std::thread;
use std::time::Duration;
use pacman::db::get_repo_add;

fn main() {
    let config = get_config();
    let database = config
        .get_table("databases").unwrap()
        .get("main").unwrap()
        .get("url").unwrap()
        .as_str().unwrap();
    let conn = &PgConnection::establish(database).unwrap();

    loop {
        if let Ok(repo_add) = get_repo_add(conn) {
            println!("{:?}", repo_add);
        } else {
            thread::sleep(Duration::from_secs(10));
        }
    }
}
