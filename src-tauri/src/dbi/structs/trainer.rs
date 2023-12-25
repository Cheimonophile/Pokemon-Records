use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::trainer)]
pub struct InsertTrainer<'a> {
    pub name: &'a str,
    pub class_id: i32,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::trainer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Trainer {
    pub id: i32,
    pub name: String,
    pub class_id: i32,
}
