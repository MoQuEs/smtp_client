#![allow(dead_code)]

use crate::response::AnyResult;

pub mod encrypt {
    use super::*;
    use age::scrypt::{Identity, Recipient};
    use age::secrecy::SecretString;
    use age::{decrypt, encrypt};

    pub fn generate_crypt_key_from_string(
        password: impl AsRef<str>,
    ) -> AnyResult<(Recipient, Identity)> {
        log::trace!("generate_crypt_key_from_string");

        let passphrase = SecretString::from(password.as_ref().to_string());

        Ok((
            Recipient::new(passphrase.clone()),
            Identity::new(passphrase),
        ))
    }

    pub fn encrypt_text(key: &Recipient, text: impl AsRef<str>) -> AnyResult<Vec<u8>> {
        log::trace!("encrypt_text");

        encrypt_data(key, text.as_ref().as_bytes())
    }

    pub fn encrypt_data(key: &Recipient, data: &[u8]) -> AnyResult<Vec<u8>> {
        log::trace!("encrypt_data");

        Ok(encrypt(key, data)?)
    }

    pub fn decrypt_text(key: &Identity, text: Vec<u8>) -> AnyResult<String> {
        log::trace!("decrypt_text");

        let decrypted_text = decrypt_data(key, text.as_slice())?;
        Ok(std::str::from_utf8(decrypted_text.as_slice())?.to_string())
    }

    pub fn decrypt_data(key: &Identity, data: &[u8]) -> AnyResult<Vec<u8>> {
        log::trace!("decrypt_data");

        Ok(decrypt(key, data)?)
    }
}

pub mod password {
    use super::*;
    use orion::{
        pwhash,
        pwhash::{Password, PasswordHash},
    };

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
