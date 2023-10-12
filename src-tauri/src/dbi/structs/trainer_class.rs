use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Trainer_Class)]
pub struct InsertTrainerClass<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Trainer_Class)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TrainerClass {
    pub name: String,
}

impl std::fmt::Display for TrainerClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
