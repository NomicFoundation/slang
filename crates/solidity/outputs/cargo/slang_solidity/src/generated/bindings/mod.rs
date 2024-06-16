// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "generated/binding_rules.rs"]
mod binding_rules;

use metaslang_graph_builder::ast;
pub use metaslang_graph_builder::functions::Functions;
pub use metaslang_graph_builder::{ExecutionConfig, ExecutionError, NoCancellation, Variables};

use crate::cst::KindTypes;

pub type File = ast::File<KindTypes>;
