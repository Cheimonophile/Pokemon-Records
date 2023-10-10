use std::{sync::Mutex};

use diesel::prelude::*;

use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

use crate::error::{PkmnResult, StringError};




pub struct GameState {
    connection: Mutex<Option<SqliteConnection>>,
}


impl GameState {
    pub fn new() -> Self {
        Self {
            connection: Mutex::new(None),
        }
    }
    pub fn set_connection(&self, connection: SqliteConnection) -> PkmnResult<()> {
        if let Ok(mut guard) = self.connection.lock() {
            *guard = Some(connection);
            Ok(())
        } else {
            Ok(StringError::new("Could not lock connection").err()?)
        }
    }
}