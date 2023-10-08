use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Region)]
pub struct InsertRegion<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Region)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Region {
    pub name: String,
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
