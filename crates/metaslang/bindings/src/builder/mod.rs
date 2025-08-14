mod loader;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::rc::Rc;

use loader::{LoadResult, Loader};
use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;
use metaslang_cst::nodes::NodeId;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use semver::Version;
use stack_graphs::graph::{NodeID as GraphNodeId, StackGraph};

pub type GraphHandle = stack_graphs::arena::Handle<stack_graphs::graph::Node>;
pub(crate) type FileHandle = stack_graphs::arena::Handle<stack_graphs::graph::File>;

pub use crate::graph::BindingGraph;

pub(crate) struct DefinitionBindingInfo<KT: KindTypes + 'static> {
    cursor: Cursor<KT>,
    definiens: Cursor<KT>,
    parents: Vec<GraphHandle>,
    extension_scope: Option<GraphHandle>,
    inherit_extensions: bool,
}

pub(crate) struct ReferenceBindingInfo<KT: KindTypes + 'static> {
    cursor: Cursor<KT>,
    parents: Vec<GraphHandle>,
}

pub struct BindingGraphBuilder<KT: KindTypes + 'static> {
    graph_builder_file: File<KT>,
    functions: Functions<KT>,
    graph: ExtendedStackGraph<KT>,
}

pub(crate) struct ExtendedStackGraph<KT: KindTypes + 'static> {
    pub(crate) stack_graph: StackGraph,
    definitions_info: HashMap<GraphHandle, DefinitionBindingInfo<KT>>,
    references_info: HashMap<GraphHandle, ReferenceBindingInfo<KT>>,
    definitions_by_node_id: HashMap<NodeId, GraphHandle>,
    references_by_node_id: HashMap<NodeId, GraphHandle>,
    extension_hooks: HashSet<GraphHandle>,
}

#[derive(Clone)]
pub enum FileDescriptor {
    User(String),
    BuiltIns(String),
}

#[derive(Debug)]
pub(crate) struct FileDescriptorError;

impl FileDescriptor {
    // Internal functions to convert a FileDescriptor to and from a string for
    // representation inside the stack graph

    pub(crate) fn as_string(&self) -> String {
        match self {
            Self::User(path) => format!("user:{path}"),
            Self::BuiltIns(path) => format!("built-ins:{path}"),
        }
    }

    pub(crate) fn try_from(value: &str) -> Result<Self, FileDescriptorError> {
        value
            .strip_prefix("user:")
            .map(|path| FileDescriptor::User(path.into()))
            .or_else(|| {
                value
                    .strip_prefix("built-ins:")
                    .map(|path| FileDescriptor::BuiltIns(path.into()))
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
            Self::BuiltIns(path) => path,
        }
    }

    #[cfg(feature = "__private_testing_utils")]
    pub fn is_built_ins(&self) -> bool {
        matches!(self, Self::BuiltIns(_))
    }

    pub fn is_user(&self) -> bool {
        matches!(self, Self::User(_))
    }

    #[cfg(feature = "__private_testing_utils")]
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
            graph: ExtendedStackGraph {
                stack_graph,
                definitions_info: HashMap::new(),
                references_info: HashMap::new(),
                definitions_by_node_id: HashMap::new(),
                references_by_node_id: HashMap::new(),
                extension_hooks: HashSet::new(),
            },
        }
    }

    pub fn add_user_file(&mut self, file_path: &str, tree_cursor: Cursor<KT>) {
        let file_kind = FileDescriptor::User(file_path.into());
        let file = self.graph.get_or_create_file(&file_kind);
        _ = self.add_file_internal(file, tree_cursor);
    }

    pub fn build_built_ins_file(
        &mut self,
        file_path: &str,
        cursor: Cursor<KT>,
    ) -> FileGraphBuilder<'_, KT> {
        let file_kind = FileDescriptor::BuiltIns(file_path.into());
        let file = self.graph.get_or_create_file(&file_kind);
        FileGraphBuilder::new(&mut self.graph, file, cursor)
    }

    #[cfg(feature = "__private_testing_utils")]
    pub fn add_user_file_returning_graph(
        &mut self,
        file_path: &str,
        tree_cursor: Cursor<KT>,
    ) -> metaslang_graph_builder::graph::Graph<KT> {
        let file_kind = FileDescriptor::User(file_path.into());
        let file = self.graph.get_or_create_file(&file_kind);
        let result = self.add_file_internal(file, tree_cursor);
        result.graph
    }

    fn add_file_internal(&mut self, file: FileHandle, tree_cursor: Cursor<KT>) -> LoadResult<KT> {
        let loader = Loader::new(
            &self.graph_builder_file,
            &self.functions,
            &mut self.graph.stack_graph,
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
                let name_node_id = definition_info.cursor.node().id();
                self.graph
                    .definitions_by_node_id
                    .insert(name_node_id, *handle);
                let definiens_node_id = definition_info.definiens.node().id();
                self.graph
                    .definitions_by_node_id
                    .insert(definiens_node_id, *handle);
            });
        result
            .references_info
            .iter()
            .for_each(|(handle, reference_info)| {
                let node_id = reference_info.cursor.node().id();
                self.graph.references_by_node_id.insert(node_id, *handle);
            });

        self.graph
            .definitions_info
            .extend(result.definitions_info.drain());
        self.graph
            .references_info
            .extend(result.references_info.drain());
        self.graph
            .extension_hooks
            .extend(result.extension_hooks.drain());

        result
    }

    pub fn build(self) -> Rc<BindingGraph<KT>> {
        BindingGraph::build(self.graph)
    }
}

impl<KT: KindTypes + 'static> ExtendedStackGraph<KT> {
    fn get_or_create_file(&mut self, file_kind: &FileDescriptor) -> FileHandle {
        self.stack_graph.get_or_create_file(&file_kind.as_string())
    }

    #[cfg(feature = "__private_testing_utils")]
    pub(crate) fn iter_definitions(&self) -> impl Iterator<Item = GraphHandle> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter(|handle| self.is_definition(*handle))
    }

    pub(crate) fn iter_references(&self) -> impl Iterator<Item = GraphHandle> + '_ {
        self.stack_graph
            .iter_nodes()
            .filter(|handle| self.is_reference(*handle))
    }

    pub(crate) fn iter_files(&self) -> impl Iterator<Item = FileHandle> + '_ {
        self.stack_graph.iter_files()
    }

    pub(crate) fn definition_by_node_id(&self, node_id: NodeId) -> Option<GraphHandle> {
        self.definitions_by_node_id.get(&node_id).copied()
    }

    pub(crate) fn reference_by_node_id(&self, node_id: NodeId) -> Option<GraphHandle> {
        self.references_by_node_id.get(&node_id).copied()
    }

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

    pub(crate) fn get_file_descriptor(&self, handle: GraphHandle) -> Option<FileDescriptor> {
        self.stack_graph[handle]
            .file()
            .map(|file| FileDescriptor::from(self.stack_graph[file].name()))
    }

    pub(crate) fn is_extension_hook(&self, node_handle: GraphHandle) -> bool {
        self.extension_hooks.contains(&node_handle)
    }

    pub(crate) fn get_cursor(&self, handle: GraphHandle) -> Option<&Cursor<KT>> {
        if self.is_definition(handle) {
            self.definitions_info.get(&handle).map(|info| &info.cursor)
        } else if self.is_reference(handle) {
            self.references_info.get(&handle).map(|info| &info.cursor)
        } else {
            None
        }
    }

    pub(crate) fn get_definiens_cursor(&self, handle: GraphHandle) -> Option<&Cursor<KT>> {
        if self.is_definition(handle) {
            self.definitions_info
                .get(&handle)
                .map(|info| &info.definiens)
        } else {
            None
        }
    }
}

pub struct FileGraphBuilder<'a, KT: KindTypes + 'static> {
    graph: &'a mut ExtendedStackGraph<KT>,
    file: FileHandle,
    cursor: Cursor<KT>,
}

impl<'a, KT: KindTypes + 'static> FileGraphBuilder<'a, KT> {
    fn new(graph: &'a mut ExtendedStackGraph<KT>, file: FileHandle, cursor: Cursor<KT>) -> Self {
        FileGraphBuilder {
            graph,
            file,
            cursor,
        }
    }

    pub fn root_node(&self) -> GraphHandle {
        self.graph
            .stack_graph
            .node_for_id(GraphNodeId::root())
            .expect("Root node not found in stack graph")
    }

    pub fn new_scope_node(&mut self, extension_hook: bool) -> GraphHandle {
        let id = self.graph.stack_graph.new_node_id(self.file);
        let scope_node = self
            .graph
            .stack_graph
            .add_scope_node(id, extension_hook)
            .expect("Failed to add scope node");
        if extension_hook {
            self.graph.extension_hooks.insert(scope_node);
        }
        scope_node
    }

    pub fn new_pop_symbol_node<S: AsRef<str> + ?Sized>(
        &mut self,
        symbol: &S,
        is_definition: bool,
    ) -> GraphHandle {
        let id = self.graph.stack_graph.new_node_id(self.file);
        let symbol = self.graph.stack_graph.add_symbol(symbol);
        let node = self
            .graph
            .stack_graph
            .add_pop_symbol_node(id, symbol, is_definition)
            .expect("Failed to add pop symbol node");
        if is_definition {
            self.graph.definitions_info.insert(
                node,
                DefinitionBindingInfo {
                    cursor: self.cursor.clone(),
                    definiens: self.cursor.clone(),
                    parents: Vec::new(),
                    extension_scope: None,
                    inherit_extensions: false,
                },
            );
        }
        node
    }

    pub fn new_push_symbol_node<S: AsRef<str> + ?Sized>(
        &mut self,
        symbol: &S,
        is_reference: bool,
    ) -> GraphHandle {
        let id = self.graph.stack_graph.new_node_id(self.file);
        let symbol = self.graph.stack_graph.add_symbol(symbol);
        let node = self
            .graph
            .stack_graph
            .add_push_symbol_node(id, symbol, is_reference)
            .expect("Failed to add push symbol node");
        if is_reference {
            self.graph.references_info.insert(
                node,
                ReferenceBindingInfo {
                    cursor: self.cursor.clone(),
                    parents: Vec::new(),
                },
            );
        }
        node
    }

    pub fn edge(&mut self, source: GraphHandle, sink: GraphHandle) {
        self.graph.stack_graph.add_edge(source, sink, 0);
    }
}

pub struct ScopeGraphBuilder {
    resolution_scope: GraphHandle,
    defs: GraphHandle,
    typeof_refs: HashMap<String, GraphHandle>,
}

impl ScopeGraphBuilder {
    pub fn new<KT: KindTypes + 'static>(
        builder: &mut FileGraphBuilder<'_, KT>,
        guard_symbol: &str,
        root_node: GraphHandle,
        parent_scope: Option<GraphHandle>,
    ) -> Self {
        let resolution_scope = builder.new_scope_node(true);

        let guard = builder.new_pop_symbol_node(guard_symbol, false);
        let defs = builder.new_scope_node(false);
        builder.edge(root_node, guard);
        builder.edge(guard, defs);
        builder.edge(resolution_scope, defs);

        if let Some(parent_scope) = parent_scope {
            builder.edge(resolution_scope, parent_scope);
        }

        Self {
            resolution_scope,
            defs,
            typeof_refs: HashMap::new(),
        }
    }

    #[must_use]
    pub fn new_context<KT: KindTypes + 'static>(
        &self,
        builder: &mut FileGraphBuilder<'_, KT>,
        guard_symbol: &str,
    ) -> Self {
        Self::new(builder, guard_symbol, self.defs, None)
    }

    fn typeof_reference<KT: KindTypes + 'static>(
        &mut self,
        builder: &mut FileGraphBuilder<'_, KT>,
        symbol: &str,
    ) -> GraphHandle {
        if let Some(handle) = self.typeof_refs.get(symbol) {
            *handle
        } else {
            let r#typeof = builder.new_push_symbol_node("@typeof", false);
            let r#type = builder.new_push_symbol_node(symbol, false);
            builder.edge(r#typeof, r#type);
            builder.edge(r#type, self.resolution_scope);
            self.typeof_refs.insert(symbol.to_owned(), r#typeof);
            r#typeof
        }
    }

    pub fn define_function<KT: KindTypes + 'static>(
        &mut self,
        builder: &mut FileGraphBuilder<'_, KT>,
        identifier: &str,
        return_type: Option<&str>,
    ) {
        let function_def = builder.new_pop_symbol_node(identifier, true);
        builder.edge(self.defs, function_def);

        let function_typeof = self.typeof_reference(builder, "%Function");
        builder.edge(function_def, function_typeof);

        let call = builder.new_pop_symbol_node("()", false);
        builder.edge(function_def, call);

        if let Some(return_type) = return_type {
            let return_typeof = self.typeof_reference(builder, return_type);
            builder.edge(call, return_typeof);
        }
    }

    pub fn define_field<KT: KindTypes + 'static>(
        &mut self,
        builder: &mut FileGraphBuilder<'_, KT>,
        identifier: &str,
        field_type: &str,
    ) {
        let field_def = builder.new_pop_symbol_node(identifier, true);
        builder.edge(self.defs, field_def);

        let field_typeof = self.typeof_reference(builder, field_type);
        builder.edge(field_def, field_typeof);
    }

    #[must_use]
    pub fn define_type<KT: KindTypes + 'static>(
        &mut self,
        builder: &mut FileGraphBuilder<'_, KT>,
        identifier: &str,
    ) -> Self {
        let type_def = builder.new_pop_symbol_node(identifier, true);
        builder.edge(self.defs, type_def);
        let typeof_def = builder.new_pop_symbol_node("@typeof", false);
        builder.edge(type_def, typeof_def);
        let call_def = builder.new_pop_symbol_node("()", false);
        builder.edge(type_def, call_def);
        let dot = builder.new_pop_symbol_node(".", false);
        builder.edge(typeof_def, dot);
        builder.edge(call_def, dot);

        let defs = builder.new_scope_node(false);
        builder.edge(dot, defs);

        Self {
            resolution_scope: self.resolution_scope,
            defs,
            typeof_refs: HashMap::new(),
        }
    }
}
