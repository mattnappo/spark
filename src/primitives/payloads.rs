use crate::crypto::SALT_LEN;
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    Generic(GenericPayload),
    Credentials(CredentialsPayload),
    Keypair(KeypairPayload)
}

