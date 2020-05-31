use diesel::PgConnection;
use fehler::throws;
use serde::Deserialize;
use serde::Serialize;

use crate::db::claim_job;
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
    if let Some(job) = claim_job(conn, "repo_action", worker)? {
        Some((job.id, serde_json::from_value(job.spec)?))
    } else {
        None
    }
}
