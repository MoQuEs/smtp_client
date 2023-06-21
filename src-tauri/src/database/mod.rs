#![allow(dead_code)]

mod key_value;
mod settings;
mod smtp_configuration;
mod smtp_message;

use crate::response::AnyResult;
use crate::serialize::{deserialize, serialize};
use rust_utils::log::LogU;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use tauri::api::path::app_data_dir;
use tauri::Config;

#[derive(Debug, Clone, Copy)]
enum Section {
    SMTPConfiguration,
    SMTPMessage,
    Settings,
    KeyValue,
}

impl AsRef<str> for Section {
    fn as_ref(&self) -> &str {
        match self {
            Self::SMTPConfiguration => "smtp_configuration",
            Self::SMTPMessage => "smtp_message",
            Self::Settings => "settings",
            Self::KeyValue => "key_value",
        }
    }
}

pub struct Database {
    db: sled::Db,
}

impl Database {
    pub fn new(config: &Config) -> AnyResult<Self> {
        log::trace!(target: "backend::database::Database::new", "new");

        log::debug!(target: "backend::database::Database::new", "config: {:?}", config);
        log::debug!(target: "backend::database::Database::new", "app_data_dir: {:?}", app_data_dir(config));

        Ok(Self {
            db: sled::open(
                app_data_dir(config)
                    .log_error_u("backend::database::Database::new", "Can't get app data dir")
                    .join("data.sled"),
            )?,
        })
    }

    fn insert<T: Serialize + Debug>(
        &self,
        section: Section,
        key: impl AsRef<str>,
        data: &T,
    ) -> AnyResult<()> {
        log::trace!(target: "backend::database::Database::insert", "insert");
        log::debug!(target: "backend::database::Database::insert", "section: {:?}", section);
        log::debug!(target: "backend::database::Database::insert", "key: {}", key.as_ref());
        log::debug!(target: "backend::database::Database::insert", "data: {:?}", data);

        let tree = self.db.open_tree(section.as_ref())?;
        tree.insert(key.as_ref(), serialize(data)?)?;
        Ok(())
    }

    fn get<T: DeserializeOwned + Debug>(
        &self,
        section: Section,
        key: impl AsRef<str>,
    ) -> AnyResult<Option<T>> {
        log::trace!(target: "backend::database::Database::get", "get");
        log::debug!(target: "backend::database::Database::get", "section: {:?}", section);
        log::debug!(target: "backend::database::Database::get", "key: {}", key.as_ref());

        let tree = self.db.open_tree(section.as_ref())?;
        match tree.get(key.as_ref())? {
            Some(bytes) => Ok(deserialize(&bytes)?),
            None => Ok(None),
        }
    }

    fn remove(&self, section: Section, key: impl AsRef<str>) -> AnyResult<()> {
        log::trace!(target: "backend::database::Database::remove", "remove");
        log::debug!(target: "backend::database::Database::remove", "section: {:?}", section);
        log::debug!(target: "backend::database::Database::remove", "key: {}", key.as_ref());

        let tree = self.db.open_tree(section.as_ref())?;
        tree.remove(key.as_ref())?;
        Ok(())
    }

    fn get_all<T: DeserializeOwned + Debug>(&self, section: Section) -> AnyResult<Vec<T>> {
        log::trace!(target: "backend::database::Database::get_all", "get_all");
        log::debug!(target: "backend::database::Database::get_all", "section: {:?}", section);

        let mut res = Vec::new();

        let tree = self.db.open_tree(section.as_ref())?;
        for (_, bytes) in tree.iter().flatten() {
            res.push(deserialize(&bytes)?)
        }

        Ok(res)
    }
}
