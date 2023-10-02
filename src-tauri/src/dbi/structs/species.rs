use diesel::prelude::*;

use crate::schema::{self, Ball::name};

#[derive(Insertable)]
#[diesel(table_name = schema::Species)]
pub struct InsertSpecies<'a> {
    pub name: &'a str,
    pub dex_no: &'a i32,
    pub generation: &'a i32,
    pub type1: &'a str,
    pub type2: Option<&'a str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Species)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Species {
    pub name: String,
    pub dex_no: i32,
    pub generation: i32,
    pub type1: String,
    pub type2: Option<String>,
}

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
