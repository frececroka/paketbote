use std::fs;
use std::io;

use alpm::Alpm;
use alpm::SigLevel;
use fehler::throws;

use crate::error::Error;

#[throws]
pub fn sync(arch: &str, mirror: impl Fn(&str, &str) -> String) {
    let mut alpm = create(arch)?;
    for mut db in alpm.syncdbs_mut() {
        let server_url = mirror(db.name(), arch);
        db.add_server(server_url)?;
        db.update(false)?;
    }
}

#[throws]
pub fn create(arch: &str) -> Alpm {
    let db_path = db_path(arch)?;
    let alpm = Alpm::new("/", &db_path)?;
    alpm.register_syncdb("core", SigLevel::NONE)?;
    alpm.register_syncdb("community", SigLevel::NONE)?;
    alpm.register_syncdb("extra", SigLevel::NONE)?;
    alpm
}

#[throws(io::Error)]
fn db_path(arch: &str) -> String {
    let path = format!("pacman-db/{}", arch);
    fs::create_dir_all(&path)?;
    path
}
