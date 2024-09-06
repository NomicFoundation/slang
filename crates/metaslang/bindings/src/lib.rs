mod builder;
mod resolver;

use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::sync::Arc;

use builder::{BuildResult, Selector};
use metaslang_cst::cursor::Cursor;
use metaslang_cst::KindTypes;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use resolver::Resolver;
use semver::Version;
use stack_graphs::graph::StackGraph;

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
    parents: HashMap<GraphHandle, Vec<GraphHandle>>,
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
            parents: HashMap::new(),
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
        self.parents.extend(result.parents.drain());

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

    fn resolve_parents(&self, handle: GraphHandle) -> Vec<Definition<'_, KT>> {
        if let Some(parents) = self.parents.get(&handle) {
            // NOTE: the Builder ensures that all handles in the parents are
            // either definitions or references
            parents
                .iter()
                .filter_map(|handle| {
                    if self.stack_graph[*handle].is_definition() {
                        self.to_definition(*handle)
                    } else {
                        // TODO: what should we do if the parent reference
                        // cannot be resolved at this point?
                        // TODO: also, since jump_to_definition() may
                        // recursively use this method, we need to handle cycles
                        self.to_reference(*handle).unwrap().jump_to_definition()
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
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

    pub(crate) fn has_selector(&self, selector: Selector) -> bool {
        self.owner
            .selectors
            .get(&self.handle)
            .map_or(false, |s| *s == selector)
    }

    pub(crate) fn resolve_parents(&self) -> Vec<Definition<'a, KT>> {
        self.owner.resolve_parents(self.handle)
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

impl<KT: KindTypes + 'static> Debug for Definition<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<KT: KindTypes + 'static> PartialEq for Definition<'_, KT> {
    fn eq(&self, other: &Self) -> bool {
        let our_owner: *const Bindings<KT> = self.owner;
        let other_owner: *const Bindings<KT> = other.owner;
        our_owner == other_owner && self.handle == other.handle
    }
}

impl<KT: KindTypes + 'static> Eq for Definition<'_, KT> {}

impl<KT: KindTypes + 'static> Hash for Definition<'_, KT> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let owner: *const Bindings<KT> = self.owner;
        owner.hash(state);
        self.handle.hash(state);
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
        Resolver::build_for(self).first()
    }

    pub fn definitions(&self) -> Vec<Definition<'a, KT>> {
        Resolver::build_for(self).all()
    }

    pub(crate) fn resolve_parents(&self) -> Vec<Definition<'a, KT>> {
        self.owner.resolve_parents(self.handle)
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
