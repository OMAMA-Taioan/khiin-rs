use std::fmt::format;
use std::path::PathBuf;

use anyhow::Result;
use rusqlite::Connection;
use rusqlite::OpenFlags;

use crate::config::engine_cfg::InputType;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(file: &PathBuf) -> Result<Self> {
        let mut mem_conn = Connection::open_in_memory_with_flags(
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;

        mem_conn.restore(rusqlite::DatabaseName::Main, file, Some(|_| {}))?;

        Ok(Database { conn: mem_conn })
    }

    pub fn all_words_by_freq(
        &self,
        input_type: InputType,
    ) -> Result<Vec<String>> {
        let table = match input_type {
            InputType::Numeric => "input_numeric",
            InputType::Telex => "input_telex",
        };
        let sql = format!("SELECT key_sequence FROM {} ORDER BY input_id ASC", table);

        let mut result = Vec::new();
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            result.push(row.get("key_sequence")?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn it_finds_the_db_file() {
        let dbfile = debug_db_path();
        println!("dbfile: {}", dbfile.display());
        assert!(dbfile.exists());
    }

    #[test]
    fn it_loads_the_db_file() {
        let db = Database::new(&debug_db_path());
        assert!(db.is_ok());
    }

    #[test]
    fn it_loads_results() {
        let db = Database::new(&debug_db_path()).expect("Could not load DB");
        let res = db.all_words_by_freq(InputType::Numeric);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.len() > 100);
        let r0 = res[0].as_str();
        let r1 = res[1].as_str();
        let r2 = res[2].as_str();
        assert_eq!(r0, "e5");
        assert_eq!(r1, "goa2");
        assert_eq!(r2, "li2");
    }
}
