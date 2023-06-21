use crate::database::{Database, Section};
use crate::response::{AnyResult, Settings};

impl Database {
    pub fn get_settings(&self) -> AnyResult<Settings> {
        log::trace!(target: "backend::database::settings::Database::get_settings", "get_settings");

        let settings = self.get(Section::Settings, "settings");
        if let Ok(Some(settings)) = settings {
            log::debug!(target: "backend::database::settings::Database::get_settings", "settings: {:?}", settings);
            return Ok(settings);
        }

        log::info!(target: "backend::database::settings::Database::get_settings", "settings not found create default");

        let default = Settings::default();
        self.save_settings(&default)?;
        Ok(default)
    }

    pub fn save_settings(&self, settings: &Settings) -> AnyResult<()> {
        log::trace!(target: "backend::database::settings::Database::save_settings", "save_settings");
        log::debug!(target: "backend::database::settings::Database::save_settings", "settings: {:?}", settings);

        self.insert(Section::Settings, "settings", settings)
    }
}
