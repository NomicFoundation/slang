use crate::abi::ContractAbi;
use crate::ast::{
    ContractMember, FunctionDefinition, FunctionMutability, LibraryDefinitionStruct,
    StorageLocation,
};

impl LibraryDefinitionStruct {
    /// The ABI of the library's `view` and `pure` external functions, public constants' getters,
    /// errors and events. A function that writes state or takes or returns a storage reference is
    /// reachable only by `DELEGATECALL` from a contract and is left out, as solc does; a library
    /// has no storage of its own, so both layouts are empty.
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        let entries = self
            .members()
            .iter()
            .filter_map(|member| match member {
                ContractMember::FunctionDefinition(function)
                    if function.is_externally_visible() && is_callable_on_library(&function) =>
                {
                    Some(function.compute_abi_entry())
                }
                ContractMember::StateVariableDefinition(state_variable)
                    if state_variable.is_externally_visible() =>
                {
                    Some(state_variable.compute_abi_entry())
                }
                ContractMember::ErrorDefinition(error) => Some(error.compute_abi_entry()),
                ContractMember::EventDefinition(event) => Some(event.compute_abi_entry()),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?;
        Some(ContractAbi::new(
            self.ir_node.id(),
            self.ir_node.name.unparse().to_string(),
            self.get_file_id().clone(),
            entries,
            Vec::new(),
            Vec::new(),
        ))
    }
}

/// A library function a contract can `CALL` rather than `DELEGATECALL`: it neither writes state
/// nor takes or returns a storage reference.
fn is_callable_on_library(function: &FunctionDefinition) -> bool {
    let reads_only = matches!(
        function.attributes().mutability(),
        FunctionMutability::View | FunctionMutability::Pure
    );
    let references_storage = function
        .parameters()
        .iter()
        .chain(function.returns().iter().flat_map(|returns| returns.iter()))
        .any(|parameter| {
            matches!(
                parameter.storage_location(),
                Some(StorageLocation::StorageKeyword(_))
            )
        });
    reads_only && !references_storage
}
