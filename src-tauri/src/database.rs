#![allow(dead_code)]

use crate::response::{
    AnyResult, MaybeSMTPConfiguration, MaybeSMTPMessage, NamedSMTPConfiguration, NamedSMTPMessage,
    SMTPConfigurations, SMTPMessages,
};
use bincode::serde::{decode_from_slice, encode_to_vec};
use serde::de::DeserializeOwned;
use serde::Serialize;

const DATABASE: &str = "data.sled";

enum Section {
    SMTPConfiguration,
    SMTPMessage,
    Secrets,
}

impl AsRef<str> for Section {
    fn as_ref(&self) -> &str {
        match self {
            Self::SMTPConfiguration => "smtp_configuration",
            Self::SMTPMessage => "smtp_message",
            Self::Secrets => "secrets",
        }
    }
}

fn config() -> bincode::config::Configuration {
    bincode::config::standard()
}

fn connect() -> AnyResult<sled::Db> {
    Ok(sled::open(DATABASE)?)
}

fn to_bytes<T: Serialize>(data: &T) -> AnyResult<Vec<u8>> {
    Ok(encode_to_vec(data, config())?)
}

fn from_bytes<T: DeserializeOwned>(data: &[u8]) -> AnyResult<T> {
    Ok(decode_from_slice(data, config())?.0)
}

fn insert<T: Serialize + DeserializeOwned>(
    section: Section,
    key: impl AsRef<str>,
    data: &T,
) -> AnyResult<Option<()>> {
    let db = connect()?;
    let tree = db.open_tree(section.as_ref())?;
    match tree.insert(key.as_ref(), to_bytes(data)?)? {
        Some(_) => Ok(Some(())),
        None => Ok(None),
    }
}

fn get<T: DeserializeOwned>(section: Section, key: impl AsRef<str>) -> AnyResult<Option<T>> {
    let db = connect()?;
    let tree = db.open_tree(section.as_ref())?;
    match tree.get(key.as_ref())? {
        Some(bytes) => Ok(from_bytes(&bytes)?),
        None => Ok(None),
    }
}

fn remove(section: Section, key: impl AsRef<str>) -> AnyResult<Option<()>> {
    let db = connect()?;
    let tree = db.open_tree(section.as_ref())?;
    match tree.remove(key.as_ref())? {
        Some(bytes) => Ok(Some(())),
        None => Ok(None),
    }
}

fn get_all<T: DeserializeOwned>(section: Section) -> AnyResult<Vec<T>> {
    let mut res = Vec::new();

    let db = connect()?;
    let tree = db.open_tree(section.as_ref())?;
    for (_, bytes) in tree.iter().flatten() {
        res.push(from_bytes(&bytes)?)
    }

    Ok(res)
}

pub fn save_configuration(configuration: &NamedSMTPConfiguration) -> AnyResult<Option<()>> {
    insert(
        Section::SMTPConfiguration,
        configuration.name.as_str(),
        configuration,
    )
}

pub fn remove_configuration(configuration: &NamedSMTPConfiguration) -> AnyResult<Option<()>> {
    remove(Section::SMTPConfiguration, configuration.name.as_str())
}

pub fn get_configurations() -> AnyResult<SMTPConfigurations> {
    get_all(Section::SMTPConfiguration)
}

pub fn save_message(message: &NamedSMTPMessage) -> AnyResult<Option<()>> {
    insert(Section::SMTPMessage, message.name.as_str(), message)
}

pub fn get_messages() -> AnyResult<SMTPMessages> {
    get_all(Section::SMTPMessage)
}
