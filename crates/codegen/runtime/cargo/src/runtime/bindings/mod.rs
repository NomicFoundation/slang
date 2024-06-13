use metaslang_graph_builder::ast;
pub use metaslang_graph_builder::functions::Functions;
pub use metaslang_graph_builder::{ExecutionConfig, ExecutionError, NoCancellation, Variables};

use crate::cst::KindTypes;

pub type File = ast::File<KindTypes>;
