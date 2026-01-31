pub trait StringIndexing {
    fn at(&self, index: usize) -> Option<char>;
}

impl StringIndexing for str {
    fn at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
}
