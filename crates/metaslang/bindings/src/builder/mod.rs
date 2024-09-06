// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, stack-graphs authors.
// Copyright © 2024, slang authors.
// Licensed under MIT license
// Please see the LICENSE file in the root of this crate for license details.
// ------------------------------------------------------------------------------------------------

//! This module lets you construct [stack graphs][] using this crate's [graph construction DSL][].
//! The graph DSL lets you construct arbitrary graph structures from the parsed syntax tree of a
//! source file.  If you construct a graph using the vocabulary of attributes described below, then
//! the result of executing the graph DSL will be a valid stack graph, which we can then use for
//! name binding lookups.
//!
//! ## Prerequisites
//!
//! [stack graphs]: https://docs.rs/stack-graphs/*/
//! [graph construction DSL]: https://docs.rs/metaslang_graph_builder/*/
//!
//! To process a particular source language, you'll need to first get the CST from the source, using
//! the parser constructed from the language definition that uses the `metaslang_cst` crate.
//!
//! You will then need to create _stack graph construction rules_ for your language.  These rules
//! are implemented using metaslang's [graph construction DSL][], which is based from tree-sitter's
//! graph construction DSL. They define the particular stack graph nodes and edges that should be
//! created for each part of the parsed syntax tree of a source file.
//!
//! ## Graph DSL vocabulary
//!
//! **Please note**: This documentation assumes you are already familiar with stack graphs, and how
//! to use different stack graph node types, and the connectivity between nodes, to implement the
//! name binding semantics of your language.  We assume that you know what kind of stack graph you
//! want to produce; this documentation focuses only on the mechanics of _how_ to create that stack
//! graph content.
//!
//! As mentioned above, your stack graph construction rules should create stack graph nodes and
//! edges from the parsed content of a source file.  You will use MSGB [stanzas][] to match on
//! different parts of the parsed syntax tree, and create stack graph content for each match.
//!
//! ### Creating stack graph nodes
//!
//! To create a stack graph node for each identifier in a Solidity file, you could use the following
//! MSGB stanza:
//!
//! ``` skip
//! [Identifier] {
//!   node new_node
//! }
//! ```
//!
//! (Here, `node` is a MSGB statement that creates a new node, and `new_node` is the name of a local
//! variable that the new node is assigned to, letting you refer to the new node in the rest of the
//! stanza.)
//!
//! [stanzas]: https://docs.rs/tree-sitter-graph/*/tree_sitter_graph/reference/index.html#high-level-structure
//!
//! By default, this new node will be a _scope node_.  If you need to create a different kind of stack
//! graph node, set the `type` attribute on the new node:
//!
//! ``` skip
//! [Identifier] {
//!   node new_node
//!   attr (new_node) type = "push_symbol"
//! }
//! ```
//!
//! The valid `type` values are:
//!
//! - `drop_scopes`: a _drop scopes_ node
//! - `pop_symbol`: a _pop symbol_ node
//! - `pop_scoped_symbol`: a _pop scoped symbol_ node
//! - `push_symbol`: a _push symbol_ node
//! - `push_scoped_symbol`: a _push scoped symbol_ node
//! - `scope`: a _scope_ node
//!
//! A node without an explicit `type` attribute is assumed to be of type `scope`.
//!
//! Certain node types — `pop_symbol`, `pop_scoped_symbol`, `push_symbol` and `push_scoped_symbol` —
//! also require you to provide a `symbol` attribute. Its value must be a string, but will typically
//! come from the content of a parsed syntax node using the [`source-text`][] function and a syntax
//! capture:
//!
//! [`source-text`]: https://docs.rs/tree-sitter-graph/*/tree_sitter_graph/reference/functions/index.html#source-text
//!
//! ``` skip
//! @id [Identifier] {
//!   node new_node
//!   attr (new_node) type = "push_symbol", symbol = (source-text @id)
//! }
//! ```
//!
//! Node types `pop_symbol` and `pop_scoped_symbol` allow an optional `is_definition` attribute,
//! which marks that node as a proper definition. Node types `push_symbol` and `push_scoped_symbol`
//! allow an optional `is_reference` attribute, which marks the node as a proper reference. When
//! `is_definition` or `is_reference` are set, the `source_node` attribute is required.
//!
//! ``` skip
//! @id [Identifier] {
//!   node new_node
//!   attr (new_node) type = "push_symbol", symbol = (source-text @id), is_reference, source_node = @id
//! }
//! ```
//!
//! A _push scoped symbol_ node requires a `scope` attribute. Its value must be a reference to an
//! `exported` node that you've already created. (This is the exported scope node that will be
//! pushed onto the scope stack.) For instance:
//!
//! ``` skip
//! @id [Identifier] {
//!   node new_exported_scope_node
//!   attr (new_exported_scope_node) is_exported
//!   node new_push_scoped_symbol_node
//!   attr (new_push_scoped_symbol_node)
//!     type = "push_scoped_symbol",
//!     symbol = (source-text @id),
//!     scope = new_exported_scope_node
//! }
//! ```
//!
//! Nodes of type `scope` allow an optional `is_exported` attribute, that is required to use the
//! scope in a `push_scoped_symbol` node.
//!
//!
//! ### Annotating nodes with location information
//!
//! You can annotate any stack graph node that you create with location information, identifying
//! the portion of the source file that the node "belongs to".  This is _required_ for definition
//! and reference nodes, since the location information determines which parts of the source file
//! the user can _click on_, and the _destination_ of any code navigation queries the user makes.
//! To do this, add a `source_node` attribute, whose value is a syntax node capture:
//!
//! ``` skip
//! @func [FunctionDefinition ... [FunctionName @id [Identifier]] ...] {
//!   node def
//!   attr (def) type = "pop_symbol", symbol = (source-text @id), source_node = @func, is_definition
//! }
//! ```
//!
//! Note how in this example, we use a different syntax node for the _target_ of the definition
//! (the entirety of the function definition) and for the _name_ of the definition (the content of
//! the function's `name`).
//!
//! Adding the `empty_source_span` attribute will use an empty source span located at the start of
//! the span of the `source_node`. This can be useful when a proper reference or definition is
//! desired, and thus `source_node` is required, but the span of the available source node is too
//! large. For example, a module definition which is located at the start of the program, but does
//! span the whole program:
//!
//! ``` skip
//! @unit [SourceUnit] {
//!   ; ...
//!   node mod_def
//!   attr mod_def type = "pop_symbol", symbol = mod_name, is_definition, source_node = @unit, empty_source_span
//!   ; ...
//! }
//! ```
//!
//! ### Annotating nodes with syntax type information
//!
//! You can annotate any stack graph node with information about its syntax type. To do this, add a
//! `syntax_type` attribute, whose value is a string indicating the syntax type.
//!
//! ``` skip
//! @func [FunctionDefinition ... [FunctionName @id [Identifier]] ...] {
//!   node def
//!   ; ...
//!   attr (def) syntax_type = "function"
//! }
//! ```
//!
//! ### Annotating definitions with definiens information
//!
//! You cannot annotate definitions with a definiens, which is the thing the definition covers. For
//! example, for a function definition, the definiens would be the function body. To do this, add a
//! `definiens_node` attribute, whose value is a syntax node that spans the definiens.
//!
//! ``` skip
//! @func [FunctionDefinition ... [FunctionName @id [Identifier]] ... @body [FunctionBody] ...] {
//!   node def
//!   ; ...
//!   attr (def) definiens_node = @body
//! }
//! ```
//!
//! Definiens are optional and setting them to `#null` explicitly is allowed.
//!
//! ### Connecting stack graph nodes with edges
//!
//! To connect two stack graph nodes, use the `edge` statement to add an edge between them:
//!
//! ``` skip
//! @func [FunctionDefinition ... [FunctionName @id [Identifier]] ...] {
//!   node def
//!   attr (def) type = "pop_symbol", symbol = (source-text @id), source_node = @func, is_definition
//!   node body
//!   edge def -> body
//! }
//! ```
//!
//! To implement shadowing (which determines which definitions are selected when multiple are available),
//! you can add a `precedence` attribute to each edge to indicate which paths are prioritized:
//!
//! ``` skip
//! @func [FunctionDefinition ... [FunctionName @id [Identifier]] ...] {
//!   node def
//!   attr (def) type = "pop_symbol", symbol = (source-text @id), source_node = @func, is_definition
//!   node body
//!   edge def -> body
//!   attr (def -> body) precedence = 1
//! }
//! ```
//!
//! (If you don't specify a `precedence`, the default is 0.)
//!
//! ### Referring to the singleton nodes
//!
//! The _root node_ and _jump to scope node_ are singleton nodes that always exist for all stack
//! graphs.  You can refer to them using the `ROOT_NODE` and `JUMP_TO_SCOPE_NODE` global variables:
//!
//! ``` skip
//! global ROOT_NODE
//!
//! @func [FunctionDefinition ... [FunctionName @id [Identifier]] ...] {
//!   node def
//!   attr (def) type = "pop_symbol", symbol = (source-text @id), source_node = @func, is_definition
//!   edge ROOT_NODE -> def
//! }
//! ```
//!
//! ### Attaching debug information to nodes
//!
//! It is possible to attach extra information to nodes for debugging purposes.  This is done by adding
//! `debug_*` attributes to nodes.  Each attribute defines a debug entry, with the key derived from the
//! attribute name, and the value the string representation of the attribute value.  For example, mark
//! a scope node with a kind as follows:
//!
//! ``` skip
//! @func [FunctionDefinition ... [FunctionName @id [Identifier]] ...] {
//!   ; ...
//!   node param_scope
//!   attr (param_scope) debug_kind = "param_scope"
//!   ; ...
//! }
//! ```
//!

mod cancellation;
mod functions;

use std::collections::{HashMap, HashSet};
use std::path::Path;

pub use cancellation::{CancellationFlag, NoCancellation};
pub use functions::default_functions;
use metaslang_cst::cursor::Cursor;
use metaslang_cst::KindTypes;
use metaslang_graph_builder::ast::File as GraphBuilderFile;
use metaslang_graph_builder::functions::Functions;
use metaslang_graph_builder::graph::{Edge, Graph, GraphNode, GraphNodeRef, Value};
use metaslang_graph_builder::{ExecutionConfig, ExecutionError, Variables};
use once_cell::sync::Lazy;
use stack_graphs::arena::Handle;
use stack_graphs::graph::{File, Node, NodeID, StackGraph};
use thiserror::Error;

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
static PARENTS_ATTR: &str = "parents";
static SCOPE_ATTR: &str = "scope";
static SELECTOR_ATTR: &str = "selector";
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
        SELECTOR_ATTR,
        PARENTS_ATTR,
        SYNTAX_TYPE_ATTR,
    ])
});
static POP_SYMBOL_ATTRS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        TYPE_ATTR,
        SYMBOL_ATTR,
        IS_DEFINITION_ATTR,
        DEFINIENS_NODE_ATTR,
        SELECTOR_ATTR,
        PARENTS_ATTR,
        SYNTAX_TYPE_ATTR,
    ])
});
static PUSH_SCOPED_SYMBOL_ATTRS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        TYPE_ATTR,
        SYMBOL_ATTR,
        SCOPE_ATTR,
        IS_REFERENCE_ATTR,
        PARENTS_ATTR,
    ])
});
static PUSH_SYMBOL_ATTRS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| HashSet::from([TYPE_ATTR, SYMBOL_ATTR, IS_REFERENCE_ATTR, PARENTS_ATTR]));
static SCOPE_ATTRS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| HashSet::from([TYPE_ATTR, IS_EXPORTED_ATTR, IS_ENDPOINT_ATTR]));

// Edge attribute names
static PRECEDENCE_ATTR: &str = "precedence";

// Global variables
/// Name of the variable used to pass the root node.
pub const ROOT_NODE_VAR: &str = "ROOT_NODE";
/// Name of the variable used to pass the file path.
pub const FILE_PATH_VAR: &str = "FILE_PATH";

pub(crate) struct Builder<'a, KT: KindTypes> {
    msgb: &'a GraphBuilderFile<KT>,
    functions: &'a Functions<KT>,
    stack_graph: &'a mut StackGraph,
    file: Handle<File>,
    tree_cursor: Cursor<KT>,
    graph: Graph<KT>,
    remapped_nodes: HashMap<usize, NodeID>,
    injected_node_count: usize,
    cursors: HashMap<Handle<Node>, Cursor<KT>>,
    definiens: HashMap<Handle<Node>, Cursor<KT>>,
    selectors: HashMap<Handle<Node>, Selector>,
    parents: HashMap<Handle<Node>, Vec<Handle<Node>>>,
}

#[derive(Debug)]
pub(crate) enum Selector {
    Alias,
    C3,
}

pub(crate) struct BuildResult<KT: KindTypes> {
    #[cfg(feature = "__private_testing_utils")]
    pub graph: Graph<KT>,
    pub cursors: HashMap<Handle<Node>, Cursor<KT>>,
    pub definiens: HashMap<Handle<Node>, Cursor<KT>>,
    pub selectors: HashMap<Handle<Node>, Selector>,
    pub parents: HashMap<Handle<Node>, Vec<Handle<Node>>>,
}

impl<'a, KT: KindTypes + 'static> Builder<'a, KT> {
    pub fn new(
        msgb: &'a GraphBuilderFile<KT>,
        functions: &'a Functions<KT>,
        stack_graph: &'a mut StackGraph,
        file: Handle<File>,
        tree_cursor: Cursor<KT>,
    ) -> Self {
        Builder {
            msgb,
            functions,
            stack_graph,
            file,
            tree_cursor,
            graph: Graph::new(),
            remapped_nodes: HashMap::new(),
            injected_node_count: 0,
            cursors: HashMap::new(),
            definiens: HashMap::new(),
            selectors: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    fn build_global_variables(&mut self) -> Variables<'a> {
        let mut variables = Variables::new();
        let file_path = self.stack_graph[self.file].name();
        variables
            .add(FILE_PATH_VAR.into(), file_path.into())
            .expect("failed to add FILE_PATH variable");

        let root_node = self.inject_node(NodeID::root());
        variables
            .add(ROOT_NODE_VAR.into(), root_node.into())
            .expect("Failed to set ROOT_NODE");

        #[cfg(feature = "__private_testing_utils")]
        {
            // For debugging purposes only
            self.graph[root_node]
                .attributes
                .add(
                    [DEBUG_ATTR_PREFIX, "msgb_variable"]
                        .concat()
                        .as_str()
                        .into(),
                    ROOT_NODE_VAR.to_string(),
                )
                .expect("Failed to set ROOT_NODE variable name for debugging");
        }

        variables
    }

    /// Executes this builder.
    pub fn build(
        mut self,
        cancellation_flag: &dyn CancellationFlag,
    ) -> Result<BuildResult<KT>, BuildError> {
        let variables = self.build_global_variables();

        let config = ExecutionConfig::new(self.functions, &variables)
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

        self.msgb.execute_into(
            &mut self.graph,
            &self.tree_cursor,
            &config,
            &(cancellation_flag as &dyn CancellationFlag),
        )?;

        self.load(cancellation_flag)?;

        Ok(BuildResult {
            #[cfg(feature = "__private_testing_utils")]
            graph: self.graph,
            cursors: self.cursors,
            definiens: self.definiens,
            selectors: self.selectors,
            parents: self.parents,
        })
    }

    /// Create a graph node to represent the stack graph node. It is the callers responsibility to
    /// ensure the stack graph node exists.
    pub fn inject_node(&mut self, id: NodeID) -> GraphNodeRef {
        let node = self.graph.add_graph_node();
        self.remapped_nodes.insert(node.index(), id);
        self.injected_node_count += 1;
        node
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
    #[error("Unknown selector ‘{0}’")]
    UnknownSelector(String),
    #[error("Parent must be either a reference or definition")]
    InvalidParent(GraphNodeRef),
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

impl<'a, KT: KindTypes + 'static> Builder<'a, KT> {
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

        // First create a stack graph node for each MSGB node.  (The skip(...) is because the first
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

        // Iterate again to resolve parents attribute, which refers to other nodes in the graph
        for node_ref in self.graph.iter_nodes().skip(self.injected_node_count) {
            cancellation_flag.check("loading graph nodes")?;
            self.load_parents_info(node_ref)?;
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

impl<'a, KT: KindTypes> Builder<'a, KT> {
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
            self.load_selector_info(node_ref, node_handle)?;
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
            self.load_selector_info(node_ref, node_handle)?;
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

        // For every added graph node which links to a corresponding source
        // node, save the corresponding CST cursor so our caller can extract
        // that info later.
        if let Some(source_node) = node.attributes.get(SOURCE_NODE_ATTR) {
            let syntax_node_ref = source_node.as_syntax_node_ref()?;
            let source_node = &self.graph[syntax_node_ref];
            self.cursors.insert(node_handle, source_node.clone());
        }

        if let Some(syntax_type) = node.attributes.get(SYNTAX_TYPE_ATTR) {
            let syntax_type = syntax_type.as_str()?;
            let syntax_type = self.stack_graph.add_string(syntax_type);
            let source_info = self.stack_graph.source_info_mut(node_handle);
            source_info.syntax_type = syntax_type.into();
        }

        Ok(())
    }

    fn load_definiens_info(
        &mut self,
        node_ref: GraphNodeRef,
        node_handle: Handle<Node>,
    ) -> Result<(), BuildError> {
        let node = &self.graph[node_ref];
        // Save the definiens CST cursor so our caller can extract the mapping
        // to the stack graph node later
        if let Some(definiens_node) = node.attributes.get(DEFINIENS_NODE_ATTR) {
            let syntax_node_ref = definiens_node.as_syntax_node_ref()?;
            let definiens_node = &self.graph[syntax_node_ref];
            self.definiens.insert(node_handle, definiens_node.clone());
        };
        Ok(())
    }

    fn load_selector_info(
        &mut self,
        node_ref: GraphNodeRef,
        node_handle: Handle<Node>,
    ) -> Result<(), BuildError> {
        let node = &self.graph[node_ref];
        if let Some(selector_value) = node.attributes.get(SELECTOR_ATTR) {
            let selector_type = selector_value.as_str()?;

            let selector = match selector_type {
                "alias" => Selector::Alias,
                "c3" => Selector::C3,
                _ => return Err(BuildError::UnknownSelector(selector_type.to_string())),
            };
            self.selectors.insert(node_handle, selector);
        }
        Ok(())
    }

    fn load_parents_info(&mut self, node_ref: GraphNodeRef) -> Result<(), BuildError> {
        let node = &self.graph[node_ref];
        if let Some(parents) = node.attributes.get(PARENTS_ATTR) {
            let node_id = self.node_id_for_graph_node(node_ref);
            let node_handle = self
                .stack_graph
                .node_for_id(node_id)
                .expect("node already exists in the graph");
            let parent_handles = parents
                .as_list()?
                .iter()
                .flat_map(|value| {
                    value.as_graph_node_ref().map(|id| {
                        self.stack_graph
                            .node_for_id(self.node_id_for_graph_node(id))
                            .expect("parent node exists in the stack graph")
                    })
                })
                .flat_map(|parent| {
                    // ensure parents are either definitions or references
                    let parent_node = &self.stack_graph[parent];
                    if !parent_node.is_definition() && !parent_node.is_reference() {
                        Err(BuildError::InvalidParent(node_ref))
                    } else {
                        Ok(parent)
                    }
                })
                .collect::<Vec<_>>();
            self.parents.insert(node_handle, parent_handles);
        }
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
