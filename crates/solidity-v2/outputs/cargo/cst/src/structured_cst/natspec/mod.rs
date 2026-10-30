//! `NatSpec` comments, which document the code that follows them.
//!
//! See <https://docs.soliditylang.org/en/latest/natspec-format.html>.

use std::ops::Range;

/// A `NatSpec` comment's range: either a single `/** ... */` comment, or consecutive `///` lines.
#[derive(Clone, Debug, PartialEq)]
pub struct NatSpec {
    /// Range of the comment in the source.
    ///
    /// A `///` comment ends with its last line, without the line break after it.
    pub range: Range<usize>,
}
