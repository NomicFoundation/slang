use std::sync::Arc;

use crate::abi::ContractAbi;
use crate::abi::types::AbiTypeCache;
use crate::ast::{ContractBase, InterfaceDefinitionStruct};

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

    /// The ABI over the interface's linearised hierarchy: an overriding function stands in for
    /// the one it overrides, inherited errors and events are listed with its own. An interface
    /// has no storage, so both layouts are empty. `None` when a base is a contract, which solc
    /// rejects.
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        if self
            .linearised_bases()
            .iter()
            .any(|base| matches!(base, ContractBase::Contract(_)))
        {
            return None;
        }
        let mut cache = AbiTypeCache::default();
        let mut entries = Vec::new();
        for function in &self.linearised_functions() {
            entries.push(function.compute_abi_entry_cached(&mut cache)?);
        }
        for error in &self.linearised_errors() {
            entries.push(error.compute_abi_entry_cached(&mut cache)?);
        }
        for event in &self.linearised_events() {
            entries.push(event.compute_abi_entry_cached(&mut cache)?);
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
