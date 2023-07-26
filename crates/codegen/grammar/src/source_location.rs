#[derive(Clone, Copy, Debug)]
pub struct SourceLocation {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug)]
pub struct Located<T: Clone + std::fmt::Debug> {
    pub value: T,
    pub source_location: SourceLocation,
}

impl<T: Clone + std::fmt::Debug> std::ops::Deref for Located<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
