// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;

mod dbi;
mod master;
mod master2;



fn main() {
  master2::run()
  // tauri::Builder::default()
  //   .run(tauri::generate_context!())
  //   .expect("error while running tauri application");
}
