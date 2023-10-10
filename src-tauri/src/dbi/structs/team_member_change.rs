use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::Team_Member_Change)]
pub struct InsertTeamMemberChange<'a> {
    pub event_no: &'a i32,
    pub team_member_id: &'a i32,
    pub level: Option<&'a i32>,
    pub species_name: Option<&'a str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::Team_Member_Change)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeamMemberChange {
    pub id: i32,
    pub team_member_id: i32,
    pub event_no: i32,
    pub level: Option<i32>,
    pub species_name: Option<String>,
}
