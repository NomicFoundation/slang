// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "generated/binding_rules.rs"]
mod binding_rules;

pub use metaslang_graph_builder::functions::Parameters;
pub use metaslang_graph_builder::graph::{Edge, GraphNode, GraphNodeRef, Value};
use metaslang_graph_builder::{ast, functions, stack_graph};
pub use metaslang_graph_builder::{
    CancellationError, CancellationFlag, ExecutionConfig, ExecutionError, NoCancellation,
    ParseError, Variables,
};

use crate::cst::KindTypes;

pub type File = ast::File<KindTypes>;
pub type Functions = functions::Functions<KindTypes>;
pub type Graph = metaslang_graph_builder::graph::Graph<KindTypes>;
pub type Builder<'a> = stack_graph::Builder<'a, KindTypes>;
pub type StackGraphLanguage = stack_graph::StackGraphLanguage<KindTypes>;

use std::io;
use std::path::PathBuf;

use semver::Version;
use stack_graphs::graph::StackGraph;
use thiserror::Error;

use crate::cursor::Cursor;

#[derive(Error, Debug)]
pub enum BindingsError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    BuildError(#[from] metaslang_graph_builder::stack_graph::BuildError),
}

pub struct Bindings {
    version: Version,
    stack_graph: StackGraph,
    sgl: StackGraphLanguage,
}

pub fn create_for(version: Version) -> Bindings {
    Bindings::create(version, binding_rules::BINDING_RULES_SOURCE)
}

impl Bindings {
    #[allow(dead_code)]
    pub(crate) fn create(version: Version, msgb_source: &str) -> Self {
        let graph_builder_file =
            File::from_str(msgb_source).expect("Bindings stack graph builder parse error");
        let sgl = StackGraphLanguage::new(graph_builder_file);
        let stack_graph = StackGraph::new();

        Self {
            version,
            stack_graph,
            sgl,
        }
    }

    pub fn add_file(&mut self, file_path: &str, tree_cursor: Cursor) -> Result<(), BindingsError> {
        let path = PathBuf::from(&file_path).canonicalize()?;
        let root_path = path
            .parent()
            .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
        let file = self.stack_graph.get_or_create_file(file_path);

        let mut globals = Variables::new();
        // TODO: add the Language version as well to allow semantic changes between versions
        globals
            .add(
                stack_graph::ROOT_PATH_VAR.into(),
                root_path.to_str().unwrap().into(),
            )
            .expect("failed to add ROOT_PATH variable");
        globals
            .add(stack_graph::FILE_PATH_VAR.into(), file_path.into())
            .expect("failed to add FILE_PATH variable");
        globals
            .add(
                stack_graph::VERSION_VAR.into(),
                self.version.to_string().into(),
            )
            .expect("failed to add VERSION_VAR variable");

        let mut builder =
            self.sgl
                .builder_into_stack_graph(&mut self.stack_graph, file, tree_cursor);
        builder.build(&globals, &stack_graph::NoCancellation)?;

        Ok(())
    }
}
