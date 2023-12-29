use sqlx::prelude::*;

use serde;
use sqlx;
use tauri::{self, async_runtime::block_on};

use crate::{
    error::{PkmnResult, StringError},
    state,
};

use super::{
    battle_event::BattleEvent,
    battle_type::BattleType,
    location::Location,
    playthrough::{self, Playthrough},
    region::Region,
    trainer::Trainer,
    trainer_class::TrainerClass,
    version::Version,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Event {
    no: i64,
    playthrough: Playthrough,
    location: Location,
    date: String,

    // subtypes
    battle: Option<BattleEvent>,
    // catch: CatchEvent,
    // item: ItemEvent,
}

struct EventRaw {
    no: i64,
    playthrough_id_no: String,
    playthrough_name: String,
    playthrough_version_id: i64,
    playthrough_version_name: String,
    playthrough_generation: i64,
    playthrough_adventure_started: String,
    location_id: i64,
    location_name: String,
    location_region_id: i64,
    location_region_name: String,
    date: String,
    battle_no: Option<i64>,
    battle_battle_type_id: Option<i64>,
    battle_battle_type_name: Option<String>,
    battle_opponent1_id: Option<i64>,
    battle_opponent1_name: Option<String>,
    battle_opponent1_trainer_class_id: Option<i64>,
    battle_opponent1_trainer_class_name: Option<String>,
    battle_opponent2_id: Option<i64>,
    battle_opponent2_name: Option<String>,
    battle_opponent2_trainer_class_id: Option<i64>,
    battle_opponent2_trainer_class_name: Option<String>,
    battle_partner_id: Option<i64>,
    battle_partner_name: Option<String>,
    battle_partner_trainer_class_id: Option<i64>,
    battle_partner_trainer_class_name: Option<String>,
    battle_lost: Option<bool>,
    battle_round: Option<i64>,
}

impl Into<PkmnResult<Event>> for EventRaw {
    fn into(self) -> PkmnResult<Event> {
        Ok(Event {
            no: self.no,
            playthrough: Playthrough {
                id_no: self.playthrough_id_no,
                name: self.playthrough_name,
                version: Version {
                    id: self.playthrough_version_id,
                    name: self.playthrough_version_name,
                    generation: self.playthrough_generation,
                },
                adventure_started: self.playthrough_adventure_started,
            },
            location: Location {
                id: self.location_id,
                name: self.location_name,
                region: Region {
                    id: self.location_region_id,
                    name: self.location_region_name,
                },
            },
            date: self.date,
            battle: if self.battle_no.is_none() {
                None
            } else {
                Some(BattleEvent {
                    no: self
                        .battle_no
                        .ok_or(StringError::new("BattleEvent no is None"))?,
                    battle_type: BattleType {
                        id: self
                            .battle_battle_type_id
                            .ok_or(StringError::new("BattleEvent battle_type_id is None"))?,
                        name: self
                            .battle_battle_type_name
                            .ok_or(StringError::new("BattleEvent battle_type_name is None"))?,
                    },
                    opponent1: Trainer {
                        id: self
                            .battle_opponent1_id
                            .ok_or(StringError::new("BattleEvent opponent1_id is None"))?,
                        name: self
                            .battle_opponent1_name
                            .ok_or(StringError::new("BattleEvent opponent1_name is None"))?,
                        class: TrainerClass {
                            id: self
                                .battle_opponent1_trainer_class_id
                                .ok_or(StringError::new(
                                    "BattleEvent opponent1_trainer_class_id is None",
                                ))?,
                            name: self.battle_opponent1_trainer_class_name.ok_or(
                                StringError::new(
                                    "BattleEvent opponent1_trainer_class_name is None",
                                ),
                            )?,
                        },
                    },
                    opponent2: if self.battle_opponent2_id.is_none() {
                        None
                    } else {
                        Some(Trainer {
                            id: self
                                .battle_opponent2_id
                                .ok_or(StringError::new("BattleEvent opponent2_id is None"))?,
                            name: self
                                .battle_opponent2_name
                                .ok_or(StringError::new("BattleEvent opponent2_name is None"))?,
                            class: TrainerClass {
                                id: self.battle_opponent2_trainer_class_id.ok_or(
                                    StringError::new(
                                        "BattleEvent opponent2_trainer_class_id is None",
                                    ),
                                )?,
                                name: self.battle_opponent2_trainer_class_name.ok_or(
                                    StringError::new(
                                        "BattleEvent opponent2_trainer_class_name is None",
                                    ),
                                )?,
                            },
                        })
                    },
                    partner: if self.battle_partner_id.is_none() {
                        None
                    } else {
                        Some(Trainer {
                            id: self
                                .battle_partner_id
                                .ok_or(StringError::new("BattleEvent partner_id is None"))?,
                            name: self
                                .battle_partner_name
                                .ok_or(StringError::new("BattleEvent partner_name is None"))?,
                            class: TrainerClass {
                                id: self.battle_partner_trainer_class_id.ok_or(
                                    StringError::new(
                                        "BattleEvent partner_trainer_class_id is None",
                                    ),
                                )?,
                                name: self.battle_partner_trainer_class_name.ok_or(
                                    StringError::new(
                                        "BattleEvent partner_trainer_class_name is None",
                                    ),
                                )?,
                            },
                        })
                    },
                    lost: self
                        .battle_lost
                        .ok_or(StringError::new("BattleEvent lost is None"))?,
                    round: self
                        .battle_round
                        .ok_or(StringError::new("BattleEvent round is None"))?,
                })
            },
        })
    }
}

/**
 * A battle event with only its id
 */
pub struct EventId {
    pub no: i64,
}

#[tauri::command]
pub fn create_battle_event(
    state: tauri::State<state::GameState>,
    playthrough_id_no: String,
    location_id: i64,
    date: String,
    battle_type_id: i64,
    opponent1_id: i64,
    opponent2_id: Option<i64>,
    partner_id: Option<i64>,
    lost: bool,
    round: i64,
) -> PkmnResult<i64> {
    let mut transaction = state.transaction()?;

    // create the event
    let EventId { no } = block_on(
        sqlx::query_as!(
            EventId,
            r#"
            INSERT INTO event (playthrough_id_no, location_id, date)
            VALUES (?, ?, ?)
            RETURNING no;
        "#,
            playthrough_id_no,
            location_id,
            date
        )
        .fetch_one(&mut *transaction),
    )?;

    // create the battle event
    block_on(sqlx::query!(
        r#"
            INSERT INTO battle_event (no, battle_type_id, opponent1_id, opponent2_id, partner_id, lost, round)
            VALUES (?, ?, ?, ?, ?, ?, ?);
        "#,
        no,
        battle_type_id,
        opponent1_id,
        opponent2_id,
        partner_id,
        lost,
        round
    )
    .fetch_one(&mut *transaction))?;

    block_on(transaction.commit())?;
    Ok(no)
}

#[tauri::command]
pub async fn read_events(
    state: tauri::State<'_, state::GameState>,
    playthrough_id_no: String,
) -> PkmnResult<Vec<Event>> {
    let mut transaction = state.transaction()?;
    let raw_events = block_on(sqlx::query_as!(
        EventRaw,
        r#"
            SELECT
                event.no AS no,
                playthrough.id_no AS playthrough_id_no,
                playthrough.name AS playthrough_name,
                playthrough.version_id AS playthrough_version_id,
                playthrough_version.name AS playthrough_version_name,
                playthrough_version.generation AS playthrough_generation,
                playthrough.adventure_started AS playthrough_adventure_started,
                location.id AS location_id,
                location.name AS location_name,
                location_region.id AS location_region_id,
                location_region.name AS location_region_name,
                event.date AS date,
                battle_event.no AS battle_no,
                battle_type.id AS battle_battle_type_id,
                battle_type.name AS battle_battle_type_name,
                opponent1.id AS battle_opponent1_id,
                opponent1.name AS battle_opponent1_name,
                opponent1_class.id AS battle_opponent1_trainer_class_id,
                opponent1_class.name AS battle_opponent1_trainer_class_name,
                opponent2.id AS battle_opponent2_id,
                opponent2.name AS battle_opponent2_name,
                opponent2_class.id AS battle_opponent2_trainer_class_id,
                opponent2_class.name AS battle_opponent2_trainer_class_name,
                partner.id AS battle_partner_id,
                partner.name AS battle_partner_name,
                partner_trainer_class.id AS battle_partner_trainer_class_id,
                partner_trainer_class.name AS battle_partner_trainer_class_name,
                battle_event.lost AS battle_lost,
                battle_event.round AS battle_round
            FROM event
            LEFT JOIN playthrough ON event.playthrough_id_no = playthrough.id_no
            LEFT JOIN version AS playthrough_version ON playthrough.version_id = playthrough_version.id
            LEFT JOIN location ON event.location_id = location.id
            LEFT JOIN region AS location_region ON location.region_id = location_region.id
            LEFT JOIN battle_event ON event.no = battle_event.no
            LEFT JOIN battle_type ON battle_event.battle_type_id = battle_type.id
            LEFT JOIN trainer opponent1 ON battle_event.opponent1_id = opponent1.id
            LEFT JOIN trainer_class opponent1_class ON opponent1.class_id = opponent1_class.id
            LEFT JOIN trainer opponent2 ON battle_event.opponent2_id = opponent2.id
            LEFT JOIN trainer_class opponent2_class ON opponent2.class_id = opponent2_class.id
            LEFT JOIN trainer partner ON battle_event.partner_id = partner.id
            LEFT JOIN trainer_class partner_trainer_class ON partner.class_id = partner_trainer_class.id
            WHERE playthrough.id_no = ?
            ORDER BY event.date ASC;
        "#,
        playthrough_id_no
    )
    .fetch_all(&mut *transaction))?;

    let events = raw_events
        .into_iter()
        .map(|raw_event| raw_event.into())
        .collect::<PkmnResult<Vec<Event>>>()?;

    Ok(events)
}
