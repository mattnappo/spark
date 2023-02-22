use crate::protocol_capnp;
use crate::protocol_capnp::keyserver;
use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, TryFutureExt};
use log::{debug, info};
use std::net::ToSocketAddrs;

struct Client {
addr: &str
}


impl Client {

    pub fn init(addr: &str) -> Self {

        Self { addr: addr.to_socket_addrs()?.next().expect("invalid addrress")}
    }

pub async fn request(&self) -> Result<(), Box<dyn Error>> {

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

            let mut rpc_system = RpcSystem::new(Box::new(rpc_network), None);
            let client: keyserver::Client =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

            tokio::task::spawn_local(rpc_system);

            match request.method {
                Method::List => {
                    let mut req = client.list_request();
                    let res = req.send().promise.await?;
                    info!("response: {res:?}");
                }
                _ => ()
            }

            Ok(())
        })
        .await
}
}
