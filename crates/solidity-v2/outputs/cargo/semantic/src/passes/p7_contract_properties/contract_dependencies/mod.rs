//! Computes what every contract's reachable code references.
//!
//! `new C`, `type(C).creationCode` and `type(C).runtimeCode` embed `C`'s
//! bytecode into the referencing contract. This pass records, per contract
//! and library, the contracts its creation code and its deployed code
//! depend on this way, together with the first expression referencing each
//! dependency. It also records the errors that code can revert with and the
//! events it can emit, to list in the contract's ABI next to the
//! ones its hierarchy declares.
//!
//! Each code unit is walked once, not once per contract.
//! 1. Collect each code unit's contract, callable, error and event references
//!    ([`units`], [`references`]).
//! 2. For each contract, traverse the unit references reachable from its
//!    entry points, resolving virtual callables against it ([`dependencies`]).
//! 3. For each segment, traverse its calls alone to list the functions it
//!    runs. A function taken as a value joins as a pointer target once the
//!    traversal reaches a pointer call of its signature ([`dependencies`]).

mod dependencies;
mod references;
mod units;

use crate::binder::Binder;
use crate::context::ContractData;
use crate::types::TypeRegistry;

pub(super) fn run(binder: &Binder, contract_data: &mut ContractData, types: &TypeRegistry) {
    let units = units::collect(binder);
    let unit_references = references::collect(binder, types, &units);
    let dependencies = dependencies::build(binder, contract_data, types, &units, &unit_references);
    contract_data.set_contract_dependencies(dependencies.creation, dependencies.deployed);
    contract_data.set_segment_functions(
        dependencies.creation_functions,
        dependencies.deployed_functions,
    );
    contract_data.set_pointer_targets(
        dependencies.creation_pointer_targets,
        dependencies.deployed_pointer_targets,
    );
    contract_data.set_used_errors_and_events(dependencies.used_errors, dependencies.used_events);
}
