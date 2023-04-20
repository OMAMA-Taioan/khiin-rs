pub const HANJI_CUTOFF: char = '\u{2e80}';
pub const TONE_2: char = '\u{0301}';
pub const TONE_3: char = '\u{0300}';
pub const TONE_5: char = '\u{0302}';
pub const TONE_6: char = '\u{030C}';
pub const TONE_7: char = '\u{0304}';
pub const TONE_8: char = '\u{030D}';
pub const TONE_9: char = '\u{0306}';
pub const TONE_LB: char = TONE_3;
pub const TONE_UB: char = TONE_8;
pub const NASAL_LC: char = '\u{207f}';
pub const NASAL_UC: char = '\u{1d3a}';
pub const DOT_ABOVE_RIGHT: char = '\u{0358}';
pub const DOTS_BELOW: char = '\u{0324}';
pub const DOT_KHIN: char = '\u{00b7}';

pub trait ToByteLen {
    fn to_byte_len(&self, char_count: usize) -> usize;
}

impl ToByteLen for &str {
    /// Computes the length in bytes of the first `char_count` chars of this
    /// string.
    fn to_byte_len(&self, char_count: usize) -> usize {
        let mut len = 0;
        let mut chars = self.chars();

        for _ in 0..char_count {
            if let Some(c) = chars.next() {
                len += c.len_utf8();
            } else {
                break;
            }
        }

        len
    }
}

pub trait IsHanji {
    fn is_hanji(&self) -> bool;
}

impl IsHanji for char {
    fn is_hanji(&self) -> bool {
        self >= &HANJI_CUTOFF
    }
}

pub fn contains_hanji(s: &str) -> bool {
    s.chars().any(|c| c.is_hanji())
}
