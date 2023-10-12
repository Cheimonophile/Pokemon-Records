use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Item_Event)]
pub struct InsertItemEvent<'a> {
    pub no: &'a i32,
    pub item: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Item_Event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ItemEvent {
    pub no: i32,
    pub item: String,
}

impl std::fmt::Display for ItemEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.item)
    }
}
