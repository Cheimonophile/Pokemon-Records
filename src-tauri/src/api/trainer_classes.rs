use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::trainer_class::TrainerClass;
use crate::state;
use crate::{error::PkmnResult, schema};

#[tauri::command]
pub fn read_trainer_classes(
    state: tauri::State<state::GameState>,
    name: Option<&str>,
) -> PkmnResult<Vec<String>> {
    let trainer_classes = state.transact(|connection| {
        let mut query = schema::trainer_class::table.into_boxed();
        if let Some(name) = name {
            query = query.filter(schema::trainer_class::name.eq(name));
        }
        let results = query.load::<TrainerClass>(connection)?;
        QueryResult::<Vec<TrainerClass>>::Ok(results)
    })?;
    Ok(trainer_classes.iter().map(|tc| tc.name.clone()).collect())
}

#[tauri::command]
pub fn create_trainer_class(
    state: tauri::State<state::GameState>,
    name: &str,
) -> PkmnResult<String> {
    let trainer_class = state.transact(|connection| {
        diesel::insert_into(schema::trainer_class::table)
            .values(schema::trainer_class::name.eq(name))
            .execute(connection)?;
        let trainer_class = schema::trainer_class::table
            .filter(schema::trainer_class::name.eq(name))
            .first::<TrainerClass>(connection)?;
        QueryResult::<TrainerClass>::Ok(trainer_class)
    })?;
    Ok(trainer_class.name.clone())
}
