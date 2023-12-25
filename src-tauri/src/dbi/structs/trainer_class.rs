use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::trainer_class)]
pub struct InsertTrainerClass<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::trainer_class)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TrainerClass {
    pub id: i32,
    pub name: String,
}
