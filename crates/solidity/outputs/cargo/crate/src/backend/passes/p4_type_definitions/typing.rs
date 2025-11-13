use super::Pass;
use crate::backend::binder::{Definition, Scope};
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::backend::types::{DataLocation, FunctionType, FunctionTypeKind, Type, TypeId};
use crate::cst::NodeId;

impl Pass<'_> {
    pub(super) fn type_of_identifier_path(
        &mut self,
        identifier_path: &input_ir::IdentifierPath,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        identifier_path
            .last()
            .and_then(|identifier| {
                self.binder
                    .find_reference_by_identifier_node_id(identifier.id())
            })
            .and_then(|reference| reference.resolution.as_definition_id())
            .and_then(|node_id| self.type_of_definition(node_id, data_location))
    }

    fn type_of_definition(
        &mut self,
        definition_id: NodeId,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        if let Some(definition) = self.binder.find_definition_by_id(definition_id) {
            match definition {
                Definition::Contract(_) | Definition::Library(_) => {
                    // TODO: do we need a separate type for libraries?
                    Some(self.types.register_type(Type::Contract { definition_id }))
                }
                Definition::Enum(_) => Some(self.types.register_type(Type::Enum { definition_id })),
                Definition::Interface(_) => {
                    Some(self.types.register_type(Type::Interface { definition_id }))
                }
                Definition::Struct(_) => data_location.map(|data_location| {
                    self.types.register_type(Type::Struct {
                        definition_id,
                        location: data_location,
                    })
                }),
                Definition::TypeParameter(_) => {
                    unreachable!("type parameter names don't have a type")
                }
                Definition::UserDefinedValueType(_) => Some(
                    self.types
                        .register_type(Type::UserDefinedValue { definition_id }),
                ),

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
                | Definition::YulLabel(_)
                | Definition::YulParameter(_)
                | Definition::YulVariable(_) => None,
            }
        } else {
            None
        }
    }

    pub(super) fn type_of_elementary_type(
        &mut self,
        elementary_type: &input_ir::ElementaryType,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        match elementary_type {
            input_ir::ElementaryType::AddressType(address_type) => {
                Some(self.types.register_type(Type::Address {
                    payable: address_type.payable_keyword,
                }))
            }
            input_ir::ElementaryType::BytesKeyword(bytes_keyword) => {
                Type::from_bytes_keyword(&bytes_keyword.unparse(), data_location)
                    .map(|type_| self.types.register_type(type_))
            }
            input_ir::ElementaryType::IntKeyword(int_keyword) => Some(
                self.types
                    .register_type(Type::from_int_keyword(&int_keyword.unparse())),
            ),
            input_ir::ElementaryType::UintKeyword(uint_keyword) => Some(
                self.types
                    .register_type(Type::from_uint_keyword(&uint_keyword.unparse())),
            ),
            input_ir::ElementaryType::FixedKeyword(fixed_keyword) => Some(
                self.types
                    .register_type(Type::from_fixed_keyword(&fixed_keyword.unparse())),
            ),
            input_ir::ElementaryType::UfixedKeyword(ufixed_keyword) => Some(
                self.types
                    .register_type(Type::from_ufixed_keyword(&ufixed_keyword.unparse())),
            ),
            input_ir::ElementaryType::BoolKeyword => Some(self.types.boolean()),
            input_ir::ElementaryType::ByteKeyword => {
                Some(self.types.register_type(Type::ByteArray { width: 1 }))
            }
            input_ir::ElementaryType::StringKeyword => data_location.map(|data_location| {
                self.types.register_type(Type::String {
                    location: data_location,
                })
            }),
        }
    }

    pub(super) fn type_of_function_definition(
        &mut self,
        function_definition: &input_ir::FunctionDefinition,
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
                self.types.register_type(Type::Tuple {
                    types: return_types,
                })
            }
        } else {
            self.types.void()
        };
        let kind = (&function_definition.mutability).into();
        let external = matches!(
            function_definition.visibility,
            input_ir::FunctionVisibility::External | input_ir::FunctionVisibility::Public
        );
        Some(self.types.register_type(Type::Function(FunctionType {
            definition_id: Some(function_definition.node_id),
            implicit_receiver_type,
            parameter_types,
            return_type,
            external,
            kind,
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
                Type::Address { .. }
                | Type::Boolean
                | Type::ByteArray { .. }
                | Type::Bytes { .. }
                | Type::Contract { .. }
                | Type::Enum { .. }
                | Type::FixedPointNumber { .. }
                | Type::Function { .. }
                | Type::Integer { .. }
                | Type::Interface { .. }
                | Type::String { .. }
                | Type::UserDefinedValue { .. } => break,

                Type::Struct { definition_id, .. } => {
                    // For structs the getter will return a tuple with all value
                    // type and string/bytes fields. It won't return nested
                    // structs or arrays.

                    // To retrieve the fields we need to go through the
                    // scope associated to the struct...
                    let scope_id = self
                        .binder
                        .scope_id_for_node_id(*definition_id)
                        .expect("definition in struct type has no associated scope");
                    let Scope::Struct(struct_scope) = self.binder.get_scope_by_id(scope_id) else {
                        unreachable!("definition in struct type has no valid scope");
                    };
                    let mut types = Vec::new();
                    for member_id in struct_scope.definitions.values() {
                        let Some(member_type_id) = self.binder.node_typing(*member_id).as_type_id()
                        else {
                            // member type cannot be resolved
                            return None;
                        };
                        if self
                            .types
                            .get_type_by_id(member_type_id)
                            .can_return_from_getter()
                        {
                            types.push(member_type_id);
                        }
                    }
                    return_type = match types.len() {
                        0 => return None,
                        1 => types[0],
                        _ => self.types.register_type(Type::Tuple { types }),
                    };
                    break;
                }

                // non-scalar types
                Type::Array { element_type, .. } => {
                    return_type = *element_type;
                    parameter_types.push(self.types.uint256());
                }
                Type::Mapping {
                    key_type_id,
                    value_type_id,
                } => {
                    return_type = *value_type_id;
                    parameter_types.push(*key_type_id);
                }

                // invalid types
                Type::Literal(_) | Type::Tuple { .. } | Type::Void => {
                    unreachable!("cannot compute the getter type for {type_id:?}")
                }
            }
        }

        let getter_type = Type::Function(FunctionType {
            definition_id: Some(definition_id),
            implicit_receiver_type: Some(receiver_type_id),
            parameter_types,
            return_type,
            external: true,
            kind: FunctionTypeKind::View,
        });
        Some(self.types.register_type(getter_type))
    }

    pub(super) fn visit_parameters(
        &mut self,
        parameters: &input_ir::Parameters,
        default_location: Option<DataLocation>,
    ) {
        for parameter in parameters {
            let type_id = self.resolve_type_name(
                &parameter.type_name,
                parameter
                    .storage_location
                    .as_ref()
                    .map(Into::into)
                    .or(default_location),
            );
            self.binder.set_node_type(parameter.node_id, type_id);
        }
    }

    pub(super) fn register_super_types(&mut self, type_id: TypeId, bases: &[NodeId]) {
        // register super types in the type registry
        let super_types = bases
            .iter()
            .filter_map(|base_id| {
                let Some(base) = self.binder.find_definition_by_id(*base_id) else {
                    unreachable!("base for {base_id:?} does not exist");
                };
                let base_type = match base {
                    Definition::Contract(_) => Type::Contract {
                        definition_id: *base_id,
                    },
                    Definition::Interface(_) => Type::Interface {
                        definition_id: *base_id,
                    },
                    _ => return None,
                };
                Some(self.types.register_type(base_type))
            })
            .collect();
        self.types.register_super_types(type_id, super_types);
    }
}
