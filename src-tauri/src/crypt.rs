#![allow(dead_code)]

use crate::response::AnyResult;

pub mod encrypt {
    use super::*;
    use orion::aead::{open, seal};

    pub use orion::aead::SecretKey;

    pub fn generate_crypt_key() -> AnyResult<SecretKey> {
        log::trace!("generate_crypt_key");
        Ok(SecretKey::generate(32)?)
    }

    pub fn generate_crypt_key_from_string(password: impl AsRef<str>) -> AnyResult<SecretKey> {
        log::trace!("generate_crypt_key_from_string");
        log::debug!("password: ***OMITTED***");

        Ok(SecretKey::from_slice(password.as_ref().as_bytes())?)
    }

    pub fn encrypt_text(key: &SecretKey, text: impl AsRef<str>) -> AnyResult<Vec<u8>> {
        log::trace!("encrypt_text");
        log::debug!("key: {:?}", key);
        log::debug!("text: ***OMITTED***");

        Ok(encrypt_data(key, text.as_ref().as_bytes())?)
    }

    pub fn encrypt_data(key: &SecretKey, data: &[u8]) -> AnyResult<Vec<u8>> {
        log::trace!("encrypt_data");
        log::debug!("key: {:?}", key);
        log::debug!("data: ***OMITTED***");

        Ok(seal(key, data)?)
    }

    pub fn decrypt_text(key: &SecretKey, text: Vec<u8>) -> AnyResult<String> {
        log::trace!("decrypt_text");
        log::debug!("key: {:?}", key);
        log::debug!("text: ***OMITTED***");

        let decrypted_text = decrypt_data(key, text.as_slice())?;
        Ok(std::str::from_utf8(decrypted_text.as_slice())?.to_string())
    }

    pub fn decrypt_data(key: &SecretKey, data: &[u8]) -> AnyResult<Vec<u8>> {
        log::trace!("decrypt_data");
        log::debug!("key: {:?}", key);
        log::debug!("data: ***OMITTED***");

        Ok(open(key, data)?)
    }
}

pub mod password {
    use super::*;
    use orion::{pwhash, pwhash::Password};

    pub use orion::pwhash::PasswordHash;

    fn to_password(password: impl AsRef<str>) -> AnyResult<Password> {
        log::trace!("to_password");
        log::debug!("password: ***OMITTED***");

        Ok(Password::from_slice(password.as_ref().as_bytes())?)
    }

    pub fn hash_password(password: impl AsRef<str>) -> AnyResult<PasswordHash> {
        log::trace!("hash_password");
        log::debug!("password: ***OMITTED***");

        let password = to_password(password)?;
        Ok(pwhash::hash_password(&password, 3, 1 << 16)?)
    }

    pub fn password_verify(hash: &PasswordHash, password: impl AsRef<str>) -> AnyResult<bool> {
        log::trace!("password_verify");
        log::debug!("hash: {:?}", hash);
        log::debug!("password: ***OMITTED***");

        let password = to_password(password)?;
        Ok(pwhash::hash_password_verify(hash, &password).is_ok())
    }
}

pub mod hash {
    use super::*;
    use orion::hash::{digest, digest_from_reader, Digest};

    pub fn generate(text: impl AsRef<str>) -> AnyResult<Digest> {
        log::trace!("generate");
        log::debug!("text: ***OMITTED***");

        Ok(digest(text.as_ref().as_bytes())?)
    }

    pub fn generate_from_reader(reader: impl std::io::Read) -> AnyResult<Digest> {
        log::trace!("generate_from_reader");
        log::debug!("reader: ***OMITTED***");

        Ok(digest_from_reader(reader)?)
    }
}
