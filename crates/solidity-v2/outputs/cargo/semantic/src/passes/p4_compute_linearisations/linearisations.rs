//! Pre-computing the collections of members visible in a contract's or an
//! interface's hierarchy: its state variables, errors and events in
//! base-to-derived source order, and its functions flattened
//! most-derived-first, resolving overrides.

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
    let mut candidates: Vec<Overridable<'_>> =
        Vec::with_capacity(base_members.iter().map(|(members, _)| members.len()).sum());
    for (members, base_is_interface) in base_members.iter().rev() {
        candidates.extend(candidates_of(members, *base_is_interface));
    }
    // Only same-named members can override each other, so grouping the
    // candidates by name (stably, keeping them most-derived-first within a
    // name) confines each comparison to the predecessors in its own group. It
    // also leaves the survivors in the order the list wants: sorted by name,
    // with the nameless fallback and receive first.
    candidates.sort_by_key(Overridable::name);

    let mut kept: Vec<Overridable<'_>> = Vec::with_capacity(candidates.len());
    let mut group_start = 0;
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 && candidate.name() != candidates[index - 1].name() {
            group_start = kept.len();
        }
        if kept[group_start..]
            .iter()
            .any(|slot| slot.overrides(binder, types, candidate))
        {
            continue;
        }
        kept.push(*candidate);
        // TODO(validation): if overriding multiple ancestors, the function needs to
        // specify the bases in a specifier
    }
    kept.into_iter()
        .filter_map(|slot| match slot {
            Overridable::Function { definition, .. } => Some(Arc::clone(definition)),
            _ => None,
        })
        .collect()
}

/// The members competing for a slot in the hierarchy's function list: the
/// functions, and the public state variables, whose getter takes the slot of a
/// same-signature function inherited from a base contract without joining the
/// list itself. Modifiers are overridable too, but never part of the list.
fn candidates_of(
    members: &[ir::ContractMember],
    base_is_interface: bool,
) -> impl Iterator<Item = Overridable<'_>> {
    members
        .iter()
        .filter_map(move |member| Overridable::of(member, base_is_interface))
        .filter(|candidate| !candidate.is_modifier())
}
