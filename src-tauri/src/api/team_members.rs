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
        let mut query = schema::Team_Member::table
            .inner_join(schema::Playthrough::table)
            .into_boxed();
        if let Some(playthrough_id_no) = playthrough_id_no {
            query = query.filter(schema::Team_Member::playthrough_id_no.eq(playthrough_id_no));
        };
        let team_members = query
            .select((
                schema::Team_Member::all_columns,
                schema::Playthrough::all_columns,
                schema::Team_Member_Change::table
                    .filter(
                        schema::Team_Member_Change::team_member_id
                            .eq(schema::Team_Member::id)
                            .and(schema::Team_Member_Change::level.is_not_null()),
                    )
                    .order(schema::Team_Member_Change::id.desc())
                    .select(schema::Team_Member_Change::level)
                    .single_value(),
            ))
            .load::<(TeamMember, Playthrough, Option<i32>)>(connection)?;
        let team_member_results = team_members
            .into_iter()
            .map(|(team_member, playthrough, level)| {
                let species = schema::Team_Member_Change::table
                    .inner_join(schema::Species::table)
                    .filter(
                        schema::Team_Member_Change::team_member_id
                            .eq(team_member.id)
                            .and(schema::Team_Member_Change::species_name.is_not_null()),
                    )
                    .order(schema::Team_Member_Change::id.desc())
                    .select(schema::Species::all_columns)
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
