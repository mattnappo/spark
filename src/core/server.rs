use crate::primitives::secret::*;
use crate::Error;
use std::path::Path;

/// A running server instance
pub struct Server {
    /// A database mapping from `Header`s to `EncSecret`s
    store: sled::Db,

    /// The RPC server instance
    rpc: Option<()>,
}

impl Server {
    /// Initialize a new server given path to key and database
    pub fn load(db_path: &Path) -> Result<Self, Error> {
        Ok(Self {
            store: sled::open(db_path)?,
            rpc: None,
        })
    }

    pub fn dump(&self) {
        println!("-- dump --");
        self.store.iter().keys().for_each(|k| {
            println!("{:?}", bincode::deserialize::<Header>(&k.unwrap()))
        });
        println!("----------");
    }

    pub fn put_secret(&mut self, secret: EncSecret) -> Result<(), Error> {
        self.store
            .insert(bincode::serialize(&secret.header)?, secret.secret)
            .map(|_| ())
            .map_err(|e| Error::Sled(e))
    }

    pub fn get_secret(&mut self, secret: EncSecret) {}

    pub fn serve() {
        println!("serving!");
    }
}
