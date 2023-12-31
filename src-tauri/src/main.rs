// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use state::GameState;


// mod api;
mod model;
mod error;
mod state;
mod util;
mod pkmndb;
mod reports;

fn main() {
        tauri::Builder::default()
            .manage(GameState::new())
            .invoke_handler(tauri::generate_handler![
                crate::model::ball::read_balls,
                // crate::model::battles::update_battle,
                crate::model::battle_type::read_battle_types,
                crate::model::catch_type::read_catch_types,
                crate::model::event::create_event,
                crate::model::event::read_events,
                crate::model::event::delete_event,
                crate::model::item::read_items,
                // crate::model::data::playthrough_data::team_over_time,
                crate::model::location::create_location,
                crate::model::location::read_locations,
                // crate::model::playthrough::create_playthrough,
                crate::model::playthrough::read_playthroughs,
                crate::model::region::read_regions,
                crate::model::species::read_species,
                crate::model::team_member_change::create_team_member_change,
                crate::model::team_member::read_team_members,
                crate::model::trainer_class::create_trainer_class,
                crate::model::trainer_class::read_trainer_classes,
                crate::model::trainer::create_trainer,
                crate::model::trainer::read_trainers,
                crate::model::type_::read_types,
                crate::model::version::read_versions,
                crate::reports::team_over_time::team_over_time,
                crate::state::set_db_connection,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
}
