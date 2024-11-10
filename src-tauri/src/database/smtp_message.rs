use crate::database::{Database, Section};
use crate::response::{AnyResult, NamedSMTPMessage, SMTPMessages};

impl Database {
    pub fn save_message(&self, message: &NamedSMTPMessage) -> AnyResult<()> {
        log::trace!("save_message");
        log::debug!("message: {:?}", message);

        self.insert(Section::SMTPMessage, message.name.as_str(), message)
    }

    pub fn remove_message(&self, message: &NamedSMTPMessage) -> AnyResult<()> {
        log::trace!("remove_message");
        log::debug!("message: {:?}", message);

        self.remove(Section::SMTPMessage, message.name.as_str())
    }

    pub fn get_messages(&self) -> AnyResult<SMTPMessages> {
        log::trace!("get_messages");

        self.get_all(Section::SMTPMessage)
    }
}
