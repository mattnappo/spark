pub mod types;

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use std::str;

pub const SALT_LEN: usize = 16;
pub const KEY_SIZE: usize = 2048;

pub fn a2_hash(data: Vec<u8>, salt: [u8; SALT_LEN]) -> Vec<u8> {
    let ctx = Argon2::default();
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
