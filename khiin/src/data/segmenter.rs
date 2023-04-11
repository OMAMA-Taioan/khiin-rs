use std::collections::HashMap;

use anyhow::Result;
use qp_trie::{wrapper::BString, Trie};

use super::database::Input;

static BIG: f64 = 1e10;

pub struct Segmenter {
    max_word_length: usize,
    cost_map: HashMap<String, f64>,
}

impl Segmenter {
    pub fn new(words_by_frequency: Vec<Input>) -> Result<Self> {
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

    pub fn split(&self, input: &str) -> Result<Vec<String>> {
        Ok(split_words(input, &self.cost_map, self.max_word_length))
    }
}

fn split_words(
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
        let result = split_words(input, &cost_map, max_word_len);
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
        .map(|s| Input {
            id: 0,
            key_sequence: s.to_string(),
            p: 0.01,
        })
        .collect();
        let segmenter =
            Segmenter::new(words).expect("Could not build segmenter");
        let result = segmenter
            .split("goamchaiujoachelanghamgoaukangkhoanesengtiong")
            .expect("Could not segment text");
        println!("{}", result.join(" "));
        assert_eq!(result.len(), 12);
    }
}
