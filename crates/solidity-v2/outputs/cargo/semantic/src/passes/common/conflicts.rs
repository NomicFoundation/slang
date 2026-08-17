//! Pairwise definition-conflict comparison shared between the per-pass conflict
//! detectors.
//!
//! The scope-walk algorithms differ per pass. Declaring a Solidity name and
//! declaring a Yul name obey different rules and traverse the scope tree
//! differently, so each lives with its pass.

use slang_solidity_v2_common::nodes::NodeId;
use smallvec::smallvec;

use crate::binder::{Binder, Definition, DefinitionIds};

// Returns the declaration(s) `definition_id` stands for once any imports have
// been followed. A definition that isn't an imported symbol stands for itself.
//
// An imported symbol that resolves to nothing (because the import path didn't
// resolve to a file, which gets its own diagnostic, or because the imported
// file doesn't declare the symbol) also stands for itself. That keeps the
// unresolvable import a distinct opaque declaration which conflicts with
// anything else sharing its name, rather than one compatible with everything.
pub(crate) fn underlying_declarations(binder: &Binder, definition_id: NodeId) -> DefinitionIds {
    // The definitions were resolved once at the end of `p1_collect_definitions`,
    // so this is a copy of an inline slice rather than a walk of the imports.
    match binder.imported_symbol_definitions(definition_id) {
        Some(definition_ids) if !definition_ids.is_empty() => {
            DefinitionIds::from_slice(definition_ids)
        }
        _ => smallvec![definition_id],
    }
}

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
