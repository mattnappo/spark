use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, TryFutureExt};
use log::info;
use regex::Regex;
use std::error::Error;
use std::net::ToSocketAddrs;

use crate::map_capnp;
use crate::server::DType;

#[derive(Debug, PartialEq)]
enum Method {
    Put,
    Get,
    Del,
}

struct Args {
    id: u64,
    label: String,
    data: Option<Vec<u8>>,
    dtype: Option<DType>,
}

#[derive(Debug)]
struct ParseError(String);

impl ParseError {
    fn fail(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.0)
    }
}

impl Error for ParseError {}

struct Parser;

/// Syntax
/// put {id:num,label:"text"} {data:[bytes],type:{a,b,c,d}}
/// get {id:num,label:text} {}
impl Parser {
    fn parse_data(mut data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        data.to_string().retain(|c| !c.is_whitespace());
        let mut chars = data.chars();
        // Remove first and last char
        chars.next();
        chars.next_back();
        chars.as_str();

        // Split
        Ok(data.split(",").filter_map(|n| n.parse::<u8>().ok()).collect::<Vec<u8>>())
    }

    pub fn parse_query(query: &str) -> Result<(Method, Args), Box<dyn Error>> {
        let mut query = query.to_string();
        query.retain(|c| !c.is_whitespace());

        let re = Regex::new(r"(?P<method>put|get)\s*\{\s*id:\s*(?P<id>[0-9]*),\s*label:\s*'(?P<label>[A-z0-9\s]*)'}(?:\s*,\s*\{data:\s*(?P<data>\[\s*(?:\d+)(?:,\s*\d+)*\s*\]),\s*type:\s*(?P<type>a|b|c)})?")?;

        let params = re.captures_iter(&query).next();
        if params.is_none() {
            return Err(Box::new(ParseError::fail("no matches of query")));
        }
        let params = params.unwrap();

        let method = match &params["method"] {
            "get" => Method::Get,
            "put" => Method::Put,
            "del" => Method::Del,
            _ => unreachable!(),
        };

        let id = params["id"].parse::<u64>()?;
        let label = &params["label"];
        let mut data = None;
        let mut dtype = None;
        if method == Method::Put {
            data = Some(Parser::parse_data(&params["data"])?);
            dtype = Some(match &params["type"] {
                "a" => DType::A,
                "b" => DType::B,
                "c" => DType::C,
                _ => unreachable!(),
            });
        }

        println!("{method:?} {id:?} {label:?} {data:?} {dtype:?}");

        Ok((
            method,
            Args {
                id: id,
                label: label.to_string(),
                data: data,
                dtype: dtype,
            },
        ))
    }
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
            //Parser::parse_query(query).unwrap();
            //let mut request = map_capnp::map::put_request();

            Ok(())
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        Parser::parse_query("put {id: 1234, label:'this is the label'}, {data: [10, 20, 30, 40], type:c}").unwrap();
        Parser::parse_query("get {id: 1234, label:'this is the label'}").unwrap();
    }
}
