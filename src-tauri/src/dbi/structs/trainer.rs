use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Trainer)]
pub struct InsertTrainer<'a> {
    pub name: Option<&'a str>,
    pub class: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Trainer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Trainer {
    pub name: Option<String>,
    pub class: String,
}

impl Trainer {
    pub fn format(&self, _: &mut SqliteConnection) -> String {
        let mut display = self.class.clone();
        if let Some(name) = &self.name {
            display.push(' ');
            display.push_str(name);
        }
        display
    }
}
