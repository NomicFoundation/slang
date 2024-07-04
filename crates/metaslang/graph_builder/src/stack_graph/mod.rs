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

mod builder;
mod cancellation;

pub use builder::{BuildError, Builder, Globals};
pub use cancellation::{CancellationFlag, NoCancellation};
