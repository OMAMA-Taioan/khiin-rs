use std::path::Path;

fn main() {
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(Path::new("C:\\dev\\bin\\protoc.exe"))
        .cargo_out_dir("protos")
        .include("src")
        .input("src/command.proto")
        .input("src/config.proto")
        .run_from_script();
}
