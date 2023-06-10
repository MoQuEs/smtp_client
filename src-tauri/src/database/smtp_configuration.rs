use crate::database::{Database, Section};
use crate::response::{AnyResult, NamedSMTPConfiguration, SMTPConfigurations};

impl Database {
    pub fn save_configuration(
        &self,
        configuration: &NamedSMTPConfiguration,
    ) -> AnyResult<Option<()>> {
        self.insert(
            Section::SMTPConfiguration,
            configuration.name.as_str(),
            configuration,
        )
    }

    pub fn remove_configuration(
        &self,
        configuration: &NamedSMTPConfiguration,
    ) -> AnyResult<Option<()>> {
        self.remove(Section::SMTPConfiguration, configuration.name.as_str())
    }

    pub fn get_configurations(&self) -> AnyResult<SMTPConfigurations> {
        self.get_all(Section::SMTPConfiguration)
    }
}
