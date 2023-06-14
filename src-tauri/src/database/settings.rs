use crate::database::{Database, Section};
use crate::response::{AnyResult, Settings};
use rust_utils::ignore::Ignore;

impl Database {
    pub fn get_settings(&self) -> AnyResult<Settings> {
        let settings = self.get(Section::Settings, "settings");
        if let Ok(Some(settings)) = settings {
            return Ok(settings);
        }

        let default = Settings::default();
        self.save_settings(&default).ignore();
        Ok(default)
    }

    pub fn save_settings(&self, settings: &Settings) -> AnyResult<Option<()>> {
        self.insert(Section::Settings, "settings", settings)
    }
}
