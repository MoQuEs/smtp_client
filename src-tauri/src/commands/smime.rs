use crate::commands::db_to_response;
use crate::database::SmimeDatabase;
use crate::dialogs::blocking::{confirm_dialog, pick_file_dialog};
use crate::dialogs::simple_error_dialog;
use crate::file::{file_get_contents, get_file_data_from_dialog, get_file_data_from_request};
use crate::response::{
    error, success, AddSmime, AddSmimeFrom, NamedSmime, NamedSmimes, Smime, TauriResponse,
};
use crate::state::{AppHandle, ServiceAccess};

#[tauri::command]
pub fn get_smime_command(app_handle: AppHandle) -> TauriResponse<NamedSmimes> {
    log::trace!("get_configurations_command");

    db_to_response(&app_handle, |db| db.get_smimes())
}

#[tauri::command]
pub fn add_smime_command(app_handle: AppHandle, add_smime: AddSmime) -> TauriResponse<NamedSmime> {
    log::trace!("add_smime_command");
    log::debug!("add_smime: {:?}", add_smime);

    let (key_data, cert_data, additional_certs_data) = match &add_smime.from {
        AddSmimeFrom::PKCS12 => {
            let file_data = match get_file_data_from_dialog(&app_handle, vec![]) {
                Ok(data) => data,
                Err(err) => {
                    log::error!("Failed to get file data: {:?}", err);
                    return error(Some(format!("Failed to get file data: {:?}", err)), None);
                }
            };

            unimplemented!();
        }
        AddSmimeFrom::Separate => {
            let key_data = match get_file_data_from_dialog(&app_handle, vec![]) {
                Ok(data) => data,
                Err(err) => {
                    log::error!("Failed to get key data: {:?}", err);
                    return error(Some(format!("Failed to get key data: {:?}", err)), None);
                }
            };

            let cert_data = match get_file_data_from_dialog(&app_handle, vec![]) {
                Ok(data) => data,
                Err(err) => {
                    log::error!("Failed to get cert data: {:?}", err);
                    return error(Some(format!("Failed to get cert data: {:?}", err)), None);
                }
            };

            let mut additional_certs_data = vec![];
            while confirm_dialog(
                &app_handle,
                "Additional Certificates",
                "Do you want to add a certificate?",
            ) {
                let cert = match get_file_data_from_dialog(&app_handle, vec![]) {
                    Ok(data) => data,
                    Err(err) => {
                        log::error!("Failed to get CA cert data: {:?}", err);
                        return error(Some(format!("Failed to get CA cert data: {:?}", err)), None);
                    }
                };

                additional_certs_data.push(cert.binary);
            }

            (key_data.binary, cert_data.binary, additional_certs_data)
        }
    };

    let named_smime = NamedSmime {
        name: add_smime.name,
        from: add_smime.from,
        smime: Smime {
            email: add_smime.email,
            key_data: vec![],
            cert_data: vec![],
            additional_certs_data: vec![],
        },
    };

    match app_handle.db(|db| db.save_smime(&named_smime)) {
        Ok(_) => success(None, Some(named_smime)),
        Err(err) => {
            log::error!("Failed to save smime: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("Failed to save smime: {:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn remove_smime_command(app_handle: AppHandle, smime: NamedSmime) -> TauriResponse<()> {
    log::trace!("remove_smime_command");
    log::debug!("smime: {:?}", smime);

    db_to_response(&app_handle, |db| db.remove_smime(&smime))
}
