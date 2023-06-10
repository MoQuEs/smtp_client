use crate::database::{Database, Section};
use crate::response::{AnyResult, MaybeSecret, Secret};
use serde::de::DeserializeOwned;
use serde::Serialize;

impl Database {
    pub fn get_secret<T: DeserializeOwned>(
        &self,
        secret: impl AsRef<str>,
    ) -> AnyResult<MaybeSecret<T>> {
        self.get(Section::Secrets, secret)
    }

    pub fn save_secret<T: DeserializeOwned + Serialize>(
        &self,
        secret: &Secret<T>,
    ) -> AnyResult<Option<()>> {
        self.insert(Section::Secrets, secret.name.as_str(), secret)
    }

    pub fn remove_secret<T>(&self, secret: &Secret<T>) -> AnyResult<Option<()>> {
        self.remove(Section::Secrets, secret.name.as_str())
    }
}
