use clap::Parser;
use spark::core::server;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    port: u16,
    db_path: String,
    key_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    env_logger::init();

    let server =
        server::Server::init(&args.db_path, &args.key_path, args.port)?;
    server.serve().await
}
