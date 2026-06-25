//! Pairwise definition-conflict comparison shared between the per-pass conflict
//! detectors ([`p1_collect_definitions::conflicts`] and
//! [`p6_resolve_yul::conflicts`]).
//!
//! The scope-walk algorithms differ per pass — declaring a Solidity name and
//! declaring a Yul name obey different rules and traverse the scope tree
//! differently — so each lives with its pass. Only these overload-aware
//! comparison leaves are genuinely shared: deciding whether two definitions
//! that share a name actually conflict, or may legally coexist as overloads
//! (see [`Definition::overloads_with`]).
//!
//! [`p1_collect_definitions::conflicts`]: crate::passes::p1_collect_definitions
//! [`p6_resolve_yul::conflicts`]: crate::passes::p6_resolve_yul

use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition};

// Returns `Some(existing_id)` when an existing definition conflicts with the
// one being declared, or `None` when they may legally coexist (overloads).
pub(crate) fn conflicting_definition(
    binder: &Binder,
    existing_id: NodeId,
    new_definition: &Definition,
) -> Option<NodeId> {
    let existing = binder.find_definition_by_id(existing_id)?;
    if new_definition.overloads_with(existing) {
        None
    } else {
        Some(existing_id)
    }
}

// Returns the first of `existing_ids` that conflicts with `new_definition`.
pub(crate) fn first_conflicting_definition(
    binder: &Binder,
    existing_ids: &[NodeId],
    new_definition: &Definition,
) -> Option<NodeId> {
    existing_ids
        .iter()
        .find_map(|existing_id| conflicting_definition(binder, *existing_id, new_definition))
}
