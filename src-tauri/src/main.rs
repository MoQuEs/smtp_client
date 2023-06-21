#![allow(dead_code, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

use crate::state::{AppState, ServiceAccess};
use rust_utils::log::Log;
use std::sync::Mutex;

mod backup;
mod commands;
mod crypt;
mod database;
mod migration;
mod response;
mod serialize;
mod state;

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level_for("sled::pagecache", log::LevelFilter::Warn)
                .targets([
                    tauri_plugin_log::LogTarget::Stdout,
                    tauri_plugin_log::LogTarget::Webview,
                    tauri_plugin_log::LogTarget::LogDir,
                ])
                .build(),
        )
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(AppState {
            db: Mutex::new(None),
        })
        .setup(|app| {
            app.handle().setup(app);

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
        .log_error("backend::main", "Tauri application failed")
        .unwrap()
}
