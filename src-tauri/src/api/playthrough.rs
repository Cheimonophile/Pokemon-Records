use diesel::{prelude::*, query_dsl::InternalJoinDsl, dsl::max, sql_query};

use crate::{dbi::structs::playthrough::Playthrough, error::PkmnResult, schema};

#[tauri::command]
pub fn read_playthroughs() -> PkmnResult<Vec<Playthrough>> {
    let mut connection = crate::dbi::connection::connect();
    let results = schema::Playthrough::table
        .order(schema::Playthrough::columns::adventure_started.desc())
        .select(Playthrough::as_select())
        .distinct()
        .load(&mut connection)?;
    Ok(results)
}

#[tauri::command]
pub fn create_playthrough(
    id_no: &str,
    name: &str,
    version: &str,
    adventure_started: &str,
) -> Option<Playthrough> {
    let playthrough = match Playthrough::create(id_no, name, version, adventure_started) {
        Ok(playthrough) => Some(playthrough),
        Err(error) => {
            eprintln!("Error creating playthrough: {}", error);
            None
        }
    };
    playthrough
}
