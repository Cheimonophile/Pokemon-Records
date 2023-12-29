use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::species)]
pub struct InsertSpecies<'a> {
    pub name: &'a str,
    pub form: Option<&'a str>,
    pub dex_no: &'a i32,
    pub generation: &'a i32,
    pub type1_id: &'a i32,
    pub type2_id: Option<&'a i32>,
}

#[derive(serde::Serialize, Clone)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::species)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Species {
    pub id: i32,
    pub name: String,
    pub form: Option<String>,
    pub dex_no: i32,
    pub generation: i32,
    pub type1_id: i32,
    pub type2_id: Option<i32>,
}

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
