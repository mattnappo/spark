pub mod crypto;
pub mod primitives;

use std::io;

pub const DATA_DIR: &'static str = "./data/";

/// Custom error type for this crate
#[derive(Debug)]
pub enum Error {
    Bincode(bincode::Error),
    Io(io::Error),
    Aes(aes_gcm::Error),
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
