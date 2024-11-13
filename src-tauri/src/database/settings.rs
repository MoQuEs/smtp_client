use crate::database::{Database, Section};
use crate::response::{AnyResult, Settings};

impl Database {
    pub fn get_settings(&self) -> AnyResult<Settings> {
        log::trace!("get_settings");

        let settings = self.get(Section::Settings, "settings");
        log::debug!("settings: {:?}", settings);
        if let Ok(Some(settings)) = settings {
            log::debug!("settings: {:?}", settings);
            return Ok(settings);
        }

        log::info!("settings not found create default");

        let default = Settings::default();
        self.save_settings(&default)?;

        let settings = self.get(Section::Settings, "settings");
        log::debug!("settings: {:?}", settings);
        if let Ok(Some(settings)) = settings {
            log::debug!("settings: {:?}", settings);
            return Ok(settings);
        }

        Ok(default)
    }

    pub fn save_settings(&self, settings: &Settings) -> AnyResult<()> {
        log::trace!("save_settings");
        log::debug!("settings: {:?}", settings);

        self.insert(Section::Settings, "settings", settings)
    }
}
