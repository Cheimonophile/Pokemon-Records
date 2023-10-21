use diesel::prelude::*;
use diesel::sql_query;
use diesel::SqliteConnection;

use crate::{error::PkmnResult, state};

#[tauri::command]
pub fn set_db_connection(
    state: tauri::State<state::GameState>,
    database_url: &str,
) -> PkmnResult<()> {
    println!("{}", database_url);
    let mut connection = SqliteConnection::establish(&database_url)?;
    sql_query("PRAGMA foreign_keys = ON").execute(&mut connection)?;
    crate::run_db_migrations(&mut connection)?;
    state.set_connection(connection)?;
    Ok(())
}
