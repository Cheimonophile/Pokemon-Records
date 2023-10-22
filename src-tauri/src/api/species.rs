use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::species::Species;
use crate::state;
use crate::{error::PkmnResult, schema};

#[tauri::command]
pub fn read_species(
    state: tauri::State<state::GameState>,
    name: Option<&str>,
) -> PkmnResult<Vec<Species>> {
    let species = state.transact(|connection| {
        let mut query = schema::Species::table.into_boxed();
        if let Some(name) = name {
            query = query.filter(schema::Species::name.eq(name));
        }
        let results = query.load::<Species>(connection)?;
        QueryResult::<Vec<Species>>::Ok(results)
    })?;
    Ok(species)
}
