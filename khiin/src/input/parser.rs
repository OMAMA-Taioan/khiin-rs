use khiin_ji::ToByteLen;

use crate::data::dictionary::Dictionary;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SectionType {
    Plaintext,
    Hyphens,
    Punct,
    Splittable,
}

pub(crate) fn parse_longest_from_start<'a>(
    dict: &Dictionary,
    raw_buffer: &'a str,
) -> (SectionType, &'a str) {
    let len = max_segmentable_len(dict, raw_buffer);
    if len > 0 {
        return (SectionType::Splittable, &raw_buffer[..len]);
    }

    (SectionType::Plaintext, raw_buffer)
}

pub(crate) fn parse_whole_input<'a>(
    dict: &Dictionary,
    raw_buffer: &'a str,
) -> Vec<(SectionType, &'a str)> {
    parse_input_to_sections(dict, raw_buffer)
}

/// Returns byte length of splittable section from the beginning
fn max_segmentable_len(dict: &Dictionary, raw_buffer: &str) -> usize {
    let char_count = dict.can_segment_max(raw_buffer);
    raw_buffer.to_byte_len(char_count)
}

/// Iterates over the input string, matching the string against the functions
/// provided in the while loop. These functions should provide the length (in
/// bytes) of the consumed text.
///
/// Returns a Vec of sections, with each section containing a
/// slice of the input string and the type of the section. Unknown sections that
/// were not matched by any of the matcher functions are passed through as
/// SectionType::Plaintext.
fn parse_input_to_sections<'a>(
    dict: &Dictionary,
    input: &'a str,
) -> Vec<(SectionType, &'a str)> {
    let mut result = Vec::new();
    let input_len = input.len();
    let mut start = 0;
    let mut unknown_start: Option<usize> = None;

    while start < input_len {
        let remaining = &input[start..];
        let mut parsed_type = SectionType::Plaintext;
        let mut parsed_len = 0;
        let mut done = false;

        if !done {
            let bytes = max_segmentable_len(dict, remaining);
            if bytes > 0 {
                parsed_type = SectionType::Splittable;
                parsed_len = bytes;
                done = true;
            }
        }

        if !done {
            // TODO check for a different segment type
        }

        if parsed_type != SectionType::Plaintext {
            if let Some(unk_start) = unknown_start.take() {
                let slice = &input[unk_start..start];
                result.push((SectionType::Plaintext, slice));
                unknown_start = None;
            }

            let slice = &input[start..start + parsed_len];
            result.push((parsed_type, slice));
            start += parsed_len;
        } else {
            if unknown_start.is_none() {
                unknown_start = Some(start);
            }
            let bytes = remaining.chars().next().unwrap().len_utf8();
            start += bytes;
        }
    }

    if let Some(unk_start) = unknown_start.take() {
        let slice = &input[unk_start..];
        result.push((SectionType::Plaintext, slice));
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::input::parser::SectionType;
    use crate::tests::get_dict;

    use super::parse_whole_input;

    #[test]
    fn it_finds_a_word() {
        let dict = get_dict();
        let result = parse_whole_input(&dict, "ho2");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, SectionType::Splittable);
        assert_eq!(result[0].1, "ho2");
    }

    #[test]
    fn it_finds_unknown() {
        let dict = get_dict();
        let result = parse_whole_input(&dict, "zzz");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, SectionType::Plaintext);
        assert_eq!(result[0].1, "zzz");

        let result = parse_whole_input(&dict, "平安");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, SectionType::Plaintext);
        assert_eq!(result[0].1, "平安");
    }

    #[test]
    fn it_finds_different_types() {
        let dict = get_dict();
        let result = parse_whole_input(&dict, "zzzho2bo5");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, SectionType::Plaintext);
        assert_eq!(result[0].1, "zzz");
        assert_eq!(result[1].0, SectionType::Splittable);
        assert_eq!(result[1].1, "ho2bo5");

        let result = parse_whole_input(&dict, "ho2bo5zzz");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, SectionType::Splittable);
        assert_eq!(result[0].1, "ho2bo5");
        assert_eq!(result[1].0, SectionType::Plaintext);
        assert_eq!(result[1].1, "zzz");
    }
}
