use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::db::models::Account;
use crate::web::db::Db;
use crate::web::principal::LoggedIn;

pub struct Props {
    pub db: Db,
    pub account: Option<Account>
}

macro_rules! req_get_guard {
    ($req: expr, $guard: ty) => {
        match $req.guard::<$guard>() {
            Outcome::Success(value) => value,
            _ => return Outcome::Failure((Status::InternalServerError, ()))
        };
    };
}

impl FromRequest<'_, '_> for Props {
    type Error = ();
    fn from_request(request: &Request) -> Outcome<Self, Self::Error> {
        let db = req_get_guard!(request, Db);
        let account = req_get_guard!(request, LoggedIn).into();
        let props = Props { db, account };
        Outcome::Success(props)
    }
}
