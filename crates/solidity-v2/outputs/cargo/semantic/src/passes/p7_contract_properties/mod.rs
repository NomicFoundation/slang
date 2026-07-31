//! Computes properties of each contract, once the whole program is resolved.

mod contract_dependencies;

use crate::binder::Binder;
use crate::context::ContractData;
use crate::types::TypeRegistry;

pub(crate) fn run(binder: &Binder, contract_data: &mut ContractData, types: &TypeRegistry) {
    contract_dependencies::run(binder, contract_data, types);
}
