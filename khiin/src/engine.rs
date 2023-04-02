pub struct Engine;

impl Engine {
    pub fn new() -> Option<Engine> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let engine = Engine::new();
        assert!(engine.is_none());
    }
}
