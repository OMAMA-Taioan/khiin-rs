#[derive(Default, Debug, Clone)]
pub struct Input {
    pub id: i64,
    pub input: String,
    pub corpus_count: i64,
    pub chhan_id: i64,
    pub n_syls: usize,
    pub p: f64,
}
