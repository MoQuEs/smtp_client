use crate::response::{
    error, success, MaybeSMTPMessage, NamedSMTPMessage, SMTPMessages, TauriResponse,
};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_messages_command(app_handle: AppHandle) -> TauriResponse<SMTPMessages> {
    log::trace!(target: "backend::commands::smtp_message::get_messages_command", "get_messages_command");

    match app_handle.db(|db| db.get_messages()) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            log::error!(target: "backend::commands::smtp_message::get_messages_command", "Error: {:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_message_command(
    app_handle: AppHandle,
    message: NamedSMTPMessage,
) -> TauriResponse<MaybeSMTPMessage> {
    log::trace!(target: "backend::commands::smtp_message::save_message_command", "save_message_command");
    log::debug!(target: "backend::commands::smtp_message::save_message_command", "message: {:?}", message);

    match app_handle.db(|db| db.save_message(&message)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!(target: "backend::commands::smtp_message::save_message_command", "Error: {:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn remove_message_command(
    app_handle: AppHandle,
    message: NamedSMTPMessage,
) -> TauriResponse<MaybeSMTPMessage> {
    log::trace!(target: "backend::commands::smtp_message::remove_message_command", "remove_message_command");
    log::debug!(target: "backend::commands::smtp_message::remove_message_command", "message: {:?}", message);

    match app_handle.db(|db| db.remove_message(&message)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!(target: "backend::commands::smtp_message::remove_message_command", "Error: {:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
