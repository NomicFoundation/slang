//! Pre-computing the collections of members visible in a contract's hierarchy:
//! its state variables, errors and events in base-to-derived source order, and
//! its functions flattened most-derived-first, resolving overrides.

use std::cmp::Ordering;
use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::ContractLinearisations;
use crate::passes::common::overrides;
use crate::types::TypeRegistry;

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
            Some(Definition::Contract(contract)) => (&contract.ir_node.members[..], false),
            Some(Definition::Interface(interface)) => (&interface.ir_node.members[..], true),
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
    let mut getters: Vec<&ir::StateVariableDefinition> = Vec::new();
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
                        .any(|kept| overrides(binder, types, *kept, function))
                        || getters
                            .iter()
                            .any(|getter| overrides(binder, types, *getter, function));
                    if !already_overridden {
                        functions.push(function);
                    }
                    // TODO(validation): if overriding multiple ancestors, the function needs to
                    // specify the bases in a specifier
                }
                ir::ContractMember::StateVariableDefinition(state_variable)
                    if matches!(
                        state_variable.attributes.visibility,
                        ir::StateVariableVisibility::Public
                    ) =>
                {
                    // Record its getter, if it has one, so it can shadow a
                    // matching function inherited from a base contract.
                    getters.push(state_variable);
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
