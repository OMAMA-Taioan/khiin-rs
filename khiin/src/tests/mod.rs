#![cfg(test)]

use std::path::PathBuf;

pub fn debug_db_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("..")
    .join("target")
    .join("debug")
    .join("khiin.db")
}
