#[path = "generated/binding_rules.rs"]
mod binding_rules;

pub use metaslang_graph_builder::functions::Parameters;
pub use metaslang_graph_builder::graph::{Edge, GraphNode, GraphNodeRef, Value};
use metaslang_graph_builder::{ast, functions};
pub use metaslang_graph_builder::{
    CancellationError, CancellationFlag, ExecutionConfig, ExecutionError, NoCancellation,
    ParseError, Variables,
};

use crate::cst::KindTypes;

pub type File = ast::File<KindTypes>;
pub type Functions = functions::Functions<KindTypes>;
pub type Graph = metaslang_graph_builder::graph::Graph<KindTypes>;

pub mod stack_graph {
    use metaslang_graph_builder::stack_graph;
    pub use metaslang_graph_builder::stack_graph::{
        BuildError, NoCancellation, FILE_PATH_VAR, ROOT_PATH_VAR,
    };

    use crate::cst::KindTypes;

    pub type Builder<'a> = stack_graph::Builder<'a, KindTypes>;
    pub type StackGraphLanguage = stack_graph::StackGraphLanguage<KindTypes>;
}
