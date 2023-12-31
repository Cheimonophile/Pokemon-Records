use serde;
use sqlx;
use tauri::{self, async_runtime::block_on};

use crate::{
    error::{PkmnResult, StringError},
    pkmndb::{Create, Delete, Read},
    state, util,
};

use super::{
    battle_type::BattleType, catch_type::CatchType, location::Location, playthrough::Playthrough,
    team_member::TeamMember, trainer::Trainer,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BattleEvent {
    pub battle_type: BattleType,
    pub opponent1: Trainer,
    pub opponent2: Option<Trainer>,
    pub partner: Option<Trainer>,
    pub lost: bool,
    pub round: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CatchEvent {
    pub catch_type: CatchType,
    pub team_member: TeamMember,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Event {
    no: i64,
    playthrough: Playthrough,
    location: Location,
    date: String,

    // subtypes
    battle: Option<BattleEvent>,
    catch: Option<CatchEvent>,
    // item: ItemEvent,
}

pub struct RawEvent {
    no: i64,
    playthrough_id_no: String,
    location_id: i64,
    date: String,

    // battle
    battle_type_id: Option<i64>,
    opponent1_id: Option<i64>,
    opponent2_id: Option<i64>,
    partner_id: Option<i64>,
    lost: Option<bool>,
    round: Option<i64>,

    // catch
    catch_type_id: Option<i64>,
    team_member_id: Option<i64>,
}

impl Read for Event {
    type Raw = RawEvent;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.no
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let raw_events = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT
                        event.no AS no,
                        event.playthrough_id_no AS playthrough_id_no,
                        event.location_id AS location_id,
                        event.date AS date,
                        battle_event.battle_type_id AS battle_type_id,
                        battle_event.opponent1_id AS opponent1_id,
                        battle_event.opponent2_id AS opponent2_id,
                        battle_event.partner_id AS partner_id,
                        battle_event.lost AS lost,
                        battle_event.round AS round,
                        catch_event.catch_type_id AS catch_type_id,
                        catch_event.team_member_id AS team_member_id
                    FROM event
                    LEFT JOIN battle_event ON event.no = battle_event.no
                    LEFT JOIN catch_event ON event.no = catch_event.no
                "#,
            )
            .fetch_all(&mut *transaction),
        )?;

        let playthroughs = Playthrough::get_map(transaction)?;
        let locations = Location::get_map(transaction)?;
        let battle_types = BattleType::get_map(transaction)?;
        let trainers = Trainer::get_map(transaction)?;
        let catch_types = CatchType::get_map(transaction)?;
        let team_members = TeamMember::get_map(transaction)?;

        let events = raw_events
            .into_iter()
            .map(|raw_event| {
                let battle =
                    if let (Some(battle_type_id), Some(opponent1_id), Some(lost), Some(round)) = (
                        raw_event.battle_type_id,
                        raw_event.opponent1_id,
                        raw_event.lost,
                        raw_event.round,
                    ) {
                        Some(BattleEvent {
                            battle_type: battle_types
                                .get(&battle_type_id)
                                .ok_or_else(|| {
                                    StringError::new(&format!(
                                        "No battle type found with id {}",
                                        battle_type_id
                                    ))
                                })?
                                .clone(),
                            opponent1: trainers
                                .get(&opponent1_id)
                                .ok_or_else(|| {
                                    StringError::new(&format!(
                                        "No trainer found with id {}",
                                        opponent1_id
                                    ))
                                })?
                                .clone(),
                            opponent2: if let Some(opponent2_id) = raw_event.opponent2_id {
                                Some(
                                    trainers
                                        .get(&opponent2_id)
                                        .ok_or_else(|| {
                                            StringError::new(&format!(
                                                "No trainer found with id {}",
                                                opponent2_id
                                            ))
                                        })?
                                        .clone(),
                                )
                            } else {
                                None
                            },
                            partner: if let Some(partner_id) = raw_event.partner_id {
                                Some(
                                    trainers
                                        .get(&partner_id)
                                        .ok_or_else(|| {
                                            StringError::new(&format!(
                                                "No trainer found with id {}",
                                                partner_id
                                            ))
                                        })?
                                        .clone(),
                                )
                            } else {
                                None
                            },
                            lost,
                            round,
                        })
                    } else {
                        None
                    };

                let catch = if let (Some(catch_type_id), Some(team_member_id)) =
                    (raw_event.catch_type_id, raw_event.team_member_id)
                {
                    Some(CatchEvent {
                        catch_type: catch_types
                            .get(&catch_type_id)
                            .ok_or_else(|| {
                                StringError::new(&format!(
                                    "No catch type found with id {}",
                                    catch_type_id
                                ))
                            })?
                            .clone(),
                        team_member: team_members
                            .get(&team_member_id)
                            .ok_or_else(|| {
                                StringError::new(&format!(
                                    "No team member found with id {}",
                                    team_member_id
                                ))
                            })?
                            .clone(),
                    })
                } else {
                    None
                };

                Ok(Event {
                    no: raw_event.no,
                    playthrough: playthroughs
                        .get(&raw_event.playthrough_id_no)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "No playthrough found with id {}",
                                raw_event.playthrough_id_no
                            ))
                        })?
                        .clone(),
                    location: locations
                        .get(&raw_event.location_id)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "No location found with id {}",
                                raw_event.location_id
                            ))
                        })?
                        .clone(),
                    date: raw_event.date,
                    battle,
                    catch,
                })
            })
            .collect::<PkmnResult<Vec<Event>>>()?;

        Ok(events)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateBattleEvent {
    battle_type_id: i64,
    opponent1_id: i64,
    opponent2_id: Option<i64>,
    partner_id: Option<i64>,
    lost: bool,
    round: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateCatchEvent {
    catch_type_id: i64,
    team_member_id: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateEvent {
    playthrough_id_no: String,
    location_id: i64,
    date: String,
    battle: Option<CreateBattleEvent>,
    catch: Option<CreateCatchEvent>,
}

impl Create for Event {
    type Create = CreateEvent;
    fn create(transaction: &mut sqlx::SqliteConnection, raw: &Self::Create) -> PkmnResult<i64> {
        let util::No { no } = block_on(
            sqlx::query_as!(
                util::No,
                r#"
                    INSERT INTO event (playthrough_id_no, location_id, date)
                    VALUES (?, ?, ?)
                    RETURNING no;
                "#,
                raw.playthrough_id_no,
                raw.location_id,
                raw.date
            )
            .fetch_one(&mut *transaction),
        )?;

        if let Some(battle) = &raw.battle {
            block_on(
                sqlx::query!(
                    r#"
                        INSERT INTO battle_event (no, battle_type_id, opponent1_id, opponent2_id, partner_id, lost, round)
                        VALUES (?, ?, ?, ?, ?, ?, ?);
                    "#,
                    no,
                    battle.battle_type_id,
                    battle.opponent1_id,
                    battle.opponent2_id,
                    battle.partner_id,
                    battle.lost,
                    battle.round
                )
                .fetch_one(&mut *transaction),
            )?;
        }

        if let Some(catch) = &raw.catch {
            block_on(
                sqlx::query!(
                    r#"
                        INSERT INTO catch_event (no, catch_type_id, team_member_id)
                        VALUES (?, ?, ?);
                    "#,
                    no,
                    catch.catch_type_id,
                    catch.team_member_id
                )
                .fetch_one(&mut *transaction),
            )?;
        }
        Ok(no)
    }
}

impl Delete for Event {
    fn delete(transaction: &mut sqlx::SqliteConnection, no: &Self::Key) -> PkmnResult<()> {
        block_on(
            sqlx::query!(
                r#"
                    DELETE FROM battle_event
                    WHERE no = ?;
                "#,
                no
            )
            .fetch_one(&mut *transaction),
        )
        .ok();
        block_on(
            sqlx::query!(
                r#"
                    DELETE FROM catch_event
                    WHERE no = ?;
                "#,
                no
            )
            .fetch_one(&mut *transaction),
        )
        .ok();
        block_on(
            sqlx::query!(
                r#"
                    DELETE FROM event
                    WHERE no = ?;
                "#,
                no
            )
            .fetch_one(&mut *transaction),
        )?;
        Ok(())
    }
}

#[tauri::command]
pub fn create_event(state: tauri::State<state::GameState>, event: CreateEvent) -> PkmnResult<i64> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let no = Event::create(&mut *transaction, &event)?;
    transaction.commit()?;
    Ok(no)
}

/**
 * Read all events
 */
#[tauri::command]
pub async fn read_events(
    state: tauri::State<'_, state::GameState>,
    playthrough_id_no: Option<String>,
) -> PkmnResult<Vec<Event>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let events = Event::read(&mut *transaction)?
        .into_iter()
        .filter(|event| {
            if let Some(playthrough_id_no) = &playthrough_id_no {
                event.playthrough.id_no == *playthrough_id_no
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
    Ok(events)
}

/**
 * Delete an event
 */
#[tauri::command]
pub fn delete_event(state: tauri::State<state::GameState>, event_no: i64) -> PkmnResult<()> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    Event::delete(&mut *transaction, &event_no)?;
    transaction.commit()?;
    Ok(())
}
