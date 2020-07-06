use serde::Serialize;

use crate::db::models::Account;

#[derive(Serialize)]
pub struct BaseContext {
    account: Option<String>
}

impl BaseContext {
    pub fn new(account: &Option<Account>) -> BaseContext {
        let account = account.as_ref().map(|a| a.name.clone());
        BaseContext { account }
    }
}
