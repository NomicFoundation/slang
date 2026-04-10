use slang_solidity_v2_semantic::binder;

use crate::abi::{
    extract_function_type_parameters_abi, selector_from_signature, AbiEntry, AbiFunction,
    AbiMutability, AbiParameter,
};
use crate::ast::{StateVariableDefinitionStruct, StateVariableVisibility};

impl StateVariableDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(self.visibility(), StateVariableVisibility::Public)
    }

    fn extract_getter_type_parameters_abi(&self) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
        let binder::Definition::StateVariable(definition) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())?
        else {
            unreachable!("definition is not a state variable");
        };
        extract_function_type_parameters_abi(&self.semantic, definition.getter_type_id?)
    }

    pub fn compute_abi_entry(&self) -> Option<AbiEntry> {
        if !self.is_externally_visible() {
            return None;
        }
        let (inputs, outputs) = self.extract_getter_type_parameters_abi()?;

        Some(AbiEntry::Function(AbiFunction {
            node_id: self.ir_node.id(),
            name: self
                .ir_node
                .name
                .unparse(self.semantic.interner())
                .to_string(),
            inputs,
            outputs,
            state_mutability: AbiMutability::View,
        }))
    }

    pub fn compute_selector(&self) -> Option<u32> {
        if !self.is_externally_visible() {
            return None;
        }
        let (inputs, _) = self.extract_getter_type_parameters_abi()?;

        let signature = format!(
            "{name}({parameters})",
            name = self.ir_node.name.unparse(self.semantic.interner()),
            parameters = inputs
                .into_iter()
                .map(|parameter| parameter.type_name().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );

        Some(selector_from_signature(&signature))
    }
}
