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
type Builder<'a> = stack_graph::Builder<'a, KindTypes>;

use std::path::PathBuf;

use semver::Version;
use stack_graphs::graph::StackGraph;
use thiserror::Error;

use crate::cursor::Cursor;

#[derive(Error, Debug)]
pub enum BindingsError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Unknown error condition: {0}")]
    UnknownError(String),

    #[error(transparent)]
    BuildError(#[from] metaslang_graph_builder::stack_graph::BuildError),
}

pub struct Bindings {
    version: Version,
    stack_graph: StackGraph,
    graph_builder_file: File,
    functions: Functions,
}

pub fn create_for(version: Version) -> Bindings {
    Bindings::create(version, binding_rules::BINDING_RULES_SOURCE)
}

impl Bindings {
    #[allow(dead_code)]
    pub(crate) fn create(version: Version, msgb_source: &str) -> Self {
        let graph_builder_file =
            File::from_str(msgb_source).expect("Bindings stack graph builder parse error");
        let stack_graph = StackGraph::new();
        let functions = stack_graph::default_functions();

        Self {
            version,
            stack_graph,
            graph_builder_file,
            functions,
        }
    }

    pub fn add_file(&mut self, file_path: &str, tree_cursor: Cursor) -> Result<(), BindingsError> {
        let globals = self.get_globals_for_file(file_path)?;
        let file = self.stack_graph.get_or_create_file(file_path);

        let mut builder = Builder::new(
            &self.graph_builder_file,
            &self.functions,
            &mut self.stack_graph,
            file,
            tree_cursor,
        );
        builder.build(&globals, &stack_graph::NoCancellation)?;

        Ok(())
    }

    fn get_globals_for_file(&self, file_path: &str) -> Result<Variables<'static>, BindingsError> {
        let path = PathBuf::from(&file_path).canonicalize()?;
        let root_path = path.parent().ok_or(BindingsError::UnknownError(
            "Cannot compute the ROOT_PATH".to_owned(),
        ))?;

        let mut globals = Variables::new();
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

        Ok(globals)
    }
}
