use std::cmp::Ordering;
use std::sync::Arc;

use slang_solidity_v2_common::collections::Map;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::{ContractData, ContractLinearisations};
use crate::types::TypeRegistry;

/// In this pass we walk every contract's linearised bases and pre-compute the
/// collections of functions, state variables, errors and events visible in the
/// contract's hierarchy, storing them in a `ContractData`.
///
/// This pass depends on all definitions being fully typed, which is
/// accomplished in the previous pass. The next pass is not strictly dependant
/// on the result of this pass. Eventually we could use the linearisation
/// information produced by this pass to aid in expressions/statements
/// resolution and typing, but it's fully independent for now.
pub fn run(binder: &Binder, types: &TypeRegistry) -> ContractData {
    let mut contracts = Vec::new();
    let mut data = Map::default();

    for (contract_id, definition) in binder.definitions() {
        let Definition::Contract(contract) = definition else {
            continue;
        };
        let contract_id = *contract_id;

        let ir_node = Arc::clone(&contract.ir_node);
        contracts.push(ir_node);

        let linearisations = compute_linearised_members(binder, types, contract_id);
        data.insert(contract_id, linearisations);
    }

    ContractData::new(contracts, data)
}

fn compute_linearised_members(
    binder: &Binder,
    types: &TypeRegistry,
    contract_id: NodeId,
) -> ContractLinearisations {
    let functions = collect_linearised_functions(binder, types, contract_id);
    let state_variables = collect_linearised_state_variables(binder, contract_id);
    let errors = collect_linearised_errors(binder, contract_id);
    let events = collect_linearised_events(binder, contract_id);

    // TODO(validation): check that there is no redefinition of identifiers among all contracts in the hierarchy

    ContractLinearisations {
        functions,
        state_variables,
        errors,
        events,
    }
}

fn collect_linearised_functions(
    binder: &Binder,
    types: &TypeRegistry,
    contract_id: NodeId,
) -> Vec<ir::FunctionDefinition> {
    let Some(linearised_bases) = binder.get_linearised_bases(contract_id) else {
        return Vec::new();
    };

    let mut functions: Vec<ir::FunctionDefinition> = Vec::new();
    for base_id in linearised_bases {
        // TODO(validation) SDR[3]: we don't pick up functions defined in
        // interfaces because they should be implemented in inheriting contracts,
        // but this is not yet enforced anywhere.
        let Some(Definition::Contract(base)) = binder.find_definition_by_id(*base_id) else {
            continue;
        };

        for member in &base.ir_node.members {
            let ir::ContractMember::FunctionDefinition(function) = member else {
                continue;
            };
            if !matches!(
                function.kind,
                ir::FunctionKind::Regular | ir::FunctionKind::Fallback | ir::FunctionKind::Receive
            ) {
                continue;
            }
            let overridden = functions
                .iter()
                .any(|existing| function_overrides(binder, types, existing, function));
            if !overridden {
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

fn function_overrides(
    binder: &Binder,
    types: &TypeRegistry,
    function: &ir::FunctionDefinition,
    other: &ir::FunctionDefinition,
) -> bool {
    let name_matches = match (&function.name, &other.name) {
        (None, None) => function.kind == other.kind,
        (Some(name), Some(other_name)) => name.unparse() == other_name.unparse(),
        _ => false,
    };
    if !name_matches {
        return false;
    }
    let type_id = binder.node_typing(function.id()).as_type_id();
    let other_type_id = binder.node_typing(other.id()).as_type_id();
    match (type_id, other_type_id) {
        (Some(type_id), Some(other_type_id)) => {
            types.type_id_is_function_and_overrides(type_id, other_type_id)
        }
        _ => false,
    }
    // TODO(validation) SDR[6]: check also that the function mutability is stricter than other's
}

/// Walks the linearised bases in reverse (most-base first) and concatenates
/// every contract's state-variable members in source order. Interfaces don't
/// contribute state variables in Solidity.
fn collect_linearised_state_variables(
    binder: &Binder,
    contract_id: NodeId,
) -> Vec<ir::StateVariableDefinition> {
    let mut state_variables = Vec::new();
    let Some(linearised_bases) = binder.get_linearised_bases(contract_id) else {
        return state_variables;
    };
    for base_id in linearised_bases.iter().rev() {
        let Some(Definition::Contract(base)) = binder.find_definition_by_id(*base_id) else {
            continue;
        };
        for member in &base.ir_node.members {
            if let ir::ContractMember::StateVariableDefinition(state_variable) = member {
                state_variables.push(Arc::clone(state_variable));
            }
            // TODO(validation): check for duplicate declarations
            // TODO(validation): if the state variable is `public`, check if it
            // overrides a function and validate the presence of `override` (in
            // the variable) and `virtual` (in the function) modifiers
        }
    }
    state_variables
}

/// Walks the linearised bases in reverse and concatenates every contract /
/// interface error definition.
fn collect_linearised_errors(binder: &Binder, contract_id: NodeId) -> Vec<ir::ErrorDefinition> {
    let mut errors = Vec::new();
    let Some(linearised_bases) = binder.get_linearised_bases(contract_id) else {
        return errors;
    };
    for base_id in linearised_bases.iter().rev() {
        let members = match binder.find_definition_by_id(*base_id) {
            Some(Definition::Contract(base)) => &base.ir_node.members,
            Some(Definition::Interface(base)) => &base.ir_node.members,
            _ => continue,
        };
        for member in members {
            if let ir::ContractMember::ErrorDefinition(error) = member {
                errors.push(Arc::clone(error));
            }
            // TODO(validation): check for duplicate declarations
        }
    }
    errors
}

/// Walks the linearised bases in reverse and concatenates every contract /
/// interface event definition.
fn collect_linearised_events(binder: &Binder, contract_id: NodeId) -> Vec<ir::EventDefinition> {
    let mut events = Vec::new();
    let Some(linearised_bases) = binder.get_linearised_bases(contract_id) else {
        return events;
    };
    for base_id in linearised_bases.iter().rev() {
        let members = match binder.find_definition_by_id(*base_id) {
            Some(Definition::Contract(base)) => &base.ir_node.members,
            Some(Definition::Interface(base)) => &base.ir_node.members,
            _ => continue,
        };
        for member in members {
            if let ir::ContractMember::EventDefinition(event) = member {
                events.push(Arc::clone(event));
            }
            // TODO(validation): check for duplicate declarations, considering
            // the full signature since events can be overloaded
        }
    }
    events
}
