use crate::db::schema::*;

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
