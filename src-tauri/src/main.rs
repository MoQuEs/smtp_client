#![allow(dead_code, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

use crate::database::Database;
use crate::state::AppState;
use rust_utils::log_result::LogErrorFromResult;
use std::sync::Mutex;
use tauri::{Manager, State};

mod commands;
mod crypt;
mod database;
mod response;
mod state;

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([tauri_plugin_log::LogTarget::LogDir])
                .build(),
        )
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(AppState {
            db: Mutex::new(None),
        })
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = Database::new(app.config().as_ref())
                .log_error("backend::main::main", "Database initialize failed")
                .expect("Database initialize should succeed");

            *app_state
                .db
                .lock()
                .log_error("backend::main::main", "Lock database for data")
                .expect("Lock database for data") = Some(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::test,
            commands::get_configurations_command,
            commands::save_configuration_command,
            commands::remove_configuration_command,
            commands::get_messages_command,
            commands::save_message_command,
            commands::remove_message_command,
            commands::send_mail_command,
            commands::get_settings_command,
            commands::save_settings_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
