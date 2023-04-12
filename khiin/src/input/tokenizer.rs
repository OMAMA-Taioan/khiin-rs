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

pub fn tokenize(raw_buffer: &str) -> Vec<TokenOffset> {


    Vec::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_finds_a_word() {}
}