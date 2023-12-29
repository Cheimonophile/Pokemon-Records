use sqlx::prelude::*;
use tauri::async_runtime::block_on;

use crate::error::{PkmnError, PkmnResult, StringError};

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!();

pub struct GameState {
    db_url: Option<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self { db_url: None }
    }
    pub async fn set_connection(&mut self, db_url: &str) -> PkmnResult<()> {
        let mut connection = sqlx::SqliteConnection::connect(&db_url).await?;
        // test the sqlite connection
        if let Err(error) = MIGRATOR.run(&mut connection).await {
            return Ok(
                StringError::new(&format!("Could not connect to database: {}", error)).err()?,
            );
        }
        self.db_url = Some(db_url.to_string());
        // if let Ok(mut guard) = self.connection.lock() {
        //     *guard = Some(connection);
        //     Ok(())
        // } else {
        //     Ok(StringError::new("Could not lock connection").err()?)
        // }
        Ok(())
    }

    pub fn transaction(&self) -> PkmnResult<sqlx::Transaction<'_, sqlx::Sqlite>> {
        if let Some(db_url) = &self.db_url {
            let mut connection = block_on(sqlx::SqliteConnection::connect(&db_url))?;
            let transaction = block_on(sqlx::Acquire::begin(&mut connection))?;
            Ok(transaction)
        } else {
            StringError::new("No connection set").err()
        }
    }
}
