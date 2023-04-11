use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::collection;

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

#[repr(i32)]
#[derive(Default, PartialEq, Eq, Hash)]
enum Tone {
    #[default]
    None = 0,
    T1 = 1,
    T2 = 2,
    T3 = 3,
    T4 = 4,
    T5 = 5,
    T6 = 6,
    T7 = 7,
    T8 = 8,
    T9 = 9,
}

impl From<i32> for Tone {
    fn from(value: i32) -> Self {
        match value {
            x if x == Tone::T1 as i32 => Tone::T1,
            x if x == Tone::T2 as i32 => Tone::T2,
            x if x == Tone::T3 as i32 => Tone::T3,
            x if x == Tone::T4 as i32 => Tone::T4,
            x if x == Tone::T5 as i32 => Tone::T5,
            x if x == Tone::T6 as i32 => Tone::T6,
            x if x == Tone::T7 as i32 => Tone::T7,
            x if x == Tone::T8 as i32 => Tone::T8,
            x if x == Tone::T9 as i32 => Tone::T9,
            _ => Tone::None,
        }
    }
}

fn tone_to_char(tone: &Tone) -> Option<char> {
    TONE_CHAR_MAP.get(tone).map(|&c| c)
}

fn get_tone_position(syllable: &str) -> Option<usize> {
    for (pat, offset) in TONE_LETTER_PATTERNS.iter() {
        if let Some(mat) = pat.find(syllable) {
            return Some(mat.start() + offset);
        }
    }

    None
}

#[derive(Default)]
pub struct Syllable {
    raw_body: String,
    tone: Tone,
    khin: bool,
}

impl Syllable {
    pub fn new(ascii: &str) -> Self {
        if ascii.is_empty() {
            return Self::default();
        }

        let mut raw_body = ascii.to_string();
        let last = raw_body.chars().last().unwrap();
        
        if let Some(index) = NUMERIC_TONE_CHARS.iter().position(|&c| c == last)
        {
            raw_body.pop();
            let tone: Tone = (index as i32).into();
            return Self {
                raw_body,
                tone,
                khin: false,
            };
        }

        Self {
            raw_body,
            tone: Tone::None,
            khin: false,
        }
    }

    pub fn compose(&self) -> String {
        let mut ret = self.raw_body.replace("nn", "ⁿ").replace("ou", "o͘");

        if self.tone == Tone::None {
            return ret;
        }

        if let Some(pos) = get_tone_position(&ret) {
            if let Some(tone_char) = tone_to_char(&self.tone) {
                ret.insert(pos + 1, tone_char.to_owned());
                return ret.nfc().collect::<String>();
            }
        }

        self.raw_body.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_places_tones() {
        assert_eq!(Syllable::new("a2").compose(), "á");
        assert_eq!(Syllable::new("oan5").compose(), "oân");
        assert_eq!(Syllable::new("goeh8").compose(), "goe̍h");
    }

    #[test]
    fn it_parses_long_o() {
        assert_eq!(Syllable::new("hou2").compose(), "hó͘");
        assert_eq!(Syllable::new("houh").compose(), "ho͘h");
    }

    #[test]
    fn it_parses_nasal_n() {
        assert_eq!(Syllable::new("ann3").compose(), "àⁿ");
        assert_eq!(Syllable::new("hahnn9").compose(), "hăhⁿ");
    }

    #[test]
    fn it_works_for_uppercase() {
        assert_eq!(Syllable::new("OAN5").compose(), "OÂN");
    }

    #[test]
    fn it_finds_tone_char_index() {
        assert_eq!(get_tone_position("a"), Some(0));
        assert_eq!(get_tone_position("oan"), Some(1));
        assert_eq!(get_tone_position("goeh"), Some(2));
        assert_eq!(get_tone_position("chhan"), Some(3));
        assert_eq!(get_tone_position("mng"), Some(1));
        assert_eq!(get_tone_position("mh"), Some(0));
        assert_eq!(get_tone_position("hmh"), Some(1));
        assert_eq!(get_tone_position("choat"), Some(3));
    }
}
