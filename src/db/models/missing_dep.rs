use crate::db::schema::*;

#[derive(Debug, Queryable)]
pub struct MissingDep {
    pub id: i32,
    pub repo_id: i32,
    pub dependency: String
}

#[derive(Debug, Insertable)]
#[table_name="missing_dep"]
pub struct NewMissingDep {
    pub repo_id: i32,
    pub dependency: String
}
