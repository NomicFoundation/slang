use std::fmt;

use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::{AbiNameError, AbiTypeSpelling};
use slang_solidity_v2_semantic::types::{FunctionType, Type, TypeId};

use crate::abi::entries::{Entry, Function, Parameter, SpellingPolicy};
use crate::abi::{AbiMutability, SignatureHasher};
use crate::ast::{StateVariableDefinitionStruct, StateVariableVisibility};

impl StateVariableDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.attributes().visibility(),
            StateVariableVisibility::Public
        )
    }

    /// The type of the variable's generated getter. Only a public variable has one, so every getter
    /// computation starts here.
    fn getter_function_type(&self) -> Option<&FunctionType> {
        let Type::Function(function_type) =
            self.semantic.types().get_type_by_id(self.getter_type_id()?)
        else {
            unreachable!("getter type is not a function");
        };
        Some(function_type)
    }

    /// The getter's ABI inputs and outputs.
    fn extract_getter_type_parameters_abi(&self) -> Option<(Vec<Parameter>, Vec<Parameter>)> {
        let to_abi = |parameters: &[binder::ParameterDefinition]| {
            parameters
                .iter()
                .map(|parameter| {
                    Parameter::new(
                        parameter.name().map(str::to_owned),
                        parameter.type_id()?,
                        false,
                        &self.semantic,
                    )
                })
                .collect::<Option<Vec<_>>>()
        };
        Some((
            to_abi(self.getter_parameters()?)?,
            to_abi(self.getter_return_parameters()?)?,
        ))
    }

    pub(crate) fn compute_abi_entry(&self) -> Option<Entry> {
        let (inputs, outputs) = self.extract_getter_type_parameters_abi()?;

        Some(Entry::Function(Function {
            node_id: self.ir_node.id(),
            name: self.ir_node.name.unparse().to_string(),
            inputs,
            outputs,
            state_mutability: AbiMutability::View,
            spelling_policy: SpellingPolicy::of(self.enclosing_definition().as_ref()),
        }))
    }

    pub fn compute_canonical_signature(&self) -> Option<String> {
        let mut signature = String::new();
        self.write_canonical_signature(self.getter_function_type()?, &mut signature)
            .ok()?;
        Some(signature)
    }

    pub fn compute_internal_signature(&self) -> Option<String> {
        let mut signature = String::new();
        self.write_signature(
            self.getter_function_type()?,
            &mut signature,
            |out, type_id| Ok(self.semantic.write_type_internal_name(type_id, out)?),
        )
        .ok()?;
        Some(signature)
    }

    pub fn compute_selector(&self) -> Option<u32> {
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
        self.write_signature(getter_type, out, |out, type_id| {
            self.semantic
                .write_type_abi_name(type_id, AbiTypeSpelling::Selector, out)
        })
    }

    /// Writes `name(T1,T2)` over the getter's parameter types, each written by `write_type`.
    fn write_signature<W: fmt::Write>(
        &self,
        getter_type: &FunctionType,
        out: &mut W,
        write_type: impl Fn(&mut W, TypeId) -> Result<(), AbiNameError>,
    ) -> Result<(), AbiNameError> {
        write!(out, "{}(", self.ir_node.name.unparse())?;
        for (index, type_id) in getter_type.parameter_types.iter().enumerate() {
            if index > 0 {
                out.write_char(',')?;
            }
            write_type(out, *type_id)?;
        }
        Ok(out.write_char(')')?)
    }
}
