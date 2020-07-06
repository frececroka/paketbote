use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

#[derive(Debug)]
pub struct Boundary(pub String);

impl FromRequest<'_, '_> for Boundary {
    type Error = ();
    fn from_request(request: &Request) -> Outcome<Self, Self::Error> {
        let ct = match request.headers().get_one("Content-Type") {
            Some(ct) => ct,
            None => return Outcome::Failure((Status::BadRequest, ()))
        };
        let idx = match ct.find("boundary=") {
            Some(idx) => idx,
            None => return Outcome::Failure((Status::BadRequest, ()))
        };
        Outcome::Success(Boundary(ct[(idx + "boundary=".len())..].to_string()))
    }
}
