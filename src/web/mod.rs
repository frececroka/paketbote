mod boundary;
mod db;
mod principal;
mod routes;

use db::Db;

pub fn run() {
    rocket::ignite()
        .attach(Db::fairing())
        .mount("/", routes![
            routes::upload::upload,
            routes::getfile::getfile])
        .launch();
}
