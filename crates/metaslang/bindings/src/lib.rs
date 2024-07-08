pub mod builder;

use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::fmt::Debug;
use std::iter::once;

use metaslang_cst::cursor::Cursor;
use metaslang_cst::KindTypes;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use semver::Version;
use stack_graphs::graph::StackGraph;
use stack_graphs::partial::PartialPaths;
use stack_graphs::stitching::{ForwardPartialPathStitcher, GraphEdgeCandidates, StitcherConfig};

type Builder<'a, KT> = builder::Builder<'a, KT>;
type GraphHandle = stack_graphs::arena::Handle<stack_graphs::graph::Node>;

pub struct Bindings<KT: KindTypes + 'static> {
    version: Version,
    graph_builder_file: File<KT>,
    functions: Functions<KT>,
    stack_graph: StackGraph,
    cursors: HashMap<GraphHandle, Cursor<KT>>,
}

impl<KT: KindTypes + 'static> Bindings<KT> {
    pub fn create(version: Version, binding_rules: &str) -> Self {
        let graph_builder_file =
            File::from_str(binding_rules).expect("Bindings stack graph builder parse error");
        let stack_graph = StackGraph::new();
        let functions = Functions::stdlib();
        let cursors = HashMap::new();

        Self {
            version,
            graph_builder_file,
            functions,
            stack_graph,
            cursors,
        }
    }

    pub fn add_file(&mut self, file_path: &str, tree_cursor: Cursor<KT>) {
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
        builder
            .build(&globals, &builder::NoCancellation, |handle, cursor| {
                self.cursors.insert(handle, cursor.clone());
            })
            .expect("Internal error while building bindings");
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

impl<KT: KindTypes + 'static> Debug for Handle<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BindingsHandle").field(&self.handle).finish()
    }
}
