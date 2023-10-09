use diesel::prelude::*;

use diesel::QueryResult;

use crate::{error::PkmnResult, dbi::{self, structs::battle_type::BattleType}, schema};




#[tauri::command]
pub fn read_battle_types() -> PkmnResult<Vec<BattleType>> {
    let battle_types = dbi::connection::connect()?.transaction(|connection| {
        let battle_types = schema::Battle_Type::table
            .load::<BattleType>(connection)?;
        QueryResult::<Vec<BattleType>>::Ok(battle_types)
    })?;
    Ok(battle_types)
}