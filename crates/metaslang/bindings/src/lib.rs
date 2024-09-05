pub mod builder;

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::iter::once;
use std::sync::Arc;

use builder::{BuildResult, Selector};
use metaslang_cst::cursor::Cursor;
use metaslang_cst::KindTypes;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use semver::Version;
use stack_graphs::graph::{Node, StackGraph};
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
    selectors: HashMap<GraphHandle, Selector>,
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
            selectors: HashMap::new(),
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
        self.selectors.extend(result.selectors.drain());

        result
    }

    fn to_definition(&self, handle: GraphHandle) -> Option<Definition<'_, KT>> {
        if self.stack_graph[handle].is_definition() {
            Some(Definition {
                owner: self,
                handle,
            })
        } else {
            None
        }
    }

    pub fn all_definitions(&self) -> impl Iterator<Item = Definition<'_, KT>> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter_map(|handle| self.to_definition(handle))
    }

    fn to_reference(&self, handle: GraphHandle) -> Option<Reference<'_, KT>> {
        if self.stack_graph[handle].is_reference() {
            Some(Reference {
                owner: self,
                handle,
            })
        } else {
            None
        }
    }

    pub fn all_references(&self) -> impl Iterator<Item = Reference<'_, KT>> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter_map(|handle| self.to_reference(handle))
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
    // TODO: review the comments above
    pub fn jump_to_definition(&self) -> Option<Definition<'a, KT>> {
        let alternatives = self.resolve();
        if alternatives.len() <= 1 {
            alternatives
                .first()
                .and_then(|path| self.owner.to_definition(path.end_node()))
        } else {
            // attempt to disambiguate from all found alternatives

            // remove aliases
            let alternatives = alternatives
                .into_iter()
                .filter(|path| {
                    self.owner
                        .selectors
                        .get(&path.end_node())
                        .map_or(true, |s| !matches!(s, Selector::Alias))
                })
                .collect::<Vec<_>>();

            match alternatives.len() {
                0 => {
                    // TODO: this is an error because the actual definition
                    // is not found, but maybe we can revert back to
                    // returning the longest path?
                    panic!("More than one definition found but they are all aliases")
                }
                1 => alternatives.first().map(|path| Definition {
                    owner: self.owner,
                    handle: path.end_node(),
                }),
                _ => {
                    for (index, result) in alternatives.iter().enumerate() {
                        let definition = self.owner.to_definition(result.end_node()).unwrap();
                        let selector = self.owner.selectors.get(&result.end_node());
                        println!(
                            "  {index}. {definition} (length {length}) (selector = {selector})",
                            index = index + 1,
                            length = result.edges.len(),
                            selector =
                                selector.map_or(String::from("<none>"), |s| format!("{s:?}")),
                        );
                        let Some(selector) = selector else {
                            continue;
                        };

                        if let Selector::ParentDefinitions(related) = selector {
                            let enclosing_node = related.first().expect("at least one parent");
                            if self.owner.stack_graph[*enclosing_node].is_definition() {
                                let enclosing_def =
                                    self.owner.to_definition(*enclosing_node).unwrap();
                                println!("   Selector points to {enclosing_def}");
                                let related_sel = self.owner.selectors.get(enclosing_node);
                                if let Some(Selector::ParentReferences(parents)) = related_sel {
                                    println!("      with parents:");
                                    for parent in parents {
                                        let parent_reference =
                                            self.owner.to_reference(*parent).unwrap();
                                        let parent_definition =
                                            parent_reference.jump_to_definition().unwrap();
                                        println!(
                                            "        {parent_reference} -> {parent_definition}"
                                        );
                                    }
                                }
                            } else {
                                println!("   Virtual selector doesn't point to a definition");
                            }
                        }
                    }

                    panic!(concat!(
                        "More than one non-alias definitions found and ",
                        "disambiguation not implemented yet"
                    ));
                }
            }
        }
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
