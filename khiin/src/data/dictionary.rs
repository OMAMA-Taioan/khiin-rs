use std::collections::HashSet;

use anyhow::Result;
use qp_trie::wrapper::BString;
use qp_trie::Trie;

use crate::config::engine_cfg::InputType;

use super::database::Database;

pub struct Dictionary {
    word_trie: Trie<BString, u32>,
}

impl Dictionary {
    pub fn new(db: &Database, input_type: InputType) -> Result<Self> {
        let words = db.all_words_by_freq(input_type)?;

        let mut trie = Trie::new();
        for word in words.iter() {
            trie.insert_str(&word.key_sequence, word.id);
        }

        Ok(Self { word_trie: trie })
    }

    pub fn find_words_by_prefix(&self, query: &str) -> Vec<u32> {
        let mut result = HashSet::new();
        let bstr = BString::from(query);
        for (_, v) in self.word_trie.iter_prefix(&bstr) {
            result.insert(v.clone());
        }
        let mut v: Vec<u32> = result.iter().map(|&ea| ea).collect();
        v.sort_unstable();
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::get_db;

    use super::*;

    fn setup() -> Dictionary {
        let db = get_db();
        Dictionary::new(&db, InputType::Numeric).unwrap()
    }

    #[test]
    fn it_loads() {
        let db = get_db();
        let dict = Dictionary::new(&db, InputType::Numeric);
        assert!(dict.is_ok());
    }

    #[test]
    fn it_finds_words_by_prefix() {
        let dict = setup();
        let ids = dict.find_words_by_prefix("goa");
        assert!(ids.len() > 0);
        let ids = dict.find_words_by_prefix("e");
        assert!(ids.len() > 0);
        let ids = dict.find_words_by_prefix("si");
        assert!(ids.len() > 0);
        let ids2 = dict.find_words_by_prefix("k");
        assert!(ids2.len() > 0);
        let ids = dict.find_words_by_prefix("chh");
        assert!(ids.len() > 0);
        let ids = dict.find_words_by_prefix("a");
        assert!(ids.len() > 0);
    }
}
