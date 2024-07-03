use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::vec;
use unicode_normalization::UnicodeNormalization;

use crate::collection;
use crate::Tone;

const TONE_CHAR_MAP: Lazy<HashMap<Tone, char>> = Lazy::new(|| {
    collection!(
        Tone::T2 => '\u{0301}',
        Tone::T3 => '\u{0300}',
        Tone::T5 => '\u{0302}',
        Tone::T6 => '\u{030C}',
        Tone::T7 => '\u{0304}',
        Tone::T8 => '\u{030D}',
        Tone::T9 => '\u{0306}',
    )
});

const CHAR_TONE_MAP: Lazy<HashMap<char, Tone>> = Lazy::new(|| {
    collection!(
        '\u{0301}' => Tone::T2,
        '\u{0300}' => Tone::T3,
        '\u{0302}' => Tone::T5,
        '\u{0303}' => Tone::T6,
        '\u{0304}' => Tone::T7,
        '\u{030D}' => Tone::T8,
        '\u{0306}' => Tone::T9,
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
    )
});

const TONE_LETTER_PATTERNS: Lazy<Vec<(Regex, usize)>> = Lazy::new(|| {
    vec![
        (Regex::new("(?i)o[ae][ptkhmn]").unwrap(), 1),
        (Regex::new("(?i)o").unwrap(), 0),
        (Regex::new("(?i)a").unwrap(), 0),
        (Regex::new("(?i)e").unwrap(), 0),
        (Regex::new("(?i)u").unwrap(), 0),
        (Regex::new("(?i)i").unwrap(), 0),
        (Regex::new("(?i)n").unwrap(), 0),
        (Regex::new("(?i)m").unwrap(), 0),
    ]
});

const NUMERIC_TONE_CHARS: [char; 10] =
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const TELEX_TONE_CHARS: [char; 10] =
    ['0', '1', 's', 'f', '4', 'l', '6', 'j', 'j', 'w'];

const T4_SUFFIXES: &[&str] = &["h", "p", "t", "k", "hnn", "h\u{207f}"];

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
    TONE_LETTER_PATTERNS.iter().any(|(pat, _)| pat.is_match(syllable))
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
}
