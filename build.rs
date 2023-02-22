extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .capnp_executable("/u/mnappo/Downloads/capnproto-c++-0.10.3/capnp")
        .output_path("src/")
        .src_prefix("src/proto/")
        .file("src/proto/protocol.capnp")
        .run()
        .unwrap();
}
