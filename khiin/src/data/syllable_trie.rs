use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

pub(crate) struct SyllableTrie {
    root: TrieNode,
}

impl SyllableTrie {
    pub fn new() -> Self {
        let mut root = TrieNode::new();
        for line in khiin_data::SYLLABLES_TXT.lines() {
            let mut current_node = &mut root;
            for c in line.chars() {
                current_node =
                    current_node.children.entry(c).or_insert(TrieNode::new());
            }
            current_node.is_end_of_word = true;
        }

        SyllableTrie { root: root }
    }

    pub fn is_valid_prefix(&self, prefix: &str) -> bool {
        let mut current_node: &TrieNode = &self.root;
        let mut query = prefix.to_string();
        
        if query.ends_with("n") {
            query.pop();
            // query.push('ⁿ');
        } else if query.ends_with('o') {
            query.pop();
            // query.push_str("o͘");
        }
        for c in query.chars() {
            match current_node.children.get(&c.to_ascii_lowercase()) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        true
    }

    pub fn is_valid_syllable(&self, word: &str) -> bool {
        let mut current_node: &TrieNode = &self.root;
        for c in word.chars() {
            match current_node.children.get(&c.to_ascii_lowercase()) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        return current_node.is_end_of_word
    }
}
