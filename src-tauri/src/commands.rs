use crate::database::{get_configurations, get_messages, save_configuration, save_message};
use crate::response::{
    error, success, AnyResult, MaybeSMTPConfiguration, MaybeSMTPMessage, NamedSMTPConfiguration,
    NamedSMTPMessage, SMTPConfiguration, SMTPConfigurations, SMTPMessage, SMTPMessages,
    TauriResponse,
};
use mail_send::mail_builder::headers::address::Address;
use mail_send::mail_builder::headers::HeaderType;
use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;

#[tauri::command]
pub fn save_configuration_command(
    configuration: NamedSMTPConfiguration,
) -> TauriResponse<MaybeSMTPConfiguration> {
    match save_configuration(&configuration) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn get_configurations_command() -> TauriResponse<SMTPConfigurations> {
    match get_configurations() {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn save_message_command(message: NamedSMTPMessage) -> TauriResponse<MaybeSMTPMessage> {
    match save_message(&message) {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub fn get_messages_command() -> TauriResponse<SMTPMessages> {
    match get_messages() {
        Ok(data) => success(None, Some(data)),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

#[tauri::command]
pub async fn send_mail_command(
    server: SMTPConfiguration,
    message: SMTPMessage,
) -> TauriResponse<()> {
    match send_mail(server, message).await {
        Ok(_) => success(None, None),
        Err(err) => {
            println!("{:?}", err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

pub async fn send_mail(server: SMTPConfiguration, message: SMTPMessage) -> AnyResult<()> {
    let mut message_builder = MessageBuilder::new()
        .to(Address::new_address(message.to.name, message.to.email))
        .from(Address::new_address(message.from.name, message.from.email))
        .subject(message.subject)
        .html_body(message.body.html)
        .text_body(message.body.text);

    if !message.reply_to.email.is_empty() {
        message_builder = message_builder.reply_to(Address::new_address(
            message.reply_to.name,
            message.reply_to.email,
        ));
    }

    if !message.cc.email.is_empty() {
        message_builder =
            message_builder.cc(Address::new_address(message.cc.name, message.cc.email));
    }

    if !message.bcc.email.is_empty() {
        message_builder =
            message_builder.bcc(Address::new_address(message.bcc.name, message.bcc.email));
    }

    if !message.headers.is_empty() {
        for header in message.headers {
            message_builder =
                message_builder.header(header.name, HeaderType::Text(header.value.into()));
        }
    }

    let mut smtp_builder = SmtpClientBuilder::new(server.address.address, server.address.port)
        .implicit_tls(server.require_ssl);

    if server.auth.use_auth {
        smtp_builder = smtp_builder.credentials((server.auth.user, server.auth.password))
    }

    if !server.verify_certificates {
        smtp_builder = smtp_builder.allow_invalid_certs();
    }

    smtp_builder.connect().await?.send(message_builder).await?;

    Ok(())
}
