use std::collections::HashMap;
use std::rc::Rc;

use semver::Version;

use super::p0_build_ast::Output as Input;
use crate::backend::ir::ir2_flat_contracts::transformer::Transformer;
use crate::backend::ir::ir2_flat_contracts::{self as output, input, SourceUnit};
use crate::compilation::CompilationUnit;
use crate::cst::TerminalNode;
use crate::utils::versions::VERSION_0_5_0;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, SourceUnit>,
}

/// This pass is reserved to make ergonomic changes to the AST in order to make
/// it easier to use. For now, it will only flatten contract specifiers:
/// inheritance and storage layout specifiers. In the future, more
/// transformations will be added.
pub fn run(input: Input) -> Output {
    let mut pass = Pass {
        language_version: input.compilation_unit.language_version().clone(),
    };
    let files = input
        .files
        .iter()
        .map(|(file_id, source_unit)| (file_id.clone(), pass.transform_source_unit(source_unit)))
        .collect();
    let compilation_unit = input.compilation_unit;
    Output {
        compilation_unit,
        files,
    }
}

struct Pass {
    language_version: Version,
}

impl Transformer for Pass {
    fn transform_source_unit_member(
        &mut self,
        source: &input::SourceUnitMember,
    ) -> output::SourceUnitMember {
        match source {
            input::SourceUnitMember::ImportDirective(import_directive) => {
                output::SourceUnitMember::ImportClause(
                    self.transform_import_clause(&import_directive.clause),
                )
            }
            _ => self.default_transform_source_unit_member(source),
        }
    }

    fn transform_import_clause(&mut self, source: &input::ImportClause) -> output::ImportClause {
        match source {
            input::ImportClause::NamedImport(named_import) => {
                output::ImportClause::PathImport(self.transform_named_import(named_import))
            }
            _ => self.default_transform_import_clause(source),
        }
    }

    fn transform_contract_definition(
        &mut self,
        source: &input::ContractDefinition,
    ) -> output::ContractDefinition {
        let node_id = source.node_id;
        let abstract_keyword = source.abstract_keyword;
        let name = Rc::clone(&source.name);
        let members = self.transform_contract_members(&source.members);
        let inheritance_types = source
            .specifiers
            .iter()
            .find_map(|specifier| {
                if let input::ContractSpecifier::InheritanceSpecifier(inheritance) = specifier {
                    Some(self.transform_inheritance_types(&inheritance.types))
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let storage_layout = source.specifiers.iter().find_map(|specifier| {
            if let input::ContractSpecifier::StorageLayoutSpecifier(storage_layout) = specifier {
                Some(self.transform_storage_layout_specifier(storage_layout))
            } else {
                None
            }
        });

        Rc::new(output::ContractDefinitionStruct {
            node_id,
            abstract_keyword,
            name,
            members,
            inheritance_types,
            storage_layout,
        })
    }

    fn transform_function_definition(
        &mut self,
        source: &input::FunctionDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let (name, kind) = match &source.name {
            input::FunctionName::Identifier(identifier) => {
                (Some(Rc::clone(identifier)), output::FunctionKind::Regular)
            }
            input::FunctionName::FallbackKeyword => (None, output::FunctionKind::Fallback),
            input::FunctionName::ReceiveKeyword => (None, output::FunctionKind::Receive),
        };
        let visibility = self.function_visibility(&source.attributes);
        let mutability = Self::function_mutability(&source.attributes);
        let virtual_keyword = source
            .attributes
            .iter()
            .any(|attribute| matches!(attribute, input::FunctionAttribute::VirtualKeyword));
        // TODO(validation): function definitions can have only a single override specifier
        let override_specifier = self.function_override_specifier(&source.attributes);
        let modifier_invocations = self.function_modifier_invocations(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_returns_declaration(returns));

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
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

    fn transform_contract_member(
        &mut self,
        source: &input::ContractMember,
    ) -> output::ContractMember {
        match source {
            input::ContractMember::ConstructorDefinition(constructor_definition) => {
                output::ContractMember::FunctionDefinition(
                    self.transform_constructor_definition(constructor_definition),
                )
            }
            input::ContractMember::FallbackFunctionDefinition(fallback_function_definition) => {
                output::ContractMember::FunctionDefinition(
                    self.transform_fallback_function_definition(fallback_function_definition),
                )
            }
            input::ContractMember::ReceiveFunctionDefinition(receive_function_definition) => {
                output::ContractMember::FunctionDefinition(
                    self.transform_receive_function_definition(receive_function_definition),
                )
            }
            input::ContractMember::UnnamedFunctionDefinition(unnamed_function_definition) => {
                output::ContractMember::FunctionDefinition(
                    self.transform_unnamed_function_definition(unnamed_function_definition),
                )
            }
            input::ContractMember::ModifierDefinition(modifier_definition) => {
                output::ContractMember::FunctionDefinition(
                    self.transform_modifier_definition(modifier_definition),
                )
            }
            input::ContractMember::StateVariableDefinition(state_variable_definition) => {
                let output = self.transform_state_variable_definition(state_variable_definition);
                if matches!(output.mutability, output::StateVariableMutability::Constant)
                    && !matches!(output.visibility, output::StateVariableVisibility::Public)
                {
                    // Transmute the `StateVariableDefinition` into a `ConstantDefinition`
                    output::ContractMember::ConstantDefinition(
                        Self::state_variable_definition_as_constant_definition(
                            Rc::into_inner(output).expect("Created node is not yet shared"),
                        ),
                    )
                } else {
                    output::ContractMember::StateVariableDefinition(output)
                }
            }
            _ => self.default_transform_contract_member(source),
        }
    }

    fn transform_function_type(&mut self, source: &input::FunctionType) -> output::FunctionType {
        let node_id = source.node_id;
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_returns_declaration(returns));
        let visibility = Self::function_type_visibility(&source.attributes);
        let mutability = Self::function_type_mutability(&source.attributes);

        Rc::new(output::FunctionTypeStruct {
            node_id,
            parameters,
            returns,
            visibility,
            mutability,
        })
    }

    fn transform_index_access_expression(
        &mut self,
        source: &input::IndexAccessExpression,
    ) -> output::IndexAccessExpression {
        let node_id = source.node_id;
        let operand = self.transform_expression(&source.operand);
        let start = source
            .start
            .as_ref()
            .map(|start| self.transform_expression(start));
        let end = source
            .end
            .as_ref()
            .and_then(|end| end.end.as_ref().map(|end| self.transform_expression(end)));

        Rc::new(output::IndexAccessExpressionStruct {
            node_id,
            operand,
            start,
            end,
        })
    }

    fn transform_arguments_declaration(
        &mut self,
        source: &input::ArgumentsDeclaration,
    ) -> output::ArgumentsDeclaration {
        match source {
            input::ArgumentsDeclaration::PositionalArgumentsDeclaration(
                positional_arguments_declaration,
            ) => output::ArgumentsDeclaration::PositionalArguments(
                self.transform_positional_arguments(&positional_arguments_declaration.arguments),
            ),
            input::ArgumentsDeclaration::NamedArgumentsDeclaration(named_arguments_declaration) => {
                output::ArgumentsDeclaration::NamedArguments(
                    named_arguments_declaration.arguments.as_ref().map_or(
                        Vec::new(),
                        |named_argument_group| {
                            self.transform_named_argument_group(named_argument_group)
                        },
                    ),
                )
            }
        }
    }

    fn transform_constant_definition(
        &mut self,
        source: &input::ConstantDefinition,
    ) -> output::ConstantDefinition {
        let node_id = source.node_id;
        let type_name = self.transform_type_name(&source.type_name);
        let name = Rc::clone(&source.name);
        let visibility = None;
        let value = Some(self.transform_expression(&source.value));

        Rc::new(output::ConstantDefinitionStruct {
            node_id,
            type_name,
            name,
            visibility,
            value,
        })
    }

    fn transform_state_variable_definition(
        &mut self,
        source: &input::StateVariableDefinition,
    ) -> output::StateVariableDefinition {
        let node_id = source.node_id;
        let type_name = self.transform_type_name(&source.type_name);
        let name = Rc::clone(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.transform_state_variable_definition_value(value));
        let visibility = Self::state_variable_visibility(&source.attributes);
        let mutability = Self::state_variable_mutability(&source.attributes);
        let override_specifier = self.state_variable_override_specifier(&source.attributes);

        Rc::new(output::StateVariableDefinitionStruct {
            node_id,
            type_name,
            name,
            value,
            visibility,
            mutability,
            override_specifier,
        })
    }

    fn transform_string_expression(
        &mut self,
        source: &input::StringExpression,
    ) -> output::StringExpression {
        match source {
            input::StringExpression::StringLiteral(string_literal) => {
                output::StringExpression::Strings(vec![Self::string_literal_terminal_node(
                    string_literal,
                )])
            }
            input::StringExpression::StringLiterals(string_literals) => {
                output::StringExpression::Strings(
                    string_literals
                        .iter()
                        .map(Self::string_literal_terminal_node)
                        .collect(),
                )
            }
            input::StringExpression::HexStringLiteral(hex_string_literal) => {
                output::StringExpression::HexStrings(vec![Self::hex_string_literal_terminal_node(
                    hex_string_literal,
                )])
            }
            input::StringExpression::HexStringLiterals(hex_string_literals) => {
                output::StringExpression::HexStrings(
                    hex_string_literals
                        .iter()
                        .map(Self::hex_string_literal_terminal_node)
                        .collect(),
                )
            }
            input::StringExpression::UnicodeStringLiterals(unicode_string_literals) => {
                output::StringExpression::UnicodeStrings(
                    unicode_string_literals
                        .iter()
                        .map(Self::unicode_string_literal_terminal_node)
                        .collect(),
                )
            }
        }
    }

    fn transform_path_import(&mut self, source: &input::PathImport) -> output::PathImport {
        let node_id = source.node_id;
        let alias = source
            .alias
            .as_ref()
            .map(|alias| self.transform_import_alias(alias));
        let path = Self::string_literal_terminal_node(&source.path);

        Rc::new(output::PathImportStruct {
            node_id,
            alias,
            path,
        })
    }

    fn transform_import_deconstruction(
        &mut self,
        source: &input::ImportDeconstruction,
    ) -> output::ImportDeconstruction {
        let node_id = source.node_id;
        let symbols = self.transform_import_deconstruction_symbols(&source.symbols);
        let path = Self::string_literal_terminal_node(&source.path);

        Rc::new(output::ImportDeconstructionStruct {
            node_id,
            symbols,
            path,
        })
    }

    fn transform_assembly_statement(
        &mut self,
        source: &input::AssemblyStatement,
    ) -> output::AssemblyStatement {
        let node_id = source.node_id;
        let flags = source.flags.as_ref().map_or(Vec::new(), |flags| {
            flags
                .flags
                .iter()
                .map(Self::string_literal_terminal_node)
                .collect()
        });
        let body = self.transform_yul_block(&source.body);
        let label = source
            .label
            .as_ref()
            .map(Self::string_literal_terminal_node);

        Rc::new(output::AssemblyStatementStruct {
            node_id,
            body,
            flags,
            label,
        })
    }

    fn transform_experimental_feature(
        &mut self,
        source: &input::ExperimentalFeature,
    ) -> output::ExperimentalFeature {
        match source {
            input::ExperimentalFeature::StringLiteral(string_literal) => {
                output::ExperimentalFeature::StringLiteral(Self::string_literal_terminal_node(
                    string_literal,
                ))
            }
            _ => self.default_transform_experimental_feature(source),
        }
    }

    fn transform_yul_literal(&mut self, source: &input::YulLiteral) -> output::YulLiteral {
        match source {
            input::YulLiteral::HexStringLiteral(hex_string_literal) => {
                output::YulLiteral::HexStringLiteral(Self::hex_string_literal_terminal_node(
                    hex_string_literal,
                ))
            }
            input::YulLiteral::StringLiteral(string_literal) => output::YulLiteral::StringLiteral(
                Self::string_literal_terminal_node(string_literal),
            ),
            _ => self.default_transform_yul_literal(source),
        }
    }

    fn transform_variable_declaration_statement(
        &mut self,
        source: &input::VariableDeclarationStatement,
    ) -> output::VariableDeclarationStatement {
        let type_name = match &source.variable_type {
            input::VariableDeclarationType::TypeName(type_name) => {
                Some(self.transform_type_name(type_name))
            }
            input::VariableDeclarationType::VarKeyword => None,
        };
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.transform_storage_location(value));
        let name = Rc::clone(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.transform_variable_declaration_value(value));

        Rc::new(output::VariableDeclarationStatementStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
            value,
        })
    }

    fn transform_tuple_deconstruction_statement(
        &mut self,
        source: &input::TupleDeconstructionStatement,
    ) -> output::TupleDeconstructionStatement {
        let expression = self.transform_expression(&source.expression);
        let members =
            self.transform_tuple_deconstruction_elements(&source.elements, source.var_keyword);

        Rc::new(output::TupleDeconstructionStatementStruct {
            node_id: source.node_id,
            expression,
            members,
        })
    }

    fn transform_parameter(&mut self, source: &input::Parameter) -> output::Parameter {
        let type_name = self.transform_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|location| self.transform_storage_location(location));
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::ParameterStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
            indexed: false,
        })
    }

    fn transform_event_definition(
        &mut self,
        source: &input::EventDefinition,
    ) -> output::EventDefinition {
        let name = Rc::clone(&source.name);
        let anonymous_keyword = source.anonymous_keyword;
        let parameters = self.transform_event_parameters(&source.parameters.parameters);

        Rc::new(output::EventDefinitionStruct {
            node_id: source.node_id,
            name,
            anonymous_keyword,
            parameters,
        })
    }

    fn transform_error_definition(
        &mut self,
        source: &input::ErrorDefinition,
    ) -> output::ErrorDefinition {
        let name = Rc::clone(&source.name);
        let parameters = self.transform_error_parameters(&source.members.parameters);

        Rc::new(output::ErrorDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
        })
    }
}

impl Pass {
    fn transform_named_import(&mut self, source: &input::NamedImport) -> output::PathImport {
        let node_id = source.node_id;
        let alias = Some(self.transform_import_alias(&source.alias));
        let path = Self::string_literal_terminal_node(&source.path);

        Rc::new(output::PathImportStruct {
            node_id,
            alias,
            path,
        })
    }

    fn function_visibility(
        &self,
        attributes: &input::FunctionAttributes,
    ) -> output::FunctionVisibility {
        // TODO(validation): only a single visibility keyword can be provided
        // TODO(validation): free functions are always internal, but
        // otherwise a visibility *must* be set explicitly (>= 0.5.0)
        attributes.iter().fold(
            if self.language_version < VERSION_0_5_0 {
                // In Solidity < 0.5.0 free functions are not valid and the
                // default visibility for contract functions is public
                output::FunctionVisibility::Public
            } else {
                // Otherwise the default for free functions is internal, and for
                // contract function you must specify a visibility attribute.
                output::FunctionVisibility::Internal
            },
            |visibility, attribute| match attribute {
                input::FunctionAttribute::ExternalKeyword => output::FunctionVisibility::External,
                input::FunctionAttribute::InternalKeyword => output::FunctionVisibility::Internal,
                input::FunctionAttribute::PrivateKeyword => output::FunctionVisibility::Private,
                input::FunctionAttribute::PublicKeyword => output::FunctionVisibility::Public,
                _ => visibility,
            },
        )
    }

    fn function_mutability(attributes: &input::FunctionAttributes) -> output::FunctionMutability {
        // TODO(validation): only a single mutability keyword can be provided
        attributes.iter().fold(
            output::FunctionMutability::NonPayable,
            |mutability, attribute| match attribute {
                input::FunctionAttribute::PayableKeyword => output::FunctionMutability::Payable,
                input::FunctionAttribute::PureKeyword => output::FunctionMutability::Pure,
                input::FunctionAttribute::ConstantKeyword
                | input::FunctionAttribute::ViewKeyword => output::FunctionMutability::View,
                _ => mutability,
            },
        )
    }

    fn transform_override_specifier_into_override_paths(
        &mut self,
        override_specifier: &input::OverrideSpecifier,
    ) -> output::OverridePaths {
        override_specifier
            .overridden
            .as_ref()
            .map_or(Vec::new(), |overriden| {
                self.transform_override_paths(&overriden.paths)
            })
    }

    fn function_override_specifier(
        &mut self,
        attributes: &input::FunctionAttributes,
    ) -> Option<output::OverridePaths> {
        attributes.iter().find_map(|attribute| {
            if let input::FunctionAttribute::OverrideSpecifier(override_specifier) = attribute {
                Some(self.transform_override_specifier_into_override_paths(override_specifier))
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
            .iter()
            .filter_map(|attribute| {
                if let input::FunctionAttribute::ModifierInvocation(modifier_invocation) = attribute
                {
                    Some(self.transform_modifier_invocation(modifier_invocation))
                } else {
                    None
                }
            })
            .collect()
    }

    fn transform_function_body(&mut self, source: &input::FunctionBody) -> Option<output::Block> {
        match source {
            input::FunctionBody::Block(block) => Some(self.transform_block(block)),
            input::FunctionBody::Semicolon => None,
        }
    }

    fn function_type_visibility(
        attributes: &input::FunctionTypeAttributes,
    ) -> output::FunctionVisibility {
        // TODO(validation): only a single visibility keyword can be provided
        attributes.iter().fold(
            output::FunctionVisibility::Internal,
            |visibility, attribute| match attribute {
                input::FunctionTypeAttribute::ExternalKeyword => {
                    output::FunctionVisibility::External
                }
                input::FunctionTypeAttribute::InternalKeyword => {
                    output::FunctionVisibility::Internal
                }
                input::FunctionTypeAttribute::PrivateKeyword => output::FunctionVisibility::Private,
                input::FunctionTypeAttribute::PublicKeyword => output::FunctionVisibility::Public,
                _ => visibility,
            },
        )
    }

    fn function_type_mutability(
        attributes: &input::FunctionTypeAttributes,
    ) -> output::FunctionMutability {
        // TODO(validation): only a single mutability keyword can be provided
        attributes.iter().fold(
            output::FunctionMutability::NonPayable,
            |mutability, attribute| match attribute {
                input::FunctionTypeAttribute::PayableKeyword => output::FunctionMutability::Payable,
                input::FunctionTypeAttribute::PureKeyword => output::FunctionMutability::Pure,
                input::FunctionTypeAttribute::ConstantKeyword
                | input::FunctionTypeAttribute::ViewKeyword => output::FunctionMutability::View,
                _ => mutability,
            },
        )
    }

    fn transform_constructor_definition(
        &mut self,
        source: &input::ConstructorDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Constructor;
        let name = None;
        let visibility = Self::constructor_visibility(&source.attributes);
        let mutability = Self::constructor_mutability(&source.attributes);
        let virtual_keyword = source
            .attributes
            .iter()
            .any(|attribute| matches!(attribute, input::ConstructorAttribute::VirtualKeyword));
        let override_specifier = Self::constructor_override_specifier(&source.attributes);
        let modifier_invocations = self.constructor_modifier_invocations(&source.attributes);
        let body = Some(self.transform_block(&source.body));
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
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
        attributes.iter().fold(
            output::FunctionVisibility::Public,
            |visibility, attribute| match attribute {
                input::ConstructorAttribute::InternalKeyword => {
                    output::FunctionVisibility::Internal
                }
                input::ConstructorAttribute::PublicKeyword => output::FunctionVisibility::Public,
                _ => visibility,
            },
        )
    }

    fn constructor_mutability(
        attributes: &input::ConstructorAttributes,
    ) -> output::FunctionMutability {
        attributes.iter().fold(
            output::FunctionMutability::NonPayable,
            |mutability, attribute| match attribute {
                input::ConstructorAttribute::PayableKeyword => output::FunctionMutability::Payable,
                _ => mutability,
            },
        )
    }

    fn constructor_override_specifier(
        attributes: &input::ConstructorAttributes,
    ) -> Option<output::OverridePaths> {
        attributes.iter().find_map(|attribute| {
            if matches!(attribute, input::ConstructorAttribute::OverrideKeyword) {
                Some(Vec::new())
            } else {
                None
            }
        })
    }

    fn constructor_modifier_invocations(
        &mut self,
        attributes: &input::ConstructorAttributes,
    ) -> output::ModifierInvocations {
        attributes
            .iter()
            .filter_map(|attribute| {
                if let input::ConstructorAttribute::ModifierInvocation(modifier_invocation) =
                    attribute
                {
                    Some(self.transform_modifier_invocation(modifier_invocation))
                } else {
                    None
                }
            })
            .collect()
    }

    fn transform_unnamed_function_definition(
        &mut self,
        source: &input::UnnamedFunctionDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Unnamed;
        let name = None;
        // TODO(validation): unnamed (aka fallback) functions *must* have external visibility
        let visibility = output::FunctionVisibility::External;
        let mutability = Self::unnamed_function_mutability(&source.attributes);
        let virtual_keyword = false;
        let override_specifier = None;
        let modifier_invocations = self.unnamed_function_modifier_invocations(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
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

    fn unnamed_function_mutability(
        attributes: &input::UnnamedFunctionAttributes,
    ) -> output::FunctionMutability {
        attributes.iter().fold(
            output::FunctionMutability::NonPayable,
            |mutability, attribute| match attribute {
                input::UnnamedFunctionAttribute::PayableKeyword => {
                    output::FunctionMutability::Payable
                }
                input::UnnamedFunctionAttribute::PureKeyword => output::FunctionMutability::Pure,
                input::UnnamedFunctionAttribute::ConstantKeyword
                | input::UnnamedFunctionAttribute::ViewKeyword => output::FunctionMutability::View,
                _ => mutability,
            },
        )
    }

    fn unnamed_function_modifier_invocations(
        &mut self,
        attributes: &input::UnnamedFunctionAttributes,
    ) -> output::ModifierInvocations {
        attributes
            .iter()
            .filter_map(|attribute| {
                if let input::UnnamedFunctionAttribute::ModifierInvocation(modifier_invocation) =
                    attribute
                {
                    Some(self.transform_modifier_invocation(modifier_invocation))
                } else {
                    None
                }
            })
            .collect()
    }

    fn transform_fallback_function_definition(
        &mut self,
        source: &input::FallbackFunctionDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Fallback;
        let name = None;
        // TODO(validation): fallback functions *must* have external visibility
        let visibility = output::FunctionVisibility::External;
        let mutability = Self::fallback_function_mutability(&source.attributes);
        let virtual_keyword = source
            .attributes
            .iter()
            .any(|attribute| matches!(attribute, input::FallbackFunctionAttribute::VirtualKeyword));
        let override_specifier = self.fallback_function_override_specifier(&source.attributes);
        let modifier_invocations = self.fallback_function_modifier_invocations(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_returns_declaration(returns));

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
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
        attributes.iter().fold(
            output::FunctionMutability::NonPayable,
            |mutability, attribute| match attribute {
                input::FallbackFunctionAttribute::PayableKeyword => {
                    output::FunctionMutability::Payable
                }
                input::FallbackFunctionAttribute::PureKeyword => output::FunctionMutability::Pure,
                input::FallbackFunctionAttribute::ViewKeyword => output::FunctionMutability::View,
                _ => mutability,
            },
        )
    }

    fn fallback_function_override_specifier(
        &mut self,
        attributes: &input::FallbackFunctionAttributes,
    ) -> Option<output::OverridePaths> {
        attributes.iter().find_map(|attribute| {
            if let input::FallbackFunctionAttribute::OverrideSpecifier(override_specifier) =
                attribute
            {
                Some(self.transform_override_specifier_into_override_paths(override_specifier))
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
            .iter()
            .filter_map(|attribute| {
                if let input::FallbackFunctionAttribute::ModifierInvocation(modifier_invocation) =
                    attribute
                {
                    Some(self.transform_modifier_invocation(modifier_invocation))
                } else {
                    None
                }
            })
            .collect()
    }

    fn transform_receive_function_definition(
        &mut self,
        source: &input::ReceiveFunctionDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Receive;
        let name = None;
        // TODO(validation): receive functions *must* have an external visibility
        let visibility = output::FunctionVisibility::External;
        // TODO(validation): receive functions *must* be payable
        let mutability = output::FunctionMutability::Payable;
        let virtual_keyword = source
            .attributes
            .iter()
            .any(|attribute| matches!(attribute, input::ReceiveFunctionAttribute::VirtualKeyword));
        let override_specifier = self.receive_function_override_specifier(&source.attributes);
        let modifier_invocations = self.receive_function_modifier_invocations(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
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
        attributes.iter().find_map(|attribute| {
            if let input::ReceiveFunctionAttribute::OverrideSpecifier(override_specifier) =
                attribute
            {
                Some(self.transform_override_specifier_into_override_paths(override_specifier))
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
            .iter()
            .filter_map(|attribute| {
                if let input::ReceiveFunctionAttribute::ModifierInvocation(modifier_invocation) =
                    attribute
                {
                    Some(self.transform_modifier_invocation(modifier_invocation))
                } else {
                    None
                }
            })
            .collect()
    }

    fn transform_modifier_definition(
        &mut self,
        source: &input::ModifierDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Modifier;
        let name = Some(Rc::clone(&source.name));
        let visibility = output::FunctionVisibility::Internal;
        // mutability is irrelevant for modifiers
        let mutability = output::FunctionMutability::NonPayable;
        let virtual_keyword = source
            .attributes
            .iter()
            .any(|attribute| matches!(attribute, input::ModifierAttribute::VirtualKeyword));
        let override_specifier = self.modifier_override_specifier(&source.attributes);
        let modifier_invocations = Vec::new();
        let body = self.transform_function_body(&source.body);
        let parameters = source.parameters.as_ref().map_or(Vec::new(), |parameters| {
            self.transform_parameters_declaration(parameters)
        });
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
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
        attributes.iter().find_map(|attribute| {
            if let input::ModifierAttribute::OverrideSpecifier(override_specifier) = attribute {
                Some(self.transform_override_specifier_into_override_paths(override_specifier))
            } else {
                None
            }
        })
    }

    fn state_variable_visibility(
        attributes: &input::StateVariableAttributes,
    ) -> output::StateVariableVisibility {
        // TODO(validation): only one visibility keyword is allowed
        attributes.iter().fold(
            output::StateVariableVisibility::Internal,
            |visibility, attribute| match attribute {
                input::StateVariableAttribute::InternalKeyword => {
                    output::StateVariableVisibility::Internal
                }
                input::StateVariableAttribute::PrivateKeyword => {
                    output::StateVariableVisibility::Private
                }
                input::StateVariableAttribute::PublicKeyword => {
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
        attributes.iter().fold(
            output::StateVariableMutability::Mutable,
            |mutability, attribute| match attribute {
                input::StateVariableAttribute::ConstantKeyword => {
                    output::StateVariableMutability::Constant
                }
                input::StateVariableAttribute::ImmutableKeyword => {
                    output::StateVariableMutability::Immutable
                }
                input::StateVariableAttribute::TransientKeyword => {
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
        attributes.iter().find_map(|attribute| {
            if let input::StateVariableAttribute::OverrideSpecifier(override_specifier) = attribute
            {
                Some(self.transform_override_specifier_into_override_paths(override_specifier))
            } else {
                None
            }
        })
    }

    fn state_variable_definition_as_constant_definition(
        state_variable_definition: output::StateVariableDefinitionStruct,
    ) -> output::ConstantDefinition {
        let node_id = state_variable_definition.node_id;
        let type_name = state_variable_definition.type_name;
        let name = Rc::clone(&state_variable_definition.name);
        let value = state_variable_definition.value;
        let visibility = Some(state_variable_definition.visibility);

        Rc::new(output::ConstantDefinitionStruct {
            node_id,
            type_name,
            name,
            visibility,
            value,
        })
    }

    fn string_literal_terminal_node(value: &input::StringLiteral) -> Rc<TerminalNode> {
        match value {
            input::StringLiteral::SingleQuotedStringLiteral(terminal_node)
            | input::StringLiteral::DoubleQuotedStringLiteral(terminal_node) => {
                Rc::clone(terminal_node)
            }
        }
    }

    fn hex_string_literal_terminal_node(value: &input::HexStringLiteral) -> Rc<TerminalNode> {
        match value {
            input::HexStringLiteral::SingleQuotedHexStringLiteral(terminal_node)
            | input::HexStringLiteral::DoubleQuotedHexStringLiteral(terminal_node) => {
                Rc::clone(terminal_node)
            }
        }
    }

    fn unicode_string_literal_terminal_node(
        value: &input::UnicodeStringLiteral,
    ) -> Rc<TerminalNode> {
        match value {
            input::UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(terminal_node)
            | input::UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(terminal_node) => {
                Rc::clone(terminal_node)
            }
        }
    }

    fn transform_tuple_deconstruction_elements(
        &mut self,
        source: &input::TupleDeconstructionElements,
        var_keyword: bool,
    ) -> output::TupleDeconstructionMembers {
        source
            .iter()
            .map(|element| {
                match &element.member {
                    Some(member) => match member {
                        input::TupleMember::TypedTupleMember(typed_tuple_member) => {
                            // TODO(validation): `var_keyword` should be false here,
                            // since we have a typed tuple member
                            output::TupleDeconstructionMember::VariableDeclarationStatement(
                                self.transform_typed_tuple_member(typed_tuple_member),
                            )
                        }
                        input::TupleMember::UntypedTupleMember(untyped_tuple_member) => {
                            if var_keyword {
                                output::TupleDeconstructionMember::VariableDeclarationStatement(
                                    self.transform_untyped_tuple_member(untyped_tuple_member),
                                )
                            } else {
                                // TODO(validation): the storage location of the
                                // member should be absent, since this is an
                                // assignment and not a declaration
                                output::TupleDeconstructionMember::Identifier(Rc::clone(
                                    &untyped_tuple_member.name,
                                ))
                            }
                        }
                    },
                    None => output::TupleDeconstructionMember::None,
                }
            })
            .collect()
    }

    fn transform_typed_tuple_member(
        &mut self,
        source: &input::TypedTupleMember,
    ) -> output::VariableDeclarationStatement {
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|location| self.transform_storage_location(location));
        let name = Rc::clone(&source.name);
        let type_name = Some(self.transform_type_name(&source.type_name));

        Rc::new(output::VariableDeclarationStatementStruct {
            node_id: source.node_id,
            storage_location,
            name,
            value: None,
            type_name,
        })
    }

    fn transform_untyped_tuple_member(
        &mut self,
        source: &input::UntypedTupleMember,
    ) -> output::VariableDeclarationStatement {
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|location| self.transform_storage_location(location));
        let name = Rc::clone(&source.name);

        Rc::new(output::VariableDeclarationStatementStruct {
            node_id: source.node_id,
            storage_location,
            name,
            value: None,
            type_name: None,
        })
    }

    fn transform_event_parameters(
        &mut self,
        source: &input::EventParameters,
    ) -> output::Parameters {
        source
            .iter()
            .map(|parameter| self.transform_event_parameter(parameter))
            .collect()
    }

    fn transform_event_parameter(&mut self, source: &input::EventParameter) -> output::Parameter {
        let type_name = self.transform_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);
        let indexed = source.indexed_keyword;

        Rc::new(output::ParameterStruct {
            node_id: source.node_id,
            type_name,
            storage_location: None,
            name,
            indexed,
        })
    }

    fn transform_error_parameters(
        &mut self,
        source: &input::ErrorParameters,
    ) -> output::Parameters {
        source
            .iter()
            .map(|parameter| self.transform_error_parameter(parameter))
            .collect()
    }

    fn transform_error_parameter(&mut self, source: &input::ErrorParameter) -> output::Parameter {
        let type_name = self.transform_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::ParameterStruct {
            node_id: source.node_id,
            type_name,
            storage_location: None,
            name,
            indexed: false,
        })
    }
}
