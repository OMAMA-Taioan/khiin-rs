use anyhow::{Result, anyhow};

use crate::config::engine_cfg::InputMode;

use super::Buffer;

pub struct BufferMgr {
    buffer: Buffer,
    char_caret: usize,
}

impl BufferMgr {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::default(),
            char_caret: 0,
        }
    }

    pub fn insert(&mut self, ch: char, mode: InputMode) -> Result<()> {
        match mode {
            InputMode::Continuous => self.insert_continuous(ch),
            InputMode::SingleWord => self.insert_single_word(ch),
            InputMode::Manual => self.insert_manual(ch),
        }
    }

    fn insert_continuous(&mut self, ch: char) -> Result<()> {
        let composition = self.buffer.composition();

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
