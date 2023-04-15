use std::str::CharIndices;

use crate::data::dictionary::Dictionary;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SectionType {
    Unknown,
    Hyphens,
    Punct,
    Splittable,
}

pub struct Section<'a> {
    pub ty: SectionType,
    pub raw_buffer: &'a str,
}

struct Offset {
    ty: SectionType,
    start: usize,
    end: usize,
}

impl Offset {
    pub fn new(ty: SectionType, start: usize, end: usize) -> Self {
        Self { ty, start, end }
    }

    pub fn as_slice_of<'a>(&self, raw_buffer: &'a str) -> Section<'a> {
        let text = &raw_buffer[self.start..self.end];
        Section { ty: self.ty, raw_buffer: text }
    }
}

trait IterAdvance: Iterator {
    fn adv_by(&mut self, n: usize) {
        for _ in 0..n {
            if self.next().is_none() {
                break;
            }
        }
    }
}

impl<'a> IterAdvance for CharIndices<'a> {}

pub fn parse_input<'a>(
    dict: &Dictionary,
    raw_buffer: &'a str,
) -> Vec<Section<'a>> {
    let mut offsets = Vec::new();

    let j = raw_buffer.len();
    let mut iter = raw_buffer.char_indices();

    let mut unk_start = 0;
    let mut unk_end = 0;

    let flush_unk =
        |start: &mut usize, end: &mut usize, offs: &mut Vec<Offset>| {
            if *end > 0 {
                offs.push(Offset::new(SectionType::Unknown, *start, *end));
                *start = 0;
                *end = 0;
            }
        };

    while let Some((i, ch)) = iter.next() {
        let remainder = &raw_buffer[i..j];



        if dict.can_segment(remainder) {
            flush_unk(&mut unk_start, &mut unk_end, &mut offsets);
            offsets.push(Offset::new(SectionType::Splittable, i, j));
            break;
        }

        if unk_end == 0 {
            unk_start = i;
            unk_end = i;
        }

        unk_end += ch.len_utf8();
    }

    flush_unk(&mut unk_start, &mut unk_end, &mut offsets);
    offsets
        .into_iter()
        .map(|sec| sec.as_slice_of(raw_buffer))
        .collect()
}

fn check_splittable(dict: &Dictionary, raw_buffer: &str) -> usize {
    dict.can_segment_max(raw_buffer)
}

#[cfg(test)]
mod tests {
    use crate::input::parser::SectionType;
    use crate::tests::get_dict;

    use super::parse_input;

    #[test]
    fn it_finds_a_word() {
        let dict = get_dict();
        let result = parse_input(&dict, "ho2");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].ty, SectionType::Splittable);
        assert_eq!(result[0].raw_buffer, "ho2");
    }

    #[test]
    fn it_finds_unknown() {
        let dict = get_dict();
        let result = parse_input(&dict, "zzz");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].ty, SectionType::Unknown);
        assert_eq!(result[0].raw_buffer, "zzz");

        let result = parse_input(&dict, "平安");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].ty, SectionType::Unknown);
        assert_eq!(result[0].raw_buffer, "平安");
    }

    #[test]
    fn it_finds_different_types() {
        let dict = get_dict();
        let result = parse_input(&dict, "zzzho2bo5");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].ty, SectionType::Unknown);
        assert_eq!(result[0].raw_buffer, "zzz");
        assert_eq!(result[1].ty, SectionType::Splittable);
        assert_eq!(result[1].raw_buffer, "ho2bo5");

        let result = parse_input(&dict, "ho2bo5zzz");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].ty, SectionType::Splittable);
        assert_eq!(result[0].raw_buffer, "ho2bo5");
        assert_eq!(result[1].ty, SectionType::Unknown);
        assert_eq!(result[1].raw_buffer, "zzz");
    }
}
