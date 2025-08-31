use crate::commands::db_to_response;
use crate::database::AttachmentDatabase;
use crate::dialogs::blocking::pick_file_dialog;
use crate::file::file_get_contents;
use crate::response::{
    error, Attachment, NamedAttachment, NamedAttachments, TauriResponse, ToSaveAttachment,
};
use crate::state::AppHandle;

#[tauri::command]
pub fn get_attachments_command(app_handle: AppHandle) -> TauriResponse<NamedAttachments> {
    log::trace!("get_configurations_command");

    db_to_response(&app_handle, |db| db.get_attachments())
}

#[tauri::command]
pub fn add_attachment_command(
    app_handle: AppHandle,
    to_save_attachment: ToSaveAttachment,
) -> TauriResponse<()> {
    log::trace!("save_attachment_command");
    log::debug!("to_save_attachment: {:?}", to_save_attachment);

    let file_path = pick_file_dialog(&app_handle, vec![("All Files", &["*"])]);
    if file_path.is_none() {
        log::info!("No file selected");
        return error(Some("No file selected".to_string()), None);
    }

    let file_path = file_path.unwrap();
    log::info!("Opening file: {:?}", file_path);

    let path = match file_path.as_path() {
        Some(p) => p,
        None => {
            return error(Some("Invalid file path".to_string()), None);
        }
    };

    let mime = mime_guess::from_path(&path);
    let binary = match file_get_contents(&path) {
        Ok(b) => b,
        Err(e) => {
            return error(Some(format!("Failed to read file: {}", e)), None);
        }
    };

    let attachment = NamedAttachment {
        name: to_save_attachment.name,
        attachment: Attachment {
            path: file_path.to_string(),
            mime: match mime.first() {
                Some(m) => m.to_string(),
                None => "application/octet-stream".to_string(),
            },
            binary,
        },
    };

    db_to_response(&app_handle, |db| db.save_attachment(&attachment))
}

#[tauri::command]
pub fn remove_attachment_command(
    app_handle: AppHandle,
    attachment: NamedAttachment,
) -> TauriResponse<()> {
    log::trace!("remove_attachment_command");
    log::debug!("attachment: {:?}", attachment);

    db_to_response(&app_handle, |db| db.remove_attachment(&attachment))
}
