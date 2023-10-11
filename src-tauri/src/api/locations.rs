use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::location::InsertLocation;
use crate::dbi::structs::location::Location;
use crate::state;
use crate::{
    dbi::{self},
    error::PkmnResult,
    schema,
};

#[tauri::command]
pub fn read_locations(
    state: tauri::State<state::GameState>,
    name: Option<&str>,
    region: Option<&str>,
) -> PkmnResult<Vec<Location>> {
    let locations = state.transact(|connection| {
        let mut query = schema::Location::table.into_boxed();
        if let Some(name) = name {
            query = query.filter(schema::Location::name.eq(name));
        }
        if let Some(region) = region {
            query = query.filter(schema::Location::region.eq(region));
        }
        let results = query.load::<Location>(connection)?;
        QueryResult::<Vec<Location>>::Ok(results)
    })?;
    Ok(locations)
}

#[tauri::command]
pub fn create_location(
    state: tauri::State<state::GameState>,
    name: &str,
    region: &str,
) -> PkmnResult<Location> {
    let location = state.transact(|connection| {
        let location = InsertLocation {
            name: name,
            region: region,
        };
        diesel::insert_into(schema::Location::table)
            .values(&location)
            .execute(connection)?;
        let location = schema::Location::table
            .filter(
                schema::Location::name
                    .eq(name)
                    .and(schema::Location::region.eq(region)),
            )
            .first(connection)?;
        QueryResult::<Location>::Ok(location)
    })?;
    Ok(location)
}
