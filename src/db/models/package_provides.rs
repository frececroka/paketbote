use crate::db::schema::*;

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
