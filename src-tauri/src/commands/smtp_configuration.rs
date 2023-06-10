use crate::response::{
    error, success, MaybeSMTPConfiguration, NamedSMTPConfiguration, SMTPConfigurations,
    TauriResponse,
};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_configurations_command(app_handle: AppHandle) -> TauriResponse<SMTPConfigurations> {
    match app_handle.db(|db| db.get_configurations()) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_configuration_command(
    app_handle: AppHandle,
    configuration: NamedSMTPConfiguration,
) -> TauriResponse<MaybeSMTPConfiguration> {
    match app_handle.db(|db| db.save_configuration(&configuration)) {
        Ok(data) => success(None, None),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn remove_configuration_command(
    app_handle: AppHandle,
    configuration: NamedSMTPConfiguration,
) -> TauriResponse<MaybeSMTPConfiguration> {
    match app_handle.db(|db| db.remove_configuration(&configuration)) {
        Ok(data) => success(None, None),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
