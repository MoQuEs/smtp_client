use crate::response::{
    error, success, MaybeSMTPConfiguration, NamedSMTPConfiguration, SMTPConfigurations,
    TauriResponse,
};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_configurations_command(app_handle: AppHandle) -> TauriResponse<SMTPConfigurations> {
    log::trace!(target: "backend::commands::smtp_configuration::get_configurations_command", "get_configurations_command");

    match app_handle.db(|db| db.get_configurations()) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            log::error!(target: "backend::commands::smtp_configuration::get_configurations_command", "Error: {:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_configuration_command(
    app_handle: AppHandle,
    configuration: NamedSMTPConfiguration,
) -> TauriResponse<MaybeSMTPConfiguration> {
    log::trace!(target: "backend::commands::smtp_configuration::save_configuration_command", "save_configuration_command");
    log::debug!(target: "backend::commands::smtp_configuration::save_configuration_command", "configuration: {:?}", configuration);

    match app_handle.db(|db| db.save_configuration(&configuration)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!(target: "backend::commands::smtp_configuration::save_configuration_command", "Error: {:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn remove_configuration_command(
    app_handle: AppHandle,
    configuration: NamedSMTPConfiguration,
) -> TauriResponse<MaybeSMTPConfiguration> {
    log::trace!(target: "backend::commands::smtp_configuration::remove_configuration_command", "remove_configuration_command");
    log::debug!(target: "backend::commands::smtp_configuration::remove_configuration_command", "configuration: {:?}", configuration);

    match app_handle.db(|db| db.remove_configuration(&configuration)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!(target: "backend::commands::smtp_configuration::remove_configuration_command", "Error: {:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
