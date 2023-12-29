use serde;
use sqlx;
use tauri;

use crate::{error::PkmnResult, state};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Version {
    pub id: i64,
    pub name: String,
    pub generation: i64,
}

#[tauri::command]
pub async fn read_versions(state: tauri::State<state::GameState, '_>) -> PkmnResult<Vec<Version>> {
    let connection = state.get_connection()?;
    let results = sqlx::query_as!(
        Version,
        r#"
            SELECT id, name, generation
            FROM version
            ORDER BY generation, id
        "#
    )
    .fetch_all(connection)
    .await?;
    Ok(results)
}
