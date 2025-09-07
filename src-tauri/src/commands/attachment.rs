use crate::commands::db_to_response;
use crate::database::AttachmentDatabase;
use crate::dialogs::blocking::pick_file_dialog;
use crate::dialogs::simple_error_dialog;
use crate::file::file_get_contents;
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
    mut add_attachment: AddAttachment,
) -> TauriResponse<NamedAttachment> {
    log::trace!("add_attachment_command");
    log::debug!("add_attachment: {:?}", add_attachment);

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

    let attachment = match &add_attachment.from {
        AddAttachmentFrom::File => Attachment {
            path: file_path.to_string(),
            name: path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "ERROR".to_string()),
            extension: path
                .extension()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "ERROR".to_string()),
            mime: match mime.first() {
                Some(m) => m.to_string(),
                None => "application/octet-stream".to_string(),
            },
            size: binary.len() as u32,
            binary,
        },
        AddAttachmentFrom::Url => {
            if add_attachment.url.is_none() {
                return error(
                    Some("URL is required for AddAttachmentFrom::Url".to_string()),
                    None,
                );
            }
            let path = add_attachment.url.take().unwrap();
            let url_parsed = match url::Url::parse(&path) {
                Ok(u) => u,
                Err(e) => {
                    return error(Some(format!("Invalid URL: {}", e)), None);
                }
            };
            let response = match reqwest::blocking::get(url_parsed.clone()) {
                Ok(r) => r,
                Err(e) => {
                    return error(Some(format!("Failed to fetch URL: {}", e)), None);
                }
            };
            if !response.status().is_success() {
                return error(
                    Some(format!("Failed to fetch URL: HTTP {}", response.status())),
                    None,
                );
            }
            // Try to get filename from Content-Disposition header
            let name = response
                .headers()
                .get(reqwest::header::CONTENT_DISPOSITION)
                .and_then(|cd| cd.to_str().ok())
                .and_then(|cd| {
                    let cd_str = cd.to_string();
                    let parts: Vec<&str> = cd_str.split("filename=").collect();
                    if parts.len() > 1 {
                        Some(parts[1].trim_matches('"').to_string())
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    // Fallback to the last segment of the URL path
                    url_parsed
                        .path_segments()
                        .and_then(|segments| segments.last())
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| "downloaded_file".to_string());
            let extension = std::path::Path::new(&name)
                .extension()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "bin".to_string());
            let mime = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|ct| ct.to_str().ok())
                .unwrap_or("application/octet-stream")
                .to_string();
            let binary = match response.bytes() {
                Ok(b) => b.to_vec(),
                Err(e) => {
                    return error(Some(format!("Failed to read response body: {}", e)), None);
                }
            };

            Attachment {
                path,
                name,
                extension,
                mime,
                size: binary.len() as u32,
                binary,
            }
        }
    };

    let named_attachment = NamedAttachment {
        name: add_attachment.name,
        attachment,
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
