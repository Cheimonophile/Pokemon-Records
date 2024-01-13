use std::collections::HashMap;

use tauri::async_runtime::block_on;

use crate::{
    error::PkmnResult,
    model::{species::Species, team_member::TeamMember},
    pkmndb::Read,
    state,
};

pub type TeamOverTimeResult = Vec<Vec<TeamMemberInstant>>;

#[derive(serde::Serialize, Clone)]
pub struct TeamMemberInstant {
    team_member: TeamMember,
    species: Option<Species>,
    level: Option<i64>,
}

pub struct QueryResult {
    event_no: i64,
    level: Option<i64>,
    team_member_id: Option<i64>,
    species_id: Option<i64>,
}

#[tauri::command]
pub fn team_over_time(
    state: tauri::State<state::GameState>,
    playthrough_id_no: &str,
) -> PkmnResult<TeamOverTimeResult> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;

    let db_results = block_on(
        sqlx::query_as!(
            QueryResult,
            r#"
                SELECT
                event.no AS event_no,
                team_member_change.team_member_id AS team_member_id,
                team_member_change.level AS level,
                team_member_change.species_id AS species_id
                FROM event
                LEFT JOIN team_member_change ON team_member_change.event_no = event.no
                WHERE event.playthrough_id_no = ?
                GROUP BY team_member_change.no, event.no, team_member_change.team_member_id
                ORDER BY event.no, team_member_change.no
            "#,
            playthrough_id_no
        )
        .fetch_all(&mut *transaction),
    )?;

    let team_members = TeamMember::get_map(&mut *transaction)?;
    let species = Species::get_map(&mut *transaction)?;

    pub type FoldV = Vec<(i64, HashMap<i64, TeamMemberInstant>)>;
    let results = db_results
        .into_iter()
        .fold(vec![], |mut sum: FoldV, db_result| {
            let QueryResult {
                event_no,
                level,
                team_member_id,
                species_id,
            } = db_result;
            let team_member = team_member_id.and_then(|id| team_members.get(&id).cloned());
            let species = species_id.and_then(|id| species.get(&id).cloned());

            // make sure vec is inited
            let (no, mut instant) = if let Some((i, instant)) = sum.pop() {
                if i == event_no {
                    (i, instant)
                } else {
                    let new_instant = instant.clone();
                    sum.push((i, instant));
                    (event_no, new_instant)
                }
            } else {
                (event_no, HashMap::new())
            };

            // populate species entry
            if let Some(team_member) = team_member {
                if let Some(team_member_instant) = instant.get_mut(&team_member.id) {
                    if let Some(species) = species {
                        team_member_instant.species = Some(species);
                    }
                    if let Some(level) = level {
                        team_member_instant.level = Some(level);
                    }
                } else {
                    instant.insert(
                        team_member.id,
                        TeamMemberInstant {
                            team_member: team_member.clone(),
                            species: species.clone(),
                            level,
                        },
                    );
                };
            }

            // push the event
            sum.push((no, instant));
            sum
        });

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
