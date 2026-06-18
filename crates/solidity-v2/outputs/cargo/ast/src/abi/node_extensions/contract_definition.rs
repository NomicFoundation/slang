use ruint::aliases::U256;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::SemanticContext;

use crate::abi::{AbiEntry, ContractAbi, StorageItem};
use crate::ast::{ContractDefinitionStruct, StateVariableDefinition, StateVariableMutability};

impl ContractDefinitionStruct {
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        let name = self.ir_node.name.unparse().to_string();
        let file_id = self.get_file_id().clone();
        let entries = self.compute_abi_entries()?;
        let (storage_layout, transient_storage_layout) = self.compute_storage_layout()?;
        Some(ContractAbi {
            node_id: self.ir_node.id(),
            name,
            file_id,
            entries,
            storage_layout,
            transient_storage_layout,
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

    /// Retrieves the custom base slot for this contract, if specified. This is
    /// used for computing the base of the storage layout for non-transient
    /// state variables.
    fn base_slot(&self) -> Option<U256> {
        let binder::Definition::Contract(definition) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())?
        else {
            unreachable!("definition is not a contract");
        };
        definition.base_slot
    }

    /// Computes the layouts of both permanent and transient state variables
    fn compute_storage_layout(&self) -> Option<(Vec<StorageItem>, Vec<StorageItem>)> {
        let all_state_variables = self.linearised_state_variables();

        // TODO(validation) SDR[2]: it is an error if any contract in the hierarchy
        // other than the leaf has a custom offset layout
        let storage_layout = self.lay_out_state_variables(
            self.base_slot().unwrap_or(U256::ZERO),
            all_state_variables.iter().filter(|state_variable| {
                matches!(
                    state_variable.mutability(),
                    StateVariableMutability::Mutable
                )
            }),
        )?;
        let transient_storage_layout = self.lay_out_state_variables(
            U256::ZERO,
            all_state_variables.iter().filter(|state_variable| {
                matches!(
                    state_variable.mutability(),
                    StateVariableMutability::Transient
                )
            }),
        )?;
        Some((storage_layout, transient_storage_layout))
    }

    fn lay_out_state_variables<'a>(
        &self,
        base_slot: U256,
        variables: impl Iterator<Item = &'a StateVariableDefinition>,
    ) -> Option<Vec<StorageItem>> {
        let mut storage_layout = Vec::new();
        let mut current_slot: U256 = base_slot;
        let mut byte_offset_in_slot: usize = 0;
        for state_variable in variables {
            let node_id = state_variable.ir_node.id();
            let variable_type_id = self.semantic.binder().node_typing(node_id).as_type_id()?;
            let variable_size = self.semantic.storage_size_of_type_id(variable_type_id)?;

            // Check if we can pack the variable in the current slot, otherwise
            // we start at the beginning of the next slot.
            let remaining_bytes = SemanticContext::SLOT_SIZE - byte_offset_in_slot;
            if byte_offset_in_slot > 0 && variable_size > remaining_bytes {
                current_slot += U256::from(1u64);
                byte_offset_in_slot = 0;
            }

            let label = state_variable.ir_node.name.unparse().to_string();
            let type_name = self.semantic.type_internal_name(variable_type_id);
            storage_layout.push(StorageItem {
                node_id,
                label,
                slot: current_slot,
                offset: byte_offset_in_slot,
                type_name,
            });

            // Ready slot and offset for the next variable
            byte_offset_in_slot += variable_size;
            current_slot += U256::from(byte_offset_in_slot / SemanticContext::SLOT_SIZE);
            byte_offset_in_slot %= SemanticContext::SLOT_SIZE;
        }
        Some(storage_layout)
    }
}
