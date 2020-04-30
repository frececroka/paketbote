mod boundary;
mod db;
mod principal;
mod routes;

use db::Db;

pub(crate) fn run() {
    rocket::ignite()
        .attach(Db::fairing())
        .mount("/", routes![
            routes::upload::upload,
            routes::getpackage::get_package])
        .launch();
}
