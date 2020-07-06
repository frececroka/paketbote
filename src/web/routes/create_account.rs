use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use fehler::throws;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::db::create_account;
use crate::db::models::NewAccount;
use crate::web::db::Db;
use crate::web::routes::{no_context, hash_password};

#[get("/create-account")]
pub fn route_create_account() -> Template {
    Template::render("create-account", no_context())
}

#[derive(FromForm)]
pub struct CreateAccount {
    username: String,
    password: String,
}

#[throws(Status)]
#[post("/create-account", data = "<body>")]
pub fn route_perform_create_account(db: Db, body: Form<CreateAccount>) -> Redirect {
    let name = body.username.to_string();
    let salt = "asdf".to_string();
    let password = hash_password(&salt, &body.password);
    let account = NewAccount { name, salt, hashed_password: password };
    match create_account(&*db, &account) {
        Ok(_) => Redirect::to("/login?account-created"),
        Err(DatabaseError(UniqueViolation, _)) =>
            Redirect::to("/create-account?username-taken"),
        Err(_) => Err(Status::InternalServerError)?
    }
}
