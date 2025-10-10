use std::collections::HashMap;
use std::rc::Rc;

use super::p0_build_ast::Output as Input;
use crate::backend::ir::ir2_flat_contracts::transformer::Transformer;
use crate::backend::ir::ir2_flat_contracts::{self as output, input, SourceUnit};
use crate::compilation::CompilationUnit;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, SourceUnit>,
}

/// This pass is reserved to make ergonomic changes to the AST in order to make
/// it easier to use. For now, it will only flatten contract specifiers:
/// inheritance and storage layout specifiers. In the future, more
/// transformations will be added.
pub fn run(input: Input) -> Output {
    let mut pass = Pass {};
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

struct Pass;

impl Transformer for Pass {
    fn transform_contract_definition(
        &mut self,
        source: &input::ContractDefinition,
    ) -> output::ContractDefinition {
        let node_id = source.node_id;
        let abstract_keyword = source.abstract_keyword.as_ref().map(Rc::clone);
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
        let attributes = self.transform_function_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_returns_declaration(returns));

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
            parameters,
            attributes,
            returns,
            kind,
            name,
            body,
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
            _ => self.default_transform_contract_member(source),
        }
    }

    fn transform_function_type(&mut self, source: &input::FunctionType) -> output::FunctionType {
        let node_id = source.node_id;
        let attributes = self.transform_function_type_attributes(&source.attributes);
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_returns_declaration(returns));

        Rc::new(output::FunctionTypeStruct {
            node_id,
            parameters,
            attributes,
            returns,
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
}

impl Pass {
    fn transform_function_body(&mut self, source: &input::FunctionBody) -> Option<output::Block> {
        match source {
            input::FunctionBody::Block(block) => Some(self.transform_block(block)),
            input::FunctionBody::Semicolon => None,
        }
    }

    fn transform_constructor_definition(
        &mut self,
        source: &input::ConstructorDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Constructor;
        let name = None;
        let attributes = self.transform_constructor_attributes(&source.attributes);
        let body = Some(self.transform_block(&source.body));
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
            parameters,
            attributes,
            returns,
            kind,
            name,
            body,
        })
    }

    fn transform_constructor_attributes(
        &mut self,
        source: &input::ConstructorAttributes,
    ) -> output::FunctionAttributes {
        source
            .iter()
            .filter_map(|item| self.transform_constructor_attribute(item))
            .collect()
    }

    fn transform_constructor_attribute(
        &mut self,
        source: &input::ConstructorAttribute,
    ) -> Option<output::FunctionAttribute> {
        match source {
            input::ConstructorAttribute::ModifierInvocation(modifier_invocation) => {
                Some(output::FunctionAttribute::ModifierInvocation(
                    self.transform_modifier_invocation(modifier_invocation),
                ))
            }
            input::ConstructorAttribute::InternalKeyword => {
                Some(output::FunctionAttribute::InternalKeyword)
            }
            input::ConstructorAttribute::OverrideKeyword => {
                // `override` as a constructor attribute is useless; it was only
                // enabled from 0.6.0 until 0.6.7
                None
            }
            input::ConstructorAttribute::PayableKeyword => {
                Some(output::FunctionAttribute::PayableKeyword)
            }
            input::ConstructorAttribute::PublicKeyword => {
                Some(output::FunctionAttribute::PublicKeyword)
            }
            input::ConstructorAttribute::VirtualKeyword => {
                Some(output::FunctionAttribute::VirtualKeyword)
            }
        }
    }

    fn transform_unnamed_function_definition(
        &mut self,
        source: &input::UnnamedFunctionDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Unnamed;
        let name = None;
        let attributes = self.transform_unnamed_function_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
            parameters,
            attributes,
            returns,
            kind,
            name,
            body,
        })
    }

    fn transform_unnamed_function_attributes(
        &mut self,
        source: &input::UnnamedFunctionAttributes,
    ) -> output::FunctionAttributes {
        source
            .iter()
            .map(|item| self.transform_unnamed_function_attribute(item))
            .collect()
    }

    fn transform_unnamed_function_attribute(
        &mut self,
        source: &input::UnnamedFunctionAttribute,
    ) -> output::FunctionAttribute {
        match source {
            input::UnnamedFunctionAttribute::ModifierInvocation(modifier_invocation) => {
                output::FunctionAttribute::ModifierInvocation(
                    self.transform_modifier_invocation(modifier_invocation),
                )
            }
            input::UnnamedFunctionAttribute::ConstantKeyword => {
                output::FunctionAttribute::ConstantKeyword
            }
            input::UnnamedFunctionAttribute::ExternalKeyword => {
                output::FunctionAttribute::ExternalKeyword
            }
            input::UnnamedFunctionAttribute::InternalKeyword => {
                output::FunctionAttribute::InternalKeyword
            }
            input::UnnamedFunctionAttribute::PayableKeyword => {
                output::FunctionAttribute::PayableKeyword
            }
            input::UnnamedFunctionAttribute::PrivateKeyword => {
                output::FunctionAttribute::PrivateKeyword
            }
            input::UnnamedFunctionAttribute::PublicKeyword => {
                output::FunctionAttribute::PublicKeyword
            }
            input::UnnamedFunctionAttribute::PureKeyword => output::FunctionAttribute::PureKeyword,
            input::UnnamedFunctionAttribute::ViewKeyword => output::FunctionAttribute::ViewKeyword,
        }
    }

    fn transform_fallback_function_definition(
        &mut self,
        source: &input::FallbackFunctionDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Fallback;
        let name = None;
        let attributes = self.transform_fallback_function_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_returns_declaration(returns));

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
            parameters,
            attributes,
            returns,
            kind,
            name,
            body,
        })
    }

    fn transform_fallback_function_attributes(
        &mut self,
        source: &input::FallbackFunctionAttributes,
    ) -> output::FunctionAttributes {
        source
            .iter()
            .map(|item| self.transform_fallback_function_attribute(item))
            .collect()
    }

    fn transform_fallback_function_attribute(
        &mut self,
        source: &input::FallbackFunctionAttribute,
    ) -> output::FunctionAttribute {
        match source {
            input::FallbackFunctionAttribute::ModifierInvocation(modifier_invocation) => {
                output::FunctionAttribute::ModifierInvocation(
                    self.transform_modifier_invocation(modifier_invocation),
                )
            }
            input::FallbackFunctionAttribute::OverrideSpecifier(override_specifier) => {
                output::FunctionAttribute::OverrideSpecifier(
                    self.transform_override_specifier(override_specifier),
                )
            }
            input::FallbackFunctionAttribute::ExternalKeyword => {
                output::FunctionAttribute::ExternalKeyword
            }
            input::FallbackFunctionAttribute::PayableKeyword => {
                output::FunctionAttribute::PayableKeyword
            }
            input::FallbackFunctionAttribute::PureKeyword => output::FunctionAttribute::PureKeyword,
            input::FallbackFunctionAttribute::ViewKeyword => output::FunctionAttribute::ViewKeyword,
            input::FallbackFunctionAttribute::VirtualKeyword => {
                output::FunctionAttribute::VirtualKeyword
            }
        }
    }

    fn transform_receive_function_definition(
        &mut self,
        source: &input::ReceiveFunctionDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Receive;
        let name = None;
        let attributes = self.transform_receive_function_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters_declaration(&source.parameters);
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
            parameters,
            attributes,
            returns,
            kind,
            name,
            body,
        })
    }

    fn transform_receive_function_attributes(
        &mut self,
        source: &input::ReceiveFunctionAttributes,
    ) -> output::FunctionAttributes {
        source
            .iter()
            .map(|item| self.transform_receive_function_attribute(item))
            .collect()
    }

    fn transform_receive_function_attribute(
        &mut self,
        source: &input::ReceiveFunctionAttribute,
    ) -> output::FunctionAttribute {
        match source {
            input::ReceiveFunctionAttribute::ModifierInvocation(modifier_invocation) => {
                output::FunctionAttribute::ModifierInvocation(
                    self.transform_modifier_invocation(modifier_invocation),
                )
            }
            input::ReceiveFunctionAttribute::OverrideSpecifier(override_specifier) => {
                output::FunctionAttribute::OverrideSpecifier(
                    self.transform_override_specifier(override_specifier),
                )
            }
            input::ReceiveFunctionAttribute::ExternalKeyword => {
                output::FunctionAttribute::ExternalKeyword
            }
            input::ReceiveFunctionAttribute::PayableKeyword => {
                output::FunctionAttribute::PayableKeyword
            }
            input::ReceiveFunctionAttribute::VirtualKeyword => {
                output::FunctionAttribute::VirtualKeyword
            }
        }
    }

    fn transform_modifier_definition(
        &mut self,
        source: &input::ModifierDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let kind = output::FunctionKind::Modifier;
        let name = Some(Rc::clone(&source.name));
        let attributes = self.transform_modifier_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = source.parameters.as_ref().map_or(Vec::new(), |parameters| {
            self.transform_parameters_declaration(parameters)
        });
        let returns = None;

        Rc::new(output::FunctionDefinitionStruct {
            node_id,
            parameters,
            attributes,
            returns,
            kind,
            name,
            body,
        })
    }

    fn transform_modifier_attributes(
        &mut self,
        source: &input::ModifierAttributes,
    ) -> output::FunctionAttributes {
        source
            .iter()
            .map(|item| self.transform_modifier_attribute(item))
            .collect()
    }

    fn transform_modifier_attribute(
        &mut self,
        source: &input::ModifierAttribute,
    ) -> output::FunctionAttribute {
        match source {
            input::ModifierAttribute::OverrideSpecifier(override_specifier) => {
                output::FunctionAttribute::OverrideSpecifier(
                    self.transform_override_specifier(override_specifier),
                )
            }
            input::ModifierAttribute::VirtualKeyword => output::FunctionAttribute::VirtualKeyword,
        }
    }
}
