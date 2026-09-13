use std::sync::Arc;

use slang_solidity_v2_common::collections::Set;

use crate::abi::ContractAbi;
use crate::ast::{ContractBase, ContractMember, InterfaceDefinitionStruct};

impl InterfaceDefinitionStruct {
    /// Computes the ERC-165 interface identifier: the XOR of the 4-byte selectors of the functions
    /// the interface itself declares, excluding inherited ones.
    pub fn compute_interface_id(&self) -> Option<u32> {
        let mut interface_id = 0u32;
        for function in self.members().iter_function_definitions() {
            interface_id ^= function.compute_selector()?;
        }
        Some(interface_id)
    }

    /// The ABI over the interface's linearised hierarchy, itself first: an overriding function
    /// stands in for the one it overrides, inherited errors and events are listed with its own.
    /// An interface has no storage, so both layouts are empty. `None` when a base is a contract,
    /// which solc rejects.
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        let mut entries = Vec::new();
        let mut signatures = Set::default();
        for base in &self.linearised_bases() {
            let ContractBase::Interface(base) = base else {
                return None;
            };
            for member in base.members().iter() {
                match member {
                    ContractMember::FunctionDefinition(function) => {
                        if signatures.insert(function.compute_abi_key()?) {
                            entries.push(function.compute_abi_entry()?);
                        }
                    }
                    ContractMember::ErrorDefinition(error) => {
                        entries.push(error.compute_abi_entry()?);
                    }
                    ContractMember::EventDefinition(event) => {
                        entries.push(event.compute_abi_entry()?);
                    }
                    _ => {}
                }
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
