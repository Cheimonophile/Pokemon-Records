use serde;
use sqlx::{self};
use tauri::{self, async_runtime::block_on};

use crate::{
    error::{PkmnResult, StringError},
    pkmndb::Read,
    state, util,
};

use super::region::Region;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub region: Region,
}

pub struct RawLocation {
    pub id: i64,
    pub name: String,
    pub region_id: i64,
}

impl Read for Location {
    type Raw = RawLocation;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        // sql
        let raw_locations = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT location.*
                    FROM location
                "#,
            )
            .fetch_all(&mut *transaction),
        )?;

        // relations
        let regions = Region::get_map(transaction)?;

        // cleanup
        let locations = raw_locations
            .into_iter()
            .map(|raw_location| {
                Ok(Location {
                    id: raw_location.id,
                    name: raw_location.name,
                    region: regions
                        .get(&raw_location.region_id)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "No region found with id {}",
                                raw_location.region_id
                            ))
                        })?
                        .clone(),
                })
            })
            .collect::<PkmnResult<Vec<Location>>>()?;
        Ok(locations)
    }
}

#[tauri::command]
pub fn create_location(
    state: tauri::State<state::GameState>,
    name: String,
    region_id: i64,
) -> PkmnResult<i64> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;

    let util::Id { id } = block_on(
        sqlx::query_as!(
            util::Id,
            r#"
                INSERT INTO location (name, region_id)
                VALUES (?, ?)
                RETURNING id
            "#,
            name,
            region_id,
        )
        .fetch_one(&mut *transaction),
    )?;

    transaction.commit()?;

    Ok(id)
}

#[tauri::command]
pub fn read_locations(
    state: tauri::State<state::GameState>,
    id: Option<i64>,
    region_id: Option<i64>,
) -> PkmnResult<Vec<Location>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let locations = Location::read(&mut *transaction)?
        .into_iter()
        .filter(|location| {
            let mut keep = true;
            keep &= if let Some(id) = id {
                location.id == id
            } else {
                true
            };
            keep &= if let Some(region_id) = region_id {
                location.region.id == region_id
            } else {
                true
            };
            keep
        })
        .collect::<Vec<Location>>();
    Ok(locations)
}
