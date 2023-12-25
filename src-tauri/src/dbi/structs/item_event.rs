use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::item_event)]
pub struct InsertItemEvent<'a> {
    pub no: &'a i32,
    pub item_id: &'a i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::item_event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ItemEvent {
    pub no: i32,
    pub item_id: i32,
}
