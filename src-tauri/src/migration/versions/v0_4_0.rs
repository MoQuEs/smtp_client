use crate::migration::MigrationVersion;
use crate::response::AnyResult;
use crate::state::AppHandle;

pub fn run(app_handle: &AppHandle) -> AnyResult<MigrationVersion> {
    log::trace!("run");

    log::trace!("end run");

    Ok(MigrationVersion::V0_4_0)
}
