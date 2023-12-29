use serde;
use sqlx;
use tauri;

use crate::{error::PkmnResult, state};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Type {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[tauri::command]
pub async fn read_types(state: tauri::State<state::GameState, '_>) -> PkmnResult<Vec<Type>> {
    let connection = state.get_connection()?;
    let results = sqlx::query_as!(
        Type,
        r#"
            SELECT id, name, color
            FROM type
            ORDER BY id
        "#
    )
    .fetch_all(connection)
    .await?;
    Ok(results)
}
