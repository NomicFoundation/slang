#[path = "kinds.generated.rs"]
mod kinds;

#[path = "nodes.generated.rs"]
mod nodes;
pub use nodes::*;

#[path = "node_identity.generated.rs"]
mod node_identity;
pub use node_identity::NodeIdentity;

#[path = "text_range.generated.rs"]
mod text_range;
pub use text_range::TextRange;

mod node_extensions;

mod builder;
pub use builder::node_id_generator::{NodeIdGenerator, NodeKindHistogram};
pub use builder::{BuildOutput, build_source_unit as build};

mod source;
pub use source::Source;

#[path = "visitor.generated.rs"]
pub mod visitor;
