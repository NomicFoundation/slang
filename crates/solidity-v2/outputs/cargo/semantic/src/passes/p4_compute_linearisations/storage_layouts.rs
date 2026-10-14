//! Laying out each contract's state variables in storage.

use std::sync::Arc;

use ruint::aliases::U256;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::{NoStorageSize, StateVariablePosition, StorageAnalyzer, StorageLayoutBuilder};
use crate::types::TypeRegistry;

/// The persistent and transient storage layouts of `contract`, given the
/// `state_variables` of its hierarchy. A layout is `None` when it overflows,
/// when a variable has no storage size, or when the base slot failed to
/// evaluate.
pub(super) fn compute_storage_layouts(
    binder: &Binder,
    types: &TypeRegistry,
    contract: &ir::ContractDefinition,
    state_variables: &[ir::StateVariableDefinition],
) -> (
    Option<Vec<StateVariablePosition>>,
    Option<Vec<StateVariablePosition>>,
) {
    let analyzer = StorageAnalyzer::new(binder, types);
    let Some(Definition::Contract(definition)) = binder.find_definition_by_id(contract.id()) else {
        unreachable!("definition is not a contract");
    };

    // TODO(validation) SDR[2]: it is an error if any contract in the hierarchy
    // other than the leaf has a custom offset layout
    let persistent = match (&contract.storage_layout, definition.base_slot) {
        // A base slot that failed to evaluate is already reported.
        (Some(_), None) => None,
        (_, base_slot) => lay_out(
            &analyzer,
            binder,
            base_slot.unwrap_or(U256::ZERO),
            ir::StateVariableMutability::Mutable,
            state_variables,
        )
        .ok(),
    };
    let transient = lay_out(
        &analyzer,
        binder,
        U256::ZERO,
        ir::StateVariableMutability::Transient,
        state_variables,
    )
    .ok();

    (persistent, transient)
}

/// Positions the `state_variables` with the given `mutability` from
/// `base_slot`, in order.
fn lay_out(
    analyzer: &StorageAnalyzer<'_>,
    binder: &Binder,
    base_slot: U256,
    mutability: ir::StateVariableMutability,
    state_variables: &[ir::StateVariableDefinition],
) -> Result<Vec<StateVariablePosition>, NoStorageSize> {
    let count = state_variables
        .iter()
        .filter(|state_variable| state_variable.attributes.mutability == mutability)
        .count();
    if count == 0 {
        return Ok(Vec::new());
    }
    let mut positions = Vec::with_capacity(count);
    let mut builder = StorageLayoutBuilder::new(base_slot);
    for state_variable in state_variables {
        if state_variable.attributes.mutability != mutability {
            continue;
        }
        let type_id = binder
            .node_typing(state_variable.id())
            .as_type_id()
            .ok_or(NoStorageSize::Unsized)?;
        let size = analyzer.storage_size_of_type_id(type_id)?;
        let position = builder.allocate(size).ok_or(NoStorageSize::TooLarge)?;
        positions.push(StateVariablePosition {
            state_variable: Arc::clone(state_variable),
            type_id,
            position,
        });
    }
    builder.slots_used().ok_or(NoStorageSize::TooLarge)?;
    Ok(positions)
}
