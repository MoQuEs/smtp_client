use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use typeshare::typeshare;

pub type AnyResult<T> = anyhow::Result<T>;

#[typeshare]
#[derive(Deserialize, Serialize, Debug)]
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
pub type MaybeSMTPMessage = Option<NamedSMTPMessage>;
pub type SMTPConfigurations = Vec<NamedSMTPConfiguration>;
pub type SMTPMessages = Vec<NamedSMTPMessage>;

#[typeshare]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NamedSMTPConfiguration {
    pub name: String,
    pub configuration: SMTPConfiguration,
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SMTPConfiguration {
    pub address: SMTPConfigurationAddress,
    pub auth: SMTPConfigurationAuth,
    pub require_ssl: bool,
    pub verify_certificates: bool,
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SMTPConfigurationAddress {
    pub address: String,
    pub port: u16,
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SMTPConfigurationAuth {
    pub use_auth: bool,
    pub user: String,
    pub password: String,
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NamedSMTPMessage {
    pub name: String,
    pub message: SMTPMessage,
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug, Clone)]
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SMTPMessageAddress {
    pub name: Option<String>,
    pub email: String,
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SMTPMessageHeader {
    pub name: String,
    pub value: String,
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SMTPMessageBody {
    pub html: String,
    pub text: String,
}
