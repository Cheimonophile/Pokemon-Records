use diesel::prelude::*;

use crate::schema;

use serde;

#[derive(Insertable)]
#[diesel(table_name = schema::trainer_class)]
pub struct InsertTrainerClass<'a> {
    pub name: &'a str,
}


#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::trainer_class)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TrainerClass {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct TrainerClassResult {
    #[serde(flatten)]
    pub trainer_class: TrainerClass,
}
