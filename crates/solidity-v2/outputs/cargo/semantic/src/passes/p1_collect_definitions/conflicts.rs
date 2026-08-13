//! Symbol conflict detection for the definition collection pass.
//!
//! [`find_conflicting_solidity_definition`] is the scope-walk used when
//! registering each Solidity definition: it answers "may I declare this name
//! here?". The search is bounded to the *local lexical chain* — it stops at
//! lexical and namespace boundaries (block/contract/struct/enum), since
//! shadowing an outer Solidity scope is legal, and continues *past*
//! non-conflicting hits (overloads).
//!
//! The rest of this module handles *file-level* clashes, via
//! [`find_file_scope_conflicts`]. Those are separate because they run as a
//! second step once every file scope is populated: deciding whether two
//! file-level names clash may require following an import alias into a file
//! that hasn't been visited yet, and clashes involving default imports need
//! provenance (which import directive brought a symbol in) that `Resolution`
//! discards.
//!
//! Both parts share the same pairwise primitives from
//! [`crate::passes::common::conflicts`]; the file-level detectors wrap them
//! with alias-following (see [`aliased_definitions_conflict`]).

use std::collections::VecDeque;
use std::ops::Range;

use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, DefaultImport, Definition, FileScope, Scope, ScopeId};
use crate::passes::common::conflicts::{
    conflicting_definition, first_conflicting_definition, underlying_declarations,
};

// Looks for a previously-registered definition that conflicts with a Solidity
// `new_definition` being declared under `symbol` in `scope_id`, returning the
// conflicting definition's `NodeId` (or `None` if the declaration is allowed).
//
// The search is bounded to the local lexical chain: chained, function, modifier
// and parameter scopes delegate to their enclosing local scope, but the walk
// stops at lexical and namespace boundaries (block/contract/file/struct/enum).
// This means shadowing a declaration from an inner lexical scope, or a
// contract- or file-level member from a local scope, is permitted, matching
// Solidity. Function and event overloads are also permitted (see
// `Definition::overloads_with`).
pub(super) fn find_conflicting_solidity_definition(
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
            .lookup_definition(symbol)
            .and_then(|existing| conflicting_definition(binder, existing, new_definition)),
        // A chained scope is a continuation of the parent's lexical scope, so
        // continue the search into the parent.
        Scope::Chained(chained_scope) => chained_scope
            .lookup_definition(symbol)
            .and_then(|existing| conflicting_definition(binder, existing, new_definition))
            .or_else(|| {
                find_conflicting_solidity_definition(
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
                find_conflicting_solidity_definition(
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
        // Namespace scopes are only checked against their own definitions;
        // the `Vec`-based ones may hold several entries for a symbol (eg.
        // function/event overloads).
        Scope::Contract(contract_scope) => contract_scope
            .definitions
            .get(symbol)
            .and_then(|existing| first_conflicting_definition(binder, existing, new_definition)),
        // File-scope conflicts are detected by `find_file_scope_conflicts`
        // once every file scope is populated, so the walk never starts (nor
        // ends up) here.
        Scope::File(_) => {
            unreachable!("file-scope conflicts are checked after all files are visited")
        }
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
        // A Solidity definition is never declared inside a Yul scope.
        Scope::YulBlock(_) | Scope::YulFunction(_) => {
            unreachable!("Solidity definitions are not declared in Yul scopes")
        }
    }
}

// Detects redeclaration clashes at file scope, once every file scope has been
// populated.
//
// This covers three kinds of clash:
//
//  * between two of a file's own declarations sharing a name — including
//    aliased/deconstructed imported symbols, which register a local definition;
//  * between symbols brought in by two different (unqualified) default import
//    directives (reported at the later directive, like solc does);
//  * between the file's own declarations and a symbol brought into scope
//    through a default import (reported at the local declaration).
//
// Every comparison follows import aliases to the underlying declarations, so
// re-importing the *same* declaration through several paths is idempotent, and
// several free functions (or events) sharing a name form an overload set rather
// than a redeclaration — matching solc.
//
// Returns the list of `(file_id, range)` pairs locating each conflict.
pub(super) fn find_file_scope_conflicts<'a>(
    binder: &Binder,
    file_ids: impl Iterator<Item = &'a FileId>,
) -> Vec<(FileId, Range<usize>)> {
    let mut conflicts: Vec<(FileId, Range<usize>)> = Vec::new();

    for file_id in file_ids {
        let file_scope = binder.get_file_scope(file_id);

        find_own_declaration_conflicts(binder, file_scope, &mut conflicts);

        if file_scope.default_imports.is_empty() {
            continue;
        }

        // Collect the scopes each directive transitively brings in once; both
        // import detectors below iterate them.
        let directive_scopes: Vec<(&DefaultImport, Vec<&FileScope>)> = file_scope
            .default_imports
            .iter()
            .map(|import| {
                let imported_scopes =
                    transitive_file_scopes(binder, &import.file_id, &file_scope.file_id);
                (import, imported_scopes)
            })
            .collect();

        if directive_scopes.len() > 1 {
            // Look for conflicts among imported symbols if there is more than
            // one default import
            find_imported_symbol_conflicts(binder, file_scope, &directive_scopes, &mut conflicts);
        }
        find_local_definition_conflicts(binder, file_scope, &directive_scopes, &mut conflicts);
    }

    conflicts
}

// Whether declaring `new_id` under the same name as the already-visible
// `existing_id` is a redeclaration, resolving both through any import aliases
// first. They may legally coexist when every underlying declaration pair is
// either the very same declaration (an idempotent re-import) or a legal
// overload, as decided by the shared `conflicting_definition` primitive.
fn aliased_definitions_conflict(binder: &Binder, existing_id: NodeId, new_id: NodeId) -> bool {
    let existing_declarations = underlying_declarations(binder, existing_id);

    underlying_declarations(binder, new_id).iter().any(|&new| {
        let new_definition = binder
            .find_definition_by_id(new)
            .expect("definition is registered");
        existing_declarations
            .iter()
            // The same underlying declaration reached through both names is
            // not a conflict.
            .filter(|&&existing| existing != new)
            .any(|&existing| conflicting_definition(binder, existing, new_definition).is_some())
    })
}

// Returns whether `new_id` conflicts with any of `existing_ids`; the
// alias-following counterpart of `first_conflicting_definition`.
fn any_aliased_conflict(binder: &Binder, existing_ids: &[NodeId], new_id: NodeId) -> bool {
    existing_ids
        .iter()
        .any(|&existing_id| aliased_definitions_conflict(binder, existing_id, new_id))
}

// Detects clashes between two of a file's own declarations sharing a name. Each
// declaration is checked against the earlier ones (in source order), and a
// clash is reported at the later declaration, matching solc. Aliased imported
// symbols participate here too, following their aliases so that idempotent
// re-imports and function/event overload sets don't count as redeclarations.
fn find_own_declaration_conflicts(
    binder: &Binder,
    file_scope: &FileScope,
    conflicts: &mut Vec<(FileId, Range<usize>)>,
) {
    for definition_ids in file_scope.definitions.values() {
        if definition_ids.len() < 2 {
            continue;
        }
        for (index, &new_id) in definition_ids.iter().enumerate() {
            if any_aliased_conflict(binder, &definition_ids[..index], new_id) {
                let new_definition = binder
                    .find_definition_by_id(new_id)
                    .expect("definition is registered");
                conflicts.push((
                    file_scope.file_id.clone(),
                    new_definition.identifier().range.clone(),
                ));
            }
        }
    }
}

// Detects clashes between symbols brought in by two different default import
// directives of `file_scope`. Mirroring solc, directives are processed in
// source order, and a clash is reported at the directive that re-imports an
// already-visible symbol. Re-importing the *same* definition through several
// paths (eg. diamond imports) is legal, as are clashes between symbols
// brought in by a single directive (those are reported when processing the
// imported file itself).
fn find_imported_symbol_conflicts<'a>(
    binder: &Binder,
    file_scope: &FileScope,
    directive_scopes: &[(&DefaultImport, Vec<&'a FileScope>)],
    conflicts: &mut Vec<(FileId, Range<usize>)>,
) {
    // All the symbols imported by the directives processed so far.
    let mut seen: Map<&'a str, Vec<NodeId>> = Map::default();

    let mut import_iter = directive_scopes.iter().peekable();
    while let Some((import, imported_scopes)) = import_iter.next() {
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
                if any_aliased_conflict(binder, seen_ids, definition_id) {
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
    directive_scopes: &[(&DefaultImport, Vec<&FileScope>)],
    conflicts: &mut Vec<(FileId, Range<usize>)>,
) {
    if directive_scopes
        .iter()
        .all(|(_, imported_scopes)| imported_scopes.is_empty())
    {
        return;
    }

    let symbols: Vec<&String> = file_scope.definitions.keys().collect();
    for symbol in symbols {
        for (default_import, imported_scopes) in directive_scopes {
            let imported: Vec<NodeId> = imported_scopes
                .iter()
                .flat_map(|scope| scope.lookup_symbol(symbol))
                .collect();
            if imported.is_empty() {
                continue;
            }

            for &local_id in &file_scope.definitions[symbol] {
                if any_aliased_conflict(binder, &imported, local_id) {
                    // Report the conflict on the definition if it appears later
                    // in the file (most common case). Otherwise, report the
                    // conflict on the import.
                    let local_definition_range = &binder
                        .find_definition_by_id(local_id)
                        .expect("local definition is registered")
                        .identifier()
                        .range;
                    let conflict_range =
                        if local_definition_range.start > default_import.range.start {
                            local_definition_range.clone()
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
