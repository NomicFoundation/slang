mod loader;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::rc::Rc;

use loader::{LoadResult, Loader};
use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use semver::Version;
use stack_graphs::graph::StackGraph;

pub(crate) type GraphHandle = stack_graphs::arena::Handle<stack_graphs::graph::Node>;
pub(crate) type FileHandle = stack_graphs::arena::Handle<stack_graphs::graph::File>;
pub(crate) type CursorID = usize;

pub use crate::graph::BindingGraph;

pub(crate) struct DefinitionBindingInfo<KT: KindTypes + 'static> {
    pub(crate) cursor: Cursor<KT>,
    pub(crate) definiens: Cursor<KT>,
    parents: Vec<GraphHandle>,
    extension_scope: Option<GraphHandle>,
    inherit_extensions: bool,
}

pub(crate) struct ReferenceBindingInfo<KT: KindTypes + 'static> {
    pub(crate) cursor: Cursor<KT>,
    parents: Vec<GraphHandle>,
}

pub struct BindingGraphBuilder<KT: KindTypes + 'static> {
    graph_builder_file: File<KT>,
    functions: Functions<KT>,
    pub(crate) info: BindingInfo<KT>,
}

pub(crate) struct BindingInfo<KT: KindTypes + 'static> {
    pub(crate) stack_graph: StackGraph,
    pub(crate) definitions_info: HashMap<GraphHandle, DefinitionBindingInfo<KT>>,
    pub(crate) references_info: HashMap<GraphHandle, ReferenceBindingInfo<KT>>,
    pub(crate) cursor_to_definitions: HashMap<CursorID, GraphHandle>,
    pub(crate) cursor_to_references: HashMap<CursorID, GraphHandle>,
    extension_hooks: HashSet<GraphHandle>,
}

#[derive(Clone)]
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

impl<KT: KindTypes + 'static> BindingGraphBuilder<KT> {
    pub fn create(
        version: Version,
        binding_rules: &str,
        path_resolver: Rc<dyn PathResolver<KT>>,
    ) -> Self {
        let graph_builder_file =
            File::from_str(binding_rules).expect("Bindings stack graph builder parse error");
        let stack_graph = StackGraph::new();
        let functions = loader::default_functions(version, path_resolver);

        Self {
            graph_builder_file,
            functions,
            info: BindingInfo {
                stack_graph,
                definitions_info: HashMap::new(),
                references_info: HashMap::new(),
                cursor_to_definitions: HashMap::new(),
                cursor_to_references: HashMap::new(),
                extension_hooks: HashSet::new(),
            },
        }
    }

    pub fn add_system_file(&mut self, file_path: &str, tree_cursor: Cursor<KT>) {
        let file_kind = FileDescriptor::System(file_path.into());
        let file = self
            .info
            .stack_graph
            .get_or_create_file(&file_kind.as_string());
        _ = self.add_file_internal(file, tree_cursor);
    }

    pub fn add_user_file(&mut self, file_path: &str, tree_cursor: Cursor<KT>) {
        let file_kind = FileDescriptor::User(file_path.into());
        let file = self
            .info
            .stack_graph
            .get_or_create_file(&file_kind.as_string());
        _ = self.add_file_internal(file, tree_cursor);
    }

    #[cfg(feature = "__private_testing_utils")]
    pub fn add_user_file_returning_graph(
        &mut self,
        file_path: &str,
        tree_cursor: Cursor<KT>,
    ) -> metaslang_graph_builder::graph::Graph<KT> {
        let file_kind = FileDescriptor::User(file_path.into());
        let file = self
            .info
            .stack_graph
            .get_or_create_file(&file_kind.as_string());
        let result = self.add_file_internal(file, tree_cursor);
        result.graph
    }

    fn add_file_internal(&mut self, file: FileHandle, tree_cursor: Cursor<KT>) -> LoadResult<KT> {
        let loader = Loader::new(
            &self.graph_builder_file,
            &self.functions,
            &mut self.info.stack_graph,
            file,
            tree_cursor,
        );
        let mut result = loader
            .execute(&loader::NoCancellation)
            .expect("Internal error while building bindings");

        result
            .definitions_info
            .iter()
            .for_each(|(handle, definition_info)| {
                let cursor_id = definition_info.cursor.node().id();
                self.info.cursor_to_definitions.insert(cursor_id, *handle);
            });
        result
            .references_info
            .iter()
            .for_each(|(handle, reference_info)| {
                let cursor_id = reference_info.cursor.node().id();
                self.info.cursor_to_references.insert(cursor_id, *handle);
            });

        self.info
            .definitions_info
            .extend(result.definitions_info.drain());
        self.info
            .references_info
            .extend(result.references_info.drain());
        self.info
            .extension_hooks
            .extend(result.extension_hooks.drain());

        result
    }

    pub fn build(self) -> Rc<BindingGraph<KT>> {
        BindingGraph::build(self.info)
    }
}

impl<KT: KindTypes + 'static> BindingInfo<KT> {
    pub(crate) fn get_parents(&self, handle: GraphHandle) -> Vec<GraphHandle> {
        if self.is_definition(handle) {
            self.definitions_info
                .get(&handle)
                .map(|info| info.parents.clone())
                .unwrap_or_default()
        } else {
            self.references_info
                .get(&handle)
                .map(|info| info.parents.clone())
                .unwrap_or_default()
        }
    }

    pub(crate) fn is_definition(&self, handle: GraphHandle) -> bool {
        self.stack_graph[handle].is_definition()
    }

    pub(crate) fn is_reference(&self, handle: GraphHandle) -> bool {
        self.stack_graph[handle].is_reference()
    }

    pub(crate) fn get_extension_scope(&self, handle: GraphHandle) -> Option<GraphHandle> {
        self.definitions_info
            .get(&handle)
            .and_then(|info| info.extension_scope)
    }

    pub(crate) fn inherits_extensions(&self, handle: GraphHandle) -> bool {
        self.definitions_info
            .get(&handle)
            .is_some_and(|info| info.inherit_extensions)
    }

    pub(crate) fn get_file(&self, handle: GraphHandle) -> Option<FileDescriptor> {
        self.stack_graph[handle]
            .file()
            .map(|file| FileDescriptor::from(self.stack_graph[file].name()))
    }

    pub(crate) fn is_extension_hook(&self, node_handle: GraphHandle) -> bool {
        self.extension_hooks.contains(&node_handle)
    }
}
