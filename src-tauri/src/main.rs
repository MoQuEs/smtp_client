#![allow(dead_code, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

mod commands;
mod crypt;
mod database;
mod response;

fn main() {
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
        //.manage(database::connect())
        .invoke_handler(tauri::generate_handler![
            commands::get_configurations_command,
            commands::save_configuration_command,
            commands::remove_configuration_command,
            commands::get_messages_command,
            commands::save_message_command,
            commands::send_mail_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
