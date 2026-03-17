use std::ops::Range;

pub trait Source {
    fn text(&self, range: Range<usize>) -> &str;
}

impl Source for str {
    fn text(&self, range: Range<usize>) -> &str {
        &self[range]
    }
}

impl Source for String {
    fn text(&self, range: Range<usize>) -> &str {
        &self[range]
    }
}
