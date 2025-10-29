use std::rc::Rc;

use super::Pass;
use crate::backend::binder::{
    ContractDefinition, Definition, ImportDefinition, InterfaceDefinition, LibraryDefinition,
    Reference, Resolution, ScopeId,
};
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::backend::types::{DataLocation, FunctionType, Type, TypeId};

impl Pass {
    // Resolves an IdentifierPath. It starts resolution at the "contract" scope
    // level, or at the file level if there's no contract scope open. It will
    // follow through in contracts/intrefaces/libraries as well as imports and
    // treat them as namespaces.
    // Returns the resolution of the last reference.
    pub(super) fn resolve_identifier_path(
        &mut self,
        identifier_path: &input_ir::IdentifierPath,
    ) -> Resolution {
        // start resolution from the current contract (or file if there's no
        // contract scope open)
        let mut scope_id = Some(self.current_contract_or_file_scope_id());
        let mut use_lexical_resolution = true;
        let mut last_resolution: Resolution = Resolution::Unresolved;

        for identifier in identifier_path {
            let symbol = identifier.unparse();
            let resolution = if let Some(scope_id) = scope_id {
                if use_lexical_resolution {
                    self.binder.resolve_in_scope(scope_id, &symbol)
                } else {
                    self.binder.resolve_in_scope_as_namespace(scope_id, &symbol)
                }
            } else {
                Resolution::Unresolved
            };
            let definition_id = resolution.as_definition_id();

            let reference = Reference::new(Rc::clone(identifier), resolution.clone());
            self.binder.insert_reference(reference);

            // recurse into file scopes pointed by the resolved definition
            // to resolve the next identifier in the path
            scope_id = definition_id
                .and_then(|node_id| self.binder.find_definition_by_id(node_id))
                .and_then(|definition| match definition {
                    Definition::Import(ImportDefinition {
                        resolved_file_id, ..
                    }) => resolved_file_id.as_ref().and_then(|resolved_file_id| {
                        self.binder.scope_id_for_file_id(resolved_file_id)
                    }),
                    Definition::Contract(ContractDefinition { node_id, .. })
                    | Definition::Interface(InterfaceDefinition { node_id, .. })
                    | Definition::Library(LibraryDefinition { node_id, .. }) => {
                        use_lexical_resolution = false;
                        self.binder.scope_id_for_node_id(*node_id)
                    }
                    _ => None,
                });

            last_resolution = resolution;
        }
        last_resolution
    }

    pub(super) fn resolve_parameter_types(
        &mut self,
        parameters: &input_ir::Parameters,
    ) -> Option<Vec<TypeId>> {
        let mut parameter_types = Vec::new();
        for parameter in parameters {
            let parameter_type_id = self.binder.node_typing(parameter.node_id).as_type_id()?;
            parameter_types.push(parameter_type_id);
        }
        Some(parameter_types)
    }

    pub(super) fn resolve_type_name(
        &mut self,
        type_name: &input_ir::TypeName,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        match type_name {
            input_ir::TypeName::ArrayTypeName(array_type_name) => {
                data_location.and_then(|data_location| {
                    self.resolve_type_name(&array_type_name.operand, Some(data_location))
                        .map(|element_type| {
                            self.types.register_type(Type::Array {
                                element_type,
                                location: data_location,
                            })
                        })
                })
            }
            input_ir::TypeName::FunctionType(function_type) => {
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
                let kind = (&function_type.mutability).into();
                let external = matches!(
                    function_type.visibility,
                    input_ir::FunctionVisibility::External | input_ir::FunctionVisibility::Public
                );
                Some(self.types.register_type(Type::Function(FunctionType {
                    definition_id: None,
                    implicit_receiver_type: None,
                    parameter_types,
                    return_type,
                    external,
                    kind,
                })))
            }
            input_ir::TypeName::MappingType(mapping_type) => {
                let data_location = Some(DataLocation::Storage);
                let key_type_id = match &mapping_type.key_type.key_type {
                    input_ir::MappingKeyType::ElementaryType(elementary_type) => {
                        self.type_of_elementary_type(elementary_type, data_location)
                    }
                    input_ir::MappingKeyType::IdentifierPath(identifier_path) => {
                        self.type_of_identifier_path(identifier_path, data_location)
                    }
                }?;
                let value_type_id =
                    self.resolve_type_name(&mapping_type.value_type.type_name, data_location)?;
                Some(self.types.register_type(Type::Mapping {
                    key_type_id,
                    value_type_id,
                }))
            }
            input_ir::TypeName::ElementaryType(elementary_type) => {
                self.type_of_elementary_type(elementary_type, data_location)
            }
            input_ir::TypeName::IdentifierPath(identifier_path) => {
                self.type_of_identifier_path(identifier_path, data_location)
            }
        }
    }

    pub(super) fn find_library_scope_id_for_identifier_path(
        &self,
        identifier_path: &input_ir::IdentifierPath,
    ) -> Option<ScopeId> {
        let definition_id = identifier_path
            .last()
            .and_then(|identifier| {
                self.binder
                    .find_reference_by_identifier_node_id(identifier.id())
            })
            .and_then(|reference| reference.resolution.as_definition_id())?;

        let Some(Definition::Library(_)) = self.binder.find_definition_by_id(definition_id) else {
            // the referenced definition is not a library
            return None;
        };
        self.binder.scope_id_for_node_id(definition_id)
    }
}
