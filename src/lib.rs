pub mod core;
pub mod crypto;
pub mod primitives;
pub mod protocol_capnp;

use argon2::password_hash::errors::Error as HashError;
use rsa::errors::Error as RsaError;
use std::error::Error as StdError;
use std::str;
use std::time::SystemTimeError;
use std::{fmt, io};

pub const DATA_DIR: &'static str = "./data/";

#[derive(Debug)]
pub struct GeneralError {
    details: String,
}

impl GeneralError {
    pub fn new(details: String) -> Self {
        Self { details }
    }
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeneralError: {}", self.details)
    }
}

impl StdError for GeneralError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

/// Custom error type for this crate
#[derive(Debug)]
pub enum Error {
    Bincode(bincode::Error),
    Io(io::Error),
    Aes(aes_gcm::Error),
    Rsa(RsaError),
    General(GeneralError),
    Argon2(argon2::Error),
    Utf8(str::Utf8Error),
    Hashing(HashError),
    SystemTime(SystemTimeError),
    Sled(sled::Error),
    Fail(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self.to_string())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// TODO: Write a derive macro for these

impl From<RsaError> for Error {
    fn from(err: RsaError) -> Error {
        Error::Rsa(err)
    }
}

impl From<SystemTimeError> for Error {
    fn from(err: SystemTimeError) -> Error {
        Error::SystemTime(err)
    }
}

impl From<GeneralError> for Error {
    fn from(err: GeneralError) -> Error {
        Error::General(err)
    }
}

impl From<HashError> for Error {
    fn from(err: HashError) -> Error {
        Error::Hashing(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Error {
        Error::Argon2(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::Bincode(err)
    }
}
impl From<aes_gcm::Error> for Error {
    fn from(err: aes_gcm::Error) -> Error {
        Error::Aes(err)
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Error {
        Error::Sled(err)
    }
}
