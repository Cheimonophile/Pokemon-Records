use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::playthrough)]
pub struct InsertPlaythrough<'a> {
    pub id_no: &'a str,
    pub name: &'a str,
    pub version_id: i32,
    pub adventure_started: &'a str,
}

#[derive(serde::Deserialize, serde::Serialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::playthrough)]
#[diesel(primary_key(id_no))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Playthrough {
    pub id_no: String,
    pub name: String,
    pub version_id: i32,
    pub adventure_started: String,
}
