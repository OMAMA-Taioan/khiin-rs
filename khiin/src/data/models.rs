#[derive(Debug, Clone)]
pub struct Conversion {
    pub key_sequence: String,
    pub input: String,
    pub input_id: u32,
    pub output: String,
    pub weight: i32,
    pub category: Option<i32>,
    pub annotation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KeySequence {
    pub id: u32,
    pub key_sequence: String,
    pub p: f64,
}
