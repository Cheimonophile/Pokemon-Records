use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::{state, error::PkmnResult};




#[tauri::command]
pub fn set_db_connection(state: tauri::State<state::GameState>, database_url: &str) -> PkmnResult<()> {
    println!("{}", database_url);
    let connection = SqliteConnection::establish(&database_url)?;
    state.set_connection(connection)?;
    Ok(())
}