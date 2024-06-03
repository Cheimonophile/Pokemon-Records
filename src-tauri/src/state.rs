use std::{
    fs::{self},
    ops::{Deref, DerefMut},
    path,
    sync::RwLock,
};

use chrono;

use sqlx::prelude::*;
use tauri::async_runtime::block_on;

use crate::error::{PkmnResult, StringError};

use sqlx::migrate::Migrator;

static NUM_BACKUPS: usize = 3;

static MIGRATOR: Migrator = sqlx::migrate!();

pub struct GameState {
    db_url: RwLock<Option<String>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            db_url: RwLock::new(None),
        }
    }
    pub fn set_connection(&self, db_url: &str) -> PkmnResult<()> {
        let url = format!("sqlite://{}", &db_url);

        // do nothing if the url matches the old url
        if let Ok(db_url_guard) = self.db_url.read() {
            if let Some(db_url_guard) = &*db_url_guard {
                if db_url_guard == db_url {
                    return Ok(());
                }
            }
        }

        // copy file
        let mut copy_path = path::PathBuf::from(db_url);
        let file_stem = copy_path
            .file_stem()
            .and_then(|file_stem| file_stem.to_str())
            .unwrap_or_default()
            .to_owned();
        let ext = copy_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
            .to_owned();
        let copy_stem = format!("{} backup {}", file_stem, chrono::Local::now().to_rfc3339());
        copy_path.set_file_name(copy_stem);
        copy_path.set_extension(ext);
        fs::copy(db_url, &copy_path).map_err(|error| {
            StringError::new(&format!(
                "Could not copy database file to backup location: {}",
                error
            ))
        })?;

        // open database and run migrations
        let mut connection = block_on(sqlx::SqliteConnection::connect(&url))?;
        if let Err(error) = block_on(MIGRATOR.run(&mut connection)) {
            return Ok(
                StringError::new(&format!("Could not connect to database: {}", error)).err()?,
            );
        }
        match self.db_url.write() {
            Ok(mut db_url_guard) => *db_url_guard = Some(db_url.to_string()),
            Err(error) => StringError::new(&format!("{}", error)).err()?,
        };

        // remove old backups
        if let Some(parent_path) = path::Path::new(db_url).parent() {
            if let Some(mut backup_paths) = fs::read_dir(parent_path).ok().and_then(|read_dir| {
                Some(
                    read_dir
                        .filter_map(|entry| {
                            entry.ok().and_then(|entry| {
                                entry.file_name().to_str().and_then(|file_name| {
                                    if file_name.starts_with(&format!("{} backup", file_stem)) {
                                        Some(entry.path())
                                    } else {
                                        None
                                    }
                                })
                            })
                        })
                        .collect::<Vec<path::PathBuf>>(),
                )
            }) {
                backup_paths.sort();
                backup_paths.reverse();
                backup_paths.iter().skip(NUM_BACKUPS).for_each(|backup_path| {
                    fs::remove_file(backup_path).ok();
                });
            };
        }

        Ok(())
    }

    pub fn connection<'a>(&self) -> PkmnResult<PkmnConnection> {
        match self.db_url.read() {
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
