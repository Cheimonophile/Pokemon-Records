use diesel::prelude::*;

use diesel::QueryResult;

use crate::dbi::structs::location::InsertLocation;
use crate::dbi::structs::location::Location;
use crate::state;
use crate::{ error::PkmnResult, schema};

#[tauri::command]
pub fn read_locations(
    state: tauri::State<state::GameState>,
    name: Option<&str>,
    region_id: Option<i32>,
) -> PkmnResult<Vec<Location>> {
    let locations = state.transact(|connection| {
        let mut query = schema::location::table.into_boxed();
        if let Some(name) = name {
            query = query.filter(schema::location::name.eq(name));
        }
        if let Some(region_id) = region_id {
            query = query.filter(schema::location::region_id.eq(region_id));
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
    region_id: i32,
) -> PkmnResult<Location> {
    let location = state.transact(|connection| {
        let location = InsertLocation {
            name: name,
            region_id,
        };
        diesel::insert_into(schema::location::table)
            .values(&location)
            .execute(connection)?;
        let location = schema::location::table
            .filter(
                schema::location::name
                    .eq(name)
                    .and(schema::location::region_id.eq(region_id)),
            )
            .first(connection)?;
        QueryResult::<Location>::Ok(location)
    })?;
    Ok(location)
}
