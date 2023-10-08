use diesel::prelude::*;

use diesel::QueryResult;

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
