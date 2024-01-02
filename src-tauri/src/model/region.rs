use serde;
use sqlx::{self};
use tauri;
use tauri::async_runtime::block_on;

use crate::pkmndb::Read;
use crate::{error::PkmnResult, state};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Region {
    pub id: i64,
    pub name: String,
}

impl Read for Region {
    type Raw = Region;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let regions = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT id, name
                    FROM region
                    ORDER BY id
                "#,
            )
            .fetch_all(transaction),
        )?;
        Ok(regions)
    }
}

#[tauri::command]
pub fn read_regions(state: tauri::State<state::GameState>) -> PkmnResult<Vec<Region>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let regions = Region::read(&mut *transaction)?;
    Ok(regions)
}
