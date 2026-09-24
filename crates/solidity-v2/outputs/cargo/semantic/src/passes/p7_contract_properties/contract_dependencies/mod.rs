//! Computes what every contract's reachable code references.
//!
//! `new C`, `type(C).creationCode` and `type(C).runtimeCode` embed `C`'s
//! bytecode into the referencing contract. This pass records, per contract
//! and library, the contracts its creation code and its deployed code
//! depend on this way, together with the first expression referencing each
//! dependency. It also records the errors that code can revert with and the
//! events it can emit, which solc lists in the contract's ABI next to the
//! ones its hierarchy declares.
//!
//! The dependencies are computed only when the binder saw a contract
//! reference. The errors and events are computed with them then, and
//! otherwise on first use ([`used_errors_and_events`]). Each code unit is
//! walked once, not once per contract.
//! 1. Collect each code unit's contract, callable, error and event references
//!    ([`units`], [`references`]).
//! 2. For each contract, traverse the unit references reachable from its
//!    entry points, resolving virtual callables against it ([`dependencies`]).

mod dependencies;
mod references;
mod units;

use crate::binder::Binder;
use crate::context::{ContractData, UsedErrorsAndEvents};
use crate::types::TypeRegistry;

pub(super) fn run(binder: &Binder, contract_data: &mut ContractData, types: &TypeRegistry) {
    if !binder.has_contract_references() {
        return;
    }

    let units = units::collect(binder);
    let unit_references = references::collect(binder, types, &units);
    let dependencies = dependencies::build(binder, contract_data, types, &unit_references);
    contract_data.set_contract_dependencies(dependencies.creation, dependencies.deployed);
    contract_data.set_used_errors_and_events(dependencies.used_errors_and_events);
}

/// The errors and events reached by every contract's and library's code, for
/// a program without contract references, whose dependencies are all empty.
pub(crate) fn used_errors_and_events(
    binder: &Binder,
    contract_data: &ContractData,
    types: &TypeRegistry,
) -> UsedErrorsAndEvents {
    let units = units::collect(binder);
    let unit_references = references::collect(binder, types, &units);
    dependencies::build(binder, contract_data, types, &unit_references).used_errors_and_events
}
