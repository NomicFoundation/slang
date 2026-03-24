use std::rc::Rc;

use slang_solidity_v2_cst::structured_cst::nodes as input;

#[path = "default.generated.rs"]
mod default;
use default::Builder;

use super::Source;
use crate::ir::nodes as output;

pub fn build_source_unit(
    source_unit: &input::SourceUnit,
    source: &impl Source,
) -> output::SourceUnit {
    let mut builder = CstToIrBuilder { source };
    builder.build_source_unit(source_unit)
}

struct CstToIrBuilder<'a, S: Source> {
    pub source: &'a S,
}

impl<S: Source> Builder for CstToIrBuilder<'_, S> {
    fn unparse_range(&self, range: std::ops::Range<usize>) -> String {
        self.source.text(range).to_owned()
    }

    //
    // Abstract sequence methods
    //

    fn build_constant_definition(
        &mut self,
        source: &input::ConstantDefinition,
    ) -> output::ConstantDefinition {
        let type_name = self.build_type_name(&source.type_name);
        let name = self.build_identifier(&source.name);
        let visibility = None;
        let value = Some(self.build_expression(&source.value));

        Rc::new(output::ConstantDefinitionStruct {
            type_name,
            name,
            visibility,
            value,
        })
    }

    fn build_contract_definition(
        &mut self,
        source: &input::ContractDefinition,
    ) -> output::ContractDefinition {
        let abstract_keyword = source.abstract_keyword.is_some();
        let name = self.build_identifier(&source.name);
        let members = self.build_contract_members(&source.members);
        let inheritance_types = source
            .specifiers
            .elements
            .iter()
            .find_map(|specifier| {
                if let input::ContractSpecifier::InheritanceSpecifier(inheritance) = specifier {
                    Some(self.build_inheritance_specifier(inheritance))
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let storage_layout = source.specifiers.elements.iter().find_map(|specifier| {
            if let input::ContractSpecifier::StorageLayoutSpecifier(storage_layout) = specifier {
                Some(self.build_storage_layout_specifier(storage_layout))
            } else {
                None
            }
        });

        Rc::new(output::ContractDefinitionStruct {
            abstract_keyword,
            name,
            members,
            inheritance_types,
            storage_layout,
        })
    }

    fn build_error_definition(
        &mut self,
        source: &input::ErrorDefinition,
    ) -> output::ErrorDefinition {
        let name = self.build_identifier(&source.name);
        let parameters = source
            .members
            .parameters
            .elements
            .iter()
            .map(|parameter| self.build_error_parameter(parameter))
            .collect();

        Rc::new(output::ErrorDefinitionStruct { name, parameters })
    }

    fn build_event_definition(
        &mut self,
        source: &input::EventDefinition,
    ) -> output::EventDefinition {
        let name = self.build_identifier(&source.name);
        let anonymous_keyword = source.anonymous_keyword.is_some();
        let parameters = source
            .parameters
            .parameters
            .elements
            .iter()
            .map(|parameter| self.build_event_parameter(parameter))
            .collect();

        Rc::new(output::EventDefinitionStruct {
            name,
            anonymous_keyword,
            parameters,
        })
    }

    fn build_function_definition(
        &mut self,
        source: &input::FunctionDefinition,
    ) -> output::FunctionDefinition {
        let (kind, name) = match &source.name {
            input::FunctionName::Identifier(identifier) => (
                output::FunctionKind::Regular,
                Some(self.build_identifier(identifier)),
            ),
            input::FunctionName::FallbackKeyword(_) => (output::FunctionKind::Fallback, None),
            input::FunctionName::ReceiveKeyword(_) => (output::FunctionKind::Receive, None),
        };
        let visibility = Self::function_visibility(&source.attributes);
        let mutability = Self::function_mutability(&source.attributes);
        let virtual_keyword = source
            .attributes
            .elements
            .iter()
            .any(|attribute| matches!(attribute, input::FunctionAttribute::VirtualKeyword(_)));
        // TODO(validation): function definitions can have only a single override specifier
        let override_specifier = self.function_override_specifier(&source.attributes);
        let modifier_invocations = self.function_modifier_invocations(&source.attributes);
        let body = self.build_function_body(&source.body);
        let parameters = self.build_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.build_returns_declaration(returns));

        Rc::new(output::FunctionDefinitionStruct {
            parameters,
            returns,
            kind,
            name,
            body,
            visibility,
            mutability,
            virtual_keyword,
            override_specifier,
            modifier_invocations,
        })
    }

    fn build_function_type(&mut self, source: &input::FunctionType) -> output::FunctionType {
        let parameters = self.build_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.build_returns_declaration(returns));
        let visibility = Self::function_type_visibility(&source.attributes);
        let mutability = Self::function_type_mutability(&source.attributes);

        Rc::new(output::FunctionTypeStruct {
            parameters,
            returns,
            visibility,
            mutability,
        })
    }

    fn build_index_access_expression(
        &mut self,
        source: &input::IndexAccessExpression,
    ) -> output::IndexAccessExpression {
        let operand = self.build_expression(&source.operand);
        let start = source
            .start
            .as_ref()
            .map(|start| self.build_expression(start));
        let end = source
            .end
            .as_ref()
            .and_then(|end| end.end.as_ref().map(|end| self.build_expression(end)));

        Rc::new(output::IndexAccessExpressionStruct {
            operand,
            start,
            end,
        })
    }

    fn build_mapping_type(&mut self, source: &input::MappingType) -> output::MappingType {
        let key_type = self.build_mapping_key_as_parameter(&source.key_type);
        let value_type = self.build_mapping_value_as_parameter(&source.value_type);

        Rc::new(output::MappingTypeStruct {
            key_type,
            value_type,
        })
    }

    fn build_parameter(&mut self, source: &input::Parameter) -> output::Parameter {
        let type_name = self.build_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|location| self.build_storage_location(location));
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Rc::new(output::ParameterStruct {
            type_name,
            storage_location,
            name,
            indexed: false,
        })
    }

    fn build_state_variable_definition(
        &mut self,
        source: &input::StateVariableDefinition,
    ) -> output::StateVariableDefinition {
        let type_name = self.build_type_name(&source.type_name);
        let name = self.build_identifier(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.build_state_variable_definition_value(value));
        let visibility = Self::state_variable_visibility(&source.attributes);
        let mutability = Self::state_variable_mutability(&source.attributes);
        let override_specifier = self.state_variable_override_specifier(&source.attributes);

        Rc::new(output::StateVariableDefinitionStruct {
            type_name,
            name,
            value,
            visibility,
            mutability,
            override_specifier,
        })
    }

    //
    // Choice dispatch overrides
    //

    fn build_import_clause(&mut self, source: &input::ImportClause) -> output::ImportClause {
        match source {
            input::ImportClause::NamedImport(named_import) => output::ImportClause::PathImport(
                self.build_named_import_as_path_import(named_import),
            ),
            _ => self.default_build_import_clause(source),
        }
    }

    fn build_contract_member(&mut self, source: &input::ContractMember) -> output::ContractMember {
        match source {
            input::ContractMember::ConstructorDefinition(constructor) => {
                output::ContractMember::FunctionDefinition(
                    self.build_constructor_as_function(constructor),
                )
            }
            input::ContractMember::FallbackFunctionDefinition(fallback) => {
                output::ContractMember::FunctionDefinition(
                    self.build_fallback_as_function(fallback),
                )
            }
            input::ContractMember::ReceiveFunctionDefinition(receive) => {
                output::ContractMember::FunctionDefinition(self.build_receive_as_function(receive))
            }
            input::ContractMember::ModifierDefinition(modifier) => {
                output::ContractMember::FunctionDefinition(
                    self.build_modifier_as_function(modifier),
                )
            }
            input::ContractMember::StateVariableDefinition(state_variable_definition) => {
                let output = self.build_state_variable_definition(state_variable_definition);
                if matches!(output.mutability, output::StateVariableMutability::Constant)
                    && !matches!(output.visibility, output::StateVariableVisibility::Public)
                {
                    output::ContractMember::ConstantDefinition(
                        Self::state_variable_as_constant_definition(
                            Rc::into_inner(output).expect("Created node is not yet shared"),
                        ),
                    )
                } else {
                    output::ContractMember::StateVariableDefinition(output)
                }
            }
            _ => self.default_build_contract_member(source),
        }
    }

    fn build_arguments_declaration(
        &mut self,
        source: &input::ArgumentsDeclaration,
    ) -> output::ArgumentsDeclaration {
        match source {
            input::ArgumentsDeclaration::PositionalArgumentsDeclaration(positional) => {
                output::ArgumentsDeclaration::PositionalArguments(
                    self.build_positional_arguments(&positional.arguments),
                )
            }
            input::ArgumentsDeclaration::NamedArgumentsDeclaration(named) => {
                output::ArgumentsDeclaration::NamedArguments(
                    self.build_named_argument_group(&named.arguments),
                )
            }
        }
    }
}

//
// Private helper methods
//

impl<S: Source> CstToIrBuilder<'_, S> {
    fn build_function_body(&mut self, source: &input::FunctionBody) -> Option<output::Block> {
        match source {
            input::FunctionBody::Block(block) => Some(self.build_block(block)),
            input::FunctionBody::Semicolon(_) => None,
        }
    }

    fn function_visibility(attributes: &input::FunctionAttributes) -> output::FunctionVisibility {
        // TODO(validation): only a single visibility keyword can be provided
        // TODO(validation): free functions are always internal, but
        // otherwise a visibility *must* be set explicitly (>= 0.8.0)
        attributes.elements.iter().fold(
            // For >= 0.8.0, default for free functions is internal
            output::FunctionVisibility::Internal,
            |visibility, attribute| match attribute {
                input::FunctionAttribute::ExternalKeyword(_) => {
                    output::FunctionVisibility::External
                }
                input::FunctionAttribute::InternalKeyword(_) => {
                    output::FunctionVisibility::Internal
                }
                input::FunctionAttribute::PrivateKeyword(_) => output::FunctionVisibility::Private,
                input::FunctionAttribute::PublicKeyword(_) => output::FunctionVisibility::Public,
                _ => visibility,
            },
        )
    }

    fn function_mutability(attributes: &input::FunctionAttributes) -> output::FunctionMutability {
        // TODO(validation): only a single mutability keyword can be provided
        attributes.elements.iter().fold(
            output::FunctionMutability::NonPayable,
            |mutability, attribute| match attribute {
                input::FunctionAttribute::PayableKeyword(_) => output::FunctionMutability::Payable,
                input::FunctionAttribute::PureKeyword(_) => output::FunctionMutability::Pure,
                input::FunctionAttribute::ViewKeyword(_) => output::FunctionMutability::View,
                _ => mutability,
            },
        )
    }

    fn function_override_specifier(
        &mut self,
        attributes: &input::FunctionAttributes,
    ) -> Option<output::OverridePaths> {
        attributes.elements.iter().find_map(|attribute| {
            if let input::FunctionAttribute::OverrideSpecifier(specifier) = attribute {
                Some(self.build_override_specifier_as_paths(specifier))
            } else {
                None
            }
        })
    }

    fn function_modifier_invocations(
        &mut self,
        attributes: &input::FunctionAttributes,
    ) -> output::ModifierInvocations {
        attributes
            .elements
            .iter()
            .filter_map(|attribute| {
                if let input::FunctionAttribute::ModifierInvocation(mi) = attribute {
                    Some(self.build_modifier_invocation(mi))
                } else {
                    None
                }
            })
            .collect()
    }

    fn function_type_visibility(
        attributes: &input::FunctionTypeAttributes,
    ) -> output::FunctionVisibility {
        // TODO(validation): only a single visibility keyword can be provided
        attributes.elements.iter().fold(
            output::FunctionVisibility::Internal,
            |visibility, attribute| match attribute {
                input::FunctionTypeAttribute::ExternalKeyword(_) => {
                    output::FunctionVisibility::External
                }
                input::FunctionTypeAttribute::InternalKeyword(_) => {
                    output::FunctionVisibility::Internal
                }
                input::FunctionTypeAttribute::PrivateKeyword(_) => {
                    output::FunctionVisibility::Private
                }
                input::FunctionTypeAttribute::PublicKeyword(_) => {
                    output::FunctionVisibility::Public
                }
                _ => visibility,
            },
        )
    }

    fn function_type_mutability(
        attributes: &input::FunctionTypeAttributes,
    ) -> output::FunctionMutability {
        // TODO(validation): only a single mutability keyword can be provided
        attributes.elements.iter().fold(
            output::FunctionMutability::NonPayable,
            |mutability, attribute| match attribute {
                input::FunctionTypeAttribute::PayableKeyword(_) => {
                    output::FunctionMutability::Payable
                }
                input::FunctionTypeAttribute::PureKeyword(_) => output::FunctionMutability::Pure,
                input::FunctionTypeAttribute::ViewKeyword(_) => output::FunctionMutability::View,
                _ => mutability,
            },
        )
    }

    //
    // Constructor / Fallback / Receive / Modifier → FunctionDefinition
    //

    fn build_constructor_as_function(
        &mut self,
        source: &input::ConstructorDefinition,
    ) -> output::FunctionDefinition {
        let kind = output::FunctionKind::Constructor;
        let name = None;
        let visibility = Self::constructor_visibility(&source.attributes);
        let mutability = Self::constructor_mutability(&source.attributes);
        // v2 ConstructorAttribute has no VirtualKeyword
        let virtual_keyword = false;
        // v2 ConstructorAttribute has no OverrideSpecifier
        let override_specifier = None;
        let modifier_invocations = self.constructor_modifier_invocations(&source.attributes);
        let body = Some(self.build_block(&source.body));
        let parameters = self.build_parameters_declaration(&source.parameters);
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            parameters,
            returns,
            kind,
            name,
            body,
            visibility,
            mutability,
            virtual_keyword,
            override_specifier,
            modifier_invocations,
        })
    }

    fn constructor_visibility(
        attributes: &input::ConstructorAttributes,
    ) -> output::FunctionVisibility {
        attributes.elements.iter().fold(
            output::FunctionVisibility::Public,
            |visibility, attribute| match attribute {
                input::ConstructorAttribute::InternalKeyword(_) => {
                    output::FunctionVisibility::Internal
                }
                input::ConstructorAttribute::PublicKeyword(_) => output::FunctionVisibility::Public,
                _ => visibility,
            },
        )
    }

    fn constructor_mutability(
        attributes: &input::ConstructorAttributes,
    ) -> output::FunctionMutability {
        attributes.elements.iter().fold(
            output::FunctionMutability::NonPayable,
            |mutability, attribute| match attribute {
                input::ConstructorAttribute::PayableKeyword(_) => {
                    output::FunctionMutability::Payable
                }
                _ => mutability,
            },
        )
    }

    fn constructor_modifier_invocations(
        &mut self,
        attributes: &input::ConstructorAttributes,
    ) -> output::ModifierInvocations {
        attributes
            .elements
            .iter()
            .filter_map(|attribute| {
                if let input::ConstructorAttribute::ModifierInvocation(mi) = attribute {
                    Some(self.build_modifier_invocation(mi))
                } else {
                    None
                }
            })
            .collect()
    }

    fn build_fallback_as_function(
        &mut self,
        source: &input::FallbackFunctionDefinition,
    ) -> output::FunctionDefinition {
        let kind = output::FunctionKind::Fallback;
        let name = None;
        // TODO(validation): fallback functions *must* have external visibility
        let visibility = output::FunctionVisibility::External;
        let mutability = Self::fallback_function_mutability(&source.attributes);
        let virtual_keyword = source.attributes.elements.iter().any(|attribute| {
            matches!(
                attribute,
                input::FallbackFunctionAttribute::VirtualKeyword(_)
            )
        });
        let override_specifier = self.fallback_function_override_specifier(&source.attributes);
        let modifier_invocations = self.fallback_function_modifier_invocations(&source.attributes);
        let body = self.build_function_body(&source.body);
        let parameters = self.build_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.build_returns_declaration(returns));

        Rc::new(output::FunctionDefinitionStruct {
            parameters,
            returns,
            kind,
            name,
            body,
            visibility,
            mutability,
            virtual_keyword,
            override_specifier,
            modifier_invocations,
        })
    }

    fn fallback_function_mutability(
        attributes: &input::FallbackFunctionAttributes,
    ) -> output::FunctionMutability {
        attributes.elements.iter().fold(
            output::FunctionMutability::NonPayable,
            |mutability, attribute| match attribute {
                input::FallbackFunctionAttribute::PayableKeyword(_) => {
                    output::FunctionMutability::Payable
                }
                input::FallbackFunctionAttribute::PureKeyword(_) => {
                    output::FunctionMutability::Pure
                }
                input::FallbackFunctionAttribute::ViewKeyword(_) => {
                    output::FunctionMutability::View
                }
                _ => mutability,
            },
        )
    }

    fn fallback_function_override_specifier(
        &mut self,
        attributes: &input::FallbackFunctionAttributes,
    ) -> Option<output::OverridePaths> {
        attributes.elements.iter().find_map(|attribute| {
            if let input::FallbackFunctionAttribute::OverrideSpecifier(specifier) = attribute {
                Some(self.build_override_specifier_as_paths(specifier))
            } else {
                None
            }
        })
    }

    fn fallback_function_modifier_invocations(
        &mut self,
        attributes: &input::FallbackFunctionAttributes,
    ) -> output::ModifierInvocations {
        attributes
            .elements
            .iter()
            .filter_map(|attribute| {
                if let input::FallbackFunctionAttribute::ModifierInvocation(mi) = attribute {
                    Some(self.build_modifier_invocation(mi))
                } else {
                    None
                }
            })
            .collect()
    }

    fn build_receive_as_function(
        &mut self,
        source: &input::ReceiveFunctionDefinition,
    ) -> output::FunctionDefinition {
        let kind = output::FunctionKind::Receive;
        let name = None;
        // TODO(validation): receive functions *must* have external visibility
        let visibility = output::FunctionVisibility::External;
        // TODO(validation): receive functions *must* be payable
        let mutability = output::FunctionMutability::Payable;
        let virtual_keyword = source.attributes.elements.iter().any(|attribute| {
            matches!(
                attribute,
                input::ReceiveFunctionAttribute::VirtualKeyword(_)
            )
        });
        let override_specifier = self.receive_function_override_specifier(&source.attributes);
        let modifier_invocations = self.receive_function_modifier_invocations(&source.attributes);
        let body = self.build_function_body(&source.body);
        let parameters = self.build_parameters_declaration(&source.parameters);
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            parameters,
            returns,
            kind,
            name,
            body,
            visibility,
            mutability,
            virtual_keyword,
            override_specifier,
            modifier_invocations,
        })
    }

    fn receive_function_override_specifier(
        &mut self,
        attributes: &input::ReceiveFunctionAttributes,
    ) -> Option<output::OverridePaths> {
        attributes.elements.iter().find_map(|attribute| {
            if let input::ReceiveFunctionAttribute::OverrideSpecifier(specifier) = attribute {
                Some(self.build_override_specifier_as_paths(specifier))
            } else {
                None
            }
        })
    }

    fn receive_function_modifier_invocations(
        &mut self,
        attributes: &input::ReceiveFunctionAttributes,
    ) -> output::ModifierInvocations {
        attributes
            .elements
            .iter()
            .filter_map(|attribute| {
                if let input::ReceiveFunctionAttribute::ModifierInvocation(invocation) = attribute {
                    Some(self.build_modifier_invocation(invocation))
                } else {
                    None
                }
            })
            .collect()
    }

    fn build_modifier_as_function(
        &mut self,
        source: &input::ModifierDefinition,
    ) -> output::FunctionDefinition {
        let kind = output::FunctionKind::Modifier;
        let name = Some(self.build_identifier(&source.name));
        let visibility = output::FunctionVisibility::Internal;
        let mutability = output::FunctionMutability::NonPayable;
        let virtual_keyword = source
            .attributes
            .elements
            .iter()
            .any(|attribute| matches!(attribute, input::ModifierAttribute::VirtualKeyword(_)));
        let override_specifier = self.modifier_override_specifier(&source.attributes);
        let modifier_invocations = Vec::new();
        let body = self.build_function_body(&source.body);
        let parameters = source.parameters.as_ref().map_or(Vec::new(), |parameter| {
            self.build_parameters_declaration(parameter)
        });
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            parameters,
            returns,
            kind,
            name,
            body,
            visibility,
            mutability,
            virtual_keyword,
            override_specifier,
            modifier_invocations,
        })
    }

    fn modifier_override_specifier(
        &mut self,
        attributes: &input::ModifierAttributes,
    ) -> Option<output::OverridePaths> {
        attributes.elements.iter().find_map(|attribute| {
            if let input::ModifierAttribute::OverrideSpecifier(specifier) = attribute {
                Some(self.build_override_specifier_as_paths(specifier))
            } else {
                None
            }
        })
    }

    //
    // State variable helpers
    //

    fn state_variable_visibility(
        attributes: &input::StateVariableAttributes,
    ) -> output::StateVariableVisibility {
        // TODO(validation): only one visibility keyword is allowed
        attributes.elements.iter().fold(
            output::StateVariableVisibility::Internal,
            |visibility, attribute| match attribute {
                input::StateVariableAttribute::InternalKeyword(_) => {
                    output::StateVariableVisibility::Internal
                }
                input::StateVariableAttribute::PrivateKeyword(_) => {
                    output::StateVariableVisibility::Private
                }
                input::StateVariableAttribute::PublicKeyword(_) => {
                    output::StateVariableVisibility::Public
                }
                _ => visibility,
            },
        )
    }

    fn state_variable_mutability(
        attributes: &input::StateVariableAttributes,
    ) -> output::StateVariableMutability {
        // TODO(validation): only one mutability keyword is allowed
        attributes.elements.iter().fold(
            output::StateVariableMutability::Mutable,
            |mutability, attribute| match attribute {
                input::StateVariableAttribute::ConstantKeyword(_) => {
                    output::StateVariableMutability::Constant
                }
                input::StateVariableAttribute::ImmutableKeyword(_) => {
                    output::StateVariableMutability::Immutable
                }
                input::StateVariableAttribute::TransientKeyword(_) => {
                    output::StateVariableMutability::Transient
                }
                _ => mutability,
            },
        )
    }

    fn state_variable_override_specifier(
        &mut self,
        attributes: &input::StateVariableAttributes,
    ) -> Option<output::OverridePaths> {
        // TODO(validation): only one override specifier is allowed
        attributes.elements.iter().find_map(|attribute| {
            if let input::StateVariableAttribute::OverrideSpecifier(specifier) = attribute {
                Some(self.build_override_specifier_as_paths(specifier))
            } else {
                None
            }
        })
    }

    fn state_variable_as_constant_definition(
        state_variable_definition: output::StateVariableDefinitionStruct,
    ) -> output::ConstantDefinition {
        Rc::new(output::ConstantDefinitionStruct {
            type_name: state_variable_definition.type_name,
            name: state_variable_definition.name,
            visibility: Some(state_variable_definition.visibility),
            value: state_variable_definition.value,
        })
    }

    //
    // Override specifier helper
    //

    fn build_override_specifier_as_paths(
        &mut self,
        source: &input::OverrideSpecifier,
    ) -> output::OverridePaths {
        source
            .overridden
            .as_ref()
            .map_or(Vec::new(), |declaration| {
                self.build_override_paths_declaration(declaration)
            })
    }

    //
    // Mapping helpers
    //

    fn build_mapping_key_as_parameter(&mut self, source: &input::MappingKey) -> output::Parameter {
        let type_name = self.build_mapping_key_type(&source.key_type);
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Rc::new(output::ParameterStruct {
            type_name,
            storage_location: None,
            name,
            indexed: false,
        })
    }

    fn build_mapping_key_type(&mut self, source: &input::MappingKeyType) -> output::TypeName {
        match source {
            input::MappingKeyType::ElementaryType(elementary_type) => {
                output::TypeName::ElementaryType(self.build_elementary_type(elementary_type))
            }
            input::MappingKeyType::IdentifierPath(identifier_path) => {
                output::TypeName::IdentifierPath(self.build_identifier_path(identifier_path))
            }
        }
    }

    fn build_mapping_value_as_parameter(
        &mut self,
        source: &input::MappingValue,
    ) -> output::Parameter {
        let type_name = self.build_type_name(&source.type_name);
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Rc::new(output::ParameterStruct {
            type_name,
            storage_location: None,
            name,
            indexed: false,
        })
    }

    //
    // Import helpers
    //

    fn build_named_import_as_path_import(
        &mut self,
        source: &input::NamedImport,
    ) -> output::PathImport {
        let path = self.build_string_literal(&source.path);
        let alias = Some(self.build_import_alias(&source.alias));

        Rc::new(output::PathImportStruct { path, alias })
    }

    //
    // Event/Error parameter helpers
    //

    fn build_event_parameter(&mut self, source: &input::EventParameter) -> output::Parameter {
        let type_name = self.build_type_name(&source.type_name);
        let indexed = source.indexed_keyword.is_some();
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Rc::new(output::ParameterStruct {
            type_name,
            storage_location: None,
            name,
            indexed,
        })
    }

    fn build_error_parameter(&mut self, source: &input::ErrorParameter) -> output::Parameter {
        let type_name = self.build_type_name(&source.type_name);
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Rc::new(output::ParameterStruct {
            type_name,
            storage_location: None,
            name,
            indexed: false,
        })
    }
}
