use crate::dialogs::simple_error_dialog;
use crate::response::{
    error, success_empty, AnyResult, SMTPConfiguration, SMTPMessage, TauriResponse,
};
use crate::state::AppHandle;
use mail_send::mail_builder::headers::address::Address;
use mail_send::mail_builder::headers::HeaderType;
use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use rust_utils::log::Log;

#[tauri::command]
pub async fn send_mail_command(
    app_handle: AppHandle,
    configuration: SMTPConfiguration,
    message: SMTPMessage,
) -> TauriResponse<()> {
    log::trace!("send_mail_command");
    log::debug!("configuration: {:?}", configuration);
    log::debug!("message: {:?}", message);

    match send_mail(configuration, message).await {
        Ok(_) => success_empty(),
        Err(err) => {
            log::error!("err: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

pub async fn send_mail(configuration: SMTPConfiguration, message: SMTPMessage) -> AnyResult<()> {
    log::trace!("send_mail");
    log::debug!("configuration: {:?}", configuration);
    log::debug!("message: {:?}", message);

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

    let mut smtp_builder =
        SmtpClientBuilder::new(configuration.address.address, configuration.address.port)
            .implicit_tls(configuration.require_ssl);

    if configuration.auth.use_auth {
        smtp_builder =
            smtp_builder.credentials((configuration.auth.user, configuration.auth.password))
    }

    if !configuration.verify_certificates {
        smtp_builder = smtp_builder.allow_invalid_certs();
    }

    smtp_builder
        .connect()
        .await
        .log_error(
            "backend::commands::send_mail::send_mail",
            "Error connecting to SMTP server",
        )?
        .send(message_builder)
        .await
        .log_error(
            "backend::commands::send_mail::send_mail",
            "Error sending mail",
        )?;

    Ok(())
}
