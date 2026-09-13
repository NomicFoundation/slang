//! Pre-computing the collections of members visible in a contract's or an
//! interface's hierarchy: its state variables, errors and events in
//! base-to-derived source order, and its functions flattened
//! most-derived-first, resolving overrides.

use std::cmp::Ordering;
use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::ContractLinearisations;
use crate::passes::common::Overridable;
use crate::types::TypeRegistry;

/// Walks the contract's or interface's linearised bases in reverse
/// (most-base-first) and gathers the members visible in its hierarchy.
pub(super) fn compute_linearisations(
    binder: &Binder,
    types: &TypeRegistry,
    contract_id: NodeId,
) -> ContractLinearisations {
    let Some(linearised_bases) = binder.get_linearised_bases(contract_id) else {
        return ContractLinearisations::default();
    };

    // The members of each base, gathered most-base-first, so the hierarchy's
    // functions can be flattened most-derived-first at the end. Interface
    // bases contribute too: a concrete contract implements every interface
    // function, so each declaration is overridden and dropped, while an
    // abstract contract keeps the ones it leaves unimplemented, like any other
    // body-less function.
    let mut base_members = Vec::with_capacity(linearised_bases.len());
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

        base_members.push((members, base_is_interface));
    }

    ContractLinearisations {
        functions: linearise_functions(binder, types, &base_members),
        state_variables,
        errors,
        events,
    }
}

/// Flattens the bases' members (gathered most-base-first) into the
/// hierarchy's function list: most-derived-first, dropping a function once a
/// more-derived function or a public state variable's getter overrides it, then
/// sorted by name. Functions are cloned out only once they're known to survive
/// override resolution.
fn linearise_functions(
    binder: &Binder,
    types: &TypeRegistry,
    base_members: &[(&[ir::ContractMember], bool)],
) -> Vec<ir::FunctionDefinition> {
    // A public state variable is kept as well, so its getter can shadow a
    // matching function inherited from a base contract.
    let mut kept: Vec<Overridable<'_>> = Vec::new();
    for (members, base_is_interface) in base_members.iter().rev() {
        for member in *members {
            let Some(candidate) = Overridable::of(member, *base_is_interface) else {
                continue;
            };
            if candidate.is_modifier()
                || kept
                    .iter()
                    .any(|slot| slot.overrides(binder, types, &candidate))
            {
                continue;
            }
            kept.push(candidate);
            // TODO(validation): if overriding multiple ancestors, the function needs to
            // specify the bases in a specifier
        }
    }
    let mut functions: Vec<ir::FunctionDefinition> = kept
        .iter()
        .filter_map(|slot| match slot {
            Overridable::Function { definition, .. } => Some(Arc::clone(*definition)),
            _ => None,
        })
        .collect();
    functions.sort_by(|a, b| match (&a.name, &b.name) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a), Some(b)) => a.unparse().cmp(b.unparse()),
    });
    functions
}
