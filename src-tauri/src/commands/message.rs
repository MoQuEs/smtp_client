use crate::commands::db_to_response;
use crate::dialogs::simple_error_dialog;
use crate::response::{error, success, MaybeMessage, NamedMessage, NamedMessages, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_messages_command(app_handle: AppHandle) -> TauriResponse<NamedMessages> {
    log::trace!("get_messages_command");

    db_to_response(&app_handle, |db| db.get_messages())
}

#[tauri::command]
pub fn save_message_command(
    app_handle: AppHandle,
    message: NamedMessage,
) -> TauriResponse<MaybeMessage> {
    log::trace!("save_message_command");
    log::debug!("message: {:?}", message);

    db_to_response(&app_handle, |db| db.save_message(&message))
}

#[tauri::command]
pub fn remove_message_command(
    app_handle: AppHandle,
    message: NamedMessage,
) -> TauriResponse<MaybeMessage> {
    log::trace!("remove_message_command");
    log::debug!("message: {:?}", message);

    db_to_response(&app_handle, |db| db.remove_message(&message))
}
