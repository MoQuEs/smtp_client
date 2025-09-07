#![allow(dead_code)]

use bincode::{Decode, Encode};
use rust_utils::string::mask_if_email;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use typeshare::typeshare;

pub type AnyResult<T> = anyhow::Result<T>;

pub trait Named {
    fn name(&self) -> &str;
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct TauriResponse<T> {
    success: bool,
    message: Option<String>,
    data: Option<T>,
}

impl<T> TauriResponse<T> {
    pub fn new(success: bool, message: Option<String>, data: Option<T>) -> Self {
        Self {
            success,
            message,
            data,
        }
    }
}

pub fn success<T>(message: Option<String>, data: Option<T>) -> TauriResponse<T> {
    TauriResponse::new(true, message, data)
}

pub fn success_empty() -> TauriResponse<()> {
    TauriResponse::new(true, None, None)
}

pub fn error<T>(message: Option<String>, data: Option<T>) -> TauriResponse<T> {
    TauriResponse::new(false, message, data)
}

pub fn error_empty() -> TauriResponse<()> {
    TauriResponse::new(false, None, None)
}

#[typeshare]
pub type MaybeConfiguration = Option<NamedConfiguration>;

#[typeshare]
pub type NamedConfigurations = Vec<NamedConfiguration>;

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct NamedConfiguration {
    pub name: String,
    pub configuration: Configuration,
}

impl Named for NamedConfiguration {
    fn name(&self) -> &str {
        &self.name
    }
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct Configuration {
    pub address: ConfigurationAddress,
    pub auth: ConfigurationAuth,
    pub require_ssl: bool,
    pub verify_certificates: bool,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct ConfigurationAddress {
    pub address: String,
    pub port: u16,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Clone)]
pub struct ConfigurationAuth {
    pub use_auth: bool,
    pub user: String,
    pub password: String,
}

impl Debug for ConfigurationAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigurationAuth")
            .field("use_auth", &self.use_auth)
            .field("user", &self.user)
            .field("password", &"***OMITTED***")
            .finish()
    }
}

#[typeshare]
pub type MaybeMessage = Option<NamedMessage>;

#[typeshare]
pub type NamedMessages = Vec<NamedMessage>;

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct NamedMessage {
    pub name: String,
    pub message: Message,
}

impl Named for NamedMessage {
    fn name(&self) -> &str {
        &self.name
    }
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct Message {
    pub to: MessageAddress,
    pub from: MessageAddress,
    pub reply_to: MessageAddress,
    pub cc: MessageAddress,
    pub bcc: MessageAddress,
    pub headers: Vec<MessageHeader>,
    pub subject: String,
    pub body: MessageBody,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Clone)]
pub struct MessageAddress {
    pub name: Option<String>,
    pub email: String,
}

impl Debug for MessageAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_ref().map(mask_if_email);

        f.debug_struct("MessageAddress")
            .field("name", &name)
            .field("email", &mask_if_email(&self.email))
            .finish()
    }
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct MessageHeader {
    pub name: String,
    pub value: String,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct MessageBody {
    pub html: String,
    pub text: String,
    pub convert_html_to_text: bool,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct KeyValue<T> {
    pub name: String,
    pub value: T,
}

impl<T> Named for KeyValue<T> {
    fn name(&self) -> &str {
        &self.name
    }
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct Secret<T> {
    pub name: String,
    pub value: T,
}

impl<T> Named for Secret<T> {
    fn name(&self) -> &str {
        &self.name
    }
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Default, Debug, Clone)]
pub struct Settings {
    pub theme: SettingsTheme,
    pub language: SettingsLanguage,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Default, Debug, Clone)]
pub enum SettingsTheme {
    #[default]
    Dark,
    Light,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Default, Debug, Clone)]
pub enum SettingsLanguage {
    #[default]
    EN,
    PL,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Clone)]
pub struct ImportExportSettings {
    pub password: String,
    pub configurations: bool,
    pub messages: bool,
    pub settings: bool,
}

impl Debug for ImportExportSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportExportSettings")
            .field("password", &"***OMITTED***")
            .field("configurations", &self.configurations)
            .field("messages", &self.messages)
            .field("settings", &self.settings)
            .finish()
    }
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct AddAttachment {
    pub name: String,
    pub from: AddAttachmentFrom,
    pub url: Option<String>,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub enum AddAttachmentFrom {
    File,
    Url,
}

#[typeshare]
pub type MaybeAttachment = Option<NamedAttachment>;

#[typeshare]
pub type NamedAttachments = Vec<NamedAttachment>;

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct NamedAttachment {
    pub name: String,
    pub attachment: Attachment,
}

impl Named for NamedAttachment {
    fn name(&self) -> &str {
        &self.name
    }
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Clone)]
pub struct Attachment {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub mime: String,
    pub size: u32,
    pub binary: Vec<u8>,
}

impl Debug for Attachment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Attachment")
            .field("path", &self.path)
            .field("name", &self.name)
            .field("extension", &self.extension)
            .field("mime", &self.mime)
            .field("size", &self.size)
            .field("binary", &"***OMITTED***".to_string())
            .finish()
    }
}
