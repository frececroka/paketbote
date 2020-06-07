use serde::Serialize;

use crate::db::models::Account;

#[derive(Debug, Serialize)]
pub struct BaseContext {
    git_ref: String,
    account: Option<String>
}

impl BaseContext {
    pub fn new(account: &Option<Account>) -> BaseContext {
        let git_ref = include_str!("../../.git/refs/heads/master").to_owned();
        let account = account.as_ref().map(|a| a.name.clone());
        BaseContext { git_ref, account }
    }
}
