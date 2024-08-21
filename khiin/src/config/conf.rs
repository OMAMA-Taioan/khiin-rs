use crate::db::models::InputType;

#[derive(PartialEq, Copy, Clone)]
pub enum InputMode {
    Continuous,
    Classic,
    Manual,
}

#[derive(PartialEq, Copy, Clone)]
pub enum ToneMode {
    Numeric,
    Telex,
}

impl Into<InputType> for ToneMode {
    fn into(self) -> InputType {
        match self {
            ToneMode::Numeric => InputType::Numeric,
            ToneMode::Telex => InputType::Telex,
        }
    }
}

pub struct KeyConfig {
    pub t2: char,
    pub t3: char,
    pub t5: char,
    pub t6: char,
    pub t7: char,
    pub t8: char,
    pub t9: char,
    pub khin: char,
    pub hyphon: char,
    pub done: char,
}
pub struct Config {
    enabled: bool,
    input_mode: InputMode,
    tone_mode: ToneMode,
    key_config: KeyConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            enabled: false,
            input_mode: InputMode::Manual,
            tone_mode: ToneMode::Telex,
            key_config: KeyConfig {
                t2: 's',
                t3: 'f',
                t5: 'l',
                t6: 'x',
                t7: 'j',
                t8: 'j',
                t9: 'w',
                khin: 'v',
                hyphon: 'd',
                done: 'r',
            },
        }
    }

    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn tone_mode(&self) -> ToneMode {
        self.tone_mode
    }

    pub fn t2(&self) -> char {
        if self.tone_mode == ToneMode::Numeric {
            '2'
        } else {
            self.key_config.t2
        }
    }

    pub fn t3(&self) -> char {
        if self.tone_mode == ToneMode::Numeric {
            '3'
        } else {
            self.key_config.t3
        }
    }

    pub fn t5(&self) -> char {
        if self.tone_mode == ToneMode::Numeric {
            '5'
        } else {
            self.key_config.t5
        }
    }

    pub fn t6(&self) -> char {
        if self.tone_mode == ToneMode::Numeric {
            '6'
        } else {
            self.key_config.t6
        }
    }

    pub fn t7(&self) -> char {
        if self.tone_mode == ToneMode::Numeric {
            '7'
        } else {
            self.key_config.t7
        }
    }

    pub fn t8(&self) -> char {
        if self.tone_mode == ToneMode::Numeric {
            '8'
        } else {
            self.key_config.t8
        }
    }

    pub fn t9(&self) -> char {
        if self.tone_mode == ToneMode::Numeric {
            '9'
        } else {
            self.key_config.t9
        }
    }

    pub fn khin(&self) -> char {
        self.key_config.khin
    }

    pub fn hyphon(&self) -> char {
        self.key_config.hyphon
    }

    pub fn done(&self) -> char {
        self.key_config.done
    }

    pub fn is_reserved_char(&self, ch: char) -> bool {
        if ch == self.key_config.khin {
            true
        } else if ch == self.key_config.hyphon {
            true
        } else if ch == self.key_config.done {
            true
        } else if ch == self.key_config.t2 {
            true
        } else if ch == self.key_config.t3 {
            true
        } else if ch == self.key_config.t5 {
            true
        } else if ch == self.key_config.t6 {
            true
        } else if ch == self.key_config.t7 {
            true
        } else if ch == self.key_config.t8 {
            true
        } else if ch == self.key_config.t9 {
            true
        } else {
            false
        }
    }

    pub fn is_tone_char(&self, ch: char) -> bool {
        if ch == self.key_config.t2 {
            true
        } else if ch == self.key_config.t3 {
            true
        } else if ch == self.key_config.t5 {
            true
        } else if ch == self.key_config.t6 {
            true
        } else if ch == self.key_config.t7 {
            true
        } else if ch == self.key_config.t8 {
            true
        } else if ch == self.key_config.t9 {
            true
        } else {
            false
        }
    }
    
    // set input_mode
    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }

    // set tone_mode
    pub fn set_tone_mode(&mut self, mode: ToneMode) {
        self.tone_mode = mode;
    }
}
