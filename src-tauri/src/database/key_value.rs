use crate::database::{Database, Section};
use crate::response::AnyResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

impl Database {
    pub fn get_value<T: DeserializeOwned + Debug>(
        &self,
        key: impl AsRef<str>,
    ) -> AnyResult<Option<T>> {
        log::trace!(target: "backend::database::key_value::Database::get_value", "get_value");
        log::debug!(target: "backend::database::key_value::Database::get_value", "key: {}", key.as_ref());

        self.get(Section::KeyValue, key)
    }

    pub fn save_value<T: Serialize + Debug>(
        &self,
        key: impl AsRef<str>,
        value: &T,
    ) -> AnyResult<()> {
        log::trace!(target: "backend::database::key_value::Database::save_value", "save_value");
        log::debug!(target: "backend::database::key_value::Database::save_value", "key: {}", key.as_ref());
        log::debug!(target: "backend::database::key_value::Database::save_value", "value: {:?}", value);

        self.insert(Section::KeyValue, key, value)
    }

    pub fn remove_value(&self, key: impl AsRef<str>) -> AnyResult<()> {
        log::trace!(target: "backend::database::key_value::Database::remove_value", "remove_value");
        log::debug!(target: "backend::database::key_value::Database::remove_value", "key: {}", key.as_ref());

        self.remove(Section::KeyValue, key)
    }
}
