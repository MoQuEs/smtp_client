use crate::response::AnyResult;
use bincode::config::standard;
use bincode::{decode_from_slice, encode_to_vec};
pub use bincode::{Decode, Encode};
use std::fmt::Debug;

pub fn encode<T: Encode + Debug>(data: &T) -> AnyResult<Vec<u8>> {
    log::trace!("serialize");
    log::debug!("data: ***OMITTED***");
    log::debug!("data: {:?}", data);

    let s =
        encode_to_vec(data, standard()).inspect_err(|e| log::error!("Error serialize '{:?}'"))?;

    log::debug!("s: {:?}", s);

    Ok(s)
}

pub fn decode<T: Decode<()> + Debug>(data: &[u8]) -> AnyResult<T> {
    log::trace!("deserialize");
    log::debug!("data: ***OMITTED***");
    log::debug!("data: {:?}", data);

    let d = decode_from_slice(data, standard())
        .inspect_err(|e| log::error!("Error deserialize '{:?}'"))?;

    log::debug!("s: {:?}", d);

    Ok(d.0)
}
