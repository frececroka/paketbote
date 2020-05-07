use diesel::QueryResult;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;

pub use account::*;
pub use package::*;
pub use repo::*;
pub use repo_action::*;
pub use token::*;

mod schema;
pub mod models;

mod account;
mod token;
mod package;
mod repo;
mod repo_action;

pub trait ExpectConflict {
    type Output;
    fn expect_conflict(self) -> Self::Output;
}

impl<T> ExpectConflict for QueryResult<T> {
    type Output = QueryResult<Option<T>>;
    fn expect_conflict(self) -> Self::Output {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(DatabaseError(UniqueViolation, _)) => Ok(None),
            Err(error) => Err(error)
        }
    }
}
