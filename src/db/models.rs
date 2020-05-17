#![allow(dead_code)]

use std::fmt;
use std::io::Write;
use std::str::FromStr;

use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use fehler::throws;
use serde::Serialize;

use crate::db::schema::*;
use crate::error::Error;

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

#[derive(Debug, Clone, Copy, Serialize, FromSqlRow, AsExpression)]
#[sql_type = "Text"]
pub enum Compression {
    Zstd, Gzip, Lzma
}

impl FromStr for Compression {
    type Err = Error;
    #[throws]
    fn from_str(string: &str) -> Self {
        match string {
            "xz" => Compression::Lzma,
            "gz" => Compression::Gzip,
            "zst" => Compression::Zstd,
            _ => Err(Error)?
        }
    }
}

impl fmt::Display for Compression {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Compression::Lzma => "xz",
            Compression::Gzip => "gz",
            Compression::Zstd => "zst",
        };
        write!(fmt, "{}", repr)
    }
}

impl<DB> FromSql<Text, DB> for Compression
    where DB: Backend, String: FromSql<Text, DB>,
{
    #[throws(Box<dyn std::error::Error + Send + Sync>)]
    fn from_sql(bytes: Option<&DB::RawValue>) -> Self {
        String::from_sql(bytes)?.parse()?
    }
}

impl<DB> ToSql<Text, DB> for Compression
    where DB: Backend, String: ToSql<Text, DB>,
{
    #[throws(Box<dyn std::error::Error + Send + Sync>)]
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> IsNull {
        self.to_string().to_sql(out)?
    }
}

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

#[derive(Debug, Queryable)]
pub struct PackageDepends {
    pub id: i32,
    pub package_id: i32,
    pub depends: String
}

#[derive(Debug, Insertable)]
#[table_name="package_depends"]
pub struct NewPackageDepends {
    pub package_id: i32,
    pub depends: String
}

#[derive(Debug, Queryable)]
pub struct PackageProvides {
    pub id: i32,
    pub package_id: i32,
    pub provides: String
}

#[derive(Debug, Insertable)]
#[table_name="package_provides"]
pub struct NewPackageProvides {
    pub package_id: i32,
    pub provides: String
}

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

#[derive(Debug, Clone, Copy, Serialize, FromSqlRow, AsExpression)]
#[sql_type = "Text"]
pub enum RepoActionOp {
    Add, Remove
}

impl FromStr for RepoActionOp {
    type Err = Error;
    #[throws]
    fn from_str(string: &str) -> Self {
        match string {
            "add" => RepoActionOp::Add,
            "remove" => RepoActionOp::Remove,
            _ => Err(Error)?
        }
    }
}

impl fmt::Display for RepoActionOp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", match self {
            RepoActionOp::Add => "add",
            RepoActionOp::Remove => "remove",
        })
    }
}

impl<DB> FromSql<Text, DB> for RepoActionOp
    where DB: Backend, String: FromSql<Text, DB>,
{
    #[throws(Box<dyn std::error::Error + Send + Sync>)]
    fn from_sql(bytes: Option<&DB::RawValue>) -> Self {
        String::from_sql(bytes)?.parse()?
    }
}

impl<DB> ToSql<Text, DB> for RepoActionOp
    where DB: Backend, String: ToSql<Text, DB>,
{
    #[throws(Box<dyn std::error::Error + Send + Sync>)]
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> IsNull {
        self.to_string().to_sql(out)?
    }
}

#[derive(Debug, Serialize, Queryable)]
pub struct RepoAction {
    pub id: i32,
    pub package_id: i32,
    pub action: RepoActionOp,
    pub worker: Option<String>
}

#[derive(Debug, Serialize, Insertable)]
#[table_name="repo_action"]
pub struct NewRepoAction {
    pub package_id: i32,
    pub action: RepoActionOp
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
