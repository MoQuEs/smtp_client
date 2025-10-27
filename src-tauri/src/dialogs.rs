use crate::state::AppHandle;
use anyhow::Error;
use std::fmt::Debug;
use tauri::Wry;
use tauri_plugin_dialog::{
    DialogExt, FileDialogBuilder, MessageDialogBuilder, MessageDialogButtons, MessageDialogKind,
};

pub fn simple_error_dialog(app_handle: &AppHandle, err: &Error) {
    log::trace!("Error: {:?}", err);

    error_ok_dialog(app_handle, "ERROR", format!("Error: {:?}", err));
}

pub fn error_ok_dialog(app_handle: &AppHandle, title: impl AsRef<str>, message: impl AsRef<str>) {
    log::trace!("error_ok_dialog");
    log::debug!("title: {:?}", title.as_ref());
    log::debug!("message: {:?}", message.as_ref());

    error_dialog_builder(app_handle, title, message)
        .buttons(MessageDialogButtons::OkCustom("OK".into()))
        .show(|_| {});
}

pub fn error_dialog_builder(
    app_handle: &AppHandle,
    title: impl AsRef<str>,
    message: impl AsRef<str>,
) -> MessageDialogBuilder<Wry> {
    log::trace!("error_dialog");
    log::debug!("title: {:?}", title.as_ref());
    log::debug!("message: {:?}", message.as_ref());

    dialog_builder(app_handle, title, message).kind(MessageDialogKind::Error)
}

pub fn confirm_dialog_builder(
    app_handle: &AppHandle,
    title: impl AsRef<str>,
    message: impl AsRef<str>,
) -> MessageDialogBuilder<Wry> {
    log::trace!("error_dialog");
    log::debug!("title: {:?}", title.as_ref());
    log::debug!("message: {:?}", message.as_ref());

    dialog_builder(app_handle, title, message)
        .kind(MessageDialogKind::Info)
        .buttons(MessageDialogButtons::OkCancel)
}

pub fn dialog_builder(
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

pub fn file_dialog_builder(
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

pub mod blocking {
    use crate::dialogs::{dialog_builder, file_dialog_builder};
    use crate::state::AppHandle;
    use std::fmt::Debug;
    use tauri::Wry;
    use tauri_plugin_dialog::{
        DialogExt, FileDialogBuilder, FilePath, MessageDialogBuilder, MessageDialogButtons,
        MessageDialogKind,
    };

    pub fn save_file_dialog(
        app_handle: &AppHandle,
        filter: Vec<(impl AsRef<str> + Debug, &[&str])>,
    ) -> Option<FilePath> {
        log::trace!("save_file_dialog");
        log::debug!("filter: {:?}", filter);

        file_dialog_builder(app_handle, filter).blocking_save_file()
    }

    pub fn pick_file_dialog(
        app_handle: &AppHandle,
        filter: Vec<(impl AsRef<str> + Debug, &[&str])>,
    ) -> Option<FilePath> {
        log::trace!("save_file_dialog");
        log::debug!("filter: {:?}", filter);

        file_dialog_builder(app_handle, filter).blocking_pick_file()
    }

    pub fn confirm_dialog(
        app_handle: &AppHandle,
        title: impl AsRef<str>,
        message: impl AsRef<str>,
    ) -> bool {
        log::trace!("confirm_dialog");
        log::debug!("title: {:?}", title.as_ref());
        log::debug!("message: {:?}", message.as_ref());

        dialog_builder(app_handle, title, message)
            .kind(MessageDialogKind::Info)
            .buttons(MessageDialogButtons::OkCancel)
            .blocking_show()
    }
}
