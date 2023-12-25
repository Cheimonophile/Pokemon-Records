use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::location)]
pub struct InsertLocation<'a> {
    pub name: &'a str,
    pub region_id: i32,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::location)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub region_id: i32,
}
