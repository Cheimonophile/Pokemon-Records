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
            crate::api::playthrough::read_playthroughs,
            crate::api::playthrough::create_playthrough,

            // battle handlers
            crate::api::battles::read_battles
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
