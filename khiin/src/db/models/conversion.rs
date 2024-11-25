#[derive(Default)]
pub struct Conversion {
    pub input_id: i64,
    pub output: String,
    pub weight: i64,
    pub annotation: Option<String>,
    pub category: Option<i64>,
    pub is_hanji: bool,
}
