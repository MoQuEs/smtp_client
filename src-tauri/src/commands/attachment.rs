use crate::commands::db_to_response;
use crate::database::AttachmentDatabase;
use crate::dialogs::blocking::pick_file_dialog;
use crate::dialogs::simple_error_dialog;
use crate::file::{file_get_contents, get_file_data_from_dialog, get_file_data_from_request};
use crate::response::{
    error, success, AddAttachment, AddAttachmentFrom, Attachment, NamedAttachment,
    NamedAttachments, TauriResponse,
};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_attachments_command(app_handle: AppHandle) -> TauriResponse<NamedAttachments> {
    log::trace!("get_configurations_command");

    db_to_response(&app_handle, |db| db.get_attachments())
}

#[tauri::command]
pub fn add_attachment_command(
    app_handle: AppHandle,
    add_attachment: AddAttachment,
) -> TauriResponse<NamedAttachment> {
    log::trace!("add_attachment_command");
    log::debug!("add_attachment: {:?}", add_attachment);

    let attachment_data = match &add_attachment.from {
        AddAttachmentFrom::File => {
            get_file_data_from_dialog(&app_handle, vec![("All Files", &["*"])])
        }
        AddAttachmentFrom::Url => get_file_data_from_request(add_attachment.url),
    };

    let file_data = match attachment_data {
        Ok(data) => data,
        Err(err) => {
            log::error!("Failed to get file data: {:?}", err);
            return error(Some(format!("Failed to get file data: {:?}", err)), None);
        }
    };

    let named_attachment = NamedAttachment {
        name: add_attachment.name,
        from: add_attachment.from,
        attachment: Attachment {
            path: file_data.path,
            name: file_data.name,
            extension: file_data.extension,
            mime: file_data.mime,
            size: file_data.size,
            binary: file_data.binary,
        },
    };

    match app_handle.db(|db| db.save_attachment(&named_attachment)) {
        Ok(_) => success(None, Some(named_attachment)),
        Err(err) => {
            log::error!("Failed to save attachment: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("Failed to save attachment: {:?}", err)), None)
        }
    }
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
