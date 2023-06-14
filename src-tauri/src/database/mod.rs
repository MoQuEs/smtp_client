#![allow(dead_code)]

mod secret;
mod settings;
mod smtp_configuration;
mod smtp_message;

use crate::response::AnyResult;
use rust_utils::log_option::LogErrorFromOption;
use rust_utils::log_result::LogErrorFromResult;
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
            db: sled::open(
                app_data_dir(config)
                    .log_error("backend::database::Database::new", "Can't get app data dir")
                    .expect("Can't get app data dir")
                    .join("data.sled"),
            )?,
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
}
