use itertools::Either;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::types::{TupleType, Type};

use crate::abi::types::AbiTypeCache;
use crate::abi::{AbiEntry, AbiFunction, AbiMutability, AbiParameter, selector_from_signature};
use crate::ast::{StateVariableDefinitionStruct, StateVariableVisibility, TypeName};

impl StateVariableDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.attributes().visibility(),
            StateVariableVisibility::Public
        )
    }

    /// The getter's ABI inputs and outputs, from its function type. Function types don't track
    /// parameter names, so solc's are read off the declaration: mapping key names for the
    /// inputs, and for the outputs the struct members the return type is built from, else the
    /// innermost mapping's value name.
    fn extract_getter_type_parameters_abi(
        &self,
        cache: &mut AbiTypeCache,
    ) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
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
            return None;
        };
        let (input_names, value_name) = self.getter_parameter_names();

        let mut inputs = Vec::new();
        for (index, parameter_type_id) in function_type.parameter_types.iter().enumerate() {
            inputs.push(AbiParameter {
                node_id: None,
                name: input_names.get(index).cloned().flatten(),
                abi_type: cache.abi_type(&self.semantic, *parameter_type_id)?,
                type_id: *parameter_type_id,
                indexed: false,
            });
        }

        // A tuple return type stands for multiple return values, so it is flattened.
        let output_types = match self
            .semantic
            .types()
            .get_type_by_id(function_type.return_type)
        {
            Type::Tuple(TupleType { types }) => Either::Left(types.iter()),
            _ => Either::Right(std::iter::once(&function_type.return_type)),
        };
        let mut output_names = if definition.getter_member_ids.is_empty() {
            Either::Left(std::iter::once(value_name))
        } else {
            Either::Right(definition.getter_member_ids.iter().map(|member_id| {
                self.semantic
                    .binder()
                    .find_definition_by_id(*member_id)
                    .map(|member| member.identifier().unparse().to_string())
            }))
        };
        let outputs = output_types
            .map(|output_type_id| {
                Some(AbiParameter {
                    node_id: None,
                    name: output_names.next().flatten(),
                    abi_type: cache.abi_type(&self.semantic, *output_type_id)?,
                    type_id: *output_type_id,
                    indexed: false,
                })
            })
            .collect::<Option<Vec<_>>>()?;
        Some((inputs, outputs))
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
        self.compute_abi_entry_cached(&mut AbiTypeCache::default())
    }

    pub(crate) fn compute_abi_entry_cached(&self, cache: &mut AbiTypeCache) -> Option<AbiEntry> {
        if !self.is_externally_visible() {
            return None;
        }
        let (inputs, outputs) = self.extract_getter_type_parameters_abi(cache)?;

        Some(AbiEntry::Function(AbiFunction::new(
            self.ir_node.id(),
            self.ir_node.name.unparse().to_string(),
            inputs,
            outputs,
            AbiMutability::View,
        )))
    }

    pub fn compute_canonical_signature(&self) -> Option<String> {
        let (inputs, _) = self.extract_getter_type_parameters_abi(&mut AbiTypeCache::default())?;
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
