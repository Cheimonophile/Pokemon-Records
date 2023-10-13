


use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Type)]
pub struct InsertType<'a> {
    pub name: &'a str,
    pub color: &'a str,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Type)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Type {
    pub name: String,
    pub color: String,
}