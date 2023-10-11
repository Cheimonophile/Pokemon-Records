use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::region::Region;
use crate::state;
use crate::{
    dbi::{self},
    error::PkmnResult,
    schema,
};

#[tauri::command]
pub fn read_regions(
    state: tauri::State<state::GameState>,
    name: Option<&str>,
) -> PkmnResult<Vec<String>> {
    let trainer_classes = state.transact(|connection| {
        let mut query = schema::Region::table.into_boxed();
        if let Some(name) = name {
            query = query.filter(schema::Region::name.eq(name));
        }
        let results = query.load::<Region>(connection)?;
        QueryResult::<Vec<Region>>::Ok(results)
    })?;
    Ok(trainer_classes.iter().map(|tc| tc.name.clone()).collect())
}
