#![allow(dead_code)]

use crate::response::AnyResult;
use orion::aead::{open, seal, SecretKey};

fn get_key() -> AnyResult<SecretKey> {
    Ok(SecretKey::generate(32)?)
}

pub fn crypt_text(text: impl AsRef<str>) -> AnyResult<Vec<u8>> {
    Ok(seal(&get_key()?, text.as_ref().as_bytes())?)
}

pub fn decrypt_text(text: Vec<u8>) -> AnyResult<String> {
    let decrypted_text = open(&get_key()?, text.as_slice())?;
    Ok(std::str::from_utf8(decrypted_text.as_slice())?.to_string())
}
