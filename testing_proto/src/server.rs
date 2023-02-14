use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, TryFutureExt};
use log::{debug, info};

use crate::map_capnp;
use crate::map_capnp::map;

use std::collections::HashMap;
use std::default::Default;
use std::net::ToSocketAddrs;

#[derive(Hash, PartialEq, Eq, Debug)]
struct DBKey {
    id: u64,
    label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DType {
    A,
    B,
    C,
    Fail,
}

impl DType {
    pub fn from(label: &str) -> Self {
        match label {
            "a" => DType::A,
            "b" => DType::B,
            "c" => DType::C,
            _ => panic!(),
        }
    }

    pub fn from_capn(dtype: map_capnp::Type) -> DType {
        match dtype {
            map_capnp::Type::A => DType::A,
            map_capnp::Type::B => DType::B,
            map_capnp::Type::C => DType::C,
        }
    }

    pub fn to_capn(dtype: DType) -> map_capnp::Type {
        match dtype {
            DType::A => map_capnp::Type::A,
            DType::B => map_capnp::Type::B,
            DType::C => map_capnp::Type::C,
            DType::Fail => map_capnp::Type::A,
        }
    }
}

impl Default for DType {
    fn default() -> Self {
        Self::Fail
    }
}

#[derive(Default, Clone, Debug)]
struct DBVal {
    data: Vec<u8>,
    dtype: DType,
}

// Represents internal logic/code separate from capn
type DB = HashMap<DBKey, DBVal>;

#[derive(Default)]
struct MapImpl {
    db: DB,
}

fn potential_err() -> Result<(), i32> {
    Err(31)
}

impl map::Server for MapImpl {
    fn get(
        &mut self,
        params: map::GetParams,
        mut results: map::GetResults,
    ) -> Promise<(), capnp::Error> {
        let key = pry!(params.get()).get_key();
        let id: u64 = pry!(key).get_id();
        let label: String = pry!(pry!(key).get_label()).to_string();
        let val: DBVal = self
            .db
            .get(&DBKey { id, label })
            .cloned()
            .unwrap_or_default();

        let mut res = results.get().init_val();
        res.set_data(&val.data);
        res.set_type(DType::to_capn(val.dtype));
        Promise::ok(())
    }

    fn put(
        &mut self,
        params: map::PutParams,
        mut results: map::PutResults,
    ) -> Promise<(), capnp::Error> {
        // Extract key info
        let key = pry!(params.get()).get_key();
        let id = pry!(key).get_id();
        let label = pry!(pry!(key).get_label()).to_string();

        // Extract val info
        let val = pry!(params.get()).get_val();
        let data = pry!(pry!(val).get_data()).to_vec();
        let dtype = DType::from_capn(pry!(pry!(val).get_type()));

        debug!("from req");
        debug!("{id:?}");
        debug!("{label:?}");
        debug!("{data:?}");
        debug!("{dtype:?}");

        // Internal logic
        debug!("before insert: {:?}", self.db);
        self.db.insert(
            DBKey {
                id,
                label: label.clone(),
            },
            DBVal {
                data: data.clone(),
                dtype,
            },
        );
        debug!("after insert: {:?}", self.db);

        // Build response
        let mut rtime = results.get().init_entry().init_info().init_time();
        rtime.set_minute(15);
        rtime.set_second(20);

        let mut rkey = pry!(results.get().get_entry()).init_key();
        rkey.set_id(id);
        rkey.set_label(&label);

        let mut rval = pry!(results.get().get_entry()).init_val();
        rval.set_data(&data[..]);
        rval.set_type(pry!(pry!(val).get_type()));

        Promise::ok(())
    }

    fn del(
        &mut self,
        _params: map::DelParams,
        mut _results: map::DelResults,
    ) -> Promise<(), capnp::Error> {
        match potential_err() {
            Ok(_) => Promise::ok(()),
            Err(e) => {
                Promise::err(capnp::Error::failed(format!("custom err {e}")))
            }
        }
    }
}

pub async fn serve(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("127.0.0.1:{port}")
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            let map_rpc: map::Client =
                capnp_rpc::new_client(MapImpl::default());

            info!("serving on {port}");

            loop {
                let (stream, _) = listener.accept().await?;
                info!("handling new conn {stream:?}");

                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream)
                        .split();

                let network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system = RpcSystem::new(
                    Box::new(network),
                    Some(map_rpc.clone().client),
                );
                tokio::task::spawn_local(
                    rpc_system.map_err(|e| eprintln!("error: {e:?}")),
                );
            }
        })
        .await
}
