use crate::response::{error, success, Settings, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_settings_command(app_handle: AppHandle) -> TauriResponse<Settings> {
    match app_handle.db(|db| db.get_settings()) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_settings_command(app_handle: AppHandle, settings: Settings) -> TauriResponse<Settings> {
    match app_handle.db(|db| db.save_settings(&settings)) {
        Ok(data) => success(None, None),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
