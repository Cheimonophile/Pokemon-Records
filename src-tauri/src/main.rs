// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use error::{PkmnResult, StringError};
use state::GameState;

mod schema;

mod api;
mod dbi;
mod error;
mod state;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) -> PkmnResult<()> {
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(StringError::new(&format!("Error running migrations: {}", e)).into())
        },
    }
}

fn main() {
        tauri::Builder::default()
            .manage(GameState::new())
            .invoke_handler(tauri::generate_handler![
                crate::api::battle_types::read_battle_types,
                crate::api::battles::create_battle,
                crate::api::battles::read_battles,
                crate::api::battles::update_battle,
                crate::api::battles::delete_battle,
                crate::api::catches::create_catch,
                crate::api::catches::read_catches,
                crate::api::catches::delete_catch,
                crate::api::data::playthrough_data::team_over_time,
                crate::api::locations::create_location,
                crate::api::locations::read_locations,
                crate::api::playthrough::create_playthrough,
                crate::api::playthrough::read_playthroughs,
                crate::api::regions::read_regions,
                crate::api::state::set_db_connection,
                crate::api::team_member_changes::create_team_member_change,
                crate::api::team_members::read_team_members,
                crate::api::trainer_classes::create_trainer_class,
                crate::api::trainer_classes::read_trainer_classes,
                crate::api::trainers::create_trainer,
                crate::api::trainers::read_trainers,
                crate::api::types::read_types,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
}
