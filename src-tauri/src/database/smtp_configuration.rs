use crate::database::{Database, Section};
use crate::response::{AnyResult, NamedSMTPConfiguration, SMTPConfigurations};

impl Database {
    pub fn save_configuration(&self, configuration: &NamedSMTPConfiguration) -> AnyResult<()> {
        log::trace!(target: "backend::database::smtp_configuration::Database::save_configuration", "save_configuration");
        log::debug!(target: "backend::database::smtp_configuration::Database::save_configuration", "configuration: {:?}", configuration);

        self.insert(
            Section::SMTPConfiguration,
            configuration.name.as_str(),
            configuration,
        )
    }

    pub fn remove_configuration(&self, configuration: &NamedSMTPConfiguration) -> AnyResult<()> {
        log::trace!(target: "backend::database::smtp_configuration::Database::remove_configuration", "remove_configuration");
        log::debug!(target: "backend::database::smtp_configuration::Database::remove_configuration", "configuration: {:?}", configuration);

        self.remove(Section::SMTPConfiguration, configuration.name.as_str())
    }

    pub fn get_configurations(&self) -> AnyResult<SMTPConfigurations> {
        log::trace!(target: "backend::database::smtp_configuration::Database::get_configurations", "get_configurations");

        self.get_all(Section::SMTPConfiguration)
    }
}
