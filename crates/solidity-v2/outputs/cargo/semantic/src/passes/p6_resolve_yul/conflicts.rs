//! Symbol conflict detection for the Yul resolution pass.
//!
//! [`find_conflicting_yul_definition`] is the scope-walk used when registering
//! each Yul definition: it answers "may I declare this name here?". Yul forbids
//! shadowing entirely, so unlike the Solidity counterpart
//! ([`p1_collect_definitions::conflicts::find_conflicting_solidity_definition`])
//! this search does not stop at scope boundaries: it continues through every
//! enclosing Yul scope and, once it crosses out of the assembly block into the
//! enclosing Solidity scopes, switches to [`find_shadowed_solidity_definition`],
//! since an assembly-local name may not shadow any Solidity declaration visible
//! at the assembly site. The two passes share only the pairwise comparison
//! leaves in [`crate::passes::common::conflicts`].
//!
//! [`p1_collect_definitions::conflicts::find_conflicting_solidity_definition`]: crate::passes::p1_collect_definitions

use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition, Scope, ScopeId};
use crate::passes::common::conflicts::{conflicting_definition, first_conflicting_definition};

// Looks for a previously-registered definition that conflicts with a Yul
// `new_definition` being declared under `symbol` in `scope_id`, returning the
// conflicting definition's `NodeId` (or `None` if the declaration is allowed).
//
// Shadowing is entirely forbidden in Yul, so unlike the Solidity walk this
// search does not stop at scope boundaries: it continues through every
// enclosing Yul scope and, once it crosses out of the assembly block into the
// enclosing Solidity scopes, switches to `find_shadowed_solidity_definition`,
// since an assembly-local name may not shadow any Solidity declaration visible
// at the assembly site.
pub(super) fn find_conflicting_yul_definition(
    binder: &Binder,
    scope_id: ScopeId,
    symbol: &str,
    new_definition: &Definition,
) -> Option<NodeId> {
    match binder.get_scope_by_id(scope_id) {
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
        // A Yul definition is always declared inside a Yul scope.
        _ => unreachable!("Yul definitions are only declared in Yul scopes"),
    }
}

// Continues a Yul conflict search from a Yul scope into its parent. Shadowing is
// not allowed in Yul, even across Yul function boundaries, so while within Yul
// scopes the regular search just continues. Once the walk crosses into the
// Solidity scope enclosing the assembly block, the search switches to the
// unbounded variant below, since assembly-local names may not shadow *any*
// Solidity declaration visible at the assembly site.
fn find_conflict_in_yul_parent(
    binder: &Binder,
    parent_scope_id: ScopeId,
    symbol: &str,
    new_definition: &Definition,
) -> Option<NodeId> {
    match binder.get_scope_by_id(parent_scope_id) {
        Scope::YulBlock(_) | Scope::YulFunction(_) => {
            find_conflicting_yul_definition(binder, parent_scope_id, symbol, new_definition)
        }
        _ => find_shadowed_solidity_definition(binder, parent_scope_id, symbol, new_definition),
    }
}

// Looks for a Solidity definition that would be shadowed by an assembly-local
// name. Unlike `find_conflicting_solidity_definition`, this walk does not stop
// at lexical or namespace boundaries: it continues through blocks into the
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
