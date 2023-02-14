use std::env;
use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};

pub mod client;
pub mod map_capnp;
pub mod server;

static USAGE: &str =
    "usage:\n  ./testing_proto --[server|client] [port|addr:port]";

async fn client(addr: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn server(port: u16) -> Result<(), Box<dyn Error>> {
    server::serve(port).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let argc = args.len() - 1;
    if argc != 2 {
        panic!("{}", USAGE);
    }

    env_logger::init();

    match args[1].as_ref() {
        "--client" => client(args[2].as_ref()).await?,
        "--server" => {
            server(args[2].parse::<u16>().expect("invalid port")).await?
        }
        _ => panic!("{}", USAGE),
    }

    Ok(())
}
