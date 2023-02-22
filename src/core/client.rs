use crate::protocol_capnp;
use crate::protocol_capnp::keyserver;
use crate::Error;
use crate::BOOTSTRAP_FILE;
use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, TryFutureExt};
use log::{debug, info};
use std::net;
use std::net::ToSocketAddrs;

pub struct Client {
    addr: net::SocketAddr,
}

impl Client {
    pub fn init(addr: &str) -> Result<Self, Error> {
        Ok(Self {
            addr: addr.to_socket_addrs()?.next().expect("invalid addrress"),
        })
    }

    pub async fn request(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::LocalSet::new()
            .run_until(async move {
                let stream = tokio::net::TcpStream::connect(&self.addr).await?;
                //stream.set_nodelay(true);
                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream)
                        .split();

                let rpc_network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Client,
                    Default::default(),
                );

                let mut rpc_system =
                    RpcSystem::new(Box::new(rpc_network), None);
                let client: keyserver::Client =
                    rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

                tokio::task::spawn_local(rpc_system);

                Ok(())
            })
            .await
    }
}
