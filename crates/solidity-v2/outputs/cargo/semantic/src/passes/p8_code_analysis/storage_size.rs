//! Checks that each contract's state variables fit in storage.
//!
//! A contract lays out the persistent state variables of its whole hierarchy,
//! from its storage layout base slot, so this needs the linearised state
//! variables and the evaluated base slot, both final by this pass.

use ruint::aliases::U256;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    ContractTooLargeForStorage, StorageLayoutBasePastEnd,
};
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::{ContractData, FileNodeMapper, StorageAnalyzer, StorageLayoutBuilder};
use crate::passes::common::node_location;
use crate::types::TypeRegistry;

/// Reports every contract whose persistent state variables extend past the
/// end of storage.
pub(crate) fn check_contract_storage_sizes(
    binder: &Binder,
    contract_data: &ContractData,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let analyzer = StorageAnalyzer::new(binder, types);
    for contract in contract_data.all_contracts() {
        let Some(Definition::Contract(definition)) = binder.find_definition_by_id(contract.id())
        else {
            continue;
        };
        // A base slot that failed to evaluate is already reported.
        let base_slot = match (&contract.storage_layout, definition.base_slot) {
            (Some(_), None) => continue,
            (_, base_slot) => base_slot.unwrap_or(U256::ZERO),
        };
        if fits_in_storage(
            &analyzer,
            binder,
            base_slot,
            contract_data.linearised_state_variables(contract.id()),
        ) {
            continue;
        }
        match &contract.storage_layout {
            Some(base_slot_expression) if !base_slot.is_zero() => {
                let (file_id, range) = node_location(base_slot_expression, file_node_mapper);
                diagnostics.push(file_id, range, StorageLayoutBasePastEnd);
            }
            _ => diagnostics.push(
                file_node_mapper
                    .file_id_from_node_id(contract.id())
                    .to_owned(),
                contract.range.clone(),
                ContractTooLargeForStorage,
            ),
        }
    }
}

/// Whether the persistent `state_variables` fit in storage from `base_slot`.
/// A variable with no storage size is reported where its type is checked, so
/// the contract is assumed to fit.
fn fits_in_storage(
    analyzer: &StorageAnalyzer<'_>,
    binder: &Binder,
    base_slot: U256,
    state_variables: &[ir::StateVariableDefinition],
) -> bool {
    let mut builder = StorageLayoutBuilder::new(base_slot);
    for state_variable in state_variables {
        if !matches!(
            state_variable.attributes.mutability,
            ir::StateVariableMutability::Mutable
        ) {
            continue;
        }
        let Some(type_id) = binder.node_typing(state_variable.id()).as_type_id() else {
            return true;
        };
        let Ok(size) = analyzer.storage_size(type_id) else {
            return true;
        };
        if builder.allocate(size).is_none() {
            return false;
        }
    }
    builder.slots_used().is_some()
}
