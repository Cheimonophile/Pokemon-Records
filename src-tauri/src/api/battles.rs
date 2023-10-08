use diesel::prelude::*;

use serde;

use crate::{
    dbi::{
        self,
        structs::{battle_event::BattleEvent, event::Event},
    },
    error::PkmnResult,
    schema,
};

#[derive(serde::Serialize)]
pub struct ReadBattlesResult {
    #[serde(flatten)]
    battle: BattleEvent,
    event: Event,
}

#[tauri::command]
pub fn read_battles() -> PkmnResult<Vec<ReadBattlesResult>> {
    let raw_battles = dbi::connection::connect().transaction(|connection| {
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
pub fn create_battle() {
}
