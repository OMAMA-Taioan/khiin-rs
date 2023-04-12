use anyhow::anyhow;
use anyhow::Result;

use crate::config::engine_cfg::InputMode;
use crate::data::dictionary::Dictionary;
use crate::input::tokenize;

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
        dict: &Dictionary,
        ch: char,
        mode: InputMode,
    ) -> Result<()> {
        match mode {
            InputMode::Continuous => self.insert_continuous(dict, ch),
            InputMode::SingleWord => self.insert_single_word(ch),
            InputMode::Manual => self.insert_manual(ch),
        }
    }

    fn insert_continuous(&mut self, dict: &Dictionary, ch: char) -> Result<()> {
        self.mock_buffer.push(ch);
        tokenize(dict, &self.mock_buffer);
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
