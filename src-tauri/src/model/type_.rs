use serde;
use sqlx;
use tauri::{self, async_runtime::block_on};

use crate::{error::PkmnResult, pkmndb::Read, state};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Type {
    pub id: i64,
    pub name: String,
    pub color: String,
}

impl Read for Type {
    type Raw = Type;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let types = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT id, name, color
                    FROM type
                    ORDER BY id
                "#,
            )
            .fetch_all(transaction),
        )?;
        Ok(types)
    }
}

#[tauri::command]
pub fn read_types(state: tauri::State<state::GameState, '_>) -> PkmnResult<Vec<Type>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let types = Type::read(&mut *transaction)?;
    Ok(types)
}
