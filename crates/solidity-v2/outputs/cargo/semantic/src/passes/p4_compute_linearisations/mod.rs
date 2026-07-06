use std::cmp::Ordering;
use std::sync::Arc;

use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::diagnostics::kinds::resolution::IdentifierRedeclaration;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::{ContractData, ContractLinearisations, FileNodeMapper};
use crate::types::TypeRegistry;

/// In this pass we walk every contract's and interface's linearised bases once,
/// accumulating the members visible across the hierarchy. From that single walk
/// we both:
///
/// - report `IdentifierRedeclaration` for a member that illegally redeclares a
///   same-named member inherited from a base (Solidity allows overloading
///   functions/events and overriding functions/modifiers; see
///   [`Definition::may_coexist_with_inherited`]); and
/// - pre-compute, for each contract, the collections of functions, state
///   variables, errors and events visible in its hierarchy, storing them in a
///   `ContractData`.
///
/// This pass depends on all definitions being fully typed, which is accomplished
/// in the previous pass. The next pass is not strictly dependant on the result
/// of this pass. Eventually we could use the linearisation information produced
/// here to aid in expressions/statements resolution and typing, but it's fully
/// independent for now.
pub fn run(
    binder: &Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) -> ContractData {
    let mut contracts = Vec::new();
    let mut linearisations_by_id = Map::default();
    // A redeclaration in a shared base is visited once per descendant, so we
    // dedup by the redeclaring member's node to report each clash only once.
    // The emitted diagnostic is fully determined by that node, so the result is
    // independent of the order in which we visit contracts.
    let mut reported = Set::default();

    for (definition_id, definition) in binder.definitions() {
        match definition {
            Definition::Contract(contract) => {
                contracts.push(Arc::clone(&contract.ir_node));
                let linearisations = compute_linearised_members(
                    binder,
                    types,
                    *definition_id,
                    file_node_mapper,
                    diagnostics,
                    &mut reported,
                );
                linearisations_by_id.insert(*definition_id, linearisations);
            }
            // Interfaces don't get a `ContractData` entry, but their members
            // still take part in redeclaration checks, so we walk them too and
            // discard the (mostly empty) collections they produce.
            Definition::Interface(_) => {
                compute_linearised_members(
                    binder,
                    types,
                    *definition_id,
                    file_node_mapper,
                    diagnostics,
                    &mut reported,
                );
            }
            _ => {}
        }
    }

    ContractData::new(contracts, linearisations_by_id)
}

/// Walks `definition_id`'s linearised bases once, both reporting inherited
/// redeclarations and building the contract's linearised member collections.
///
/// The bases are visited most-base-first, so that a redeclaration is reported
/// on the more-derived member (matching solc) and state variables, errors and
/// events come out in base-to-derived source order.
fn compute_linearised_members(
    binder: &Binder,
    types: &TypeRegistry,
    definition_id: NodeId,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
    reported: &mut Set<NodeId>,
) -> ContractLinearisations {
    let Some(linearised_bases) = binder.get_linearised_bases(definition_id) else {
        return ContractLinearisations::default();
    };

    // The members visible so far, keyed by name; a base contributes only its
    // members that derived types inherit. A member redeclares an inherited one
    // when it can't coexist with any member already recorded under its name.
    let mut visible_members_by_name: Map<String, Vec<NodeId>> = Map::default();

    let mut state_variables = Vec::new();
    let mut errors = Vec::new();
    let mut events = Vec::new();
    // Functions are gathered per base so `linearise_functions` can later flatten
    // them most-derived-first.
    let mut functions_per_base: Vec<Vec<ir::FunctionDefinition>> = Vec::new();

    for base_id in linearised_bases.iter().rev() {
        let (members, from_interface) = match binder.find_definition_by_id(*base_id) {
            Some(Definition::Contract(contract)) => (&contract.ir_node.members, false),
            Some(Definition::Interface(interface)) => (&interface.ir_node.members, true),
            _ => continue,
        };
        // The head of the linearisation is the type we're computing for; every
        // other entry is one of its proper bases.
        let declared_here = *base_id == definition_id;

        let mut base_functions = Vec::new();
        // The definitions of this base's named members, gathered in a single
        // pass and reused below to both check for and record redeclarations.
        let mut member_definitions = Vec::new();

        for member in members {
            match member {
                // Interfaces don't contribute functions to the linearisation:
                // they must be implemented by inheriting contracts (enforced
                // elsewhere).
                ir::ContractMember::FunctionDefinition(function)
                    if !from_interface
                        && matches!(
                            function.kind,
                            ir::FunctionKind::Regular
                                | ir::FunctionKind::Fallback
                                | ir::FunctionKind::Receive
                        ) =>
                {
                    base_functions.push(Arc::clone(function));
                }
                // Interfaces don't have state variables in Solidity.
                ir::ContractMember::StateVariableDefinition(state_variable) if !from_interface => {
                    state_variables.push(Arc::clone(state_variable));
                }
                ir::ContractMember::ErrorDefinition(error) => errors.push(Arc::clone(error)),
                ir::ContractMember::EventDefinition(event) => events.push(Arc::clone(event)),
                _ => {}
            }

            if let Some(definition) = member_definition(binder, member) {
                member_definitions.push(definition);
            }
        }

        // Check this base's members against everything inherited from
        // less-derived bases. Only members in this type's namespace can clash:
        // its own declarations (even `private` ones, which still shadow a
        // visible inherited member), plus a proper base's members that are
        // visible to it. A proper base's own private member isn't inherited, so
        // it never clashes.
        for definition in &member_definitions {
            if !declared_here && !definition.is_internally_visible() {
                continue;
            }
            let name = definition.identifier().unparse();
            let Some(inherited) = visible_members_by_name.get(name) else {
                continue;
            };
            let redeclares = inherited.iter().any(|inherited_id| {
                let inherited_definition = binder.find_definition_by_id(*inherited_id).unwrap();
                !definition.may_coexist_with_inherited(inherited_definition)
            });
            if redeclares && reported.insert(definition.node_id()) {
                let file_id = file_node_mapper.file_id_from_node_id(definition.node_id());
                diagnostics.push(
                    file_id.to_owned(),
                    definition.identifier().range.clone(),
                    IdentifierRedeclaration,
                );
            }
        }

        // Record this base's inheritable members so more-derived bases are
        // checked against them. Private members (and external functions) aren't
        // inherited, so reusing their name is allowed. Recording only after the
        // checks above keeps members declared together in the same base from
        // clashing with each other (a same-scope concern handled in `p1`).
        for definition in member_definitions {
            if definition.is_internally_visible() {
                visible_members_by_name
                    .entry(definition.identifier().unparse().to_owned())
                    .or_default()
                    .push(definition.node_id());
            }
        }

        functions_per_base.push(base_functions);
    }

    ContractLinearisations {
        functions: linearise_functions(binder, types, &functions_per_base),
        state_variables,
        errors,
        events,
    }
}

/// Flattens the per-base function lists (gathered most-base-first) into the
/// hierarchy's function list: most-derived-first, dropping a function once a
/// more-derived one overrides it, then sorted by name.
fn linearise_functions(
    binder: &Binder,
    types: &TypeRegistry,
    functions_per_base: &[Vec<ir::FunctionDefinition>],
) -> Vec<ir::FunctionDefinition> {
    let mut functions: Vec<ir::FunctionDefinition> = Vec::new();
    for base_functions in functions_per_base.iter().rev() {
        for function in base_functions {
            let already_overridden = functions
                .iter()
                .any(|kept| function_overrides(binder, types, kept, function));
            if !already_overridden {
                functions.push(Arc::clone(function));
            }
            // TODO(validation): if overriding, function must have the `override` specifier and the overriden functions must be marked `virtual`
            // TODO(validation): if overriding multiple ancestors, the function needs to specify the bases in a specifier
        }
    }
    functions.sort_by(|a, b| match (&a.name, &b.name) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a), Some(b)) => a.unparse().cmp(b.unparse()),
    });
    functions
}

/// Returns the `Definition` naming `member`, or `None` for members that don't
/// introduce a name into the contract namespace (using directives, and unnamed
/// functions such as constructors, fallback and receive).
fn member_definition<'a>(
    binder: &'a Binder,
    member: &ir::ContractMember,
) -> Option<&'a Definition> {
    let node_id = match member {
        ir::ContractMember::UsingDirective(_) => return None,
        ir::ContractMember::FunctionDefinition(function) => function.id(),
        ir::ContractMember::StructDefinition(struct_) => struct_.id(),
        ir::ContractMember::EnumDefinition(enum_) => enum_.id(),
        ir::ContractMember::EventDefinition(event) => event.id(),
        ir::ContractMember::ErrorDefinition(error) => error.id(),
        ir::ContractMember::UserDefinedValueTypeDefinition(udvt) => udvt.id(),
        ir::ContractMember::StateVariableDefinition(variable) => variable.id(),
        ir::ContractMember::ConstantDefinition(constant) => constant.id(),
    };
    binder.find_definition_by_id(node_id)
}

/// Whether `overriding` overrides `overridden`: they share a name (or are the
/// same kind of unnamed function) and their signatures are in an override
/// relationship.
fn function_overrides(
    binder: &Binder,
    types: &TypeRegistry,
    overriding: &ir::FunctionDefinition,
    overridden: &ir::FunctionDefinition,
) -> bool {
    let name_matches = match (&overriding.name, &overridden.name) {
        (None, None) => overriding.kind == overridden.kind,
        (Some(name), Some(other_name)) => name.unparse() == other_name.unparse(),
        _ => false,
    };
    if !name_matches {
        return false;
    }
    let overriding_type_id = binder.node_typing(overriding.id()).as_type_id();
    let overridden_type_id = binder.node_typing(overridden.id()).as_type_id();
    match (overriding_type_id, overridden_type_id) {
        (Some(overriding_type_id), Some(overridden_type_id)) => {
            types.type_id_is_function_and_overrides(overriding_type_id, overridden_type_id)
        }
        _ => false,
    }
    // TODO(validation) SDR[6]: check also that the function mutability is stricter than the overridden one's
}
