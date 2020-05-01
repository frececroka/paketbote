mod boundary;
mod db;
mod principal;
mod routes;

use db::Db;

#[catch(409)]
fn catch_409_conflict() -> String {
    "This package/version/arch combination already exists.\n".into()
}

pub fn run() {
    rocket::ignite()
        .attach(Db::fairing())
        .register(catchers![
            catch_409_conflict])
        .mount("/", routes![
            routes::list::list,
            routes::getfile::getfile,
            routes::upload::upload])
        .launch();
}
