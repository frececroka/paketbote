use fehler::throws;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::db::{get_account_by_name, get_account_for_token};
use crate::db::models::Account;
use crate::error::Error;
use crate::web::db::Db;

pub enum LoggedIn {
    T(Account),
    F
}

impl Into<Option<Account>> for LoggedIn {
    fn into(self) -> Option<Account> {
        match self {
            LoggedIn::T(account) => Some(account),
            LoggedIn::F => None
        }
    }
}

impl FromRequest<'_, '_> for LoggedIn {
    type Error = ();
    fn from_request(request: &Request) -> Outcome<Self, Self::Error> {
        match from_request_fallible(request) {
            Ok(Some(account)) => Outcome::Success(LoggedIn::T(account)),
            Ok(None) => Outcome::Success(LoggedIn::F),
            Err(_) => Outcome::Failure((Status::InternalServerError, ()))
        }
    }
}

impl FromRequest<'_, '_> for Account {
    type Error = ();
    fn from_request(request: &Request) -> Outcome<Self, Self::Error> {
        match from_request_fallible(request) {
            Ok(Some(account)) => Outcome::Success(account),
            Ok(None) => Outcome::Failure((Status::Unauthorized, ())),
            Err(_) => Outcome::Failure((Status::InternalServerError, ()))
        }
    }
}

#[throws]
fn from_request_fallible(request: &Request) -> Option<Account> {
    if let Some(account) = from_authorization_header(request)? {
        Some(account)
    } else if let Some(account) = from_session_cookie(request)? {
        Some(account)
    } else {
        None
    }
}

#[throws]
fn from_authorization_header(request: &Request) -> Option<Account> {
    // We need an authorization header.
    let authorization = if let Some(authorization) = request.headers().get_one("Authorization") {
        authorization
    } else {
        return None;
    };

    // With a bearer token.
    let token = if authorization.starts_with("Bearer ") {
        &authorization["Bearer ".len()..]
    } else {
        return None;
    };

    // The token must belong to a user account.
    let db = Db::from_request(request).unwrap();
    get_account_for_token(&*db, token)?
}

#[throws]
fn from_session_cookie(request: &Request) -> Option<Account> {
    if let Some(session_cookie) = request.cookies().get_private("account") {
        let db = Db::from_request(request).unwrap();
        get_account_by_name(&*db, session_cookie.value())?
    } else {
        None
    }
}
