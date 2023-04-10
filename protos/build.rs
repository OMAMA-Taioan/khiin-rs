use std::path::Path;

fn main() {
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .cargo_out_dir("protos")
        .include("src")
        .input("src/command.proto")
        .input("src/config.proto")
        .run_from_script();
}
