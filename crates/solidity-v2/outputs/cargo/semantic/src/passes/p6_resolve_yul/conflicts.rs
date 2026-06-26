//! Symbol conflict detection for the Yul resolution pass.
//!
//! [`find_conflicting_yul_definition`] is the scope-walk used when registering
//! each Yul definition: it answers "may I declare this name here?". Yul forbids
//! shadowing entirely *within* an assembly block, so the search always walks
//! every enclosing Yul scope first (a name can't be redeclared, and Yul
//! functions and variables share one namespace).
//!
//! What may be shadowed *outside* the assembly block depends on the kind of Yul
//! declaration:
//!
//! - A Yul **function** may shadow any enclosing Solidity declaration.
//! - A Yul function **parameter or return variable** may shadow non-local
//!   Solidity declarations (state variables, constants, functions, ...), but not
//!   a local Solidity variable. solc actually allows the declaration and only
//!   rejects a *reference* to the shadowed name from inside the Yul function
//!   ("Cannot access local Solidity variables from inside an inline assembly
//!   function"); we reject the declaration outright as a simpler, conservative
//!   approximation (so we diverge from solc only when the name is never used).
//! - A Yul **variable** (`let`) may not shadow any visible Solidity declaration.
//!
//! This is the Yul counterpart to
//! [`p1_collect_definitions::conflicts::find_conflicting_solidity_definition`],
//! with which it shares only the pairwise comparison leaves in
//! [`crate::passes::common::conflicts`].
//!
//! [`p1_collect_definitions::conflicts::find_conflicting_solidity_definition`]: crate::passes::p1_collect_definitions

use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition, Scope, ScopeId};
use crate::passes::common::conflicts::{conflicting_definition, first_conflicting_definition};

// Looks for a previously-registered definition that conflicts with a Yul
// `new_definition` being declared under `symbol` in `scope_id`, returning the
// conflicting definition's `NodeId` (or `None` if the declaration is allowed).
// See the module docs for the per-declaration-kind shadowing rules.
pub(super) fn find_conflicting_yul_definition(
    binder: &Binder,
    scope_id: ScopeId,
    symbol: &str,
    new_definition: &Definition,
) -> Option<NodeId> {
    let scope = binder.get_scope_by_id(scope_id);
    // The search always starts at the Yul scope the definition is declared in.
    debug_assert!(
        matches!(scope, Scope::YulBlock(_) | Scope::YulFunction(_)),
        "a Yul definition is always declared in a Yul scope"
    );

    // A Yul function's parameters and return variables are declared directly in
    // the function scope; a Yul variable (`let`) and the (hoisted) function
    // names themselves live in a Yul block scope. Only parameters and return
    // variables may shadow non-local Solidity declarations, so remember whether
    // the walk started from a function signature.
    let from_function_signature = matches!(scope, Scope::YulFunction(_));

    find_conflict_from_scope(
        binder,
        scope_id,
        symbol,
        new_definition,
        from_function_signature,
    )
}

// Walks up the scope tree from `scope_id`, looking for a definition the Yul
// `new_definition` would illegally conflict with. While inside Yul scopes the
// search just continues into the parent (shadowing is never allowed between Yul
// scopes, even across function boundaries). Once it crosses out of the assembly
// block into the enclosing Solidity scopes, what counts as a conflict depends on
// the kind of Yul declaration (see the module docs).
//
// `from_function_signature` records the kind of the original declaration and is
// preserved unchanged across the walk, since it describes the declaration site,
// not the scope currently being inspected.
fn find_conflict_from_scope(
    binder: &Binder,
    scope_id: ScopeId,
    symbol: &str,
    new_definition: &Definition,
    from_function_signature: bool,
) -> Option<NodeId> {
    match binder.get_scope_by_id(scope_id) {
        Scope::YulBlock(yul_block_scope) => yul_block_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_conflict_from_scope(
                    binder,
                    yul_block_scope.parent_scope_id,
                    symbol,
                    new_definition,
                    from_function_signature,
                )
            }),
        Scope::YulFunction(yul_function_scope) => yul_function_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_conflict_from_scope(
                    binder,
                    yul_function_scope.parent_scope_id,
                    symbol,
                    new_definition,
                    from_function_signature,
                )
            }),
        // Crossed out of the assembly block into the enclosing Solidity scopes.
        // A Yul function may shadow any enclosing Solidity declaration; a
        // parameter or return variable may shadow non-local declarations but not
        // a local variable; a `let` variable may shadow nothing. The latter two
        // differ only in where the shadow search stops (see
        // `find_shadowed_solidity_definition`).
        _ if new_definition.is_yul_function() => None,
        _ => find_shadowed_solidity_definition(
            binder,
            scope_id,
            symbol,
            new_definition,
            from_function_signature,
        ),
    }
}

// Looks for a Solidity definition that would be illegally shadowed by an
// assembly-local name. Unlike `find_conflicting_solidity_definition` (from p1),
// this walk does not stop at lexical boundaries: it continues through every
// enclosing block into the function (and its parameters) and modifier scopes,
// since an assembly-local name may not shadow any of those local declarations.
//
// `from_function_signature` controls the *namespace* boundary, and is the only
// difference between the two callers (see the module docs): a Yul `let`
// variable (flag unset) may shadow nothing, so the walk also descends into the
// contract and file scopes; a Yul function parameter or return variable (flag
// set) may shadow non-local declarations, so it stops at that boundary and only
// reports local variables and parameters.
fn find_shadowed_solidity_definition(
    binder: &Binder,
    scope_id: ScopeId,
    symbol: &str,
    new_definition: &Definition,
    from_function_signature: bool,
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
                    from_function_signature,
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
                    from_function_signature,
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
                    from_function_signature,
                )
            })
            // For a `let` variable, continue past the function into the
            // enclosing contract/file; for a function signature, those scopes
            // are shadowable and the recursion stops there (see below).
            .or_else(|| {
                find_shadowed_solidity_definition(
                    binder,
                    function_scope.parent_scope_id,
                    symbol,
                    new_definition,
                    from_function_signature,
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
                    from_function_signature,
                )
            }),
        Scope::Parameters(parameters_scope) => parameters_scope
            .lookup_definition(symbol)
            .and_then(|existing| conflicting_definition(binder, existing, new_definition)),
        // The contract and file scopes hold non-local declarations (state
        // variables, constants, functions, imports), which a function signature
        // may shadow — so the walk only descends here for a `let` variable.
        // TODO: members of base contracts are also shadowed, but bases are
        // not resolved yet in this pass. We probably will need to run conflict
        // checks for all Yul identifiers in a separate pass after p2.
        // Alternatively, we may process assembly blocks entirely after p2.
        Scope::Contract(contract_scope) if !from_function_signature => contract_scope
            .definitions
            .get(symbol)
            .and_then(|existing| first_conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_shadowed_solidity_definition(
                    binder,
                    contract_scope.file_scope_id,
                    symbol,
                    new_definition,
                    from_function_signature,
                )
            }),
        Scope::File(file_scope) if !from_function_signature => file_scope
            .definitions
            .get(symbol)
            .and_then(|existing| first_conflicting_definition(binder, existing, new_definition)),
        // Stop: either the namespace boundary for a function signature, or a
        // scope kind that cannot enclose an inline assembly block.
        Scope::Contract(_)
        | Scope::File(_)
        | Scope::Enum(_)
        | Scope::Struct(_)
        | Scope::Using(_)
        | Scope::YulBlock(_)
        | Scope::YulFunction(_) => None,
    }
}
