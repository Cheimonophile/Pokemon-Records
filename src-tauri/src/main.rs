// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use state::GameState;


// mod api;
mod model;
mod error;
mod state;
mod sqlx_ext;

fn main() {
        tauri::Builder::default()
            .manage(GameState::new())
            .invoke_handler(tauri::generate_handler![
                crate::model::ball::read_balls,
                crate::model::battle_type::read_battle_types,
                crate::model::event::create_battle_event,
                crate::model::battles::read_battles,
                crate::model::battles::update_battle,
                crate::model::battles::delete_battle,
                crate::model::catches::create_catch,
                crate::model::catches::read_catches,
                crate::model::catches::delete_catch,
                crate::model::catch_types::read_catch_types,
                crate::model::data::playthrough_data::team_over_time,
                crate::model::locations::create_location,
                crate::model::locations::read_locations,
                crate::model::playthrough::create_playthrough,
                crate::model::playthrough::read_playthroughs,
                crate::model::regions::read_regions,
                crate::model::species::read_species,
                crate::model::state::set_db_connection,
                crate::model::team_member_changes::create_team_member_change,
                crate::model::team_members::read_team_members,
                crate::model::trainer_classes::create_trainer_class,
                crate::model::trainer_classes::read_trainer_classes,
                crate::model::trainers::create_trainer,
                crate::model::trainers::read_trainers,
                crate::model::type_::read_types,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
}
