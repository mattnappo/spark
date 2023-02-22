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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Scope {
    /// Any client
    Public,

    /// Only systems on the local network
    Local,

    /// A custom set of IPs
    Custom(Vec<Ipv4Addr>),
}

/// A unique secret ID
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecretID(Vec<u8>);

impl SecretID {
    pub fn from(
        label: &str,
        desc: Option<&str>,
        creation: u128,
    ) -> Result<Self, Error> {
        let data = format!("{}{:?}{}", &label, &desc, creation);
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let id = match argon2.hash_password(data.as_bytes(), &salt)?.hash {
            Some(h) => h.as_bytes().to_vec(),
            None => return Err(Error::Fail("could not hash salt".to_string())),
        };

        Ok(Self(id))
    }

    pub fn from_vec(id: &[u8]) -> Result<Self, Error> {
        if id.len() == 32 {
            Ok(Self(id.to_vec()))
        } else {
            Err(Error::General(crate::GeneralError::new(format!(
                "cannot construct SecretID from vec {id:?} of length {}",
                id.len()
            ))))
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl ToString for SecretID {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

/// Secret metadata information contained in every secret
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Header {
    /// The unique secret identifier
    pub id: SecretID,

    /// A label
    pub label: String,

    /// A description of the secret
    pub desc: Option<String>,

    /// The secret type
    pub tag: Option<Tag>,

    /// Epoch in ms of creation time
    pub creation: u128,

    /// Expiration time (0 for no expiration)
    pub expiration: u64,

    /// The secret's scope
    pub scope: Scope,
    // TODO
    // A checksum for integrity
    //sum: Vec<u8>,

    // ed25519 digital signature
    //sig: Signature,
}

impl Header {
    pub fn new(
        label: &str,
        desc: Option<&str>,
        tag: Option<Tag>,
        expiration: u64,
        scope: Scope,
    ) -> Result<Self, Error> {
        let creation =
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        Ok(Self {
            id: SecretID::from(label, desc, creation)?,
            label: label.to_owned(),
            desc: desc.map(|d| d.to_owned()),
            tag,
            creation,
            expiration,
            scope,
        })
    }
}

/// The type of secret
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Tag {
    APIKey,
    PublicKey,
    PrivateKey,
    Keypair,
    Credentials,
    Other,
}

impl Default for Tag {
    fn default() -> Tag {
        Tag::Other
    }
}

/// The core secret type
#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    /// The secret
    pub secret: Payload,

    /// The secret metadata (should not be encrypted)
    #[serde(skip_serializing)]
    pub header: Header,
}

/// An encrypted secret, which is what is written to fs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
