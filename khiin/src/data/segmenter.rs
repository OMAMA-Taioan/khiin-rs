use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Result;
use bit_vec::BitVec;

use super::models::KeySequence;
use super::trie::Trie;

static BIG: f64 = 1e10;

pub struct Segmenter {
    max_word_length: usize,
    cost_map: HashMap<String, f64>,
}

impl Segmenter {
    pub fn new(words_by_frequency: Vec<KeySequence>) -> Result<Self> {
        let mut max_word_length = 0;
        let mut cost_map = HashMap::new();

        for word in words_by_frequency.into_iter() {
            if cost_map.contains_key(&word.key_sequence) {
                continue;
            }

            let word_len = word.key_sequence.chars().count();
            max_word_length = std::cmp::max(max_word_length, word_len);

            let p = if word.p <= 0.0 {
                1e-5 / 10f64.powf(word_len as f64)
            } else {
                word.p
            };

            let cost = (1.0 / p).ln();
            cost_map.insert(word.key_sequence, cost);
        }

        Ok(Segmenter {
            max_word_length,
            cost_map,
        })
    }

    pub fn segment(&self, input: &str) -> Result<Vec<String>> {
        Ok(segment_min_cost(
            input,
            &self.cost_map,
            self.max_word_length,
        ))
    }

    pub fn can_segment<T>(is_word: T, query: &str) -> bool
    where
        T: Fn(&str) -> bool,
    {
        let splits = lsegment(is_word, query);
        splits.get(query.chars().count() - 1).unwrap_or(false)
    }

    pub fn can_segment_max<T>(is_word: T, query: &str) -> usize
    where
        T: Fn(&str) -> bool,
    {
        let splits = lsegment(is_word, query);

        for (i, b) in splits.iter().rev().enumerate() {
            if b {
                return splits.len() - i;
            }
        }

        0
    }
}

/// A dynamic programming algorithm to test if a query string can be split into
/// words.
///
/// - The outer loop iterates the string from left to right, where `i` marks the
///   end index of substrings to be tested during the inner loop.
/// - The inner loop iterates start indices using the `split_indices` array,
///   which contains the index of the last char of each word found.
///
/// In the initial case, the `split_index` is given as -1, which can be thought
/// of as the end of the "previous" word. When a word is found, the index of the
/// last char in the word is added to `split_indices`, so that for each `j` in
/// split_indices`, `j + 1` marks the starting index of substrings to test.
fn lsegment<T>(is_word: T, query: &str) -> BitVec
where
    T: Fn(&str) -> bool,
{
    let size = query.chars().count();

    let mut splits_at = BitVec::from_elem(size + 1, false);
    let mut split_indices = vec![-1];

    for i in 0..size {
        for j in split_indices.iter() {
            let start = (j + 1) as usize;
            let end = i + 1;
            let substr = &query[start..end];

            // println!("Checking substr: {}", substr);
            if is_word(&substr) {
                // println!("Ok!");
                splits_at.set(i, true);
                split_indices.push(i as i32);
                break;
            }
        }
    }

    splits_at
}

/// A dynamic programming algorithm to split a string based on a simple
/// least-cost model. This functions in nearly the same way as `can_split`, but
/// rather than simply detecting whether or not a split is possible, it
/// associates a cost with each word found and chooses the split pattern with
/// the minimum cost as the result.
///
/// It would be good to experiment with different models for calculating the
/// cost, or to improve our corpus for better results. The current model uses:
///
/// ```
/// cost =  ln (1 / 𝓟)
/// ```
/// where `𝓟` is the number of occurrences of a word in the corpus divided by
/// the total number of words in the corpus. This seems to give decent results.
fn segment_min_cost(
    input: &str,
    cost_map: &HashMap<String, f64>,
    max_word_len: usize,
) -> Vec<String> {
    let len = input.chars().count();
    let mut costs: Vec<(f64, i32)> = Vec::new();
    costs.push((0.0, -1));

    #[allow(unused_assignments)]
    let mut curr_cost = 0.0f64;

    for i in 1..len + 1 {
        let mut min_cost = costs[i - 1].0 + BIG;
        let mut min_cost_idx = i - 1;

        for j in i.saturating_sub(max_word_len)..i {
            let chunk = &input[j..i];

            // println!("chunk: {}", chunk);

            if !cost_map.contains_key(chunk) {
                continue;
            }

            // println!("chunk cost: {}", cost_map.get(chunk).unwrap());

            curr_cost = costs[j].0 + cost_map.get(chunk).unwrap();
            if curr_cost <= min_cost {
                min_cost = curr_cost;
                min_cost_idx = j;
            }
        }

        costs.push((min_cost, min_cost_idx as i32));
    }

    let mut result: Vec<String> = Vec::new();
    let mut k = len;

    while k > 0 {
        let pre_index = costs[k].1 as usize;
        let insert_str = &input[pre_index..k];

        let tmp = if !result.is_empty() {
            insert_str.to_string() + &result[0]
        } else {
            String::new()
        };

        if !tmp.is_empty() && tmp.chars().all(|c| c.is_ascii_digit()) {
            result[0] = tmp;
        } else {
            result.insert(0, insert_str.to_string());
        }

        k = pre_index;
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::collection;

    use super::Segmenter as S;
    use super::*;

    #[test]
    fn it_works() {
        let input = "goabehchiahpng";
        let cost_map: HashMap<String, f64> = collection!(
            "goa".into() => 10.0,
            "beh".into() => 20.0,
            "chiah".into() => 30.0,
            "png".into() => 40.0,
        );
        let max_word_len = 5;
        let result = segment_min_cost(input, &cost_map, max_word_len);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn it_splits_using_a_word_list() {
        let words = vec![
            "goa2",
            "goa",
            "m7chai",
            "mchai",
            "joache",
            "joa7che7",
            "lang5",
            "lang",
            "ham5",
            "ham",
            "u7",
            "u",
            "kangkhoan2",
            "kangkhoan",
            "e",
            "seng",
            "tiong",
            "li2",
            "li",
            "ho2",
            "ho",
            "la",
            "toa7",
            "toa",
            "toa7lang5",
            "toalang",
            "to",
            "a",
            "ng",
        ]
        .iter()
        .map(|s| KeySequence {
            id: 0,
            key_sequence: s.to_string(),
            p: 0.01,
        })
        .collect();
        let segmenter =
            Segmenter::new(words).expect("Could not build segmenter");
        let result = segmenter
            .segment("goamchaiujoachelanghamgoaukangkhoanesengtiong")
            .expect("Could not segment text");
        println!("{}", result.join(" "));
        assert_eq!(result.len(), 12);
        let result = segmenter
            .segment("goa2mchaiu7joa7che7lang5ham5goa2ukangkhoan2esengtiong")
            .expect("Could not segment text");
        println!("{}", result.join(" "));
        assert_eq!(result.len(), 12);
    }

    #[test]
    fn it_finds_segmentation_indices() {
        let v = vec!["hello", "world"];
        let is_word = |s: &str| v.contains(&s);
        let result = lsegment(is_word, "helloworld");
        assert_eq!(result.get(4).unwrap(), true);
        assert_eq!(result.get(9).unwrap(), true);

        assert_eq!(S::can_segment(is_word, "helloworld"), true);
        assert_eq!(S::can_segment(is_word, "helloworldo"), false);

        assert_eq!(S::can_segment_max(is_word, "helloworld"), 10);
        assert_eq!(S::can_segment_max(is_word, "helloworldo"), 10);
        assert_eq!(S::can_segment_max(is_word, "helloworldhello"), 15);
    }
}
