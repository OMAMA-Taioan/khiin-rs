pub(crate) enum BufferElement {
    Plaintext(String),
}

impl From<&str> for BufferElement {
    fn from(value: &str) -> Self {
        BufferElement::Plaintext(value.to_owned())
    }
}

pub(crate) trait Insertable {
    fn raw_char_count(&self) -> usize;
    fn composed_char_count(&self) -> usize;
    fn insert_at(&mut self, idx: usize, ch: char);
    fn raw(&self) -> &str;
}

impl Insertable for BufferElement {
    fn raw_char_count(&self) -> usize {
        match self {
            BufferElement::Plaintext(str) => str.raw_char_count(),
        }
    }

    fn composed_char_count(&self) -> usize {
        todo!()
    }

    fn insert_at(&mut self, idx: usize, ch: char) {
        todo!()
    }

    fn raw(&self) -> &str {
        match self {
            BufferElement::Plaintext(str) => str.as_str(),
        }
    }
}

impl Insertable for String {
    fn raw_char_count(&self) -> usize {
        self.chars().count()
    }

    fn composed_char_count(&self) -> usize {
        self.chars().count()
    }

    fn insert_at(&mut self, idx: usize, ch: char) {
        self.insert(idx, ch);
    }

    fn raw(&self) -> &str {
        self.as_str()
    }
}
