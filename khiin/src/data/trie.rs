use std::collections::HashSet;

use anyhow::Result;
use qp_trie::wrapper::BString;
use qp_trie::Trie as QpTrie;

use super::models::KeySequence;

pub struct Trie {
    qp_trie: QpTrie<BString, u32>,
}

impl Trie {
    pub fn new(inputs: &Vec<KeySequence>) -> Result<Self> {
        let mut qp_trie = QpTrie::new();

        for word in inputs.iter() {
            qp_trie.insert_str(&word.key_sequence, word.id);
        }

        Ok(Self { qp_trie })
    }

    pub fn find_words_by_prefix(&self, query: &str) -> Vec<u32> {
        let mut result = HashSet::new();
        for (_, v) in self.qp_trie.iter_prefix_str(query) {
            result.insert(v.clone());
        }
        let mut v: Vec<u32> = result.iter().map(|&ea| ea).collect();
        v.sort_unstable();
        v
    }

    pub fn contains(&self, query: &str) -> bool {
        self.qp_trie.contains_key_str(query)
    }
}
