use crate::backup::{Backup, BackupDataV1};
use crate::response::{error, success_empty, ImportExportSettings, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};
use rust_utils::log::Log;
use std::fs;
use std::io::Write;
use tauri::api::dialog;
use tauri::api::dialog::{FileDialogBuilder, MessageDialogBuilder};

#[tauri::command]
pub fn import_command(
    app_handle: AppHandle,
    import_export_settings: ImportExportSettings,
) -> TauriResponse<()> {
    log::trace!(target: "backend::commands::import_export::import_command", "import_command");
    log::debug!(target: "backend::commands::import_export::import_command", "import_export_settings: {:?}", import_export_settings);

    success_empty()
}

#[tauri::command]
pub fn export_command(
    app_handle: AppHandle,
    import_export_settings: ImportExportSettings,
) -> TauriResponse<()> {
    log::trace!(target: "backend::commands::import_export::export_command", "export_command");
    log::debug!(target: "backend::commands::import_export::export_command", "import_export_settings: {:?}", import_export_settings);

    let mut backup = BackupDataV1::default();

    if import_export_settings.settings {
        match app_handle.db(|db| db.get_settings()) {
            Ok(setting) => {
                log::info!(target: "backend::commands::import_export::export_command", "Added settings: {:?}", setting);
                backup.add_settings(setting);
            }
            Err(err) => {
                log::error!(target: "backend::commands::import_export::export_command", "Error: {:?}", err);
                return error(Some(format!("{:?}", err)), None);
            }
        }
    }

    if import_export_settings.smtp_messages {
        match app_handle.db(|db| db.get_messages()) {
            Ok(messages) => {
                log::info!(target: "backend::commands::import_export::export_command", "Added messages: {:?}", messages);
                backup.add_messages(messages);
            }
            Err(err) => {
                log::error!(target: "backend::commands::import_export::export_command", "Error: {:?}", err);
                return error(Some(format!("{:?}", err)), None);
            }
        }
    }

    if import_export_settings.smtp_configurations {
        match app_handle.db(|db| db.get_configurations()) {
            Ok(configurations) => {
                log::info!(target: "backend::commands::import_export::export_command", "Added configurations: {:?}", configurations);
                backup.add_configurations(configurations);
            }
            Err(err) => {
                log::error!(target: "backend::commands::import_export::export_command", "Error: {:?}", err);
                return error(Some(format!("{:?}", err)), None);
            }
        }
    };

    let serialized_backup = Backup::v1(import_export_settings.password, backup);
    if let Err(err) = serialized_backup {
        log::error!(target: "backend::commands::import_export::export_command", "Error: {:?}", err);
        return error(Some(format!("{:?}", err)), None);
    }
    let serialized_backup = serialized_backup.unwrap();

    FileDialogBuilder::default()
        .add_filter("SMTPclient backup file", &["scb"])
        .save_file(move |path_buf| {
            if let Some(file_path) = path_buf {
                log::info!(target: "backend::commands::import_export::export_command::FileDialogBuilder", "Saving to file: {:?}", file_path);

                let file = fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_path)
                    .log_error("backend::commands::import_export::export_command::FileDialogBuilder", "Error opening file");

                if let Err(err) = file {
                    let error_message = format!("Error: {:?}", err);
                    log::error!(target: "backend::commands::import_export::export_command::FileDialogBuilder", "{}", error_message.as_str());

                    MessageDialogBuilder::new("ERROR", error_message.as_str())
                        .kind(dialog::MessageDialogKind::Error)
                        .buttons(dialog::MessageDialogButtons::Ok);

                    return;
                }

                if let Err(err) = file.unwrap().write_all(&serialized_backup) {
                    let error_message = format!("Error: {:?}", err);
                    log::error!(target: "backend::commands::import_export::export_command::FileDialogBuilder", "{}", error_message.as_str());

                    MessageDialogBuilder::new("ERROR", error_message.as_str())
                        .kind(dialog::MessageDialogKind::Error)
                        .buttons(dialog::MessageDialogButtons::Ok);
                }
            }
        });

    success_empty()
}
