use crate::response::{
    error, success, MaybeSMTPMessage, NamedSMTPMessage, SMTPMessages, TauriResponse,
};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_messages_command(app_handle: AppHandle) -> TauriResponse<SMTPMessages> {
    match app_handle.db(|db| db.get_messages()) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_message_command(
    app_handle: AppHandle,
    message: NamedSMTPMessage,
) -> TauriResponse<MaybeSMTPMessage> {
    match app_handle.db(|db| db.save_message(&message)) {
        Ok(data) => success(None, None),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn remove_message_command(
    app_handle: AppHandle,
    message: NamedSMTPMessage,
) -> TauriResponse<MaybeSMTPMessage> {
    match app_handle.db(|db| db.remove_message(&message)) {
        Ok(data) => success(None, None),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
