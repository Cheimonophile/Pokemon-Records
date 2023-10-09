use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::region::Region;
use crate::{
    dbi::{self},
    error::PkmnResult,
    schema,
};

#[tauri::command]
pub fn read_regions(name: Option<&str>) -> PkmnResult<Vec<String>> {
    let trainer_classes = dbi::connection::connect()?.transaction(|connection| {
        let mut query = schema::Region::table.into_boxed();
        if let Some(name) = name {
            query = query.filter(schema::Region::name.eq(name));
        }
        let results = query.load::<Region>(connection)?;
        QueryResult::<Vec<Region>>::Ok(results)
    })?;
    Ok(trainer_classes.iter().map(|tc| tc.name.clone()).collect())
}
