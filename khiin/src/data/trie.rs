use std::collections::HashSet;

use anyhow::Result;
use qp_trie::wrapper::BString;
use qp_trie::Trie as QpTrie;

use crate::db::models::KeySequence;

type WordTrie = QpTrie<BString, Vec<i64>>;

pub(crate) struct Trie {
    qp_trie: WordTrie,
}

impl Trie {
    pub fn new(inputs: &Vec<KeySequence>) -> Result<Self> {
        let mut qp_trie: WordTrie = QpTrie::new();

        for word in inputs.iter() {
            if let Some(ids) = qp_trie.get_mut_str(&word.keys) {
                ids.push(word.input_id);
            } else {
                let v = vec![word.input_id];
                qp_trie.insert_str(&word.keys, v);
            }
        }

        Ok(Self { qp_trie })
    }

    pub fn find_words_by_prefix(&self, query: &str) -> Vec<i64> {
        let mut result = HashSet::new();
        for (_, vec) in self.qp_trie.iter_prefix_str(query) {
            for v in vec {
                result.insert(v.clone());
            }
        }
        let mut v: Vec<i64> = result.iter().map(|&ea| ea).collect();
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

impl<'a> Walker<'a> for WordTrie {
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
    use crate::db::models::InputType;

    use super::*;

    fn get_trie(words: Vec<&str>) -> Trie {
        let ks = words
            .into_iter()
            .enumerate()
            .map(|(i, w)| KeySequence {
                input_id: (i + 1) as i64,
                keys: w.to_string(),
                input_type: InputType::Numeric,
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
