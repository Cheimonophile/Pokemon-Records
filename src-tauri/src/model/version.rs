use serde;
use sqlx;
use tauri::{self, async_runtime::block_on};

use crate::{error::PkmnResult, pkmndb::Read, state};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: i64,
    pub name: String,
    pub generation: i64,
}

pub struct RawVersion {
    pub id: i64,
    pub name: String,
    pub generation: i64,
}

impl Read for Version {
    type Raw = RawVersion;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let results = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT id, name, generation
                    FROM version
                    ORDER BY generation, id
                "#,
            )
            .fetch_all(transaction),
        )?;
        let versions = results
            .into_iter()
            .map(|raw_version| {
                Ok(Version {
                    id: raw_version.id,
                    name: raw_version.name,
                    generation: raw_version.generation,
                })
            })
            .collect::<PkmnResult<Vec<Version>>>()?;
        Ok(versions)
    }
}

#[tauri::command]
pub fn read_versions(state: tauri::State<state::GameState, '_>) -> PkmnResult<Vec<Version>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let versions = Version::read(&mut *transaction)?;
    Ok(versions)
}
