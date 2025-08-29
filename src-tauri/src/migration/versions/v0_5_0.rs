use crate::database::{DatabaseTrait, Section};
use crate::migration::MigrationVersion;
use crate::response::AnyResult;
use crate::state::{AppHandle, ServiceAccess};

pub fn run(app_handle: &AppHandle) -> AnyResult<MigrationVersion> {
    log::trace!("run");

    app_handle.db(|db| {
        if db.section_exists(Section::SMTPConfiguration)? {
            let all_confs = db.get_all(Section::SMTPConfiguration);
            db.insert_all(Section::Configuration, &all_confs?)?;
            db.remove_section(Section::SMTPConfiguration)?;
        }

        Ok(())
    })?;

    app_handle.db(|db| {
        if db.section_exists(Section::SMTPMessage)? {
            let all_confs = db.get_all(Section::SMTPMessage);
            db.insert_all(Section::Message, &all_confs?)?;
            db.remove_section(Section::SMTPMessage)?;
        }

        Ok(())
    })?;

    log::trace!("end run");

    Ok(MigrationVersion::V0_5_0)
}

pub fn undo(app_handle: &AppHandle) -> AnyResult<MigrationVersion> {
    log::trace!("undo");

    app_handle.db(|db| {
        if db.section_exists(Section::Configuration)? {
            let all_confs = db.get_all(Section::Configuration);
            db.insert_all(Section::SMTPConfiguration, &all_confs?)?;
            db.remove_section(Section::Configuration)?;
        }

        Ok(())
    })?;

    app_handle.db(|db| {
        if db.section_exists(Section::Message)? {
            let all_confs = db.get_all(Section::Message);
            db.insert_all(Section::SMTPMessage, &all_confs?)?;
            db.remove_section(Section::Message)?;
        }

        Ok(())
    })?;

    log::trace!("end undo");

    Ok(MigrationVersion::V0_4_0)
}
