use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::trainer::InsertTrainer;
use crate::dbi::structs::trainer::Trainer;
use crate::state;
use crate::{error::PkmnResult, schema};

#[tauri::command]
pub fn read_trainers(
    state: tauri::State<state::GameState>,
    name: Option<&str>,
    class: Option<&str>,
) -> PkmnResult<Vec<Trainer>> {
    let trainer_classes = state.transact(|connection| {
        let mut query = schema::Trainer::table.into_boxed();
        if let Some(name) = name {
            query = query.filter(schema::Trainer::name.eq(name));
        }
        if let Some(class) = class {
            query = query.filter(schema::Trainer::class.eq(class));
        }
        let results = query.load::<Trainer>(connection)?;
        QueryResult::<Vec<Trainer>>::Ok(results)
    })?;
    Ok(trainer_classes)
}

#[tauri::command]
pub fn create_trainer(
    state: tauri::State<state::GameState>,
    name: &str,
    class: &str,
) -> PkmnResult<Trainer> {
    let trainer = state.transact(|connection| {
        let new_trainer = InsertTrainer { name, class };
        diesel::insert_into(schema::Trainer::table)
            .values(&new_trainer)
            .execute(connection)?;
        let trainer = schema::Trainer::table
            .filter(
                schema::Trainer::name
                    .eq(name)
                    .and(schema::Trainer::class.eq(class)),
            )
            .first::<Trainer>(connection)?;
        QueryResult::<Trainer>::Ok(trainer)
    })?;
    Ok(trainer)
}
