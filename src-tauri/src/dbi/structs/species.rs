use diesel::prelude::*;

use crate::schema::{self, Ball::name};

#[derive(Insertable)]
#[diesel(table_name = schema::Species)]
pub struct InsertSpecies<'a> {
    pub dex_no: &'a i32,
    pub form: Option<&'a str>,
    pub name: &'a str,
    pub generation: &'a i32,
    pub type1: &'a str,
    pub type2: Option<&'a str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Species)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Species {
    pub dex_no: i32,
    pub name: String,
    pub form: Option<String>,
    pub generation: i32,
    pub type1: String,
    pub type2: Option<String>,
}

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_string: String = self.name.clone();
        if let Some(form) = &self.form {
            display_string.push_str(" (");
            display_string.push_str(form);
            display_string.push_str(")");
        }
        write!(f, "{}", self.name)
    }
}
