//! The abstractness check: reports a non-`abstract` contract that leaves a
//! function or modifier (its own or inherited) unimplemented.

use slang_solidity_v2_common::diagnostics::kinds::structure::ContractShouldBeAbstract;
use slang_solidity_v2_ir::ir;

use super::Lineariser;
use crate::binder::{Binder, Definition};
use crate::types::{TypeId, TypeRegistry};

impl<'a> Lineariser<'a> {
    /// Folds this base's functions and modifiers into the abstract-slot set.
    /// Bases are visited most-base-first, so a member here is more-derived than
    /// anything already recorded and overrides (updating the implementation
    /// status of) the matching slot — mirroring solc's base-to-derived overwrite
    /// of its unimplemented-declaration map.
    pub(super) fn record_abstract(&mut self, members: &[&'a Definition]) {
        let (binder, types) = (self.binder, self.types);
        'members: for definition in members {
            let Some(candidate) = AbstractSlot::of(binder, definition) else {
                continue;
            };
            for slot in &mut self.abstract_slots {
                // TODO: check for SDR[1122]
                if slot.overridden_by(types, &candidate) {
                    slot.type_id = candidate.type_id;
                    slot.implemented = candidate.implemented;
                    continue 'members;
                }
            }
            self.abstract_slots.push(candidate);
        }
    }

    /// Emits [`ContractShouldBeAbstract`] when a non-`abstract` contract still
    /// leaves a function or modifier unimplemented.
    pub(super) fn report_abstractness(&mut self) {
        let Some(contract) = self.contract else {
            return;
        };
        if contract.is_abstract || self.abstract_slots.iter().all(|slot| slot.implemented) {
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

/// A member of a contract's hierarchy that requires an implementation for the
/// contract to be concrete: a function or a modifier, together with whether it
/// is currently implemented.
pub(super) struct AbstractSlot<'a> {
    kind: AbstractSlotKind,
    /// The member's name, used to match declarations across bases. Borrowed from
    /// the owning definition, which lives in the binder for the whole walk.
    name: &'a str,
    /// The member's (function) type, used to distinguish overloads and detect
    /// overrides. `None` for modifiers, which cannot be overloaded and so match
    /// on name alone.
    type_id: Option<TypeId>,
    implemented: bool,
}

#[derive(PartialEq, Eq)]
enum AbstractSlotKind {
    Function,
    Modifier,
}

impl<'a> AbstractSlot<'a> {
    /// Builds the slot for a member that requires an implementation, or `None`
    /// for members that don't participate in the check.
    ///
    /// A `public` state variable contributes its (always-implemented) getter,
    /// which can satisfy a function declared in a base contract or interface.
    fn of(binder: &Binder, definition: &'a Definition) -> Option<Self> {
        let (kind, type_id, implemented) = match definition {
            Definition::Function(function) => (
                AbstractSlotKind::Function,
                binder.node_typing(function.ir_node.id()).as_type_id(),
                function.ir_node.body.is_some(),
            ),
            Definition::Modifier(modifier) => (
                AbstractSlotKind::Modifier,
                None,
                modifier.ir_node.body.is_some(),
            ),
            Definition::StateVariable(state_variable)
                if matches!(
                    state_variable.ir_node.attributes.visibility,
                    ir::StateVariableVisibility::Public
                ) =>
            {
                (
                    AbstractSlotKind::Function,
                    state_variable.getter_type_id,
                    true,
                )
            }
            _ => return None,
        };
        Some(AbstractSlot {
            kind,
            name: definition.identifier().unparse(),
            type_id,
            implemented,
        })
    }

    /// Whether the more-derived `candidate` overrides `self`: they are the same
    /// kind of member with the same name, and (for functions) their signatures
    /// are in an override relationship. Modifiers match on name alone since they
    /// cannot be overloaded.
    fn overridden_by(&self, types: &TypeRegistry, candidate: &AbstractSlot<'_>) -> bool {
        if self.kind != candidate.kind || self.name != candidate.name {
            return false;
        }
        match candidate.kind {
            AbstractSlotKind::Modifier => true,
            AbstractSlotKind::Function => match (candidate.type_id, self.type_id) {
                (Some(candidate_type_id), Some(slot_type_id)) => {
                    types.type_id_is_function_and_overrides(candidate_type_id, slot_type_id)
                }
                _ => false,
            },
        }
    }
}
