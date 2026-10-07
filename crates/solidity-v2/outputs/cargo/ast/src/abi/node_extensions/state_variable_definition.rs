use std::fmt;

use itertools::Either;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::{AbiNameError, AbiTypeSpelling};
use slang_solidity_v2_semantic::types::{FunctionType, TupleType, Type};

use crate::abi::{
    AbiEntry, AbiFunction, AbiMutability, AbiParameter, SignatureHasher, TypeSpelling,
};
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

    /// The getter's ABI inputs and outputs.
    fn extract_getter_type_parameters_abi(&self) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
        let function_type = self.getter_function_type()?;
        let (input_names, output_names) = self.compute_input_output_parameter_names();

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

    /// Compute getter parameter names, extracting them from the declared type: each mapping key's
    /// name for the inputs, an array index unnamed, and for the outputs the struct members the
    /// getter returns, else the innermost mapping's value name.
    fn compute_input_output_parameter_names(&self) -> (Vec<Option<String>>, Vec<Option<String>>) {
        let mut input_names = Vec::new();
        let mut value_name = None;
        let mut type_name = &self.ir_node.type_name;
        loop {
            match type_name {
                ir::TypeName::MappingType(mapping) => {
                    input_names.push(mapping.key_type.name_as_string());
                    value_name = mapping.value_type.name_as_string();
                    type_name = &mapping.value_type.type_name;
                }
                ir::TypeName::ArrayTypeName(array) => {
                    input_names.push(None);
                    type_name = &array.operand;
                }
                _ => break,
            }
        }
        let member_ids = self.getter_member_ids();
        let output_names = if member_ids.is_empty() {
            vec![value_name]
        } else {
            member_ids
                .iter()
                .map(|member_id| {
                    let Some(member) = self.semantic.binder().find_definition_by_id(*member_id)
                    else {
                        unreachable!("getter member without a definition");
                    };
                    Some(member.identifier().unparse().to_string())
                })
                .collect()
        };
        (input_names, output_names)
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
            type_spelling: TypeSpelling::of_function_in(self.enclosing_definition().as_ref()),
        }))
    }

    pub fn compute_canonical_signature(&self) -> Option<String> {
        let mut signature = String::new();
        self.write_canonical_signature(self.getter_function_type()?, &mut signature)
            .ok()?;
        Some(signature)
    }

    pub fn compute_internal_signature(&self) -> Option<String> {
        if !self.is_externally_visible() {
            // There is no getter defined if the variable is not public
            return None;
        }
        let getter_type = self.getter_function_type()?;
        let mut signature = format!("{}(", self.ir_node.name.unparse());
        for (index, type_id) in getter_type.parameter_types.iter().enumerate() {
            if index > 0 {
                signature.push(',');
            }
            self.semantic
                .write_type_internal_name(*type_id, &mut signature)
                .ok()?;
        }
        signature.push(')');
        Some(signature)
    }

    pub fn compute_selector(&self) -> Option<u32> {
        if !self.is_externally_visible() {
            return None;
        }
        let mut hasher = SignatureHasher::default();
        self.write_canonical_signature(self.getter_function_type()?, &mut hasher)
            .ok()?;
        Some(hasher.selector())
    }

    fn write_canonical_signature(
        &self,
        getter_type: &FunctionType,
        out: &mut impl fmt::Write,
    ) -> Result<(), AbiNameError> {
        write!(out, "{}(", self.ir_node.name.unparse())?;
        for (index, type_id) in getter_type.parameter_types.iter().enumerate() {
            if index > 0 {
                out.write_char(',')?;
            }
            self.semantic
                .write_type_abi_name(*type_id, AbiTypeSpelling::Selector, out)?;
        }
        Ok(out.write_char(')')?)
    }
}
