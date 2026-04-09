#[path = "text_start.generated.rs"]
mod text_start;

#[path = "text_end.generated.rs"]
mod text_end;

#[allow(clippy::module_inception)]
#[path = "text_range.generated.rs"]
mod text_range;

// TODO(v2): this currently has a bug, and might return a smaller range than expected,
// because `structured_cst` doesn't capture all nodes in input (like separators).
// We need to represent all non-trivia nodes in the CST.
pub use text_range::TextRange;
