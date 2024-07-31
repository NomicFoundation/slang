pub mod builder;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::iter::once;
use std::sync::Arc;

use metaslang_cst::cursor::Cursor;
use metaslang_cst::KindTypes;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use semver::Version;
use stack_graphs::graph::StackGraph;
use stack_graphs::partial::PartialPaths;
use stack_graphs::stitching::{
    Appendable, ForwardPartialPathStitcher, GraphEdgeCandidates, StitcherConfig,
};

type Builder<'a, KT> = builder::Builder<'a, KT>;
type GraphHandle = stack_graphs::arena::Handle<stack_graphs::graph::Node>;

pub struct Bindings<KT: KindTypes + 'static> {
    graph_builder_file: File<KT>,
    functions: Functions<KT>,
    pub stack_graph: StackGraph,
    cursors: HashMap<GraphHandle, Cursor<KT>>,
}

pub trait PathResolver {
    fn resolve_path(&self, context_path: &str, path_to_resolve: &str) -> Option<String>;
}

pub struct DefaultPathResolver;

impl PathResolver for DefaultPathResolver {
    fn resolve_path(&self, _context_path: &str, path_to_resolve: &str) -> Option<String> {
        Some(path_to_resolve.to_string())
    }
}

impl<KT: KindTypes + 'static> Bindings<KT> {
    pub fn create(
        version: Version,
        binding_rules: &str,
        path_resolver: Arc<dyn PathResolver + Sync + Send>,
    ) -> Self {
        let graph_builder_file =
            File::from_str(binding_rules).expect("Bindings stack graph builder parse error");
        let stack_graph = StackGraph::new();
        let functions = builder::default_functions(version, path_resolver);
        let cursors = HashMap::new();

        Self {
            graph_builder_file,
            functions,
            stack_graph,
            cursors,
        }
    }

    pub fn add_file(&mut self, file_path: &str, tree_cursor: Cursor<KT>) {
        _ = self.add_file_internal(file_path, tree_cursor);
    }

    #[cfg(feature = "__private_testing_utils")]
    pub fn add_file_returning_graph(
        &mut self,
        file_path: &str,
        tree_cursor: Cursor<KT>,
    ) -> metaslang_graph_builder::graph::Graph<KT> {
        let builder = self.add_file_internal(file_path, tree_cursor);
        builder.graph()
    }

    fn add_file_internal(&mut self, file_path: &str, tree_cursor: Cursor<KT>) -> Builder<'_, KT> {
        let file = self.stack_graph.get_or_create_file(file_path);

        let mut builder = Builder::new(
            &self.graph_builder_file,
            &self.functions,
            &mut self.stack_graph,
            file,
            tree_cursor,
        );
        builder
            .build(file_path, &builder::NoCancellation, |handle, cursor| {
                self.cursors.insert(handle, cursor.clone());
            })
            .expect("Internal error while building bindings");
        builder
    }

    pub fn all_definitions(&self) -> impl Iterator<Item = Definition<'_, KT>> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter(|handle| self.stack_graph[*handle].is_definition())
            .map(|handle| Definition {
                owner: self,
                handle,
            })
    }

    pub fn all_references(&self) -> impl Iterator<Item = Reference<'_, KT>> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter(|handle| self.stack_graph[*handle].is_reference())
            .map(|handle| Reference {
                owner: self,
                handle,
            })
    }

    pub fn definition_at(&self, cursor: &Cursor<KT>) -> Option<Definition<'_, KT>> {
        for (handle, handle_cursor) in &self.cursors {
            if handle_cursor == cursor && self.stack_graph[*handle].is_definition() {
                return Some(Definition {
                    owner: self,
                    handle: *handle,
                });
            }
        }
        None
    }

    pub fn reference_at(&self, cursor: &Cursor<KT>) -> Option<Reference<'_, KT>> {
        for (handle, handle_cursor) in &self.cursors {
            if handle_cursor == cursor && self.stack_graph[*handle].is_reference() {
                return Some(Reference {
                    owner: self,
                    handle: *handle,
                });
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Definition<'a, KT: KindTypes + 'static> {
    owner: &'a Bindings<KT>,
    handle: GraphHandle,
}

impl<'a, KT: KindTypes + 'static> Definition<'a, KT> {
    pub fn get_cursor(&self) -> Option<Cursor<KT>> {
        self.owner.cursors.get(&self.handle).cloned()
    }

    pub fn get_file(&self) -> Option<&'a str> {
        self.owner.stack_graph[self.handle]
            .file()
            .map(|file| self.owner.stack_graph[file].name())
    }
}

impl<KT: KindTypes + 'static> Debug for Definition<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BindingsHandle").field(&self.handle).finish()
    }
}

impl<KT: KindTypes + 'static> PartialEq for Definition<'_, KT> {
    fn eq(&self, other: &Self) -> bool {
        let our_owner: *const Bindings<KT> = self.owner;
        let other_owner: *const Bindings<KT> = other.owner;
        our_owner == other_owner && self.handle == other.handle
    }
}

#[derive(Clone)]
pub struct Reference<'a, KT: KindTypes + 'static> {
    owner: &'a Bindings<KT>,
    handle: GraphHandle,
}

impl<'a, KT: KindTypes + 'static> Reference<'a, KT> {
    pub fn get_cursor(&self) -> Option<Cursor<KT>> {
        self.owner.cursors.get(&self.handle).cloned()
    }

    pub fn get_file(&self) -> Option<&'a str> {
        self.owner.stack_graph[self.handle]
            .file()
            .map(|file| self.owner.stack_graph[file].name())
    }

    pub fn jump_to_definition(&self) -> Option<Definition<'a, KT>> {
        let mut partials = PartialPaths::new();
        let mut reference_paths = Vec::new();
        ForwardPartialPathStitcher::find_all_complete_partial_paths(
            &mut GraphEdgeCandidates::new(&self.owner.stack_graph, &mut partials, None),
            once(self.handle),
            StitcherConfig::default(),
            &stack_graphs::NoCancellation,
            |_graph, _paths, path| {
                reference_paths.push(path.clone());
            },
        )
        .expect("should never be cancelled");

        let mut results = Vec::new();
        for reference_path in &reference_paths {
            if reference_paths
                .iter()
                .all(|other| !other.shadows(&mut partials, reference_path))
            {
                results.push(reference_path);
            }
        }
        if results.len() > 1 {
            println!(
                "WARN: More than one definition found for {}, will return the longest path",
                self.handle.display(&self.owner.stack_graph)
            );
            results.sort_by(|a, b| b.edges.len().cmp(&a.edges.len()));
        }
        results.first().map(|path| Definition {
            owner: self.owner,
            handle: path.end_node(),
        })
    }
}

impl<KT: KindTypes + 'static> Debug for Reference<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BindingsHandle").field(&self.handle).finish()
    }
}

impl<KT: KindTypes + 'static> PartialEq for Reference<'_, KT> {
    fn eq(&self, other: &Self) -> bool {
        let our_owner: *const Bindings<KT> = self.owner;
        let other_owner: *const Bindings<KT> = other.owner;
        our_owner == other_owner && self.handle == other.handle
    }
}
