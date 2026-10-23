use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::StateVariablePosition;

use crate::abi::{
    ContractAbi, StorageLayout, StorageType, StorageVariable, storage_type_identifier,
};
use crate::ast::ContractDefinitionStruct;

impl ContractDefinitionStruct {
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        let mut entries = Vec::new();
        // An abstract contract cannot be deployed, so leave its constructor out.
        if let Some(constructor) = self.constructor()
            && !self.is_abstract()
        {
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
        // solc lists the errors and events the code reaches with the ones the
        // hierarchy declares, each definition once.
        let mut listed = Set::default();
        for error in self.linearised_errors().iter().chain(&self.used_errors()) {
            if listed.insert(error.node_id()) {
                entries.push(error.compute_abi_entry()?);
            }
        }
        for event in self.linearised_events().iter().chain(&self.used_events()) {
            if listed.insert(event.node_id()) {
                entries.push(event.compute_abi_entry()?);
            }
        }
        Some(ContractAbi::new(
            self.ir_node.id(),
            self.ir_node.name.unparse().to_string(),
            self.get_file_id().clone(),
            entries,
            &self.semantic,
        ))
    }

    /// ERC-165 identifier of an abstract contract: the XOR of the selectors of its own public
    /// functions and getters, excluding inherited ones. `None` for a concrete contract.
    pub fn compute_interface_id(&self) -> Option<u32> {
        if !self.is_abstract() {
            return None;
        }
        let mut interface_id = 0u32;
        for function in self.members().iter_function_definitions() {
            if function.ir_node.kind != ir::FunctionKind::Regular
                || !function.is_externally_visible()
            {
                continue;
            }
            interface_id ^= function.compute_selector()?;
        }
        for state_variable in self.members().iter_state_variable_definitions() {
            if !state_variable.is_externally_visible() {
                continue;
            }
            interface_id ^= state_variable.compute_selector()?;
        }
        Some(interface_id)
    }

    /// The items of [`Self::compute_storage_layout`], without describing their
    /// types, for callers that only need each variable's slot and offset.
    pub fn compute_storage_items(&self) -> Option<Vec<StorageVariable>> {
        self.semantic
            .storage_layout(self.ir_node.id())
            .map(Self::storage_items)
    }

    /// The layout and types of the persistent state variables over the
    /// contract's hierarchy.
    pub fn compute_storage_layout(&self) -> Option<StorageLayout> {
        let items = self.compute_storage_items()?;
        self.describe_storage_layout(items)
    }

    /// The items of [`Self::compute_transient_storage_layout`], without
    /// describing their types, for callers that only need each variable's slot
    /// and offset.
    pub fn compute_transient_storage_items(&self) -> Option<Vec<StorageVariable>> {
        self.semantic
            .transient_storage_layout(self.ir_node.id())
            .map(Self::storage_items)
    }

    /// The layout and types of the transient state variables over the
    /// contract's hierarchy.
    pub fn compute_transient_storage_layout(&self) -> Option<StorageLayout> {
        let items = self.compute_transient_storage_items()?;
        self.describe_storage_layout(items)
    }

    /// The items of a storage layout pre-computed by the semantic analysis.
    fn storage_items(positions: &[StateVariablePosition]) -> Vec<StorageVariable> {
        positions
            .iter()
            .map(|position| {
                StorageVariable::new(
                    position.state_variable().id(),
                    position.state_variable().name.unparse().to_string(),
                    position.type_id(),
                    position.position(),
                )
            })
            .collect()
    }

    /// Completes `items` into a layout with the table of their types.
    fn describe_storage_layout(&self, items: Vec<StorageVariable>) -> Option<StorageLayout> {
        let types = self.semantic.storage_type_table(
            items.iter().map(StorageVariable::type_id),
            |type_id, layout| StorageType {
                type_id,
                type_identifier: storage_type_identifier(&self.semantic, type_id),
                label: self.semantic.type_abi_internal_name(type_id),
                size: layout.size,
                kind: layout.kind,
            },
        )?;
        let contract = format!(
            "{file_id}:{name}",
            file_id = self.get_file_id(),
            name = self.ir_node.name.unparse(),
        );
        Some(StorageLayout::new(contract, items, types))
    }
}
