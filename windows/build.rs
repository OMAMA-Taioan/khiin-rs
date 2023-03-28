use std::path::Path;

use protobuf_codegen::Codegen;

extern crate embed_resource;

fn main() {
    embed_resource::compile("res/khiin.rc", embed_resource::NONE);

    Codegen::new()
        .protoc()
        .protoc_path(Path::new("C:\\dev\\bin\\protoc.exe"))
        .cargo_out_dir("protos")
        .include("src/protos")
        .input("src/protos/command.proto")
        .input("src/protos/config.proto")
        .run_from_script();
}
