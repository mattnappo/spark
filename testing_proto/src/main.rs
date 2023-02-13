use std::error::Error;

pub mod map_capnp;
pub mod server;

fn main() -> Result<(), Box<dyn Error>> {
    server::serve(3000)
}
