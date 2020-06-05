use chrono::NaiveDateTime;
use serde::Serialize;

use crate::db::schema::*;

use super::Compression;

#[derive(Debug, Queryable)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub arch: String,
    pub size: i32,
    pub archive: String,
    pub signature: String,
    pub compression: Compression,
    pub created: NaiveDateTime,
    pub active: bool,
    pub deleted: bool,
    pub repo_id: i32
}

#[derive(Debug, Serialize, Insertable)]
#[table_name="package"]
pub struct NewPackage {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub size: i32,
    pub archive: String,
    pub signature: String,
    pub compression: Compression,
    pub repo_id: i32
}
