use diesel::prelude::*;

use serde;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::region)]
pub struct InsertRegion<'a> {
    pub name: &'a str,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::region)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Region {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct RegionResult {
    #[serde(flatten)]
    region: Region,
}
