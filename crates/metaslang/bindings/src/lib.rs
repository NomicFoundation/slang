pub mod builder;

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::iter::once;
use std::sync::Arc;

use builder::BuildResult;
use metaslang_cst::cursor::Cursor;
use metaslang_cst::KindTypes;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use semver::Version;
use stack_graphs::graph::StackGraph;
use stack_graphs::partial::{PartialPath, PartialPaths};
use stack_graphs::stitching::{
    Appendable, ForwardPartialPathStitcher, GraphEdgeCandidates, StitcherConfig,
};

type Builder<'a, KT> = builder::Builder<'a, KT>;
type GraphHandle = stack_graphs::arena::Handle<stack_graphs::graph::Node>;
type CursorID = usize;

pub struct Bindings<KT: KindTypes + 'static> {
    graph_builder_file: File<KT>,
    functions: Functions<KT>,
    stack_graph: StackGraph,
    cursors: HashMap<GraphHandle, Cursor<KT>>,
    definiens: HashMap<GraphHandle, Cursor<KT>>,
    cursor_to_definitions: HashMap<CursorID, GraphHandle>,
    cursor_to_references: HashMap<CursorID, GraphHandle>,
}

pub trait PathResolver {
    fn resolve_path(&self, context_path: &str, path_to_resolve: &str) -> Option<String>;
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

        Self {
            graph_builder_file,
            functions,
            stack_graph,
            cursors: HashMap::new(),
            definiens: HashMap::new(),
            cursor_to_definitions: HashMap::new(),
            cursor_to_references: HashMap::new(),
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
        let result = self.add_file_internal(file_path, tree_cursor);
        result.graph
    }

    fn add_file_internal(&mut self, file_path: &str, tree_cursor: Cursor<KT>) -> BuildResult<KT> {
        let file = self.stack_graph.get_or_create_file(file_path);

        let builder = Builder::new(
            &self.graph_builder_file,
            &self.functions,
            &mut self.stack_graph,
            file,
            tree_cursor,
        );
        let mut result = builder
            .build(&builder::NoCancellation)
            .expect("Internal error while building bindings");

        for (handle, cursor) in result.cursors.drain() {
            let cursor_id = cursor.node().id();
            if self.stack_graph[handle].is_definition() {
                self.cursor_to_definitions.insert(cursor_id, handle);
            } else {
                self.cursor_to_references.insert(cursor_id, handle);
            }
            self.cursors.insert(handle, cursor);
        }
        self.definiens.extend(result.definiens.drain());

        result
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
        let cursor_id = cursor.node().id();
        self.cursor_to_definitions
            .get(&cursor_id)
            .map(|handle| Definition {
                owner: self,
                handle: *handle,
            })
    }

    pub fn reference_at(&self, cursor: &Cursor<KT>) -> Option<Reference<'_, KT>> {
        let cursor_id = cursor.node().id();
        self.cursor_to_references
            .get(&cursor_id)
            .map(|handle| Reference {
                owner: self,
                handle: *handle,
            })
    }
}

struct DisplayCursor<'a, KT: KindTypes + 'static> {
    cursor: &'a Cursor<KT>,
    file: Option<&'a str>,
}

impl<'a, KT: KindTypes + 'static> fmt::Display for DisplayCursor<'a, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offset = self.cursor.text_offset();
        write!(
            f,
            "`{}` at {}:{}:{}",
            self.cursor.node().unparse(),
            self.file.unwrap_or("<unknown_file>"),
            offset.line + 1,
            offset.column + 1,
        )
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

    pub fn get_definiens_cursor(&self) -> Option<Cursor<KT>> {
        self.owner.definiens.get(&self.handle).cloned()
    }

    pub fn get_file(&self) -> Option<&'a str> {
        self.owner.stack_graph[self.handle]
            .file()
            .map(|file| self.owner.stack_graph[file].name())
    }
}

impl<KT: KindTypes + 'static> Display for Definition<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(cursor) = self.get_cursor() {
            write!(
                f,
                "definition {}",
                DisplayCursor {
                    cursor: &cursor,
                    file: self.get_file()
                }
            )
        } else {
            write!(f, "{}", self.handle.display(&self.owner.stack_graph))
        }
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

    /// Attempt to resolve the reference and find its definition. If the
    /// reference cannot be resolved in the current state of the bindings (eg.
    /// you may still need to add an imported file), this function will return
    /// `None`. Otherwise, it will always return the definition with the
    /// longest, not shadowed, path in the underlying stack graph. This is to
    /// ensure that we always get the actual definition of some identifier
    /// reference, and not an intermediate result such as an import alias.
    ///
    /// There are multiple reasons why a reference may resolve to more than one
    /// definition in well formed and valid code. For example:
    ///
    /// 1. Variable shadowing: this should be resolved in the rules file by
    ///    setting the precedence attribute to the appropriate edges.
    ///
    /// 2. Destructuring imports (with or without aliases): these are
    ///    represented in the graph as intermediate definition nodes along the
    ///    path to the actual definition; hence why this function will return
    ///    the longest path available.
    ///
    /// 3. Overriden virtual methods: a reference will find valid paths to all
    ///    available definitions in the class hierarchy.
    ///
    pub fn jump_to_definition(&self) -> Option<Definition<'a, KT>> {
        let definitions = self.resolve();
        if definitions.len() > 1 {
            println!(
                "WARN: More than one definition found for {self}, will return the longest path",
            );

            for (index, result) in definitions.iter().enumerate() {
                let definition = Definition {
                    owner: self.owner,
                    handle: result.end_node(),
                };
                println!(
                    "  {index}. {definition} (length {length})",
                    index = index + 1,
                    length = result.edges.len(),
                );
            }
        }
        definitions.first().map(|path| Definition {
            owner: self.owner,
            handle: path.end_node(),
        })
    }

    pub fn definitions(&self) -> Vec<Definition<'a, KT>> {
        self.resolve()
            .into_iter()
            .map(|path| Definition {
                owner: self.owner,
                handle: path.end_node(),
            })
            .collect()
    }

    fn resolve(&self) -> Vec<PartialPath> {
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
                results.push(reference_path.clone());
            }
        }
        results.sort_by(|a, b| b.edges.len().cmp(&a.edges.len()));
        results
    }
}

impl<KT: KindTypes + 'static> Display for Reference<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(cursor) = self.get_cursor() {
            write!(
                f,
                "reference {}",
                DisplayCursor {
                    cursor: &cursor,
                    file: self.get_file()
                }
            )
        } else {
            write!(f, "{}", self.handle.display(&self.owner.stack_graph))
        }
    }
}

impl<KT: KindTypes + 'static> PartialEq for Reference<'_, KT> {
    fn eq(&self, other: &Self) -> bool {
        let our_owner: *const Bindings<KT> = self.owner;
        let other_owner: *const Bindings<KT> = other.owner;
        our_owner == other_owner && self.handle == other.handle
    }
}
