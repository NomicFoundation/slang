use ruint::aliases::U256;
use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::StorageLayoutBuilder;

use crate::abi::{ContractAbi, StorageItem, StorageKind, StorageLayout, StorageType};
use crate::ast::{ContractDefinitionStruct, StateVariableDefinition, StateVariableMutability};

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
        let (storage_layout, transient_storage_layout) = self.compute_storage_layouts()?;
        Some(ContractAbi::new(
            self.ir_node.id(),
            self.ir_node.name.unparse().to_string(),
            self.get_file_id().clone(),
            entries,
            storage_layout,
            transient_storage_layout,
        ))
    }

    /// The layout of the `kind` state variables over the contract's
    /// hierarchy, as in [`ContractAbi::storage_layout`] and
    /// [`ContractAbi::transient_storage_layout`], without computing the rest
    /// of the ABI.
    pub fn compute_storage_layout(&self, kind: StorageKind) -> Option<StorageLayout> {
        let items = self.lay_out_state_variables(kind, &self.linearised_state_variables())?;
        self.describe_storage_layout(items)
    }

    /// The items of [`Self::compute_storage_layout`], without describing their
    /// types, for callers that only need each variable's slot and offset.
    pub fn compute_storage_items(&self, kind: StorageKind) -> Option<Vec<StorageItem>> {
        self.lay_out_state_variables(kind, &self.linearised_state_variables())
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

    /// Computes the layouts of both the persistent and the transient state
    /// variables, linearising them once.
    fn compute_storage_layouts(&self) -> Option<(StorageLayout, StorageLayout)> {
        let state_variables = self.linearised_state_variables();
        let items = self.lay_out_state_variables(StorageKind::Persistent, &state_variables)?;
        let transient_items =
            self.lay_out_state_variables(StorageKind::Transient, &state_variables)?;
        Some((
            self.describe_storage_layout(items)?,
            self.describe_storage_layout(transient_items)?,
        ))
    }

    /// Lays out the `kind` state variables of `state_variables`.
    fn lay_out_state_variables(
        &self,
        kind: StorageKind,
        state_variables: &[StateVariableDefinition],
    ) -> Option<Vec<StorageItem>> {
        // TODO(validation) SDR[2]: it is an error if any contract in the hierarchy
        // other than the leaf has a custom offset layout
        let base_slot = match kind {
            StorageKind::Persistent => self.base_slot().unwrap_or(U256::ZERO),
            StorageKind::Transient => U256::ZERO,
        };
        let variables = state_variables.iter().filter(|state_variable| {
            match state_variable.attributes().mutability() {
                StateVariableMutability::Mutable => kind == StorageKind::Persistent,
                StateVariableMutability::Transient => kind == StorageKind::Transient,
                StateVariableMutability::Constant | StateVariableMutability::Immutable => false,
            }
        });
        let mut items = Vec::new();
        let mut builder = StorageLayoutBuilder::new(base_slot);
        for state_variable in variables {
            let node_id = state_variable.ir_node.id();
            let variable_type_id = self.semantic.binder().node_typing(node_id).as_type_id()?;
            let variable_size = self.semantic.storage_size_of_type_id(variable_type_id)?;
            let position = builder.allocate(variable_size)?;
            items.push(StorageItem {
                node_id,
                name: state_variable.ir_node.name.unparse().to_string(),
                slot: position.slot,
                offset: position.offset,
                type_id: variable_type_id,
            });
        }
        Some(items)
    }

    /// Completes `items` into a layout with the table of their types.
    fn describe_storage_layout(&self, items: Vec<StorageItem>) -> Option<StorageLayout> {
        let types = self.semantic.storage_type_table(
            items.iter().map(|item| item.type_id),
            |type_id, layout| StorageType {
                type_id,
                label: self.semantic.type_abi_internal_name(type_id),
                size: layout.size,
                kind: layout.kind,
            },
        )?;
        Some(StorageLayout::new(items, types))
    }
}
