use crate::crypto::*;
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use serde::{
    ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer,
};
use std::time::Instant;

/// Server secret information
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerKey {
    /// The core private key used to decrypt all secrets
    privkey: RsaPrivateKey,

    /// The associated public key
    pubkey: RsaPublicKey,

    /// The salt used for all hashing. Must be unique for each key
    salt: [u8; SALT_LEN],
}

impl ServerKey {
    /// Initialize a new server key
    pub fn new() -> Self {
        // Keygen
        let mut rng = rand::thread_rng();
        let privkey = RsaPrivateKey::new(&mut rng, KEY_SIZE)
            .expect("failed to generate a key");
        let pubkey = RsaPublicKey::from(&privkey);

        // Saltgen
        let raw_salt = SaltString::generate(&mut OsRng);
        let raw_salt = raw_salt.as_bytes();
        let mut salt = [0u8; SALT_LEN];
        for i in 0..SALT_LEN {
            salt[i] = raw_salt[i];
        }

        Self {
            privkey,
            pubkey,
            salt,
        }
    }

    /// Return an encrypted `ServerKey` with a user passphrase
    pub fn lock(self) -> Result<Vec<u8>, bincode::Error> {
        // Serialize
        let ser = bincode::serialize(&self).unwrap();

        // Read user passphrase from stdin and expand
        let phrase = rpassword::read_password().unwrap();
        let expanded = a2_hash(phrase.as_bytes(), &self.salt[..]);

        // Generate cipher
        let key = Key::from_slice(expanded.as_bytes());
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&self.salt);

        // Encrypt
        Ok(cipher.encrypt(nonce, &ser[..]).unwrap())
    }
}

impl TryFrom<Vec<u8>> for ServerKey {
    type Error = bincode::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        bincode::deserialize(&bytes[..])
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_serverkey() {
        let sk = ServerKey::new();
        println!("generated server key: {:?}", sk);
    }

    #[test]
    fn test_lock() {
        let sk = ServerKey::new();
        println!("server key: {:?}", sk);

        let locked = sk.lock().unwrap();
        println!("ser len : {:?}", locked.len());
        println!("ser: {:?}", locked);
    }
}
