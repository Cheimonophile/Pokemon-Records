use serde;
use sqlx;
use tauri::{self, async_runtime::block_on};

use crate::{
    error::{PkmnResult, StringError},
    pkmndb::Read,
    state,
};

use super::version::Version;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Playthrough {
    pub id_no: String,
    pub name: String,
    pub version: Version,
    pub adventure_started: String,
}

pub struct RawPlaythrough {
    pub id_no: String,
    pub name: String,
    pub version_id: i64,
    pub adventure_started: String,
}

impl Read for Playthrough {
    type Raw = RawPlaythrough;
    type Key = String;
    fn id(&self) -> Self::Key {
        self.id_no.clone()
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let raw_playthroughs = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT DISTINCT playthrough.*
                    FROM playthrough
                    LEFT JOIN event ON event.playthrough_id_no = playthrough.id_no
                    ORDER BY event.no DESC
                "#,
            )
            .fetch_all(&mut *transaction),
        )?;

        let versions = Version::get_map(transaction)?;

        let playthroughs = raw_playthroughs
            .into_iter()
            .map(|raw_playthrough| {
                Ok(Playthrough {
                    id_no: raw_playthrough.id_no,
                    name: raw_playthrough.name,
                    version: versions
                        .get(&raw_playthrough.version_id)
                        .ok_or_else(|| {
                            StringError::new(&format!(
                                "No version found with id {}",
                                raw_playthrough.version_id
                            ))
                        })?
                        .clone(),
                    adventure_started: raw_playthrough.adventure_started,
                })
            })
            .collect::<PkmnResult<Vec<Playthrough>>>()?;
        Ok(playthroughs)
    }
}

#[tauri::command]
pub fn read_playthroughs(state: tauri::State<state::GameState>) -> PkmnResult<Vec<Playthrough>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let playthroughs = Playthrough::read(&mut *transaction)?;
    Ok(playthroughs)
}
