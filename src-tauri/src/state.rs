use std::ops::DerefMut;
use std::sync::Mutex;

use diesel::prelude::*;

use diesel::sqlite::SqliteConnection;

use crate::{
    dbi::structs,
    error::{PkmnError, PkmnResult, StringError},
    schema,
};

pub struct GameState {
    connection: Mutex<Option<SqliteConnection>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            connection: Mutex::new(None),
        }
    }
    pub fn set_connection(&self, mut connection: SqliteConnection) -> PkmnResult<()> {
        // test the sqlite connection
        // TODO run the migration here
        if let Err(error) =
            schema::Playthrough::table.first::<structs::playthrough::Playthrough>(&mut connection)
        {
            return Ok(
                StringError::new(&format!("Could not connect to database: {}", error)).err()?,
            );
        }

        if let Ok(mut guard) = self.connection.lock() {
            *guard = Some(connection);
            Ok(())
        } else {
            Ok(StringError::new("Could not lock connection").err()?)
        }
    }
    pub fn transact<T, F>(&self, callback: F) -> PkmnResult<T>
    where
        F: FnOnce(&mut SqliteConnection) -> QueryResult<T>,
    {
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
