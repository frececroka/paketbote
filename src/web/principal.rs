use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::db;
use crate::db::get_account_for_token;
use crate::web::db::Db;

#[derive(Debug)]
pub struct Principal(pub db::models::Account);

impl<'a, 'r> FromRequest<'a, 'r> for Principal {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // We need an authorization header.
        let authorization = if let Some(authorization) = request.headers().get_one("Authorization") {
            authorization
        } else {
            return Outcome::Failure((Status::Unauthorized, ()));
        };

        // We need a bearer token.
        let token = if authorization.starts_with("Bearer ") {
            &authorization["Bearer ".len()..]
        } else {
            return Outcome::Failure((Status::Unauthorized, ()));
        };

        // The token must belong to a user account.
        let db = Db::from_request(request).unwrap();
        if let Ok(account) = get_account_for_token(&*db, token) {
            Outcome::Success(Principal(account))
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
