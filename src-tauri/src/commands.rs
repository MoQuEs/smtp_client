#![allow(dead_code, unused)]

use crate::AppState;
use lettre::message::header::ContentType;
use lettre::message::{IntoBody, MaybeString};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParametersBuilder};
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use tauri::State;
use typeshare::typeshare;

type AnyResult<T> = anyhow::Result<T>;

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

fn success<T>(message: Option<String>, data: Option<T>) -> TauriResponse<T> {
    TauriResponse::new(true, message, data)
}

fn success_empty() -> TauriResponse<()> {
    TauriResponse::new(true, None, None)
}

fn error<T>(message: Option<String>, data: Option<T>) -> TauriResponse<T> {
    TauriResponse::new(false, message, data)
}

fn error_empty(success: bool) -> TauriResponse<()> {
    TauriResponse::new(false, None, None)
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug)]
pub struct ServerRequest {
    address: String,
    port: i32,
    use_auth: bool,
    auth_user: String,
    auth_password: String,
    use_ssl: bool,
    ssl_verify: bool,
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug)]
pub struct MessageRequest {
    to_name: String,
    to_email: String,
    from_name: String,
    from_email: String,
    replay_to_name: String,
    replay_to_email: String,
    headers: Vec<MessageHeaderRequest>,
    subject: String,
    body: String,
}

#[typeshare]
#[derive(Deserialize, Serialize, Debug)]
pub struct MessageHeaderRequest {
    header: String,
    value: String,
}

#[tauri::command]
pub async fn save_server(state: State<'_, Arc<AppState>>) -> TauriResponse<()> {
    success_empty()
}

#[tauri::command]
pub async fn get_server(state: State<'_, Arc<AppState>>) -> TauriResponse<()> {
    success_empty()
}

#[tauri::command]
pub async fn save_message(state: State<'_, Arc<AppState>>, name: String) -> TauriResponse<()> {
    success_empty()
}

#[tauri::command]
pub async fn get_message(state: State<'_, Arc<AppState>>, name: String) -> TauriResponse<()> {
    success_empty()
}

#[tauri::command]
pub fn send_mail(server: ServerRequest, message: MessageRequest) -> TauriResponse<()> {
    match send_mail_command(server, message) {
        Ok(data) => success(None, None),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

pub fn send_mail_command(server: ServerRequest, message: MessageRequest) -> AnyResult<Response> {
    let mut mail_builder = Message::builder()
        .to(format!("{} <{}>", message.to_name, message.to_email).parse()?)
        .from(format!("{} <{}>", message.from_name, message.from_email).parse()?)
        .subject(message.subject)
        .header(ContentType::TEXT_HTML);

    if !message.replay_to_email.is_empty() {
        mail_builder = mail_builder
            .reply_to(format!("{} <{}>", message.replay_to_name, message.replay_to_email).parse()?);
    }

    let mut mailer_builder =
        SmtpTransport::builder_dangerous(server.address.as_str()).port(server.port as u16);

    if server.use_ssl {
        let tls_builder = TlsParametersBuilder::new(server.address.as_str().into())
            .dangerous_accept_invalid_certs(server.ssl_verify)
            .dangerous_accept_invalid_hostnames(server.ssl_verify);

        let tls = tls_builder.build()?;
        mailer_builder = mailer_builder.tls(Tls::Required(tls));
    }

    if server.use_auth {
        mailer_builder =
            mailer_builder.credentials(Credentials::new(server.auth_user, server.auth_password));
    }

    let mail = mail_builder.body(MaybeString::String(message.body))?;
    let mailer = mailer_builder.build();

    Ok(mailer.send(&mail)?)
}
