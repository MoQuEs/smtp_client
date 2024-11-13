#![allow(dead_code)]

use bincode::{Decode, Encode};
use rust_utils::string::mask_if_email;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use typeshare::typeshare;

pub type AnyResult<T> = anyhow::Result<T>;

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

pub type MaybeSMTPConfiguration = Option<NamedSMTPConfiguration>;
pub type NamedSMTPConfigurations = Vec<NamedSMTPConfiguration>;
pub type MaybeSMTPMessage = Option<NamedSMTPMessage>;
pub type NamedSMTPMessages = Vec<NamedSMTPMessage>;

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct NamedSMTPConfiguration {
    pub name: String,
    pub configuration: SMTPConfiguration,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct SMTPConfiguration {
    pub address: SMTPConfigurationAddress,
    pub auth: SMTPConfigurationAuth,
    pub require_ssl: bool,
    pub verify_certificates: bool,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct SMTPConfigurationAddress {
    pub address: String,
    pub port: u16,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Clone)]
pub struct SMTPConfigurationAuth {
    pub use_auth: bool,
    pub user: String,
    pub password: String,
}

impl Debug for SMTPConfigurationAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SMTPConfigurationAuth")
            .field("use_auth", &self.use_auth)
            .field("user", &self.user)
            .field("password", &"***OMITTED***")
            .finish()
    }
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct NamedSMTPMessage {
    pub name: String,
    pub message: SMTPMessage,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct SMTPMessage {
    pub to: SMTPMessageAddress,
    pub from: SMTPMessageAddress,
    pub reply_to: SMTPMessageAddress,
    pub cc: SMTPMessageAddress,
    pub bcc: SMTPMessageAddress,
    pub headers: Vec<SMTPMessageHeader>,
    pub subject: String,
    pub body: SMTPMessageBody,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Clone)]
pub struct SMTPMessageAddress {
    pub name: Option<String>,
    pub email: String,
}

impl Debug for SMTPMessageAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_ref().map(mask_if_email);

        f.debug_struct("SMTPMessageAddress")
            .field("name", &name)
            .field("email", &mask_if_email(&self.email))
            .finish()
    }
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct SMTPMessageHeader {
    pub name: String,
    pub value: String,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct SMTPMessageBody {
    pub html: String,
    pub text: String,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct KeyValue<T> {
    pub name: String,
    pub value: T,
}

#[typeshare]
#[derive(Deserialize, Serialize, Encode, Decode, Debug, Clone)]
pub struct Secret<T> {
    pub name: String,
    pub value: T,
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
    pub smtp_configurations: bool,
    pub smtp_messages: bool,
    pub settings: bool,
}

impl Debug for ImportExportSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportExportSettings")
            .field("password", &"***OMITTED***")
            .field("smtp_configurations", &self.smtp_configurations)
            .field("smtp_messages", &self.smtp_messages)
            .field("settings", &self.settings)
            .finish()
    }
}
