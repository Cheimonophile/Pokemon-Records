use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::trainer;
use crate::dbi::structs::trainer_class::TrainerClass;
use crate::{
    dbi::{self},
    error::PkmnResult,
    schema,
};

#[tauri::command]
pub fn read_trainer_classes(name: Option<&str>) -> PkmnResult<Vec<String>> {
    let trainer_classes = dbi::connection::connect().transaction(|connection| {
        let mut query = schema::Trainer_Class::table.into_boxed();
        if let Some(name) = name {
            query = query.filter(schema::Trainer_Class::name.eq(name));
        }
        let results = query.load::<TrainerClass>(connection)?;
        QueryResult::<Vec<TrainerClass>>::Ok(results)
    })?;
    Ok(trainer_classes.iter().map(|tc| tc.name.clone()).collect())
}

#[tauri::command]
pub fn create_trainer_class(name: &str) -> PkmnResult<String> {
    let trainer_class = dbi::connection::connect().transaction::<_, diesel::result::Error, _>(|connection| {
        diesel::insert_into(schema::Trainer_Class::table)
            .values(schema::Trainer_Class::name.eq(name))
            .execute(connection)?;
        let trainer_class = schema::Trainer_Class::table
            .filter(schema::Trainer_Class::name.eq(name))
            .first::<TrainerClass>(connection)?;
        QueryResult::<TrainerClass>::Ok(trainer_class)
    })?;
    Ok(trainer_class.name.clone())
}
