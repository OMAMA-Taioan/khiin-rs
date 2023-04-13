pub struct Conversion {
    pub key_sequence: String,
    pub input: String,
    pub input_id: i32,
    pub output: String,
    pub weight: i32,
    pub category: Option<i32>,
    pub annotation: Option<String>,
}
