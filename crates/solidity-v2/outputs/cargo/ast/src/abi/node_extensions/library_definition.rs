use std::sync::Arc;

use crate::abi::ContractAbi;
use crate::ast::{ContractMember, FunctionMutability, LibraryDefinitionStruct, StorageLocation};

impl LibraryDefinitionStruct {
    /// The ABI of the library's `view` and `pure` external functions, public constants' getters,
    /// errors and events. A function that writes state or takes or returns a storage reference is
    /// reachable only by `DELEGATECALL` from a contract and is left out, as solc does; a library
    /// has no storage of its own, so both layouts are empty.
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        let mut entries = Vec::new();
        for member in self.members().iter() {
            match member {
                ContractMember::FunctionDefinition(function) => {
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
                    let reads_only = matches!(
                        function.attributes().mutability(),
                        FunctionMutability::View | FunctionMutability::Pure
                    );
                    if function.is_externally_visible() && reads_only && !references_storage {
                        entries.push(function.compute_abi_entry()?);
                    }
                }
                ContractMember::StateVariableDefinition(state_variable) => {
                    if state_variable.is_externally_visible() {
                        entries.push(state_variable.compute_abi_entry()?);
                    }
                }
                ContractMember::ErrorDefinition(error) => entries.push(error.compute_abi_entry()?),
                ContractMember::EventDefinition(event) => entries.push(event.compute_abi_entry()?),
                _ => {}
            }
        }
        entries.sort();
        Some(ContractAbi {
            node_id: self.ir_node.id(),
            name: self.ir_node.name.unparse().to_string(),
            file_id: self.get_file_id().clone(),
            entries,
            storage_layout: Vec::new(),
            transient_storage_layout: Vec::new(),
            semantic: Arc::clone(&self.semantic),
        })
    }
}
