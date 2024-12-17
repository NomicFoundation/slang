mod builder;
mod location;
mod resolver;

use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::rc::Rc;

use builder::BuildResult;
use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use resolver::{ResolveOptions, Resolver};
use semver::Version;
use stack_graphs::graph::StackGraph;

type Builder<'a, KT> = builder::Builder<'a, KT>;
type GraphHandle = stack_graphs::arena::Handle<stack_graphs::graph::Node>;
type FileHandle = stack_graphs::arena::Handle<stack_graphs::graph::File>;
type CursorID = usize;

pub use location::{BindingLocation, BuiltInLocation, UserFileLocation};

pub(crate) struct DefinitionBindingInfo<KT: KindTypes + 'static> {
    definiens: Option<Cursor<KT>>,
    parents: Vec<GraphHandle>,
    extension_scope: Option<GraphHandle>,
    inherit_extensions: bool,
}

pub(crate) struct ReferenceBindingInfo {
    parents: Vec<GraphHandle>,
}

pub struct BindingGraph<KT: KindTypes + 'static> {
    graph_builder_file: File<KT>,
    functions: Functions<KT>,
    stack_graph: StackGraph,
    cursors: HashMap<GraphHandle, Cursor<KT>>,
    definitions_info: HashMap<GraphHandle, DefinitionBindingInfo<KT>>,
    references_info: HashMap<GraphHandle, ReferenceBindingInfo>,
    cursor_to_definitions: HashMap<CursorID, GraphHandle>,
    cursor_to_references: HashMap<CursorID, GraphHandle>,
    extension_hooks: HashSet<GraphHandle>,
}

pub enum FileDescriptor {
    User(String),
    System(String),
}

#[derive(Debug)]
pub(crate) struct FileDescriptorError;

impl FileDescriptor {
    // Internal functions to convert a FileDescriptor to and from a string for
    // representation inside the stack graph

    pub(crate) fn as_string(&self) -> String {
        match self {
            Self::User(path) => format!("user:{path}"),
            Self::System(path) => format!("system:{path}"),
        }
    }

    pub(crate) fn try_from(value: &str) -> Result<Self, FileDescriptorError> {
        value
            .strip_prefix("user:")
            .map(|path| FileDescriptor::User(path.into()))
            .or_else(|| {
                value
                    .strip_prefix("system:")
                    .map(|path| FileDescriptor::System(path.into()))
            })
            .ok_or(FileDescriptorError)
    }

    pub(crate) fn from(value: &str) -> Self {
        Self::try_from(value)
            .unwrap_or_else(|_| panic!("{value} should be a valid file descriptor"))
    }

    pub fn get_path(&self) -> &str {
        match self {
            Self::User(path) => path,
            Self::System(path) => path,
        }
    }

    pub fn is_system(&self) -> bool {
        matches!(self, Self::System(_))
    }

    pub fn is_user(&self) -> bool {
        matches!(self, Self::User(_))
    }

    pub fn is_user_path(&self, path: &str) -> bool {
        matches!(self, Self::User(user_path) if user_path == path)
    }
}

pub trait PathResolver<KT: KindTypes + 'static> {
    fn resolve_path(&self, context_path: &str, path_to_resolve: &Cursor<KT>) -> Option<String>;
}

impl<KT: KindTypes + 'static> BindingGraph<KT> {
    pub fn create(
        version: Version,
        binding_rules: &str,
        path_resolver: Rc<dyn PathResolver<KT>>,
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
            definitions_info: HashMap::new(),
            references_info: HashMap::new(),
            cursor_to_definitions: HashMap::new(),
            cursor_to_references: HashMap::new(),
            extension_hooks: HashSet::new(),
        }
    }

    pub fn add_system_file(&mut self, file_path: &str, tree_cursor: Cursor<KT>) {
        let file_kind = FileDescriptor::System(file_path.into());
        let file = self.stack_graph.get_or_create_file(&file_kind.as_string());
        _ = self.add_file_internal(file, tree_cursor);
    }

    pub fn add_user_file(&mut self, file_path: &str, tree_cursor: Cursor<KT>) {
        let file_kind = FileDescriptor::User(file_path.into());
        let file = self.stack_graph.get_or_create_file(&file_kind.as_string());
        _ = self.add_file_internal(file, tree_cursor);
    }

    #[cfg(feature = "__private_testing_utils")]
    pub fn add_user_file_returning_graph(
        &mut self,
        file_path: &str,
        tree_cursor: Cursor<KT>,
    ) -> metaslang_graph_builder::graph::Graph<KT> {
        let file_kind = FileDescriptor::User(file_path.into());
        let file = self.stack_graph.get_or_create_file(&file_kind.as_string());
        let result = self.add_file_internal(file, tree_cursor);
        result.graph
    }

    fn add_file_internal(&mut self, file: FileHandle, tree_cursor: Cursor<KT>) -> BuildResult<KT> {
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
        self.definitions_info
            .extend(result.definitions_info.drain());
        self.references_info.extend(result.references_info.drain());
        self.extension_hooks.extend(result.extension_hooks.drain());

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

    fn resolve_handles(&self, handles: &[GraphHandle]) -> Vec<Definition<'_, KT>> {
        // NOTE: the Builder ensures that all handles in the parents are
        // either definitions or references
        handles
            .iter()
            .flat_map(|handle| {
                if self.stack_graph[*handle].is_definition() {
                    vec![self.to_definition(*handle).unwrap()]
                } else {
                    self.to_reference(*handle).unwrap().non_recursive_resolve()
                }
            })
            .collect()
    }

    pub(crate) fn is_extension_hook(&self, node_handle: GraphHandle) -> bool {
        self.extension_hooks.contains(&node_handle)
    }
}

struct DisplayCursor<'a, KT: KindTypes + 'static> {
    cursor: &'a Cursor<KT>,
    file: FileDescriptor,
}

impl<'a, KT: KindTypes + 'static> fmt::Display for DisplayCursor<'a, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offset = self.cursor.text_offset();
        write!(
            f,
            "`{}` at {}:{}:{}",
            self.cursor.node().unparse(),
            self.file.get_path(),
            offset.line + 1,
            offset.column + 1,
        )
    }
}

#[derive(Clone)]
pub struct Definition<'a, KT: KindTypes + 'static> {
    owner: &'a BindingGraph<KT>,
    handle: GraphHandle,
}

impl<'a, KT: KindTypes + 'static> Definition<'a, KT> {
    pub fn id(&self) -> usize {
        self.get_cursor().node().id()
    }

    pub fn name_location(&self) -> BindingLocation<KT> {
        match self.get_file() {
            FileDescriptor::System(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.get_cursor().to_owned())
            }
        }
    }

    pub fn definiens_location(&self) -> BindingLocation<KT> {
        match self.get_file() {
            FileDescriptor::System(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.get_definiens_cursor().to_owned())
            }
        }
    }

    pub fn get_cursor(&self) -> &Cursor<KT> {
        self.owner
            .cursors
            .get(&self.handle)
            .expect("Definition does not have a valid cursor")
    }

    pub fn get_definiens_cursor(&self) -> &Cursor<KT> {
        self.owner
            .definitions_info
            .get(&self.handle)
            .expect("Definition does not have valid binding info")
            .definiens
            .as_ref()
            .expect("Definiens does not have a valid cursor")
    }

    pub fn get_file(&self) -> FileDescriptor {
        self.owner.stack_graph[self.handle]
            .file()
            .map(|file| FileDescriptor::from(self.owner.stack_graph[file].name()))
            .expect("Definition does not have a valid file descriptor")
    }

    pub(crate) fn resolve_parents(&self) -> Vec<Definition<'a, KT>> {
        self.owner
            .definitions_info
            .get(&self.handle)
            .map(|info| &info.parents)
            .map(|handles| self.owner.resolve_handles(handles))
            .unwrap_or_default()
    }

    pub(crate) fn get_extension_scope(&self) -> Option<GraphHandle> {
        self.owner
            .definitions_info
            .get(&self.handle)
            .and_then(|info| info.extension_scope)
    }

    pub(crate) fn inherit_extensions(&self) -> bool {
        self.owner
            .definitions_info
            .get(&self.handle)
            .map_or(false, |info| info.inherit_extensions)
    }
}

impl<KT: KindTypes + 'static> Display for Definition<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "definition {}",
            DisplayCursor {
                cursor: self.get_cursor(),
                file: self.get_file()
            }
        )
    }
}

impl<KT: KindTypes + 'static> Debug for Definition<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<KT: KindTypes + 'static> PartialEq for Definition<'_, KT> {
    fn eq(&self, other: &Self) -> bool {
        let our_owner: *const BindingGraph<KT> = self.owner;
        let other_owner: *const BindingGraph<KT> = other.owner;
        our_owner == other_owner && self.handle == other.handle
    }
}

impl<KT: KindTypes + 'static> Eq for Definition<'_, KT> {}

impl<KT: KindTypes + 'static> Hash for Definition<'_, KT> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let owner: *const BindingGraph<KT> = self.owner;
        owner.hash(state);
        self.handle.hash(state);
    }
}

#[derive(Clone)]
pub struct Reference<'a, KT: KindTypes + 'static> {
    owner: &'a BindingGraph<KT>,
    handle: GraphHandle,
}

pub enum ResolutionError<'a, KT: KindTypes + 'static> {
    Unresolved,
    AmbiguousDefinitions(Vec<Definition<'a, KT>>),
}

impl<'a, KT: KindTypes + 'static> Reference<'a, KT> {
    pub fn id(&self) -> usize {
        self.get_cursor().node().id()
    }

    pub fn location(&self) -> BindingLocation<KT> {
        match self.get_file() {
            FileDescriptor::System(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.get_cursor().to_owned())
            }
        }
    }

    pub fn get_cursor(&self) -> &Cursor<KT> {
        self.owner
            .cursors
            .get(&self.handle)
            .expect("Reference does not have a valid cursor")
    }

    pub fn get_file(&self) -> FileDescriptor {
        self.owner.stack_graph[self.handle]
            .file()
            .map(|file| FileDescriptor::from(self.owner.stack_graph[file].name()))
            .expect("Reference does not have a valid file descriptor")
    }

    pub fn definitions(&self) -> Vec<Definition<'a, KT>> {
        Resolver::build_for(self, ResolveOptions::Full).all()
    }

    pub(crate) fn non_recursive_resolve(&self) -> Vec<Definition<'a, KT>> {
        // This was likely originated from a full resolution call, so cut
        // recursion here by restricting the resolution algorithm.
        Resolver::build_for(self, ResolveOptions::NonRecursive).all()
    }

    pub(crate) fn resolve_parents(&self) -> Vec<Definition<'a, KT>> {
        self.owner
            .references_info
            .get(&self.handle)
            .map(|info| &info.parents)
            .map(|handles| self.owner.resolve_handles(handles))
            .unwrap_or_default()
    }
}

impl<KT: KindTypes + 'static> Display for Reference<'_, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "reference {}",
            DisplayCursor {
                cursor: self.get_cursor(),
                file: self.get_file()
            }
        )
    }
}

impl<KT: KindTypes + 'static> PartialEq for Reference<'_, KT> {
    fn eq(&self, other: &Self) -> bool {
        let our_owner: *const BindingGraph<KT> = self.owner;
        let other_owner: *const BindingGraph<KT> = other.owner;
        our_owner == other_owner && self.handle == other.handle
    }
}
