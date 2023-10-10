

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::error::PkmnResult;


pub fn connect() -> PkmnResult<SqliteConnection> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url)?)
}