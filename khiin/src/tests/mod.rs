#![cfg(test)]

use std::path::PathBuf;

use crate::config::InputType;
use crate::data::Database;
use crate::data::Dictionary;

pub fn debug_db_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("debug")
        .join("khiin.db")
}

pub fn get_db() -> Database {
    let db_path = debug_db_path();
    Database::new(&db_path).unwrap()
}

pub fn get_dict() -> Dictionary {
    let db = get_db();
    Dictionary::new(&db, InputType::Numeric).expect("Could not load dictionary")
}
