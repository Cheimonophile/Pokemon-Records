use serde;
use sqlx;
use tauri::{self, async_runtime::block_on};

use crate::{error::PkmnResult, pkmndb::Read, state};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Ball {
    pub id: i64,
    pub name: String,
}

impl Read for Ball {
    type Raw = Ball;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let balls = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT id, name
                    FROM ball
                    ORDER BY id
                "#,
            )
            .fetch_all(transaction),
        )?;
        Ok(balls)
    }
}

#[tauri::command]
pub fn read_balls(state: tauri::State<state::GameState>) -> PkmnResult<Vec<Ball>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let balls = Ball::read(&mut *transaction)?;
    Ok(balls)
}
