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
    battle: BattleEvent,
    event: Event,
}

#[tauri::command]
pub fn read_battles() -> PkmnResult<Vec<ReadBattlesResult>> {
    let mut connection = dbi::connection::connect();
    let raw_battles = schema::Battle_Event::table
        .inner_join(schema::Event::table)
        .order(schema::Battle_Event::no.desc())
        .select((
            BattleEvent::as_select(),
            Event::as_select(),
        ))
        .get_results::<(BattleEvent, Event)>(&mut connection)?;
    let battles = raw_battles
        .into_iter()
        .map(|(battle, event)| ReadBattlesResult { battle, event })
        .collect();
    Ok(battles)
}
