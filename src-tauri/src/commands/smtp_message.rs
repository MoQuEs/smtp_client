use crate::dialogs::simple_error_dialog;
use crate::response::{
    error, success, MaybeSMTPMessage, NamedSMTPMessage, NamedSMTPMessages, TauriResponse,
};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_messages_command(app_handle: AppHandle) -> TauriResponse<NamedSMTPMessages> {
    log::trace!("get_messages_command");

    match app_handle.db(|db| db.get_messages()) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            log::error!("Error: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_message_command(
    app_handle: AppHandle,
    message: NamedSMTPMessage,
) -> TauriResponse<MaybeSMTPMessage> {
    log::trace!("save_message_command");
    log::debug!("message: {:?}", message);

    match app_handle.db(|db| db.save_message(&message)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!("Error: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn remove_message_command(
    app_handle: AppHandle,
    message: NamedSMTPMessage,
) -> TauriResponse<MaybeSMTPMessage> {
    log::trace!("remove_message_command");
    log::debug!("message: {:?}", message);

    match app_handle.db(|db| db.remove_message(&message)) {
        Ok(data) => success(None, None),
        Err(err) => {
            log::error!("Error: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
