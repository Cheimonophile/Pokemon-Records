use tauri::async_runtime::block_on;

use crate::{error::PkmnResult, pkmndb::Read};








#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]

pub struct CatchType {
    pub id: i64,
    pub name: String,
    pub detail: Option<String>,
}



impl Read for CatchType {
    type Raw = CatchType;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let catch_types = block_on(sqlx::query_as!(
            Self::Raw,
            r#"
                SELECT id, name, detail
                FROM catch_type
                ORDER BY id
            "#,
        ).fetch_all(transaction))?;
        Ok(catch_types)
    }
}



#[tauri::command]
pub fn read_catch_types(state: tauri::State<crate::state::GameState>) -> PkmnResult<Vec<CatchType>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let catch_types = CatchType::read(&mut *transaction)?;    
    Ok(catch_types)
}