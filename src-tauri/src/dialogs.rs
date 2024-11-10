use crate::state::AppHandle;
use anyhow::Error;
use tauri::Wry;
use tauri_plugin_dialog::{
    DialogExt, MessageDialogBuilder, MessageDialogButtons, MessageDialogKind,
};

pub fn simple_error_dialog(app_handle: &AppHandle, err: &Error) {
    log::trace!("Error: {:?}", err);

    error_ok_dialog(app_handle, "ERROR", format!("Error: {:?}", err));
}

pub fn error_ok_dialog(app_handle: &AppHandle, title: impl AsRef<str>, message: impl AsRef<str>) {
    log::trace!("error_ok_dialog");
    log::debug!("title: {:?}", title.as_ref());
    log::debug!("message: {:?}", message.as_ref());

    error_dialog_b(app_handle, title, message)
        .buttons(MessageDialogButtons::OkCustom("OK".into()))
        .show(|_| {});
}

pub fn error_dialog_b(
    app_handle: &AppHandle,
    title: impl AsRef<str>,
    message: impl AsRef<str>,
) -> MessageDialogBuilder<Wry> {
    log::trace!("error_dialog");
    log::debug!("title: {:?}", title.as_ref());
    log::debug!("message: {:?}", message.as_ref());

    dialog_b(app_handle, title, message).kind(MessageDialogKind::Error)
}

pub fn dialog_b(
    app_handle: &AppHandle,
    title: impl AsRef<str>,
    message: impl AsRef<str>,
) -> MessageDialogBuilder<Wry> {
    log::trace!("dialog");

    MessageDialogBuilder::new(
        app_handle.dialog().clone(),
        title.as_ref(),
        message.as_ref(),
    )
}

pub mod blocking {
    use crate::state::AppHandle;
    use std::fmt::Debug;
    use tauri::Wry;
    use tauri_plugin_dialog::{DialogExt, FileDialogBuilder, FilePath};

    pub fn save_file_dialog(
        app_handle: &AppHandle,
        filter: Vec<(impl AsRef<str> + Debug, &[&str])>,
    ) -> Option<FilePath> {
        log::trace!("save_file_dialog");
        log::debug!("filter: {:?}", filter);

        file_dialog_b(app_handle, filter).blocking_save_file()
    }

    pub fn pick_file_dialog(
        app_handle: &AppHandle,
        filter: Vec<(impl AsRef<str> + Debug, &[&str])>,
    ) -> Option<FilePath> {
        log::trace!("save_file_dialog");
        log::debug!("filter: {:?}", filter);

        file_dialog_b(app_handle, filter).blocking_pick_file()
    }

    pub fn file_dialog_b(
        app_handle: &AppHandle,
        filter: Vec<(impl AsRef<str> + Debug, &[&str])>,
    ) -> FileDialogBuilder<Wry> {
        log::trace!("file_dialog");
        log::debug!("filter: {:?}", filter);

        let mut file_dialog = FileDialogBuilder::new(app_handle.dialog().clone());
        for (name, extensions) in filter {
            file_dialog = file_dialog.add_filter(name.as_ref(), extensions);
        }

        file_dialog
    }
}
