use crate::abi::ContractAbi;
use crate::ast::{ContractMember, LibraryDefinitionStruct, StorageLocation};

impl LibraryDefinitionStruct {
    /// The ABI of the library's externally visible functions, errors and events. A function
    /// taking a storage reference is reachable only by `DELEGATECALL` from a contract and is
    /// left out, as solc does; a library has no storage of its own, so both layouts are empty.
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        let mut entries = Vec::new();
        for member in self.members().iter() {
            match member {
                ContractMember::FunctionDefinition(function) => {
                    let takes_storage_reference = function.parameters().iter().any(|parameter| {
                        matches!(
                            parameter.storage_location(),
                            Some(StorageLocation::StorageKeyword(_))
                        )
                    });
                    if !takes_storage_reference && let Some(entry) = function.compute_abi_entry() {
                        entries.push(entry);
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
        })
    }
}
