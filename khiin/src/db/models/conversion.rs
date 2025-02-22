#[derive(Default)]
pub struct Conversion {
    pub input_id: i64,
    pub output: String,
    pub weight: i64,
    pub annotation: Option<String>,
    pub khin_ok: bool,
    pub khinless_ok: bool,
    pub is_hanji: bool,
}
