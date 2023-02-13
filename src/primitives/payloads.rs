use crate::crypto::SALT_LEN;
use serde::{Deserialize, Serialize};

// TODO: make a trait for all of these as common behavior
// Use derive-new crate

#[derive(Debug, Serialize, Deserialize)]
pub enum KeypairType {
    Ed25519,
    Aes,
    Rsa,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericPayload {
    secret: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialsPayload {
    service: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeypairPayload {
    public: Vec<u8>,
    private: Vec<u8>,
    key_type: KeypairType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    Generic(GenericPayload),
    Credentials(CredentialsPayload),
    Keypair(KeypairPayload),
}

impl GenericPayload {
    pub fn new(secret: Vec<u8>) -> Self {
        Self { secret }
    }
}

impl CredentialsPayload {
    pub fn new(service: &str, username: &str, password: &str) -> Self {
        Self {
            service: service.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl KeypairPayload {
    pub fn new(
        public: Vec<u8>,
        private: Vec<u8>,
        key_type: KeypairType,
    ) -> Self {
        Self {
            public,
            private,
            key_type,
        }
    }
}
