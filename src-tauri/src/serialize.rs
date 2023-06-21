use crate::response::AnyResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub fn serialize<T: Serialize + Debug>(data: &T) -> AnyResult<Vec<u8>> {
    log::trace!(target: "backend::serialize::serialize", "serialize");
    log::debug!(target: "backend::serialize::serialize", "data: {:?}", data);

    Ok(bincode::serialize(data)?)
}

pub fn deserialize<T: DeserializeOwned + Debug>(data: &[u8]) -> AnyResult<T> {
    log::trace!(target: "backend::serialize::deserialize", "deserialize");
    log::debug!(target: "backend::serialize::deserialize", "data: {:?}", data);

    Ok(bincode::deserialize(data)?)
}
