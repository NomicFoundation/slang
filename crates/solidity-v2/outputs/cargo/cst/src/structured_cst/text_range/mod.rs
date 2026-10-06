#[path = "text_start.generated.rs"]
mod text_start;

#[path = "text_end.generated.rs"]
mod text_end;

#[allow(clippy::module_inception)]
#[path = "text_range.generated.rs"]
mod text_range;

pub use text_range::TextRange;
