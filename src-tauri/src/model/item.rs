use tauri::async_runtime::block_on;

use crate::{error::PkmnResult, pkmndb::Read, state};


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    id: i64,
    name: String,
}


impl Read for Item {
    type Raw = Item;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let items = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT id, name
                    FROM item
                    ORDER BY id
                "#,
            )
            .fetch_all(transaction),
        )?;
        Ok(items)
    }
}


#[tauri::command]
pub fn read_items(state: tauri::State<state::GameState>) -> PkmnResult<Vec<Item>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let items = Item::read(&mut *transaction)?;
    Ok(items)
}