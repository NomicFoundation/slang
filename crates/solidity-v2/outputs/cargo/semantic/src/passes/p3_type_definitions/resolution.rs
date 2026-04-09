use slang_solidity_v2_ir::interner::StringId;
use slang_solidity_v2_ir::ir;

use super::evaluator::{evaluate_compile_time_uint_constant, ConstantIdentifierResolver};
use super::Pass;
use crate::binder::{Definition, Resolution, ScopeId};
use crate::passes::common::resolve_identifier_path_in_scope;
use crate::types::{DataLocation, FunctionType, Type, TypeId};

impl Pass<'_> {
    // Resolves an IdentifierPath starting at the "contract" scope level, or at
    // the file level if there's no contract scope open.
    // Returns the resolution of the last reference.
    pub(super) fn resolve_identifier_path(
        &mut self,
        identifier_path: &ir::IdentifierPath,
    ) -> Resolution {
        // start resolution from the current contract (or file if there's no
        // contract scope open)
        let starting_scope_id = self.current_contract_or_file_scope_id();
        resolve_identifier_path_in_scope(self.binder, identifier_path, starting_scope_id)
    }

    pub(super) fn resolve_parameter_types(
        &mut self,
        parameters: &ir::Parameters,
    ) -> Option<Vec<TypeId>> {
        let mut parameter_types = Vec::new();
        for parameter in parameters {
            let parameter_type_id = self.binder.node_typing(parameter.id()).as_type_id()?;
            parameter_types.push(parameter_type_id);
        }
        Some(parameter_types)
    }

    pub(super) fn resolve_type_name(
        &mut self,
        type_name: &ir::TypeName,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        match type_name {
            ir::TypeName::ArrayTypeName(array_type_name) => {
                data_location.and_then(|data_location| {
                    self.resolve_type_name(&array_type_name.operand, Some(data_location))
                        .map(|element_type| {
                            if let Some(size_expression) = &array_type_name.index {
                                // TODO(validation): if the size of the array
                                // cannot be evaluated, it's not a compile-time
                                // constant
                                let size = evaluate_compile_time_uint_constant(
                                    size_expression,
                                    self.current_contract_or_file_scope_id(),
                                    self,
                                )
                                .unwrap_or_default();
                                self.types.register_type(Type::FixedSizeArray {
                                    element_type,
                                    size,
                                    location: data_location,
                                })
                            } else {
                                self.types.register_type(Type::Array {
                                    element_type,
                                    location: data_location,
                                })
                            }
                        })
                })
            }
            ir::TypeName::FunctionType(function_type) => {
                // NOTE: Keep in sync with `type_of_function_definition`
                let parameter_types = self.resolve_parameter_types(&function_type.parameters)?;
                let return_type = if let Some(returns) = &function_type.returns {
                    let return_types = self.resolve_parameter_types(returns)?;
                    if return_types.len() == 1 {
                        return_types.first().copied().unwrap()
                    } else {
                        self.types.register_type(Type::Tuple {
                            types: return_types,
                        })
                    }
                } else {
                    self.types.void()
                };
                Some(self.types.register_type(Type::Function(FunctionType {
                    definition_id: None,
                    implicit_receiver_type: None,
                    parameter_types,
                    return_type,
                    // TODO(validation): function types can only be internal or external
                    visibility: function_type.visibility,
                    mutability: function_type.mutability,
                })))
            }
            ir::TypeName::MappingType(mapping_type) => {
                let data_location = Some(DataLocation::Storage);
                let key_type_id =
                    self.resolve_type_name(&mapping_type.key_type.type_name, data_location)?;
                let value_type_id =
                    self.resolve_type_name(&mapping_type.value_type.type_name, data_location)?;
                Some(self.types.register_type(Type::Mapping {
                    key_type_id,
                    value_type_id,
                }))
            }
            ir::TypeName::ElementaryType(elementary_type) => {
                self.type_of_elementary_type(elementary_type, data_location)
            }
            ir::TypeName::IdentifierPath(identifier_path) => {
                self.type_of_identifier_path(identifier_path, data_location)
            }
        }
    }

    pub(super) fn find_library_scope_id_for_identifier_path(
        &self,
        identifier_path: &ir::IdentifierPath,
    ) -> Option<ScopeId> {
        let definition_id = identifier_path
            .last()
            .and_then(|identifier| {
                self.binder
                    .find_reference_by_identifier_node_id(identifier.id())
            })
            .and_then(|reference| {
                // reference may resolve to an imported library, so we need to
                // follow aliases
                self.binder
                    .follow_symbol_aliases(&reference.resolution)
                    .as_definition_id()
            })?;

        let Some(Definition::Library(_)) = self.binder.find_definition_by_id(definition_id) else {
            // the referenced definition is not a library
            return None;
        };
        self.binder.scope_id_for_node_id(definition_id)
    }
}

impl ConstantIdentifierResolver<ScopeId> for Pass<'_> {
    fn resolve_identifier_in_scope(
        &self,
        identifier: StringId,
        scope_id: &ScopeId,
    ) -> Option<(ir::Expression, ScopeId)> {
        let resolution = self.binder.resolve_in_scope(*scope_id, identifier);
        let definition_id = resolution.as_definition_id()?;
        match self.binder.find_definition_by_id(definition_id)? {
            Definition::Constant(constant_definition) => constant_definition
                .ir_node
                .value
                .as_ref()
                .map(|value| (value.clone(), constant_definition.scope_id)),
            _ => None,
        }
    }
}
