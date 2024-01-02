use std::{
    ops::{Deref, DerefMut},
    sync::Mutex,
};

use sqlx::prelude::*;
use tauri::async_runtime::block_on;

use crate::error::{PkmnResult, StringError};

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!();

pub struct GameState {
    db_url: Mutex<Option<String>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            db_url: Mutex::new(None),
        }
    }
    pub fn set_connection(&self, db_url: &str) -> PkmnResult<()> {
        let mut connection = block_on(sqlx::SqliteConnection::connect(&db_url))?;
        if let Err(error) = block_on(MIGRATOR.run(&mut connection)) {
            return Ok(
                StringError::new(&format!("Could not connect to database: {}", error)).err()?,
            );
        }
        match self.db_url.lock() {
            Ok(mut db_url_guard) => *db_url_guard = Some(db_url.to_string()),
            Err(error) => StringError::new(&format!("{}", error)).err()?,
        };
        Ok(())
    }

    pub fn connection<'a>(&self) -> PkmnResult<PkmnConnection> {
        match self.db_url.lock() {
            Ok(db_url_guard) => {
                if let Some(db_url) = &*db_url_guard {
                    let connection = block_on(sqlx::SqliteConnection::connect(&db_url))?;
                    Ok(PkmnConnection { connection })
                } else {
                    StringError::new("No connection set").err()
                }
            }
            Err(error) => StringError::new(&format!("Unable to lock state {}", error)).err(),
        }
    }
}

#[tauri::command]
pub fn set_db_connection(state: tauri::State<GameState>, database_url: &str) -> PkmnResult<()> {
    println!("{}", database_url);
    state.set_connection(database_url)?;
    Ok(())
}

pub struct PkmnConnection {
    connection: sqlx::SqliteConnection,
}

impl PkmnConnection {
    pub fn transaction(&mut self) -> PkmnResult<PkmnTransaction> {
        let transaction = block_on(sqlx::Acquire::begin(&mut self.connection))?;
        Ok(PkmnTransaction { transaction })
    }
}

pub struct PkmnTransaction<'a> {
    transaction: sqlx::Transaction<'a, sqlx::Sqlite>,
}

impl Deref for PkmnTransaction<'_> {
    type Target = sqlx::SqliteConnection;
    fn deref(&self) -> &Self::Target {
        &*self.transaction
    }
}

impl DerefMut for PkmnTransaction<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.transaction
    }
}

impl PkmnTransaction<'_> {
    pub fn commit(self) -> PkmnResult<()> {
        block_on(self.transaction.commit())?;
        Ok(())
    }
}
