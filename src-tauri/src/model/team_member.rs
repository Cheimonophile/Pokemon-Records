use tauri::async_runtime::block_on;

use crate::{
    error::{PkmnResult, StringError},
    pkmndb::Read,
    state,
};

use super::{ball::Ball, location::Location, playthrough::Playthrough, species::Species};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TeamMember {
    pub id: i64,
    pub playthrough: Playthrough,
    pub slot: i64,
    pub nickname: Option<String>,
    pub caught_date: String,
    pub caught_location: Location,
    pub caught_species: Species,
    pub caught_level: i64,
    pub ball: Ball,
    pub gender: String,

    // flow fields
    pub species: Species,
    pub level: i64,
}

pub struct RawTeamMember {
    pub id: i64,
    pub playthrough_id_no: String,
    pub slot: i64,
    pub nickname: Option<String>,
    pub caught_date: String,
    pub caught_location_id: i64,
    pub caught_species_id: i64,
    pub caught_level: i64,
    pub ball_id: i64,
    pub gender: String,

    // flow fields
    pub species_id: Option<i64>,
    pub level: Option<i64>,
}

impl Read for TeamMember {
    type Key = i64;
    type Raw = RawTeamMember;
    fn id(&self) -> Self::Key {
        self.id
    }

    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let raw_team_member = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT
                        team_member.*,
                        (
                            SELECT species_id
                            FROM team_member_change
                            WHERE team_member_change.team_member_id = team_member.id AND team_member_change.species_id IS NOT NULL
                            ORDER BY team_member_change.no DESC
                            LIMIT 1
                        ) AS species_id,
                        (
                            SELECT level
                            FROM team_member_change
                            WHERE team_member_change.team_member_id = team_member.id AND team_member_change.level IS NOT NULL
                            ORDER BY team_member_change.no DESC
                            LIMIT 1
                        ) AS level
                    FROM team_member
                "#,
            )
            .fetch_all(&mut *transaction),
        )?;

        let playthroughs = Playthrough::get_map(transaction)?;
        let locations = Location::get_map(transaction)?;
        let species = Species::get_map(transaction)?;
        let balls = Ball::get_map(transaction)?;

        let team_members = raw_team_member
            .into_iter()
            .map(|raw_team_member| {
                let caught_species =
                    species
                        .get(&raw_team_member.caught_species_id)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "No species found with id {}",
                                raw_team_member.caught_species_id
                            ))
                        })?;
                Ok(TeamMember {
                    id: raw_team_member.id,
                    playthrough: playthroughs
                        .get(&raw_team_member.playthrough_id_no)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "No playthrough found with id {}",
                                raw_team_member.playthrough_id_no
                            ))
                        })?
                        .clone(),
                    slot: raw_team_member.slot,
                    nickname: raw_team_member.nickname,
                    caught_date: raw_team_member.caught_date,
                    caught_location: locations
                        .get(&raw_team_member.caught_location_id)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "No location found with id {}",
                                raw_team_member.caught_location_id
                            ))
                        })?
                        .clone(),
                    caught_species: caught_species.clone(),
                    caught_level: raw_team_member.caught_level,
                    ball: balls
                        .get(&raw_team_member.ball_id)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "No ball found with id {}",
                                raw_team_member.ball_id
                            ))
                        })?
                        .clone(),
                    gender: raw_team_member.gender,
                    species: raw_team_member
                        .species_id
                        .and_then(|species_id| species.get(&species_id).cloned())
                        .unwrap_or_else(|| caught_species.clone()),
                    level: raw_team_member
                        .level
                        .unwrap_or(raw_team_member.caught_level),
                })
            })
            .collect::<PkmnResult<Vec<TeamMember>>>()?;
        Ok(team_members)
    }
}

#[tauri::command]
pub fn read_team_members(
    state: tauri::State<state::GameState>,
    playthrough_id_no: Option<String>,
) -> PkmnResult<Vec<TeamMember>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let team_members = TeamMember::read(&mut *transaction)?
        .into_iter()
        .filter(|team_member| {
            if let Some(playthrough_id_no) = &playthrough_id_no {
                team_member.playthrough.id_no == *playthrough_id_no
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
    Ok(team_members)
}
