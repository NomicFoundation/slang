//! Symbol conflict detection for the definition collection pass. Validates
//! that a definition being declared doesn't illegally redeclare an identifier
//! that is already visible in its scope, either locally or through default
//! imports.
//!
//! Although these walks look similar to the `Binder` resolution methods
//! (`resolve_in_scope` and friends), they intentionally don't share code:
//! resolution answers "what is visible here?" while this module answers "may
//! I declare this here?", and those questions traverse different scope
//! relations. Resolution walks the full lexical chain and stops at the first
//! hit; the conflict search stops at lexical/namespace boundaries (shadowing
//! outer Solidity scopes is legal) but continues *past* non-conflicting hits
//! (overloads), and for Yul walks even further than resolution would, since
//! Yul forbids shadowing entirely. Conflict checks also run during pass 1 —
//! before linearisations exist, which contract-scope resolution depends on —
//! and need provenance (which import directive brought a symbol in) that
//! `Resolution` discards.

use std::collections::VecDeque;
use std::ops::Range;

use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition, FileScope, Scope, ScopeId};

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
pub(super) fn find_conflicting_definition(
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
    if new_definition.overloads_with(existing) {
        None
    } else {
        Some(existing_id)
    }
}

// Returns the first of `existing_ids` that conflicts with `new_definition`.
fn first_conflicting_definition(
    binder: &Binder,
    existing_ids: &[NodeId],
    new_definition: &Definition,
) -> Option<NodeId> {
    existing_ids
        .iter()
        .find_map(|existing_id| conflicting_definition(binder, *existing_id, new_definition))
}

// Detects clashes involving the symbols brought into a file's scope through
// (unqualified) default imports.
//
// Unlike aliased or deconstructed imports — which register a local
// definition and are therefore caught while collecting definitions — a
// default import (`import "file";`) makes *all* of the imported file's
// symbols available without a local definition. A clash can only be
// detected once every file scope is populated, so this runs as a second
// step after all files have been visited.
//
// Two kinds of clash are detected for each file: between symbols brought in
// by two different import directives (reported at the later directive, like
// solc does), and between the file's own declarations and an imported symbol
// (reported at the local declaration).
//
// Returns the list of `(file_id, range)` pairs locating each conflict,
// ordered deterministically (by the given file order, then directive order,
// then symbol name).
pub(super) fn find_default_import_conflicts<'a>(
    binder: &Binder,
    file_ids: impl Iterator<Item = &'a FileId>,
) -> Vec<(FileId, Range<usize>)> {
    let mut conflicts: Vec<(FileId, Range<usize>)> = Vec::new();

    for file_id in file_ids {
        let file_scope = binder.get_file_scope(file_id);
        if file_scope.default_imports.is_empty() {
            continue;
        }
        if file_scope.default_imports.len() > 1 {
            // Look for conflicts among imported symbols if there is more than
            // one default import
            find_imported_symbol_conflicts(binder, file_scope, &mut conflicts);
        }
        find_local_definition_conflicts(binder, file_scope, &mut conflicts);
    }

    conflicts
}

// Detects clashes between symbols brought in by two different default import
// directives of `file_scope`. Mirroring solc, directives are processed in
// source order, and a clash is reported at the directive that re-imports an
// already-visible symbol. Re-importing the *same* definition through several
// paths (eg. diamond imports) is legal, as are clashes between symbols
// brought in by a single directive (those are reported when processing the
// imported file itself).
fn find_imported_symbol_conflicts<'a>(
    binder: &'a Binder,
    file_scope: &'a FileScope,
    conflicts: &mut Vec<(FileId, Range<usize>)>,
) {
    // All the symbols imported by the directives processed so far.
    let mut seen: Map<&'a str, Vec<NodeId>> = Map::default();

    let mut import_iter = file_scope.default_imports.iter().peekable();
    while let Some(import) = import_iter.next() {
        let imported_scopes = transitive_file_scopes(binder, &import.file_id, &file_scope.file_id);

        // Gather the symbols this directive brings in. We don't care about them
        // being sorted because if there's any conflict we will report the
        // conflict on the import directive anyway.
        // NOTE: if we ever add secondary diagnostics (eg. "first declared here"
        // information), ordering would be relevant.
        let imported: Vec<(&str, NodeId)> = imported_scopes
            .iter()
            .flat_map(|scope| {
                scope
                    .definitions
                    .iter()
                    .flat_map(|(symbol, ids)| ids.iter().map(|id| (symbol.as_str(), *id)))
            })
            .collect();

        // Look for clashes with already seen symbols from previous imports
        // Skip if there are no seen imported symbols
        if !seen.is_empty() {
            for &(symbol, definition_id) in &imported {
                let Some(seen_ids) = seen.get(symbol) else {
                    continue;
                };
                if seen_ids.contains(&definition_id) {
                    // The same definition is visible through an earlier directive.
                    continue;
                }
                let definition = binder
                    .find_definition_by_id(definition_id)
                    .expect("definition is registered");
                if first_conflicting_definition(binder, seen_ids, definition).is_some() {
                    conflicts.push((file_scope.file_id.clone(), import.range.clone()));
                    // If we found a conflict produced by this import directive,
                    // we report it only once and stop looking for more
                    // conflicts from the same directive.
                    break;
                }
            }
        }

        // Now register imported symbols as seen
        // Skip if this is the last import, as it's unnecessary
        if import_iter.peek().is_none() {
            break;
        }
        for (symbol, definition_id) in imported {
            let seen_ids = seen.entry(symbol).or_default();
            if !seen_ids.contains(&definition_id) {
                seen_ids.push(definition_id);
            }
        }
    }
}

// Detects clashes between `file_scope`'s own declarations and the symbols
// brought into its scope through default imports, reported at the local
// declaration.
fn find_local_definition_conflicts(
    binder: &Binder,
    file_scope: &FileScope,
    conflicts: &mut Vec<(FileId, Range<usize>)>,
) {
    let default_imports_scopes: Vec<_> = file_scope
        .default_imports
        .iter()
        .map(|default_import| {
            let imported_scopes =
                transitive_file_scopes(binder, &default_import.file_id, &file_scope.file_id);
            (default_import, imported_scopes)
        })
        .collect();
    if default_imports_scopes
        .iter()
        .all(|(_, imported_scopes)| imported_scopes.is_empty())
    {
        return;
    }

    let symbols: Vec<&String> = file_scope.definitions.keys().collect();
    for symbol in symbols {
        for (default_import, imported_scopes) in &default_imports_scopes {
            let imported: Vec<NodeId> = imported_scopes
                .iter()
                .flat_map(|scope| scope.lookup_symbol(symbol))
                .collect();
            if imported.is_empty() {
                continue;
            }

            for &local_id in &file_scope.definitions[symbol] {
                let local_definition = binder
                    .find_definition_by_id(local_id)
                    .expect("local definition is registered");
                if first_conflicting_definition(binder, &imported, local_definition).is_some() {
                    // Report the conflict on the definition if it appears later
                    // in the file (most common case). Otherwise, report the
                    // conflict on the import.
                    let conflict_range =
                        if local_definition.identifier().range.start > default_import.range.start {
                            local_definition.identifier().range.clone()
                        } else {
                            default_import.range.clone()
                        };
                    conflicts.push((file_scope.file_id.clone(), conflict_range));
                }
            }
        }
    }
}

// Collects the file scopes reachable through (transitive) default imports
// starting from the `start` file IDs, excluding `excluded_file_id`.
// Mutually-recursive imports are handled by the `visited` set, which is
// seeded with the excluded file so its scope is never collected (even when
// reached through a cycle).
fn transitive_file_scopes<'a>(
    binder: &'a Binder,
    starting_file_id: &FileId,
    excluded_file_id: &FileId,
) -> Vec<&'a FileScope> {
    let mut found = Vec::new();
    let mut visited = Set::default();
    visited.insert(excluded_file_id);

    let mut queue: VecDeque<&FileId> = [starting_file_id].into_iter().collect();

    while let Some(imported_file_id) = queue.pop_front() {
        if !visited.insert(imported_file_id) {
            continue;
        }
        let Some(scope_id) = binder.scope_id_for_file_id(imported_file_id) else {
            continue;
        };
        let Scope::File(imported_scope) = binder.get_scope_by_id(scope_id) else {
            unreachable!("expected a file scope");
        };
        found.push(imported_scope);
        queue.extend(
            imported_scope
                .default_imports
                .iter()
                .map(|import| &import.file_id),
        );
    }

    found
}
