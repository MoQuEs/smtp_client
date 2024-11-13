use crate::backup::{Backup, BackupData, BackupDataV1};
use crate::dialogs::blocking::save_file_dialog;
use crate::dialogs::simple_error_dialog;
use crate::file::file_put_contents;
use crate::response::{error, success_empty, AnyResult, ImportExportSettings, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};
use rust_utils::log::Log;

#[tauri::command]
pub fn export_command(
    app_handle: AppHandle,
    import_export_settings: ImportExportSettings,
) -> TauriResponse<()> {
    log::trace!("export_command");
    log::debug!("import_export_settings: {:?}", import_export_settings);

    match export(&app_handle, import_export_settings).inspect_err(|err| {
        log::error!("Error: {:?}", err);
    }) {
        Ok(data) => success_empty(),
        Err(err) => {
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

fn export(app_handle: &AppHandle, import_export_settings: ImportExportSettings) -> AnyResult<()> {
    log::trace!("export");

    let serialized_backup = prepare_backup(app_handle, import_export_settings).log_error(
        "backend::commands::export::export",
        "Error preparing backup",
    )?;

    save_backup(app_handle, serialized_backup)
        .log_error("backend::commands::export::export", "Error saving backup")
}

fn save_backup(app_handle: &AppHandle, mut serialized_backup: Vec<u8>) -> AnyResult<()> {
    log::trace!("save_backup");

    let file_path = save_file_dialog(app_handle, vec![("SMTPclient backup file", &["scb"])]);
    if file_path.is_none() {
        log::info!("No file selected");
        return Ok(());
    }

    log::info!("Saving to file: {:?}", file_path);

    file_put_contents(
        file_path.unwrap().into_path()?,
        serialized_backup.as_mut_slice(),
    )
}

fn prepare_backup(
    app_handle: &AppHandle,
    import_export_settings: ImportExportSettings,
) -> AnyResult<Vec<u8>> {
    log::trace!("prepare_backup");

    let mut backup = BackupDataV1::default();

    if import_export_settings.settings {
        log::info!("Added settings");
        backup.settings = Some(app_handle.db(|db| db.get_settings()).log_error(
            "backend::commands::export::prepare_backup",
            "Error getting settings",
        )?);
    }

    if import_export_settings.smtp_messages {
        log::info!("Added messages");
        backup.messages = Some(app_handle.db(|db| db.get_messages()).log_error(
            "backend::commands::export::prepare_backup",
            "Error getting messages",
        )?);
    }

    if import_export_settings.smtp_configurations {
        log::info!("Added configurations");
        backup.configurations = Some(app_handle.db(|db| db.get_configurations()).log_error(
            "backend::commands::export::prepare_backup",
            "Error getting configurations",
        )?);
    }

    Backup::serialize_backup(import_export_settings.password, BackupData::V1(backup))
}
