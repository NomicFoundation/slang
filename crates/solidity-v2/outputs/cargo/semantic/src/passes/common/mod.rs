//! Helpers shared across multiple semantic passes, grouped by functionality:
//!
//! - [`conflicts`]: symbol redeclaration/shadowing detection (used by
//!   `p1_collect_definitions` and `p6_resolve_yul`).
//! - [`constant_evaluator`]: compile-time constant folding (used by
//!   `p3_type_definitions` for array lengths and `p5_resolve_references` for
//!   storage base slots).
//! - [`resolution`]: shared reference-resolution helpers.
//! - [`node_extensions`]: small helpers over IR nodes (eg. computing a node's
//!   source location).

pub(crate) mod conflicts;
pub(crate) mod constant_evaluator;
mod node_extensions;
mod resolution;

pub(crate) use node_extensions::node_location;
pub(crate) use resolution::{
    bases_after, filter_overriden_definitions, find_definition_namespace_scope_id,
    function_overrides, has_virtual_semantics, modifier_dispatches_virtually,
    most_derived_modifier, most_derived_override, resolve_identifier_path_in_scope, super_override,
};
