use std::ops::Range;

use crate::ir;

impl ir::FunctionDefinitionStruct {
    /// The text range of the function's signature, ie. everything but its body.
    ///
    /// Falls back to the full range for bodyless declarations (eg. functions in
    /// interfaces or abstract contracts).
    ///
    pub fn signature_text_range(&self) -> Range<usize> {
        // TODO: This shouldn't use the start of the body, but right now
        // the IR doesn't have enough information to compute the
        // actual range.
        let end = match &self.body {
            Some(body) => body.range.start,
            None => self.range.end,
        };
        self.range.start..end
    }
}
