use diesel::{prelude::*, QueryResult};

use crate::{dbi::structs::ball::Ball, error::PkmnResult, schema, state};

#[tauri::command]
pub fn read_balls(state: tauri::State<state::GameState>) -> PkmnResult<Vec<Ball>> {
    let results = state.transact(|connection| {
        let results = schema::Ball::table.load::<Ball>(connection)?;
        QueryResult::<Vec<Ball>>::Ok(results)
    })?;
    Ok(results)
}
