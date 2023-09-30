use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Playthrough)]
pub struct InsertPlaythrough<'a> {
    pub id_no: &'a str,
    pub name: &'a str,
    pub version: &'a str,
    pub adventure_started: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Playthrough)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Playthrough {
    pub id_no: String,
    pub name: String,
    pub version: String,
    pub adventure_started: String,
}

impl std::fmt::Display for Playthrough {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.version, self.adventure_started)
    }
}
