use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::Definition;
use crate::types::{
    AddressType, ArrayType, ContractType, DataLocation, EnumType, FixedSizeArrayType, FunctionType,
    InterfaceType, LibraryType, MappingType, StringType, StructType, TupleType, Type, TypeId,
    UserDefinedValueType,
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
            mutability: (&function_definition.attributes.mutability).into(),
            call_mode: (&function_definition.attributes.visibility).into(),
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
        if !function_definition.is_externally_visible()
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
