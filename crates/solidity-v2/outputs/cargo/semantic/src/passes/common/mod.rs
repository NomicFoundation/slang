//! Helpers shared across multiple semantic passes, grouped by functionality:
//!
//! - [`conflicts`]: symbol redeclaration/shadowing detection (used by
//!   `p1_collect_definitions` and `p6_resolve_yul`).
//! - [`constant_evaluator`]: compile-time constant folding (used by
//!   `p3_type_definitions` for array lengths and `p5_resolve_references` for
//!   storage base slots).
//! - [`resolution`]: shared reference-resolution helpers.
//! - [`scope_stack`]: the stack of enclosing scopes kept while traversing (used
//!   by `p1_collect_definitions`, `p3_type_definitions` and
//!   `p5_resolve_references`).
//! - [`node_extensions`]: small helpers over IR nodes (eg. computing a node's
//!   source location).

pub(crate) mod conflicts;
pub(crate) mod constant_evaluator;
mod node_extensions;
mod resolution;
mod scope_stack;

pub(crate) use node_extensions::node_location;
pub(crate) use resolution::{
    Overridable, PathResolution, filter_overridden_definitions,
    filter_overridden_definitions_by_selector, find_definition_namespace_scope_id,
    resolve_identifier_path_as_expression_in_scope, resolve_identifier_path_as_type_in_scope,
};
pub(crate) use scope_stack::ScopeStack;
