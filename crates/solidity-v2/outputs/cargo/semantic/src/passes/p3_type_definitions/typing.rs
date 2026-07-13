use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::Definition;
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

    pub(super) fn compute_getter_type(
        &mut self,
        receiver_type_id: TypeId,
        definition_id: NodeId,
        type_id: TypeId,
    ) -> Option<TypeId> {
        let mut return_type = type_id;
        let mut parameter_types = Vec::new();

        loop {
            match self.types.get_type_by_id(return_type) {
                // scalar types
                Type::Address(_)
                | Type::Boolean
                | Type::ByteArray(_)
                | Type::Contract(_)
                | Type::Enum(_)
                | Type::FixedPointNumber(_)
                | Type::Function(_)
                | Type::Integer(_)
                | Type::Interface(_)
                | Type::UserDefinedValue(_) => break,

                Type::Bytes(_) => {
                    // getters will always return values in memory
                    return_type = self.types.bytes_memory();
                    break;
                }
                Type::String(_) => {
                    // getters will always return values in memory
                    return_type = self.types.string_memory();
                    break;
                }

                Type::Struct(StructType { definition_id, .. }) => {
                    // For structs the getter will return a tuple with all value
                    // type and string/bytes fields, in declaration order. It
                    // won't return nested mappings or arrays.
                    // We iterate the struct's IR members directly: the struct
                    // scope is a name->id map with no stable ordering, but the
                    // getter's tuple must preserve the source field order.
                    let Some(Definition::Struct(struct_definition)) =
                        self.binder.find_definition_by_id(*definition_id)
                    else {
                        unreachable!("struct type does not refer to a struct definition");
                    };
                    let member_ids: Vec<NodeId> = struct_definition
                        .ir_node
                        .members
                        .iter()
                        .map(|member| member.id())
                        .collect();
                    let mut types = Vec::new();
                    for member_id in member_ids {
                        let Some(member_type_id) = self.binder.node_typing(member_id).as_type_id()
                        else {
                            // member type cannot be resolved
                            return None;
                        };
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
                            // Data location is always memory for getters, so we
                            // need to override it if necessary
                            let member_type = member_type.clone();
                            self.types
                                .register_type_with_data_location(member_type, DataLocation::Memory)
                        };
                        types.push(member_type_id);
                    }
                    return_type = match types.len() {
                        0 => return None,
                        1 => types[0],
                        _ => self.types.register_type(Type::Tuple(TupleType { types })),
                    };
                    break;
                }

                // non-scalar types
                Type::Array(ArrayType { element_type, .. })
                | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
                    return_type = *element_type;
                    parameter_types.push(self.types.uint256());
                }
                Type::Mapping(MappingType {
                    key_type_id,
                    value_type_id,
                }) => {
                    return_type = *value_type_id;
                    parameter_types.push(*key_type_id);
                }

                // invalid types
                Type::Library(_)
                | Type::Literal(_)
                | Type::MetaType(_)
                | Type::Tuple(_)
                | Type::UserMetaType(_)
                | Type::Void => {
                    unreachable!("cannot compute the getter type for {type_id:?}")
                }
            }
        }

        let getter_type = Type::Function(FunctionType {
            definition_id: Some(definition_id),
            implicit_receiver_type: Some(receiver_type_id),
            parameter_types,
            return_type,
            visibility: FunctionTypeVisibility::Public,
            mutability: FunctionTypeMutability::View,
            partially_applied: false,
        });
        Some(self.types.register_type(getter_type))
    }

    pub(super) fn visit_parameters(&mut self, parameters: &ir::Parameters) {
        for parameter in parameters {
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
