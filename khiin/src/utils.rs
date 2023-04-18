use std::collections::HashSet;
use std::hash::Hash;

pub(crate) trait CharSubstr {
    fn char_substr(
        &self,
        start_char_index: usize,
        end_char_index: usize,
    ) -> Self;
}

impl CharSubstr for &str {
    fn char_substr(
        &self,
        start_char_index: usize,
        end_char_index: usize,
    ) -> Self {
        let mut char_count = 0;
        let mut start_byte_index = None;
        let mut end_byte_index = None;

        for (i, _) in self.char_indices() {
            if char_count == start_char_index {
                start_byte_index = Some(i);
            }
            if char_count == end_char_index {
                end_byte_index = Some(i);
                break;
            }
            char_count += 1;
        }

        if let Some(start) = start_byte_index {
            if let Some(end) = end_byte_index {
                return &self[start..end];
            }
        }

        return "";
    }
}

impl CharSubstr for String {
    fn char_substr(
        &self,
        start_char_index: usize,
        end_char_index: usize,
    ) -> Self {
        let str = self.as_str();
        str.char_substr(start_char_index, end_char_index).to_owned()
    }
}

pub trait Unique<T> {
    fn all_unique(&self) -> bool;
}

impl<T: Eq + Hash> Unique<T> for Vec<T> {
    fn all_unique(&self) -> bool {
        let set: HashSet<_> = self.iter().collect();
        self.len() == set.len()
    }
}
