//! The abstractness check: reports a non-`abstract` contract that leaves a
//! function or modifier (its own or inherited) unimplemented.

use slang_solidity_v2_common::diagnostics::kinds::structure::ContractShouldBeAbstract;
use slang_solidity_v2_ir::ir;
use smallvec::SmallVec;

use super::HierarchyChecker;
use crate::passes::common::Overridable;

impl<'a> HierarchyChecker<'a> {
    /// Folds this base's overridable members into the abstract-slot set. A
    /// public state variable takes a slot through its getter, which is always
    /// implemented. Bases are visited most-base-first, so a member here is
    /// more-derived than anything already recorded and takes over the matching
    /// slot when it has a body. Slots are grouped by name, so a candidate is
    /// only ever compared against same-named slots. `in_interface` says
    /// whether this base is an interface.
    pub(super) fn record_abstract(
        &mut self,
        members: &'a [ir::ContractMember],
        in_interface: bool,
    ) {
        let binder = self.binder;
        let types = self.types;
        let abstract_slots = &mut self.abstract_slots;
        'members: for member in members {
            let Some(candidate) = Overridable::of(member, in_interface) else {
                continue;
            };
            let slots = abstract_slots.entry(candidate.name()).or_default();
            for slot in slots.iter_mut() {
                if candidate.overrides(binder, types, slot) {
                    // A bodiless override is reported by the override check
                    // and leaves the slot as it is, so it cannot make an
                    // implemented member unimplemented again.
                    if candidate.is_implemented() {
                        *slot = candidate;
                    }
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
                .all(Overridable::is_implemented)
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
pub(super) type AbstractSlots<'a> = SmallVec<[Overridable<'a>; 1]>;
