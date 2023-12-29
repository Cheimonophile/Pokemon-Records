use serde;
use sqlx;
use tauri;

use crate::{error::PkmnResult, state};


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Ball {
    pub id: i64,
    pub name: String,
}



#[tauri::command]
pub async fn read_balls(state: tauri::State<state::GameState, '_>) -> PkmnResult<Vec<Ball>> {
    let connection = state.get_connection()?;
    let results = sqlx::query_as!(
        Ball,
        r#"
            SELECT id, name
            FROM ball
            ORDER BY id
        "#
    )
    .fetch_all(connection)
    .await?;
    Ok(results)
}