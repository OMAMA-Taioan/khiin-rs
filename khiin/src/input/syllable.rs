use std::default;

use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use khiin_ji::lomaji::get_tone_position;
use khiin_ji::lomaji::key_to_tone;
use khiin_ji::lomaji::strip_khin;
use khiin_ji::lomaji::strip_tone_diacritic;
use khiin_ji::lomaji::tone_to_char;
use khiin_ji::Tone;

#[derive(Default, Debug, PartialEq, Clone)]
pub(crate) struct Syllable {
    pub raw_input: String,
    pub raw_body: String,
    pub tone: Tone,
    pub khin: bool,
}

impl Syllable {
    pub fn new() -> Self {
        Self {
            raw_input: String::new(),
            raw_body: String::new(),
            tone: Tone::None,
            khin: false,
        }
    }

    pub fn compose(&self) -> String {
        let mut ret: String = self
            .raw_body
            .replace("ou", "o͘")
            .replace("oU", "o͘")
            .replace("Ou", "O͘")
            .replace("OU", "O͘");

        // to handle NASAL
        let re_single_nasal: Regex =
            Regex::new(r"(?i)[aeiouptkhmo͘]nn$").unwrap();
        if re_single_nasal.is_match(&ret) {
            ret = ret
                .replace("nn", "ⁿ")
                .replace("nN", "ⁿ")
                .replace("Nn", "ⁿ")
                .replace("NN", "ᴺ");
        }
        // move 'ⁿ' to end
        if ret.contains("ⁿ") && !ret.ends_with("ⁿ") {
            ret = ret.replace("ⁿ", "");
            ret.push_str("ⁿ");
        }
        if ret.contains("ᴺ") && !ret.ends_with("ᴺ") {
            ret = ret.replace("ᴺ", "");
            ret.push_str("ᴺ");
        }
        ret = ret
                .replace("Oᴺ", "O͘ᴺ")
                .replace("Oⁿ", "O͘ⁿ")
                .replace("oⁿ", "o͘ⁿ")
                .replace("OHᴺ", "O͘Hᴺ")
                .replace("Ohⁿ", "O͘hⁿ")
                .replace("ohⁿ", "o͘hⁿ");
        if self.khin {
            // ret.insert(0, '·');
            ret.insert(0, '-');
            ret.insert(1, '-');
        }

        if self.tone == Tone::None
            || self.tone == Tone::T1
            || self.tone == Tone::T4
        {
            return ret;
        }

        if let Some(pos) = get_tone_position(&ret) {
            if let Some(tone_char) = tone_to_char(&self.tone) {
                ret.insert(pos + 1, tone_char.to_owned());
                return ret.nfc().collect::<String>();
            }
        }

        self.raw_input.to_owned()
    }

    pub fn from_raw(raw_input: &str) -> Self {
        assert!(raw_input.is_ascii());
        let raw_input = raw_input.to_string();

        if raw_input.is_empty() {
            return Self::default();
        }

        let mut raw_body = raw_input.to_string();

        let last = raw_body.chars().last().unwrap();
        let tone = key_to_tone(last);
        if tone != Tone::None {
            raw_body.pop();
        }

        // TODO khin

        Self {
            raw_input,
            raw_body,
            tone,
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

        let mut raw_input = raw_body.clone();

        if let Some(ch) = get_tone_char(tone) {
            raw_input.push(ch);
        }

        if khin {
            raw_input.push('0');
        }

        Self {
            raw_input,
            raw_body,
            khin,
            tone,
        }
    }

    pub fn from_conversion_alignment(
        raw_input: &str,
        target: &str,
    ) -> Option<(usize, Syllable)> {
        let target = Syllable::from_composed(target);

        let mut shared_prefix_count = raw_input
            .chars()
            .zip(target.raw_input.chars())
            .take_while(|&(c1, c2)| c1.to_lowercase().eq(c2.to_lowercase()))
            .count();

        if shared_prefix_count == 0 {
            return None;
        }

        // User's tone key could be different from the tone key provided by
        // Syllable::from_composed. For the tone in particular, we must check
        // the actual tone represented by the key, and not just the key itself.
        // In this case, the shared prefix length would be one short, with the
        // next key being the tone.
        if target.tone != Tone::None {
            if let Some(ch) = raw_input.chars().nth(shared_prefix_count + 1) {
                let tone = key_to_tone(ch);
                if tone == target.tone {
                    shared_prefix_count += 1;
                }
            }
        }

        let raw_syl: String =
            raw_input.chars().take(shared_prefix_count).collect();

        let shared_prefix_len = raw_input
            .char_indices()
            .nth(shared_prefix_count)
            .map(|(i, _)| i)
            .unwrap_or_default();

        Some((shared_prefix_len, Syllable::from_raw(&raw_syl)))
    }

    pub(crate) fn raw_caret_from_composed(&self, remainder: usize) -> usize {
        // TODO
        0
    }
}

fn get_tone_char(tone: Tone) -> Option<char> {
    match tone {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_from_raw() {
        let syl = Syllable::from_raw("ho2");
        assert_eq!(syl.raw_body, "ho");
        assert_eq!(syl.raw_input, "ho2");
        assert_eq!(syl.tone, Tone::T2);
        assert_eq!(syl.khin, false);
    }

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
            assert_eq!(syl.raw_input, case.4);
        }
    }

    #[test]
    fn it_aligns_with_conversions() {
        let cases = vec![
            ("hobo", "hó", 2, "ho", Tone::None, false),
            ("ho2bo5", "hó", 3, "ho", Tone::T2, false),
        ];

        for case in cases {
            let (n, syl) = Syllable::from_conversion_alignment(case.0, case.1)
                .expect("Could not do conversion alignment");
            assert_eq!(case.2, n);
            assert_eq!(case.3, syl.raw_body);
            assert_eq!(case.4, syl.tone);
            assert_eq!(case.5, syl.khin);
        }
    }
}
