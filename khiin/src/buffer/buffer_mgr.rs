use anyhow::anyhow;
use anyhow::Result;

use crate::config::Config;
use crate::config::InputMode;
use crate::data::Database;
use crate::data::Dictionary;
use crate::input::converter::convert_all;
use crate::input::parse_input;
use crate::Engine;

use super::Buffer;

pub struct BufferMgr {
    buffer: Buffer,
    mock_buffer: String,
    char_caret: usize,
}

impl BufferMgr {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::default(),
            mock_buffer: String::new(),
            char_caret: 0,
        }
    }

    pub fn insert(
        &mut self,
        db: &Database,
        dict: &Dictionary,
        conf: &Config,
        ch: char,
    ) -> Result<()> {
        match conf.input_mode() {
            InputMode::Continuous => self.insert_continuous(db, dict, conf, ch),
            InputMode::SingleWord => self.insert_single_word(ch),
            InputMode::Manual => self.insert_manual(ch),
        }
    }

    fn insert_continuous(
        &mut self,
        db: &Database,
        dict: &Dictionary,
        conf: &Config,
        ch: char,
    ) -> Result<()> {
        self.mock_buffer.push(ch);
        assert!(self.mock_buffer.is_ascii());
        convert_all(db, dict, conf, &self.mock_buffer)?;
        Ok(())
    }

    fn insert_single_word(&mut self, ch: char) -> Result<()> {
        Err(anyhow!("Not implemented"))
    }

    fn insert_manual(&mut self, ch: char) -> Result<()> {
        Err(anyhow!("Not implemented"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buf = BufferMgr::new();
        assert_eq!(buf.char_caret, 0);
    }
}
