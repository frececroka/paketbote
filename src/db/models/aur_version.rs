use serde::Serialize;

use crate::db::schema::*;

#[derive(Debug, Serialize, Queryable)]
pub struct AurVersion {
    pub id: i32,
    pub package: String,
    pub version: String
}

#[derive(Debug, Serialize, Insertable)]
#[table_name="aur_version"]
pub struct NewAurVersion {
    pub package: String,
    pub version: String
}
