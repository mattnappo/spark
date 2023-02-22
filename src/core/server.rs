use super::store::Store;
use crate::crypto::types::EncServerKey;
use crate::primitives::secret::*;
use crate::Error;
use std::path::Path;

use crate::protocol_capnp;
use crate::protocol_capnp::keyserver;
use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, TryFutureExt};
use log::{debug, info};
use std::net::ToSocketAddrs;

macro_rules! bry {
    ($st:expr) => {
        match $st {
            Ok(x) => x,
            Err(e) => return Promise::err(capnp::Error::failed(e.to_string())),
        }
    };
}

macro_rules! conv_tag {
    ($tag:expr) => {
        if let Some(t) = $tag {
            match t {
                Tag::APIKey => keyserver::Tag::Apikey,
                Tag::PublicKey => keyserver::Tag::Publickey,
                Tag::PrivateKey => keyserver::Tag::Privatekey,
                Tag::Keypair => keyserver::Tag::Keypair,
                Tag::Credentials => keyserver::Tag::Credentials,
                Tag::Other => keyserver::Tag::Other,
            }
        } else {
            keyserver::Tag::Other
        }
    };
}

/// A running server instance
pub struct Server {
    pub db: Store, // should not be pub
    master_key: EncServerKey,
    port: u16,
}

pub trait Protocol {
    fn get_master(&self) -> EncServerKey;
    fn list(&self) -> Result<Vec<Header>, Error>;
    fn put(&mut self, secret: EncSecret) -> Result<(), Error>;
    fn get(&self, header: Header) -> Result<Option<EncSecret>, Error>;
    fn delete(&mut self, header: Header) -> EncSecret;
}

impl Server {
    pub fn init<P: AsRef<Path>>(
        db_path: P,
        key_path: P,
        port: u16,
    ) -> Result<Self, Error> {
        Ok(Self {
            db: Store::load(db_path)?,
            master_key: EncServerKey::load(key_path)?,
            port,
        })
    }

    /// Start serving requests
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", self.port)
            .to_socket_addrs()?
            .next()
            .expect("could not parse address");

        tokio::task::LocalSet::new()
            .run_until(async move {
                let listener = tokio::net::TcpListener::bind(&addr).await?;
                info!("serving on {}", self.port);

                let rpc: keyserver::Client = capnp_rpc::new_client(self);

                loop {
                    let (stream, _) = listener.accept().await?;
                    info!("handling new conn {stream:?}");

                    let (reader, writer) =
                        tokio_util::compat::TokioAsyncReadCompatExt::compat(
                            stream,
                        )
                        .split();

                    let network = twoparty::VatNetwork::new(
                        reader,
                        writer,
                        rpc_twoparty_capnp::Side::Server,
                        Default::default(),
                    );

                    let rpc_system = RpcSystem::new(
                        Box::new(network),
                        Some(rpc.clone().client),
                    );
                    tokio::task::spawn_local(
                        rpc_system.map_err(|e| eprintln!("error: {e:?}")),
                    );
                }
            })
            .await
    }

    pub fn print_db(&self) {
        self.db.dump()
    }
}

impl Protocol for Server {
    fn get_master(&self) -> EncServerKey {
        self.master_key.clone()
    }

    fn list(&self) -> Result<Vec<Header>, Error> {
        self.db.list_secrets()
    }

    fn put(&mut self, secret: EncSecret) -> Result<(), Error> {
        self.db.put_secret(secret)
    }

    fn get(&self, header: Header) -> Result<Option<EncSecret>, Error> {
        self.db.get_secret(header)
    }

    fn delete(&mut self, header: Header) -> EncSecret {
        unimplemented!()
    }
}

impl keyserver::Server for Server {
    fn get_master(
        &mut self,
        _: keyserver::GetMasterParams,
        mut results: keyserver::GetMasterResults,
    ) -> Promise<(), capnp::Error> {
        let master = <Server as Protocol>::get_master(self);
        let mut res = results.get().init_key();
        res.set_salt(&master.salt);
        res.set_nonce(&master.nonce);
        res.set_rawkey(&master.server_key[..]);

        Promise::ok(())
    }

    fn list(
        &mut self,
        _: keyserver::ListParams,
        mut results: keyserver::ListResults,
    ) -> Promise<(), capnp::Error> {
        let secrets = bry!(<Server as Protocol>::list(self));

        let mut res = results.get().init_keys(secrets.len() as u32);
        secrets.into_iter().enumerate().for_each(|(i, sec)| {
            res.reborrow().get(i as u32).set_id(&sec.id.bytes()[..]);
            res.reborrow().get(i as u32).set_tag(conv_tag!(sec.tag));
            res.reborrow()
                .get(i as u32)
                .set_desc(&sec.desc.unwrap_or_default());
        });

        Promise::ok(())
    }

    fn put(
        &mut self,
        params: keyserver::PutParams,
        mut results: keyserver::PutResults,
    ) -> Promise<(), capnp::Error> {
        Promise::ok(())
    }

    fn get(
        &mut self,
        params: keyserver::GetParams,
        mut results: keyserver::GetResults,
    ) -> Promise<(), capnp::Error> {
        Promise::ok(())
    }

    fn delete(
        &mut self,
        params: keyserver::DeleteParams,
        mut results: keyserver::DeleteResults,
    ) -> Promise<(), capnp::Error> {
        Promise::ok(())
    }
}
