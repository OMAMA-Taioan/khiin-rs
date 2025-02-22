#![cfg(test)]

pub(crate) mod mock_protos;

use std::path::PathBuf;

// use khiin_db::models;
// use khiin_db::models::KeyConversion;
// use khiin_db::tests::debug_db_path;
// use khiin_db::tests::get_db;
// use khiin_db::Database;

use crate::buffer::BufferMgr;
use crate::config::Config;
use crate::config::ToneMode;
use crate::data::Dictionary;
use crate::db::models::InputType;
use crate::db::models::KeyConversion;
use crate::db::Database;
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

pub(crate) fn get_db() -> Database {
    let db_path = debug_db_path();
    Database::new(db_path.to_str().unwrap()).unwrap()
}

pub(crate) fn get_engine() -> Option<Engine> {
    let filename = debug_db_path();
    Engine::new(filename)
}

pub(crate) fn get_dict() -> Dictionary {
    let db = get_db();
    Dictionary::new(&db, ToneMode::Numeric).expect("Could not load dictionary")
}

pub fn get_conf() -> Config {
    Config::new()
}

pub fn mock_conversion(input: &str, output: &str) -> KeyConversion {
    KeyConversion {
        key_sequence: String::new(),
        input_type: InputType::Numeric,
        input: input.into(),
        input_id: 0,
        output: output.into(),
        weight: 0,
        khin_ok: true,
        khinless_ok: true,
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
