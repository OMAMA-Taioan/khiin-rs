use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::collection;
use crate::input::lomaji::strip_khin;
use crate::input::lomaji::strip_tone_diacritic;

use super::lomaji::get_tone_position;
use super::lomaji::tone_char_to_index;
use super::lomaji::tone_to_char;
use super::Tone;

#[derive(Default)]
pub struct Syllable {
    pub raw_body: String,
    pub tone: Tone,
    pub khin: bool,
}

impl Syllable {
    pub fn compose(&self) -> String {
        let mut ret = self.raw_body.replace("nn", "ⁿ").replace("ou", "o͘");

        if self.khin {
            ret.insert(0, '·');
        }

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

    pub fn compose_raw(&self) -> String {
        let mut composed = self.raw_body.clone();

        let tone_char: Option<char> = match self.tone {
            Tone::None => None,
            Tone::T1 => Some('1'),
            Tone::T2 => Some('2'),
            Tone::T3 => Some('3'),
            Tone::T4 => Some('4'),
            Tone::T5 => Some('5'),
            Tone::T6 => Some('6'),
            Tone::T7 => Some('7'),
            Tone::T8 => Some('8'),
            Tone::T9 => Some('9'),
        };

        if let Some(ch) = tone_char {
            composed.push(ch);
        }

        if self.khin {
            composed.push('0');
        }

        composed
    }

    pub fn from_raw(raw_input: &str) -> Self {
        assert!(raw_input.is_ascii());

        if raw_input.is_empty() {
            return Self::default();
        }

        let mut raw_body = raw_input.to_string();
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

    pub fn from_composed(input: &str) -> Self {
        if input.is_empty() {
            return Self::default();
        }

        let replacements = vec![
            ('\u{0358}', "u".to_string()),
            ('\u{207f}', "nn".to_string()),
        ];

        let (mut stripped, tone) = strip_tone_diacritic(input);
        let khin = strip_khin(&mut stripped);
        let raw_body: String = stripped
            .chars()
            .flat_map(|ch| {
                for (x, y) in &replacements {
                    if ch == *x {
                        return y.chars().collect::<Vec<_>>();
                    }
                }
                vec![ch]
            })
            .collect();

        Self {
            raw_body,
            khin,
            tone,
        }
    }

    pub fn from_conversion_alignment(
        raw_input: &str,
        target: &Syllable,
    ) -> Syllable {
        

        Syllable::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_places_tones() {
        assert_eq!(Syllable::from_raw("a2").compose(), "á");
        assert_eq!(Syllable::from_raw("oan5").compose(), "oân");
        assert_eq!(Syllable::from_raw("goeh8").compose(), "goe̍h");
    }

    #[test]
    fn it_parses_long_o() {
        assert_eq!(Syllable::from_raw("hou2").compose(), "hó͘");
        assert_eq!(Syllable::from_raw("houh").compose(), "ho͘h");
    }

    #[test]
    fn it_parses_nasal_n() {
        assert_eq!(Syllable::from_raw("ann3").compose(), "àⁿ");
        assert_eq!(Syllable::from_raw("hahnn9").compose(), "hăhⁿ");
    }

    #[test]
    fn it_works_for_uppercase() {
        assert_eq!(Syllable::from_raw("OAN5").compose(), "OÂN");
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

    #[test]
    fn it_parses_from_composed_lomaji() {
        let cases = vec![
            ("hó", "ho", Tone::T2, false, "ho2"),
            ("goe̍h", "goeh", Tone::T8, false, "goeh8"),
            ("·hó͘ⁿ", "hounn", Tone::T2, true, "hounn20"),
            ("hm̍h", "hmh", Tone::T8, false, "hmh8"),
            ("mn̂g", "mng", Tone::T5, false, "mng5"),
            ("choân", "choan", Tone::T5, false, "choan5"),
        ];

        for case in cases {
            let syl = Syllable::from_composed(case.0);
            assert_eq!(syl.raw_body, case.1);
            assert_eq!(syl.tone, case.2);
            assert_eq!(syl.khin, case.3);
            assert_eq!(syl.compose(), case.0);
            assert_eq!(syl.compose_raw(), case.4);
        }
    }
}
