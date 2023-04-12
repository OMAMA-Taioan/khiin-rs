use crate::data::dictionary::Dictionary;

pub enum TokenType {
    Unknown,
    Hyphens,
    Punct,
    Splittable,
}

pub struct TokenOffset {
    ty: TokenType,
    start: usize,
    size: usize,
}

pub fn tokenize(dict: &Dictionary, raw_buffer: &str) -> Vec<TokenOffset> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use crate::data::dictionary::Dictionary;
    use crate::tests::get_dict;

    use super::tokenize;

    #[test]
    fn it_finds_a_word() {
        let dict = get_dict();
        let result = tokenize(&dict, "ho2");
        assert_eq!(result.len(), 1);
    }
}
