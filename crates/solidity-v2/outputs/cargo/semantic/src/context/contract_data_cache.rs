use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::types::TypeRegistry;

/// Pre-computed derived data for a single contract.
#[allow(clippy::struct_field_names)]
struct ContractData {
    linearised_functions: Vec<ir::FunctionDefinition>,
    linearised_state_variables: Vec<ir::StateVariableDefinition>,
    linearised_errors: Vec<ir::ErrorDefinition>,
    linearised_events: Vec<ir::EventDefinition>,
}

/// Cache of derived data about contracts, computed once at the end of the
/// semantic passes and stored on the `SemanticContext`. Every contract's
/// `NodeId` has an entry in `data`.
pub(super) struct ContractDataCache {
    /// All contract definitions in this compilation unit, in registration
    /// order (deterministic iteration for `all_contracts`).
    contracts: Vec<ir::ContractDefinition>,
    /// Per-contract derived data, keyed by contract `NodeId`.
    data: HashMap<NodeId, ContractData>,
}

impl ContractDataCache {
    pub(super) fn build_from(binder: &Binder, types: &TypeRegistry) -> Self {
        let mut contracts = Vec::new();
        let mut data = HashMap::new();

        for (contract_id, definition) in binder.definitions() {
            let Definition::Contract(contract) = definition else {
                continue;
            };
            let contract_id = *contract_id;
            let ir_node = Arc::clone(&contract.ir_node);

            let linearised_functions = compute_linearised_functions(binder, types, contract_id);
            let linearised_state_variables =
                collect_linearised_state_variables(binder, contract_id);
            let linearised_errors = collect_linearised_errors(binder, contract_id);
            let linearised_events = collect_linearised_events(binder, contract_id);

            data.insert(
                contract_id,
                ContractData {
                    linearised_functions,
                    linearised_state_variables,
                    linearised_errors,
                    linearised_events,
                },
            );

            contracts.push(ir_node);
        }

        Self { contracts, data }
    }

    fn get(&self, contract_id: NodeId) -> &ContractData {
        self.data
            .get(&contract_id)
            .expect("contract_id is a registered contract")
    }

    pub(super) fn all_contracts(&self) -> impl Iterator<Item = &ir::ContractDefinition> {
        self.contracts.iter()
    }

    pub(super) fn find_contract_by_name<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = ir::ContractDefinition> + use<'a> {
        self.contracts
            .iter()
            .filter(move |contract| contract.name.unparse() == name)
            .cloned()
    }

    pub(super) fn linearised_functions(&self, contract_id: NodeId) -> &[ir::FunctionDefinition] {
        &self.get(contract_id).linearised_functions
    }

    pub(super) fn linearised_state_variables(
        &self,
        contract_id: NodeId,
    ) -> &[ir::StateVariableDefinition] {
        &self.get(contract_id).linearised_state_variables
    }

    pub(super) fn linearised_errors(&self, contract_id: NodeId) -> &[ir::ErrorDefinition] {
        &self.get(contract_id).linearised_errors
    }

    pub(super) fn linearised_events(&self, contract_id: NodeId) -> &[ir::EventDefinition] {
        &self.get(contract_id).linearised_events
    }
}

fn compute_linearised_functions(
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
        }
    }
    events
}
