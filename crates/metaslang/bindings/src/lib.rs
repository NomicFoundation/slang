pub mod builder;

use std::collections::{BTreeSet, HashMap};
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
    stack_graph: StackGraph,
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

    pub fn all_definitions(&self) -> impl Iterator<Item = Handle<'_, KT>> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter(|handle| self.stack_graph[*handle].is_definition())
            .map(|handle| Handle {
                owner: self,
                handle,
            })
    }

    pub fn all_references(&self) -> impl Iterator<Item = Handle<'_, KT>> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter(|handle| self.stack_graph[*handle].is_reference())
            .map(|handle| Handle {
                owner: self,
                handle,
            })
    }

    pub fn cursor_to_handle(&self, cursor: &Cursor<KT>) -> Option<Handle<'_, KT>> {
        for (handle, handle_cursor) in &self.cursors {
            if handle_cursor == cursor {
                return Some(Handle {
                    owner: self,
                    handle: *handle,
                });
            }
        }
        None
    }
}

pub struct Handle<'a, KT: KindTypes + 'static> {
    owner: &'a Bindings<KT>,
    handle: GraphHandle,
}

impl<KT: KindTypes + 'static> Handle<'_, KT> {
    pub fn is_definition(&self) -> bool {
        self.owner.stack_graph[self.handle].is_definition()
    }

    pub fn is_reference(&self) -> bool {
        self.owner.stack_graph[self.handle].is_reference()
    }

    pub fn get_cursor(&self) -> Option<Cursor<KT>> {
        self.owner.cursors.get(&self.handle).cloned()
    }

    pub fn get_file(&self) -> Option<&str> {
        self.owner.stack_graph[self.handle]
            .file()
            .map(|file| self.owner.stack_graph[file].name())
    }

    pub fn jump_to_definition(&self) -> Option<Self> {
        let mut partials = PartialPaths::new();
        let mut reference_paths = Vec::new();
        if self.is_reference() {
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
        }

        let mut results = BTreeSet::new();
        for reference_path in &reference_paths {
            if reference_paths
                .iter()
                .all(|other| !other.shadows(&mut partials, reference_path))
            {
                results.insert(reference_path.end_node());
            }
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

impl<KT: KindTypes + 'static> Debug for Handle<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BindingsHandle").field(&self.handle).finish()
    }
}

impl<KT: KindTypes + 'static> PartialEq for Handle<'_, KT> {
    fn eq(&self, other: &Self) -> bool {
        let our_owner: *const Bindings<KT> = self.owner;
        let other_owner: *const Bindings<KT> = other.owner;
        our_owner == other_owner && self.handle == other.handle
    }
}
