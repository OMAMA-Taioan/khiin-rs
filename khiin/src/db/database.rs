use std::fs::create_dir_all;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use once_cell::sync::Lazy;
use rusqlite::backup::Progress;
use rusqlite::named_params;
use rusqlite::params_from_iter;
use rusqlite::Connection;
use rusqlite::DatabaseName;
use rusqlite::Row;
use rusqlite_migration::Migrations;
use rusqlite_migration::M;

use super::init::sql_gen::build_sql;
use super::models::InputType;
use super::models::KeyConversion;
use super::models::KeySequence;

static MIGRATIONS: Lazy<Migrations> = Lazy::new(|| {
    Migrations::new(vec![M::up(include_str!("migrations/001/up.sql"))])
});

type Noop = Box<dyn Fn(Progress)>;

pub struct Database {
    conn: Connection,
    file: PathBuf,
}

impl Deref for Database {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

impl Database {
    pub fn new<P>(file: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let conn = Connection::open_in_memory()?;
        let file = file.as_ref().to_path_buf();
        let this = Self { conn, file };

        if this.file.exists() {
            this.open()
        } else {
            this.init()
        }
    }

    fn init(mut self) -> Result<Self> {
        self.set_pragmas()?;
        self.migrate_to_latest()?;
        build_sql(&mut self.conn)?;
        self.backup()?;
        Ok(self)
    }

    fn open(mut self) -> Result<Self> {
        self.restore()?;
        self.migrate_to_latest()?;
        Ok(self)
    }

    fn migrate_to_latest(&mut self) -> Result<()> {
        MIGRATIONS.to_latest(&mut self.conn)?;
        Ok(())
    }

    fn set_pragmas(&self) -> Result<()> {
        self.pragma_update(None, "journal_mode", "WAL")?;
        self.pragma_update(None, "foreign_keys", "ON")?;
        Ok(())
    }

    fn backup(&self) -> Result<()> {
        ensure_dirs(&self.file)?;
        self.conn.backup(DatabaseName::Main, &self.file, None)?;
        Ok(())
    }

    fn restore(&mut self) -> Result<()> {
        self.conn
            .restore(DatabaseName::Main, &self.file, None::<Noop>)?;
        Ok(())
    }

    pub fn select_all_words_by_freq(
        &self,
        input_type: InputType,
    ) -> Result<Vec<KeySequence>> {
        let sql = include_str!("sql/select_all_words_by_freq.sql");

        let mut result = Vec::new();
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query([input_type as i64])?;

        while let Some(row) = rows.next()? {
            result.push(row.try_into()?);
        }

        Ok(result)
    }

    pub fn select_conversions(
        &self,
        input_type: InputType,
        query: &str,
        limit: Option<usize>,
    ) -> Result<Vec<KeyConversion>> {
        let sql = match limit {
            Some(n) => format!(
                include_str!("sql/select_conversions.sql"),
                limit = format!("limit {}", n)
            ),
            None => {
                format!(include_str!("sql/select_conversions.sql"), limit = "")
            },
        };

        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query(named_params! {
            ":query": query,
            ":input_type": input_type as i64,
        })?;

        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(row.try_into()?);
        }

        Ok(result)
    }

    pub fn select_conversions_by_hanlo(
        &self,
        input_type: InputType,
        query: &str,
        is_hanji_first: bool,
        is_khinless: bool,
    ) -> Result<Vec<KeyConversion>> {
        let sql = if is_hanji_first {
            format!(
                include_str!("sql/select_conversions_by_hanji.sql"),
                limit = "limit 1",
                khin_mode = if is_khinless {
                    "khinless_ok"
                } else {
                    "khin_ok"
                }
            )
        } else {
            format!(
                include_str!("sql/select_conversions_by_lomaji.sql"),
                limit = "limit 1",
                khin_mode = if is_khinless {
                    "khinless_ok"
                } else {
                    "khin_ok"
                }
            )
        };

        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query(named_params! {
            ":query": query,
            ":input_type": input_type as i64,
        })?;

        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(row.try_into()?);
        }

        Ok(result)
    }

    pub fn select_conversions_for_tone(
        &self,
        input_type: InputType,
        query: &str,
        is_hanji_first: bool,
        is_khinless: bool,
    ) -> Result<Vec<KeyConversion>> {
        let sql = if is_hanji_first {
            format!(
                include_str!("sql/select_conversions_for_tone_by_hanji.sql"),
                limit = "",
                khin_mode = if is_khinless {
                    "khinless_ok"
                } else {
                    "khin_ok"
                }
            )
        } else {
            format!(
                include_str!("sql/select_conversions_for_tone_by_lomaji.sql"),
                limit = "",
                khin_mode = if is_khinless {
                    "khinless_ok"
                } else {
                    "khin_ok"
                }
            )
        };
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query(named_params! {
            ":query": query,
            ":input_type": input_type as i64,
        })?;

        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(row.try_into()?);
        }

        Ok(result)
    }

    pub fn select_conversions_for_word(
        &self,
        input_type: InputType,
        query: &str,
        detoned_query: &str,
        is_hanji_first: bool,
        is_khinless: bool,
    ) -> Result<Vec<KeyConversion>> {
        let sql = if is_hanji_first {
            format!(
                include_str!("sql/select_conversions_for_word_by_hanji.sql"),
                limit = "",
                khin_mode = if is_khinless {
                    "khinless_ok"
                } else {
                    "khin_ok"
                }
            )
        } else {
            format!(
                include_str!("sql/select_conversions_for_word_by_lomaji.sql"),
                limit = "",
                khin_mode = if is_khinless {
                    "khinless_ok"
                } else {
                    "khin_ok"
                }
            )
        };
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query(named_params! {
            ":query": query,
            ":input_type": input_type as i64,
            ":detoned_query": detoned_query,
        })?;

        let mut result: Vec<KeyConversion> = Vec::new();
        while let Some(row) = rows.next()? {
            let mut detoned_row: KeyConversion = row.try_into()?;
            detoned_row.key_sequence = detoned_query.to_string();
            result.push(detoned_row);
        }

        Ok(result)
    }

    pub fn select_conversions_for_multiple(
        &self,
        input_type: InputType,
        words: &Vec<&str>,
    ) -> Result<Vec<KeyConversion>> {
        let sql = format!(
            include_str!("sql/select_conversions_for_multiple.sql"),
            vars = repeat_vars(words.len()),
            input_type = input_type as i64,
        );

        log::trace!("{}", sql);
        log::trace!("{:?}", words);
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query(params_from_iter(words))?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(row.try_into()?)
        }

        Ok(result)
    }
}

impl TryFrom<&Row<'_>> for KeySequence {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> std::result::Result<Self, Self::Error> {
        Ok(KeySequence {
            input_id: row.get("input_id")?,
            keys: row.get("key_sequence")?,
            input_type: row.get("input_type")?,
            n_syls: row.get("n_syls")?,
            p: row.get("p")?,
        })
    }
}

impl TryFrom<&Row<'_>> for KeyConversion {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> std::result::Result<Self, Self::Error> {
        Ok(KeyConversion {
            key_sequence: row.get("key_sequence")?,
            input_type: row.get("input_type")?,
            input: row.get("input")?,
            input_id: row.get("input_id")?,
            output: row.get("output")?,
            weight: row.get("weight")?,
            khin_ok: row.get("khin_ok")?,
            khinless_ok: row.get("khinless_ok")?,
            annotation: row.get("annotation")?,
        })
    }
}

// from rusqlite docs
fn repeat_vars(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}

#[cfg(feature = "db_cli")]
mod cli {
    use std::path::PathBuf;

    use crate::db::init::csv::CsvFiles;
    use crate::db::sql_gen::build_sql_from_csv;
    use anyhow::Result;
    use rusqlite::Connection;

    use super::Database;

    impl Database {
        pub fn from_csv(db_file: &str, csv_files: CsvFiles) -> Result<Self> {
            let conn = Connection::open_in_memory()?;
            let file = PathBuf::from(db_file);
            let mut db = Self { conn, file };

            db.set_pragmas()?;
            db.migrate_to_latest()?;
            build_sql_from_csv(&mut db.conn, csv_files)?;
            db.backup()?;
            Ok(db)
        }
    }
}

fn ensure_dirs(db_file: &PathBuf) -> Result<()> {
    if !db_file.exists() {
        if let Some(p) = db_file.parent() {
            create_dir_all(p)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn it_finds_the_db_file() {
        let dbfile = debug_db_path();
        log::debug!("dbfile: {}", dbfile.display());
        assert!(dbfile.exists());
    }

    #[test]
    fn it_loads_the_db_file() {
        let db = Database::new(&debug_db_path());
        assert!(db.is_ok());
    }

    #[test_log::test]
    fn it_loads_results() {
        let db = Database::new(&debug_db_path()).expect("Could not load DB");
        let res = db.select_all_words_by_freq(InputType::Numeric);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.len() > 100);
        let r0 = res[0].keys.as_str();
        let r1 = res[1].keys.as_str();
        let r2 = res[2].keys.as_str();
        assert_eq!(r0, "e5");
        assert_eq!(r1, "e");
        assert_eq!(r2, "goa2");
    }

    #[test]
    fn it_finds_conversions() {
        let db = get_db();
        let res = db
            .select_conversions(InputType::Numeric, "ho2", None)
            .unwrap();
        assert!(res.len() >= 2);
        assert!(res.iter().any(|row| row.output == "好"));
        assert!(res.iter().any(|row| row.output == "hó"));
        assert!(res[0].annotation.is_none());
    }

    #[test_log::test]
    fn it_converts_by_id_vec() {
        let db = get_db();
        let words = vec!["ho", "hong"];
        let res = db
            .select_conversions_for_multiple(InputType::Numeric, &words)
            .unwrap();
        assert!(res.len() >= 20);
    }
}
