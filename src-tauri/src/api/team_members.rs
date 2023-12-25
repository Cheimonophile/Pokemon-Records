use diesel::prelude::*;

use crate::{
    dbi::structs::{playthrough::Playthrough, species::Species, team_member::TeamMember},
    error::PkmnResult,
    schema, state,
};

#[derive(serde::Serialize)]
pub struct ReadTeamMembersResult {
    #[serde(flatten)]
    team_member: TeamMember,
    playthrough: Playthrough,
    level: i32,
    species: Species,
}

type ReadTeamMembersResults = Vec<ReadTeamMembersResult>;

#[tauri::command]
pub fn read_team_members(
    state: tauri::State<state::GameState>,
    playthrough_id_no: Option<String>,
) -> PkmnResult<ReadTeamMembersResults> {
    let team_members = state.transact(|connection| {
        let mut query = schema::team_member::table
            .inner_join(schema::playthrough::table)
            .into_boxed();
        if let Some(playthrough_id_no) = playthrough_id_no {
            query = query.filter(schema::team_member::playthrough_id_no.eq(playthrough_id_no));
        };
        let team_members = query
            .select((
                schema::team_member::all_columns,
                schema::playthrough::all_columns,
                schema::team_member_change::table
                    .filter(
                        schema::team_member_change::team_member_id
                            .eq(schema::team_member::id)
                            .and(schema::team_member_change::level.is_not_null()),
                    )
                    .order(schema::team_member_change::no.desc())
                    .select(schema::team_member_change::level)
                    .single_value(),
            ))
            .load::<(TeamMember, Playthrough, Option<i32>)>(connection)?;
        let team_member_results = team_members
            .into_iter()
            .map(|(team_member, playthrough, level)| {
                let species = schema::team_member_change::table
                    .inner_join(schema::species::table)
                    .filter(
                        schema::team_member_change::team_member_id
                            .eq(team_member.id)
                            .and(schema::team_member_change::species_id.is_not_null()),
                    )
                    .order(schema::team_member_change::no.desc())
                    .select(schema::species::all_columns)
                    .first::<Species>(connection)?;

                QueryResult::<ReadTeamMembersResult>::Ok(ReadTeamMembersResult {
                    team_member,
                    playthrough,
                    level: level.unwrap_or(0),
                    species,
                })
            })
            .collect::<QueryResult<Vec<ReadTeamMembersResult>>>()?;
        QueryResult::<Vec<ReadTeamMembersResult>>::Ok(team_member_results)
    })?;
    Ok(team_members)
}
