#[path = "generated/binding_rules.rs"]
mod binding_rules;

use metaslang_graph_builder::{ast, functions};
pub use metaslang_graph_builder::graph::{Edge, GraphNode, GraphNodeRef, Value};
pub use metaslang_graph_builder::{
    CancellationError, CancellationFlag, ExecutionConfig, ExecutionError, NoCancellation,
    ParseError, Variables,
};
pub use metaslang_graph_builder::functions::Parameters;

use crate::cst::KindTypes;

pub type File = ast::File<KindTypes>;
pub type Functions = functions::Functions<KindTypes>;
pub type Graph = metaslang_graph_builder::graph::Graph<KindTypes>;
