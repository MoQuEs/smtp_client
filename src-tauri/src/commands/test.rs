use crate::response::{success_empty, TauriResponse};
use crate::state::AppHandle;
use tauri::api::dialog;

#[tauri::command]
pub fn test(app_handle: AppHandle) -> TauriResponse<()> {
    log::trace!(target: "backend::commands::test::test", "test");

    dialog::FileDialogBuilder::default()
        .add_filter("Markdown", &["md"])
        .pick_file(|path_buf| {
            if let Some(p) = path_buf {
                println!("picked file: {:?}", p);
            }
        });

    success_empty()
}
