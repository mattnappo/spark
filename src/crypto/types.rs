use crate::crypto::*;
use crate::{Error, GeneralError, DATA_DIR};
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
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

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
struct EncServerKey {
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
    fn lock(self) -> Result<EncServerKey, bincode::Error> {
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
    fn unlock(enc: EncServerKey) -> Result<Self, Error> {
        // Decrypt the bytes
        let cipher = derive_key(enc.salt);
        let nonce = Nonce::from_slice(&enc.nonce[..]);
        let decrypted = cipher.decrypt(nonce, &enc.server_key[..]).unwrap();

        // Deserialize
        Ok(bincode::deserialize::<Self>(&decrypted[..])?)
    }

    /// Lock and write this key to the disk
    // TODO Make return path
    pub fn write_key(self) -> Result<String, Error> {
        // Lock and serialize
        let salt: [u8; SALT_LEN] = self.salt;
        let locked = self.lock().unwrap();
        let ser = bincode::serialize(&locked)?;

        // Set destination
        let filename = format!("{}{}", &hex::encode(salt)[0..12], ".esk");
        fs::create_dir_all(DATA_DIR);
        let path = Path::new(DATA_DIR).join(filename);
        let mut file = File::create(&path)?;
        file.write_all(&ser[..])?;

        // Return filename
        match path.to_str() {
            Some(p) => Ok(String::from(p)),
            None => Err(Error::General(GeneralError::new(
                "cannot construct path".to_string(),
            ))),
        }
    }

    /// Read and decrypt a key from disk
    pub fn read_key(path: &Path) -> Result<Self, Error> {
        // Read file, deserialize, decrypt
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let key = bincode::deserialize::<EncServerKey>(&buf)?;

        // Decrypt
        Self::unlock(key)
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
