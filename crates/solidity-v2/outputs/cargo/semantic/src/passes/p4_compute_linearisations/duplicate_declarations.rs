//! The duplicate-declaration checks: report two same-named functions (or
//! events) whose parameter lists a caller cannot tell apart, so that naming
//! either of them would be ambiguous.
//!
//! Overloading lets same-named functions and events coexist — that much is
//! settled when they're declared (see the redeclaration checks) — but only
//! while their parameter lists differ. Two overloads are compared the way a
//! caller sees them, so `memory` and `calldata` are interchangeable (the ABI
//! encodes both the same way) while `storage` stays distinct, and return
//! types, mutability, `indexed` and `anonymous` play no part.
//!
//! The two kinds differ in how far the comparison reaches:
//!
//! * functions are only compared against the ones declared alongside them, in
//!   the same contract, interface or library. A function that matches an
//!   *inherited* one is an override question, settled by the override rules.
//! * events are compared across the whole hierarchy: a derived contract's
//!   event and a base's share one ABI slot, so an `emit` naming either is
//!   ambiguous even though the declarations are in different contracts.
//!
//! Both kinds are also compared at file level, over every free function and
//! event *visible* in a file (see [`check_file_scopes`]).

use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::resolution::{
    DuplicateEventDefinition, DuplicateFunctionDefinition,
};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use smallvec::SmallVec;

use super::HierarchyChecker;
use crate::binder::{Binder, Definition, FileScope, Scope};
use crate::context::FileNodeMapper;
use crate::passes::common::conflicts::underlying_declarations;
use crate::types::{Type, TypeId, TypeRegistry};

/// The parameter types deciding whether a declaration duplicates a same-named
/// one. Stored inline: signatures long enough to spill are rare.
pub(super) type ParameterTypes = SmallVec<[TypeId; 4]>;

/// The signatures occupying one name so far. Stored inline in the map entry in
/// the common case of a single declaration per name; only overloaded names
/// spill to the heap.
pub(super) type Signatures = SmallVec<[ParameterTypes; 1]>;

impl<'a> HierarchyChecker<'a> {
    /// Reports this type's own functions that duplicate one declared before
    /// them. Only the declarations written here take part, so this runs for the
    /// head of the linearisation alone — every type in the hierarchy gets its
    /// own [`HierarchyChecker`] run, and so its own turn as the head.
    pub(super) fn check_duplicate_functions(&mut self, members: &[&'a Definition]) {
        let binder = self.binder;
        let types = self.types;
        let file_node_mapper = self.file_node_mapper;
        let diagnostics = &mut *self.diagnostics;

        let mut functions_by_name: Map<&'a str, Signatures> = Map::default();
        for definition in members {
            if !matches!(definition, Definition::Function(_)) {
                continue;
            }
            let Some(parameter_types) = parameter_types_of(binder, types, definition) else {
                continue;
            };
            let signatures = functions_by_name
                .entry(definition.identifier().unparse())
                .or_default();
            if clashes_with_any(types, signatures, &parameter_types) {
                report_duplicate(definition, file_node_mapper, diagnostics);
            } else {
                signatures.push(parameter_types);
            }
        }
    }

    /// Folds this base's events into the per-name state, reporting each one
    /// that duplicates an event already visible in the hierarchy. Bases are
    /// folded most-base-first, so a clash is reported on the more-derived
    /// declaration; recording each event as it is checked means two events
    /// declared side by side in one base clash with each other too.
    pub(super) fn check_duplicate_events(&mut self, members: &[&'a Definition]) {
        let binder = self.binder;
        let types = self.types;
        let file_node_mapper = self.file_node_mapper;
        let diagnostics = &mut *self.diagnostics;
        let events_by_name = &mut self.events_by_name;
        let reported = &mut *self.reported_duplicate;

        for definition in members {
            if !matches!(definition, Definition::Event(_)) {
                continue;
            }
            let Some(parameter_types) = parameter_types_of(binder, types, definition) else {
                continue;
            };
            let signatures = events_by_name
                .entry(definition.identifier().unparse())
                .or_default();
            if !clashes_with_any(types, signatures, &parameter_types) {
                signatures.push(parameter_types);
                continue;
            }
            // A clash inside a shared base is revisited once per contract
            // deriving from it, so each declaration is reported only once. The
            // diagnostic is fully determined by that declaration, so the result
            // doesn't depend on the order contracts are visited in.
            if reported.insert(definition.node_id()) {
                report_duplicate(definition, file_node_mapper, diagnostics);
            }
        }
    }
}

/// Reports duplicates among the file-level functions and events *visible* in
/// each file: the ones it declares itself, plus everything its (transitive)
/// unqualified imports bring into scope, with aliases followed to the
/// declaration they ultimately name.
///
/// A clash between two imported declarations is visible from every file
/// importing both, so declarations already reported are skipped — which also
/// keeps the result independent of the order files are visited in.
pub(super) fn check_file_scopes(
    binder: &Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let mut reported: Set<NodeId> = Set::default();

    for file_scope in binder.file_scopes() {
        let mut visible: Map<&str, Vec<NodeId>> = Map::default();
        collect_visible_declarations(binder, file_scope, &mut visible);

        for definition_ids in visible.values_mut() {
            if definition_ids.len() < 2 {
                continue;
            }
            // Order the overload set by declaration site so that a clash is
            // reported on the later declaration. Declarations imported from
            // several files are ordered by file identifier: arbitrary, but
            // stable across runs.
            definition_ids.sort_by_key(|definition_id| {
                declaration_site(binder, file_node_mapper, *definition_id)
            });

            for (index, definition_id) in definition_ids.iter().enumerate() {
                let definition = binder
                    .find_definition_by_id(*definition_id)
                    .expect("visible declaration is registered");
                let Some(parameter_types) = parameter_types_of(binder, types, definition) else {
                    continue;
                };
                let clashes = definition_ids[..index]
                    .iter()
                    .filter_map(|earlier_id| binder.find_definition_by_id(*earlier_id))
                    // Functions only clash with functions and events with
                    // events; a name shared by both kinds is a redeclaration,
                    // reported when the file scopes are populated.
                    .filter(|earlier| earlier.overloads_with(definition))
                    .any(|earlier| {
                        parameter_types_of(binder, types, earlier).is_some_and(|earlier_types| {
                            types.parameter_lists_are_indistinguishable(
                                &earlier_types,
                                &parameter_types,
                            )
                        })
                    });
                if clashes && reported.insert(*definition_id) {
                    report_duplicate(definition, file_node_mapper, diagnostics);
                }
            }
        }
    }
}

/// Collects the file-level functions and events visible in `file_scope`, keyed
/// by the name they're visible under.
fn collect_visible_declarations<'a>(
    binder: &'a Binder,
    file_scope: &'a FileScope,
    visible: &mut Map<&'a str, Vec<NodeId>>,
) {
    // The closure is only precomputed for files that do have unqualified
    // imports; without any, a file sees just its own declarations.
    if file_scope.default_import_closure.is_empty() {
        record_declarations(binder, file_scope, visible);
        return;
    }
    for scope_id in &file_scope.default_import_closure {
        let Scope::File(imported_scope) = binder.get_scope_by_id(*scope_id) else {
            unreachable!("the import closure only holds file scopes");
        };
        record_declarations(binder, imported_scope, visible);
    }
}

/// Records the functions and events `file_scope` declares, following import
/// aliases so that the same declaration reached under one name through several
/// paths is recorded once rather than clashing with itself.
fn record_declarations<'a>(
    binder: &'a Binder,
    file_scope: &'a FileScope,
    visible: &mut Map<&'a str, Vec<NodeId>>,
) {
    for (symbol, definition_ids) in &file_scope.definitions {
        for definition_id in definition_ids {
            for target_id in underlying_declarations(binder, *definition_id) {
                let Some(target) = binder.find_definition_by_id(target_id) else {
                    continue;
                };
                if !matches!(target, Definition::Function(_) | Definition::Event(_)) {
                    continue;
                }
                let declarations = visible.entry(symbol.as_str()).or_default();
                if !declarations.contains(&target_id) {
                    declarations.push(target_id);
                }
            }
        }
    }
}

/// Locates a declaration, for ordering an overload set spanning several files.
fn declaration_site<'a>(
    binder: &Binder,
    file_node_mapper: &'a FileNodeMapper,
    definition_id: NodeId,
) -> (&'a FileId, usize) {
    let start = binder
        .find_definition_by_id(definition_id)
        .map_or(0, |definition| definition.identifier().range.start);
    (file_node_mapper.file_id_from_node_id(definition_id), start)
}

/// Whether `parameter_types` cannot be told apart from any of `signatures`.
fn clashes_with_any(
    types: &TypeRegistry,
    signatures: &Signatures,
    parameter_types: &ParameterTypes,
) -> bool {
    signatures
        .iter()
        .any(|earlier| types.parameter_lists_are_indistinguishable(earlier, parameter_types))
}

/// The parameter types deciding whether `definition` duplicates a same-named
/// declaration, or `None` when it isn't an overloadable declaration or one of
/// its parameters has no type. An untyped parameter is reported on its own, and
/// treating it as matching anything here would invent a clash.
fn parameter_types_of(
    binder: &Binder,
    types: &TypeRegistry,
    definition: &Definition,
) -> Option<ParameterTypes> {
    match definition {
        Definition::Function(function) => {
            let type_id = binder.node_typing(function.ir_node.id()).as_type_id()?;
            let Type::Function(function_type) = types.get_type_by_id(type_id) else {
                unreachable!("type of function definition is not a function");
            };
            Some(function_type.parameter_types.iter().copied().collect())
        }
        // An event has no type of its own; its parameter types live in its
        // parameters scope, already registered at `memory` — the location an
        // ABI-encoded argument is decoded into — so they compare directly.
        // Neither `indexed` nor `anonymous` reaches a type, so both are ignored.
        Definition::Event(event) => {
            let Scope::Parameters(parameters_scope) =
                binder.get_scope_by_id(event.parameters_scope_id)
            else {
                unreachable!("incorrect scope kind, expected parameters");
            };
            parameters_scope
                .parameters
                .iter()
                .map(|parameter| parameter.type_id)
                .collect()
        }
        _ => None,
    }
}

/// Reports `definition` as a duplicate of an earlier declaration. A function is
/// reported on its signature — its body plays no part in the clash — and an
/// event on its whole declaration.
fn report_duplicate(
    definition: &Definition,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let file_id = file_node_mapper
        .file_id_from_node_id(definition.node_id())
        .to_owned();
    match definition {
        Definition::Function(function) => diagnostics.push(
            file_id,
            function.ir_node.signature_text_range(),
            DuplicateFunctionDefinition,
        ),
        Definition::Event(event) => {
            diagnostics.push(
                file_id,
                event.ir_node.range.clone(),
                DuplicateEventDefinition,
            );
        }
        _ => unreachable!("only functions and events take part in the duplicate checks"),
    }
}
