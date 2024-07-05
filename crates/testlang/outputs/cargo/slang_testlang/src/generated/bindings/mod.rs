// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "generated/binding_rules.rs"]
mod binding_rules;

pub mod graph_builder {
    use metaslang_graph_builder::{ast, functions, graph};

    use crate::cst::KindTypes;

    pub type File = ast::File<KindTypes>;
    pub type Functions = functions::Functions<KindTypes>;
    pub type Graph = graph::Graph<KindTypes>;

    pub use metaslang_graph_builder::{ExecutionConfig, NoCancellation, Variables};
}

use metaslang_bindings;
pub use metaslang_bindings::BindingsError;
use semver::Version;

use crate::cst::KindTypes;
pub type Bindings = metaslang_bindings::Bindings<KindTypes>;
pub type Handle<'a> = metaslang_bindings::Handle<'a, KindTypes>;

pub fn create(version: Version) -> Bindings {
    Bindings::create(version, binding_rules::BINDING_RULES_SOURCE)
}
