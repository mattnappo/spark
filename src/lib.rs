pub mod crypto;
pub mod primitives;

use std::error::Error as StdError;
use std::{fmt, io};

pub const DATA_DIR: &'static str = "./data/";

/// Custom error type for this crate
#[derive(Debug)]
pub enum Error {
    Bincode(bincode::Error),
    Io(io::Error),
    Aes(aes_gcm::Error),
    General(GeneralError),
    Fail(String),
}

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

// TODO: Write a derive macro for these

impl From<GeneralError> for Error {
    fn from(err: GeneralError) -> Error {
        Error::General(err)
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
