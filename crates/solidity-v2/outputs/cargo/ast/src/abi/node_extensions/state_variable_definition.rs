use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::types::Type;

use crate::abi::{
    AbiEntry, AbiFunction, AbiMutability, AbiParameter, extract_function_type_parameters_abi,
    selector_from_signature,
};
use crate::ast::{StateVariableDefinitionStruct, StateVariableVisibility, TypeName};

impl StateVariableDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.attributes().visibility(),
            StateVariableVisibility::Public
        )
    }

    fn extract_getter_type_parameters_abi(&self) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
        let binder::Definition::StateVariable(definition) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())?
        else {
            unreachable!("definition is not a state variable");
        };
        let (input_names, value_name) = self.getter_parameter_names();
        extract_function_type_parameters_abi(
            &self.semantic,
            definition.getter_type_id?,
            &input_names,
            &definition.getter_member_ids,
            value_name.as_deref(),
        )
    }

    /// The names solc gives a getter's parameters, read off the declared type: each mapping
    /// key's name for the inputs, an array index unnamed, and the innermost mapping's value
    /// name for the output.
    fn getter_parameter_names(&self) -> (Vec<Option<String>>, Option<String>) {
        let mut input_names = Vec::new();
        let mut value_name = None;
        let mut type_name = self.type_name();
        loop {
            match type_name {
                TypeName::MappingType(mapping) => {
                    input_names.push(mapping.key_type().name().map(|name| name.name().to_owned()));
                    let value = mapping.value_type();
                    value_name = value.name().map(|name| name.name().to_owned());
                    type_name = value.type_name();
                }
                TypeName::ArrayTypeName(array) => {
                    input_names.push(None);
                    type_name = array.operand();
                }
                _ => break,
            }
        }
        (input_names, value_name)
    }

    pub fn compute_abi_entry(&self) -> Option<AbiEntry> {
        if !self.is_externally_visible() {
            return None;
        }
        let (inputs, outputs) = self.extract_getter_type_parameters_abi()?;

        Some(AbiEntry::Function(AbiFunction {
            node_id: self.ir_node.id(),
            name: self.ir_node.name.unparse().to_string(),
            inputs,
            outputs,
            state_mutability: AbiMutability::View,
        }))
    }

    pub fn compute_canonical_signature(&self) -> Option<String> {
        let (inputs, _) = self.extract_getter_type_parameters_abi()?;
        let parameters = inputs
            .into_iter()
            .map(|parameter| parameter.type_name())
            .collect::<Vec<_>>()
            .join(",");
        Some(format!(
            "{name}({parameters})",
            name = self.ir_node.name.unparse(),
        ))
    }

    pub fn compute_internal_signature(&self) -> Option<String> {
        if !self.is_externally_visible() {
            // There is no getter defined if the variable is not public
            return None;
        }
        let binder::Definition::StateVariable(definition) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())?
        else {
            unreachable!("definition is not a state variable");
        };
        let Type::Function(function_type) = self
            .semantic
            .types()
            .get_type_by_id(definition.getter_type_id?)
        else {
            unreachable!("getter type is not a function");
        };
        let parameters = function_type
            .parameter_types
            .iter()
            .map(|type_id| self.semantic.type_internal_name(*type_id))
            .collect::<Vec<_>>()
            .join(",");
        Some(format!(
            "{name}({parameters})",
            name = self.ir_node.name.unparse(),
        ))
    }

    pub fn compute_selector(&self) -> Option<u32> {
        if !self.is_externally_visible() {
            return None;
        }
        self.compute_canonical_signature()
            .map(|sig| selector_from_signature(&sig))
    }
}
