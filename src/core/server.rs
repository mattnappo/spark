use std::path::Path;

/// A running server instance
pub struct Server {
    /// A database mapping from `Header`s to `EncSecret`s
    store: sled::Db,

    /// The RPC server instance
    rpc: Option<()>,

    /// The server's key (in memory)
    key: ServerKey,
}

impl Server {
    /// Initialize a new server given path to key and database
    fn load(key: Path, store: Path) {}

    fn insert_secret(&mut self, secret: EncSecret) {}

    fn serve() {}
}
