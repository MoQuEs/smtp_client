use crate::database::{Database, DatabaseTrait, Section};
use crate::response::{AnyResult, NamedConfiguration, NamedConfigurations};

pub trait ConfigurationDatabase {
    fn save_configuration(&self, configuration: &NamedConfiguration) -> AnyResult<()>;
    fn remove_configuration(&self, configuration: &NamedConfiguration) -> AnyResult<()>;
    fn get_configurations(&self) -> AnyResult<NamedConfigurations>;
}

impl ConfigurationDatabase for Database {
    fn save_configuration(&self, configuration: &NamedConfiguration) -> AnyResult<()> {
        log::trace!("save_configuration");
        log::debug!("configuration: {:?}", configuration);

        self.insert(
            Section::Configuration,
            configuration.name.as_str(),
            configuration,
        )
    }

    fn remove_configuration(&self, configuration: &NamedConfiguration) -> AnyResult<()> {
        log::trace!("remove_configuration");
        log::debug!("configuration: {:?}", configuration);

        self.remove(Section::Configuration, configuration.name.as_str())
    }

    fn get_configurations(&self) -> AnyResult<NamedConfigurations> {
        log::trace!("get_configurations");

        self.get_all(Section::Configuration)
    }
}
