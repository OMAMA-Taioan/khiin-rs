use std::fs;
use std::path::*;

fn main() {
    // TODO: This needs to be replaced with something more flexible...
    #[cfg(debug_assertions)]
    copy_database("khiin.db");

    embed_resource::compile("res/khiin.rc", embed_resource::NONE);
}

#[allow(unused)]
fn copy_database(filename: &str) {
    let src_path = Path::new("../resources").join(filename);
    let dest_path = Path::new("../target/debug").join(filename);
    fs::copy(src_path, dest_path).expect(
        "Failed to copy file, is khiin.db in the res directory?",
    );
}
