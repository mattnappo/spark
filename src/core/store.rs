use crate::crypto::types::EncServerKey;
use crate::primitives::secret::{EncSecret, Header, SecretID};
use crate::Error;

use std::path::Path;

/// A secret store
pub struct Store {
    /// A database mapping from `Header`s to `EncSecret`s
    store: sled::Db,
}

impl Store {
    /// Initialize a new store given path to database and key that locks this db's secrets
    pub fn load<P: AsRef<Path>>(db_path: P) -> Result<Self, Error> {
        Ok(Self {
            store: sled::open(db_path)?,
        })
    }

    // TODO make these priv
    pub(crate) fn dump(&self) {
        println!("-- dump --");
        self.store.iter().keys().for_each(|k| {
            println!("{:?}", bincode::deserialize::<Header>(&k.unwrap()))
        });
        println!("----------");
    }

    pub(crate) fn put_secret(
        &mut self,
        secret: EncSecret,
    ) -> Result<(), Error> {
        self.store
            .insert(bincode::serialize(&secret.header)?, secret.secret)
            .map(|_| ())
            .map_err(|e| Error::Sled(e))
    }

    pub(crate) fn get_secret(
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
    /// Will panic if internal sled db is corrupted
    pub(crate) fn get_secret_from_id(
        &self,
        secret_id: SecretID,
    ) -> Result<Option<EncSecret>, Error> {
        self.store
            .iter()
            .find(|d| {
                // idk why this is a result
                bincode::deserialize::<Header>(&d.as_ref().unwrap().0)
                    .unwrap()
                    .id
                    == secret_id
            })
            .map(|d| {
                d.map(|(k, v)| EncSecret {
                    header: bincode::deserialize::<Header>(&k).unwrap(),
                    secret: v.to_vec(),
                })
            })
            .transpose()
            .map_err(|e| Error::Sled(e))
    }

    pub(crate) fn get_secrets_from_label(
        &self,
        label: &str,
    ) -> Result<Vec<EncSecret>, Error> {
        todo!()
    }

    /// Get all the secrets headers in the db
    pub(crate) fn list_secrets(&self) -> Result<Vec<Header>, Error> {
        Ok(self
            .store
            .iter()
            .keys()
            .map(|raw_h| {
                bincode::deserialize::<Header>(&raw_h.unwrap()).unwrap()
            })
            .collect::<Vec<Header>>())
    }
}
