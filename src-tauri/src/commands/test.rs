use crate::response::{error, success, success_empty, Settings, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};
use tauri::api::dialog;

#[tauri::command]
pub fn test(app_handle: AppHandle) -> TauriResponse<()> {
    dialog::FileDialogBuilder::default()
        .add_filter("Markdown", &["md"])
        .pick_file(|path_buf| {
            if let Some(p) = path_buf {
                println!("picked file: {:?}", p);
            }
        });

    success_empty()
}
