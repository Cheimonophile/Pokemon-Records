use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

use diesel::{prelude::*, result};

use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

use crate::error::{PkmnError, PkmnResult, StringError};

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
    pub fn transact<T>(
        &self,
        callback: fn(conn: &mut SqliteConnection) -> QueryResult<T>,
    ) -> PkmnResult<T> {
        if let Ok(mut guard) = self.connection.lock() {
            let connection = match guard.deref_mut() {
                Some(connection) => connection,
                None => {
                    return Err(PkmnError::StringError(StringError::new(
                        "No connection set",
                    )))
                }
            };
            let result = connection.transaction(callback)?;
            return Ok(result);
        } else {
            return Err(PkmnError::StringError(StringError::new(
                "Could not lock connection",
            )));
        }
    }
}
