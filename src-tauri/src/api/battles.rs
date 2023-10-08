use diesel::prelude::*;

use serde;

use crate::{
    dbi::{
        self,
        structs::{
            battle_event::{BattleEvent, InsertBattleEvent},
            event::{Event, InsertEvent},
            playthrough,
        },
    },
    error::PkmnResult,
    schema::{self},
};

#[derive(serde::Serialize)]
pub struct ReadBattlesResult {
    #[serde(flatten)]
    battle: BattleEvent,
    event: Event,
}

#[tauri::command]
pub fn read_battles() -> PkmnResult<Vec<ReadBattlesResult>> {
    let raw_battles =
        dbi::connection::connect().transaction(|connection: &mut SqliteConnection| {
            let raw_battles = schema::Battle_Event::table
                .inner_join(schema::Event::table)
                .order(schema::Battle_Event::no.desc())
                .select((BattleEvent::as_select(), Event::as_select()))
                .load::<(BattleEvent, Event)>(connection)?;
            QueryResult::<Vec<(BattleEvent, Event)>>::Ok(raw_battles)
        })?;
    let battles = raw_battles
        .into_iter()
        .map(|(battle, event)| ReadBattlesResult { battle, event })
        .collect();
    Ok(battles)
}

#[tauri::command]
pub fn create_battle(
    playthrough_id_no: &str,
    location_name: &str,
    location_region: &str,
    battle_type: &str,
    opponent1_name: &str,
    opponent1_class: &str,
    opponent2_name: Option<&str>,
    opponent2_class: Option<&str>,
    partner_name: Option<&str>,
    partner_class: Option<&str>,
    round: i32,
    lost: bool,
) -> PkmnResult<usize> {
    let rows_affected = dbi::connection::connect().transaction(|connection| {
        let new_event = InsertEvent {
            playthrough_id_no: playthrough_id_no,
            location_name: &location_name,
            location_region: &location_region,
        };
        diesel::insert_into(schema::Event::table)
            .values(&new_event)
            .execute(connection)?;
        let event = schema::Event::table
            .filter(schema::Event::playthrough_id_no.eq(&playthrough_id_no))
            .order(schema::Event::no.desc())
            .first::<Event>(connection)
            .expect("Error loading event");
        let new_battle_event = InsertBattleEvent {
            no: &event.no,
            battle_type,
            opponent1_name: opponent1_name,
            opponent1_class: opponent1_class,
            opponent2_name: opponent2_name,
            opponent2_class: opponent2_class,
            partner_name: partner_name,
            partner_class: partner_class,
            round: &round,
            lost: &lost,
        };
        let rows_affected = diesel::insert_into(schema::Battle_Event::table)
            .values(&new_battle_event)
            .execute(connection)?;
        QueryResult::Ok(rows_affected)
    })?;
    Ok(rows_affected)
}
