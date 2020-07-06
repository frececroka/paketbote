use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;

#[get("/logout")]
pub fn route_logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("account"));
    Redirect::to("/")
}
