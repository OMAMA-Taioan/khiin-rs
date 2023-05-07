use std::fs;
use std::path::*;

fn main() {
    embed_resource::compile("ime.rc", embed_resource::NONE);
}
