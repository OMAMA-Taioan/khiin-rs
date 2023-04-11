#[derive(Copy, Clone)]
pub enum InputMode {
    Continuous,
    SingleWord,
    Manual,
}

#[derive(Copy, Clone)]
pub enum InputType {
    Numeric,
    Telex,
}

pub struct EngineCfg {
    enabled: bool,
    input_mode: InputMode,
    input_type: InputType,
}


impl EngineCfg {
    pub fn new() -> Self {
        Self {
            enabled: false,
            input_mode: InputMode::Continuous,
            input_type: InputType::Numeric,
        }
    }

    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }
}
