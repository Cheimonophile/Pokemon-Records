use diesel::prelude::*;

use diesel::QueryResult;

use crate::state;
use crate::{dbi::structs::battle_type::BattleType, error::PkmnResult, schema};

#[tauri::command]
pub fn read_battle_types(state: tauri::State<state::GameState>) -> PkmnResult<Vec<BattleType>> {
    let battle_types = state.transact(|connection| {
        let battle_types = schema::battle_type::table.load::<BattleType>(connection)?;
        QueryResult::<Vec<BattleType>>::Ok(battle_types)
    })?;
    Ok(battle_types)
}
