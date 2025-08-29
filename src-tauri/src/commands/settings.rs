use crate::commands::db_to_response;
use crate::dialogs::simple_error_dialog;
use crate::response::{error, success, Settings, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_settings_command(app_handle: AppHandle) -> TauriResponse<Settings> {
    log::trace!("get_settings_command");

    db_to_response(&app_handle, |db| db.get_settings())
}

#[tauri::command]
pub fn save_settings_command(app_handle: AppHandle, settings: Settings) -> TauriResponse<Settings> {
    log::trace!("save_settings_command");
    log::debug!("settings: {:?}", settings);

    db_to_response(&app_handle, |db| db.save_settings(&settings))
}
