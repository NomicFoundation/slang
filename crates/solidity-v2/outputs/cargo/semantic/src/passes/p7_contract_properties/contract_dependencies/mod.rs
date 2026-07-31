//! Computes every contract's bytecode dependencies.
//!
//! `new C`, `type(C).creationCode` and `type(C).runtimeCode` embed `C`'s
//! bytecode into the referencing contract. This pass records, per contract
//! and library, the contracts its creation code and its deployed code
//! depend on this way, together with the first expression referencing each
//! dependency.
//!
//! The pass runs only when the binder saw a contract reference. Each code
//! unit is then walked once, not once per contract.
//! 1. Collect each code unit's contract and callable references
//!    ([`units`], [`references`]).
//! 2. For each contract, traverse the unit references reachable from its
//!    entry points, resolving virtual callables against it ([`dependencies`]).

mod dependencies;
mod references;
mod units;

use crate::binder::Binder;
use crate::context::ContractData;
use crate::types::TypeRegistry;

pub(super) fn run(binder: &Binder, contract_data: &mut ContractData, types: &TypeRegistry) {
    if !binder.has_contract_references() {
        return;
    }

    let units = units::collect(binder);
    let unit_references = references::collect(binder, types, &units);
    let dependencies = dependencies::build(binder, contract_data, types, &unit_references);
    contract_data.set_contract_dependencies(dependencies.creation, dependencies.deployed);
}
