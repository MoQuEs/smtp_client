use crate::database::{Database, DatabaseTrait, Section};
use crate::response::*;

pub trait AttachmentDatabase {
    fn save_attachment(&self, attachment: &NamedAttachment) -> AnyResult<()>;
    fn remove_attachment(&self, attachment: &NamedAttachment) -> AnyResult<()>;
    fn get_attachments(&self) -> AnyResult<NamedAttachments>;
}

impl AttachmentDatabase for Database {
    fn save_attachment(&self, attachment: &NamedAttachment) -> AnyResult<()> {
        log::trace!("save_attachment");
        log::debug!("attachment: {:?}", attachment);

        self.insert(Section::Attachment, attachment.name.as_str(), attachment)
    }

    fn remove_attachment(&self, attachment: &NamedAttachment) -> AnyResult<()> {
        log::trace!("remove_attachment");
        log::debug!("attachment: {:?}", attachment);

        self.remove(Section::Attachment, attachment.name.as_str())
    }

    fn get_attachments(&self) -> AnyResult<NamedAttachments> {
        log::trace!("get_attachments");

        self.get_all(Section::Attachment)
    }
}
