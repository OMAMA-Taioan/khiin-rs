use anyhow::anyhow;
use anyhow::Result;

use crate::data::models::Conversion;
use crate::input::Syllable;
use crate::input::Tone;

use super::BufferElement;

const SYL_SEPS: [char; 2] = ['-', ' '];

#[derive(Debug, Clone)]
pub struct KhiinElem {
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
            }
        }

        if elems.is_empty() {
            Err(anyhow!("Unable make an element from conversion"))
        } else {
            Ok(Self {
                value: elems,
                candidate: Some(conv.clone()),
                converted: false,
                selected: false,
            })
        }
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
        todo!()
    }

    fn raw_caret_from(&self, caret: usize) -> usize {
        todo!()
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
    use crate::data::models::Conversion;
    use crate::input::*;

    fn mock_conversion(input: &str) -> Conversion {
        Conversion {
            key_sequence: String::new(),
            input: input.into(),
            input_id: 0,
            output: String::new(),
            weight: 0,
            category: None,
            annotation: None,
        }
    }

    #[test]
    fn it_builds_from_conversion_alignment() {
        let c = mock_conversion("hó bô");
        let tt = KhiinElem::from_conversion("hobo", &c).unwrap();
        assert_eq!(tt.value.len(), 2);
        assert_eq!(
            tt.value[0],
            Syllable {
                raw_input: String::from("ho"),
                raw_body: String::from("ho"),
                tone: Tone::None,
                khin: false
            }
        );
        assert_eq!(
            tt.value[1],
            Syllable {
                raw_input: String::from("bo"),
                raw_body: String::from("bo"),
                tone: Tone::None,
                khin: false
            }
        );
    }
}
