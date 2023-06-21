#![allow(dead_code)]

use crate::crypt::encrypt::{encrypt_data, generate_crypt_key_from_string};
use crate::response::{AnyResult, SMTPConfigurations, SMTPMessages, Settings};
use crate::serialize::serialize;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Backup {
    is_secure: bool,
    data: BackupVersionData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
enum BackupVersionData {
    V1(Vec<u8>),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BackupDataV1 {
    settings: Option<Settings>,
    configurations: Option<SMTPConfigurations>,
    messages: Option<SMTPMessages>,
}

impl Backup {
    fn new(is_secure: bool, data: BackupVersionData) -> Self {
        log::trace!(target: "backend::backup::Backup::new", "new");
        log::debug!(target: "backend::backup::Backup::new", "is_secure: {:?}", is_secure);
        log::debug!(target: "backend::backup::Backup::new", "data: {:?}", data);

        Self { is_secure, data }
    }

    fn serialize_data<T: Serialize + Debug>(data: &T) -> AnyResult<Vec<u8>> {
        log::trace!(target: "backend::backup::Backup::serialize_data", "serialize_data");
        log::debug!(target: "backend::backup::Backup::serialize_data", "data: {:?}", data);

        serialize(data)
    }

    fn encrypt_data(password: impl AsRef<str>, data: Vec<u8>) -> AnyResult<Vec<u8>> {
        log::trace!(target: "backend::backup::Backup::encode_data", "encode_data");
        log::debug!(target: "backend::backup::Backup::encode_data", "password: {:?}", password.as_ref());
        log::debug!(target: "backend::backup::Backup::encode_data", "data: {:?}", data);

        let secret_key = generate_crypt_key_from_string(password)?;
        encrypt_data(&secret_key, &data)
    }

    pub fn v1(password: impl AsRef<str>, backup_data: BackupDataV1) -> AnyResult<Vec<u8>> {
        log::trace!(target: "backend::backup::Backup::v1", "v1");
        log::debug!(target: "backend::backup::Backup::v1", "password: {:?}", password.as_ref());
        log::debug!(target: "backend::backup::Backup::v1", "backup_data: {:?}", backup_data);

        let is_secure = !password.as_ref().is_empty();

        log::debug!(target: "backend::backup::Backup::v1", "is_secure: {:?}", is_secure);

        let mut data = Self::serialize_data(&backup_data)?;
        if is_secure {
            data = Self::encrypt_data(password, data)?;
        }

        let backup = Self::new(is_secure, BackupVersionData::V1(data));
        Self::serialize_data(&backup)
    }
}

impl BackupDataV1 {
    pub fn add_settings(&mut self, settings: Settings) {
        self.settings = Some(settings);
    }

    pub fn add_configurations(&mut self, configurations: SMTPConfigurations) {
        self.configurations = Some(configurations);
    }

    pub fn add_messages(&mut self, messages: SMTPMessages) {
        self.messages = Some(messages);
    }
}
