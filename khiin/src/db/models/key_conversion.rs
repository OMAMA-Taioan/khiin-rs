use khiin_ji::IsHanji;
use khiin_ji::HANJI_CUTOFF;

use super::InputType;

#[derive(Debug, Clone)]
pub struct KeyConversion {
    pub key_sequence: String,
    pub input_type: InputType,
    pub input: String,
    pub input_id: u32,
    pub output: String,
    pub weight: i32,
    pub khin_ok: bool,
    pub khinless_ok: bool,
    pub annotation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CaseType {
    Lowercase = 0,
    Uppercase = 1,
    FirstUpper = 2,
}

impl KeyConversion {
    /// Attempts to align input and output syllables in a 1-to-1 mapping, taking
    /// each Hanji to be a syllable, and otherwise splitting by space characters
    /// as per the standard database format. Returns `None` if the alignment is
    /// not 1-to-1. If it is 1-to-1, `Some(Vec)` contains elements of the form
    /// `(input, output)`.
    pub fn align_input_output_syllables(
        &self,
    ) -> Option<Vec<(String, String)>> {
        let mut output_split = Vec::new();
        let mut tmp = String::new();

        for c in self.output.chars() {
            if c == ' ' || c > HANJI_CUTOFF {
                if !tmp.is_empty() {
                    output_split.push(tmp.clone());
                    tmp.clear();
                }

                if c != ' ' {
                    output_split.push(c.to_string());
                }
            } else {
                tmp.push(c);
            }
        }

        let input_split: Vec<String> =
            self.input.split(" ").map(|s| s.to_string()).collect();

        if output_split.len() != input_split.len() {
            return None;
        }

        Some(
            input_split
                .into_iter()
                .zip(output_split.into_iter())
                .collect(),
        )
    }

    pub fn set_output_case_type(&mut self, case_type: CaseType) {
        if case_type == CaseType::Lowercase {
            return;
        }
        if (self.output.is_empty()) {
            return;
        }
        for ch in self.output.chars() {
            if ch.is_hanji() {
                return;
            }
        }
        if (case_type == CaseType::Uppercase) {
            self.output = self.output.to_uppercase().replace("ⁿ", "ᴺ");
        } else {
            self.output = self.uppercase_first_letter(&self.output);
        }
    }

    pub fn convert_to_khin_hyphen(&mut self) {
        if (self.output.contains("-·")) {
            let mut has_pre_khin = false;
            let mut pre_char = 'a';
            let mut result = String::new();
            for ch in self.output.chars() {
                if (ch == '·' && pre_char == '-') {
                    if (!has_pre_khin) {
                        result.push('-');
                    }
                    has_pre_khin = true;
                } else {
                    if (ch == '·') {
                        has_pre_khin = true;
                    } else if ((pre_char == '-' || pre_char == ' ')
                        && ch.is_ascii_alphanumeric())
                    {
                        has_pre_khin = false;
                    }
                    result.push(ch);
                    pre_char = ch;
                }
            }
            self.output = result;
        }
        self.output = self.output.replace(" ·", "--");
        self.output = self.output.replace("·", "--");
    }

    pub fn convert_to_khinless(&mut self) {
        self.output = self.output.replace("·", "");
    }

    pub fn mark_guess_annotation(&mut self) {
        self.annotation = Some("?".to_string());
    }

    fn uppercase_first_letter(&self, s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_conversion(input: &str, output: &str) -> KeyConversion {
        KeyConversion {
            key_sequence: String::new(),
            input_type: InputType::Numeric,
            input: input.into(),
            input_id: 0,
            output: output.into(),
            weight: 0,
            khin_ok: true,
            khinless_ok: true,
            annotation: None,
        }
    }

    #[test]
    fn it_aligns_syllables() {
        let c = mock_conversion("hó bô", "好無");
        let ret = c.align_input_output_syllables();
        assert!(ret.is_some());
        let ret = ret.unwrap();
        assert_eq!(ret.len(), 2);
        let (i, o) = &ret[0];
        assert_eq!(i, "hó");
        assert_eq!(o, "好");
        let (i, o) = &ret[1];
        assert_eq!(i, "bô");
        assert_eq!(o, "無");
    }
}
