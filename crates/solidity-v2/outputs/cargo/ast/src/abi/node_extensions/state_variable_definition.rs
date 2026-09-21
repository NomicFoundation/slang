use itertools::Either;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::types::{FunctionType, TupleType, Type};

use crate::abi::{AbiEntry, AbiFunction, AbiMutability, AbiParameter, selector_from_signature};
use crate::ast::{StateVariableDefinitionStruct, StateVariableVisibility};

impl StateVariableDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.attributes().visibility(),
            StateVariableVisibility::Public
        )
    }

    /// The type of the variable's generated getter, if it has one.
    fn getter_function_type(&self) -> Option<&FunctionType> {
        let Type::Function(function_type) =
            self.semantic.types().get_type_by_id(self.getter_type_id()?)
        else {
            unreachable!("getter type is not a function");
        };
        Some(function_type)
    }

    /// The getter's ABI inputs and outputs, named as solc names them.
    fn extract_getter_type_parameters_abi(&self) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
        let function_type = self.getter_function_type()?;
        let (input_names, value_name) = self.getter_parameter_names();

        assert_eq!(
            input_names.len(),
            function_type.parameter_types.len(),
            "getter inputs follow the declared mapping and array nesting"
        );
        let inputs = function_type
            .parameter_types
            .iter()
            .zip(input_names)
            .map(|(parameter_type_id, name)| {
                AbiParameter::new(None, name, *parameter_type_id, false, &self.semantic)
            })
            .collect::<Option<Vec<_>>>()?;

        // A tuple as a return type from a function represents multiple return
        // values, so we need to flatten it
        let output_types = match self
            .semantic
            .types()
            .get_type_by_id(function_type.return_type)
        {
            Type::Tuple(TupleType { types }) => Either::Left(types.iter()),
            _ => Either::Right(std::iter::once(&function_type.return_type)),
        };
        let member_ids = self.getter_member_ids();
        let output_names = if member_ids.is_empty() {
            Either::Left(std::iter::once(value_name))
        } else {
            Either::Right(member_ids.iter().map(|member_id| {
                let Some(member) = self.semantic.binder().find_definition_by_id(*member_id) else {
                    unreachable!("getter member without a definition");
                };
                Some(member.identifier().unparse().to_string())
            }))
        };
        assert_eq!(
            output_types.len(),
            output_names.len(),
            "getter outputs are the struct members or the single value"
        );
        let outputs = output_types
            .zip(output_names)
            .map(|(output_type_id, name)| {
                AbiParameter::new(None, name, *output_type_id, false, &self.semantic)
            })
            .collect::<Option<Vec<_>>>()?;
        Some((inputs, outputs))
    }

    /// solc's getter parameter names, read off the declared type: each mapping key's name for
    /// the inputs, an array index unnamed, and the innermost mapping's value name for the output.
    fn getter_parameter_names(&self) -> (Vec<Option<String>>, Option<String>) {
        let mut input_names = Vec::new();
        let mut value_name = None;
        let mut type_name = &self.ir_node.type_name;
        loop {
            match type_name {
                ir::TypeName::MappingType(mapping) => {
                    input_names.push(parameter_name(&mapping.key_type));
                    value_name = parameter_name(&mapping.value_type);
                    type_name = &mapping.value_type.type_name;
                }
                ir::TypeName::ArrayTypeName(array) => {
                    input_names.push(None);
                    type_name = &array.operand;
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
        let parameters = self
            .getter_function_type()?
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

fn parameter_name(parameter: &ir::Parameter) -> Option<String> {
    parameter
        .name
        .as_ref()
        .map(|name| name.unparse().to_string())
}
