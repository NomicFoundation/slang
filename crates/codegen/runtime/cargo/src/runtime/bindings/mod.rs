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

use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::fmt::Debug;
use std::iter::once;

use metaslang_bindings::builder;
use semver::Version;
use stack_graphs::graph::StackGraph;
use stack_graphs::partial::PartialPaths;
use stack_graphs::stitching::{ForwardPartialPathStitcher, GraphEdgeCandidates, StitcherConfig};
use thiserror::Error;

use crate::cst::KindTypes;
use crate::cursor::Cursor;

type Builder<'a> = builder::Builder<'a, KindTypes>;
type GraphHandle = stack_graphs::arena::Handle<stack_graphs::graph::Node>;

#[derive(Error, Debug)]
pub enum BindingsError {
    #[error(transparent)]
    BuildError(#[from] metaslang_bindings::builder::BuildError),
}

pub struct Bindings {
    version: Version,
    graph_builder_file: graph_builder::File,
    functions: graph_builder::Functions,
    stack_graph: StackGraph,
    cursors: HashMap<GraphHandle, Cursor>,
}

impl Bindings {
    pub fn create(version: Version) -> Self {
        let graph_builder_file = graph_builder::File::from_str(binding_rules::BINDING_RULES_SOURCE)
            .expect("Bindings stack graph builder parse error");
        let stack_graph = StackGraph::new();
        let functions = graph_builder::Functions::stdlib();
        let cursors = HashMap::new();

        Self {
            version,
            graph_builder_file,
            functions,
            stack_graph,
            cursors,
        }
    }

    pub fn add_file(&mut self, file_path: &str, tree_cursor: Cursor) -> Result<(), BindingsError> {
        let globals = builder::Globals {
            version: &self.version,
            file_path,
        };
        let file = self.stack_graph.get_or_create_file(file_path);

        let mut builder = Builder::new(
            &self.graph_builder_file,
            &self.functions,
            &mut self.stack_graph,
            file,
            tree_cursor,
        );
        builder.build(&globals, &builder::NoCancellation, |handle, cursor| {
            self.cursors.insert(handle, cursor.clone());
        })?;

        Ok(())
    }

    pub fn all_definitions(&self) -> impl Iterator<Item = Handle<'_>> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter(|handle| self.stack_graph[*handle].is_definition())
            .map(|handle| Handle {
                owner: self,
                handle,
            })
    }

    pub fn all_references(&self) -> impl Iterator<Item = Handle<'_>> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter(|handle| self.stack_graph[*handle].is_reference())
            .map(|handle| Handle {
                owner: self,
                handle,
            })
    }
}

pub struct Handle<'a> {
    owner: &'a Bindings,
    handle: GraphHandle,
}

impl Handle<'_> {
    pub fn is_definition(&self) -> bool {
        self.owner.stack_graph[self.handle].is_definition()
    }

    pub fn is_reference(&self) -> bool {
        self.owner.stack_graph[self.handle].is_reference()
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        self.owner.cursors.get(&self.handle).cloned()
    }

    pub fn get_file(&self) -> Option<&str> {
        self.owner.stack_graph[self.handle]
            .file()
            .map(|file| self.owner.stack_graph[file].name())
    }

    pub fn jump_to_definition(&self) -> Option<Handle<'_>> {
        let mut paths = PartialPaths::new();
        let mut results = BTreeSet::new();
        if self.is_reference() {
            ForwardPartialPathStitcher::find_all_complete_partial_paths(
                &mut GraphEdgeCandidates::new(&self.owner.stack_graph, &mut paths, None),
                once(self.handle),
                StitcherConfig::default(),
                &stack_graphs::NoCancellation,
                |_graph, _paths, path| {
                    results.insert(path.end_node);
                },
            )
            .expect("should never be cancelled");
        }
        if results.len() > 1 {
            println!("WARN: More than one definition found for {self:?}");
        }
        results.first().map(|handle| Handle {
            owner: self.owner,
            handle: *handle,
        })
    }
}

impl Debug for Handle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BindingsHandle").field(&self.handle).finish()
    }
}
