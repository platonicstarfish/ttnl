use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let proto_path = Path::new("src/proto");

    // Compile the .proto files into Rust code
    prost_build::Config::new()
        .out_dir(out_path)
        .compile_protos(&[&proto_path.join("msg.proto")], &[proto_path])
        .unwrap();
}
