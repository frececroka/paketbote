use std::backtrace::Backtrace;
use std::error::Error as StdError;

use rocket::http::Status;
use rocket::Request;
use rocket::response::Responder;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use thiserror::Error;

use db::Db;

mod ctx_base;
mod props;
mod boundary;
mod db;
mod principal;
mod routes;

#[catch(400)]
fn catch_400_bad_request() -> String {
    "The request you sent was malformed.\n".into()
}

#[catch(401)]
fn catch_401_unauthorized() -> String {
    "Please provide a login cookie or access token.\n".into()
}

#[catch(409)]
fn catch_409_conflict() -> String {
    "Cannot create resource because of a conflict.\n".into()
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Conflict")]
    Conflict,

    #[error("BadRequest")]
    BadRequest,

    #[error("Internal server error: {0}")]
    InternalServerError(#[from] Box<dyn std::error::Error + Send + Sync>, Backtrace)
}

macro_rules! web_error_from {
    ($t: ty) => {
        impl From<$t> for Error {
            fn from(err: $t) -> Self {
                let source = Box::new(err) as Box<dyn std::error::Error + Send + Sync>;
                let backtrace = Backtrace::capture();
                Error::InternalServerError(source, backtrace)
            }
        }
    }
}

web_error_from!(crate::error::Error);
web_error_from!(std::io::Error);
web_error_from!(diesel::result::Error);

impl Into<Status> for Error {
    fn into(self) -> Status {
        use Error::*;
        match self {
            NotFound => Status::NotFound,
            Conflict => Status::Conflict,
            Unauthorized => Status::Unauthorized,
            BadRequest => Status::BadRequest,
            InternalServerError(_, _) => Status::InternalServerError,
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, request: &Request) -> rocket::response::Result<'r> {
        use Error::*;
        match self {
            InternalServerError(_, _) => {
                println!("{}", self);
                if let Some(backtrace) = self.backtrace() {
                    println!("{}", backtrace);
                }
            }
            _ => {}
        }
        let status: Status = self.into();
        status.respond_to(request)
    }
}

pub fn run() {
    rocket::ignite()
        .attach(Db::fairing())
        .attach(Template::fairing())
        .register(catchers![
            catch_400_bad_request,
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
            routes::repo::route_repo_create,
            routes::repo::route_delete_obsolete,
            routes::getfile::getfile,
            routes::upload::upload,
            routes::remove::route_remove,
            routes::search::route_search,
            routes::search::route_search_results])
        .launch();
}
