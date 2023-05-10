#![allow(dead_code, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

use crate::database::Database;
use crate::state::AppState;
use std::sync::Mutex;
use tauri::{Manager, State};

mod commands;
mod crypt;
mod database;
mod response;
mod state;

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(None),
        })
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db =
                Database::new(app.config().as_ref()).expect("Database initialize should succeed");
            *app_state.db.lock().expect("Lock database for data") = Some(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_configurations_command,
            commands::save_configuration_command,
            commands::remove_configuration_command,
            commands::get_messages_command,
            commands::save_message_command,
            commands::remove_message_command,
            commands::send_mail_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
