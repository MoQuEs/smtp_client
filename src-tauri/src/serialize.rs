use crate::response::AnyResult;
use bincode::config::standard;
use bincode::{decode_from_slice, encode_to_vec};
pub use bincode::{Decode, Encode};
use rust_utils::log::Log;
use std::fmt::Debug;

pub fn encode<T: Encode + Debug>(data: &T) -> AnyResult<Vec<u8>> {
    log::trace!("serialize");
    log::debug!("data: ***OMITTED***");
    log::debug!("data: {:?}", data);

    let s = encode_to_vec(data, standard()).log_error(
        "backend::serialize::serialize::bincode::serialize",
        "Serialize",
    )?;

    log::debug!("s: {:?}", s);

    Ok(s)
}

pub fn decode<T: Decode + Debug>(data: &[u8]) -> AnyResult<T> {
    log::trace!("deserialize");
    log::debug!("data: ***OMITTED***");
    log::debug!("data: {:?}", data);

    let d = decode_from_slice(data, standard()).log_error(
        "backend::serialize::deserialize::bincode::deserialize",
        "Deserialize",
    )?;

    log::debug!("s: {:?}", d);

    Ok(d.0)
}
