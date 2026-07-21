//! Pre-computing the collections of members visible in a contract's hierarchy:
//! its state variables, errors and events in base-to-derived source order, and
//! its functions flattened most-derived-first, resolving overrides.

use std::cmp::Ordering;
use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::ContractLinearisations;
use crate::types::{TypeId, TypeRegistry};

/// Walks the contract's linearised bases in reverse (most-base-first) and
/// gathers the members visible in its hierarchy.
pub(super) fn compute_linearisations(
    binder: &Binder,
    types: &TypeRegistry,
    contract_id: NodeId,
) -> ContractLinearisations {
    let Some(linearised_bases) = binder.get_linearised_bases(contract_id) else {
        return ContractLinearisations::default();
    };

    // The members of each *contract* base, gathered most-base-first, so the
    // hierarchy's functions can be flattened most-derived-first at the end.
    // Interface bases are excluded since they don't contribute functions to the
    // linearisation: they must be implemented by inheriting contracts (enforced
    // by the abstractness check).
    let mut contract_base_members = Vec::with_capacity(linearised_bases.len());
    let mut state_variables = Vec::new();
    let mut errors = Vec::new();
    let mut events = Vec::new();

    for base_id in linearised_bases.iter().rev() {
        let (members, base_is_interface) = match binder.find_definition_by_id(*base_id) {
            Some(Definition::Contract(contract)) => (contract.ir_node.members.as_slice(), false),
            Some(Definition::Interface(interface)) => (interface.ir_node.members.as_slice(), true),
            _ => unreachable!("base should be a contract or interface"),
        };

        for member in members {
            match member {
                // Interfaces don't have state variables in Solidity.
                ir::ContractMember::StateVariableDefinition(state_variable)
                    if !base_is_interface =>
                {
                    state_variables.push(Arc::clone(state_variable));
                }
                ir::ContractMember::ErrorDefinition(error) => errors.push(Arc::clone(error)),
                ir::ContractMember::EventDefinition(event) => events.push(Arc::clone(event)),
                _ => {}
            }
        }

        if !base_is_interface {
            contract_base_members.push(members);
        }
    }

    ContractLinearisations {
        functions: linearise_functions(binder, types, &contract_base_members),
        state_variables,
        errors,
        events,
    }
}

/// Flattens the contract bases' members (gathered most-base-first) into the
/// hierarchy's function list: most-derived-first, dropping a function once a
/// more-derived function or a public state variable's getter overrides it, then
/// sorted by name. Functions are cloned out only once they're known to survive
/// override resolution.
fn linearise_functions(
    binder: &Binder,
    types: &TypeRegistry,
    contract_base_members: &[&[ir::ContractMember]],
) -> Vec<ir::FunctionDefinition> {
    let mut functions: Vec<&ir::FunctionDefinition> = Vec::new();
    let mut getters: Vec<GetterSlot<'_>> = Vec::new();
    for members in contract_base_members.iter().rev() {
        for member in *members {
            match member {
                ir::ContractMember::FunctionDefinition(function)
                    if matches!(
                        function.kind,
                        ir::FunctionKind::Regular
                            | ir::FunctionKind::Fallback
                            | ir::FunctionKind::Receive
                    ) =>
                {
                    let already_overridden = functions
                        .iter()
                        .any(|kept| function_overrides(binder, types, kept, function))
                        || getters
                            .iter()
                            .any(|getter| getter_overrides(binder, types, getter, function));
                    if !already_overridden {
                        functions.push(function);
                    }
                    // TODO(validation): if overriding, function must have the `override` specifier
                    // and the overriden functions must be marked `virtual`
                    // TODO(validation): if overriding multiple ancestors, the function needs to
                    // specify the bases in a specifier
                }
                ir::ContractMember::StateVariableDefinition(state_variable) => {
                    // Record its getter, if it has one, so it can shadow a
                    // matching function inherited from a base contract.
                    getters.extend(getter_slot(binder, state_variable));
                }
                _ => {}
            }
        }
    }
    functions.sort_by(|a, b| match (&a.name, &b.name) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a), Some(b)) => a.unparse().cmp(b.unparse()),
    });
    functions.into_iter().map(Arc::clone).collect()
}

/// A public state variable's generated getter, reduced to what we need to tell
/// whether it overrides a function. Just its name and its function type.
struct GetterSlot<'a> {
    name: &'a str,
    type_id: TypeId,
}

/// The [`GetterSlot`] for `state_variable`, or `None` when it has no getter.
/// Only public state variables have a getter, and `getter_type_id` is set only
/// for those, so unwrapping it below is what filters the non-public ones out.
fn getter_slot<'a>(
    binder: &Binder,
    state_variable: &'a ir::StateVariableDefinition,
) -> Option<GetterSlot<'a>> {
    match binder.find_definition_by_id(state_variable.id()) {
        Some(Definition::StateVariable(definition)) => Some(GetterSlot {
            name: state_variable.name.unparse(),
            type_id: definition.getter_type_id?,
        }),
        _ => None,
    }
}

/// Whether `getter` overrides `function`. True when they share a name and the
/// getter's type can override the function's. The getter version of
/// [`function_overrides`].
fn getter_overrides(
    binder: &Binder,
    types: &TypeRegistry,
    getter: &GetterSlot<'_>,
    function: &ir::FunctionDefinition,
) -> bool {
    function
        .name
        .as_ref()
        .is_some_and(|name| name.unparse() == getter.name)
        && binder
            .node_typing(function.id())
            .as_type_id()
            .is_some_and(|function_type_id| {
                types.type_id_is_function_and_overrides(getter.type_id, function_type_id)
            })
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
        (Some(name), Some(other_name)) => {
            debug_assert!(
                overriding.kind == overridden.kind && overriding.kind == ir::FunctionKind::Regular,
                "compared functions are both regular"
            );
            name.unparse() == other_name.unparse()
        }
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
