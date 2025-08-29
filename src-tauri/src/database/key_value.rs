use crate::database::{Database, DatabaseTrait, Section};
use crate::response::AnyResult;
use crate::serialize::{Decode, Encode};
use std::fmt::Debug;

pub trait KeyValueDatabase {
    fn get_value<T: Decode<()> + Debug>(&self, key: impl AsRef<str>) -> AnyResult<Option<T>>;
    fn set_value<T: Encode + Debug>(&self, key: impl AsRef<str>, value: &T) -> AnyResult<()>;
    fn remove_value(&self, key: impl AsRef<str>) -> AnyResult<()>;
}

impl KeyValueDatabase for Database {
    fn get_value<T: Decode<()> + Debug>(&self, key: impl AsRef<str>) -> AnyResult<Option<T>> {
        log::trace!("get_value");
        log::debug!("key: {}", key.as_ref());

        self.get(Section::KeyValue, key)
    }

    fn set_value<T: Encode + Debug>(&self, key: impl AsRef<str>, value: &T) -> AnyResult<()> {
        log::trace!("set_value");
        log::debug!("key: {}", key.as_ref());
        log::debug!("value: {:?}", value);

        self.insert(Section::KeyValue, key, value)
    }

    fn remove_value(&self, key: impl AsRef<str>) -> AnyResult<()> {
        log::trace!("remove_value");
        log::debug!("key: {}", key.as_ref());

        self.remove(Section::KeyValue, key)
    }
}
