use std::fs;
use std::path::*;

fn main() {
    // TODO: This needs to be replaced with something more flexible...
    #[cfg(debug_assertions)]
    copy_debug_resource("khiin.db");

    embed_resource::compile("res/khiin.rc", embed_resource::NONE);
}

#[allow(unused)]
fn copy_debug_resource(filename: &str) {
    let src_path = Path::new("res").join(filename);
    let dest_path = Path::new("target/debug").join(filename);
    fs::copy(src_path, dest_path).expect(
        "Failed to copy file, is khiin.db in the res directory?",
    );
}
