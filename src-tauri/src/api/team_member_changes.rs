use diesel::prelude::*;

use crate::{
    dbi::structs::team_member_change::InsertTeamMemberChange, error::PkmnResult, schema, state,
};

#[tauri::command]
pub fn create_team_member_change(
    state: tauri::State<state::GameState>,
    event_no: i32,
    team_member_id: i32,
    level: Option<i32>,
    species_name: Option<&str>,
) -> PkmnResult<()> {
    state.transact(|connection| {
        let new_team_member_change = InsertTeamMemberChange {
            event_no: &event_no,
            team_member_id: &team_member_id,
            level: level.as_ref(),
            species_name,
        };
        diesel::insert_into(schema::Team_Member_Change::table)
            .values(&new_team_member_change)
            .execute(connection)?;
        QueryResult::<()>::Ok(())
    })?;
    Ok(())
}
