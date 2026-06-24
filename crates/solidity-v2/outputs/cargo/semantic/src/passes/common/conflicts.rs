//! Scope-walk symbol conflict detection shared across passes. Validates that a
//! definition being declared doesn't illegally redeclare an identifier that is
//! already visible in its scope. Used by `p1_collect_definitions` (for Solidity
//! definitions) and `p6_resolve_yul` (for Yul definitions).
//!
//! Although these walks look similar to the `Binder` resolution methods
//! (`resolve_in_scope` and friends), they intentionally don't share code:
//! resolution answers "what is visible here?" while this module answers "may
//! I declare this here?", and those questions traverse different scope
//! relations. Resolution walks the full lexical chain and stops at the first
//! hit; the conflict search stops at lexical/namespace boundaries (shadowing
//! outer Solidity scopes is legal) but continues *past* non-conflicting hits
//! (overloads), and for Yul walks even further than resolution would, since
//! Yul forbids shadowing entirely.
//!
//! File-level conflict detection that is specific to default imports lives in
//! `p1_collect_definitions::conflicts` instead, since only that pass needs it.

use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition, Scope, ScopeId};

// Looks for a previously-registered definition that conflicts with
// `new_definition` being declared under `symbol` in `scope_id`, returning
// the conflicting definition's `NodeId` (or `None` if the declaration is
// allowed).
//
// The search is bounded to the *local lexical chain*: chained, function,
// modifier and parameters scopes delegate to their enclosing local scope,
// but the walk stops at lexical and namespace boundaries (block/contract/
// file/struct/enum). This means shadowing a declaration from an inner
// lexical scope, or a contract- or file-level member from a local scope, is
// permitted, matching Solidity. Function and event overloads are also
// permitted (see `Definition::overloads_with`).
//
// Yul scopes are different: shadowing is entirely forbidden in Yul, so the
// search continues through all enclosing Yul scopes and, past the assembly
// block, through everything visible at the assembly site (see
// `find_shadowed_solidity_definition`).
pub(crate) fn find_conflicting_definition(
    binder: &Binder,
    scope_id: ScopeId,
    symbol: &str,
    new_definition: &Definition,
) -> Option<NodeId> {
    let scope = binder.get_scope_by_id(scope_id);
    match scope {
        // A block opens a new lexical scope (a real `{ }` block or for-init
        // clause), which may legally shadow declarations in its enclosing
        // scopes, so the walk stops here.
        Scope::Block(block_scope) => block_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition)),
        // A chained scope is a continuation of the parent's lexical scope, so
        // continue the search into the parent.
        Scope::Chained(chained_scope) => chained_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_conflicting_definition(
                    binder,
                    chained_scope.parent_scope_id,
                    symbol,
                    new_definition,
                )
            }),
        Scope::Function(function_scope) => function_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                // Continue into the parameters scope, but *not* the
                // enclosing contract/file scope (shadowing is allowed there).
                find_conflicting_definition(
                    binder,
                    function_scope.parameters_scope_id,
                    symbol,
                    new_definition,
                )
            }),
        Scope::Modifier(modifier_scope) => modifier_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition)),
        Scope::Parameters(parameters_scope) => parameters_scope
            .lookup_definition(symbol)
            .and_then(|existing| conflicting_definition(binder, existing, new_definition)),
        Scope::YulBlock(yul_block_scope) => yul_block_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_conflict_in_yul_parent(
                    binder,
                    yul_block_scope.parent_scope_id,
                    symbol,
                    new_definition,
                )
            }),
        Scope::YulFunction(yul_function_scope) => yul_function_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_conflict_in_yul_parent(
                    binder,
                    yul_function_scope.parent_scope_id,
                    symbol,
                    new_definition,
                )
            }),
        // Namespace scopes are only checked against their own definitions;
        // the `Vec`-based ones may hold several entries for a symbol (eg.
        // function/event overloads).
        Scope::Contract(contract_scope) => contract_scope
            .definitions
            .get(symbol)
            .and_then(|existing| first_conflicting_definition(binder, existing, new_definition)),
        Scope::File(file_scope) => file_scope
            .definitions
            .get(symbol)
            .and_then(|existing| first_conflicting_definition(binder, existing, new_definition)),
        Scope::Enum(enum_scope) => enum_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition)),
        Scope::Struct(struct_scope) => struct_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition)),
        Scope::Using(_) => None,
    }
}

// Continues a conflict search from a Yul scope into its parent. Shadowing is
// not allowed in Yul, even across Yul function boundaries, so while within
// Yul scopes the regular search just continues. Once the walk crosses into
// the Solidity scope enclosing the assembly block, the search switches to
// the unbounded variant below, since assembly-local names may not shadow
// *any* Solidity declaration visible at the assembly site.
fn find_conflict_in_yul_parent(
    binder: &Binder,
    parent_scope_id: ScopeId,
    symbol: &str,
    new_definition: &Definition,
) -> Option<NodeId> {
    match binder.get_scope_by_id(parent_scope_id) {
        Scope::YulBlock(_) | Scope::YulFunction(_) => {
            find_conflicting_definition(binder, parent_scope_id, symbol, new_definition)
        }
        _ => find_shadowed_solidity_definition(binder, parent_scope_id, symbol, new_definition),
    }
}

// Looks for a Solidity definition that would be shadowed by an assembly-local
// name. Unlike `find_conflicting_definition`, this walk does not stop at
// lexical or namespace boundaries: it continues through blocks into the
// enclosing function (and its parameters), contract and file scopes, since
// shadowing any of those declarations from inline assembly is illegal.
fn find_shadowed_solidity_definition(
    binder: &Binder,
    scope_id: ScopeId,
    symbol: &str,
    new_definition: &Definition,
) -> Option<NodeId> {
    let scope = binder.get_scope_by_id(scope_id);
    match scope {
        Scope::Block(block_scope) => block_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_shadowed_solidity_definition(
                    binder,
                    block_scope.parent_scope_id,
                    symbol,
                    new_definition,
                )
            }),
        Scope::Chained(chained_scope) => chained_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_shadowed_solidity_definition(
                    binder,
                    chained_scope.parent_scope_id,
                    symbol,
                    new_definition,
                )
            }),
        Scope::Function(function_scope) => function_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_shadowed_solidity_definition(
                    binder,
                    function_scope.parameters_scope_id,
                    symbol,
                    new_definition,
                )
            })
            .or_else(|| {
                find_shadowed_solidity_definition(
                    binder,
                    function_scope.parent_scope_id,
                    symbol,
                    new_definition,
                )
            }),
        Scope::Modifier(modifier_scope) => modifier_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_shadowed_solidity_definition(
                    binder,
                    modifier_scope.parent_scope_id,
                    symbol,
                    new_definition,
                )
            }),
        Scope::Parameters(parameters_scope) => parameters_scope
            .lookup_definition(symbol)
            .and_then(|existing| conflicting_definition(binder, existing, new_definition)),
        // TODO: members of base contracts are also shadowed, but bases are
        // not resolved yet in this pass. We probably will need to run conflict
        // checks for all Yul identifiers in a separate pass after p2.
        // Alternatively, we may process assembly blocks entirely after p2.
        Scope::Contract(contract_scope) => contract_scope
            .definitions
            .get(symbol)
            .and_then(|existing| first_conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_shadowed_solidity_definition(
                    binder,
                    contract_scope.file_scope_id,
                    symbol,
                    new_definition,
                )
            }),
        Scope::File(file_scope) => file_scope
            .definitions
            .get(symbol)
            .and_then(|existing| first_conflicting_definition(binder, existing, new_definition)),
        // No other scope kind can enclose an inline assembly block.
        Scope::Enum(_)
        | Scope::Struct(_)
        | Scope::Using(_)
        | Scope::YulBlock(_)
        | Scope::YulFunction(_) => None,
    }
}

// Returns `Some(existing_id)` when an existing definition conflicts with the
// one being declared, or `None` when they may legally coexist (overloads).
fn conflicting_definition(
    binder: &Binder,
    existing_id: NodeId,
    new_definition: &Definition,
) -> Option<NodeId> {
    let existing = binder.find_definition_by_id(existing_id)?;
    if matches!(new_definition, Definition::YulFunction(_))
        && matches!(existing, Definition::Function(_))
    {
        return None;
    }
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
