use std::collections::HashMap;

use anyhow::Result;
use bit_vec::BitVec;

use crate::db::models::KeySequence;

/// A number 0.0 or greater. If set to 0.0, all words will be treated equally
/// regardless of frequency. The higher the number, the more heavily weighted
/// the frequency will be. The default (unbiased) weight is 1.0.
const FREQUENCY_BIAS: f64 = 1.0;

/// A number between 0.0 and 1.0, where higher numbers bias towards keeping
/// longer words un-split, and lower numbers bias towards following the rankings
/// provided in the frequency database.
///
/// A value of 1.0 will weigh any longer word higher than any shorter word. A
/// value of 0.0 will not bias the results at all, and will use only the
/// frequency index in the database to decide whether or not to split.
const LETTER_COUNT_BIAS: f64 = 0.2;

/// A number between 0.0 and 1.0, where higher numbers bias towards splitting
/// fewer syllables, and lower numbers bias towards following the rankings
/// provided in the frequency database.
///
/// For example, "hoan" could be split as 2 syllables "ho" and "an", or 1
/// syllable "hoan". A higher number would be more likely to use "hoan".
const SYLLABLE_COUNT_BIAS: f64 = 0.2;

const BIG: f64 = 1e10;

pub struct Segmenter {
    max_word_length: usize,
    cost_map: HashMap<String, f64>,
}

fn min_max(map: &HashMap<String, f64>) -> Option<(f64, f64)> {
    let mut min: Option<f64> = None;
    let mut max: Option<f64> = None;

    for &value in map.values() {
        if let Some(current_min) = min {
            if let Some(cmp) = current_min.partial_cmp(&value) {
                if cmp == std::cmp::Ordering::Greater {
                    min = Some(value);
                }
            }
        } else {
            min = Some(value);
        }

        if let Some(current_max) = max {
            if let Some(cmp) = current_max.partial_cmp(&value) {
                if cmp == std::cmp::Ordering::Less {
                    max = Some(value);
                }
            }
        } else {
            max = Some(value);
        }
    }

    match (min, max) {
        (Some(min_value), Some(max_value)) => Some((min_value, max_value)),
        _ => None,
    }
}

impl Segmenter {
    pub fn new(words_by_frequency: Vec<KeySequence>) -> Result<Self> {
        let mut max_word_length = 0;
        let mut cost_map = HashMap::new();

        for word in words_by_frequency.into_iter() {
            if cost_map.contains_key(&word.keys) {
                continue;
            }

            let word_len = word.keys.chars().count();
            max_word_length = std::cmp::max(max_word_length, word_len);

            let p = if word.p <= 0.0 {
                1e-5 / 10f64.powf(word_len as f64)
            } else {
                word.p
            };

            // Apply the cost biases
            let mut cost = (1.0 / p.powf(FREQUENCY_BIAS)).ln();
            let bias = (word_len as f64).powf(LETTER_COUNT_BIAS);
            let syl_bias = (word.n_syls as f64).powf(SYLLABLE_COUNT_BIAS);
            cost = cost / bias * syl_bias;
            cost_map.insert(word.keys, cost);
        }

        if let Some((min, max)) = min_max(&cost_map) {
            log::debug!("min {}, max {}", min, max);
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

    /// Returns the number of chars that can be segmented in the query string
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

            log::trace!("Checking substr: {}", substr);
            if is_word(&substr) {
                log::trace!("Ok!");
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
/// COST =  ln (1 / 𝓟)
///
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

            log::debug!("chunk: {}", chunk);

            if !cost_map.contains_key(chunk) {
                continue;
            }

            log::debug!("chunk cost: {}", cost_map.get(chunk).unwrap());

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

    use crate::db::models::InputType;

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
            ("goa2", 1),
            ("goa", 1),
            ("m7chai", 2),
            ("mchai", 2),
            ("joache", 2),
            ("joa7che7", 2),
            ("lang5", 1),
            ("lang", 1),
            ("ham5", 1),
            ("ham", 1),
            ("u7", 1),
            ("u", 1),
            ("kangkhoan2", 2),
            ("kangkhoan", 2),
            ("e", 1),
            ("seng", 1),
            ("tiong", 1),
            ("li2", 1),
            ("li", 1),
            ("ho2", 1),
            ("ho", 1),
            ("la", 1),
            ("toa7", 1),
            ("toa", 1),
            ("toa7lang5", 2),
            ("toalang", 2),
            ("to", 1),
            ("a", 1),
            ("ng", 1),
        ]
        .iter()
        .map(|(keys, syls)| KeySequence {
            input_id: 0,
            keys: keys.to_string(),
            input_type: InputType::Numeric,
            n_syls: *syls,
            p: 0.01,
        })
        .collect();
        let segmenter =
            Segmenter::new(words).expect("Could not build segmenter");
        let result = segmenter
            .segment("goamchaiujoachelanghamgoaukangkhoanesengtiong")
            .expect("Could not segment text");
        log::debug!("{}", result.join(" "));
        assert_eq!(result.len(), 12);
        let result = segmenter
            .segment("goa2mchaiu7joa7che7lang5ham5goa2ukangkhoan2esengtiong")
            .expect("Could not segment text");
        log::debug!("{}", result.join(" "));
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
