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
    class_id: Option<i32>,
) -> PkmnResult<Vec<Trainer>> {
    let trainer_classes = state.transact(|connection| {
        let mut query = schema::trainer::table.into_boxed();
        if let Some(name) = name {
            query = query.filter(schema::trainer::name.eq(name));
        }
        if let Some(class_id) = class_id {
            query = query.filter(schema::trainer::class_id.eq(class_id));
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
    class_id: i32,
) -> PkmnResult<Trainer> {
    let trainer = state.transact(|connection| {
        let new_trainer = InsertTrainer { name, class_id };
        diesel::insert_into(schema::trainer::table)
            .values(&new_trainer)
            .execute(connection)?;
        let trainer = schema::trainer::table
            .filter(
                schema::trainer::name
                    .eq(name)
                    .and(schema::trainer::class_id.eq(class_id)),
            )
            .first::<Trainer>(connection)?;
        QueryResult::<Trainer>::Ok(trainer)
    })?;
    Ok(trainer)
}
