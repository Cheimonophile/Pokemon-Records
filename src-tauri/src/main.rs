// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;

mod api;
mod dbi;
mod master;
mod error;

fn main() {
    // master::run();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // playthrough handlers
            crate::api::battle_types::read_battle_types,
            crate::api::battles::create_battle,
            crate::api::battles::read_battles,
            crate::api::locations::create_location,
            crate::api::locations::read_locations,
            crate::api::playthrough::create_playthrough,
            crate::api::playthrough::read_playthroughs,
            crate::api::regions::read_regions,
            crate::api::trainer_classes::create_trainer_class,
            crate::api::trainer_classes::read_trainer_classes,
            crate::api::trainers::read_trainers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
