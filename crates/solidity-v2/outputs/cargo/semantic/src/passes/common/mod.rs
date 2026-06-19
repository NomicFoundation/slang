//! Helpers shared across multiple semantic passes, grouped by functionality:
//!
//! - [`conflicts`]: symbol redeclaration/shadowing detection (used by
//!   `p1_collect_definitions` and `p6_resolve_yul`).
//! - [`resolution`]: shared reference-resolution helpers.
//! - [`node_extensions`]: small helpers over IR nodes (eg. picking the `NodeId`
//!   used to register an expression's typing).

pub(crate) mod conflicts;
mod node_extensions;
mod resolution;

pub(crate) use node_extensions::{
    node_id_for_expression_typing, node_id_for_string_expression_typing,
};
pub(crate) use resolution::{filter_overriden_definitions, resolve_identifier_path_in_scope};
