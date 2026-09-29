use std::sync::Arc;

use ruint::aliases::U256;
use slang_solidity_v2_common::collections::{DefaultWithCapacity, Map, Set};
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::{
    self as semantic, SemanticContext, StorageLayoutBuilder,
};
use slang_solidity_v2_semantic::types::TypeId;

use crate::abi::{ContractAbi, StorageItem, StorageLayout, StorageType, StorageTypeKind};
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

    /// The layout of the persistent state variables over the contract's
    /// hierarchy, as in [`ContractAbi::storage_layout`], without computing the
    /// rest of the ABI.
    pub fn compute_storage_layout(&self) -> Option<StorageLayout> {
        let mut type_names = StorageTypeNames::new(&self.semantic);
        let items = self.lay_out_state_variables(
            false,
            &self.linearised_state_variables(),
            &mut type_names,
        )?;
        self.describe_storage_layout(items, &mut type_names)
    }

    /// The layout of the transient state variables over the contract's
    /// hierarchy, as in [`ContractAbi::transient_storage_layout`], without
    /// computing the rest of the ABI.
    pub fn compute_transient_storage_layout(&self) -> Option<StorageLayout> {
        let mut type_names = StorageTypeNames::new(&self.semantic);
        let items = self.lay_out_state_variables(
            true,
            &self.linearised_state_variables(),
            &mut type_names,
        )?;
        self.describe_storage_layout(items, &mut type_names)
    }

    /// The items of [`Self::compute_storage_layout`], without describing their
    /// types, for callers that only need each variable's slot and offset.
    pub fn compute_storage_items(&self) -> Option<Vec<StorageItem>> {
        self.lay_out_state_variables(
            false,
            &self.linearised_state_variables(),
            &mut StorageTypeNames::new(&self.semantic),
        )
    }

    /// The items of [`Self::compute_transient_storage_layout`], without
    /// describing their types, for callers that only need each variable's slot
    /// and offset.
    pub fn compute_transient_storage_items(&self) -> Option<Vec<StorageItem>> {
        self.lay_out_state_variables(
            true,
            &self.linearised_state_variables(),
            &mut StorageTypeNames::new(&self.semantic),
        )
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

    /// Computes the layouts of both permanent and transient state variables,
    /// sharing the type names between them.
    fn compute_storage_layouts(&self) -> Option<(StorageLayout, StorageLayout)> {
        let state_variables = self.linearised_state_variables();
        let mut type_names = StorageTypeNames::new(&self.semantic);
        let items = self.lay_out_state_variables(false, &state_variables, &mut type_names)?;
        let storage_layout = self.describe_storage_layout(items, &mut type_names)?;
        let transient_items =
            self.lay_out_state_variables(true, &state_variables, &mut type_names)?;
        let transient_storage_layout =
            self.describe_storage_layout(transient_items, &mut type_names)?;
        Some((storage_layout, transient_storage_layout))
    }

    /// Lays out the `transient` state variables, or else the persistent ones,
    /// of `state_variables`.
    fn lay_out_state_variables(
        &self,
        transient: bool,
        state_variables: &[StateVariableDefinition],
        type_names: &mut StorageTypeNames<'_>,
    ) -> Option<Vec<StorageItem>> {
        // TODO(validation) SDR[2]: it is an error if any contract in the hierarchy
        // other than the leaf has a custom offset layout
        let base_slot = if transient {
            U256::ZERO
        } else {
            self.base_slot().unwrap_or(U256::ZERO)
        };
        let variables = state_variables.iter().filter(|state_variable| {
            match state_variable.attributes().mutability() {
                StateVariableMutability::Mutable => !transient,
                StateVariableMutability::Transient => transient,
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

            let label = state_variable.ir_node.name.unparse().to_string();
            let type_name = type_names.get(variable_type_id);
            items.push(StorageItem {
                node_id,
                label,
                slot: position.slot,
                offset: position.offset,
                type_id: variable_type_id,
                type_name,
            });
        }
        Some(items)
    }

    /// Completes `items` into a layout with the table of their types.
    fn describe_storage_layout(
        &self,
        items: Vec<StorageItem>,
        type_names: &mut StorageTypeNames<'_>,
    ) -> Option<StorageLayout> {
        let (types, type_indices) = self.compute_storage_types(&items, type_names)?;
        Some(StorageLayout::new(items, types, type_indices))
    }

    /// Describes the types of `items`, and every type those refer to, in the
    /// order they are first reached, along with each one's index in that order.
    fn compute_storage_types(
        &self,
        items: &[StorageItem],
        type_names: &mut StorageTypeNames<'_>,
    ) -> Option<(Vec<StorageType>, Map<TypeId, usize>)> {
        let mut type_ids = Vec::with_capacity(items.len());
        let mut type_indices = Map::default_with_capacity(items.len());
        let mut reach = |type_id: TypeId, type_ids: &mut Vec<TypeId>| {
            let next_index = type_ids.len();
            if *type_indices.entry(type_id).or_insert(next_index) == next_index {
                type_ids.push(type_id);
            }
        };
        for item in items {
            reach(item.type_id, &mut type_ids);
        }

        let mut storage_types = Vec::with_capacity(type_ids.len());
        let mut index = 0;
        while let Some(&type_id) = type_ids.get(index) {
            index += 1;
            let layout = self.semantic.storage_type_layout(type_id)?;
            let kind = match layout.kind {
                semantic::StorageTypeKind::Value => StorageTypeKind::Value,
                semantic::StorageTypeKind::Bytes => StorageTypeKind::Bytes,
                semantic::StorageTypeKind::DynamicArray { element } => {
                    reach(element, &mut type_ids);
                    StorageTypeKind::DynamicArray { element }
                }
                semantic::StorageTypeKind::FixedSizeArray { element } => {
                    reach(element, &mut type_ids);
                    StorageTypeKind::FixedSizeArray { element }
                }
                semantic::StorageTypeKind::Mapping { key, value } => {
                    reach(key, &mut type_ids);
                    reach(value, &mut type_ids);
                    StorageTypeKind::Mapping { key, value }
                }
                semantic::StorageTypeKind::Struct { members } => {
                    let mut member_items = Vec::with_capacity(members.len());
                    for member in members {
                        reach(member.type_id, &mut type_ids);
                        let label = self
                            .semantic
                            .binder()
                            .find_definition_by_id(member.node_id)?
                            .identifier()
                            .unparse()
                            .to_string();
                        member_items.push(StorageItem {
                            node_id: member.node_id,
                            label,
                            slot: member.position.slot,
                            offset: member.position.offset,
                            type_id: member.type_id,
                            type_name: type_names.get(member.type_id),
                        });
                    }
                    StorageTypeKind::Struct {
                        members: member_items,
                    }
                }
            };
            storage_types.push(StorageType {
                type_id,
                label: type_names.get(type_id),
                size: layout.size,
                kind,
            });
        }
        Some((storage_types, type_indices))
    }
}

/// Spells each storage type once per contract, sharing the name between the
/// items, struct members and table entries that use it.
struct StorageTypeNames<'a> {
    semantic: &'a SemanticContext,
    names: Map<TypeId, Arc<str>>,
    buffer: String,
}

impl<'a> StorageTypeNames<'a> {
    fn new(semantic: &'a SemanticContext) -> Self {
        Self {
            semantic,
            names: Map::default(),
            buffer: String::new(),
        }
    }

    fn get(&mut self, type_id: TypeId) -> Arc<str> {
        if let Some(name) = self.names.get(&type_id) {
            return Arc::clone(name);
        }
        self.buffer.clear();
        self.semantic
            .write_type_abi_internal_name(type_id, &mut self.buffer);
        let name: Arc<str> = Arc::from(self.buffer.as_str());
        self.names.insert(type_id, Arc::clone(&name));
        name
    }
}
