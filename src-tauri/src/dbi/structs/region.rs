use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::region)]
pub struct InsertRegion<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::region)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Region {
    pub id: i32,
    pub name: String,
}
