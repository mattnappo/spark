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

    pub fn get_secret(
        &self,
        secret_header: Header,
    ) -> Result<Option<EncSecret>, Error> {
        self.store.get(bincode::serialize(&secret_header)?).map(
            |v| match v {
                Some(secret) => Ok(Some(EncSecret {
                    header: secret_header,
                    secret: secret[..].to_vec(),
                })),
                None => Ok(None),
            },
        )?
    }

    // TODO This is O(n) right now. Need higher secondary indexing structure.
    pub fn get_secret_from_id(
        &self,
        secret_id: SecretID,
    ) -> Result<Option<EncSecret>, Error> {
        //self.store.iter().filter
        todo!()
    }

    pub fn serve() {
        println!("serving!");
    }
}
