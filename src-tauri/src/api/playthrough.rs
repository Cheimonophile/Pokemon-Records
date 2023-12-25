use diesel::prelude::*;

use crate::{
    dbi::structs::playthrough::{InsertPlaythrough, Playthrough},
    error::PkmnResult,
    schema, state,
};

#[tauri::command]
pub fn read_playthroughs(state: tauri::State<state::GameState>) -> PkmnResult<Vec<Playthrough>> {
    let results = state.transact(|connection| {
        let results = schema::playthrough::table
            .order(schema::playthrough::columns::adventure_started.desc())
            .select(Playthrough::as_select())
            .distinct()
            .load::<Playthrough>(connection)?;
        QueryResult::<Vec<Playthrough>>::Ok(results)
    })?;
    Ok(results)
}

#[tauri::command]
pub fn create_playthrough(
    state: tauri::State<state::GameState>,
    id_no: &str,
    name: &str,
    version_id: i32,
    adventure_started: &str,
) -> PkmnResult<Playthrough> {
    let playthrough = state.transact(|connection| {
        let new_playthrough = InsertPlaythrough {
            id_no,
            name,
            version_id,
            adventure_started,
        };
        diesel::insert_into(schema::playthrough::table)
            .values(&new_playthrough)
            .execute(connection)?;
        let playthrough = schema::playthrough::table
            .filter(schema::playthrough::id_no.eq(id_no))
            .first::<Playthrough>(connection)?;
        QueryResult::<Playthrough>::Ok(playthrough)
    })?;
    Ok(playthrough)
}
