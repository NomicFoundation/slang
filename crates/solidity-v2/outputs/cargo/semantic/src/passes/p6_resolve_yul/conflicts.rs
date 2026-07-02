//! Symbol conflict detection for the Yul resolution pass.
//!
//! [`find_conflicting_yul_definition`] is the scope-walk used when registering
//! each Yul definition: it answers "may I declare this name here?". A reserved
//! Yul built-in name (`add`, `mload`, ...) may never be declared, so that's
//! checked first. Otherwise Yul forbids shadowing entirely *within* an assembly
//! block, so the search walks every enclosing Yul scope first (a name can't be
//! redeclared, and Yul functions and variables share one namespace).
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
//! - A Yul **variable** (`let`) may not shadow anything visible from outside
//!   the assembly block: neither a Solidity declaration (state variable,
//!   constant, function, import, ...) nor a built-in (e.g. `let msg := 1`).
//!   Built-ins have no user definition to resolve to, so they're checked
//!   separately, but both produce the same `ExternalDeclarationShadowing`
//!   conflict (matching solc).

use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition, Resolution, Scope, ScopeId};
use crate::built_ins::BuiltInsResolver;
use crate::passes::common::conflicts::conflicting_definition;

/// Why a Yul declaration is not allowed, used to pick the diagnostic to report.
/// See the module docs for the per-declaration-kind shadowing rules.
pub(super) enum YulConflict {
    /// The name is a reserved Yul built-in (e.g. `let add := 1`) and may not be
    /// declared at all.
    BuiltInRedeclaration,
    /// The name redeclares another definition in a scope where redeclaration is
    /// forbidden: another Yul definition in the same assembly block, or (for a
    /// parameter/return) a local Solidity variable it may not shadow.
    Redeclaration,
    /// A Yul variable's name shadows a declaration visible from outside the
    /// assembly block — a Solidity declaration or a built-in (e.g.
    /// `let msg := 1`).
    ExternalDeclarationShadowing,
}

// Determines whether a Yul `new_definition` being declared under `symbol` in
// `scope_id` is allowed, returning the kind of conflict if not (or `None` if the
// declaration is allowed). See the module docs for the per-declaration-kind
// shadowing rules.
pub(super) fn find_conflicting_yul_definition(
    binder: &Binder,
    scope_id: ScopeId,
    symbol: &str,
    new_definition: &Definition,
) -> Option<YulConflict> {
    // A reserved Yul built-in name may never be declared, regardless of the
    // declaration kind or what's in scope; this takes precedence over any
    // redeclaration or shadowing conflict.
    //
    // TODO: this checks against built-ins across all versions/targets, but we
    // should restrict it to the current version/target.
    if BuiltInsResolver::lookup_yul_global(symbol).is_some() {
        return Some(YulConflict::BuiltInRedeclaration);
    }

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
) -> Option<YulConflict> {
    match binder.get_scope_by_id(scope_id) {
        Scope::YulBlock(yul_block_scope) => yul_block_scope
            .definitions
            .get(symbol)
            .copied()
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .map(|_| YulConflict::Redeclaration)
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
            .map(|_| YulConflict::Redeclaration)
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
        // `find_shadowed_solidity_declaration`).
        _ if new_definition.is_yul_function() => None,
        _ => find_shadowed_solidity_declaration(binder, scope_id, symbol, from_function_signature),
    }
}

// Looks for a Solidity declaration that an assembly-local name would illegally
// shadow, by resolving the name in the enclosing Solidity scope `scope_id`. A
// Yul definition never overloads with a Solidity one, so any resolved Solidity
// definition is a shadow. Resolving (rather than walking scopes by hand) also
// picks up inherited base-contract members, which an assembly-local name may
// likewise not shadow.
//
// `from_function_signature` narrows what counts as a conflict (see the module
// docs): a Yul `let` variable (flag unset) may shadow nothing — neither a
// resolved user declaration nor a Solidity built-in (which `resolve_in_scope`
// never finds, so it's checked separately); a Yul function parameter or return
// variable (flag set) may shadow non-local declarations (state variables,
// constants, functions, built-ins, ...) and only conflicts with a local
// Solidity variable or parameter.
fn find_shadowed_solidity_declaration(
    binder: &Binder,
    scope_id: ScopeId,
    symbol: &str,
    from_function_signature: bool,
) -> Option<YulConflict> {
    let resolution = binder.resolve_in_scope(scope_id, symbol);
    if from_function_signature {
        // A parameter or return may shadow non-local declarations (state
        // variables, functions, built-ins, ...); only a local Solidity variable
        // or parameter is a conflict. solc actually rejects this on a later
        // *reference* to the shadowed name rather than at the declaration, so we
        // report it as a redeclaration approximation (see the module docs).
        let shadows_local = match resolution {
            Resolution::Definition(id) => is_local_variable_or_parameter(binder, id),
            Resolution::Ambiguous(ids) => ids
                .into_iter()
                .any(|id| is_local_variable_or_parameter(binder, id)),
            _ => false,
        };
        shadows_local.then_some(YulConflict::Redeclaration)
    } else {
        // A `let` variable may shadow nothing visible from outside the block:
        // any resolved Solidity declaration, or failing that a Solidity built-in
        // (e.g. `let msg := 1`), is an illegal shadow.
        let shadows_external = matches!(
            resolution,
            Resolution::Definition(_) | Resolution::Ambiguous(_)
        ) || BuiltInsResolver::lookup_global(symbol).is_some();
        shadows_external.then_some(YulConflict::ExternalDeclarationShadowing)
    }
}

// Whether the definition is a local Solidity variable or parameter — the only
// kind of declaration a Yul function parameter or return variable may not
// shadow (state variables, constants, functions, etc. are all shadowable).
fn is_local_variable_or_parameter(binder: &Binder, definition_id: NodeId) -> bool {
    matches!(
        binder.find_definition_by_id(definition_id),
        Some(Definition::Variable(_) | Definition::Parameter(_))
    )
}
