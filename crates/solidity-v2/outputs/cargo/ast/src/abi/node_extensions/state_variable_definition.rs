use itertools::Either;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::types::{TupleType, Type};

use crate::abi::{AbiEntry, AbiFunction, AbiMutability, AbiParameter, selector_from_signature};
use crate::ast::{StateVariableDefinitionStruct, StateVariableVisibility};

impl StateVariableDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.attributes().visibility(),
            StateVariableVisibility::Public
        )
    }

    /// The getter's ABI inputs and outputs, from its function type.
    // TODO: our type system doesn't track parameter names for function types,
    // so we can't convey that information in the ABI. This is important for
    // getters where we should transfer that information from mapping or struct
    // types (eg. a getter that returns a struct should name its output
    // parameters from the struct members).
    fn extract_getter_type_parameters_abi(&self) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
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
        let mut inputs = Vec::with_capacity(function_type.parameter_types.len());
        for parameter_type_id in &function_type.parameter_types {
            inputs.push(AbiParameter::new(
                &self.semantic,
                None,
                None,
                *parameter_type_id,
                false,
            )?);
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
        let outputs = output_types
            .map(|output_type_id| {
                AbiParameter::new(&self.semantic, None, None, *output_type_id, false)
            })
            .collect::<Option<Vec<_>>>()?;
        Some((inputs, outputs))
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
