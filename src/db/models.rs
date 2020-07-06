#![allow(dead_code)]

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

#[derive(Debug, Serialize, Queryable)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub arch: String,
    pub size: i32,
    pub archive: String,
    pub signature: String,
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
    pub repo_id: i32
}

#[derive(Debug, Serialize, Queryable)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub owner_id: i32
}

#[derive(Debug, Serialize, Queryable)]
pub struct RepoAdd {
    pub id: i32,
    pub package_id: i32,
    pub worker: Option<String>
}

#[derive(Debug, Serialize, Insertable)]
#[table_name="repo_add"]
pub struct NewRepoAdd {
    pub package_id: i32
}

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
