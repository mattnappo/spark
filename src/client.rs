use clap::{Parser, Subcommand, ValueEnum};
use std::env;
use std::error::Error;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
enum PutType {
    File { path: PathBuf },
    Credentials { username: String, password: String },
}

#[derive(ValueEnum, Debug, Clone)]
enum Scope {
    Public,
    Local,
}

#[derive(Subcommand, Debug)]
enum Method {
    List,
    Get {
        name: String,
        id: Option<String>,
    },
    Put {
        name: String,
        #[arg(value_enum)]
        scope: Scope,
        #[command(subcommand)]
        dtype: PutType,
        #[arg(long = "desc")]
        desc: Option<String>,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    //#[arg(name = "ip")] // for lowercase
    ip: String,
    port: u16,
    #[command(subcommand)]
    method: Method,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{args:#?}");
    Ok(())
}
