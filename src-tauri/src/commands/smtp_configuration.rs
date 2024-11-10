use crate::dialogs::simple_error_dialog;
use crate::response::{
    error, success, MaybeSMTPConfiguration, NamedSMTPConfiguration, SMTPConfigurations,
    TauriResponse,
};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_configurations_command(app_handle: AppHandle) -> TauriResponse<SMTPConfigurations> {
    log::trace!("get_configurations_command");

    match app_handle.db(|db| db.get_configurations()) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            log::error!("Error: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_configuration_command(
    app_handle: AppHandle,
    configuration: NamedSMTPConfiguration,
) -> TauriResponse<MaybeSMTPConfiguration> {
    log::trace!("save_configuration_command");
    log::debug!("configuration: {:?}", configuration);

    match app_handle.db(|db| db.save_configuration(&configuration)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!("Error: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn remove_configuration_command(
    app_handle: AppHandle,
    configuration: NamedSMTPConfiguration,
) -> TauriResponse<MaybeSMTPConfiguration> {
    log::trace!("remove_configuration_command");
    log::debug!("configuration: {:?}", configuration);

    match app_handle.db(|db| db.remove_configuration(&configuration)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!("Error: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
