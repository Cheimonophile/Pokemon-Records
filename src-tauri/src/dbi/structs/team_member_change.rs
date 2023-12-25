use diesel::prelude::*;

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::team_member_change)]
pub struct InsertTeamMemberChange {
    pub event_no: i32,
    pub team_member_id: i32,
    pub level: Option<i32>,
    pub species_id: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::team_member_change)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeamMemberChange {
    pub no: i32,
    pub team_member_id: i32,
    pub event_no: i32,
    pub level: Option<i32>,
    pub species_id: Option<i32>,
}
