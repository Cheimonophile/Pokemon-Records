use std::collections::HashMap;

use diesel::{prelude::*, QueryDsl, QueryResult};

use crate::{
    dbi::structs::{
        species::Species, team_member::TeamMember, team_member_change::TeamMemberChange,
    },
    error::PkmnResult,
    schema, state,
};

pub type TeamOverTimeResult = Vec<Vec<TeamMemberInstant>>;

#[derive(serde::Serialize, Clone)]
pub struct TeamMemberInstant {
    team_member: TeamMember,
    species: Option<Species>,
    level: Option<i32>,
}

#[tauri::command]
pub fn team_over_time(
    state: tauri::State<state::GameState>,
    playthrough_id_no: &str,
) -> PkmnResult<TeamOverTimeResult> {
    let db_results = state.transact(|connection| {
        let results = schema::Team_Member_Change::table
            .inner_join(schema::Event::table)
            .inner_join(schema::Team_Member::table)
            .left_join(schema::Species::table)
            .filter(schema::Event::playthrough_id_no.eq(playthrough_id_no))
            .order(schema::Team_Member_Change::id.asc())
            .select((
                schema::Team_Member_Change::all_columns,
                schema::Team_Member::all_columns,
                Option::<Species>::as_select(),
            ))
            .load::<(TeamMemberChange, TeamMember, Option<Species>)>(connection)?;
        QueryResult::<Vec<(TeamMemberChange, TeamMember, Option<Species>)>>::Ok(results)
    })?;
    pub type FoldV = Vec<(i32, HashMap<i32, TeamMemberInstant>)>;
    let results = db_results.iter().fold(
        vec![],
        |mut sum: FoldV, (team_member_change, team_member, species)| {
            // make sure vec is inited
            let (no, mut instant) = if let Some((i, instant)) = sum.pop() {
                if i == team_member_change.event_no {
                    (i, instant)
                } else {
                    let new_instant = instant.clone();
                    sum.push((i, instant));
                    (team_member_change.event_no, new_instant)
                }
            } else {
                (team_member_change.event_no, HashMap::new())
            };

            // populate species entry
            if let Some(team_member_instant) = instant.get_mut(&team_member.id) {
                if let Some(species) = species {
                    team_member_instant.species = Some(species.clone());
                }
                if let Some(level) = team_member_change.level {
                    team_member_instant.level = Some(level);
                }
            } else {
                instant.insert(
                    team_member.id,
                    TeamMemberInstant {
                        team_member: team_member.clone(),
                        species: species.clone(),
                        level: team_member_change.level,
                    },
                );
            };

            // push the event
            sum.push((no, instant));
            sum
        },
    );

    // flattened
    let flattened = results
        .iter()
        .map(|(_i, instants)| {
            let instants = instants
                .into_iter()
                .map(|(_k, instant)| instant.to_owned())
                .collect::<Vec<TeamMemberInstant>>();
            instants
        })
        .collect::<Vec<Vec<TeamMemberInstant>>>();

    Ok(flattened)
}
