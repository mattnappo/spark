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
use std::io;
use std::io::Write;

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

/// An encrypted `ServerKey` containing necessary decrypting information
#[derive(Debug, Serialize, Deserialize)]
pub struct EncServerKey {
    server_key: Vec<u8>,
    nonce: [u8; NONCE_LEN],
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

        // Saltgen TODO Specify len explicitly
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

    /// Return an encrypted `ServerKey` with a user passphrase and consume the `ServerKey`
    pub fn lock(self) -> Result<EncServerKey, bincode::Error> {
        // Serialize
        let ser = bincode::serialize(&self).unwrap();

        // TODO make idiomatic
        let mut ser_nonce = [0u8; NONCE_LEN];
        for i in 0..NONCE_LEN {
            ser_nonce[i] = self.salt[i];
        }
        let nonce = Nonce::from_slice(&self.salt[0..NONCE_LEN]);

        // Encrypt
        let cipher = derive_key(self.salt);
        Ok(EncServerKey {
            server_key: cipher.encrypt(nonce, &ser[..]).unwrap(),
            nonce: ser_nonce,
            salt: self.salt,
        })
    }

    /// Unlock a `ServerKey`
    pub fn unlock(enc: EncServerKey) -> Result<Self, bincode::Error> {
        // Decrypt the bytes
        let cipher = derive_key(enc.salt);
        let nonce = Nonce::from_slice(&enc.nonce[..]);
        let decrypted = cipher.decrypt(nonce, &enc.server_key[..]).unwrap();

        // Deserialize
        bincode::deserialize::<Self>(&decrypted[..])
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
    }
}
