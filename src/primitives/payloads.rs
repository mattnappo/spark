use crate::crypto::SALT_LEN;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

pub trait Payload<'de>: Serialize + Deserialize<'de> {}

// TODO: add checksum and digital signature functions here
impl<'de> Payload<'de> for GenericPayload {}
impl<'de> Payload<'de> for CredentialsPayload {}
impl<'de> Payload<'de> for KeypairPayload {}
