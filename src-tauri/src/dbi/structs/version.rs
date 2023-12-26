




use diesel::prelude::*;

use crate::schema;

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::version)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Version {
    pub id: i32,
    pub name: String,
    pub generation: i32,
}