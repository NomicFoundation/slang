use std::sync::Arc;

use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::{Definition, ParameterDefinition, ParametersScope};
use crate::types::{
    AddressType, ArrayType, ContractType, DataLocation, EnumType, FixedSizeArrayType, FunctionType,
    FunctionTypeMutability, FunctionTypeVisibility, InterfaceType, LibraryType, MappingType,
    StringType, StructType, TupleType, Type, TypeId, UserDefinedValueType,
};

impl Pass<'_> {
    pub(super) fn type_of_identifier_path(
        &mut self,
        identifier_path: &ir::IdentifierPath,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        identifier_path
            .last()
            .and_then(|identifier| {
                self.binder
                    .find_reference_by_identifier_node_id(identifier.id())
            })
            .and_then(|reference| {
                // We should follow symbol aliases here. This is only relevant
                // if the path has a single element, as in other cases symbols
                // cannot be aliased.
                self.binder
                    .follow_symbol_aliases(reference.resolution.clone())
                    .as_definition_id()
            })
            .and_then(|node_id| self.type_of_definition(node_id, data_location))
    }

    fn type_of_definition(
        &mut self,
        definition_id: NodeId,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        if let Some(definition) = self.binder.find_definition_by_id(definition_id) {
            match definition {
                Definition::Contract(_) => Some(
                    self.types
                        .register_type(Type::Contract(ContractType { definition_id })),
                ),
                Definition::Library(_) => Some(
                    self.types
                        .register_type(Type::Library(LibraryType { definition_id })),
                ),
                Definition::Enum(_) => Some(
                    self.types
                        .register_type(Type::Enum(EnumType { definition_id })),
                ),
                Definition::Interface(_) => Some(
                    self.types
                        .register_type(Type::Interface(InterfaceType { definition_id })),
                ),
                Definition::Struct(_) => data_location.map(|data_location| {
                    self.types.register_type(Type::Struct(StructType {
                        definition_id,
                        location: data_location,
                    }))
                }),
                Definition::TypeParameter(_) => {
                    unreachable!("type parameter names don't have a type")
                }
                Definition::UserDefinedValueType(_) => Some(self.types.register_type(
                    Type::UserDefinedValue(UserDefinedValueType { definition_id }),
                )),

                Definition::Constant(_)
                | Definition::EnumMember(_)
                | Definition::Error(_)
                | Definition::Event(_)
                | Definition::Function(_)
                | Definition::Import(_)
                | Definition::ImportedSymbol(_)
                | Definition::Modifier(_)
                | Definition::Parameter(_)
                | Definition::StateVariable(_)
                | Definition::StructMember(_)
                | Definition::Variable(_)
                | Definition::YulFunction(_)
                | Definition::YulParameter(_)
                | Definition::YulVariable(_) => None,
            }
        } else {
            None
        }
    }

    pub(super) fn type_of_elementary_type(
        &mut self,
        elementary_type: &ir::ElementaryType,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        match elementary_type {
            ir::ElementaryType::AddressType(address_type) => {
                Some(self.types.register_type(Type::Address(AddressType {
                    is_payable: address_type.is_payable,
                })))
            }
            ir::ElementaryType::BytesKeyword(bytes_keyword) => {
                Type::from_bytes_keyword(bytes_keyword.unparse(), data_location)
                    .map(|type_| self.types.register_type(type_))
            }
            ir::ElementaryType::IntKeyword(int_keyword) => Some(
                self.types
                    .register_type(Type::from_int_keyword(int_keyword.unparse())),
            ),
            ir::ElementaryType::UintKeyword(uint_keyword) => Some(
                self.types
                    .register_type(Type::from_uint_keyword(uint_keyword.unparse())),
            ),
            ir::ElementaryType::FixedKeyword(fixed_keyword) => Some(
                self.types
                    .register_type(Type::from_fixed_keyword(fixed_keyword.unparse())),
            ),
            ir::ElementaryType::UfixedKeyword(ufixed_keyword) => Some(
                self.types
                    .register_type(Type::from_ufixed_keyword(ufixed_keyword.unparse())),
            ),
            ir::ElementaryType::BoolKeyword(_) => Some(self.types.boolean()),
            // No ByteKeyword in v2 (removed since Solidity >= 0.8.0)
            ir::ElementaryType::StringKeyword(_) => data_location.map(|data_location| {
                self.types.register_type(Type::String(StringType {
                    location: data_location,
                }))
            }),
        }
    }

    pub(super) fn type_of_function_definition(
        &mut self,
        function_definition: &ir::FunctionDefinition,
        implicit_receiver_type: Option<TypeId>,
    ) -> Option<TypeId> {
        // NOTE: Keep in sync with function types in `resolve_type_name` in `super::resolution`
        let parameter_types = self.resolve_parameter_types(&function_definition.parameters)?;
        let return_type = if let Some(returns) = &function_definition.returns {
            let return_types = self.resolve_parameter_types(returns)?;
            if return_types.len() == 1 {
                // TODO: I think this is more correct, but we may decide to
                // always return a tuple instead; this will require changing the
                // typing of function call expressions
                return_types.first().copied().unwrap()
            } else {
                self.types.register_type(Type::Tuple(TupleType {
                    types: return_types,
                }))
            }
        } else {
            self.types.void()
        };
        Some(self.types.register_type(Type::Function(FunctionType {
            definition_id: Some(function_definition.id()),
            implicit_receiver_type,
            parameter_types,
            return_type,
            visibility: (&function_definition.attributes.visibility).into(),
            mutability: (&function_definition.attributes.mutability).into(),
            partially_applied: false,
        })))
    }

    /// Computes the type an externally visible function is dispatched through,
    /// or `None` if it is never dispatched by selector. Only a `Regular`
    /// function is: a constructor, fallback or receive carries no name to
    /// select on.
    pub(super) fn compute_externalized_type(
        &mut self,
        function_definition: &ir::FunctionDefinition,
        type_id: TypeId,
    ) -> Option<TypeId> {
        let Type::Function(function_type) = self.types.get_type_by_id(type_id) else {
            unreachable!("function definition is not typed as a function");
        };
        if !function_type.is_externally_visible()
            || !matches!(function_definition.kind, ir::FunctionKind::Regular)
        {
            return None;
        }
        Some(self.types.externalize_function_type(type_id))
    }

    /// Registers the `Storage` form of every struct member type reachable in
    /// storage from `type_id`, the type of a state variable, so a storage
    /// layout names one type id per type. Struct members are typed with an
    /// `Inherited` location; everything else a state variable reaches is
    /// already typed in storage. `registered_structs` holds the structs already
    /// handled, which also stops at recursive ones.
    pub(super) fn register_struct_member_types_in_storage(
        &mut self,
        type_id: TypeId,
        registered_structs: &mut Set<NodeId>,
    ) {
        match self.types.get_type_by_id(type_id) {
            Type::Array(ArrayType { element_type, .. })
            | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
                self.register_struct_member_types_in_storage(*element_type, registered_structs);
            }
            Type::Mapping(MappingType { value_type_id, .. }) => {
                self.register_struct_member_types_in_storage(*value_type_id, registered_structs);
            }
            Type::Struct(StructType { definition_id, .. }) => {
                if !registered_structs.insert(*definition_id) {
                    return;
                }
                let Some(Definition::Struct(struct_definition)) =
                    self.binder.find_definition_by_id(*definition_id)
                else {
                    return;
                };
                let member_type_ids: Vec<_> = struct_definition
                    .ir_node
                    .members
                    .iter()
                    .filter_map(|member| self.binder.node_typing(member.id()).as_type_id())
                    .collect();
                for member_type_id in member_type_ids {
                    let storage_type_id = self
                        .types
                        .register_inherited_to_storage_type_mapping(member_type_id);
                    self.register_struct_member_types_in_storage(
                        storage_type_id,
                        registered_structs,
                    );
                }
            }
            _ => {}
        }
    }

    /// Computes the type of the getter generated for the public state variable
    /// `state_variable`, of type `type_id`, together with its input and output
    /// parameter slots.
    ///
    /// The getter takes one input parameter per mapping key or array index
    /// needed to reach a value that is not indexed further, and returns that
    /// value: a tuple of values per returnable member if it is a struct, or a
    /// single output otherwise. Returns `None` if the value is a struct with an
    /// unresolved member type or with no returnable member.
    pub(super) fn compute_getter_type(
        &mut self,
        receiver_type_id: Option<TypeId>,
        state_variable: &ir::StateVariableDefinition,
        type_id: TypeId,
    ) -> Option<(TypeId, ParametersScope, Vec<ParameterDefinition>)> {
        let (inputs, target) = self.collect_getter_inputs(state_variable, type_id);
        let (return_type, outputs) = self.collect_getter_outputs(state_variable, &target)?;

        let parameter_types = inputs
            .parameters
            .iter()
            .map(|input| input.type_id.expect("getter inputs are typed"))
            .collect();
        let getter_type = Type::Function(FunctionType {
            definition_id: Some(state_variable.id()),
            implicit_receiver_type: receiver_type_id,
            parameter_types,
            return_type,
            visibility: FunctionTypeVisibility::External,
            mutability: FunctionTypeMutability::View,
            partially_applied: false,
        });
        Some((self.types.register_type(getter_type), inputs, outputs))
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
                    inputs.add_typed_parameter(key.name_as_string(), key.id(), *key_type_id);

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
                    inputs.add_typed_parameter(None, array_type_name.id(), self.types.uint256());

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
    ) -> Option<(TypeId, Vec<ParameterDefinition>)> {
        let return_type = match self.types.get_type_by_id(target.type_id) {
            Type::Address(_)
            | Type::Boolean
            | Type::ByteArray(_)
            | Type::Contract(_)
            | Type::Enum(_)
            | Type::FixedPointNumber(_)
            | Type::Function(_)
            | Type::Integer(_)
            | Type::Interface(_)
            | Type::UserDefinedValue(_) => target.type_id,
            Type::Bytes(_) => self.types.bytes_memory(),
            Type::String(_) => self.types.string_memory(),

            Type::Struct(StructType { definition_id, .. }) => {
                return self.collect_getter_struct_outputs(*definition_id);
            }

            // Peeled off by `collect_getter_inputs`.
            Type::Array(_) | Type::FixedSizeArray(_) | Type::Mapping(_) => {
                unreachable!("getter target is still indexable")
            }
            // Cannot be the type of a state variable.
            Type::Library(_)
            | Type::ArraySlice(_)
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
            Some(parameter) => ParameterDefinition {
                name: parameter.name_as_string(),
                node_id: parameter.id(),
                type_id: Some(return_type),
            },
            None => ParameterDefinition {
                name: None,
                node_id: state_variable.id(),
                type_id: Some(return_type),
            },
        };
        Some((return_type, vec![output]))
    }

    /// Returns a tuple of the members of the struct `definition_id` that a
    /// getter can return (mappings and arrays are skipped), in declaration
    /// order, with one output slot per member. A single member is returned
    /// as itself rather than as a one-element tuple.
    fn collect_getter_struct_outputs(
        &mut self,
        definition_id: NodeId,
    ) -> Option<(TypeId, Vec<ParameterDefinition>)> {
        let Some(Definition::Struct(struct_definition)) =
            self.binder.find_definition_by_id(definition_id)
        else {
            unreachable!("struct type does not refer to a struct definition");
        };
        // Walk the IR members to keep the source order. Clone to avoid the borrow on `self`.
        let struct_node = Arc::clone(&struct_definition.ir_node);

        let mut outputs = Vec::new();
        for member in struct_node.members.iter() {
            let member_type_id = self.binder.node_typing(member.id()).as_type_id()?;
            let member_type = self.types.get_type_by_id(member_type_id);
            if !member_type.can_return_from_getter_directly() {
                continue;
            }
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
            outputs.push(ParameterDefinition {
                name: Some(member.name.text.clone()),
                node_id: member.id(),
                type_id: Some(member_type_id),
            });
        }

        let return_type = match outputs.as_slice() {
            [] => return None,
            [output] => output.type_id.expect("getter output is typed"),
            _ => {
                let types = outputs
                    .iter()
                    .map(|output| output.type_id.expect("getter output is typed"))
                    .collect();
                self.types.register_type(Type::Tuple(TupleType { types }))
            }
        };
        Some((return_type, outputs))
    }

    pub(super) fn visit_parameters(&mut self, parameters: &ir::Parameters) {
        for parameter in parameters.iter() {
            let type_id = self.resolve_type_name(
                &parameter.type_name,
                parameter.storage_location.as_ref().map(Into::into),
            );
            self.binder.set_node_type(parameter.id(), type_id);
        }
    }

    pub(super) fn register_super_types(&mut self, type_id: TypeId, bases: &[NodeId]) {
        // register super types in the type registry
        let super_types: Vec<_> = bases
            .iter()
            .filter_map(|base_id| {
                let Some(base) = self.binder.find_definition_by_id(*base_id) else {
                    unreachable!("base for {base_id:?} does not exist");
                };
                let base_type = match base {
                    Definition::Contract(_) => Type::Contract(ContractType {
                        definition_id: *base_id,
                    }),
                    Definition::Interface(_) => Type::Interface(InterfaceType {
                        definition_id: *base_id,
                    }),
                    _ => return None,
                };
                Some(self.types.register_type(base_type))
            })
            .collect();
        self.types.register_super_types(type_id, &super_types);
    }
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
