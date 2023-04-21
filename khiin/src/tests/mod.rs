#![cfg(test)]

pub(crate) mod mock_protos;

use std::path::PathBuf;

use crate::buffer::BufferMgr;
use crate::config::Config;
use crate::config::InputType;
use crate::data::models::Conversion;
use crate::data::Database;
use crate::data::Dictionary;
use crate::engine::EngInner;
use crate::Engine;

pub(crate) use mock_protos::*;

pub(crate) fn debug_db_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("debug")
        .join("khiin.db")
}

pub(crate) fn debug_db_filename() -> String {
    debug_db_path().into_os_string().into_string().unwrap()
}

pub(crate) fn get_engine() -> Option<Engine> {
    let filename = debug_db_filename();
    Engine::new(filename.as_str())
}

pub(crate) fn get_db() -> Database {
    let db_path = debug_db_path();
    Database::new(&db_path).unwrap()
}

pub(crate) fn get_dict() -> Dictionary {
    let db = get_db();
    Dictionary::new(&db, InputType::Numeric).expect("Could not load dictionary")
}

pub fn get_conf() -> Config {
    Config::new()
}

pub fn mock_conversion(input: &str, output: &str) -> Conversion {
    Conversion {
        key_sequence: String::new(),
        input: input.into(),
        input_id: 0,
        output: output.into(),
        weight: 0,
        category: None,
        annotation: None,
    }
}

pub(crate) fn test_harness() -> (EngInner, BufferMgr) {
    (
        EngInner {
            db: get_db(),
            dict: get_dict(),
            conf: get_conf(),
        },
        BufferMgr::new(),
    )
}
