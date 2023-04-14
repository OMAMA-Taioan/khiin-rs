use anyhow::Result;
use anyhow::anyhow;

use crate::data::models::Conversion;
use crate::input::Syllable;
use crate::input::Tone;

use super::BufferElement;

const SYL_SEPS: [char; 2] = ['-', ' '];

pub struct TaiText {
    elems: Vec<Syllable>,
}

fn get_first_syllable(target: &str) -> &str {
    for (i, c) in target.char_indices() {
        if SYL_SEPS.contains(&c) {
            return &target[..i];
        }
    }
    target
}

impl TaiText {
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
            Ok(Self { elems })
        }
    }
}

impl BufferElement for TaiText {
    fn raw_text(&self) -> &str {
        todo!()
    }

    fn raw_len(&self) -> usize {
        self.elems.iter().map(|s| s.raw_input.len()).sum()
    }

    fn raw_char_count(&self) -> usize {
        todo!()
    }

    fn raw_caret_from(&self, caret: usize) -> usize {
        todo!()
    }

    fn composed_text(&self) -> &str {
        todo!()
    }

    fn composed_char_count(&self) -> usize {
        todo!()
    }

    fn caret_from(&self, raw_caret: usize) -> usize {
        todo!()
    }

    fn converted(&self) -> &str {
        todo!()
    }

    fn is_converted(&self) -> bool {
        todo!()
    }

    fn is_selected(&self) -> bool {
        todo!()
    }

    fn set_khin(&self) {
        todo!()
    }

    fn candidate(&self) -> Option<&str> {
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
    use crate::data::models::Conversion;
    use crate::input::*;
    use super::*;

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
        let tt = TaiText::from_conversion("hobo", &c).unwrap();
        assert_eq!(tt.elems.len(), 2);
        assert_eq!(
            tt.elems[0],
            Syllable {
                raw_input: String::from("ho"),
                raw_body: String::from("ho"),
                tone: Tone::None,
                khin: false
            }
        );
        assert_eq!(
            tt.elems[1],
            Syllable {
                raw_input: String::from("bo"),
                raw_body: String::from("bo"),
                tone: Tone::None,
                khin: false
            }
        );
    }
}
