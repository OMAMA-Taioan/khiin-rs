use std::collections::HashMap;

use super::Input;

type InputIdMap = HashMap<String, i64>;
type IdInputMap = HashMap<i64, Input>;

#[derive(Default)]
pub struct InputLookup {
    input_id_map: InputIdMap,
    id_input_map: IdInputMap,
}

impl InputLookup {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, row: &Input) -> Option<i64> {
        self.id_input_map.insert(row.id, row.clone());
        self.input_id_map.insert(row.input.clone(), row.id)
    }

    pub fn contains_input(&self, input: &str) -> bool {
        self.input_id_map.contains_key(input)
    }

    pub fn id_of(&self, input: &str) -> Option<i64> {
        self.input_id_map.get(input).copied()
    }

    pub fn input_of(&self, id: i64) -> Option<&Input> {
        self.id_input_map.get(&id).map(|x| &*x)
    }
}
