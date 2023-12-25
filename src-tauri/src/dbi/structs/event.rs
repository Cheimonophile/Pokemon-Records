use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::event)]
pub struct InsertEvent<'a> {
    pub playthrough_id_no: &'a str,
    pub location_id: i32,
    pub date: &'a str,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::event)]
#[diesel(primary_key(no))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Event {
    pub no: i32,
    pub playthrough_id_no: String,
    pub location_id: i32,
    pub date: String,
}
