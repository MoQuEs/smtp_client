use crate::dialogs::simple_error_dialog;
use crate::response::{error, success, Settings, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_settings_command(app_handle: AppHandle) -> TauriResponse<Settings> {
    log::trace!("get_settings_command");

    match app_handle.db(|db| db.get_settings()) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            log::error!("Error: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_settings_command(app_handle: AppHandle, settings: Settings) -> TauriResponse<Settings> {
    log::trace!("save_settings_command");
    log::debug!("settings: {:?}", settings);

    match app_handle.db(|db| db.save_settings(&settings)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!("Error: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
