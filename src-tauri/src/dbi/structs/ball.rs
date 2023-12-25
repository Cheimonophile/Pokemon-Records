


use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::ball)]
pub struct InsertType<'a> {
    pub name: &'a str,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::ball)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Ball {
    pub id: i32,
    pub name: String,
}