use crate::dialogs::simple_error_dialog;
use crate::response::{
    error, success_empty, AnyResult, SMTPConfiguration, SMTPMessage, TauriResponse,
};
use crate::state::AppHandle;
use mail_send::mail_builder::headers::address::Address;
use mail_send::mail_builder::headers::HeaderType;
use mail_send::mail_builder::MessageBuilder;
use mail_send::{Error, SmtpClientBuilder};
use rust_utils::log::Log;

#[tauri::command]
pub async fn send_mail_command(
    app_handle: AppHandle,
    configuration: SMTPConfiguration,
    message: SMTPMessage,
    count: usize,
) -> TauriResponse<()> {
    log::trace!("send_mail_command");
    log::debug!("configuration: {:?}", configuration);
    log::debug!("message: {:?}", message);

    if count < 1 {
        return error(Some("Count must be greater than 0".to_string()), None);
    }

    match send_mail(configuration, message, count).await {
        Ok(_) => success_empty(),
        Err(err) => {
            log::error!("err: {:?}", err);
            simple_error_dialog(&app_handle, &err);
            error(Some(format!("{:?}", err)), None)
        }
    }
}

pub async fn send_mail(
    configuration: SMTPConfiguration,
    message: SMTPMessage,
    count: usize,
) -> AnyResult<()> {
    log::trace!("send_mail");
    log::debug!("configuration: {:?}", configuration);
    log::debug!("message: {:?}", message);
    log::debug!("count: {:?}", count);

    let mut handlers = Vec::with_capacity(count);
    for id in 0..count {
        let id = id.to_string();

        let message = message.clone();
        let configuration = configuration.clone();
        handlers.push(tokio::spawn(async move {
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

            let mut connection = match smtp_builder.connect().await.log_error(
                "backend::commands::send_mail::send_mail",
                "Error connecting to SMTP server",
            ) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            let mut message_builder = MessageBuilder::new()
                .to(Address::new_address(message.to.name, message.to.email))
                .from(Address::new_address(message.from.name, message.from.email))
                .subject(message.subject.replace("%id%", &id))
                .html_body(message.body.html.replace("%id%", &id))
                .text_body(message.body.text.replace("%id%", &id));

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

            let mut max_retries = 5;
            let mut last_res = Ok(());
            while max_retries > 0 {
                match connection.send(message_builder.clone()).await.log_error(
                    "backend::commands::send_mail::send_mail",
                    "Error sending mail",
                ) {
                    Ok(v) => {
                        last_res = Ok(());
                        break;
                    }
                    Err(e) => match e {
                        Error::UnexpectedReply(err_inner) => {
                            if err_inner.code < 500 {
                                max_retries -= 1;
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                                last_res = Err(Error::UnexpectedReply(err_inner));
                                continue;
                            }

                            last_res = Err(Error::UnexpectedReply(err_inner));
                            break;
                        }
                        _ => {
                            last_res = Err(e);
                            break;
                        }
                    },
                }
            }

            last_res
        }));
    }

    for handle in handlers {
        let _ = handle.await;
    }

    // TODO: Implement sending emails with attachments
    // TODO: Implement sending emails with inline images

    // TODO: Implement SMIME signing

    Ok(())
}
