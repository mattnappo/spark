use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, TryFutureExt};
use log::info;
use std::error::Error;
use std::net::ToSocketAddrs;

use crate::map_capnp;

enum Method {
    Put, Get, Del
}

enum Arg {
    Id(u64),
    Label(String),
    Data(Vec<u8>),
    Type(crate::server::DType),
}

#[derive(Debug)]
struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.0)
    }

}
impl Error for ParseError {}

fn parse_query(query: &str) -> Result<(Method, Vec<Arg>), Box<dyn Error>> {
    

}

async fn request(addr: &str, query: &str) -> Result<(), Box<dyn Error>> {
    let addr = addr.to_socket_addrs()?.next().expect("invalid addrress");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let stream = tokio::net::TcpStream::connect(&addr).await?;
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
            let map_server: map_capnp::map::Client =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

            tokio::task::spawn_local(rpc_system);

            // Build request
            let 
            let mut request = map_capnp::map::put_request()

            Ok(())
        })
        .await
}
