use crate::database::{Database, Section};
use crate::response::AnyResult;
use crate::serialize::{Decode, Encode};
use std::fmt::Debug;

impl Database {
    pub fn get_value<T: Decode + Debug>(&self, key: impl AsRef<str>) -> AnyResult<Option<T>> {
        log::trace!("get_value");
        log::debug!("key: {}", key.as_ref());

        self.get(Section::KeyValue, key)
    }

    pub fn set_value<T: Encode + Debug>(&self, key: impl AsRef<str>, value: &T) -> AnyResult<()> {
        log::trace!("set_value");
        log::debug!("key: {}", key.as_ref());
        log::debug!("value: {:?}", value);

        self.insert(Section::KeyValue, key, value)
    }

    pub fn remove_value(&self, key: impl AsRef<str>) -> AnyResult<()> {
        log::trace!("remove_value");
        log::debug!("key: {}", key.as_ref());

        self.remove(Section::KeyValue, key)
    }
}
