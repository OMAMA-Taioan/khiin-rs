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

#[derive(PartialEq, Copy, Clone)]
pub enum OutputMode {
    Lomaji,
    Hanji,
}

#[derive(PartialEq, Copy, Clone)]
pub enum KhinMode {
    Khinless,
    Hyphen,
    Dot,
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
    pub hyphen: char,
    pub done: char,
}
pub struct Config {
    enabled: bool,
    input_mode: InputMode,
    tone_mode: ToneMode,
    output_mode: OutputMode,
    khin_mode: KhinMode,
    key_config: KeyConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            enabled: false,
            input_mode: InputMode::Manual,
            tone_mode: ToneMode::Telex,
            output_mode: OutputMode::Lomaji,
            khin_mode: KhinMode::Hyphen,
            key_config: KeyConfig {
                t2: 's',
                t3: 'f',
                t5: 'l',
                t6: 'x',
                t7: 'j',
                t8: 'j',
                t9: 'w',
                khin: 'v',
                hyphen: 'd',
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

    pub fn output_mode(&self) -> OutputMode {
        self.output_mode
    }

    pub fn khin_mode(&self) -> KhinMode {
        self.khin_mode
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

    pub fn hyphen(&self) -> char {
        self.key_config.hyphen
    }

    pub fn done(&self) -> char {
        self.key_config.done
    }

    pub fn is_reserved_char(&self, ch: char) -> bool {
        if ch == self.key_config.khin {
            true
        } else if ch == self.key_config.hyphen {
            true
        } else if ch == self.key_config.done {
            true
        } else if ch == self.t2() {
            true
        } else if ch == self.t3() {
            true
        } else if ch == self.t5() {
            true
        } else if ch == self.t6() {
            true
        } else if ch == self.t7() {
            true
        } else if ch == self.t8() {
            true
        } else if ch == self.t9() {
            true
        } else {
            false
        }
    }

    pub fn is_tone_char(&self, ch: char) -> bool {
        if ch == self.t2() {
            true
        } else if ch == self.t3() {
            true
        } else if ch == self.t5() {
            true
        } else if ch == self.t6(){
            true
        } else if ch == self.t7() {
            true
        } else if ch == self.t8() {
            true
        } else if ch == self.t9() {
            true
        } else {
            false
        }
    }

    pub fn is_hanji_first(&self) -> bool {
        self.output_mode == OutputMode::Hanji
    }

    pub fn is_lomaji_first(&self) -> bool {
        self.output_mode == OutputMode::Lomaji
    }

    pub fn is_khinless(&self) -> bool {
        self.khin_mode == KhinMode::Khinless
    }

    // set input_mode
    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }

    // set tone_mode
    pub fn set_tone_mode(&mut self, mode: ToneMode) {
        self.tone_mode = mode;
    }

    // set output_mode
    pub fn set_output_mode(&mut self, mode: OutputMode) {
        self.output_mode = mode;
    }

    // set khin_mode
    pub fn set_khin_mode(&mut self, mode: KhinMode) {
        self.khin_mode = mode;
    }

    // set key_config
    pub fn set_t2_key(&mut self, t2: char) {
        self.key_config.t2 = t2;
    }

    pub fn set_t3_key(&mut self, t3: char) {
        self.key_config.t3 = t3;
    }

    pub fn set_t5_key(&mut self, t5: char) {
        self.key_config.t5 = t5;
    }

    pub fn set_t6_key(&mut self, t6: char) {
        self.key_config.t6 = t6;
    }

    pub fn set_t7_key(&mut self, t7: char) {
        self.key_config.t7 = t7;
    }

    pub fn set_t8_key(&mut self, t8: char) {
        self.key_config.t8 = t8;
    }

    pub fn set_t9_key(&mut self, t9: char) {
        self.key_config.t9 = t9;
    }

    pub fn set_khin_key(&mut self, khin: char) {
        self.key_config.khin = khin;
    }

    pub fn set_hyphen_key(&mut self, hyphen: char) {
        self.key_config.hyphen = hyphen;
    }

    pub fn set_done_key(&mut self, done: char) {
        self.key_config.done = done;
    }
}
