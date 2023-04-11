use super::BufferElement;
use super::Insertable;

#[derive(Default)]
pub(crate) struct Buffer {
    elems: Vec<BufferElement>,
}

impl Buffer {
    pub fn composition(&self) -> String {
        self.elems.iter().fold(String::default(), |mut acc, elem| {
            acc.push_str(elem.raw());
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buf = Buffer::default();
        assert_eq!(buf.elems.len(), 0);
    }

    #[test]
    fn foo() {
        let mut buf = Buffer::default();
        buf.elems.push("ho".into());
        assert_eq!(buf.composition().as_str(), "ho");
    }
}
