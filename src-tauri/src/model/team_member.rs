use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::team_member)]
pub struct InsertTeamMember<'a> {
    pub playthrough_id_no: &'a str,
    pub slot: i32,
    pub nickname: Option<&'a str>,
    pub caught_date: &'a str,
    pub caught_location_id: i32,
    pub caught_species_id: i32,
    pub caught_level: i32,
    pub ball_id: i32,
    pub gender: &'a str,
}

#[derive(serde::Serialize, Queryable, Selectable, Clone)]
#[diesel(table_name = schema::team_member)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeamMember {
    pub id: i32,
    pub playthrough_id_no: String,
    pub slot: i32,
    pub nickname: Option<String>,
    pub caught_date: String,
    pub caught_location_id: i32,
    pub caught_species_id: i32,
    pub caught_level: i32,
    pub ball_id: i32,
    pub gender: String,
}
