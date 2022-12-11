use super::payloads::*;
use crate::crypto::{NONCE_LEN, SALT_LEN};
use crate::Error;
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::net::Ipv4Addr;
use std::string::ToString;
use std::time::{SystemTime, UNIX_EPOCH};

/// The scope of which systems are allowed to access a secret
#[derive(Debug, Serialize, Deserialize)]
pub enum Scope {
    /// Any client
    Public,

    /// Only systems on the local network
    Local,

    /// A custom set of IPs
    Custom(Vec<Ipv4Addr>),
}

/// A unique secret ID
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretID(Vec<u8>);

impl SecretID {
    fn from(label: &str, desc: &str, creation: u128) -> Result<Self, Error> {
        let data = format!("{}{}{}", &label, &desc, creation);
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let id = match argon2.hash_password(data.as_bytes(), &salt)?.hash {
            Some(h) => h.as_bytes().to_vec(),
            None => return Err(Error::Fail("could not hash salt".to_string())),
        };

        Ok(Self(id))
    }
}

impl ToString for SecretID {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

/// Secret metadata information contained in every secret
#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    /// The unique secret identifier
    id: SecretID,

    /// A label
    label: String,

    /// A description of the secret
    desc: String,

    /// The secret type
    tag: Tag,

    /// Epoch in ms of creation time
    creation: u128,

    /// Expiration time (0 for no expiration)
    expiration: u64,

    /// The secret's scope
    scope: Scope,
    // TODO
    // A checksum for integrity
    //sum: Vec<u8>,

    // ed25519 digital signature
    //sig: Signature,
}

impl Header {
    fn new(
        label: &str,
        desc: &str,
        tag: Tag,
        expiration: u64,
        scope: Scope,
    ) -> Result<Self, Error> {
        let creation =
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        Ok(Self {
            id: SecretID::from(label, desc, creation)?,
            label: label.to_owned(),
            desc: desc.to_owned(),
            tag,
            creation,
            expiration,
            scope,
        })
    }
}

/// The type of secret
#[derive(Debug, Serialize, Deserialize)]
pub enum Tag {
    APIKey,
    PublicKey,
    PrivateKey,
    Keypair,
    Credentials,
    Other,
}

/// The core secret type
#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    /// The secret
    secret: Payload,

    /// The secret metadata (should not be encrypted)
    #[serde(skip_serializing)]
    pub header: Header,
}

/// An encrypted secret, which is what is written to fs
#[derive(Debug, Serialize, Deserialize)]
pub struct EncSecret {
    /// The serialized, encrypted secret
    pub secret: Vec<u8>,

    /// The secret header (plaintext)
    pub header: Header,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let h = Header::new("my key", "my key desc", 0, Scope::Local);
        println!("{:?}", h);
    }

    #[test]
    fn test_secretid() {
        let id = SecretID::from("label", "desc", 100).unwrap();
        println!("id: {:?}", id);
        println!("string id: {}", id.to_string());
    }
}
