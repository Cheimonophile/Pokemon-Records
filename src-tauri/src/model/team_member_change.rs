use tauri::async_runtime::block_on;

use crate::{
    error::{PkmnResult, StringError},
    pkmndb::{Create, Read, Update},
    util,
};

use super::{species::Species, team_member::TeamMember};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TeamMemberChange {
    pub id: i64,
    pub event_no: i64,
    pub team_member: TeamMember,
    pub level: Option<i64>,
    pub species: Option<Species>,
}

pub struct RawTeamMemberChange {
    pub no: i64,
    pub event_no: i64,
    pub team_member_id: i64,
    pub level: Option<i64>,
    pub species_id: Option<i64>,
}

impl Read for TeamMemberChange {
    type Key = i64;
    type Raw = RawTeamMemberChange;
    fn id(&self) -> Self::Key {
        self.id
    }

    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let raw_team_member_change = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT team_member_change.*
                    FROM team_member_change
                "#,
            )
            .fetch_all(&mut *transaction),
        )?;

        let team_members = TeamMember::get_map(transaction)?;
        let species = Species::get_map(transaction)?;

        let team_member_changes = raw_team_member_change
            .into_iter()
            .map(|raw_team_member_change| {
                Ok(TeamMemberChange {
                    id: raw_team_member_change.no,
                    event_no: raw_team_member_change.event_no,
                    team_member: team_members
                        .get(&raw_team_member_change.team_member_id)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "Team member with id {} not found",
                                raw_team_member_change.team_member_id
                            ))
                        })?
                        .clone(),
                    level: raw_team_member_change.level,
                    species: if let Some(species_id) = raw_team_member_change.species_id {
                        Some(
                            species
                                .get(&species_id)
                                .ok_or_else(|| {
                                    StringError::new(&format!(
                                        "Species with id {} not found",
                                        species_id
                                    ))
                                })?
                                .clone(),
                        )
                    } else {
                        None
                    },
                })
            })
            .collect::<PkmnResult<Vec<Self>>>()?;

        Ok(team_member_changes)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamMemberChange {
    pub event_no: i64,
    pub team_member_id: i64,
    pub level: Option<i64>,
    pub species_id: Option<i64>,
}

impl Create for TeamMemberChange {
    type Create = CreateTeamMemberChange;
    fn create(
        transaction: &mut sqlx::SqliteConnection,
        create: &Self::Create,
    ) -> PkmnResult<Self::Key> {
        let util::No { no } = block_on(
            sqlx::query_as!(
                util::No,
                r#"
                    INSERT INTO team_member_change (event_no, team_member_id, level, species_id)
                    VALUES (?, ?, ?, ?)
                    RETURNING no
                "#,
                create.event_no,
                create.team_member_id,
                create.level,
                create.species_id,
            )
            .fetch_one(transaction),
        )?;
        Ok(no)
    }
}

impl Update for TeamMemberChange {
    fn update(
        transaction: &mut sqlx::SqliteConnection,
        no: &Self::Key,
        update: &Self::Create,
    ) -> PkmnResult<()> {
        block_on(
            sqlx::query!(
                r#"
                    UPDATE team_member_change
                    SET event_no = ?, team_member_id = ?, level = ?, species_id = ?
                    WHERE no = ?
                "#,
                update.event_no,
                update.team_member_id,
                update.level,
                update.species_id,
                no,
            )
            .fetch_one(transaction),
        )?;
        Ok(())
    }
}

#[tauri::command]
pub fn create_team_member_change(
    state: tauri::State<crate::state::GameState>,
    team_member_change: CreateTeamMemberChange,
) -> PkmnResult<i64> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let no = TeamMemberChange::create(&mut *transaction, &team_member_change)?;
    transaction.commit()?;
    Ok(no)
}
