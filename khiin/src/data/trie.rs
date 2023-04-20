use std::collections::HashSet;

use anyhow::Result;
use qp_trie::wrapper::BString;
use qp_trie::Trie as QpTrie;

use crate::data::models::KeySequence;

pub(crate) struct Trie {
    qp_trie: QpTrie<BString, Vec<u32>>,
}

impl Trie {
    pub fn new(inputs: &Vec<KeySequence>) -> Result<Self> {
        let mut qp_trie: QpTrie<BString, Vec<u32>> = QpTrie::new();

        for word in inputs.iter() {
            if let Some(ids) = qp_trie.get_mut_str(&word.key_sequence) {
                ids.push(word.id);
            } else {
                let v = vec![word.id];
                qp_trie.insert_str(&word.key_sequence, v);
            }
        }

        Ok(Self { qp_trie })
    }

    pub fn find_words_by_prefix(&self, query: &str) -> Vec<u32> {
        let mut result = HashSet::new();
        for (_, vec) in self.qp_trie.iter_prefix_str(query) {
            for v in vec {
                result.insert(v.clone());
            }
        }
        let mut v: Vec<u32> = result.iter().map(|&ea| ea).collect();
        v.sort_unstable();
        v
    }

    pub fn find_words_from_start<'a>(&self, query: &'a str) -> Vec<&'a str> {
        self.qp_trie.get_keys_str(query)
    }

    pub fn contains(&self, query: &str) -> bool {
        self.qp_trie.contains_key_str(query)
    }
}

trait Walker<'a> {
    fn get_keys_str(&self, query: &'a str) -> Vec<&'a str>;
}

impl<'a> Walker<'a> for QpTrie<BString, Vec<u32>> {
    fn get_keys_str(&self, query: &'a str) -> Vec<&'a str> {
        let mut found: Vec<&str> = Vec::new();

        for (i, _) in query.char_indices() {
            let key = &query[0..(i + 1)];
            let st = self.subtrie_str(key);

            if st.is_empty() {
                break;
            }

            if self.contains_key_str(key) {
                found.push(key);
            }
        }

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_trie(words: Vec<&str>) -> Trie {
        let ks = words
            .into_iter()
            .enumerate()
            .map(|(i, w)| KeySequence {
                id: (i + 1) as u32,
                key_sequence: w.to_string(),
                n_syls: 1,
                p: 0.0,
            })
            .collect();
        Trie::new(&ks).unwrap()
    }

    #[test]
    fn it_gets_contained_keys() {
        let t = get_trie(vec!["ball", "tomato", "balloon", "balloonanimal"]);
        let res = t.qp_trie.get_keys_str("balloonanimal");
        assert_eq!(res.len(), 3);
        assert_eq!(res[0], "ball");
        assert_eq!(res[1], "balloon");
        assert_eq!(res[2], "balloonanimal");
    }
}
