use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::types::Type;
use crate::error::PkmnResult;
use crate::{state::GameState, schema};






#[tauri::command]
pub fn read_types(state: tauri::State<GameState>) -> PkmnResult<Vec<Type>> {
    let result = state.transact(|connection| {
        let types = schema::Type::table.load::<Type>(connection)?;

        QueryResult::<Vec<Type>>::Ok(types)
    })?;
    Ok(result)
}