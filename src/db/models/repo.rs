use serde::Serialize;

use crate::db::schema::*;

#[derive(Debug, Serialize, Queryable)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub owner_id: i32
}

#[derive(Debug, Serialize, Insertable)]
#[table_name="repo"]
pub struct NewRepo {
    pub name: String,
    pub owner_id: i32
}
