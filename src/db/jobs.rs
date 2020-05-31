use diesel::prelude::*;
use diesel::result::Error;
use fehler::throws;
use serde::Serialize;

use crate::db::models::Job;
use crate::db::models::NewJob;

use super::schema;

#[throws]
pub fn create_job(conn: &PgConnection, tag: String, job: impl Serialize) {
    use schema::job::dsl as j;
    let spec = serde_json::to_value(job).unwrap();
    let job = NewJob { tag, spec };
    diesel::insert_into(j::job)
        .values(job)
        .execute(conn)?;
}

#[throws]
#[allow(unreachable_code)]
pub fn claim_job(conn: &PgConnection, tag: &str, worker: &str) -> Option<Job> {
    use schema::job::dsl as j;
    loop {
        let job = match get_job(conn, tag)? {
            Some(job) => job,
            None => return None
        };
        let job = diesel::update(j::job)
            .filter(j::id.eq(job.id))
            .filter(j::worker.is_null())
            .set(j::worker.eq(worker))
            .get_result(conn)
            .optional()?;
        match job {
            Some(job) => return Some(job),
            None => {}
        }
    }
}

#[throws]
pub fn get_job(conn: &PgConnection, tag: &str) -> Option<Job> {
    use schema::job::dsl as j;
    j::job
        .filter(j::tag.eq(tag))
        .filter(j::worker.is_null())
        .limit(1)
        .first(conn)
        .optional()?
}

#[throws]
pub fn delete_job(conn: &PgConnection, job_id: i32) {
    use schema::job::dsl as j;
    diesel::delete(j::job)
        .filter(j::id.eq(job_id))
        .execute(conn)?
}
