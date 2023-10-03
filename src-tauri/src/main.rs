// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;

mod api;
mod dbi;
mod master;

fn main() {
    // master::run();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![crate::api::playthrough::read_playthroughs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
