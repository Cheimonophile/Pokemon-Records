use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::catch_type::CatchType;
use crate::error::PkmnResult;
use crate::{state::GameState, schema};






#[tauri::command]
pub fn read_catch_types(state: tauri::State<GameState>) -> PkmnResult<Vec<CatchType>> {
    let result = state.transact(|connection| {
        let types = schema::Catch_Type::table.load::<CatchType>(connection)?;

        QueryResult::<Vec<CatchType>>::Ok(types)
    })?;
    Ok(result)
}