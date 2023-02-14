use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::AsyncReadExt;
use log::{debug, info};
use regex::Regex;
use std::error::Error;
use std::net::ToSocketAddrs;

use crate::map_capnp;
use crate::server::DType;

#[derive(Debug, PartialEq)]
enum Method {
    Put(u64, String, Vec<u8>, DType),
    Get(u64, String),
    Del,
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
    fn parse_data(data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        data.to_string().retain(|c| !c.is_whitespace());
        let mut chars = data.chars();
        // Remove first and last char
        chars.next();
        chars.next_back();
        let data = chars.collect::<String>();

        // Split
        Ok(data
            .split(",")
            .filter_map(|n| n.parse::<u8>().ok())
            .collect::<Vec<u8>>())
    }

    pub fn parse_query(query: &str) -> Result<Method, Box<dyn Error>> {
        let mut query = query.to_string();
        query.retain(|c| !c.is_whitespace());

        let re = Regex::new(
            r"(?P<method>put|get)\s*\{\s*id:\s*(?P<id>[0-9]*),\s*label:\s*'(?P<label>[A-z0-9\s]*)'}(?:\s*,\s*\{data:\s*(?P<data>\[\s*(?:\d+)(?:,\s*\d+)*\s*\]),\s*type:\s*(?P<type>a|b|c)})?",
        )?;

        let params = re.captures_iter(&query).next();
        if params.is_none() {
            return Err(Box::new(ParseError::fail("no matches of query")));
        }
        let params = params.unwrap();

        let id = params["id"].parse::<u64>()?;
        let label = &params["label"];

        let r = Ok(match &params["method"] {
            "get" => Method::Get(id, label.to_string()),
            "put" => Method::Put(
                id,
                label.to_string(),
                Parser::parse_data(&params["data"])?,
                DType::from(&params["type"]),
            ),
            _ => unreachable!(),
        });
        debug!("parsed: {r:?}");
        r
    }
}

pub async fn request(addr: &str, query: &str) -> Result<(), Box<dyn Error>> {
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
            match Parser::parse_query(query)? {
                Method::Get(id, label) => {
                    let mut req = map_server.get_request();
                    req.get().init_key().set_id(id);
                    req.get().get_key()?.set_label(&label);

                    // Send
                    let res = req.send().promise.await?;
                    info!(
                        "res: val={{ data: {:?}, dtype: {:?} }}",
                        res.get()?.get_val()?.get_data()?,
                        res.get()?.get_val()?.get_type()
                    );
                }
                Method::Put(id, label, data, dtype) => {
                    let mut req = map_server.put_request();
                    debug!("{id:?}");
                    debug!("{label:?}");
                    debug!("{data:?}");
                    debug!("{dtype:?}");
                    // Something here is wrong
                    req.get().init_key().set_id(id);
                    req.get().get_key()?.set_label(&label);
                    req.get().init_val().set_data(&data[..]);
                    req.get().get_val()?.set_type(DType::to_capn(dtype));

                    // Send
                    let res = req.send().promise.await?;
                    info!(
                        "res: key={{ id: {}, label: {:?} }} val={{ data: {:?}, dtype: {:?} }}, info={{ time={{ min={}, sec={} }} }}",
                        res.get()?.get_entry()?.get_key()?.get_id(),
                        res.get()?.get_entry()?.get_key()?.get_label()?,
                        res.get()?.get_entry()?.get_val()?.get_data()?,
                        res.get()?.get_entry()?.get_val()?.get_type()?,
                        res.get()?.get_entry()?.get_info()?.get_time()?.get_minute(),
                        res.get()?.get_entry()?.get_info()?.get_time()?.get_second(),
                    );
                }
                _ => unreachable!(),
            }

            Ok(())
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_parser() {
        println!("{:?}", Parser::parse_data("[4,5,6]").unwrap());
    }

    #[test]
    fn test_parser() {
        println!("{:?}", Parser::parse_query("put {id: 1234, label:'this is the label'}, {data: [10, 20, 30, 40], type:c}").unwrap());
        println!(
            "{:?}",
            Parser::parse_query("get {id: 1234, label:'this is the label'}")
                .unwrap()
        );

        println!(
            "{:?}",
            Parser::parse_query("put{id:123,label:'hi'},{data:[4,5,6],type:c}")
                .unwrap()
        );
    }
}
