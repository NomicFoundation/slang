use std::sync::Arc;

use ruint::aliases::U256;
use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::StorageLayoutBuilder;

use crate::abi::types::AbiTypeCache;
use crate::abi::{AbiEntry, ContractAbi, StorageItem};
use crate::ast::{
    ContractBase, ContractDefinitionStruct, StateVariableDefinition, StateVariableMutability,
};

impl ContractDefinitionStruct {
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        self.compute_abi_cached(&mut AbiTypeCache::default())
    }

    pub(crate) fn compute_abi_cached(&self, cache: &mut AbiTypeCache) -> Option<ContractAbi> {
        let name = self.ir_node.name.unparse().to_string();
        let file_id = self.get_file_id().clone();
        let entries = self.compute_abi_entries(cache)?;
        let (storage_layout, transient_storage_layout) = self.compute_storage_layout()?;
        Some(ContractAbi {
            node_id: self.ir_node.id(),
            name,
            file_id,
            entries,
            storage_layout,
            transient_storage_layout,
            semantic: Arc::clone(&self.semantic),
        })
    }

    fn compute_abi_entries(&self, cache: &mut AbiTypeCache) -> Option<Vec<AbiEntry>> {
        let mut entries = Vec::new();
        let is_abstract = self.is_abstract();
        // An abstract contract cannot be deployed, so solc leaves its constructor out.
        if let Some(constructor) = self.constructor()
            && !is_abstract
        {
            entries.push(constructor.compute_abi_entry_cached(cache)?);
        }
        // The linearised functions exclude interface bases, but an interface function nothing
        // implements is still in the ABI, and only an abstract contract can have one. The
        // signature set is keyed by canonical signature, the same identity the linearisation's
        // override resolution uses through its typed predicates.
        let mut implemented = Set::default();
        for function in &self.linearised_functions() {
            if function.is_externally_visible() {
                if is_abstract {
                    implemented.insert(function.compute_abi_key()?);
                }
                entries.push(function.compute_abi_entry_cached(cache)?);
            }
        }
        for state_variable in &self.linearised_state_variables() {
            if state_variable.is_externally_visible() {
                if is_abstract {
                    implemented.insert(state_variable.compute_canonical_signature()?);
                }
                entries.push(state_variable.compute_abi_entry_cached(cache)?);
            }
        }
        if is_abstract {
            for base in &self.linearised_bases() {
                let ContractBase::Interface(interface) = base else {
                    continue;
                };
                for function in interface.members().iter_function_definitions() {
                    if implemented.insert(function.compute_abi_key()?) {
                        entries.push(function.compute_abi_entry_cached(cache)?);
                    }
                }
            }
        }
        for error in &self.linearised_errors() {
            entries.push(error.compute_abi_entry_cached(cache)?);
        }
        for event in &self.linearised_events() {
            entries.push(event.compute_abi_entry_cached(cache)?);
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
                    state_variable.attributes().mutability(),
                    StateVariableMutability::Mutable
                )
            }),
        )?;
        let transient_storage_layout = self.lay_out_state_variables(
            U256::ZERO,
            all_state_variables.iter().filter(|state_variable| {
                matches!(
                    state_variable.attributes().mutability(),
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
        let mut builder = StorageLayoutBuilder::new(base_slot);
        for state_variable in variables {
            let node_id = state_variable.ir_node.id();
            let variable_type_id = self.semantic.binder().node_typing(node_id).as_type_id()?;
            let variable_size = self.semantic.storage_size_of_type_id(variable_type_id)?;
            let position = builder.allocate(variable_size)?;

            let label = state_variable.ir_node.name.unparse().to_string();
            let type_name = self.semantic.type_internal_name(variable_type_id);
            storage_layout.push(StorageItem {
                node_id,
                label,
                slot: position.slot,
                offset: position.offset,
                type_name,
            });
        }
        Some(storage_layout)
    }
}
