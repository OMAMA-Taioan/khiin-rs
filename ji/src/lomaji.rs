use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::vec;
use unicode_normalization::UnicodeNormalization;

use crate::collection;
use crate::unicode::*;
use crate::Tone;

const TONE_CHAR_MAP: Lazy<HashMap<Tone, char>> = Lazy::new(|| {
    collection!(
        Tone::T2 => TONE_2,
        Tone::T3 => TONE_3,
        Tone::T5 => TONE_5,
        Tone::T6 => TONE_6,
        Tone::T7 => TONE_7,
        Tone::T8 => TONE_8,
        Tone::T9 => TONE_9,
    )
});

const CHAR_TONE_MAP: Lazy<HashMap<char, Tone>> = Lazy::new(|| {
    collection!(
        TONE_2 => Tone::T2,
        TONE_3 => Tone::T3,
        TONE_5 => Tone::T5,
        TONE_6 => Tone::T6,
        TONE_7 => Tone::T7,
        TONE_8 => Tone::T8,
        TONE_9 => Tone::T9,
    )
});

const DIGIT_TONE_MAP: Lazy<HashMap<char, Tone>> = Lazy::new(|| {
    collection!(
        '1' => Tone::T1,
        '2' => Tone::T2,
        '3' => Tone::T3,
        '4' => Tone::T4,
        '5' => Tone::T5,
        '6' => Tone::T6,
        '7' => Tone::T7,
        '8' => Tone::T8,
        '9' => Tone::T9,
    )
});

const POJ_INPUT_MAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    collection!(
        "o\u{0358}" => "ou",
        "O\u{0358}" => "Ou",
        "\u{207f}" => "nn",
        "\u{1d3a}" => "NN",
        "o\u{0324}" => "eo",
        "u\u{0324}" => "eu",
    )
});

const POJ_INPUT_MAP_OO: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    collection!(
        "o\u{0358}" => "oo",
        "O\u{0358}" => "Oo",
        "\u{207f}" => "nn",
        "\u{1d3a}" => "NN",
        "o\u{0324}" => "eo",
        "u\u{0324}" => "eu",
    )
});

const TONE_LETTER_PATTERNS: Lazy<Vec<(Regex, usize)>> = Lazy::new(|| {
    vec![
        (Regex::new("(?i)o[ae][ptkhmni]").unwrap(), 1),
        (Regex::new("(?i)o").unwrap(), 0),
        (Regex::new("(?i)a").unwrap(), 0),
        (Regex::new("(?i)e").unwrap(), 0),
        (Regex::new("(?i)u").unwrap(), 0),
        (Regex::new("(?i)i").unwrap(), 0),
        (Regex::new("(?i)ng").unwrap(), 0),
        (Regex::new("(?i)n").unwrap(), 0),
        (Regex::new("(?i)m").unwrap(), 0),
    ]
});

const NUMERIC_TONE_CHARS: [char; 10] =
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const TELEX_TONE_CHARS: [char; 10] =
    ['0', '1', 's', 'f', '4', 'l', '6', 'j', 'j', 'w'];

const T4_SUFFIXES: &[&str] = &["h", "p", "t", "k", "hnn", "h\u{207f}", "h\u{1d3a}"];

lazy_static! {
    static ref RE_LEGAL_LOMAJI: Regex= Regex::new(r"^\u{00b7}?((chh|[ckpt]h|[bhgjklmnpst])?(iau|io\u{0358}|oai|a[iu]|i[aou]|o[ae\u{0358}]|ui|[aeiou])?(ng|[mnptkh])?|(chh|[ckpt]h|[hkmnpst])ng?|(ng|m)h?)(n|\u{207f}|\u{1d3a})?$").unwrap();
}

pub fn is_legal_lomaji(str: &str) -> bool {
    let mut stripped = String::new();

    for ch in str.to_lowercase().nfd() {
        if !CHAR_TONE_MAP.contains_key(&ch) && ch != '-' && ch != '\u{207f}' && ch != '\u{1d3a}' {
            stripped.push(ch)
        }
    }
    return RE_LEGAL_LOMAJI.is_match(&stripped)
}

pub fn tone_to_char(tone: &Tone) -> Option<char> {
    TONE_CHAR_MAP.get(tone).map(|&c| c)
}

pub fn key_to_tone(ch: char) -> Tone {
    *DIGIT_TONE_MAP.get(&ch).unwrap_or(&Tone::None)
}

pub fn get_tone_position(syllable: &str) -> Option<usize> {
    for (pat, offset) in TONE_LETTER_PATTERNS.iter() {
        if let Some(mat) = pat.find(syllable) {
            return Some(mat.start() + offset);
        }
    }

    None
}

pub fn has_tone_letter(syllable: &str) -> bool {
    TONE_LETTER_PATTERNS
        .iter()
        .any(|(pat, _)| pat.is_match(syllable))
}

pub fn tone_char_to_index(ch: char) -> Option<usize> {
    NUMERIC_TONE_CHARS.iter().position(|&c| c == ch)
}

pub fn strip_tone_diacritic(syl: &str) -> (String, Tone) {
    let mut stripped = String::new();
    let mut tone = Tone::None;

    for ch in syl.nfd() {
        if CHAR_TONE_MAP.contains_key(&ch) {
            tone = *CHAR_TONE_MAP.get(&ch).unwrap();
        } else {
            stripped.push(ch)
        }
    }

    if tone == Tone::None {
        let stripped_lc = stripped.to_lowercase();
        tone = if T4_SUFFIXES.iter().any(|suf| stripped_lc.ends_with(*suf)) {
            Tone::T4
        } else {
            Tone::T1
        }
    }

    (stripped, tone)
}

pub fn poj_syl_to_key_sequences(syl: &str) -> (String, String, String) {
    let (stripped, tone) = strip_tone_diacritic(syl);

    let stripped = POJ_INPUT_MAP
        .iter()
        .fold(stripped, |agg, (pat, repl)| agg.replace(pat, repl));

    let mut numeric = stripped.clone();
    numeric.push(NUMERIC_TONE_CHARS[tone as i32 as usize]);
    let mut telex = stripped.clone();
    telex.push(TELEX_TONE_CHARS[tone as i32 as usize]);

    return (numeric, telex, stripped);
}

pub fn poj_syl_to_key_sequences_oo(syl: &str) -> (String, String, String) {
    let (stripped, tone) = strip_tone_diacritic(syl);

    let stripped = POJ_INPUT_MAP_OO
        .iter()
        .fold(stripped, |agg, (pat, repl)| agg.replace(pat, repl));

    let mut numeric = stripped.clone();
    numeric.push(NUMERIC_TONE_CHARS[tone as i32 as usize]);
    let mut telex = stripped.clone();
    telex.push(TELEX_TONE_CHARS[tone as i32 as usize]);

    return (numeric, telex, stripped);
}

pub fn syllable_to_key_sequences(syl: &str) -> Vec<String> {
    let (stripped, _tone) = strip_tone_diacritic(syl);
    let mut ret = vec![];
    let mut dots_stripped = vec![];
    if stripped.ends_with("ⁿ") {
        let new_stripped = stripped.replace("ⁿ", "");
        if let Some(vowel_pos) = new_stripped.find(|c: char| "aeiouo͘o̤ṳ".contains(c)) {
            let mut i = vowel_pos + 1;
            while i <= new_stripped.len() {
                let suffix = &new_stripped[i..];
                if suffix.starts_with(DOTS_BELOW) || suffix.starts_with(DOT_ABOVE_RIGHT){
                    i += 2;
                    continue;
                }
                let mut new_str = new_stripped[..i].to_string();
                new_str.push_str("nn");
                new_str.push_str(suffix);
                if new_str.contains(DOTS_BELOW) || new_str.contains(DOT_ABOVE_RIGHT) {
                    dots_stripped.push(new_str);
                } else {
                    ret.push(new_str);
                }
                i += 1;
            }
        }
    } else if stripped.contains(DOTS_BELOW) || stripped.contains(DOT_ABOVE_RIGHT) {
        dots_stripped.push(stripped.clone());
    } else {
        ret.push(stripped.clone());
    }
    while !dots_stripped.is_empty() {
        // take str and find first o͘
        let str: String = dots_stripped.pop().unwrap().replace("ṳ", "eu").replace("o̤", "eo");
        if str.contains("o͘") {
            // replace o͘ with oo & ou
            let oo_str = str.replace("o͘", "oo");
            let ou_str = str.replace("o͘", "ou");
            ret.push(oo_str);
            ret.push(ou_str);
        } else {
            ret.push(str);
        }
    }
    ret
}

pub fn strip_khin(syl: &mut String) -> bool {
    if syl.starts_with("--") {
        syl.drain(0..2);
        true
    } else if syl.starts_with("·") {
        syl.remove(0);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_tone_positions() {
        assert_eq!(get_tone_position("a").unwrap(), 0);
        assert_eq!(get_tone_position("choan").unwrap(), 3);
        assert_eq!(get_tone_position("goeh").unwrap(), 2);
        assert_eq!(get_tone_position("khou").unwrap(), 2);
        assert_eq!(get_tone_position("beh").unwrap(), 1);
        assert_eq!(get_tone_position("phainn").unwrap(), 2);
        assert_eq!(get_tone_position("khoai").unwrap(), 3);
        assert_eq!(get_tone_position("xyz"), None);
    }

    #[test]
    fn it_strips_diacritics() {
        assert_eq!(strip_tone_diacritic("hó"), ("ho".to_owned(), Tone::T2));
        assert_eq!(strip_tone_diacritic("hó͘"), ("ho͘".to_owned(), Tone::T2));
    }

    #[test]
    fn it_strips_khin() {
        let mut s = "--ho".to_string();
        let stripped = strip_khin(&mut s);
        assert!(stripped);
        assert_eq!(s.as_str(), "ho");

        let mut s = "·ho".to_string();
        let stripped = strip_khin(&mut s);
        assert!(stripped);
        assert_eq!(s.as_str(), "ho");
    }

    #[test]
    fn it_syllable_to_key_sequences() {
        assert_eq!(syllable_to_key_sequences("no͘").len(), 2);
        assert_eq!(syllable_to_key_sequences("tṳiⁿ").len(), 2);
        assert_eq!(syllable_to_key_sequences("peⁿ").len(), 1);        
        assert_eq!(syllable_to_key_sequences("cho͘hⁿ").len(), 4);
        assert_eq!(syllable_to_key_sequences("chho͘ⁿ").len(), 2);
    }
}
