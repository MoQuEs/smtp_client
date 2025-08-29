use crate::commands::db_to_response;
use crate::dialogs::simple_error_dialog;
use crate::response::{
    error, success, AnyResult, MaybeConfiguration, NamedConfiguration, NamedConfigurations,
    TauriResponse,
};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_configurations_command(app_handle: AppHandle) -> TauriResponse<NamedConfigurations> {
    log::trace!("get_configurations_command");

    db_to_response(&app_handle, |db| db.get_configurations())
}

#[tauri::command]
pub fn save_configuration_command(
    app_handle: AppHandle,
    configuration: NamedConfiguration,
) -> TauriResponse<MaybeConfiguration> {
    log::trace!("save_configuration_command");
    log::debug!("configuration: {:?}", configuration);

    db_to_response(&app_handle, |db| db.save_configuration(&configuration))
}

#[tauri::command]
pub fn remove_configuration_command(
    app_handle: AppHandle,
    configuration: NamedConfiguration,
) -> TauriResponse<MaybeConfiguration> {
    log::trace!("remove_configuration_command");
    log::debug!("configuration: {:?}", configuration);

    db_to_response(&app_handle, |db| db.remove_configuration(&configuration))
}
