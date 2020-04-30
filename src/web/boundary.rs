use rocket::request::{FromRequest, Outcome};
use rocket::Request;

#[derive(Debug)]
pub struct Boundary(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for Boundary {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let ct = request.headers().get_one("Content-Type").expect("no content-type");
        let idx = ct.find("boundary=").expect("no boundary");
        Outcome::Success(Boundary(ct[(idx + "boundary=".len())..].to_string()))
    }
}
