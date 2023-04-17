use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;

use crate::buffer::BufferElement;
use crate::data::models::Conversion;
use crate::input::Syllable;

const SYL_SEPS: [char; 2] = ['-', ' '];

#[derive(Debug, Clone)]
pub(crate) struct KhiinElem {
    value: Vec<Syllable>,
    candidate: Option<Conversion>,
    converted: bool,
    selected: bool,
}

fn get_first_syllable(target: &str) -> &str {
    for (i, c) in target.char_indices() {
        if SYL_SEPS.contains(&c) {
            return &target[..i];
        }
    }
    target
}

impl KhiinElem {
    // Returns just the syllables, without spacers
    pub fn syls_only(&self) -> Vec<&Syllable> {
        self.value.iter().filter(|&syl| !syl.is_spacer()).collect()
    }

    pub fn from_conversion(raw_input: &str, conv: &Conversion) -> Result<Self> {
        let mut elems = Vec::new();
        let target = conv.input.as_str();
        let mut raw = raw_input;

        for target_syl in target.split(&SYL_SEPS) {
            if let Some((len, syl)) =
                Syllable::from_conversion_alignment(raw, target_syl)
            {
                raw = &raw[len..];
                elems.push(syl);
                elems.push(Syllable::spacer());
            }
        }

        if elems.is_empty() {
            Err(anyhow!("Unable make an element from conversion"))
        } else {
            elems.pop(); // Remove the last (extra) spacer
            Ok(Self {
                value: elems,
                candidate: Some(conv.clone()),
                converted: false,
                selected: false,
            })
        }
    }

    fn raw_caret_from_composed(&self, caret: usize) -> usize {
        0
    }

    // converted 平安
    // syllable: peng, an
    // caret at converted = 1
    // raw caret should be 4
    fn raw_caret_from_converted(&self, caret: usize) -> usize {
        if caret >= self.converted_text().chars().count() {
            return self.raw_char_count();
        }

        if self.candidate.is_none() {
            return self.raw_caret_from_composed(caret);
        }

        let candidate = self.candidate.clone().unwrap();

        let cand_syls = candidate.align_input_output_syllables();

        if cand_syls.is_none() {
            return self.raw_char_count();
        }

        let cand_syls = cand_syls.unwrap();
        let self_syls = self.syls_only();

        if cand_syls.len() != self_syls.len() {
            return self.raw_char_count();
        }

        let mut remainder = caret;
        let mut raw_caret = 0;

        for (i, (_, output)) in cand_syls.iter().enumerate() {
            let converted_char_count = output.chars().count();
            if remainder >= converted_char_count {
                remainder -= converted_char_count;
                raw_caret += self.value[i].raw_input.chars().count();
            } else {
                raw_caret += self.value[i].raw_caret_from_composed(remainder);
                break;
            }
        }

        raw_caret
    }
}

impl BufferElement for KhiinElem {
    fn raw_text(&self) -> String {
        self.value.iter().fold(String::default(), |mut acc, elem| {
            acc.push_str(elem.raw_input.as_str());
            acc
        })
    }

    fn raw_len(&self) -> usize {
        self.value.iter().map(|s| s.raw_input.len()).sum()
    }

    fn raw_char_count(&self) -> usize {
        self.value.iter().map(|s| s.raw_input.chars().count()).sum()
    }

    fn raw_caret_from(&self, caret: usize) -> usize {
        if self.converted {
            self.raw_caret_from_converted(caret)
        } else {
            self.raw_caret_from_composed(caret)
        }
    }

    fn composed_text(&self) -> String {
        self.value.iter().map(|s| s.compose()).collect()
    }

    fn composed_char_count(&self) -> usize {
        todo!()
    }

    fn caret_from(&self, raw_caret: usize) -> usize {
        todo!()
    }

    fn converted_text(&self) -> String {
        if let Some(conv) = &self.candidate {
            conv.output.clone()
        } else {
            self.composed_text()
        }
    }

    fn set_converted(&mut self, converted: bool) {
        self.converted = converted;
    }

    fn is_converted(&self) -> bool {
        self.converted
    }

    fn is_selected(&self) -> bool {
        todo!()
    }

    fn set_khin(&self) {
        todo!()
    }

    fn candidate(&self) -> Option<Conversion> {
        todo!()
    }

    fn insert(&mut self, idx: usize, ch: char) {
        todo!()
    }

    fn erase(&mut self, idx: usize) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::*;

    #[test]
    fn it_builds_from_conversion_alignment() {
        let c = mock_conversion("hó bô", "好無");
        let elem = KhiinElem::from_conversion("hobo", &c).unwrap();
        assert_eq!(elem.value.len(), 3);
        assert_eq!(elem.value[0].raw_input, String::from("ho"));
        assert_eq!(elem.value[0].raw_body, String::from("ho"));
        assert!(elem.value[1].is_spacer());
        assert_eq!(elem.value[2].raw_input,String::from("bo"));
        assert_eq!(elem.value[2].raw_body,String::from("bo"));
    }

    #[test]
    fn it_gets_raw_caret_from_converted() {
        let c = mock_conversion("hó bô", "好無");
        let elem = KhiinElem::from_conversion("hobo", &c).unwrap();
        assert_eq!(elem.raw_caret_from_converted(0), 0);
        assert_eq!(elem.raw_caret_from_converted(1), 2);
        assert_eq!(elem.raw_caret_from_converted(2), 4);

        let c = mock_conversion("hó bô", "好無");
        let elem = KhiinElem::from_conversion("ho2bo5", &c).unwrap();
        assert_eq!(elem.raw_caret_from_converted(0), 0);
        assert_eq!(elem.raw_caret_from_converted(1), 3);
        assert_eq!(elem.raw_caret_from_converted(2), 6);
    }

    #[test]
    fn it_gets_raw_caret_from_composed() {
        let c = mock_conversion("hó bô", "好無");
        let elem = KhiinElem::from_conversion("hobo", &c).unwrap();
    }
}
