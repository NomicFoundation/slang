//! The abstractness check: reports a non-`abstract` contract that leaves a
//! function or modifier (its own or inherited) unimplemented.

use slang_solidity_v2_common::diagnostics::kinds::structure::ContractShouldBeAbstract;
use slang_solidity_v2_ir::ir;
use smallvec::SmallVec;

use super::HierarchyChecker;
use crate::binder::Definition;
use crate::passes::common::{Callable, overrides};

impl<'a> HierarchyChecker<'a> {
    /// Folds this base's functions and modifiers into the abstract-slot set.
    /// Bases are visited most-base-first, so a member here is more-derived than
    /// anything already recorded and overrides (updating the implementation
    /// status of) the matching slot — mirroring solc's base-to-derived overwrite
    /// of its unimplemented-declaration map. Slots are grouped by name, so a
    /// candidate is only ever compared against same-named slots.
    pub(super) fn record_abstract(&mut self, members: &[&'a Definition]) {
        let binder = self.binder;
        let types = self.types;
        let abstract_slots = &mut self.abstract_slots;
        'members: for definition in members {
            let Some(candidate) = AbstractSlot::of(definition) else {
                continue;
            };
            let slots = abstract_slots.entry(candidate.name).or_default();
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
    /// The member's name, used to match declarations across bases. Borrowed from
    /// the owning definition, which lives in the binder for the whole walk.
    name: &'a str,
    implemented: bool,
}

impl<'a> AbstractSlot<'a> {
    /// Builds the slot for a member that requires an implementation, or `None`
    /// for members that don't participate in the check.
    ///
    /// A `public` state variable contributes its (always-implemented) getter,
    /// which can satisfy a function declared in a base contract or interface.
    fn of(definition: &'a Definition) -> Option<Self> {
        let (callable, implemented): (&'a dyn Callable, bool) = match definition {
            Definition::Function(function) => (&function.ir_node, function.ir_node.body.is_some()),
            Definition::Modifier(modifier) => (&modifier.ir_node, modifier.ir_node.body.is_some()),
            Definition::StateVariable(state_variable)
                if matches!(
                    state_variable.ir_node.attributes.visibility,
                    ir::StateVariableVisibility::Public
                ) =>
            {
                (&state_variable.ir_node, true)
            }
            _ => return None,
        };
        Some(AbstractSlot {
            callable,
            name: definition.identifier().unparse(),
            implemented,
        })
    }
}
