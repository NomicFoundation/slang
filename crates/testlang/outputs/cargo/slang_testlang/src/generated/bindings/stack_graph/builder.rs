// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use metaslang_graph_builder::graph::SyntaxNodeRef;
use metaslang_graph_builder::ExecutionConfig;
use once_cell::sync::Lazy;
use stack_graphs::arena::Handle;
use stack_graphs::graph::{File, Node, NodeID, StackGraph};
use thiserror::Error;

use super::cancellation::CancellationFlag;
use super::StackGraphLanguage;
use crate::bindings::{Edge, ExecutionError, Graph, GraphNode, GraphNodeRef, Value, Variables};
use crate::cursor::Cursor;

// Node type values
static DROP_SCOPES_TYPE: &str = "drop_scopes";
static POP_SCOPED_SYMBOL_TYPE: &str = "pop_scoped_symbol";
static POP_SYMBOL_TYPE: &str = "pop_symbol";
static PUSH_SCOPED_SYMBOL_TYPE: &str = "push_scoped_symbol";
static PUSH_SYMBOL_TYPE: &str = "push_symbol";
static SCOPE_TYPE: &str = "scope";

// Node attribute names
static DEBUG_ATTR_PREFIX: &str = "debug_";
static DEFINIENS_NODE_ATTR: &str = "definiens_node";
static EMPTY_SOURCE_SPAN_ATTR: &str = "empty_source_span";
static IS_DEFINITION_ATTR: &str = "is_definition";
static IS_ENDPOINT_ATTR: &str = "is_endpoint";
static IS_EXPORTED_ATTR: &str = "is_exported";
static IS_REFERENCE_ATTR: &str = "is_reference";
static SCOPE_ATTR: &str = "scope";
static SOURCE_NODE_ATTR: &str = "source_node";
static SYMBOL_ATTR: &str = "symbol";
static SYNTAX_TYPE_ATTR: &str = "syntax_type";
static TYPE_ATTR: &str = "type";

// Expected attributes per node type
static POP_SCOPED_SYMBOL_ATTRS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        TYPE_ATTR,
        SYMBOL_ATTR,
        IS_DEFINITION_ATTR,
        DEFINIENS_NODE_ATTR,
        SYNTAX_TYPE_ATTR,
    ])
});
static POP_SYMBOL_ATTRS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        TYPE_ATTR,
        SYMBOL_ATTR,
        IS_DEFINITION_ATTR,
        DEFINIENS_NODE_ATTR,
        SYNTAX_TYPE_ATTR,
    ])
});
static PUSH_SCOPED_SYMBOL_ATTRS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| HashSet::from([TYPE_ATTR, SYMBOL_ATTR, SCOPE_ATTR, IS_REFERENCE_ATTR]));
static PUSH_SYMBOL_ATTRS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| HashSet::from([TYPE_ATTR, SYMBOL_ATTR, IS_REFERENCE_ATTR]));
static SCOPE_ATTRS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| HashSet::from([TYPE_ATTR, IS_EXPORTED_ATTR, IS_ENDPOINT_ATTR]));

// Edge attribute names
static PRECEDENCE_ATTR: &str = "precedence";

// Global variables
/// Name of the variable used to pass the root node.
pub const ROOT_NODE_VAR: &str = "ROOT_NODE";
/// Name of the variable used to pass the jump-to-scope node.
pub const JUMP_TO_SCOPE_NODE_VAR: &str = "JUMP_TO_SCOPE_NODE";
/// Name of the variable used to pass the file path.
/// If a root path is given, it should be a descendant the root path.
pub const FILE_PATH_VAR: &str = "FILE_PATH";
/// Name of the variable used to pass the root path.
/// If given, should be an ancestor of the file path.
pub const ROOT_PATH_VAR: &str = "ROOT_PATH";

pub struct Builder<'a> {
    sgl: &'a StackGraphLanguage,
    pub stack_graph: &'a mut StackGraph,
    file: Handle<File>,
    tree_cursor: Cursor,
    pub graph: Graph,
    remapped_nodes: HashMap<usize, NodeID>,
    injected_node_count: usize,
    nodes_to_syntax: HashMap<Handle<Node>, SyntaxNodeRef>,
}

impl<'a> Builder<'a> {
    pub fn new(
        sgl: &'a StackGraphLanguage,
        stack_graph: &'a mut StackGraph,
        file: Handle<File>,
        tree_cursor: Cursor,
    ) -> Self {
        Builder {
            sgl,
            stack_graph,
            file,
            tree_cursor,
            graph: Graph::new(),
            remapped_nodes: HashMap::new(),
            injected_node_count: 0,
            nodes_to_syntax: HashMap::new(),
        }
    }

    /// Executes this builder.
    pub fn build(
        &mut self,
        globals: &'a Variables<'a>,
        cancellation_flag: &dyn CancellationFlag,
    ) -> Result<(), BuildError> {
        let mut globals = Variables::nested(globals);

        if globals.get(&ROOT_NODE_VAR.into()).is_none() {
            let root_node = self.inject_node(NodeID::root());
            globals
                .add(ROOT_NODE_VAR.into(), root_node.into())
                .expect("Failed to set ROOT_NODE");
        }

        let jump_to_scope_node = self.inject_node(NodeID::jump_to());
        globals
            .add(JUMP_TO_SCOPE_NODE_VAR.into(), jump_to_scope_node.into())
            .expect("Failed to set JUMP_TO_SCOPE_NODE");

        let config = ExecutionConfig::new(&self.sgl.functions, &globals)
            .lazy(true)
            .debug_attributes(
                [DEBUG_ATTR_PREFIX, "msgb_location"]
                    .concat()
                    .as_str()
                    .into(),
                [DEBUG_ATTR_PREFIX, "msgb_variable"]
                    .concat()
                    .as_str()
                    .into(),
                [DEBUG_ATTR_PREFIX, "msgb_match_node"]
                    .concat()
                    .as_str()
                    .into(),
            );

        self.sgl.msgb.execute_into(
            &mut self.graph,
            &self.tree_cursor,
            &config,
            &(cancellation_flag as &dyn CancellationFlag),
        )?;

        self.load(cancellation_flag)
    }

    /// Create a graph node to represent the stack graph node. It is the callers responsibility to
    /// ensure the stack graph node exists.
    pub fn inject_node(&mut self, id: NodeID) -> GraphNodeRef {
        let node = self.graph.add_graph_node();
        self.remapped_nodes.insert(node.index(), id);
        self.injected_node_count += 1;
        node
    }

    pub fn node_handle_to_syntax_ref(&self, node_handle: Handle<Node>) -> Option<&SyntaxNodeRef> {
        self.nodes_to_syntax.get(&node_handle)
    }
}

/// An error that can occur while loading a stack graph from a TSG file
#[derive(Debug, Error)]
pub enum BuildError {
    #[error("{0}")]
    Cancelled(&'static str),
    #[error("Missing ‘type’ attribute on graph node")]
    MissingNodeType(GraphNodeRef),
    #[error("Missing ‘symbol’ attribute on graph node")]
    MissingSymbol(GraphNodeRef),
    #[error("Missing ‘scope’ attribute on graph node")]
    MissingScope(GraphNodeRef),
    #[error("Unknown ‘{0}’ flag type {1}")]
    UnknownFlagType(String, String),
    #[error("Unknown node type {0}")]
    UnknownNodeType(String),
    #[error("Unknown symbol type {0}")]
    UnknownSymbolType(String),
    #[error(transparent)]
    ExecutionError(ExecutionError),
    #[error("Error converting shorthand ‘{0}’ on {1} with value {2}")]
    ConversionError(String, String, String),
    #[error("Expected exported symbol scope in {0}, got {1}")]
    SymbolScopeError(String, String),
}

impl From<stack_graphs::CancellationError> for BuildError {
    fn from(value: stack_graphs::CancellationError) -> Self {
        Self::Cancelled(value.0)
    }
}

impl From<ExecutionError> for BuildError {
    fn from(value: ExecutionError) -> Self {
        match value {
            ExecutionError::Cancelled(err) => Self::Cancelled(err.0),
            err => Self::ExecutionError(err),
        }
    }
}

impl BuildError {
    pub fn display_pretty<'a>(
        &'a self,
        source_path: &'a Path,
        source: &'a str,
        tsg_path: &'a Path,
        tsg: &'a str,
    ) -> impl std::fmt::Display + 'a {
        DisplayBuildErrorPretty {
            error: self,
            source_path,
            source,
            tsg_path,
            tsg,
        }
    }
}

struct DisplayBuildErrorPretty<'a> {
    error: &'a BuildError,
    source_path: &'a Path,
    source: &'a str,
    tsg_path: &'a Path,
    tsg: &'a str,
}

impl std::fmt::Display for DisplayBuildErrorPretty<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error {
            BuildError::ExecutionError(err) => write!(
                f,
                "{}",
                err.display_pretty(self.source_path, self.source, self.tsg_path, self.tsg)
            ),
            err => err.fmt(f),
        }
    }
}

impl<'a> Builder<'a> {
    fn load(&mut self, cancellation_flag: &dyn CancellationFlag) -> Result<(), BuildError> {
        let cancellation_flag: &dyn stack_graphs::CancellationFlag = &cancellation_flag;

        // By default graph ids are used for stack graph local_ids. A remapping is computed
        // for local_ids that already exist in the graph---all other graph ids are mapped to
        // the same local_id. See [`self.node_id_for_index`] for more details.
        let mut next_local_id = u32::try_from(self.graph.node_count() - self.injected_node_count)
            .expect("nodes local_id to fit in u32");
        for node in self.stack_graph.nodes_for_file(self.file) {
            let local_id = self.stack_graph[node].id().local_id();
            let index = (local_id as usize) + self.injected_node_count;
            // find next available local_id for which no stack graph node exists yet
            while self
                .stack_graph
                .node_for_id(NodeID::new_in_file(self.file, next_local_id))
                .is_some()
            {
                next_local_id += 1;
            }
            // remap graph node index to the available stack graph node local_id
            if self
                .remapped_nodes
                .insert(index, NodeID::new_in_file(self.file, next_local_id))
                .is_some()
            {
                panic!("index already remapped");
            }
        }

        // First create a stack graph node for each TSG node.  (The skip(...) is because the first
        // DSL nodes that we create are the proxies for the injected stack graph nodes.)
        for node_ref in self.graph.iter_nodes().skip(self.injected_node_count) {
            cancellation_flag.check("loading graph nodes")?;
            let node_type = self.get_node_type(node_ref)?;
            let handle = match node_type {
                NodeType::DropScopes => self.load_drop_scopes(node_ref),
                NodeType::PopScopedSymbol => self.load_pop_scoped_symbol(node_ref)?,
                NodeType::PopSymbol => self.load_pop_symbol(node_ref)?,
                NodeType::PushScopedSymbol => self.load_push_scoped_symbol(node_ref)?,
                NodeType::PushSymbol => self.load_push_symbol(node_ref)?,
                NodeType::Scope => self.load_scope(node_ref)?,
            };
            self.load_source_info(node_ref, handle)?;
            self.load_node_debug_info(node_ref, handle);
        }

        for node in self.stack_graph.nodes_for_file(self.file) {
            self.verify_node(node)?;
        }

        // Then add stack graph edges for each TSG edge.  Note that we _don't_ skip(...) here because
        // there might be outgoing nodes from the “root” node that we need to process.
        // (Technically the caller could add outgoing nodes from “jump to scope” as well, but those
        // are invalid according to the stack graph semantics and will never be followed.
        for source_ref in self.graph.iter_nodes() {
            let source = &self.graph[source_ref];
            let source_node_id = self.node_id_for_graph_node(source_ref);
            let source_handle = self.stack_graph.node_for_id(source_node_id).unwrap();
            for (sink_ref, edge) in source.iter_edges() {
                cancellation_flag.check("loading graph edges")?;
                let precedence = match edge.attributes.get(PRECEDENCE_ATTR) {
                    Some(precedence) => precedence.as_integer()?,
                    None => 0,
                }
                .try_into()
                .map_err(|_| ExecutionError::ExpectedInteger("integer does not fit".to_string()))?;
                let sink_node_id = self.node_id_for_graph_node(sink_ref);
                let sink_handle = self.stack_graph.node_for_id(sink_node_id).unwrap();
                self.stack_graph
                    .add_edge(source_handle, sink_handle, precedence);
                Self::load_edge_debug_info(self.stack_graph, source_handle, sink_handle, edge);
            }
        }

        Ok(())
    }

    fn get_node_type(&self, node_ref: GraphNodeRef) -> Result<NodeType, BuildError> {
        let node = &self.graph[node_ref];
        let node_type = match node.attributes.get(TYPE_ATTR) {
            Some(node_type) => node_type.as_str()?,
            None => return Ok(NodeType::Scope),
        };
        if node_type == DROP_SCOPES_TYPE {
            Ok(NodeType::DropScopes)
        } else if node_type == POP_SCOPED_SYMBOL_TYPE {
            Ok(NodeType::PopScopedSymbol)
        } else if node_type == POP_SYMBOL_TYPE {
            Ok(NodeType::PopSymbol)
        } else if node_type == PUSH_SCOPED_SYMBOL_TYPE {
            Ok(NodeType::PushScopedSymbol)
        } else if node_type == PUSH_SYMBOL_TYPE {
            Ok(NodeType::PushSymbol)
        } else if node_type == SCOPE_TYPE {
            Ok(NodeType::Scope)
        } else {
            Err(BuildError::UnknownNodeType(node_type.to_string()))
        }
    }

    fn verify_node(&self, node: Handle<Node>) -> Result<(), BuildError> {
        if let Node::PushScopedSymbol(node) = &self.stack_graph[node] {
            let scope = &self.stack_graph[self.stack_graph.node_for_id(node.scope).unwrap()];
            if !scope.is_exported_scope() {
                return Err(BuildError::SymbolScopeError(
                    format!("{}", node.display(self.stack_graph)),
                    format!("{}", scope.display(self.stack_graph)),
                ));
            }
        }
        Ok(())
    }
}

enum NodeType {
    DropScopes,
    PopSymbol,
    PopScopedSymbol,
    PushSymbol,
    PushScopedSymbol,
    Scope,
}

impl<'a> Builder<'a> {
    /// Get the `NodeID` corresponding to a `Graph` node.
    ///
    /// By default, graph nodes get their index shifted by [`self.injected_node_count`] as their
    /// `local_id`, unless they have a corresponding entry in the [`self.remapped_nodes`] map. This
    /// is the case if:
    /// 1. The node was injected, in which case it is mapped to the `NodeID` of the injected node.
    /// 2. The node's default `local_id` clashes with a preexisting node, in which case it is mapped to
    ///    an available `local_id` beyond the range of default `local_ids`.
    fn node_id_for_graph_node(&self, node_ref: GraphNodeRef) -> NodeID {
        let index = node_ref.index();
        self.remapped_nodes.get(&index).map_or_else(
            || {
                NodeID::new_in_file(
                    self.file,
                    u32::try_from(index - self.injected_node_count)
                        .expect("local_id to fit in u32"),
                )
            },
            |id| *id,
        )
    }

    fn load_drop_scopes(&mut self, node_ref: GraphNodeRef) -> Handle<Node> {
        let id = self.node_id_for_graph_node(node_ref);
        self.stack_graph.add_drop_scopes_node(id).unwrap()
    }

    fn load_pop_scoped_symbol(
        &mut self,
        node_ref: GraphNodeRef,
    ) -> Result<Handle<Node>, BuildError> {
        let node = &self.graph[node_ref];
        let symbol = match node.attributes.get(SYMBOL_ATTR) {
            Some(symbol) => Self::load_symbol(symbol)?,
            None => return Err(BuildError::MissingSymbol(node_ref)),
        };
        let symbol = self.stack_graph.add_symbol(&symbol);
        let id = self.node_id_for_graph_node(node_ref);
        let is_definition = Self::load_flag(node, IS_DEFINITION_ATTR)?;
        Self::verify_attributes(node, POP_SCOPED_SYMBOL_TYPE, &POP_SCOPED_SYMBOL_ATTRS);
        let node_handle = self
            .stack_graph
            .add_pop_scoped_symbol_node(id, symbol, is_definition)
            .unwrap();
        if is_definition {
            self.load_definiens_info(node_ref, node_handle)?;
        }
        Ok(node_handle)
    }

    fn load_pop_symbol(&mut self, node_ref: GraphNodeRef) -> Result<Handle<Node>, BuildError> {
        let node = &self.graph[node_ref];
        let symbol = match node.attributes.get(SYMBOL_ATTR) {
            Some(symbol) => Self::load_symbol(symbol)?,
            None => return Err(BuildError::MissingSymbol(node_ref)),
        };
        let symbol = self.stack_graph.add_symbol(&symbol);
        let id = self.node_id_for_graph_node(node_ref);
        let is_definition = Self::load_flag(node, IS_DEFINITION_ATTR)?;
        Self::verify_attributes(node, POP_SYMBOL_TYPE, &POP_SYMBOL_ATTRS);
        let node_handle = self
            .stack_graph
            .add_pop_symbol_node(id, symbol, is_definition)
            .unwrap();
        if is_definition {
            self.load_definiens_info(node_ref, node_handle)?;
        }
        Ok(node_handle)
    }

    fn load_push_scoped_symbol(
        &mut self,
        node_ref: GraphNodeRef,
    ) -> Result<Handle<Node>, BuildError> {
        let node = &self.graph[node_ref];
        let symbol = match node.attributes.get(SYMBOL_ATTR) {
            Some(symbol) => Self::load_symbol(symbol)?,
            None => return Err(BuildError::MissingSymbol(node_ref)),
        };
        let symbol = self.stack_graph.add_symbol(&symbol);
        let id = self.node_id_for_graph_node(node_ref);
        let scope = match node.attributes.get(SCOPE_ATTR) {
            Some(scope) => self.node_id_for_graph_node(scope.as_graph_node_ref()?),
            None => return Err(BuildError::MissingScope(node_ref)),
        };
        let is_reference = Self::load_flag(node, IS_REFERENCE_ATTR)?;
        Self::verify_attributes(node, PUSH_SCOPED_SYMBOL_TYPE, &PUSH_SCOPED_SYMBOL_ATTRS);
        Ok(self
            .stack_graph
            .add_push_scoped_symbol_node(id, symbol, scope, is_reference)
            .unwrap())
    }

    fn load_push_symbol(&mut self, node_ref: GraphNodeRef) -> Result<Handle<Node>, BuildError> {
        let node = &self.graph[node_ref];
        let symbol = match node.attributes.get(SYMBOL_ATTR) {
            Some(symbol) => Self::load_symbol(symbol)?,
            None => return Err(BuildError::MissingSymbol(node_ref)),
        };
        let symbol = self.stack_graph.add_symbol(&symbol);
        let id = self.node_id_for_graph_node(node_ref);
        let is_reference = Self::load_flag(node, IS_REFERENCE_ATTR)?;
        Self::verify_attributes(node, PUSH_SYMBOL_TYPE, &PUSH_SYMBOL_ATTRS);
        Ok(self
            .stack_graph
            .add_push_symbol_node(id, symbol, is_reference)
            .unwrap())
    }

    fn load_scope(&mut self, node_ref: GraphNodeRef) -> Result<Handle<Node>, BuildError> {
        let node = &self.graph[node_ref];
        let id = self.node_id_for_graph_node(node_ref);
        let is_exported =
            Self::load_flag(node, IS_EXPORTED_ATTR)? || Self::load_flag(node, IS_ENDPOINT_ATTR)?;
        Self::verify_attributes(node, SCOPE_TYPE, &SCOPE_ATTRS);
        Ok(self.stack_graph.add_scope_node(id, is_exported).unwrap())
    }

    fn load_symbol(value: &Value) -> Result<String, BuildError> {
        match value {
            Value::Integer(i) => Ok(i.to_string()),
            Value::String(s) => Ok(s.clone()),
            _ => Err(BuildError::UnknownSymbolType(value.to_string())),
        }
    }

    fn load_flag(node: &GraphNode, attribute: &str) -> Result<bool, BuildError> {
        match node.attributes.get(attribute) {
            Some(value) => value
                .as_boolean()
                .map_err(|_| BuildError::UnknownFlagType(attribute.to_string(), value.to_string())),
            None => Ok(false),
        }
    }

    fn load_source_info(
        &mut self,
        node_ref: GraphNodeRef,
        node_handle: Handle<Node>,
    ) -> Result<(), BuildError> {
        let node = &self.graph[node_ref];

        if let Some(source_node) = node.attributes.get(SOURCE_NODE_ATTR) {
            let syntax_node_ref = source_node.as_syntax_node_ref()?;
            self.nodes_to_syntax.insert(node_handle, syntax_node_ref);
            let source_node = &self.graph[syntax_node_ref];
            let mut source_range = source_node.text_range();
            if match node.attributes.get(EMPTY_SOURCE_SPAN_ATTR) {
                Some(empty_source_span) => empty_source_span.as_boolean()?,
                None => false,
            } {
                source_range.end = source_range.start;
            }
            let _source_info = self.stack_graph.source_info_mut(node_handle);
            // TODO: map node's TextRange into a Span
            // source_info.span = text_range_into_span(source_range);
        }

        if let Some(syntax_type) = node.attributes.get(SYNTAX_TYPE_ATTR) {
            let syntax_type = syntax_type.as_str()?;
            let syntax_type = self.stack_graph.add_string(syntax_type);
            let source_info = self.stack_graph.source_info_mut(node_handle);
            source_info.syntax_type = syntax_type.into();
        }

        Ok(())
    }

    // TODO: we can probably remove this and all references to definiens since
    // we're not gonna need it?
    fn load_definiens_info(
        &mut self,
        node_ref: GraphNodeRef,
        node_handle: Handle<Node>,
    ) -> Result<(), BuildError> {
        let node = &self.graph[node_ref];
        let definiens_node = match node.attributes.get(DEFINIENS_NODE_ATTR) {
            Some(Value::Null) => return Ok(()),
            Some(definiens_node) => &self.graph[definiens_node.as_syntax_node_ref()?],
            None => return Ok(()),
        };
        let _definiens_range = definiens_node.text_range();
        let _source_info = self.stack_graph.source_info_mut(node_handle);
        // TODO: map node's TextRange into a Span
        // source_info.definiens_span = text_range_into_span(definiens_range);
        Ok(())
    }

    fn load_node_debug_info(&mut self, node_ref: GraphNodeRef, node_handle: Handle<Node>) {
        let node = &self.graph[node_ref];
        for (name, value) in node.attributes.iter() {
            let name = name.to_string();
            if let Some(name_without_prefix) = name.strip_prefix(DEBUG_ATTR_PREFIX) {
                let value = match value {
                    Value::String(value) => value.clone(),
                    value => value.to_string(),
                };
                let key = self.stack_graph.add_string(name_without_prefix);
                let value = self.stack_graph.add_string(&value);
                self.stack_graph
                    .node_debug_info_mut(node_handle)
                    .add(key, value);
            }
        }
    }

    fn load_edge_debug_info(
        stack_graph: &mut StackGraph,
        source_handle: Handle<Node>,
        sink_handle: Handle<Node>,
        edge: &Edge,
    ) {
        for (name, value) in edge.attributes.iter() {
            let name = name.to_string();
            if let Some(name_without_prefix) = name.strip_prefix(DEBUG_ATTR_PREFIX) {
                let value = match value {
                    Value::String(value) => value.clone(),
                    value => value.to_string(),
                };
                let key = stack_graph.add_string(name_without_prefix);
                let value = stack_graph.add_string(&value);
                stack_graph
                    .edge_debug_info_mut(source_handle, sink_handle)
                    .add(key, value);
            }
        }
    }

    fn verify_attributes(
        node: &GraphNode,
        node_type: &str,
        allowed_attributes: &HashSet<&'static str>,
    ) {
        for (id, _) in node.attributes.iter() {
            let id = id.as_str();
            if !allowed_attributes.contains(id)
                && id != SOURCE_NODE_ATTR
                && id != EMPTY_SOURCE_SPAN_ATTR
                && !id.starts_with(DEBUG_ATTR_PREFIX)
            {
                eprintln!("Unexpected attribute {id} on node of type {node_type}");
            }
        }
    }
}
