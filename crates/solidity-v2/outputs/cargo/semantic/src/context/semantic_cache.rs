use std::cmp::Ordering;
use std::collections::HashMap;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::types::TypeRegistry;

/// Cache of semantic information about contracts and other Solidity elements,
/// derived from the binder and type registry after the semantic passes have
/// run. Built once and stored on the `SemanticContext` so downstream consumers
/// can look up commonly needed analyses without recomputing them.
pub(super) struct SemanticCache {
    /// For each contract definition, the functions visible following the C3
    /// linearisation: overridden functions are dropped and the list is sorted
    /// by name.
    linearised_functions: HashMap<NodeId, Vec<ir::FunctionDefinition>>,
}

impl SemanticCache {
    pub(super) fn build_from(binder: &Binder, types: &TypeRegistry) -> Self {
        let mut linearised_functions = HashMap::new();

        for (contract_id, definition) in binder.definitions() {
            if !matches!(definition, Definition::Contract(_)) {
                continue;
            }
            let functions = compute_linearised_functions(binder, types, *contract_id);
            linearised_functions.insert(*contract_id, functions);
        }

        Self {
            linearised_functions,
        }
    }

    pub(super) fn linearised_functions(
        &self,
        contract_id: NodeId,
    ) -> Option<&[ir::FunctionDefinition]> {
        self.linearised_functions
            .get(&contract_id)
            .map(Vec::as_slice)
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
                functions.push(function.clone());
            }
        }
    }

    functions.sort_by(|a, b| match (&a.name, &b.name) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a), Some(b)) => a.unparse().cmp(&b.unparse()),
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
