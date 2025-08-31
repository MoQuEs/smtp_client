use crate::database::{DatabaseTrait, Section};
use crate::migration::MigrationVersion;
use crate::response::{AnyResult, NamedConfiguration, NamedMessage};
use crate::state::{AppHandle, ServiceAccess};

pub fn run(app_handle: &AppHandle) -> AnyResult<MigrationVersion> {
    log::trace!("run");

    app_handle.db(|db| -> AnyResult<()> {
        if db.section_exists(Section::SMTPConfiguration)? {
            let all = db.get_all::<NamedConfiguration>(Section::SMTPConfiguration)?;
            db.insert_all(Section::Configuration, all.as_slice())?;
            db.remove_section(Section::SMTPConfiguration)?;
        }

        Ok(())
    })?;

    app_handle.db(|db| -> AnyResult<()> {
        if db.section_exists(Section::SMTPMessage)? {
            let all = db.get_all::<NamedMessage>(Section::SMTPMessage)?;
            db.insert_all(Section::Message, all.as_slice())?;
            db.remove_section(Section::SMTPMessage)?;
        }

        Ok(())
    })?;

    log::trace!("end run");

    Ok(MigrationVersion::V0_5_0)
}

pub fn undo(app_handle: &AppHandle) -> AnyResult<MigrationVersion> {
    log::trace!("undo");

    app_handle.db(|db| -> AnyResult<()> {
        if db.section_exists(Section::Configuration)? {
            let all = db.get_all::<NamedConfiguration>(Section::Configuration)?;
            db.insert_all(Section::SMTPConfiguration, all.as_slice())?;
            db.remove_section(Section::Configuration)?;
        }

        Ok(())
    })?;

    app_handle.db(|db| -> AnyResult<()> {
        if db.section_exists(Section::Message)? {
            let all = db.get_all::<NamedMessage>(Section::Message)?;
            db.insert_all(Section::SMTPMessage, all.as_slice())?;
            db.remove_section(Section::Message)?;
        }

        Ok(())
    })?;

    log::trace!("end undo");

    Ok(MigrationVersion::V0_4_0)
}
