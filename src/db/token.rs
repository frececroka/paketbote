use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;

use crate::db::models::{NewToken, Token};

use super::schema;

#[throws]
pub fn create_token(conn: &PgConnection, token: &NewToken) {
    use schema::token::dsl as t;
    diesel::insert_into(t::token)
        .values(token)
        .execute(conn)?;
}

#[throws]
pub fn get_tokens_for_account(conn: &PgConnection, account_id: i32) -> Vec<Token> {
    use schema::token::dsl as t;
    t::token
        .filter(t::account_id.eq(account_id))
        .load(conn)?
}

#[throws]
pub fn delete_token_for_account(conn: &PgConnection, account_id: i32, token_id: i32) {
    use schema::token::dsl as t;
    diesel::delete(t::token)
        .filter(t::id.eq(token_id))
        .filter(t::account_id.eq(account_id))
        .execute(conn)?
}
