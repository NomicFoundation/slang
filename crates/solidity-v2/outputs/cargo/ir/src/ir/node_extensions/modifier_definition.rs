use std::ops::Range;

use crate::ir;

impl ir::ModifierDefinitionStruct {
    /// The text range of the modifier's signature, ie. everything but its body.
    /// Falls back to the full range for bodyless declarations.
    pub fn signature_text_range(&self) -> Range<usize> {
        let end = match &self.body {
            Some(body) => body.range.start,
            None => self.range.end,
        };
        self.range.start..end
    }
}
