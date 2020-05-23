use fehler::throws;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db::create_account;
use crate::db::ExpectConflict;
use crate::db::models::NewAccount;
use crate::web::ctx_base::BaseContext;
use crate::web::db::Db;
use crate::web::Error;
use crate::web::props::Props;
use crate::web::routes::{create_random_token, hash_password};

#[derive(Debug, Serialize)]
struct CreateAccountContext {
    base: BaseContext,
    msg: Option<String>
}

#[get("/create-account?<msg>")]
pub fn route_create_account(props: Props, msg: Option<String>) -> Template {
    let base = BaseContext::new(&props.account);
    let context = CreateAccountContext { base, msg };
    Template::render("create-account", context)
}

#[derive(FromForm)]
pub struct CreateAccount {
    username: String,
    password: String,
}

#[throws]
#[post("/create-account", data = "<body>")]
pub fn route_perform_create_account(db: Db, body: Form<CreateAccount>) -> Redirect {
    let name = body.username.to_string();
    let salt = create_random_token();
    let password = hash_password(&salt, &body.password);
    let account = NewAccount { name, salt, hashed_password: password };
    let account = create_account(&*db, &account)
        .expect_conflict()?;
    Redirect::to(match account {
        Some(_) => "/login?msg=account-created",
        None => "/create-account?msg=username-taken"
    })
}
