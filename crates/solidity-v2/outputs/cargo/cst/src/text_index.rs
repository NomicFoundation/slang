//! Text index and range types.
//!
//! TODO(v2) This is a minimal implementation, we'll probably want to expand it in the future.

use std::ops::Range;

pub type TextIndex = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct TextRange(Range<TextIndex>);

impl TextRange {
    pub fn from_bytes_range(bytes_range: Range<usize>) -> Self {
        Self(bytes_range)
    }

    pub fn bytes_range(&self) -> Range<usize> {
        self.0.clone()
    }
}
