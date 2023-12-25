use diesel::prelude::*;

use serde;

use crate::{
    dbi::structs::{
        battle_event::{BattleEvent, InsertBattleEvent},
        event::{Event, InsertEvent},
    },
    error::PkmnResult,
    schema,
    state,
};

#[derive(serde::Serialize)]
pub struct ReadBattlesResult {
    #[serde(flatten)]
    battle: BattleEvent,
    event: Event,
}

#[tauri::command]
pub fn read_battles(
    state: tauri::State<state::GameState>,
    how_many: Option<i32>,
) -> PkmnResult<Vec<ReadBattlesResult>> {
    let raw_battles = state.transact(|connection: &mut SqliteConnection| {
        let raw_battles = schema::battle_event::table
            .inner_join(schema::event::table)
            .order(schema::battle_event::no.desc())
            .limit(how_many.unwrap_or(500) as i64)
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
    state: tauri::State<state::GameState>,
    playthrough_id_no: &str,
    location_id: i32,
    date: &str,
    battle_type_id: i32,
    opponent1_id: i32,
    opponent2_id: Option<i32>,
    partner_id: Option<i32>,
    round: Option<i32>,
    lost: bool,
) -> PkmnResult<usize> {
    let rows_affected = state.transact(|connection| {
        let new_event = InsertEvent {
            playthrough_id_no,
            location_id,
            date,
        };
        diesel::insert_into(schema::event::table)
            .values(&new_event)
            .execute(connection)?;
        let event = schema::event::table
            .filter(schema::event::playthrough_id_no.eq(&playthrough_id_no))
            .order(schema::event::no.desc())
            .first::<Event>(connection)
            .expect("Error loading event");
        let new_battle_event = InsertBattleEvent {
            no: event.no,
            battle_type_id,
            opponent1_id,
            opponent2_id,
            partner_id,
            round,
            lost,
        };
        let rows_affected = diesel::insert_into(schema::battle_event::table)
            .values(&new_battle_event)
            .execute(connection)?;
        QueryResult::Ok(rows_affected)
    })?;
    Ok(rows_affected)
}

#[tauri::command]
pub fn delete_battle(state: tauri::State<state::GameState>, no: i32) -> PkmnResult<()> {
    state.transact(|connection| {
        diesel::delete(schema::battle_event::table.filter(schema::battle_event::no.eq(no)))
            .execute(connection)?;
        diesel::delete(schema::event::table.filter(schema::event::no.eq(no)))
            .execute(connection)?;
        QueryResult::<()>::Ok(())
    })?;
    Ok(())
}

#[tauri::command]
pub fn update_battle(
    state: tauri::State<state::GameState>,
    no: i32,
    lost: Option<bool>,
) -> PkmnResult<()> {
    state.transact(move |connection| {
        if let Some(lost) = lost {
            diesel::update(schema::battle_event::table.filter(schema::battle_event::no.eq(no)))
                .set(schema::battle_event::lost.eq(lost))
                .execute(connection)?;
        }
        QueryResult::<()>::Ok(())
    })?;
    Ok(())
}
