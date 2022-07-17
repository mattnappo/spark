pub mod types;

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

pub fn a2_hash(data: Vec<u8>, salt: [u8; SALT_LEN]) -> Vec<u8> {
    let ctx =
        Argon2::from(Params::new(4096u32, 3u32, 1u32, Some(HASH_LEN)).unwrap());
    // (&data[..]).asd(); // &[u8]
    match ctx
        .hash_password(&data[..], str::from_utf8(&salt[..]).unwrap())
        .unwrap()
        .hash
    {
        Some(h) => h.as_bytes().to_vec(),
        None => Vec::new(),
    }
}

/// Derive a key from a passphrase supplied by stdin
// pub fn derive_key(salt: [u8; SALT_LEN]) -> (Aes256Gcm, Nonce<u8>) {
// TODO Fix this return type to return `Nonce`
pub fn derive_key(salt: [u8; SALT_LEN]) -> Aes256Gcm {
    // Read user passphrase from stdin and expand
    let phrase = {
        print!("Set passphrase: ");
        io::stdout().flush().unwrap();
        let phrase1 = rpassword::read_password().unwrap();

        print!("Confirm passphrase: ");
        io::stdout().flush().unwrap();
        let phrase2 = rpassword::read_password().unwrap();

        if phrase1 != phrase2 {
            // .unwrap()
            panic!("passphrases do not match")
        }
        phrase1
    };
    let expanded = a2_hash(Vec::from(phrase.as_bytes()), salt);

    // The nonce (96 bits) is the a2 hash of the salt (16 byte)
    let key = Key::from_slice(&expanded[..]);
    let cipher = Aes256Gcm::new(key);
    cipher
}
