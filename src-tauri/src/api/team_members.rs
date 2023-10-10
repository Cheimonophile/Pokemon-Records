use diesel::prelude::*;

use crate::{
    dbi::{
        self,
        structs::{playthrough::Playthrough, species::Species, team_member::TeamMember},
    },
    error::PkmnResult,
    schema,
};

#[derive(serde::Serialize)]
pub struct ReadTeamMembersResult {
    #[serde(flatten)]
    team_member: TeamMember,
    playthrough: Playthrough,
    level: i32,
}

type ReadTeamMembersResults = Vec<ReadTeamMembersResult>;

#[tauri::command]
pub fn read_team_members(playthrough_id_no: Option<String>) -> PkmnResult<ReadTeamMembersResults> {
    let team_members = dbi::connection::connect()?.transaction(|connection| {
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
        QueryResult::<Vec<(TeamMember, Playthrough, Option<i32>)>>::Ok(team_members)
    })?;
    let team_member_results = team_members
        .into_iter()
        .map(|(team_member, playthrough, level)| ReadTeamMembersResult {
            team_member,
            playthrough,
            level: level.unwrap_or(0),
        })
        .collect::<Vec<ReadTeamMembersResult>>();
    Ok(team_member_results)
}
