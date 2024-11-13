use crate::database::{Database, Section};
use crate::response::{AnyResult, NamedSMTPConfiguration, NamedSMTPConfigurations};

impl Database {
    pub fn save_configuration(&self, configuration: &NamedSMTPConfiguration) -> AnyResult<()> {
        log::trace!("save_configuration");
        log::debug!("configuration: {:?}", configuration);

        self.insert(
            Section::SMTPConfiguration,
            configuration.name.as_str(),
            configuration,
        )
    }

    pub fn remove_configuration(&self, configuration: &NamedSMTPConfiguration) -> AnyResult<()> {
        log::trace!("remove_configuration");
        log::debug!("configuration: {:?}", configuration);

        self.remove(Section::SMTPConfiguration, configuration.name.as_str())
    }

    pub fn get_configurations(&self) -> AnyResult<NamedSMTPConfigurations> {
        log::trace!("get_configurations");

        self.get_all(Section::SMTPConfiguration)
    }
}
