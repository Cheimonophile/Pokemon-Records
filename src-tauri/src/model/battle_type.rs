use serde;
use sqlx;
use tauri::{self, async_runtime::block_on};

use crate::{error::PkmnResult, state, pkmndb::Read};


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BattleType {
    pub id: i64,
    pub name: String,
}


impl Read for BattleType {
    type Raw = BattleType;
    type Key = i64;
    fn id(&self) -> Self::Key {
        self.id
    }
    fn read(transaction: &mut sqlx::SqliteConnection) -> PkmnResult<Vec<Self>> {
        let battle_types = block_on(
            sqlx::query_as!(
                Self::Raw,
                r#"
                    SELECT id, name
                    FROM battle_type
                    ORDER BY id
                "#,
            )
            .fetch_all(transaction),
        )?;
        Ok(battle_types)
    }
}


#[tauri::command]
pub fn read_battle_types(state: tauri::State<state::GameState, '_>) -> PkmnResult<Vec<BattleType>> {
    let mut connection = state.connection()?;
    let mut transaction = connection.transaction()?;
    let battle_types = BattleType::read(&mut *transaction)?;
    Ok(battle_types)
}