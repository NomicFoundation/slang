//! The abstractness check: reports a non-`abstract` contract that leaves a
//! function or modifier (its own or inherited) unimplemented.

use slang_solidity_v2_common::diagnostics::kinds::structure::ContractShouldBeAbstract;
use slang_solidity_v2_ir::ir;
use smallvec::SmallVec;

use super::HierarchyChecker;
use crate::passes::common::{Callable, overrides};

impl<'a> HierarchyChecker<'a> {
    /// Folds this base's functions and modifiers into the abstract-slot set.
    /// Bases are visited most-base-first, so a member here is more-derived than
    /// anything already recorded and overrides (updating the implementation
    /// status of) the matching slot — mirroring solc's base-to-derived overwrite
    /// of its unimplemented-declaration map. Slots are grouped by name, so a
    /// candidate is only ever compared against same-named slots.
    pub(super) fn record_abstract(&mut self, members: &'a [ir::ContractMember]) {
        let binder = self.binder;
        let types = self.types;
        let abstract_slots = &mut self.abstract_slots;
        'members: for member in members {
            let Some(candidate) = AbstractSlot::of(member) else {
                continue;
            };
            let slots = abstract_slots.entry(candidate.callable.name()).or_default();
            for slot in slots.iter_mut() {
                if overrides(binder, types, candidate.callable, slot.callable) {
                    *slot = candidate;
                    continue 'members;
                }
            }
            slots.push(candidate);
        }
    }

    /// Emits [`ContractShouldBeAbstract`] when a non-`abstract` contract still
    /// leaves a function or modifier unimplemented.
    pub(super) fn report_abstractness(&mut self) {
        let Some(contract) = self.contract else {
            return;
        };
        if contract.is_abstract
            || self
                .abstract_slots
                .values()
                .flatten()
                .all(|slot| slot.implemented)
        {
            return;
        }
        let file_id = self.file_node_mapper.file_id_from_node_id(contract.id());
        self.diagnostics.push(
            file_id.to_owned(),
            contract.range.clone(),
            ContractShouldBeAbstract {
                name: contract.name.unparse().to_owned(),
            },
        );
    }
}

/// The same-named slots occupying one name. Stored inline in the map entry in
/// the common case of a single slot per name; only overloaded names spill to
/// the heap.
pub(super) type AbstractSlots<'a> = SmallVec<[AbstractSlot<'a>; 1]>;

/// A member of a contract's hierarchy that requires an implementation for the
/// contract to be concrete: a function or a modifier, together with whether it
/// is currently implemented.
#[derive(Clone, Copy)]
pub(super) struct AbstractSlot<'a> {
    callable: &'a dyn Callable,
    implemented: bool,
}

impl<'a> AbstractSlot<'a> {
    /// Builds the slot for a member that requires an implementation, or `None`
    /// for members that don't participate in the check.
    ///
    /// A `public` state variable contributes its (always-implemented) getter,
    /// which can satisfy a function declared in a base contract or interface.
    fn of(member: &'a ir::ContractMember) -> Option<Self> {
        let (callable, implemented): (&'a dyn Callable, bool) = match member {
            ir::ContractMember::FunctionDefinition(function)
                if function.kind != ir::FunctionKind::Constructor =>
            {
                (function, function.body.is_some())
            }
            ir::ContractMember::StateVariableDefinition(state_variable)
                if matches!(
                    state_variable.attributes.visibility,
                    ir::StateVariableVisibility::Public
                ) =>
            {
                (state_variable, true)
            }
            _ => return None,
        };
        Some(AbstractSlot {
            callable,
            implemented,
        })
    }
}
