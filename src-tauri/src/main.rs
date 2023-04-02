#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

use std::sync::Arc;
use tauri::Manager;

mod commands;

pub struct AppState {}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                // let window = app.get_window("main").unwrap();
                // window.open_devtools();
                // window.close_devtools();
            }
            Ok(())
        })
        .manage(Arc::new(AppState {}))
        .invoke_handler(tauri::generate_handler![commands::send_mail])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
