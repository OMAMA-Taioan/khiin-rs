use once_cell::sync::Lazy;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;
use std::collections::HashMap;

use crate::collection;

use super::Tone;

static TONE_CHAR_MAP: Lazy<HashMap<Tone, char>> = Lazy::new(|| {
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

static CHAR_TONE_MAP: Lazy<HashMap<char, Tone>> = Lazy::new(|| {
    collection!(
        '\u{0301}' => Tone::T2,
        '\u{0300}' => Tone::T3,
        '\u{0302}' => Tone::T5,
        '\u{030C}' => Tone::T6,
        '\u{0304}' => Tone::T7,
        '\u{030D}' => Tone::T8,
        '\u{0306}' => Tone::T9,
    )
});

static TONE_LETTER_PATTERNS: Lazy<Vec<(Regex, usize)>> = Lazy::new(|| {
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

static NUMERIC_TONE_CHARS: [char; 10] =
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

static TELEX_TONE_CHARS: [char; 10] =
    ['0', '1', 's', 'f', '4', 'l', '6', 'j', 'j', 'w'];

pub fn tone_to_char(tone: &Tone) -> Option<char> {
    TONE_CHAR_MAP.get(tone).map(|&c| c)
}

pub fn get_tone_position(syllable: &str) -> Option<usize> {
    for (pat, offset) in TONE_LETTER_PATTERNS.iter() {
        if let Some(mat) = pat.find(syllable) {
            return Some(mat.start() + offset);
        }
    }

    None
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

    (stripped, tone)
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
