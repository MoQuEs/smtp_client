#![allow(dead_code)]

use crate::crypt::encrypt::{decrypt_data, encrypt_data, generate_crypt_key_from_string};
use crate::response::{AnyResult, SMTPConfigurations, SMTPMessages, Settings};
use crate::serialize::{decode, encode};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Encode, Decode, Debug)]
pub struct Backup {
    is_secure: bool,
    data: BackupDataExport,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug)]
#[serde(tag = "t", content = "c")]
enum BackupDataExport {
    V1(Vec<u8>),
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug)]
#[serde(tag = "t", content = "c")]
pub enum BackupDataImport {
    V1(BackupDataV1),
}

#[derive(Serialize, Deserialize, Encode, Decode, Default, Debug)]
pub struct BackupDataV1 {
    pub settings: Option<Settings>,
    pub configurations: Option<SMTPConfigurations>,
    pub messages: Option<SMTPMessages>,
}

impl Backup {
    fn serialize_data<T: self::Encode + Debug>(data: &T) -> AnyResult<Vec<u8>> {
        log::trace!("serialize_data");
        log::debug!("data: ***OMITTED***");

        encode(data)
    }

    fn deserialize_data<T: self::Decode + Debug>(data: &[u8]) -> AnyResult<T> {
        log::trace!("deserialize_data");
        log::debug!("data: ***OMITTED***");

        decode(data)
    }

    fn encrypt_data(password: impl AsRef<str>, data: Vec<u8>) -> AnyResult<Vec<u8>> {
        log::trace!("encode_data");
        log::debug!("data: ***OMITTED***");

        let secret_key = generate_crypt_key_from_string(password)?;
        encrypt_data(&secret_key, &data)
    }

    fn decrypt_data(password: impl AsRef<str>, data: Vec<u8>) -> AnyResult<Vec<u8>> {
        log::trace!("decrypt_data");
        log::debug!("data: ***OMITTED***");

        let secret_key = generate_crypt_key_from_string(password)?;
        decrypt_data(&secret_key, &data)
    }

    pub fn serialize_backup(
        password: impl AsRef<str>,
        backup_data: BackupDataV1,
    ) -> AnyResult<Vec<u8>> {
        log::trace!("serialize_backup");
        log::debug!("password: ***OMITTED***");
        log::debug!("backup_data: {:?}", backup_data);

        let is_secure = !password.as_ref().is_empty();

        log::debug!("is_secure: {:?}", is_secure);

        let mut data = Self::serialize_data(&backup_data)?;
        if is_secure {
            data = Self::encrypt_data(password, data)?;
        }

        let backup = Self {
            is_secure,
            data: BackupDataExport::V1(data),
        };
        Self::serialize_data(&backup)
    }

    pub fn unserialize_backup(
        password: impl AsRef<str>,
        backup_data: Vec<u8>,
    ) -> AnyResult<BackupDataImport> {
        log::trace!("unserialize_backup");
        log::debug!("password: ***OMITTED***");
        log::debug!("backup_data: ***OMITTED***");

        let is_secure = !password.as_ref().is_empty();

        log::debug!("is_secure: {:?}", is_secure);

        let data = Self::deserialize_data::<Backup>(&backup_data)?;
        let mut backup_data = Vec::new();

        log::debug!("is_secure inner: {:?}", data.is_secure);

        if data.is_secure {
            backup_data = match data.data {
                BackupDataExport::V1(data) => Self::decrypt_data(password, data)?,
            };
        }

        Self::deserialize_data::<BackupDataImport>(&backup_data)
    }
}
