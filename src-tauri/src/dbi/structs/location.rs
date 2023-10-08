use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Location)]
pub struct InsertLocation<'a> {
    pub name: &'a str,
    pub region: &'a str,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Location)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Location {
    pub name: String,
    pub region: String,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.region)
    }
}
