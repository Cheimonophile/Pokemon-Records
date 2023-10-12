use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Event)]
pub struct InsertEvent<'a> {
    pub playthrough_id_no: &'a str,
    pub location_name: &'a str,
    pub location_region: &'a str,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::Event)]
#[diesel(primary_key(no))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Event {
    pub no: i32,
    pub playthrough_id_no: String,
    pub location_name: String,
    pub location_region: String,
}
