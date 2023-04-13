use std::fmt::format;
use std::path::PathBuf;

use anyhow::Result;
use rusqlite::Connection;
use rusqlite::OpenFlags;

use crate::config::InputType;

use super::conversion::Conversion;

pub struct Input {
    pub id: u32,
    pub key_sequence: String,
    pub p: f64,
}

impl Input {
    pub fn new(id: u32, key_sequence: String, p: f64) -> Self {
        Self {
            id,
            key_sequence,
            p,
        }
    }
}

pub struct Database {
    conn: Connection,
}

static T_FREQ: &str = "frequency";
static T_CONV: &str = "conversions";
static T_KEYSEQ: &str = "key_sequences";
static T_METADATA: &str = "metadata";
static T_SYL: &str = "syllables";
static T_SYM: &str = "symbols";
static T_EMO: &str = "emoji";
static T_UGRAM: &str = "unigram_freq";
static T_BGRAM: &str = "bigram_freq";
static V_LOOKUP: &str = "conversion_lookups";
static V_GRAMS: &str = "ngrams";

fn input_column(input_type: InputType) -> &'static str {
    match input_type {
        InputType::Numeric => "numeric",
        InputType::Telex => "telex",
    }
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
    ) -> Result<Vec<Input>> {
        let input_col = input_column(input_type);
        let sql = format!(
            r#"
            select
                "input_id",
                "{column}",
                "p"
            from
                "{table}"
            order by "p" desc"#,
            column = input_col,
            table = T_KEYSEQ
        );

        let mut result = Vec::new();
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let input = Input::new(
                row.get("input_id")?,
                row.get(input_col)?,
                row.get("p")?,
            );
            result.push(input);
        }
        Ok(result)
    }

    pub fn find_conversions(
        &self,
        input_type: InputType,
        query: &str,
    ) -> Result<Vec<Conversion>> {
        let input_col = input_column(input_type);
        let sql = format!(
            r#"
            select *
            from "{table}"
            where "{column}" = :query"#,
            table = V_LOOKUP,
            column = input_col,
        );

        let mut result = Vec::new();
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query(&[(":query", query)])?;
        while let Some(row) = rows.next()? {
            result.push(Conversion {
                key_sequence: row.get(input_col)?,
                input: row.get("input")?,
                input_id: row.get("input_id")?,
                output: row.get("output")?,
                weight: row.get("weight")?,
                category: row.get("category")?,
                annotation: row.get("annotation")?,
            });
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
        let r0 = res[0].key_sequence.as_str();
        let r1 = res[1].key_sequence.as_str();
        let r2 = res[2].key_sequence.as_str();
        assert_eq!(r0, "e5");
        assert_eq!(r1, "e");
        assert_eq!(r2, "goa2");
    }

    #[test]
    fn it_finds_conversions() {
        let db = get_db();
        let res = db.find_conversions(InputType::Numeric, "ho2").unwrap();
        assert!(res.len() >= 2);
        assert!(res.iter().any(|row| row.output == "好"));
        assert!(res.iter().any(|row| row.output == "hó"));
        assert!(res[0].annotation.is_none());
        assert!(res[0].category.is_none());
    }
}
