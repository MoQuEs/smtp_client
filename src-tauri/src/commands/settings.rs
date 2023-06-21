use crate::response::{error, success, Settings, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_settings_command(app_handle: AppHandle) -> TauriResponse<Settings> {
    log::trace!(target: "backend::commands::settings::get_settings_command", "get_settings_command");

    match app_handle.db(|db| db.get_settings()) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            log::error!(target: "backend::commands::settings::get_settings_command", "Error: {:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_settings_command(app_handle: AppHandle, settings: Settings) -> TauriResponse<Settings> {
    log::trace!(target: "backend::commands::settings::save_settings_command", "save_settings_command");
    log::debug!(target: "backend::commands::settings::save_settings_command", "settings: {:?}", settings);

    match app_handle.db(|db| db.save_settings(&settings)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!(target: "backend::commands::settings::save_settings_command", "Error: {:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
