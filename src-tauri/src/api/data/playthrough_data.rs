use std::collections::HashMap;

use diesel::{prelude::*, QueryDsl, QueryResult};

use crate::{
    dbi::structs::{
        species::Species, team_member::TeamMember, team_member_change::TeamMemberChange, event::Event,
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
        let results = schema::Event::table
            .left_join(
                schema::Team_Member_Change::table
                    .left_join(schema::Team_Member::table)
                    .left_join(schema::Species::table),
            )
            .filter(schema::Event::playthrough_id_no.eq(playthrough_id_no))
            .order((
                schema::Event::no.asc(),
                schema::Team_Member_Change::id.asc()
            ))
            .select((
                Event::as_select(),
                Option::<TeamMemberChange>::as_select(),
                Option::<TeamMember>::as_select(),
                Option::<Species>::as_select(),
            ))
            .load::<(Event, Option<TeamMemberChange>, Option<TeamMember>, Option<Species>)>(connection)?;
        QueryResult::<Vec<(Event,Option<TeamMemberChange>, Option<TeamMember>, Option<Species>)>>::Ok(results)
    })?;
    pub type FoldV = Vec<(i32, HashMap<i32, TeamMemberInstant>)>;
    let results = db_results.iter().fold(
        vec![],
        |mut sum: FoldV, (event, team_member_change, team_member, species)| {
            // make sure vec is inited
            let (no, mut instant) = if let Some((i, instant)) = sum.pop() {
                if i == event.no {
                    (i, instant)
                } else {
                    let new_instant = instant.clone();
                    sum.push((i, instant));
                    (event.no, new_instant)
                }
            } else {
                (event.no, HashMap::new())
            };

            // populate species entry
            if let (Some(team_member), Some(team_member_change)) = (team_member, team_member_change) {
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
            }
            

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
