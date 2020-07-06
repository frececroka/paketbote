use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::{Account, NewAccount, Token};

use super::schema;

#[throws]
pub fn create_account(conn: &PgConnection, account: &NewAccount) -> Account {
    use schema::account::dsl as a;
    diesel::insert_into(a::account)
        .values(account)
        .get_result(conn)?
}

#[throws]
pub fn get_account(conn: &PgConnection, account_id: i32) -> Account {
    use schema::account::dsl as a;
    a::account
        .filter(a::id.eq(account_id))
        .first(conn)?
}

#[throws]
pub fn get_account_by_name(conn: &PgConnection, name: &str) -> Option<Account> {
    use schema::account::dsl as a;
    a::account
        .filter(a::name.eq(name))
        .first(conn)
        .optional()?
}

#[throws]
pub fn get_account_for_token(conn: &PgConnection, token: &str) -> Option<Account> {
    use schema::token::dsl as t;
    let token: Option<Token> = t::token
        .filter(t::the_token.eq(token))
        .first(conn)
        .optional()?;
    if let Some(token) = token {
        Some(get_account(conn, token.account_id)?)
    } else {
        None
    }
}

