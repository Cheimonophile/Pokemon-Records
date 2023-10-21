use diesel::prelude::*;

use crate::schema;


#[derive(Insertable)]
#[diesel(table_name = schema::Catch_Event)]
pub struct InsertCatchEvent<'a> {
    pub no: &'a i32,
    pub catch_type: &'a str,
    pub team_member_id: &'a i32,
}

#[derive(serde::Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Catch_Event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CatchEvent {
    pub no: i32,
    pub catch_type: String,
    pub team_member_id: i32,
}
