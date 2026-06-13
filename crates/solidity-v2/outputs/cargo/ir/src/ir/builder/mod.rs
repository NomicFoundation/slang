use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::syntax::{
    MultipleMutabilitySpecifiers, MultipleOverrideSpecifiers, MultipleVirtualSpecifiers,
};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_cst::structured_cst::nodes as input;
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

#[path = "default.generated.rs"]
mod default;

use super::Source;
use crate::ir::nodes as output;

/// A strictly monotonically increasing `NodeId` generator.
pub struct NodeIdGenerator {
    next_id: usize,
}

impl NodeIdGenerator {
    /// Returns a `NodeId` greater than any previously returned by this
    /// generator.
    /// The returned ID is unique and suitable for use as a total-order key.
    pub fn next_id(&mut self) -> output::NodeId {
        let id = self.next_id;
        self.next_id += 1;
        id.into()
    }
}

impl Default for NodeIdGenerator {
    fn default() -> Self {
        Self { next_id: 1usize }
    }
}

/// The output of a CST → IR build operation, containing both the built IR
/// source unit and any diagnostics emitted during the build.
pub struct BuildOutput {
    pub ir_root: output::SourceUnit,
    pub diagnostics: DiagnosticCollection,
}

pub fn build_source_unit(
    file_id: &str,
    source_unit: &input::SourceUnit,
    source: &impl Source,
    id_generator: &mut NodeIdGenerator,
) -> BuildOutput {
    let mut builder = CstToIrBuilder {
        source,
        file_id,
        id_generator,
        diagnostics: DiagnosticCollection::default(),
    };

    let ir_root = builder.build_source_unit(source_unit);

    BuildOutput {
        ir_root,
        diagnostics: builder.diagnostics,
    }
}

pub(crate) struct CstToIrBuilder<'a, S: Source> {
    source: &'a S,
    file_id: &'a str,
    id_generator: &'a mut NodeIdGenerator,
    diagnostics: DiagnosticCollection,
}

impl<S: Source> CstToIrBuilder<'_, S> {
    fn next_id(&mut self) -> output::NodeId {
        self.id_generator.next_id()
    }

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
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let name = self.build_identifier(&source.name);
        let visibility = None;
        let value = Some(self.build_expression(&source.value));

        Arc::new(output::ConstantDefinitionStruct {
            id,
            range,
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
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let is_abstract = source.abstract_keyword.is_some();
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

        Arc::new(output::ContractDefinitionStruct {
            id,
            range,
            is_abstract,
            name,
            inheritance_types,
            storage_layout,
            members,
        })
    }

    fn build_error_definition(
        &mut self,
        source: &input::ErrorDefinition,
    ) -> output::ErrorDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let parameters = source
            .members
            .parameters
            .elements
            .iter()
            .map(|parameter| self.build_error_parameter(parameter))
            .collect();

        Arc::new(output::ErrorDefinitionStruct {
            id,
            range,
            name,
            parameters,
        })
    }

    fn build_event_definition(
        &mut self,
        source: &input::EventDefinition,
    ) -> output::EventDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let is_anonymous = source.anonymous_keyword.is_some();
        let parameters = source
            .parameters
            .parameters
            .elements
            .iter()
            .map(|parameter| self.build_event_parameter(parameter))
            .collect();

        Arc::new(output::EventDefinitionStruct {
            id,
            range,
            name,
            is_anonymous,
            parameters,
        })
    }

    fn build_function_definition(
        &mut self,
        source: &input::FunctionDefinition,
    ) -> output::FunctionDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let (kind, name) = match &source.name {
            input::FunctionName::Identifier(identifier) => (
                output::FunctionKind::Regular,
                Some(self.build_identifier(identifier)),
            ),
            input::FunctionName::FallbackKeyword(_) => (output::FunctionKind::Fallback, None),
            input::FunctionName::ReceiveKeyword(_) => (output::FunctionKind::Receive, None),
        };
        let parameters = self.build_parameters_declaration(&source.parameters);
        let visibility = Self::function_visibility(&source.attributes);
        let mutability = self
            .extract_mutability_specifier(&source.attributes.elements)
            .unwrap_or(output::FunctionMutability::NonPayable);

        let is_virtual =
            self.extract_first_virtual(source.attributes.elements.iter().filter_map(|attribute| {
                if let input::FunctionAttribute::VirtualKeyword(virtual_keyword) = attribute {
                    Some(virtual_keyword)
                } else {
                    None
                }
            }));
        let override_specifier = self.function_override_specifier(&source.attributes);
        let modifier_invocations = self.function_modifier_invocations(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.build_returns_declaration(returns));
        let body = self.build_function_body(&source.body);

        Arc::new(output::FunctionDefinitionStruct {
            id,
            range,
            kind,
            name,
            parameters,
            visibility,
            mutability,
            is_virtual,
            override_specifier,
            modifier_invocations,
            returns,
            body,
        })
    }

    fn build_function_type(&mut self, source: &input::FunctionType) -> output::FunctionType {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let parameters = self.build_parameters_declaration(&source.parameters);
        let visibility = Self::function_type_visibility(&source.attributes);
        let mutability = self
            .extract_mutability_specifier(&source.attributes.elements)
            .unwrap_or(output::FunctionMutability::NonPayable);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.build_returns_declaration(returns));

        Arc::new(output::FunctionTypeStruct {
            id,
            range,
            parameters,
            visibility,
            mutability,
            returns,
        })
    }

    fn build_index_access_expression(
        &mut self,
        source: &input::IndexAccessExpression,
    ) -> output::IndexAccessExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let start = source
            .start
            .as_ref()
            .map(|start| self.build_expression(start));
        let end = source
            .end
            .as_ref()
            .and_then(|end| end.end.as_ref().map(|end| self.build_expression(end)));

        Arc::new(output::IndexAccessExpressionStruct {
            id,
            range,
            operand,
            start,
            end,
        })
    }

    fn build_mapping_type(&mut self, source: &input::MappingType) -> output::MappingType {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let key_type = self.build_mapping_key_as_parameter(&source.key_type);
        let value_type = self.build_mapping_value_as_parameter(&source.value_type);

        Arc::new(output::MappingTypeStruct {
            id,
            range,
            key_type,
            value_type,
        })
    }

    fn build_parameter(&mut self, source: &input::Parameter) -> output::Parameter {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|location| self.build_storage_location(location));
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Arc::new(output::ParameterStruct {
            id,
            range,
            type_name,
            storage_location,
            name,
            is_indexed: false,
        })
    }

    fn build_state_variable_definition(
        &mut self,
        source: &input::StateVariableDefinition,
    ) -> output::StateVariableDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let name = self.build_identifier(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.build_state_variable_definition_value(value));
        let visibility = Self::state_variable_visibility(&source.attributes);
        let mutability = self
            .extract_mutability_specifier(&source.attributes.elements)
            .unwrap_or(output::StateVariableMutability::Mutable);
        let override_specifier = self.state_variable_override_specifier(&source.attributes);

        Arc::new(output::StateVariableDefinitionStruct {
            id,
            range,
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
                            Arc::into_inner(output).expect("Created node is not yet shared"),
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
        // TODO(validation) SDR[13]: only a single visibility keyword can be provided
        // TODO(validation) SDR[16]: free functions are always internal, but
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

    fn function_override_specifier(
        &mut self,
        attributes: &input::FunctionAttributes,
    ) -> Option<output::OverridePaths> {
        self.extract_single_override(attributes.elements.iter().filter_map(|attribute| {
            if let input::FunctionAttribute::OverrideSpecifier(specifier) = attribute {
                Some(specifier)
            } else {
                None
            }
        }))
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
        // TODO(validation) SDR[13]: only a single visibility keyword can be provided
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

    //
    // Constructor / Fallback / Receive / Modifier → FunctionDefinition
    //

    fn build_constructor_as_function(
        &mut self,
        source: &input::ConstructorDefinition,
    ) -> output::FunctionDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let kind = output::FunctionKind::Constructor;
        let name = None;
        let parameters = self.build_parameters_declaration(&source.parameters);
        let visibility = Self::constructor_visibility(&source.attributes);
        let mutability = self
            .extract_mutability_specifier(&source.attributes.elements)
            .unwrap_or(output::FunctionMutability::NonPayable);
        // v2 ConstructorAttribute has no VirtualKeyword
        let is_virtual = false;
        // v2 ConstructorAttribute has no OverrideSpecifier
        let override_specifier = None;
        let modifier_invocations = self.constructor_modifier_invocations(&source.attributes);
        let returns = None;
        let body = Some(self.build_block(&source.body));

        Arc::new(output::FunctionDefinitionStruct {
            id,
            range,
            kind,
            name,
            parameters,
            visibility,
            mutability,
            is_virtual,
            override_specifier,
            modifier_invocations,
            returns,
            body,
        })
    }

    fn constructor_visibility(
        attributes: &input::ConstructorAttributes,
    ) -> output::FunctionVisibility {
        // TODO(validation) SDR[13]: only a single visibility keyword can be provided
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
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let kind = output::FunctionKind::Fallback;
        let name = None;
        let parameters = self.build_parameters_declaration(&source.parameters);
        // TODO(validation) SDR[14]: fallback functions *must* have external visibility
        let visibility = output::FunctionVisibility::External;
        let mutability = self
            .extract_mutability_specifier(&source.attributes.elements)
            .unwrap_or(output::FunctionMutability::NonPayable);
        let is_virtual =
            self.extract_first_virtual(source.attributes.elements.iter().filter_map(|attribute| {
                if let input::FallbackFunctionAttribute::VirtualKeyword(virtual_keyword) = attribute
                {
                    Some(virtual_keyword)
                } else {
                    None
                }
            }));

        let override_specifier = self.fallback_function_override_specifier(&source.attributes);
        let modifier_invocations = self.fallback_function_modifier_invocations(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.build_returns_declaration(returns));
        let body = self.build_function_body(&source.body);

        Arc::new(output::FunctionDefinitionStruct {
            id,
            range,
            kind,
            name,
            parameters,
            visibility,
            mutability,
            is_virtual,
            override_specifier,
            modifier_invocations,
            returns,
            body,
        })
    }

    fn fallback_function_override_specifier(
        &mut self,
        attributes: &input::FallbackFunctionAttributes,
    ) -> Option<output::OverridePaths> {
        self.extract_single_override(attributes.elements.iter().filter_map(|attribute| {
            if let input::FallbackFunctionAttribute::OverrideSpecifier(specifier) = attribute {
                Some(specifier)
            } else {
                None
            }
        }))
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
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let kind = output::FunctionKind::Receive;
        let name = None;
        let parameters = self.build_parameters_declaration(&source.parameters);
        // TODO(validation) SDR[8]: receive functions *must* have external visibility
        let visibility = output::FunctionVisibility::External;
        // TODO(validation) SDR[7]: receive functions *must* have a 'payable' specifier
        let mutability = self
            .extract_mutability_specifier(&source.attributes.elements)
            .unwrap_or(output::FunctionMutability::Payable);
        let is_virtual =
            self.extract_first_virtual(source.attributes.elements.iter().filter_map(|attribute| {
                if let input::ReceiveFunctionAttribute::VirtualKeyword(virtual_keyword) = attribute
                {
                    Some(virtual_keyword)
                } else {
                    None
                }
            }));
        let override_specifier = self.receive_function_override_specifier(&source.attributes);
        let modifier_invocations = self.receive_function_modifier_invocations(&source.attributes);
        let returns = None;
        let body = self.build_function_body(&source.body);

        Arc::new(output::FunctionDefinitionStruct {
            id,
            range,
            kind,
            name,
            parameters,
            visibility,
            mutability,
            is_virtual,
            override_specifier,
            modifier_invocations,
            returns,
            body,
        })
    }

    fn receive_function_override_specifier(
        &mut self,
        attributes: &input::ReceiveFunctionAttributes,
    ) -> Option<output::OverridePaths> {
        self.extract_single_override(attributes.elements.iter().filter_map(|attribute| {
            if let input::ReceiveFunctionAttribute::OverrideSpecifier(specifier) = attribute {
                Some(specifier)
            } else {
                None
            }
        }))
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
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let kind = output::FunctionKind::Modifier;
        let name = Some(self.build_identifier(&source.name));
        let parameters = source.parameters.as_ref().map_or(Vec::new(), |parameter| {
            self.build_parameters_declaration(parameter)
        });
        let visibility = output::FunctionVisibility::Internal;
        // mutability is irrelevant for modifiers
        let mutability = output::FunctionMutability::NonPayable;
        let is_virtual =
            self.extract_first_virtual(source.attributes.elements.iter().filter_map(|attribute| {
                if let input::ModifierAttribute::VirtualKeyword(virtual_keyword) = attribute {
                    Some(virtual_keyword)
                } else {
                    None
                }
            }));
        let override_specifier = self.modifier_override_specifier(&source.attributes);
        let modifier_invocations = Vec::new();
        let returns = None;
        let body = self.build_function_body(&source.body);

        Arc::new(output::FunctionDefinitionStruct {
            id,
            range,
            kind,
            name,
            parameters,
            visibility,
            mutability,
            is_virtual,
            override_specifier,
            modifier_invocations,
            returns,
            body,
        })
    }

    fn modifier_override_specifier(
        &mut self,
        attributes: &input::ModifierAttributes,
    ) -> Option<output::OverridePaths> {
        self.extract_single_override(attributes.elements.iter().filter_map(|attribute| {
            if let input::ModifierAttribute::OverrideSpecifier(specifier) = attribute {
                Some(specifier)
            } else {
                None
            }
        }))
    }

    //
    // State variable helpers
    //

    fn state_variable_visibility(
        attributes: &input::StateVariableAttributes,
    ) -> output::StateVariableVisibility {
        // TODO(validation) SDR[10]: only one visibility keyword is allowed
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

    fn state_variable_override_specifier(
        &mut self,
        attributes: &input::StateVariableAttributes,
    ) -> Option<output::OverridePaths> {
        self.extract_single_override(attributes.elements.iter().filter_map(|attribute| {
            if let input::StateVariableAttribute::OverrideSpecifier(specifier) = attribute {
                Some(specifier)
            } else {
                None
            }
        }))
    }

    fn state_variable_as_constant_definition(
        state_variable_definition: output::StateVariableDefinitionStruct,
    ) -> output::ConstantDefinition {
        Arc::new(output::ConstantDefinitionStruct {
            id: state_variable_definition.id(),
            range: state_variable_definition.range,
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
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_mapping_key_type(&source.key_type);
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Arc::new(output::ParameterStruct {
            id,
            range,
            type_name,
            storage_location: None,
            name,
            is_indexed: false,
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
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Arc::new(output::ParameterStruct {
            id,
            range,
            type_name,
            storage_location: None,
            name,
            is_indexed: false,
        })
    }

    //
    // Import helpers
    //

    fn build_named_import_as_path_import(
        &mut self,
        source: &input::NamedImport,
    ) -> output::PathImport {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let path = self.build_string_literal(&source.path);
        let alias = Some(self.build_import_alias(&source.alias));

        Arc::new(output::PathImportStruct {
            id,
            range,
            path,
            alias,
        })
    }

    //
    // Event/Error parameter helpers
    //

    fn build_event_parameter(&mut self, source: &input::EventParameter) -> output::Parameter {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let is_indexed = source.indexed_keyword.is_some();
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Arc::new(output::ParameterStruct {
            id,
            range,
            type_name,
            storage_location: None,
            name,
            is_indexed,
        })
    }

    fn build_error_parameter(&mut self, source: &input::ErrorParameter) -> output::Parameter {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let name = source.name.as_ref().map(|name| self.build_identifier(name));

        Arc::new(output::ParameterStruct {
            id,
            range,
            type_name,
            storage_location: None,
            name,
            is_indexed: false,
        })
    }

    fn extract_mutability_specifier<'a, Input, Output>(
        &mut self,
        attributes: impl IntoIterator<Item = &'a Input>,
    ) -> Option<Output>
    where
        Input: TextRange + 'a,
        &'a Input: TryInto<Output>,
    {
        let mut result: Option<Output> = None;

        for attribute in attributes {
            if let Ok(current) = attribute.try_into() {
                if result.is_some() {
                    self.diagnostics.push(
                        self.file_id.to_owned(),
                        attribute.calculate_text_range().unwrap(),
                        MultipleMutabilitySpecifiers,
                    );
                } else {
                    result = Some(current);
                }
            }
        }

        result
    }

    /// Returns whether a `virtual` specifier is present, emitting an error for
    /// any subsequent ones.
    fn extract_first_virtual<'a>(
        &mut self,
        virtual_kws: impl IntoIterator<Item = &'a input::VirtualKeyword>,
    ) -> bool {
        let mut virtual_kws = virtual_kws.into_iter();
        let Some(_first) = virtual_kws.next() else {
            return false;
        };

        for keyword in virtual_kws {
            self.diagnostics.push(
                self.file_id.to_owned(),
                keyword.calculate_text_range().unwrap(),
                MultipleVirtualSpecifiers,
            );
        }

        true
    }

    /// Extracts and transforms the first `override` specifier, emitting an
    /// error for any subsequent ones.
    fn extract_single_override<'a>(
        &mut self,
        specifiers: impl IntoIterator<Item = &'a input::OverrideSpecifier>,
    ) -> Option<output::OverridePaths> {
        let mut specifiers = specifiers.into_iter();
        let first = self.build_override_specifier_as_paths(specifiers.next()?);

        for specifier in specifiers {
            self.diagnostics.push(
                self.file_id.to_owned(),
                specifier.calculate_text_range().unwrap(),
                MultipleOverrideSpecifiers,
            );
        }

        Some(first)
    }
}
