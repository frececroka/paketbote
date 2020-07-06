use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use db::Db;

mod ctx_base;
mod props;
mod boundary;
mod db;
mod principal;
mod routes;

#[catch(401)]
fn catch_401_unauthorized() -> String {
    "Please provide a login cookie or access token.\n".into()
}

#[catch(409)]
fn catch_409_conflict() -> String {
    "This package/version/arch combination already exists.\n".into()
}

pub fn run() {
    rocket::ignite()
        .attach(Db::fairing())
        .attach(Template::fairing())
        .register(catchers![
            catch_401_unauthorized,
            catch_409_conflict])
        .mount("/public",
            StaticFiles::from("public").rank(-100))
        .mount("/", routes![
            routes::home::home,
            routes::create_account::route_create_account,
            routes::create_account::route_perform_create_account,
            routes::login::route_login,
            routes::login::route_perform_login,
            routes::logout::route_logout,
            routes::account::route_account,
            routes::access_tokens::route_access_tokens,
            routes::access_tokens::route_access_tokens_create,
            routes::access_tokens::route_access_tokens_delete,
            routes::repo::route_repo_text,
            routes::repo::route_repo_html,
            routes::getfile::getfile,
            routes::upload::upload])
        .launch();
}
