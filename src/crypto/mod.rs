pub mod types;

use super::primitives::payloads::Payload;
use super::primitives::secret::{EncSecret, Secret};
use crate::{Error, GeneralError};
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2, Params,
};
use std::io;
use std::io::Write;
use std::str;

pub const SALT_LEN: usize = 16;
pub const HASH_LEN: usize = 32;
pub const NONCE_LEN: usize = 12;
pub const KEY_SIZE: usize = 2048;

/// A type with the ability to encrypt and decrypt secrets. Functions
/// in this trait are to be run client-side.
pub trait Encryptor<'d, T: Payload<'d>> {
    fn encrypt(&self, sec: Secret<'d, T>) -> Result<EncSecret, Error>;
    fn decrypt(&self, sec: EncSecret) -> Result<Secret<'d, T>, Error>;
}

/// Hash input data with a given salt using Argon2
pub fn a2_hash(data: Vec<u8>, salt: [u8; SALT_LEN]) -> Result<Vec<u8>, Error> {
    let ctx = Argon2::from(Params::new(4096u32, 3u32, 1u32, Some(HASH_LEN))?);
    match ctx
        .hash_password(&data[..], str::from_utf8(&salt[..])?)?
        .hash
    {
        Some(h) => Ok(h.as_bytes().to_vec()),
        None => Err(Error::Fail("error hashing with argon2".to_string())),
    }
}

/// Read a passphrase from the user
fn read_passphrase(confirm: bool) -> Result<String, Error> {
    print!("Enter passphrase: ");
    io::stdout().flush()?;
    let phrase1 = rpassword::read_password()?;
    let mut phrase2 = phrase1.clone();
    if confirm {
        print!("Confirm passphrase: ");
        io::stdout().flush()?;
        phrase2 = rpassword::read_password()?;
    }

    if phrase1 != phrase2 {
        Err(Error::Fail("Passphrases do not match".to_string()))
    } else {
        Ok(phrase1)
    }
}

/// Derive a key from a passphrase supplied by stdin
// pub fn derive_key(salt: [u8; SALT_LEN]) -> (Aes256Gcm, Nonce<u8>) {
// TODO Fix this return type to return `Nonce`
pub fn derive_key(
    salt: [u8; SALT_LEN],
    confirm: bool,
) -> Result<Aes256Gcm, Error> {
    // Read user passphrase from stdin and expand
    let phrase = read_passphrase(confirm)?;
    let expanded = a2_hash(Vec::from(phrase.as_bytes()), salt)?;

    // The nonce (96 bits) is the a2 hash of the salt (16 byte)
    let key = Key::from_slice(&expanded[..]);
    let cipher = Aes256Gcm::new(key);
    Ok(cipher)
}
