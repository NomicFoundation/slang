#[path = "kinds.generated.rs"]
mod kinds;

#[path = "nodes.generated.rs"]
mod nodes;
pub use nodes::*;

#[path = "text_range.generated.rs"]
mod text_range;
pub use text_range::TextRange;

mod node_extensions;

mod builder;

mod source;
pub use source::Source;

#[path = "visitor.generated.rs"]
pub mod visitor;

pub use builder::{build_source_unit as build, BuildOutput, NodeIdGenerator, NodeKindHistogram};
