use diesel::prelude::*;

use crate::{
    dbi::structs::playthrough::{InsertPlaythrough, Playthrough},
    dbi::structs::version::Version,
    error::PkmnResult,
    schema, state,
};

#[derive(serde::Serialize)]
pub struct ReadResult {
    #[serde(flatten)]
    playthrough: Playthrough,
    version: Version,
}

#[tauri::command]
pub fn read_playthroughs(state: tauri::State<state::GameState>) -> PkmnResult<Vec<ReadResult>> {
    let playthroughs = state.transact(|connection| {
        let results = schema::playthrough::table
            .inner_join(schema::version::table)
            .order(
                schema::event::table
                    .filter(schema::event::playthrough_id_no.eq(schema::playthrough::id_no))
                    .select(diesel::dsl::max(schema::event::no))
                    .single_value()
                    .desc(),
            )
            .select((Playthrough::as_select(), Version::as_select()))
            .load::<(Playthrough, Version)>(connection)?;
        QueryResult::<Vec<(Playthrough, Version)>>::Ok(results)
    })?;
    let results = playthroughs
        .into_iter()
        .map(|(playthrough, version)| ReadResult { playthrough, version })
        .collect();
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
