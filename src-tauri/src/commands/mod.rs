mod attachment;
mod configuration;
mod export;
mod import;
mod message;
mod send_mail;
mod settings;
mod smime;

use crate::database::Database;
use crate::dialogs::simple_error_dialog;
use crate::response::{error, success, AnyResult, TauriResponse};
use crate::state::{AppHandle, ServiceAccess};

pub use attachment::*;
pub use configuration::*;
pub use export::*;
pub use import::*;
pub use message::*;
pub use send_mail::*;
pub use settings::*;
pub use smime::*;

pub fn db_to_response<F, R>(app_handle: &AppHandle, db_fn: F) -> TauriResponse<R>
where
    F: FnOnce(&Database) -> AnyResult<R>,
{
    match app_handle.db(db_fn) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            log::error!("Error: {:?}", err);
            simple_error_dialog(app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}
