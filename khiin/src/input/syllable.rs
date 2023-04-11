use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::collection;

use super::lomaji::{get_tone_position, tone_to_char, tone_char_to_index};
use super::Tone;

#[derive(Default)]
pub struct Syllable {
    raw_body: String,
    tone: Tone,
    khin: bool,
}

impl Syllable {
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

impl From<&str> for Syllable {
    fn from(ascii: &str) -> Self {
        if ascii.is_empty() {
            return Self::default();
        }

        let mut raw_body = ascii.to_string();
        let last = raw_body.chars().last().unwrap();
        
        if let Some(index) = tone_char_to_index(last) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_places_tones() {
        assert_eq!(Syllable::from("a2").compose(), "á");
        assert_eq!(Syllable::from("oan5").compose(), "oân");
        assert_eq!(Syllable::from("goeh8").compose(), "goe̍h");
    }

    #[test]
    fn it_parses_long_o() {
        assert_eq!(Syllable::from("hou2").compose(), "hó͘");
        assert_eq!(Syllable::from("houh").compose(), "ho͘h");
    }

    #[test]
    fn it_parses_nasal_n() {
        assert_eq!(Syllable::from("ann3").compose(), "àⁿ");
        assert_eq!(Syllable::from("hahnn9").compose(), "hăhⁿ");
    }

    #[test]
    fn it_works_for_uppercase() {
        assert_eq!(Syllable::from("OAN5").compose(), "OÂN");
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
