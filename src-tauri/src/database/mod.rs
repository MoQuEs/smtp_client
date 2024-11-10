#![allow(dead_code)]

mod key_value;
mod settings;
mod smtp_configuration;
mod smtp_message;

use crate::response::AnyResult;
use crate::serialize::{decode, encode, Decode, Encode};
use sled::Tree;
use std::fmt::Debug;
use std::path::Path;
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
        log::trace!("new");
        log::debug!("config: {:?}", config);

        Ok(Self {
            db: sled::open(Path::new(".").join("data.sled"))?,
        })
    }

    fn section(&self, section: impl AsRef<str>) -> AnyResult<Tree> {
        log::trace!("section");
        log::debug!("section: {}", section.as_ref());

        Ok(self.db.open_tree(section.as_ref())?)
    }

    fn insert<T: Encode + Debug>(
        &self,
        section: impl AsRef<str>,
        key: impl AsRef<str>,
        data: &T,
    ) -> AnyResult<()> {
        log::trace!("insert");
        log::debug!("section: {}", section.as_ref());
        log::debug!("key: {}", key.as_ref());
        log::debug!("data: {:?}", data);

        self.section(section)?.insert(key.as_ref(), encode(data)?)?;
        Ok(())
    }

    fn get<T: Decode + Debug>(
        &self,
        section: impl AsRef<str>,
        key: impl AsRef<str>,
    ) -> AnyResult<Option<T>> {
        log::trace!("get");
        log::debug!("section: {}", section.as_ref());
        log::debug!("key: {}", key.as_ref());

        match self.section(section)?.get(key.as_ref())? {
            Some(bytes) => Ok(decode(&bytes)?),
            None => Ok(None),
        }
    }

    fn remove(&self, section: impl AsRef<str>, key: impl AsRef<str>) -> AnyResult<()> {
        log::trace!("remove");
        log::debug!("section: {}", section.as_ref());
        log::debug!("key: {}", key.as_ref());

        self.section(section)?.remove(key.as_ref())?;
        Ok(())
    }

    fn get_all<T: Decode + Debug>(&self, section: impl AsRef<str>) -> AnyResult<Vec<T>> {
        log::trace!("get_all");
        log::debug!("section: {}", section.as_ref());

        let mut res = Vec::new();

        for (_, bytes) in self.section(section)?.iter().flatten() {
            res.push(decode(&bytes)?)
        }

        Ok(res)
    }

    fn contains(&self, section: impl AsRef<str>, key: impl AsRef<str>) -> AnyResult<bool> {
        log::trace!("contains");
        log::debug!("section: {}", section.as_ref());
        log::debug!("key: {}", key.as_ref());

        Ok(self.section(section)?.contains_key(key.as_ref())?)
    }
}
