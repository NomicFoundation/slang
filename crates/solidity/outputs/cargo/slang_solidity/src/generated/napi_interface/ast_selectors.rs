// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use napi::{Env, JsObject};
use napi_derive::napi;

use crate::napi_interface::cst::{RuleNode, ToJS};
use crate::napi_interface::{RuleKind, RustNamedNode, RustNode, RustRuleNode, TokenKind};

//
// Sequences:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node | null>",
    catch_unwind
)]
pub fn select_sequence(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
    env: Env,
) -> Result<Vec<Option<JsObject>>> {
    let mut selector = Selector::new(node, env);

    let result = match node.kind() {
        RuleKind::SourceUnit => selector.source_unit()?,
        RuleKind::PragmaDirective => selector.pragma_directive()?,
        RuleKind::ABICoderPragma => selector.abi_coder_pragma()?,
        RuleKind::ExperimentalPragma => selector.experimental_pragma()?,
        RuleKind::VersionPragma => selector.version_pragma()?,
        RuleKind::VersionPragmaOrExpression => selector.version_pragma_or_expression()?,
        RuleKind::VersionPragmaRangeExpression => selector.version_pragma_range_expression()?,
        RuleKind::VersionPragmaPrefixExpression => selector.version_pragma_prefix_expression()?,
        RuleKind::ImportDirective => selector.import_directive()?,
        RuleKind::PathImport => selector.path_import()?,
        RuleKind::NamedImport => selector.named_import()?,
        RuleKind::ImportDeconstruction => selector.import_deconstruction()?,
        RuleKind::ImportDeconstructionSymbol => selector.import_deconstruction_symbol()?,
        RuleKind::ImportAlias => selector.import_alias()?,
        RuleKind::UsingDirective => selector.using_directive()?,
        RuleKind::UsingDeconstruction => selector.using_deconstruction()?,
        RuleKind::UsingDeconstructionSymbol => selector.using_deconstruction_symbol()?,
        RuleKind::UsingAlias => selector.using_alias()?,
        RuleKind::ContractDefinition => selector.contract_definition()?,
        RuleKind::InheritanceSpecifier => selector.inheritance_specifier()?,
        RuleKind::InheritanceType => selector.inheritance_type()?,
        RuleKind::InterfaceDefinition => selector.interface_definition()?,
        RuleKind::LibraryDefinition => selector.library_definition()?,
        RuleKind::StructDefinition => selector.struct_definition()?,
        RuleKind::StructMember => selector.struct_member()?,
        RuleKind::EnumDefinition => selector.enum_definition()?,
        RuleKind::ConstantDefinition => selector.constant_definition()?,
        RuleKind::StateVariableDefinition => selector.state_variable_definition()?,
        RuleKind::StateVariableDefinitionValue => selector.state_variable_definition_value()?,
        RuleKind::FunctionDefinition => selector.function_definition()?,
        RuleKind::ParametersDeclaration => selector.parameters_declaration()?,
        RuleKind::Parameter => selector.parameter()?,
        RuleKind::OverrideSpecifier => selector.override_specifier()?,
        RuleKind::OverridePathsDeclaration => selector.override_paths_declaration()?,
        RuleKind::ReturnsDeclaration => selector.returns_declaration()?,
        RuleKind::ConstructorDefinition => selector.constructor_definition()?,
        RuleKind::UnnamedFunctionDefinition => selector.unnamed_function_definition()?,
        RuleKind::FallbackFunctionDefinition => selector.fallback_function_definition()?,
        RuleKind::ReceiveFunctionDefinition => selector.receive_function_definition()?,
        RuleKind::ModifierDefinition => selector.modifier_definition()?,
        RuleKind::ModifierInvocation => selector.modifier_invocation()?,
        RuleKind::EventDefinition => selector.event_definition()?,
        RuleKind::EventParametersDeclaration => selector.event_parameters_declaration()?,
        RuleKind::EventParameter => selector.event_parameter()?,
        RuleKind::UserDefinedValueTypeDefinition => {
            selector.user_defined_value_type_definition()?
        }
        RuleKind::ErrorDefinition => selector.error_definition()?,
        RuleKind::ErrorParametersDeclaration => selector.error_parameters_declaration()?,
        RuleKind::ErrorParameter => selector.error_parameter()?,
        RuleKind::ArrayTypeName => selector.array_type_name()?,
        RuleKind::FunctionType => selector.function_type()?,
        RuleKind::MappingType => selector.mapping_type()?,
        RuleKind::MappingKey => selector.mapping_key()?,
        RuleKind::MappingValue => selector.mapping_value()?,
        RuleKind::AddressType => selector.address_type()?,
        RuleKind::Block => selector.block()?,
        RuleKind::UncheckedBlock => selector.unchecked_block()?,
        RuleKind::ExpressionStatement => selector.expression_statement()?,
        RuleKind::AssemblyStatement => selector.assembly_statement()?,
        RuleKind::AssemblyFlagsDeclaration => selector.assembly_flags_declaration()?,
        RuleKind::TupleDeconstructionStatement => selector.tuple_deconstruction_statement()?,
        RuleKind::TupleDeconstructionElement => selector.tuple_deconstruction_element()?,
        RuleKind::TypedTupleMember => selector.typed_tuple_member()?,
        RuleKind::UntypedTupleMember => selector.untyped_tuple_member()?,
        RuleKind::VariableDeclarationStatement => selector.variable_declaration_statement()?,
        RuleKind::VariableDeclarationValue => selector.variable_declaration_value()?,
        RuleKind::IfStatement => selector.if_statement()?,
        RuleKind::ElseBranch => selector.else_branch()?,
        RuleKind::ForStatement => selector.for_statement()?,
        RuleKind::WhileStatement => selector.while_statement()?,
        RuleKind::DoWhileStatement => selector.do_while_statement()?,
        RuleKind::ContinueStatement => selector.continue_statement()?,
        RuleKind::BreakStatement => selector.break_statement()?,
        RuleKind::ReturnStatement => selector.return_statement()?,
        RuleKind::EmitStatement => selector.emit_statement()?,
        RuleKind::DeleteStatement => selector.delete_statement()?,
        RuleKind::TryStatement => selector.try_statement()?,
        RuleKind::CatchClause => selector.catch_clause()?,
        RuleKind::CatchClauseError => selector.catch_clause_error()?,
        RuleKind::RevertStatement => selector.revert_statement()?,
        RuleKind::ThrowStatement => selector.throw_statement()?,
        RuleKind::AssignmentExpression => selector.assignment_expression()?,
        RuleKind::ConditionalExpression => selector.conditional_expression()?,
        RuleKind::OrExpression => selector.or_expression()?,
        RuleKind::AndExpression => selector.and_expression()?,
        RuleKind::EqualityExpression => selector.equality_expression()?,
        RuleKind::ComparisonExpression => selector.comparison_expression()?,
        RuleKind::BitwiseOrExpression => selector.bitwise_or_expression()?,
        RuleKind::BitwiseXorExpression => selector.bitwise_xor_expression()?,
        RuleKind::BitwiseAndExpression => selector.bitwise_and_expression()?,
        RuleKind::ShiftExpression => selector.shift_expression()?,
        RuleKind::AdditiveExpression => selector.additive_expression()?,
        RuleKind::MultiplicativeExpression => selector.multiplicative_expression()?,
        RuleKind::ExponentiationExpression => selector.exponentiation_expression()?,
        RuleKind::PostfixExpression => selector.postfix_expression()?,
        RuleKind::PrefixExpression => selector.prefix_expression()?,
        RuleKind::FunctionCallExpression => selector.function_call_expression()?,
        RuleKind::MemberAccessExpression => selector.member_access_expression()?,
        RuleKind::IndexAccessExpression => selector.index_access_expression()?,
        RuleKind::IndexAccessEnd => selector.index_access_end()?,
        RuleKind::PositionalArgumentsDeclaration => selector.positional_arguments_declaration()?,
        RuleKind::NamedArgumentsDeclaration => selector.named_arguments_declaration()?,
        RuleKind::NamedArgumentGroup => selector.named_argument_group()?,
        RuleKind::NamedArgument => selector.named_argument()?,
        RuleKind::TypeExpression => selector.type_expression()?,
        RuleKind::NewExpression => selector.new_expression()?,
        RuleKind::TupleExpression => selector.tuple_expression()?,
        RuleKind::TupleValue => selector.tuple_value()?,
        RuleKind::ArrayExpression => selector.array_expression()?,
        RuleKind::HexNumberExpression => selector.hex_number_expression()?,
        RuleKind::DecimalNumberExpression => selector.decimal_number_expression()?,
        RuleKind::YulBlock => selector.yul_block()?,
        RuleKind::YulFunctionDefinition => selector.yul_function_definition()?,
        RuleKind::YulParametersDeclaration => selector.yul_parameters_declaration()?,
        RuleKind::YulReturnsDeclaration => selector.yul_returns_declaration()?,
        RuleKind::YulVariableDeclarationStatement => {
            selector.yul_variable_declaration_statement()?
        }
        RuleKind::YulVariableDeclarationValue => selector.yul_variable_declaration_value()?,
        RuleKind::YulAssignmentStatement => selector.yul_assignment_statement()?,
        RuleKind::YulIfStatement => selector.yul_if_statement()?,
        RuleKind::YulForStatement => selector.yul_for_statement()?,
        RuleKind::YulSwitchStatement => selector.yul_switch_statement()?,
        RuleKind::YulDefaultCase => selector.yul_default_case()?,
        RuleKind::YulValueCase => selector.yul_value_case()?,
        RuleKind::YulLeaveStatement => selector.yul_leave_statement()?,
        RuleKind::YulBreakStatement => selector.yul_break_statement()?,
        RuleKind::YulContinueStatement => selector.yul_continue_statement()?,
        RuleKind::YulLabel => selector.yul_label()?,
        RuleKind::YulFunctionCallExpression => selector.yul_function_call_expression()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![self.try_select(|node| {
            node.is_rule_with_kind(RuleKind::SourceUnitMembers)
        })?])
    }
}

impl Selector {
    fn pragma_directive(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::PragmaKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Pragma))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn abi_coder_pragma(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AbicoderKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn experimental_pragma(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ExperimentalKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ExperimentalFeature))?),
        ])
    }
}

impl Selector {
    fn version_pragma(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::SolidityKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpressions))?),
        ])
    }
}

impl Selector {
    fn version_pragma_or_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::BarBar))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))?),
        ])
    }
}

impl Selector {
    fn version_pragma_range_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Minus))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))?),
        ])
    }
}

impl Selector {
    fn version_pragma_prefix_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Caret))?),
        ])
    }
}

impl Selector {
    fn import_directive(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ImportKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ImportClause))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn path_import(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ImportAlias))?,
        ])
    }
}

impl Selector {
    fn named_import(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Asterisk))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ImportAlias))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::FromKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))?),
        ])
    }
}

impl Selector {
    fn import_deconstruction(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            Some(
                self.select(|node| node.is_rule_with_kind(RuleKind::ImportDeconstructionSymbols))?,
            ),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::FromKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))?),
        ])
    }
}

impl Selector {
    fn import_deconstruction_symbol(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ImportAlias))?,
        ])
    }
}

impl Selector {
    fn import_alias(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AsKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn using_directive(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::UsingKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::UsingClause))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ForKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::UsingTarget))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::GlobalKeyword))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn using_deconstruction(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::UsingDeconstructionSymbols))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn using_deconstruction_symbol(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::UsingAlias))?,
        ])
    }
}

impl Selector {
    fn using_alias(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AsKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::UsingOperator))?),
        ])
    }
}

impl Selector {
    fn contract_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            self.try_select(|node| node.is_token_with_kind(TokenKind::AbstractKeyword))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ContractKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::InheritanceSpecifier))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ContractMembers))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn inheritance_specifier(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::IsKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::InheritanceTypes))?),
        ])
    }
}

impl Selector {
    fn inheritance_type(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration))?,
        ])
    }
}

impl Selector {
    fn interface_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::InterfaceKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::InheritanceSpecifier))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::InterfaceMembers))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn library_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::LibraryKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::LibraryMembers))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn struct_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::StructKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::StructMembers))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn struct_member(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn enum_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::EnumKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::EnumMembers))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn constant_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ConstantKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Equal))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn state_variable_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::StateVariableAttributes))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::StateVariableDefinitionValue))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn state_variable_definition_value(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Equal))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn function_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::FunctionKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::FunctionName))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::FunctionAttributes))?,
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ReturnsDeclaration))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn parameters_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::Parameters))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn parameter(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::StorageLocation))?,
            self.try_select(|node| node.is_token_with_kind(TokenKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn override_specifier(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OverrideKeyword))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::OverridePathsDeclaration))?,
        ])
    }
}

impl Selector {
    fn override_paths_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::OverridePaths))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn returns_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ReturnsKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))?),
        ])
    }
}

impl Selector {
    fn constructor_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ConstructorKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ConstructorAttributes))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Block))?),
        ])
    }
}

impl Selector {
    fn unnamed_function_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::FunctionKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::UnnamedFunctionAttributes))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn fallback_function_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::FallbackKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::FallbackFunctionAttributes))?,
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ReturnsDeclaration))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn receive_function_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ReceiveKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ReceiveFunctionAttributes))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn modifier_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ModifierKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))?,
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ModifierAttributes))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn modifier_invocation(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration))?,
        ])
    }
}

impl Selector {
    fn event_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::EventKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::EventParametersDeclaration))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::AnonymousKeyword))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn event_parameters_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::EventParameters))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn event_parameter(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::IndexedKeyword))?,
            self.try_select(|node| node.is_token_with_kind(TokenKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn user_defined_value_type_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::TypeKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::IsKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ElementaryType))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn error_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ErrorKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ErrorParametersDeclaration))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn error_parameters_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ErrorParameters))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn error_parameter(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn array_type_name(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBracket))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::Expression))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBracket))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
        ])
    }
}

impl Selector {
    fn function_type(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::FunctionKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::FunctionTypeAttributes))?,
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ReturnsDeclaration))?,
        ])
    }
}

impl Selector {
    fn mapping_type(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::MappingKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::MappingKey))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::EqualGreaterThan))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::MappingValue))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn mapping_key(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::MappingKeyType))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn mapping_value(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn address_type(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AddressKeyword))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::PayableKeyword))?,
        ])
    }
}

impl Selector {
    fn block(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::Statements))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn unchecked_block(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::UncheckedKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Block))?),
        ])
    }
}

impl Selector {
    fn expression_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn assembly_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AssemblyKeyword))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))?,
            self.try_select(|node| node.is_rule_with_kind(RuleKind::AssemblyFlagsDeclaration))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn assembly_flags_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::AssemblyFlags))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn tuple_deconstruction_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            self.try_select(|node| node.is_token_with_kind(TokenKind::VarKeyword))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(
                self.select(|node| node.is_rule_with_kind(RuleKind::TupleDeconstructionElements))?,
            ),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Equal))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn tuple_deconstruction_element(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![self.try_select(|node| {
            node.is_rule_with_kind(RuleKind::TupleMember)
        })?])
    }
}

impl Selector {
    fn typed_tuple_member(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::StorageLocation))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn untyped_tuple_member(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            self.try_select(|node| node.is_rule_with_kind(RuleKind::StorageLocation))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn variable_declaration_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::VariableDeclarationType))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::StorageLocation))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::VariableDeclarationValue))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn variable_declaration_value(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Equal))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn if_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::IfKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Statement))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ElseBranch))?,
        ])
    }
}

impl Selector {
    fn else_branch(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ElseKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Statement))?),
        ])
    }
}

impl Selector {
    fn for_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ForKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ForStatementInitialization))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ForStatementCondition))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::Expression))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Statement))?),
        ])
    }
}

impl Selector {
    fn while_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::WhileKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Statement))?),
        ])
    }
}

impl Selector {
    fn do_while_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::DoKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Statement))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::WhileKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn continue_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ContinueKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn break_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::BreakKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn return_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ReturnKeyword))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::Expression))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn emit_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::EmitKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn delete_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::DeleteKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn try_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::TryKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ReturnsDeclaration))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Block))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::CatchClauses))?),
        ])
    }
}

impl Selector {
    fn catch_clause(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CatchKeyword))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::CatchClauseError))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Block))?),
        ])
    }
}

impl Selector {
    fn catch_clause_error(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            self.try_select(|node| node.is_token_with_kind(TokenKind::Identifier))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))?),
        ])
    }
}

impl Selector {
    fn revert_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::RevertKeyword))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn throw_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ThrowKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn assignment_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Equal))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn conditional_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::QuestionMark))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Colon))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn or_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::BarBar))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn and_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AmpersandAmpersand))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn equality_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::EqualEqual))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn comparison_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::LessThan))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn bitwise_or_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Bar))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn bitwise_xor_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Caret))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn bitwise_and_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Ampersand))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn shift_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::LessThanLessThan))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn additive_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Plus))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn multiplicative_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Asterisk))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn exponentiation_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::AsteriskAsterisk))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn postfix_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::PlusPlus))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn prefix_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::PlusPlus))?),
        ])
    }
}

impl Selector {
    fn function_call_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            self.try_select(|node| node.is_rule_with_kind(RuleKind::FunctionCallOptions))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn member_access_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Period))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::MemberAccess))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn index_access_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBracket))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::Expression))?,
            self.try_select(|node| node.is_rule_with_kind(RuleKind::IndexAccessEnd))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBracket))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn index_access_end(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Colon))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::Expression))?,
        ])
    }
}

impl Selector {
    fn positional_arguments_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::PositionalArguments))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn named_arguments_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::NamedArgumentGroup))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn named_argument_group(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::NamedArguments))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn named_argument(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Colon))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn type_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::TypeKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn new_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::NewKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TypeName))?),
        ])
    }
}

impl Selector {
    fn tuple_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TupleValues))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn tuple_value(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![self.try_select(|node| {
            node.is_rule_with_kind(RuleKind::Expression)
        })?])
    }
}

impl Selector {
    fn array_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBracket))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::ArrayValues))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn hex_number_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::HexLiteral))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::NumberUnit))?,
        ])
    }
}

impl Selector {
    fn decimal_number_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::DecimalLiteral))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::NumberUnit))?,
        ])
    }
}

impl Selector {
    fn yul_block(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBrace))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::YulStatements))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn yul_function_definition(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::YulFunctionKeyword))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::YulIdentifier))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulParametersDeclaration))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::YulReturnsDeclaration))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_parameters_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::YulParameters))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn yul_returns_declaration(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::MinusGreaterThan))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulReturnVariables))?),
        ])
    }
}

impl Selector {
    fn yul_variable_declaration_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::YulLetKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulIdentifierPaths))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::YulVariableDeclarationValue))?,
        ])
    }
}

impl Selector {
    fn yul_variable_declaration_value(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ColonEqual))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulExpression))?),
        ])
    }
}

impl Selector {
    fn yul_assignment_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulIdentifierPaths))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::ColonEqual))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulExpression))?),
        ])
    }
}

impl Selector {
    fn yul_if_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::YulIfKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulExpression))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_for_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::YulForKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulBlock))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulExpression))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulBlock))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_switch_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::YulSwitchKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulExpression))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulSwitchCases))?),
        ])
    }
}

impl Selector {
    fn yul_default_case(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::YulDefaultKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_value_case(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::YulCaseKeyword))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulLiteral))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_leave_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_token_with_kind(TokenKind::YulLeaveKeyword)
        })?)])
    }
}

impl Selector {
    fn yul_break_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_token_with_kind(TokenKind::YulBreakKeyword)
        })?)])
    }
}

impl Selector {
    fn yul_continue_statement(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_token_with_kind(TokenKind::YulContinueKeyword)
        })?)])
    }
}

impl Selector {
    fn yul_label(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::YulIdentifier))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Colon))?),
        ])
    }
}

impl Selector {
    fn yul_function_call_expression(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenParen))?),
            self.try_select(|node| node.is_rule_with_kind(RuleKind::YulArguments))?,
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseParen))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::YulExpression))?),
        ])
    }
}

//
// Choices:
//

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn select_choice(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
    env: Env,
) -> Result<JsObject> {
    let mut selector = Selector::new(node, env);

    let result = match node.kind() {
        RuleKind::SourceUnitMember => selector.source_unit_member()?,
        RuleKind::Pragma => selector.pragma()?,
        RuleKind::ExperimentalFeature => selector.experimental_feature()?,
        RuleKind::VersionPragmaExpression => selector.version_pragma_expression()?,
        RuleKind::ImportClause => selector.import_clause()?,
        RuleKind::UsingClause => selector.using_clause()?,
        RuleKind::UsingOperator => selector.using_operator()?,
        RuleKind::UsingTarget => selector.using_target()?,
        RuleKind::ContractMember => selector.contract_member()?,
        RuleKind::StateVariableAttribute => selector.state_variable_attribute()?,
        RuleKind::FunctionName => selector.function_name()?,
        RuleKind::FunctionAttribute => selector.function_attribute()?,
        RuleKind::FunctionBody => selector.function_body()?,
        RuleKind::ConstructorAttribute => selector.constructor_attribute()?,
        RuleKind::UnnamedFunctionAttribute => selector.unnamed_function_attribute()?,
        RuleKind::FallbackFunctionAttribute => selector.fallback_function_attribute()?,
        RuleKind::ReceiveFunctionAttribute => selector.receive_function_attribute()?,
        RuleKind::ModifierAttribute => selector.modifier_attribute()?,
        RuleKind::TypeName => selector.type_name()?,
        RuleKind::FunctionTypeAttribute => selector.function_type_attribute()?,
        RuleKind::MappingKeyType => selector.mapping_key_type()?,
        RuleKind::ElementaryType => selector.elementary_type()?,
        RuleKind::Statement => selector.statement()?,
        RuleKind::TupleMember => selector.tuple_member()?,
        RuleKind::VariableDeclarationType => selector.variable_declaration_type()?,
        RuleKind::StorageLocation => selector.storage_location()?,
        RuleKind::ForStatementInitialization => selector.for_statement_initialization()?,
        RuleKind::ForStatementCondition => selector.for_statement_condition()?,
        RuleKind::Expression => selector.expression()?,
        RuleKind::MemberAccess => selector.member_access()?,
        RuleKind::FunctionCallOptions => selector.function_call_options()?,
        RuleKind::ArgumentsDeclaration => selector.arguments_declaration()?,
        RuleKind::NumberUnit => selector.number_unit()?,
        RuleKind::StringExpression => selector.string_expression()?,
        RuleKind::YulStatement => selector.yul_statement()?,
        RuleKind::YulSwitchCase => selector.yul_switch_case()?,
        RuleKind::YulExpression => selector.yul_expression()?,
        RuleKind::YulBuiltInFunction => selector.yul_built_in_function()?,
        RuleKind::YulLiteral => selector.yul_literal()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_member(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::PragmaDirective,
                RuleKind::ImportDirective,
                RuleKind::ContractDefinition,
                RuleKind::InterfaceDefinition,
                RuleKind::LibraryDefinition,
                RuleKind::StructDefinition,
                RuleKind::EnumDefinition,
                RuleKind::FunctionDefinition,
                RuleKind::ConstantDefinition,
                RuleKind::ErrorDefinition,
                RuleKind::UserDefinedValueTypeDefinition,
                RuleKind::UsingDirective,
                RuleKind::EventDefinition,
            ])
        })
    }
}

impl Selector {
    fn pragma(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::ABICoderPragma,
                RuleKind::ExperimentalPragma,
                RuleKind::VersionPragma,
            ])
        })
    }
}

impl Selector {
    fn experimental_feature(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_token_with_kinds(&[TokenKind::Identifier, TokenKind::AsciiStringLiteral])
        })
    }
}

impl Selector {
    fn version_pragma_expression(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::VersionPragmaOrExpression,
                RuleKind::VersionPragmaRangeExpression,
                RuleKind::VersionPragmaPrefixExpression,
                RuleKind::VersionPragmaSpecifier,
            ])
        })
    }
}

impl Selector {
    fn import_clause(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::PathImport,
                RuleKind::NamedImport,
                RuleKind::ImportDeconstruction,
            ])
        })
    }
}

impl Selector {
    fn using_clause(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[RuleKind::IdentifierPath, RuleKind::UsingDeconstruction])
        })
    }
}

impl Selector {
    fn using_operator(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_token_with_kinds(&[
                TokenKind::Ampersand,
                TokenKind::Asterisk,
                TokenKind::BangEqual,
                TokenKind::Bar,
                TokenKind::Caret,
                TokenKind::EqualEqual,
                TokenKind::GreaterThan,
                TokenKind::GreaterThanEqual,
                TokenKind::LessThan,
                TokenKind::LessThanEqual,
                TokenKind::Minus,
                TokenKind::Percent,
                TokenKind::Plus,
                TokenKind::Slash,
                TokenKind::Tilde,
            ])
        })
    }
}

impl Selector {
    fn using_target(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::TypeName)
                || node.is_token_with_kind(TokenKind::Asterisk)
        })
    }
}

impl Selector {
    fn contract_member(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::UsingDirective,
                RuleKind::FunctionDefinition,
                RuleKind::ConstructorDefinition,
                RuleKind::ReceiveFunctionDefinition,
                RuleKind::FallbackFunctionDefinition,
                RuleKind::UnnamedFunctionDefinition,
                RuleKind::ModifierDefinition,
                RuleKind::StructDefinition,
                RuleKind::EnumDefinition,
                RuleKind::EventDefinition,
                RuleKind::StateVariableDefinition,
                RuleKind::ErrorDefinition,
                RuleKind::UserDefinedValueTypeDefinition,
            ])
        })
    }
}

impl Selector {
    fn state_variable_attribute(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::OverrideSpecifier)
                || node.is_token_with_kinds(&[
                    TokenKind::ConstantKeyword,
                    TokenKind::InternalKeyword,
                    TokenKind::PrivateKeyword,
                    TokenKind::PublicKeyword,
                    TokenKind::ImmutableKeyword,
                ])
        })
    }
}

impl Selector {
    fn function_name(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_token_with_kinds(&[
                TokenKind::Identifier,
                TokenKind::FallbackKeyword,
                TokenKind::ReceiveKeyword,
            ])
        })
    }
}

impl Selector {
    fn function_attribute(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[RuleKind::ModifierInvocation, RuleKind::OverrideSpecifier])
                || node.is_token_with_kinds(&[
                    TokenKind::ConstantKeyword,
                    TokenKind::ExternalKeyword,
                    TokenKind::InternalKeyword,
                    TokenKind::PayableKeyword,
                    TokenKind::PrivateKeyword,
                    TokenKind::PublicKeyword,
                    TokenKind::PureKeyword,
                    TokenKind::ViewKeyword,
                    TokenKind::VirtualKeyword,
                ])
        })
    }
}

impl Selector {
    fn function_body(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::Block) || node.is_token_with_kind(TokenKind::Semicolon)
        })
    }
}

impl Selector {
    fn constructor_attribute(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::ModifierInvocation)
                || node.is_token_with_kinds(&[
                    TokenKind::InternalKeyword,
                    TokenKind::PayableKeyword,
                    TokenKind::PublicKeyword,
                ])
        })
    }
}

impl Selector {
    fn unnamed_function_attribute(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[RuleKind::ModifierInvocation, RuleKind::OverrideSpecifier])
                || node.is_token_with_kinds(&[
                    TokenKind::ExternalKeyword,
                    TokenKind::InternalKeyword,
                    TokenKind::PayableKeyword,
                    TokenKind::PrivateKeyword,
                    TokenKind::PublicKeyword,
                    TokenKind::PureKeyword,
                    TokenKind::ViewKeyword,
                ])
        })
    }
}

impl Selector {
    fn fallback_function_attribute(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[RuleKind::ModifierInvocation, RuleKind::OverrideSpecifier])
                || node.is_token_with_kinds(&[
                    TokenKind::ExternalKeyword,
                    TokenKind::PayableKeyword,
                    TokenKind::PureKeyword,
                    TokenKind::ViewKeyword,
                    TokenKind::VirtualKeyword,
                ])
        })
    }
}

impl Selector {
    fn receive_function_attribute(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[RuleKind::ModifierInvocation, RuleKind::OverrideSpecifier])
                || node.is_token_with_kinds(&[
                    TokenKind::ExternalKeyword,
                    TokenKind::PayableKeyword,
                    TokenKind::VirtualKeyword,
                ])
        })
    }
}

impl Selector {
    fn modifier_attribute(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::OverrideSpecifier)
                || node.is_token_with_kind(TokenKind::VirtualKeyword)
        })
    }
}

impl Selector {
    fn type_name(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::ArrayTypeName,
                RuleKind::FunctionType,
                RuleKind::MappingType,
                RuleKind::ElementaryType,
                RuleKind::IdentifierPath,
            ])
        })
    }
}

impl Selector {
    fn function_type_attribute(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_token_with_kinds(&[
                TokenKind::InternalKeyword,
                TokenKind::ExternalKeyword,
                TokenKind::PrivateKeyword,
                TokenKind::PublicKeyword,
                TokenKind::PureKeyword,
                TokenKind::ViewKeyword,
                TokenKind::PayableKeyword,
            ])
        })
    }
}

impl Selector {
    fn mapping_key_type(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[RuleKind::ElementaryType, RuleKind::IdentifierPath])
        })
    }
}

impl Selector {
    fn elementary_type(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::AddressType)
                || node.is_token_with_kinds(&[
                    TokenKind::BoolKeyword,
                    TokenKind::ByteKeyword,
                    TokenKind::StringKeyword,
                    TokenKind::BytesKeyword,
                    TokenKind::IntKeyword,
                    TokenKind::UintKeyword,
                    TokenKind::FixedKeyword,
                    TokenKind::UfixedKeyword,
                ])
        })
    }
}

impl Selector {
    fn statement(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::ExpressionStatement,
                RuleKind::VariableDeclarationStatement,
                RuleKind::TupleDeconstructionStatement,
                RuleKind::IfStatement,
                RuleKind::ForStatement,
                RuleKind::WhileStatement,
                RuleKind::DoWhileStatement,
                RuleKind::ContinueStatement,
                RuleKind::BreakStatement,
                RuleKind::DeleteStatement,
                RuleKind::ReturnStatement,
                RuleKind::ThrowStatement,
                RuleKind::EmitStatement,
                RuleKind::TryStatement,
                RuleKind::RevertStatement,
                RuleKind::AssemblyStatement,
                RuleKind::Block,
                RuleKind::UncheckedBlock,
            ])
        })
    }
}

impl Selector {
    fn tuple_member(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[RuleKind::TypedTupleMember, RuleKind::UntypedTupleMember])
        })
    }
}

impl Selector {
    fn variable_declaration_type(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::TypeName)
                || node.is_token_with_kind(TokenKind::VarKeyword)
        })
    }
}

impl Selector {
    fn storage_location(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_token_with_kinds(&[
                TokenKind::MemoryKeyword,
                TokenKind::StorageKeyword,
                TokenKind::CallDataKeyword,
            ])
        })
    }
}

impl Selector {
    fn for_statement_initialization(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::ExpressionStatement,
                RuleKind::VariableDeclarationStatement,
                RuleKind::TupleDeconstructionStatement,
            ]) || node.is_token_with_kind(TokenKind::Semicolon)
        })
    }
}

impl Selector {
    fn for_statement_condition(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::ExpressionStatement)
                || node.is_token_with_kind(TokenKind::Semicolon)
        })
    }
}

impl Selector {
    fn expression(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::AssignmentExpression,
                RuleKind::ConditionalExpression,
                RuleKind::OrExpression,
                RuleKind::AndExpression,
                RuleKind::EqualityExpression,
                RuleKind::ComparisonExpression,
                RuleKind::BitwiseOrExpression,
                RuleKind::BitwiseXorExpression,
                RuleKind::BitwiseAndExpression,
                RuleKind::ShiftExpression,
                RuleKind::AdditiveExpression,
                RuleKind::MultiplicativeExpression,
                RuleKind::ExponentiationExpression,
                RuleKind::PostfixExpression,
                RuleKind::PrefixExpression,
                RuleKind::FunctionCallExpression,
                RuleKind::MemberAccessExpression,
                RuleKind::IndexAccessExpression,
                RuleKind::NewExpression,
                RuleKind::TupleExpression,
                RuleKind::TypeExpression,
                RuleKind::ArrayExpression,
                RuleKind::HexNumberExpression,
                RuleKind::DecimalNumberExpression,
                RuleKind::StringExpression,
                RuleKind::ElementaryType,
            ]) || node.is_token_with_kinds(&[
                TokenKind::PayableKeyword,
                TokenKind::TrueKeyword,
                TokenKind::FalseKeyword,
                TokenKind::Identifier,
            ])
        })
    }
}

impl Selector {
    fn member_access(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_token_with_kinds(&[TokenKind::Identifier, TokenKind::AddressKeyword])
        })
    }
}

impl Selector {
    fn function_call_options(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[RuleKind::NamedArgumentGroups, RuleKind::NamedArgumentGroup])
        })
    }
}

impl Selector {
    fn arguments_declaration(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::PositionalArgumentsDeclaration,
                RuleKind::NamedArgumentsDeclaration,
            ])
        })
    }
}

impl Selector {
    fn number_unit(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_token_with_kinds(&[
                TokenKind::WeiKeyword,
                TokenKind::GweiKeyword,
                TokenKind::SzaboKeyword,
                TokenKind::FinneyKeyword,
                TokenKind::EtherKeyword,
                TokenKind::SecondsKeyword,
                TokenKind::MinutesKeyword,
                TokenKind::HoursKeyword,
                TokenKind::DaysKeyword,
                TokenKind::WeeksKeyword,
                TokenKind::YearsKeyword,
            ])
        })
    }
}

impl Selector {
    fn string_expression(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::HexStringLiterals,
                RuleKind::AsciiStringLiterals,
                RuleKind::UnicodeStringLiterals,
            ]) || node
                .is_token_with_kinds(&[TokenKind::HexStringLiteral, TokenKind::AsciiStringLiteral])
        })
    }
}

impl Selector {
    fn yul_statement(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::YulBlock,
                RuleKind::YulFunctionDefinition,
                RuleKind::YulVariableDeclarationStatement,
                RuleKind::YulAssignmentStatement,
                RuleKind::YulIfStatement,
                RuleKind::YulForStatement,
                RuleKind::YulSwitchStatement,
                RuleKind::YulLeaveStatement,
                RuleKind::YulBreakStatement,
                RuleKind::YulContinueStatement,
                RuleKind::YulLabel,
                RuleKind::YulExpression,
            ])
        })
    }
}

impl Selector {
    fn yul_switch_case(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[RuleKind::YulDefaultCase, RuleKind::YulValueCase])
        })
    }
}

impl Selector {
    fn yul_expression(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::YulFunctionCallExpression,
                RuleKind::YulLiteral,
                RuleKind::YulBuiltInFunction,
                RuleKind::YulIdentifierPath,
            ])
        })
    }
}

impl Selector {
    fn yul_built_in_function(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_token_with_kinds(&[
                TokenKind::YulAddKeyword,
                TokenKind::YulAddModKeyword,
                TokenKind::YulAddressKeyword,
                TokenKind::YulAndKeyword,
                TokenKind::YulBalanceKeyword,
                TokenKind::YulBlockHashKeyword,
                TokenKind::YulByteKeyword,
                TokenKind::YulCallCodeKeyword,
                TokenKind::YulCallDataCopyKeyword,
                TokenKind::YulCallDataLoadKeyword,
                TokenKind::YulCallDataSizeKeyword,
                TokenKind::YulCallerKeyword,
                TokenKind::YulCallKeyword,
                TokenKind::YulCallValueKeyword,
                TokenKind::YulCoinBaseKeyword,
                TokenKind::YulCreateKeyword,
                TokenKind::YulDelegateCallKeyword,
                TokenKind::YulDivKeyword,
                TokenKind::YulEqKeyword,
                TokenKind::YulExpKeyword,
                TokenKind::YulExtCodeCopyKeyword,
                TokenKind::YulExtCodeSizeKeyword,
                TokenKind::YulGasKeyword,
                TokenKind::YulGasLimitKeyword,
                TokenKind::YulGasPriceKeyword,
                TokenKind::YulGtKeyword,
                TokenKind::YulInvalidKeyword,
                TokenKind::YulIsZeroKeyword,
                TokenKind::YulLog0Keyword,
                TokenKind::YulLog1Keyword,
                TokenKind::YulLog2Keyword,
                TokenKind::YulLog3Keyword,
                TokenKind::YulLog4Keyword,
                TokenKind::YulLtKeyword,
                TokenKind::YulMLoadKeyword,
                TokenKind::YulModKeyword,
                TokenKind::YulMSizeKeyword,
                TokenKind::YulMStore8Keyword,
                TokenKind::YulMStoreKeyword,
                TokenKind::YulMulKeyword,
                TokenKind::YulMulModKeyword,
                TokenKind::YulNotKeyword,
                TokenKind::YulNumberKeyword,
                TokenKind::YulOriginKeyword,
                TokenKind::YulOrKeyword,
                TokenKind::YulPopKeyword,
                TokenKind::YulReturnKeyword,
                TokenKind::YulRevertKeyword,
                TokenKind::YulSDivKeyword,
                TokenKind::YulSelfDestructKeyword,
                TokenKind::YulSgtKeyword,
                TokenKind::YulSignExtendKeyword,
                TokenKind::YulSLoadKeyword,
                TokenKind::YulSltKeyword,
                TokenKind::YulSModKeyword,
                TokenKind::YulSStoreKeyword,
                TokenKind::YulStopKeyword,
                TokenKind::YulSubKeyword,
                TokenKind::YulTimestampKeyword,
                TokenKind::YulXorKeyword,
                TokenKind::YulKeccak256Keyword,
                TokenKind::YulSha3Keyword,
                TokenKind::YulSuicideKeyword,
                TokenKind::YulReturnDataCopyKeyword,
                TokenKind::YulReturnDataSizeKeyword,
                TokenKind::YulStaticCallKeyword,
                TokenKind::YulCreate2Keyword,
                TokenKind::YulExtCodeHashKeyword,
                TokenKind::YulSarKeyword,
                TokenKind::YulShlKeyword,
                TokenKind::YulShrKeyword,
                TokenKind::YulChainIdKeyword,
                TokenKind::YulSelfBalanceKeyword,
                TokenKind::YulBaseFeeKeyword,
                TokenKind::YulDifficultyKeyword,
                TokenKind::YulPrevRandaoKeyword,
            ])
        })
    }
}

impl Selector {
    fn yul_literal(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_token_with_kinds(&[
                TokenKind::YulTrueKeyword,
                TokenKind::YulFalseKeyword,
                TokenKind::YulDecimalLiteral,
                TokenKind::YulHexLiteral,
                TokenKind::HexStringLiteral,
                TokenKind::AsciiStringLiteral,
            ])
        })
    }
}

//
// Repeated:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node>",
    catch_unwind
)]
pub fn select_repeated(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
    env: Env,
) -> Result<Vec<JsObject>> {
    let mut selector = Selector::new(node, env);

    let result = match node.kind() {
        RuleKind::SourceUnitMembers => selector.source_unit_members()?,
        RuleKind::VersionPragmaExpressions => selector.version_pragma_expressions()?,
        RuleKind::ContractMembers => selector.contract_members()?,
        RuleKind::InterfaceMembers => selector.interface_members()?,
        RuleKind::LibraryMembers => selector.library_members()?,
        RuleKind::StructMembers => selector.struct_members()?,
        RuleKind::StateVariableAttributes => selector.state_variable_attributes()?,
        RuleKind::FunctionAttributes => selector.function_attributes()?,
        RuleKind::ConstructorAttributes => selector.constructor_attributes()?,
        RuleKind::UnnamedFunctionAttributes => selector.unnamed_function_attributes()?,
        RuleKind::FallbackFunctionAttributes => selector.fallback_function_attributes()?,
        RuleKind::ReceiveFunctionAttributes => selector.receive_function_attributes()?,
        RuleKind::ModifierAttributes => selector.modifier_attributes()?,
        RuleKind::FunctionTypeAttributes => selector.function_type_attributes()?,
        RuleKind::Statements => selector.statements()?,
        RuleKind::CatchClauses => selector.catch_clauses()?,
        RuleKind::NamedArgumentGroups => selector.named_argument_groups()?,
        RuleKind::HexStringLiterals => selector.hex_string_literals()?,
        RuleKind::AsciiStringLiterals => selector.ascii_string_literals()?,
        RuleKind::UnicodeStringLiterals => selector.unicode_string_literals()?,
        RuleKind::YulStatements => selector.yul_statements()?,
        RuleKind::YulSwitchCases => selector.yul_switch_cases()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_members(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::SourceUnitMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn version_pragma_expressions(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn contract_members(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ContractMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn interface_members(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ContractMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn library_members(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ContractMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn struct_members(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::StructMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn state_variable_attributes(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::StateVariableAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn function_attributes(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::FunctionAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn constructor_attributes(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ConstructorAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn unnamed_function_attributes(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::UnnamedFunctionAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn fallback_function_attributes(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::FallbackFunctionAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn receive_function_attributes(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ReceiveFunctionAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn modifier_attributes(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::ModifierAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn function_type_attributes(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::FunctionTypeAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn statements(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::Statement))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn catch_clauses(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::CatchClause))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn named_argument_groups(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::NamedArgumentGroup))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn hex_string_literals(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::HexStringLiteral))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn ascii_string_literals(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn unicode_string_literals(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::UnicodeStringLiteral))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn yul_statements(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::YulStatement))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn yul_switch_cases(&mut self) -> Result<Vec<JsObject>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::YulSwitchCase))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

//
// Separated:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "[Array<cst.Node>, Array<cst.Node>]",
    catch_unwind
)]
pub fn select_separated(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
    env: Env,
) -> Result<Vec<Vec<JsObject>>> {
    let mut selector = Selector::new(node, env);

    let result = match node.kind() {
        RuleKind::VersionPragmaSpecifier => selector.version_pragma_specifier()?,
        RuleKind::ImportDeconstructionSymbols => selector.import_deconstruction_symbols()?,
        RuleKind::UsingDeconstructionSymbols => selector.using_deconstruction_symbols()?,
        RuleKind::InheritanceTypes => selector.inheritance_types()?,
        RuleKind::EnumMembers => selector.enum_members()?,
        RuleKind::Parameters => selector.parameters()?,
        RuleKind::OverridePaths => selector.override_paths()?,
        RuleKind::EventParameters => selector.event_parameters()?,
        RuleKind::ErrorParameters => selector.error_parameters()?,
        RuleKind::AssemblyFlags => selector.assembly_flags()?,
        RuleKind::TupleDeconstructionElements => selector.tuple_deconstruction_elements()?,
        RuleKind::PositionalArguments => selector.positional_arguments()?,
        RuleKind::NamedArguments => selector.named_arguments()?,
        RuleKind::TupleValues => selector.tuple_values()?,
        RuleKind::ArrayValues => selector.array_values()?,
        RuleKind::IdentifierPath => selector.identifier_path()?,
        RuleKind::YulParameters => selector.yul_parameters()?,
        RuleKind::YulReturnVariables => selector.yul_return_variables()?,
        RuleKind::YulArguments => selector.yul_arguments()?,
        RuleKind::YulIdentifierPaths => selector.yul_identifier_paths()?,
        RuleKind::YulIdentifierPath => selector.yul_identifier_path()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn version_pragma_specifier(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_token_with_kind(TokenKind::VersionPragmaValue))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Period))?
        {
            separators.push(separator);

            separated
                .push(self.select(|node| node.is_token_with_kind(TokenKind::VersionPragmaValue))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn import_deconstruction_symbols(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(
            self.select(|node| node.is_rule_with_kind(RuleKind::ImportDeconstructionSymbol))?,
        );

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(
                self.select(|node| node.is_rule_with_kind(RuleKind::ImportDeconstructionSymbol))?,
            );
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn using_deconstruction_symbols(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated
            .push(self.select(|node| node.is_rule_with_kind(RuleKind::UsingDeconstructionSymbol))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(
                self.select(|node| node.is_rule_with_kind(RuleKind::UsingDeconstructionSymbol))?,
            );
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn inheritance_types(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::InheritanceType))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::InheritanceType))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn enum_members(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn parameters(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::Parameter))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::Parameter))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn override_paths(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn event_parameters(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::EventParameter))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::EventParameter))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn error_parameters(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::ErrorParameter))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::ErrorParameter))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn assembly_flags(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated
                .push(self.select(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn tuple_deconstruction_elements(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(
            self.select(|node| node.is_rule_with_kind(RuleKind::TupleDeconstructionElement))?,
        );

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(
                self.select(|node| node.is_rule_with_kind(RuleKind::TupleDeconstructionElement))?,
            );
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn positional_arguments(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn named_arguments(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::NamedArgument))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::NamedArgument))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn tuple_values(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::TupleValue))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::TupleValue))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn array_values(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn identifier_path(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Period))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_parameters(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_token_with_kind(TokenKind::YulIdentifier))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_token_with_kind(TokenKind::YulIdentifier))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_return_variables(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_token_with_kind(TokenKind::YulIdentifier))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_token_with_kind(TokenKind::YulIdentifier))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_arguments(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::YulExpression))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::YulExpression))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_identifier_paths(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_rule_with_kind(RuleKind::YulIdentifierPath))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Comma))?
        {
            separators.push(separator);

            separated
                .push(self.select(|node| node.is_rule_with_kind(RuleKind::YulIdentifierPath))?);
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_identifier_path(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_token_with_kind(TokenKind::YulIdentifier))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Period))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_token_with_kind(TokenKind::YulIdentifier))?);
        }

        Ok(vec![separated, separators])
    }
}

//
// Common:
//

struct Selector {
    env: Env,
    node: Rc<RustRuleNode>,
    index: usize,
}

impl Selector {
    fn new(node: &RuleNode, env: Env) -> Self {
        Self {
            env,
            node: node.0.clone(),
            index: 0,
        }
    }

    fn select(&mut self, filter: impl FnOnce(&RustNode) -> bool) -> Result<JsObject> {
        match self.try_select(filter)? {
            Some(node) => Ok(node),
            None => Error::MissingChild(self.index).into(),
        }
    }

    fn try_select(&mut self, filter: impl FnOnce(&RustNode) -> bool) -> Result<Option<JsObject>> {
        while let Some(child) = self.node.children.get(self.index) {
            match child {
                RustNamedNode {
                    name: _,
                    node: RustNode::Rule(rule),
                } if rule.kind.is_trivia() => {
                    // skip trivia, since it's not part of the AST
                    self.index += 1;
                    continue;
                }
                RustNamedNode {
                    name: _,
                    node: RustNode::Token(token),
                } if matches!(token.kind, TokenKind::SKIPPED) => {
                    return Error::SkippedToken(self.index).into();
                }
                node if filter(node) => {
                    self.index += 1;
                    return Ok(Some(node.to_js(&self.env)));
                }
                _ => {
                    break;
                }
            }
        }

        Ok(None)
    }

    fn finalize(mut self) -> Result<()> {
        if self.try_select(|_| true)?.is_some() {
            return Error::UnexpectedTrailing(self.index - 1).into();
        }

        Ok(())
    }
}

type Result<T> = std::result::Result<T, napi::Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    // Should not theoretically happen, since we're only called from our own generated AST types.
    #[error("Unexpected parent node with RuleKind '{0}'.")]
    UnexpectedParent(RuleKind),

    // Should not theoretically happen, since we're only called from our own generated AST types.
    #[error("Unexpected trailing children at index '{0}'.")]
    UnexpectedTrailing(usize),

    // Should not theoretically happen, unless AST error recovery was changed.
    #[error("Missing child node at index '{0}'.")]
    MissingChild(usize),

    // Can happen if the user decided to use an incorrect/incomplete CST node.
    #[error("Unexpected SKIPPED token at index '{0}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.")]
    SkippedToken(usize),
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(napi::Error::from_reason(error.to_string()))
    }
}
