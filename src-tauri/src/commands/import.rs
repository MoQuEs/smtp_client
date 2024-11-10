use crate::response::{success_empty, ImportExportSettings, TauriResponse};
use crate::state::AppHandle;

#[tauri::command]
pub fn import_command(
    app_handle: AppHandle,
    import_export_settings: ImportExportSettings,
) -> TauriResponse<()> {
    log::trace!("import_command");
    log::debug!("import_export_settings: {:?}", import_export_settings);

    success_empty()
}
