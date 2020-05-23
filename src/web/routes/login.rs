use fehler::throws;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db::get_account_by_name;
use crate::web::ctx_base::BaseContext;
use crate::web::Error;
use crate::web::props::Props;
use crate::web::routes::hash_password;

#[derive(Serialize)]
struct LoginContext {
    base: BaseContext,
    msg: Option<String>
}

#[get("/login?<msg>")]
pub fn route_login(props: Props, msg: Option<String>) -> Template {
    let base = BaseContext::new(&props.account);
    let context = LoginContext { base, msg };
    Template::render("login", context)
}

#[derive(FromForm)]
pub struct LoginData {
    username: String,
    password: String,
}

#[throws]
#[post("/login", data = "<body>")]
pub fn route_perform_login(props: Props, mut cookies: Cookies, body: Form<LoginData>) -> Redirect {
    let account = get_account_by_name(&*props.db, &body.username)?;
    let account =
        if let Some(account) = account { account } else {
            return Redirect::to("/login?msg=wrong-username");
        };
    let hashed_password = hash_password(&account.salt, &body.password);
    if hashed_password == account.hashed_password {
        let session_cookie = Cookie::new("account", body.username.clone());
        cookies.add_private(session_cookie);
        let target = format!("/{}", body.username);
        Redirect::to(target)
    } else {
        Redirect::to("/login?msg=wrong-password")
    }
}
