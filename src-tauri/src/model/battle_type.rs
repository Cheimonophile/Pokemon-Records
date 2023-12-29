use serde;
use sqlx;
use tauri;

use crate::{error::PkmnResult, state};


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BattleType {
    pub id: i64,
    pub name: String,
}

#[tauri::command]
pub async fn read_battle_types(state: tauri::State<state::GameState, '_>) -> PkmnResult<Vec<BattleType>> {
    let connection = state.get_connection()?;
    let results = sqlx::query_as!(
        BattleType,
        r#"
            SELECT id, name
            FROM battle_type
            ORDER BY id
        "#
    )
    .fetch_all(connection)
    .await?;
    Ok(results)
}