use std::marker::PhantomData;

use rocket::http::ext::IntoOwned;
use rocket::http::Status;
use rocket::http::uri::Uri;
use rocket::Request;
use rocket::request::FromRequest;
use rocket::request::Outcome;

#[derive(Debug)]
pub struct Referer(pub Uri<'static>);

impl FromRequest<'_, '_> for Referer {
    type Error = ();
    fn from_request(request: &Request) -> Outcome<Self, Self::Error> {
        if let Some(referer) = request.headers().get_one("referer") {
            match Uri::parse(referer) {
                Ok(uri) => Outcome::Success(Referer(uri.into_owned())),
                Err(_) => Outcome::Failure((Status::BadRequest, ()))
            }
        } else {
            Outcome::Failure((Status::BadRequest, ()))
        }
    }
}

#[derive(Debug)]
pub struct Header<T>(pub String, PhantomData<T>);

macro_rules! impl_header_from_request {
    ($header: ident, $name: expr) => {
        impl FromRequest<'_, '_> for Header<rocket::http::hyper::header::$header> {
            type Error = ();
            fn from_request(request: &Request) -> Outcome<Self, Self::Error> {
                if let Some(referer) = request.headers().get_one($name) {
                    Outcome::Success(Header(referer.to_owned(), PhantomData))
                } else {
                    Outcome::Failure((Status::BadRequest, ()))
                }
            }
        }
    }
}

impl_header_from_request!(Referer, "referer");
