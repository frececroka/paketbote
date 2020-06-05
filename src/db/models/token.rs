use serde::Serialize;

use crate::db::schema::*;

#[derive(Debug, Serialize, Queryable)]
pub struct Token {
    pub id: i32,
    pub name: String,
    pub the_token: String,
    pub account_id: i32
}

#[derive(Debug, Serialize, Insertable)]
#[table_name="token"]
pub struct NewToken {
    pub name: String,
    pub the_token: String,
    pub account_id: i32
}
