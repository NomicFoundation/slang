use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::{Definition, ParameterSlot, ParametersScope, RecursionKind};
use crate::types::{
    ArrayType, CallMode, DataLocation, FixedSizeArrayType, FunctionType, FunctionTypeMutability,
    MappingType, StructType, TupleType, Type, TypeId,
};

/// Why a public state variable has no getter.
pub(super) enum GetterError {
    /// A type it needs is unresolved, or is a library, which is not valid for
    /// a variable. Neither is a problem of the getter itself.
    Unresolved,
    /// It would return a value with no external representation.
    InvalidReturnType,
    /// It would return a struct with all its members omitted.
    NoReturnValues,
}

/// The storage element a getter reads, reached by indexing the state variable
/// with the getter's inputs.
struct GetterTarget<'a> {
    /// The element's type, before moving it to memory.
    type_id: TypeId,
    /// The value parameter of the innermost mapping indexed through, which
    /// names and declares the single output. It is kept when arrays are
    /// indexed after it, and is `None` if there are no mappings.
    mapping_value: Option<&'a ir::Parameter>,
}

impl Pass<'_> {
    /// Computes the type of the getter generated for the public state variable
    /// `state_variable`, of type `type_id`, together with its input and output
    /// parameter slots.
    ///
    /// The getter takes one input parameter per mapping key or array index
    /// needed to reach a value that is not indexed further, and returns that
    /// value: a tuple of values per returnable member if it is a struct, or a
    /// single output otherwise. Returns why there's no getter otherwise.
    pub(super) fn compute_getter_type(
        &mut self,
        receiver_type_id: Option<TypeId>,
        state_variable: &ir::StateVariableDefinition,
        type_id: TypeId,
    ) -> Result<(TypeId, ParametersScope, Vec<ParameterSlot>), GetterError> {
        let (inputs, target) = self.collect_getter_inputs(state_variable, type_id);
        let (return_type, outputs) = self.collect_getter_outputs(state_variable, &target)?;

        let parameter_types = inputs
            .parameters
            .iter()
            .map(|input| input.type_id().expect("getter inputs are typed"))
            .collect();
        let getter_type = Type::Function(FunctionType {
            definition_id: Some(state_variable.id()),
            implicit_receiver_type: receiver_type_id,
            parameter_types,
            return_type,
            mutability: FunctionTypeMutability::View,
            call_mode: CallMode::External,
        });
        Ok((self.types.register_type(getter_type), inputs, outputs))
    }

    /// Peels the mappings and arrays off the type of `state_variable`, adding
    /// one input parameter for each key or index to its getter, and returns the
    /// element they lead to.
    ///
    /// The semantic type and the declared type name of the state variable in
    /// the IR are walked in step: the type of a mapping or array was resolved
    /// from a mapping or array type name, so both always have the same shape.
    /// The type name gives each parameter its declaring node.
    fn collect_getter_inputs<'a>(
        &mut self,
        state_variable: &'a ir::StateVariableDefinition,
        type_id: TypeId,
    ) -> (ParametersScope, GetterTarget<'a>) {
        let mut inputs = ParametersScope::new();
        let mut target = GetterTarget {
            type_id,
            mapping_value: None,
        };
        // The type name declaring `target.type_id`.
        let mut type_name = &state_variable.type_name;

        loop {
            match self.types.get_type_by_id(target.type_id) {
                Type::Mapping(MappingType {
                    key_type_id,
                    value_type_id,
                }) => {
                    let ir::TypeName::MappingType(mapping_type) = type_name else {
                        unreachable!("a mapping type is declared by a mapping type name");
                    };
                    let key = &mapping_type.key_type;
                    inputs.add_parameter(self.typed_getter_slot(key, *key_type_id));

                    target.type_id = *value_type_id;
                    target.mapping_value = Some(&mapping_type.value_type);
                    type_name = &mapping_type.value_type.type_name;
                }
                Type::Array(ArrayType { element_type, .. })
                | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
                    let ir::TypeName::ArrayTypeName(array_type_name) = type_name else {
                        unreachable!("an array type is declared by an array type name");
                    };
                    // The index has no node of its own: link it to its array.
                    inputs.add_parameter(ParameterSlot::typed_unnamed(
                        array_type_name.id(),
                        self.types.uint256(),
                    ));

                    target.type_id = *element_type;
                    // Array items of a mapping value keep their name in the
                    // output for compatibility with the ABI, so we don't reset
                    // `target.mapping_value` here
                    type_name = &array_type_name.operand;
                }
                _ => return (inputs, target),
            }
        }
    }

    /// Computes the return type and output slots of a getter returning
    /// `target`. Values are always returned in memory.
    fn collect_getter_outputs(
        &mut self,
        state_variable: &ir::StateVariableDefinition,
        target: &GetterTarget<'_>,
    ) -> Result<(TypeId, Vec<ParameterSlot>), GetterError> {
        let return_type = match self.types.get_type_by_id(target.type_id) {
            Type::Address(_)
            | Type::Boolean
            | Type::ByteArray(_)
            | Type::Contract(_)
            | Type::Enum(_)
            | Type::FixedPointNumber(_)
            | Type::Integer(_)
            | Type::Interface(_)
            | Type::UserDefinedValue(_) => target.type_id,
            Type::Function(function_type) => {
                if !function_type.call_mode.is_external() {
                    return Err(GetterError::InvalidReturnType);
                }
                target.type_id
            }
            Type::Bytes(_) => self.types.bytes_memory(),
            Type::String(_) => self.types.string_memory(),

            Type::Struct(StructType { definition_id, .. }) => {
                return self.collect_getter_struct_outputs(*definition_id);
            }

            // Peeled off by `collect_getter_inputs`.
            Type::Array(_) | Type::FixedSizeArray(_) | Type::Mapping(_) => {
                unreachable!("getter target is still indexable")
            }
            // A library is resolved as a type, but it's not a valid one for a
            // variable, so there's no getter to compute.
            Type::Library(_) => return Err(GetterError::Unresolved),
            // Cannot be the type of a state variable.
            Type::ArraySlice(_)
            | Type::Error(_)
            | Type::Event(_)
            | Type::Literal(_)
            | Type::MetaType(_)
            | Type::Tuple(_)
            | Type::UserMetaType(_) => {
                unreachable!("cannot compute the getter type for {:?}", target.type_id)
            }
        };

        // The output is a single value: it's declared by the mapping value, if
        // any, or else by the state variable itself.
        let output = match target.mapping_value {
            Some(parameter) => self.typed_getter_slot(parameter, return_type),
            None => ParameterSlot::typed_unnamed(state_variable.id(), return_type),
        };
        Ok((return_type, vec![output]))
    }

    /// Returns a tuple of the members of the struct `definition_id` that a
    /// getter returns (mappings and arrays are omitted), in declaration
    /// order, with one output slot per member. A single member is returned
    /// as itself rather than as a one-element tuple.
    fn collect_getter_struct_outputs(
        &mut self,
        definition_id: NodeId,
    ) -> Result<(TypeId, Vec<ParameterSlot>), GetterError> {
        let Some(Definition::Struct(struct_definition)) =
            self.binder.find_definition_by_id(definition_id)
        else {
            unreachable!("struct type does not refer to a struct definition");
        };
        // Walk the IR members to keep the source order. Clone to avoid the borrow on `self`.
        let struct_node = Arc::clone(&struct_definition.ir_node);

        let mut outputs = Vec::new();
        for member in struct_node.members.iter() {
            let Some(member_type_id) = self.binder.node_typing(member.id()).as_type_id() else {
                return Err(GetterError::Unresolved);
            };
            let member_type = self.types.get_type_by_id(member_type_id);
            if matches!(
                member_type,
                Type::Array(_) | Type::FixedSizeArray(_) | Type::Mapping(_)
            ) {
                continue;
            }
            self.check_getter_can_return_whole(member_type_id)?;
            let member_type = self.types.get_type_by_id(member_type_id);
            let member_type_id = if member_type
                .data_location()
                .is_none_or(|location| location == DataLocation::Memory)
            {
                member_type_id
            } else {
                let member_type = member_type.clone();
                self.types
                    .register_type_with_data_location(member_type, DataLocation::Memory)
            };
            let Some(member_definition) = self.binder.find_definition_by_id(member.id()) else {
                unreachable!("struct member has no definition");
            };
            outputs.push(ParameterSlot::typed_named(
                member_definition,
                member_type_id,
            ));
        }

        let return_type = match outputs.as_slice() {
            [] => return Err(GetterError::NoReturnValues),
            [output] => output.type_id().expect("getter output is typed"),
            _ => {
                let types = outputs
                    .iter()
                    .map(|output| output.type_id().expect("getter output is typed"))
                    .collect();
                self.types.register_type(Type::Tuple(TupleType { types }))
            }
        };
        Ok((return_type, outputs))
    }

    /// Checks that a getter can return a value of type `type_id` as a whole,
    /// as it does a struct member: it needs an external representation, which
    /// an internal function, a mapping or a recursive struct doesn't have.
    fn check_getter_can_return_whole(&self, type_id: TypeId) -> Result<(), GetterError> {
        match self.types.get_type_by_id(type_id) {
            Type::Address(_)
            | Type::Boolean
            | Type::ByteArray(_)
            | Type::Bytes(_)
            | Type::Contract(_)
            | Type::Enum(_)
            | Type::FixedPointNumber(_)
            | Type::Integer(_)
            | Type::Interface(_)
            | Type::String(_)
            | Type::UserDefinedValue(_) => Ok(()),
            Type::Function(function_type) => {
                if function_type.call_mode.is_external() {
                    Ok(())
                } else {
                    Err(GetterError::InvalidReturnType)
                }
            }
            Type::Array(ArrayType { element_type, .. })
            | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
                self.check_getter_can_return_whole(*element_type)
            }
            Type::Mapping(_) => Err(GetterError::InvalidReturnType),
            Type::Struct(StructType { definition_id, .. }) => {
                let Some(Definition::Struct(struct_definition)) =
                    self.binder.find_definition_by_id(*definition_id)
                else {
                    unreachable!("struct type does not refer to a struct definition");
                };
                // A struct recursive only through function types is accepted,
                // and checking its members terminates, as the parameters of a
                // function type are not looked into.
                if matches!(
                    struct_definition.recursive,
                    Some(RecursionKind::ByReference | RecursionKind::ByValue)
                ) {
                    return Err(GetterError::InvalidReturnType);
                }
                for member in struct_definition.ir_node.members.iter() {
                    let Some(member_type_id) = self.binder.node_typing(member.id()).as_type_id()
                    else {
                        return Err(GetterError::Unresolved);
                    };
                    self.check_getter_can_return_whole(member_type_id)?;
                }
                Ok(())
            }
            // A library is resolved as a type, but it's not a valid one for a
            // variable.
            Type::Library(_) => Err(GetterError::Unresolved),
            // Cannot be the type of a struct member.
            Type::ArraySlice(_)
            | Type::Error(_)
            | Type::Event(_)
            | Type::Literal(_)
            | Type::MetaType(_)
            | Type::Tuple(_)
            | Type::UserMetaType(_) => {
                unreachable!("cannot be the type of a struct member: {type_id:?}")
            }
        }
    }

    /// The getter slot declared by a mapping's key or value `parameter`,
    /// named after its definition if it has a name.
    fn typed_getter_slot(&self, parameter: &ir::Parameter, type_id: TypeId) -> ParameterSlot {
        if parameter.name.is_none() {
            return ParameterSlot::typed_unnamed(parameter.id(), type_id);
        }
        let Some(definition) = self.binder.find_definition_by_id(parameter.id()) else {
            unreachable!("named parameter has no definition");
        };
        ParameterSlot::typed_named(definition, type_id)
    }
}
