use diesel::prelude::*;

use crate::schema;


#[derive(Insertable)]
#[diesel(table_name = schema::catch_event)]
pub struct InsertCatchEvent {
    pub no: i32,
    pub catch_type_id: i32,
    pub team_member_id: i32,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::catch_event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CatchEvent {
    pub no: i32,
    pub catch_type_id: i32,
    pub team_member_id: i32,
}
