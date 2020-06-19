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
use crate::web::routes::login::rocket_uri_macro_route_login;

#[derive(Debug, Serialize)]
struct CreateAccountContext {
    base: BaseContext,
    username: Option<String>,
    msg: Option<String>
}

#[get("/create-account?<username>&<msg>")]
pub fn route_create_account(props: Props, username: Option<String>, msg: Option<String>) ->
Template {
    let base = BaseContext::new(&props.account);
    let context = CreateAccountContext { base, username, msg };
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
    if name.is_empty() {
        return Redirect::to(uri!(route_create_account:
            username = &body.username,
            msg = "username-empty"))
    }

    let salt = create_random_token();
    let hashed_password = hash_password(&salt, &body.password);

    let account = NewAccount { name, salt, hashed_password };
    let account = create_account(&*db, &account)
        .expect_conflict()?;
    Redirect::to(match account {
        Some(_) => uri!(route_login:
            username = &body.username,
            msg = "account-created"),
        None => uri!(route_create_account:
            username = &body.username,
            msg = "username-taken")
    })
}
