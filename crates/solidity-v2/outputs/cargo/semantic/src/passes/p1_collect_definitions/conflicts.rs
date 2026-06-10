//! Symbol conflict detection for the definition collection pass. Validates
//! that a definition being declared doesn't illegally redeclare an identifier
//! that is already visible in its scope, either locally or through default
//! imports.

use std::collections::{HashSet, VecDeque};

use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition, FileScope, Scope, ScopeId};

// Looks for a previously-registered definition that conflicts with
// `new_definition` being declared under `symbol` in `scope_id`, returning
// the conflicting definition's `NodeId` (or `None` if the declaration is
// allowed).
//
// The search is bounded to the *local lexical chain*: block, function,
// modifier, parameters and yul scopes delegate to their enclosing local
// scope, but the walk stops at namespace boundaries (contract/file/struct/
// enum). This means shadowing a contract- or file-level member from an
// inner scope is permitted, matching Solidity. Function and event overloads
// are also permitted (see `Definition::overloads_with`).
pub(super) fn find_conflicting_definition(
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
                // Only continue into the parent when this block is a
                // "chained" continuation of the same lexical scope. A new
                // lexical scope (a real `{ }` block or for-init clause) may
                // legally shadow declarations in its enclosing scopes.
                if block_scope.is_lexical_scope {
                    None
                } else {
                    find_conflicting_definition(
                        binder,
                        block_scope.parent_scope_id,
                        symbol,
                        new_definition,
                    )
                }
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
                // Shadowing is not allowed in Yul, so continue lookup in the parent block
                find_conflicting_definition(
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
            .and_then(|existing| conflicting_definition(binder, existing, new_definition)),
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

// Detects clashes between each file's own declarations and the symbols
// brought into its scope through (unqualified) default imports.
//
// Unlike aliased or deconstructed imports — which register a local
// definition and are therefore caught while collecting definitions — a
// default import (`import "file";`) makes *all* of the imported file's
// symbols available without a local definition. A clash can only be
// detected once every file scope is populated, so this runs as a second
// step after all files have been visited.
//
// Returns the list of `(file_id, definition_id)` pairs identifying each
// local definition that conflicts with a default-imported one, ordered
// deterministically (by the given file order, then symbol name).
pub(super) fn find_default_import_conflicts<'a>(
    binder: &Binder,
    file_ids: impl Iterator<Item = &'a str>,
) -> Vec<(String, NodeId)> {
    let mut conflicts: Vec<(String, NodeId)> = Vec::new();

    for file_id in file_ids {
        let file_scope = binder.get_file_scope(file_id);

        let imported_scopes = transitive_default_imports(binder, file_scope);
        if imported_scopes.is_empty() {
            continue;
        }

        // TODO: order here shouldn't matter if we sort the diagnostics
        // collection before reporting, which probably makes more sense since
        // we'll be adding to the collection from multiple points in the
        // pipeline.
        let mut symbols: Vec<&String> = file_scope.definitions.keys().collect();
        symbols.sort();

        for symbol in symbols {
            let imported: Vec<NodeId> = imported_scopes
                .iter()
                .flat_map(|scope| scope.lookup_symbol(symbol))
                .collect();
            if imported.is_empty() {
                continue;
            }

            for &local_id in &file_scope.definitions[symbol] {
                let Some(local_definition) = binder.find_definition_by_id(local_id) else {
                    continue;
                };
                if first_conflicting_definition(binder, &imported, local_definition).is_some() {
                    conflicts.push((file_id.to_owned(), local_id));
                }
            }
        }
    }

    conflicts
}

// Collects the file scopes reachable through the (transitive) default
// imports of `file_scope`, excluding `file_scope` itself. Mutually-recursive
// imports are handled by the `visited` set, which is seeded with the
// starting file so its own scope is never collected (even when reached
// again through a cycle).
fn transitive_default_imports<'a>(
    binder: &'a Binder,
    file_scope: &'a FileScope,
) -> Vec<&'a FileScope> {
    let mut found = Vec::new();
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert(&file_scope.file_id);

    let mut queue: VecDeque<&str> = file_scope
        .imported_files
        .iter()
        .map(String::as_str)
        .collect();

    while let Some(imported_file_id) = queue.pop_front() {
        if !visited.insert(imported_file_id) {
            continue;
        }
        let Some(scope_id) = binder.scope_id_for_file_id(imported_file_id) else {
            continue;
        };
        let Scope::File(imported_scope) = binder.get_scope_by_id(scope_id) else {
            continue;
        };
        found.push(imported_scope);
        queue.extend(imported_scope.imported_files.iter().map(String::as_str));
    }

    found
}
