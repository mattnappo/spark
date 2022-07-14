use super::payloads::*;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use std::default::Default;
use std::net::Ipv4Addr;
use std::string::ToString;
use std::time::{SystemTime, UNIX_EPOCH};

/// The scope of which systems are allowed to access a secret
#[derive(Debug)]
enum Scope {
    /// Any client
    Public,

    /// Only systems on the local network
    Local,

    /// A custom set of IPs
    Custom(Vec<Ipv4Addr>),
}

/// A unique secret ID
#[derive(Default, Debug)]
struct SecretID(Vec<u8>);

impl SecretID {
    fn from(label: &str, desc: &str, creation: u128) -> Self {
        let data = format!("{}{}{}", &label, &desc, creation);
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        Self(
            match argon2.hash_password(data.as_bytes(), &salt).unwrap().hash {
                Some(h) => h.as_bytes().to_vec(),
                None => panic!("null hashing error"),
            },
        )
    }
}

impl ToString for SecretID {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

/// Secret metadata information contained in every secret
#[derive(Debug)]
struct Header {
    /// The unique identifier
    id: SecretID,

    /// A label
    label: String,

    /// A description of the secret
    desc: String,

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
    fn new(label: &str, desc: &str, expiration: u64, scope: Scope) -> Self {
        let creation = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self {
            id: SecretID::from(label, desc, creation),
            label: label.to_owned(),
            desc: desc.to_owned(),
            creation,
            expiration,
            scope,
        }
    }
}

/// The core secret type
#[derive(Debug)]
enum Secret {
    // General payload types
    APIKey(Header, GenericPayload<String>),
    PublicKey(Header, GenericPayload<Vec<u8>>),
    PrivateKey(Header, GenericPayload<Vec<u8>>),
    Other(Header, GenericPayload<Vec<u8>>),

    // Specific payload types
    Keypair(Header, KeypairPayload),
    Credentials(Header, CredentialsPayload),
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
        let id = SecretID::from("label", "desc", 100);
        println!("id: {:?}", id);
        println!("string id: {}", id.to_string());
    }
}
