#![allow(dead_code, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

use crate::state::{AppState, ServiceAccess};
use commands::*;
use rust_utils::log::Log;
use std::sync::Mutex;

mod backup;
mod commands;
mod crypt;
mod database;
mod dialogs;
mod file;
mod migration;
mod response;
mod serialize;
mod state;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level_for("sled::pagecache", log::LevelFilter::Warn)
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("webview".into()),
                    }),
                ])
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(AppState {
            db: Mutex::new(None),
        })
        .setup(|app| {
            app.handle().setup(app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_configurations_command,
            save_configuration_command,
            remove_configuration_command,
            get_messages_command,
            save_message_command,
            remove_message_command,
            send_mail_command,
            get_settings_command,
            save_settings_command,
            export_command,
            import_command,
        ])
        .run(tauri::generate_context!())
        .log_error("backend::main", "Tauri application failed")
        .unwrap()
}
