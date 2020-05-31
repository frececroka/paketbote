use diesel::PgConnection;
use fehler::throws;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use crate::db;
use crate::db::create_job;
use crate::error::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RepoAction {
    pub package_id: i32,
    pub operation: RepoActionOp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RepoActionOp {
    Add, Remove,
}

impl RepoAction {
    pub fn new_add(package_id: i32) -> RepoAction {
        RepoAction { package_id, operation: RepoActionOp::Add }
    }
    pub fn new_remove(package_id: i32) -> RepoAction {
        RepoAction { package_id, operation: RepoActionOp::Remove }
    }
}

#[throws]
pub fn create_repo_action(conn: &PgConnection, package_id: i32, operation: RepoActionOp) {
    let repo_action = RepoAction { package_id, operation };
    create_job(conn, "repo_action".to_owned(), repo_action)?;
}

#[throws]
pub fn get_repo_action(conn: &PgConnection, worker: &str) -> Option<(i32, RepoAction)> {
    claim_job(conn, "repo_action", worker)?
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CheckDeps {
    pub repo_id: i32
}

#[throws]
pub fn create_check_deps(conn: &PgConnection, repo_id: i32) {
    let check_deps = CheckDeps { repo_id };
    create_job(conn, "check_deps".to_owned(), check_deps)?;
}

#[throws]
pub fn get_check_deps(conn: &PgConnection, worker: &str) -> Option<(i32, CheckDeps)> {
    claim_job(conn, "check_deps", worker)?
}

#[throws]
pub fn claim_job<T: DeserializeOwned>(conn: &PgConnection, tag: &str, worker: &str) -> Option<(i32, T)> {
    if let Some(job) = db::claim_job(conn, tag, worker)? {
        Some((job.id, serde_json::from_value(job.spec)?))
    } else {
        None
    }
}
