use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Team_Member_Change)]
pub struct InsertTeamMemberChange<'a> {
    pub event_no: &'a i32,
    pub team_member_playthrough_id_no: &'a str,
    pub team_member_slot: &'a i32,  
    pub level_change: Option<&'a i32>,
    pub species_dex_no: Option<&'a i32>,
    pub species_form: Option<&'a str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Team_Member_Change)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeamMemberChange {
    pub id: i32,
    pub team_member_playthrough_id_no: String,
    pub team_member_slot: i32,
    pub event_no: i32,
    pub level_change: Option<i32>,
    pub species_dex_no: Option<i32>,
    pub species_form: Option<String>,
}
