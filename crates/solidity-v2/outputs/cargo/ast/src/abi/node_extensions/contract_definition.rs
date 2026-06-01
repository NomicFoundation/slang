use crate::abi::{AbiEntry, ContractAbi};
use crate::ast::ContractDefinitionStruct;

impl ContractDefinitionStruct {
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        let name = self.ir_node.name.unparse().to_string();
        let file_id = self.get_file_id().to_string();
        let entries = self.compute_abi_entries()?;
        let layouts = self.semantic.storage_layouts(self.ir_node.id())?.clone();
        Some(ContractAbi {
            node_id: self.ir_node.id(),
            name,
            file_id,
            entries,
            storage_layout: layouts.permanent,
            transient_storage_layout: layouts.transient,
        })
    }

    fn compute_abi_entries(&self) -> Option<Vec<AbiEntry>> {
        let mut entries = Vec::new();
        if let Some(constructor) = self.constructor() {
            entries.push(constructor.compute_abi_entry()?);
        }
        for function in &self.linearised_functions() {
            if function.is_externally_visible() {
                entries.push(function.compute_abi_entry()?);
            }
        }
        for state_variable in &self.linearised_state_variables() {
            if state_variable.is_externally_visible() {
                entries.push(state_variable.compute_abi_entry()?);
            }
        }
        for error in &self.linearised_errors() {
            entries.push(error.compute_abi_entry()?);
        }
        for event in &self.linearised_events() {
            entries.push(event.compute_abi_entry()?);
        }

        entries.sort();
        Some(entries)
    }
}
