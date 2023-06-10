#![allow(dead_code)]

use crate::response::{
    AnyResult, MaybeSecret, NamedSMTPConfiguration, NamedSMTPMessage, SMTPConfigurations,
    SMTPMessages, Secret,
};
use rust_utils::ignore::Ignore;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::api::path::app_data_dir;
use tauri::Config;

enum Section {
    SMTPConfiguration,
    SMTPMessage,
    Settings,
    Secrets,
}

impl AsRef<str> for Section {
    fn as_ref(&self) -> &str {
        match self {
            Self::SMTPConfiguration => "smtp_configuration",
            Self::SMTPMessage => "smtp_message",
            Self::Settings => "secrets",
            Self::Secrets => "secrets",
        }
    }
}

pub struct Database {
    db: sled::Db,
}

impl Database {
    pub fn new(config: &Config) -> AnyResult<Self> {
        Ok(Self {
            db: sled::open(app_data_dir(config).unwrap().join("data.sled"))?,
        })
    }

    fn serialize<T: Serialize>(data: &T) -> AnyResult<Vec<u8>> {
        Ok(bincode::serialize(data)?)
    }

    fn deserialize<T: DeserializeOwned>(data: &[u8]) -> AnyResult<T> {
        Ok(bincode::deserialize(data)?)
    }

    fn insert<T: Serialize + DeserializeOwned>(
        &self,
        section: Section,
        key: impl AsRef<str>,
        data: &T,
    ) -> AnyResult<Option<()>> {
        let tree = self.db.open_tree(section.as_ref())?;
        match tree.insert(key.as_ref(), Self::serialize(data)?)? {
            Some(_) => Ok(Some(())),
            None => Ok(None),
        }
    }

    fn get<T: DeserializeOwned>(
        &self,
        section: Section,
        key: impl AsRef<str>,
    ) -> AnyResult<Option<T>> {
        let tree = self.db.open_tree(section.as_ref())?;
        match tree.get(key.as_ref())? {
            Some(bytes) => Ok(Self::deserialize(&bytes)?),
            None => Ok(None),
        }
    }

    fn remove(&self, section: Section, key: impl AsRef<str>) -> AnyResult<Option<()>> {
        let tree = self.db.open_tree(section.as_ref())?;
        match tree.remove(key.as_ref())? {
            Some(bytes) => Ok(Some(())),
            None => Ok(None),
        }
    }

    fn get_all<T: DeserializeOwned>(&self, section: Section) -> AnyResult<Vec<T>> {
        let mut res = Vec::new();

        let tree = self.db.open_tree(section.as_ref())?;
        for (_, bytes) in tree.iter().flatten() {
            res.push(Self::deserialize(&bytes)?)
        }

        Ok(res)
    }

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

    pub fn save_message(&self, message: &NamedSMTPMessage) -> AnyResult<Option<()>> {
        self.insert(Section::SMTPMessage, message.name.as_str(), message)
    }

    pub fn remove_message(&self, message: &NamedSMTPMessage) -> AnyResult<Option<()>> {
        self.remove(Section::SMTPMessage, message.name.as_str())
    }

    pub fn get_messages(&self) -> AnyResult<SMTPMessages> {
        self.get_all(Section::SMTPMessage)
    }

    pub fn get_secret<T>(&self, secret: impl AsRef<str>) -> AnyResult<MaybeSecret<T>> {
        self.get(Section::Secrets, secret)
    }

    pub fn save_secret<T>(&self, secret: &Secret<T>) -> AnyResult<Option<()>> {
        self.insert(Section::Secrets, secret.name.as_str(), secret)
    }

    pub fn remove_secret<T>(&self, secret: &Secret<T>) -> AnyResult<Option<()>> {
        self.remove(Section::Secrets, secret.name.as_str())
    }
}
