use diesel::QueryResult;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use serde::Serialize;

pub use account::*;
pub use package::*;
pub use package_depends::*;
pub use package_provides::*;
pub use repo::*;
pub use repo_action::*;
pub use token::*;

mod schema;
pub mod models;

mod account;
mod token;
mod package;
mod package_depends;
mod package_provides;
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

#[derive(Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total_items: usize,
    pub current_page: usize,
    pub total_pages: usize,
}

impl<T> Paginated<T> {
    pub fn new(items: Vec<T>, total_items: usize, current_page: usize, page_size: usize) -> Self {
        let total_pages = total_items / page_size + 1;
        Paginated { items, total_items, current_page, total_pages }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Paginated<U> {
        let items = self.items.into_iter().map(f).collect();
        Paginated {
            items,
            total_items: self.total_items,
            current_page: self.current_page,
            total_pages: self.total_pages
        }
    }
}
