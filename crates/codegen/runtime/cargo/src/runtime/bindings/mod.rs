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

use crate::language::Language;

pub fn create_for(language: Language) -> Bindings {
    Bindings::create(language, binding_rules::BINDING_RULES_SOURCE)
}

use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;
use stack_graphs::graph::StackGraph;

#[derive(Error, Debug)]
pub enum BindingsError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Parsing source file failed")]
    ParseError(Vec<crate::parse_error::ParseError>),

    #[error(transparent)]
    BuildError(#[from] metaslang_graph_builder::stack_graph::BuildError),
}

pub struct Bindings {
    language: Language,
    stack_graph: StackGraph,
    sgl: stack_graph::StackGraphLanguage,
}

impl Bindings {
    #[allow(dead_code)]
    pub(crate) fn create(language: Language, msgb_source: &str) -> Self {
        let graph_builder_file = File::from_str(msgb_source)
            .expect("Bindings stack graph builder parse error");
        let sgl = stack_graph::StackGraphLanguage::new(graph_builder_file);
        let stack_graph = StackGraph::new();

        Self {
            language,
            stack_graph,
            sgl,
        }
    }

    pub fn add_file(&mut self, source_file_path: &Path) -> Result<(), BindingsError> {
        let input = fs::read_to_string(source_file_path)?;
        let parse_output = self.language.parse(Language::ROOT_KIND, &input);
        if !parse_output.is_valid() {
            return Err(BindingsError::ParseError(parse_output.errors))
        }
        let tree_cursor = parse_output.create_tree_cursor();

        let root_path = source_file_path
            .parent()
            .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
        let file_name = source_file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<unknown>");
        let file = self.stack_graph.get_or_create_file(file_name);

        let mut globals = Variables::new();
        // TODO: add the Language version as well to allow semantic changes between versions
        globals
            .add(
                stack_graph::ROOT_PATH_VAR.into(),
                root_path.to_str().unwrap().into(),
            )
            .expect("failed to add ROOT_PATH variable");
        globals
            .add(
                stack_graph::FILE_PATH_VAR.into(),
                source_file_path.to_str().unwrap().into(),
            )
            .expect("failed to add FILE_PATH variable");

        let mut builder =
            self.sgl
                .builder_into_stack_graph(&mut self.stack_graph, file, tree_cursor);
        builder.build(&globals, &stack_graph::NoCancellation)?;

        Ok(())
    }
}
