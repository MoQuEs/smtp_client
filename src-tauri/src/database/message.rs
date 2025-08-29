use crate::database::{Database, DatabaseTrait, Section};
use crate::response::{AnyResult, NamedMessage, NamedMessages};

pub trait MessageDatabase {
    fn save_message(&self, message: &NamedMessage) -> AnyResult<()>;
    fn remove_message(&self, message: &NamedMessage) -> AnyResult<()>;
    fn get_messages(&self) -> AnyResult<NamedMessages>;
}

impl MessageDatabase for Database {
    fn save_message(&self, message: &NamedMessage) -> AnyResult<()> {
        log::trace!("save_message");
        log::debug!("message: {:?}", message);

        self.insert(Section::Message, message.name.as_str(), message)
    }

    fn remove_message(&self, message: &NamedMessage) -> AnyResult<()> {
        log::trace!("remove_message");
        log::debug!("message: {:?}", message);

        self.remove(Section::Message, message.name.as_str())
    }

    fn get_messages(&self) -> AnyResult<NamedMessages> {
        log::trace!("get_messages");

        self.get_all(Section::Message)
    }
}
