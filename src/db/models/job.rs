use serde::Serialize;

use crate::db::schema::*;

#[derive(Debug, Serialize, Queryable)]
pub struct Job {
    pub id: i32,
    pub tag: String,
    pub spec: serde_json::Value,
    pub worker: Option<String>
}

#[derive(Debug, Serialize, Insertable)]
#[table_name="job"]
pub struct NewJob {
    pub tag: String,
    pub spec: serde_json::Value
}
