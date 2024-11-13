use crate::backup::{Backup, BackupData};
use crate::dialogs::blocking::pick_file_dialog;
use crate::dialogs::simple_error_dialog;
use crate::file::file_get_contents;
use crate::response::{error, success_empty, AnyResult, ImportExportSettings, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};
use anyhow::anyhow;
use rust_utils::log::Log;

#[tauri::command]
pub fn import_command(
    app_handle: AppHandle,
    import_export_settings: ImportExportSettings,
) -> TauriResponse<()> {
    log::trace!("import_command");
    log::debug!("import_export_settings: {:?}", import_export_settings);

    match import(&app_handle, import_export_settings).inspect_err(|err| {
        log::error!("Error: {:?}", err);
    }) {
        Ok(data) => success_empty(),
        Err(err) => {
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

fn import(app_handle: &AppHandle, import_export_settings: ImportExportSettings) -> AnyResult<()> {
    log::trace!("import");

    let backup_data = load_backup(app_handle)
        .log_error("backend::commands::export::export", "Error saving backup")?
        .ok_or(anyhow!("No backup data"))?;

    import_backup(app_handle, import_export_settings, backup_data).log_error(
        "backend::commands::export::export",
        "Error preparing backup",
    )
}

fn load_backup(app_handle: &AppHandle) -> AnyResult<Option<Vec<u8>>> {
    log::trace!("save_backup");

    let file_path = pick_file_dialog(app_handle, vec![("SMTPclient backup file", &["scb"])]);
    if file_path.is_none() {
        log::info!("No file selected");
        return Ok(None);
    }

    log::info!("Opening file: {:?}", file_path);

    Ok(Some(file_get_contents(file_path.unwrap().into_path()?)?))
}

fn import_backup(
    app_handle: &AppHandle,
    import_export_settings: ImportExportSettings,
    backup_data: Vec<u8>,
) -> AnyResult<()> {
    log::trace!("prepare_backup");

    let backup = Backup::deserialize_backup(import_export_settings.password, backup_data)?;

    let (configurations, messages, settings) = match backup {
        BackupData::V1(backup) => (backup.configurations, backup.messages, backup.settings),
    };

    if let Some(configurations) = configurations {
        log::info!("Importing configurations");

        for configuration in configurations {
            app_handle.db(|db| db.save_configuration(&configuration))?;
        }
    }

    if let Some(messages) = messages {
        log::info!("Importing messages");

        for message in messages {
            app_handle.db(|db| db.save_message(&message))?;
        }
    }

    if let Some(settings) = settings {
        log::info!("Importing configurations");
        app_handle.db(|db| db.save_settings(&settings))?;
    }

    Ok(())
}
