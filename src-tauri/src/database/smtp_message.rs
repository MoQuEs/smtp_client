use crate::database::{Database, Section};
use crate::response::{AnyResult, NamedSMTPMessage, SMTPMessages};

impl Database {
    pub fn save_message(&self, message: &NamedSMTPMessage) -> AnyResult<()> {
        log::trace!(target: "backend::database::smtp_message::Database::save_message", "save_message");
        log::debug!(target: "backend::database::smtp_message::Database::save_message", "message: {:?}", message);

        self.insert(Section::SMTPMessage, message.name.as_str(), message)
    }

    pub fn remove_message(&self, message: &NamedSMTPMessage) -> AnyResult<()> {
        log::trace!(target: "backend::database::smtp_message::Database::remove_message", "remove_message");
        log::debug!(target: "backend::database::smtp_message::Database::remove_message", "message: {:?}", message);

        self.remove(Section::SMTPMessage, message.name.as_str())
    }

    pub fn get_messages(&self) -> AnyResult<SMTPMessages> {
        log::trace!(target: "backend::database::smtp_message::Database::get_messages", "get_messages");

        self.get_all(Section::SMTPMessage)
    }
}
