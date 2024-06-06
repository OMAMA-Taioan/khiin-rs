use crate::db::models::InputType;

#[derive(Copy, Clone)]
pub enum InputMode {
    Continuous,
    SingleWord,
    Manual,
}

#[derive(Copy, Clone)]
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

pub struct Config {
    enabled: bool,
    input_mode: InputMode,
    tone_mode: ToneMode,
}

impl Config {
    pub fn new() -> Self {
        Self {
            enabled: false,
            input_mode: InputMode::Continuous,
            tone_mode: ToneMode::Numeric,
        }
    }

    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn tone_mode(&self) -> ToneMode {
        self.tone_mode
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
