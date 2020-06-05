use serde::Serialize;

use crate::db::schema::*;

#[derive(Debug, Serialize, Queryable)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub salt: String,
    pub hashed_password: String
}

#[derive(Debug, Serialize, Insertable)]
#[table_name="account"]
pub struct NewAccount {
    pub name: String,
    pub salt: String,
    pub hashed_password: String
}
