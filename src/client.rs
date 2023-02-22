use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use spark::core::client;
use spark::Error;
use spark::BOOTSTRAP_FILE;
use std::env;
use std::fs;
use std::path::PathBuf;
use toml::Table;

#[derive(Subcommand, Debug, Clone)]
enum Put {
    File { path: PathBuf },
    Credentials { username: String, password: String },
}

#[derive(ValueEnum, Debug, Clone)]
enum Scope {
    Public,
    Local,
}

#[derive(Subcommand, Debug, Clone)]
enum Method {
    #[command(about = "List all secrets on the server")]
    List,
    #[command(about = "Get a secret(s) from the server")]
    Get { name: String, id: Option<String> },
    #[command(about = "Store a secret on the server")]
    Put {
        name: String,
        #[arg(value_enum)]
        scope: Scope,
        #[command(subcommand)]
        data: Put,
        #[arg(long = "desc")]
        desc: Option<String>,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("host")
    .required(true)
    .args(["addr", "peer"]),
    ))]
struct Args {
    #[arg(long, help = "Connect to arbitrary host of the form <ip>:<port>")]
    addr: Option<String>,
    #[arg(long, help = "Connect to known peer from bootstrap file")]
    peer: Option<String>,
    #[command(subcommand)]
    method: Method,
}

fn bootstrap(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let s = fs::read_to_string(BOOTSTRAP_FILE)?;
    let file = s.parse::<Table>()?;
    let peer = &file
        .get("peers")
        .expect("malformed bootstrap file: see README")
        .get(name)
        .expect(format!("could not find bootstrap peer \"{name}\"").as_str());
    println!("peer: {peer:?}");
    Ok(peer.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{args:#?}");

    env_logger::init();

    let addr = match (args.addr, args.peer) {
        (Some(addr), None) => addr,
        (None, Some(peer)) => bootstrap(&peer)?,
        (_, _) => unreachable!(),
    };

    println!("addr: {addr:?}");

    let client = client::Client::init(&addr)?;

    Ok(())
}
