// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)] // large match statements for all non-terminals
#![allow(clippy::unnecessary_wraps)] // using `Result` for all functions for error handling

use std::rc::Rc;

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cst::{NAPINodeExtensions, NonterminalNode, TerminalNode};
use crate::napi_interface::{EdgeLabel, NonterminalKind, RustEdge, RustNode, RustNonterminalNode};

//
// Sequences:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node | null>",
    catch_unwind
)]
pub fn select_sequence(
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonterminalKind::SourceUnit => selector.source_unit_sequence()?,
        NonterminalKind::PragmaDirective => selector.pragma_directive_sequence()?,
        NonterminalKind::ABICoderPragma => selector.abi_coder_pragma_sequence()?,
        NonterminalKind::ExperimentalPragma => selector.experimental_pragma_sequence()?,
        NonterminalKind::VersionPragma => selector.version_pragma_sequence()?,
        NonterminalKind::VersionRange => selector.version_range_sequence()?,
        NonterminalKind::VersionComparator => selector.version_comparator_sequence()?,
        NonterminalKind::ImportDirective => selector.import_directive_sequence()?,
        NonterminalKind::PathImport => selector.path_import_sequence()?,
        NonterminalKind::NamedImport => selector.named_import_sequence()?,
        NonterminalKind::ImportDeconstruction => selector.import_deconstruction_sequence()?,
        NonterminalKind::ImportDeconstructionSymbol => {
            selector.import_deconstruction_symbol_sequence()?
        }
        NonterminalKind::ImportAlias => selector.import_alias_sequence()?,
        NonterminalKind::UsingDirective => selector.using_directive_sequence()?,
        NonterminalKind::UsingDeconstruction => selector.using_deconstruction_sequence()?,
        NonterminalKind::UsingDeconstructionSymbol => {
            selector.using_deconstruction_symbol_sequence()?
        }
        NonterminalKind::UsingAlias => selector.using_alias_sequence()?,
        NonterminalKind::ContractDefinition => selector.contract_definition_sequence()?,
        NonterminalKind::InheritanceSpecifier => selector.inheritance_specifier_sequence()?,
        NonterminalKind::InheritanceType => selector.inheritance_type_sequence()?,
        NonterminalKind::InterfaceDefinition => selector.interface_definition_sequence()?,
        NonterminalKind::LibraryDefinition => selector.library_definition_sequence()?,
        NonterminalKind::StructDefinition => selector.struct_definition_sequence()?,
        NonterminalKind::StructMember => selector.struct_member_sequence()?,
        NonterminalKind::EnumDefinition => selector.enum_definition_sequence()?,
        NonterminalKind::ConstantDefinition => selector.constant_definition_sequence()?,
        NonterminalKind::StateVariableDefinition => {
            selector.state_variable_definition_sequence()?
        }
        NonterminalKind::StateVariableDefinitionValue => {
            selector.state_variable_definition_value_sequence()?
        }
        NonterminalKind::FunctionDefinition => selector.function_definition_sequence()?,
        NonterminalKind::ParametersDeclaration => selector.parameters_declaration_sequence()?,
        NonterminalKind::Parameter => selector.parameter_sequence()?,
        NonterminalKind::OverrideSpecifier => selector.override_specifier_sequence()?,
        NonterminalKind::OverridePathsDeclaration => {
            selector.override_paths_declaration_sequence()?
        }
        NonterminalKind::ReturnsDeclaration => selector.returns_declaration_sequence()?,
        NonterminalKind::ConstructorDefinition => selector.constructor_definition_sequence()?,
        NonterminalKind::UnnamedFunctionDefinition => {
            selector.unnamed_function_definition_sequence()?
        }
        NonterminalKind::FallbackFunctionDefinition => {
            selector.fallback_function_definition_sequence()?
        }
        NonterminalKind::ReceiveFunctionDefinition => {
            selector.receive_function_definition_sequence()?
        }
        NonterminalKind::ModifierDefinition => selector.modifier_definition_sequence()?,
        NonterminalKind::ModifierInvocation => selector.modifier_invocation_sequence()?,
        NonterminalKind::EventDefinition => selector.event_definition_sequence()?,
        NonterminalKind::EventParametersDeclaration => {
            selector.event_parameters_declaration_sequence()?
        }
        NonterminalKind::EventParameter => selector.event_parameter_sequence()?,
        NonterminalKind::UserDefinedValueTypeDefinition => {
            selector.user_defined_value_type_definition_sequence()?
        }
        NonterminalKind::ErrorDefinition => selector.error_definition_sequence()?,
        NonterminalKind::ErrorParametersDeclaration => {
            selector.error_parameters_declaration_sequence()?
        }
        NonterminalKind::ErrorParameter => selector.error_parameter_sequence()?,
        NonterminalKind::ArrayTypeName => selector.array_type_name_sequence()?,
        NonterminalKind::FunctionType => selector.function_type_sequence()?,
        NonterminalKind::MappingType => selector.mapping_type_sequence()?,
        NonterminalKind::MappingKey => selector.mapping_key_sequence()?,
        NonterminalKind::MappingValue => selector.mapping_value_sequence()?,
        NonterminalKind::AddressType => selector.address_type_sequence()?,
        NonterminalKind::Block => selector.block_sequence()?,
        NonterminalKind::UncheckedBlock => selector.unchecked_block_sequence()?,
        NonterminalKind::ExpressionStatement => selector.expression_statement_sequence()?,
        NonterminalKind::AssemblyStatement => selector.assembly_statement_sequence()?,
        NonterminalKind::AssemblyFlagsDeclaration => {
            selector.assembly_flags_declaration_sequence()?
        }
        NonterminalKind::TupleDeconstructionStatement => {
            selector.tuple_deconstruction_statement_sequence()?
        }
        NonterminalKind::TupleDeconstructionElement => {
            selector.tuple_deconstruction_element_sequence()?
        }
        NonterminalKind::TypedTupleMember => selector.typed_tuple_member_sequence()?,
        NonterminalKind::UntypedTupleMember => selector.untyped_tuple_member_sequence()?,
        NonterminalKind::VariableDeclarationStatement => {
            selector.variable_declaration_statement_sequence()?
        }
        NonterminalKind::VariableDeclarationValue => {
            selector.variable_declaration_value_sequence()?
        }
        NonterminalKind::IfStatement => selector.if_statement_sequence()?,
        NonterminalKind::ElseBranch => selector.else_branch_sequence()?,
        NonterminalKind::ForStatement => selector.for_statement_sequence()?,
        NonterminalKind::WhileStatement => selector.while_statement_sequence()?,
        NonterminalKind::DoWhileStatement => selector.do_while_statement_sequence()?,
        NonterminalKind::ContinueStatement => selector.continue_statement_sequence()?,
        NonterminalKind::BreakStatement => selector.break_statement_sequence()?,
        NonterminalKind::ReturnStatement => selector.return_statement_sequence()?,
        NonterminalKind::EmitStatement => selector.emit_statement_sequence()?,
        NonterminalKind::TryStatement => selector.try_statement_sequence()?,
        NonterminalKind::CatchClause => selector.catch_clause_sequence()?,
        NonterminalKind::CatchClauseError => selector.catch_clause_error_sequence()?,
        NonterminalKind::RevertStatement => selector.revert_statement_sequence()?,
        NonterminalKind::ThrowStatement => selector.throw_statement_sequence()?,
        NonterminalKind::AssignmentExpression => selector.assignment_expression_sequence()?,
        NonterminalKind::ConditionalExpression => selector.conditional_expression_sequence()?,
        NonterminalKind::OrExpression => selector.or_expression_sequence()?,
        NonterminalKind::AndExpression => selector.and_expression_sequence()?,
        NonterminalKind::EqualityExpression => selector.equality_expression_sequence()?,
        NonterminalKind::ComparisonExpression => selector.comparison_expression_sequence()?,
        NonterminalKind::BitwiseOrExpression => selector.bitwise_or_expression_sequence()?,
        NonterminalKind::BitwiseXorExpression => selector.bitwise_xor_expression_sequence()?,
        NonterminalKind::BitwiseAndExpression => selector.bitwise_and_expression_sequence()?,
        NonterminalKind::ShiftExpression => selector.shift_expression_sequence()?,
        NonterminalKind::AdditiveExpression => selector.additive_expression_sequence()?,
        NonterminalKind::MultiplicativeExpression => {
            selector.multiplicative_expression_sequence()?
        }
        NonterminalKind::ExponentiationExpression => {
            selector.exponentiation_expression_sequence()?
        }
        NonterminalKind::PostfixExpression => selector.postfix_expression_sequence()?,
        NonterminalKind::PrefixExpression => selector.prefix_expression_sequence()?,
        NonterminalKind::FunctionCallExpression => selector.function_call_expression_sequence()?,
        NonterminalKind::CallOptionsExpression => selector.call_options_expression_sequence()?,
        NonterminalKind::MemberAccessExpression => selector.member_access_expression_sequence()?,
        NonterminalKind::IndexAccessExpression => selector.index_access_expression_sequence()?,
        NonterminalKind::IndexAccessEnd => selector.index_access_end_sequence()?,
        NonterminalKind::PositionalArgumentsDeclaration => {
            selector.positional_arguments_declaration_sequence()?
        }
        NonterminalKind::NamedArgumentsDeclaration => {
            selector.named_arguments_declaration_sequence()?
        }
        NonterminalKind::NamedArgumentGroup => selector.named_argument_group_sequence()?,
        NonterminalKind::NamedArgument => selector.named_argument_sequence()?,
        NonterminalKind::TypeExpression => selector.type_expression_sequence()?,
        NonterminalKind::NewExpression => selector.new_expression_sequence()?,
        NonterminalKind::TupleExpression => selector.tuple_expression_sequence()?,
        NonterminalKind::TupleValue => selector.tuple_value_sequence()?,
        NonterminalKind::ArrayExpression => selector.array_expression_sequence()?,
        NonterminalKind::HexNumberExpression => selector.hex_number_expression_sequence()?,
        NonterminalKind::DecimalNumberExpression => {
            selector.decimal_number_expression_sequence()?
        }
        NonterminalKind::YulBlock => selector.yul_block_sequence()?,
        NonterminalKind::YulFunctionDefinition => selector.yul_function_definition_sequence()?,
        NonterminalKind::YulParametersDeclaration => {
            selector.yul_parameters_declaration_sequence()?
        }
        NonterminalKind::YulReturnsDeclaration => selector.yul_returns_declaration_sequence()?,
        NonterminalKind::YulVariableDeclarationStatement => {
            selector.yul_variable_declaration_statement_sequence()?
        }
        NonterminalKind::YulVariableDeclarationValue => {
            selector.yul_variable_declaration_value_sequence()?
        }
        NonterminalKind::YulVariableAssignmentStatement => {
            selector.yul_variable_assignment_statement_sequence()?
        }
        NonterminalKind::YulStackAssignmentStatement => {
            selector.yul_stack_assignment_statement_sequence()?
        }
        NonterminalKind::YulColonEqual => selector.yul_colon_equal_sequence()?,
        NonterminalKind::YulIfStatement => selector.yul_if_statement_sequence()?,
        NonterminalKind::YulForStatement => selector.yul_for_statement_sequence()?,
        NonterminalKind::YulSwitchStatement => selector.yul_switch_statement_sequence()?,
        NonterminalKind::YulDefaultCase => selector.yul_default_case_sequence()?,
        NonterminalKind::YulValueCase => selector.yul_value_case_sequence()?,
        NonterminalKind::YulLeaveStatement => selector.yul_leave_statement_sequence()?,
        NonterminalKind::YulBreakStatement => selector.yul_break_statement_sequence()?,
        NonterminalKind::YulContinueStatement => selector.yul_continue_statement_sequence()?,
        NonterminalKind::YulLabel => selector.yul_label_sequence()?,
        NonterminalKind::YulFunctionCallExpression => {
            selector.yul_function_call_expression_sequence()?
        }
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}
impl Selector {
    fn source_unit_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(EdgeLabel::Members)?)])
    }
}

impl Selector {
    fn pragma_directive_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::PragmaKeyword)?),
            Some(self.select(EdgeLabel::Pragma)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn abi_coder_pragma_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AbicoderKeyword)?),
            Some(self.select(EdgeLabel::Version)?),
        ])
    }
}

impl Selector {
    fn experimental_pragma_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ExperimentalKeyword)?),
            Some(self.select(EdgeLabel::Feature)?),
        ])
    }
}

impl Selector {
    fn version_pragma_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::SolidityKeyword)?),
            Some(self.select(EdgeLabel::Sets)?),
        ])
    }
}

impl Selector {
    fn version_range_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn version_comparator_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::Operand)?),
        ])
    }
}

impl Selector {
    fn import_directive_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ImportKeyword)?),
            Some(self.select(EdgeLabel::Clause)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn path_import_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Path)?),
            self.try_select(EdgeLabel::Alias),
        ])
    }
}

impl Selector {
    fn named_import_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Asterisk)?),
            Some(self.select(EdgeLabel::Alias)?),
            Some(self.select(EdgeLabel::FromKeyword)?),
            Some(self.select(EdgeLabel::Path)?),
        ])
    }
}

impl Selector {
    fn import_deconstruction_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Symbols)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
            Some(self.select(EdgeLabel::FromKeyword)?),
            Some(self.select(EdgeLabel::Path)?),
        ])
    }
}

impl Selector {
    fn import_deconstruction_symbol_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Alias),
        ])
    }
}

impl Selector {
    fn import_alias_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AsKeyword)?),
            Some(self.select(EdgeLabel::Identifier)?),
        ])
    }
}

impl Selector {
    fn using_directive_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::UsingKeyword)?),
            Some(self.select(EdgeLabel::Clause)?),
            Some(self.select(EdgeLabel::ForKeyword)?),
            Some(self.select(EdgeLabel::Target)?),
            self.try_select(EdgeLabel::GlobalKeyword),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn using_deconstruction_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Symbols)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn using_deconstruction_symbol_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Alias),
        ])
    }
}

impl Selector {
    fn using_alias_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AsKeyword)?),
            Some(self.select(EdgeLabel::Operator)?),
        ])
    }
}

impl Selector {
    fn contract_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(EdgeLabel::AbstractKeyword),
            Some(self.select(EdgeLabel::ContractKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Inheritance),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn inheritance_specifier_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::IsKeyword)?),
            Some(self.select(EdgeLabel::Types)?),
        ])
    }
}

impl Selector {
    fn inheritance_type_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::Arguments),
        ])
    }
}

impl Selector {
    fn interface_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::InterfaceKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Inheritance),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn library_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LibraryKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn struct_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::StructKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn struct_member_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn enum_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::EnumKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn constant_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            Some(self.select(EdgeLabel::ConstantKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Equal)?),
            Some(self.select(EdgeLabel::Value)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn state_variable_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Value),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn state_variable_definition_value_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Equal)?),
            Some(self.select(EdgeLabel::Value)?),
        ])
    }
}

impl Selector {
    fn function_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::FunctionKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            self.try_select(EdgeLabel::Returns),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn parameters_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn parameter_sequence(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::StorageLocation),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Selector {
    fn override_specifier_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OverrideKeyword)?),
            self.try_select(EdgeLabel::Overridden),
        ])
    }
}

impl Selector {
    fn override_paths_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Paths)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn returns_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ReturnsKeyword)?),
            Some(self.select(EdgeLabel::Variables)?),
        ])
    }
}

impl Selector {
    fn constructor_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ConstructorKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn unnamed_function_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::FunctionKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn fallback_function_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::FallbackKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            self.try_select(EdgeLabel::Returns),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn receive_function_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ReceiveKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn modifier_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ModifierKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Parameters),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn modifier_invocation_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Arguments),
        ])
    }
}

impl Selector {
    fn event_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::EventKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Parameters)?),
            self.try_select(EdgeLabel::AnonymousKeyword),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn event_parameters_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn event_parameter_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::IndexedKeyword),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Selector {
    fn user_defined_value_type_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::IsKeyword)?),
            Some(self.select(EdgeLabel::ValueType)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn error_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ErrorKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn error_parameters_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn error_parameter_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Selector {
    fn array_type_name_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::OpenBracket)?),
            self.try_select(EdgeLabel::Index),
            Some(self.select(EdgeLabel::CloseBracket)?),
        ])
    }
}

impl Selector {
    fn function_type_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::FunctionKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            self.try_select(EdgeLabel::Returns),
        ])
    }
}

impl Selector {
    fn mapping_type_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::MappingKeyword)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::KeyType)?),
            Some(self.select(EdgeLabel::EqualGreaterThan)?),
            Some(self.select(EdgeLabel::ValueType)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn mapping_key_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::KeyType)?),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Selector {
    fn mapping_value_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Selector {
    fn address_type_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AddressKeyword)?),
            self.try_select(EdgeLabel::PayableKeyword),
        ])
    }
}

impl Selector {
    fn block_sequence(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Statements)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn unchecked_block_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::UncheckedKeyword)?),
            Some(self.select(EdgeLabel::Block)?),
        ])
    }
}

impl Selector {
    fn expression_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Expression)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn assembly_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AssemblyKeyword)?),
            self.try_select(EdgeLabel::Label),
            self.try_select(EdgeLabel::Flags),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn assembly_flags_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Flags)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn tuple_deconstruction_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(EdgeLabel::VarKeyword),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Elements)?),
            Some(self.select(EdgeLabel::CloseParen)?),
            Some(self.select(EdgeLabel::Equal)?),
            Some(self.select(EdgeLabel::Expression)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn tuple_deconstruction_element_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![self.try_select(EdgeLabel::Member)])
    }
}

impl Selector {
    fn typed_tuple_member_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::StorageLocation),
            Some(self.select(EdgeLabel::Name)?),
        ])
    }
}

impl Selector {
    fn untyped_tuple_member_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(EdgeLabel::StorageLocation),
            Some(self.select(EdgeLabel::Name)?),
        ])
    }
}

impl Selector {
    fn variable_declaration_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::VariableType)?),
            self.try_select(EdgeLabel::StorageLocation),
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Value),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn variable_declaration_value_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Equal)?),
            Some(self.select(EdgeLabel::Expression)?),
        ])
    }
}

impl Selector {
    fn if_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::IfKeyword)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Condition)?),
            Some(self.select(EdgeLabel::CloseParen)?),
            Some(self.select(EdgeLabel::Body)?),
            self.try_select(EdgeLabel::ElseBranch),
        ])
    }
}

impl Selector {
    fn else_branch_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ElseKeyword)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn for_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ForKeyword)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Initialization)?),
            Some(self.select(EdgeLabel::Condition)?),
            self.try_select(EdgeLabel::Iterator),
            Some(self.select(EdgeLabel::CloseParen)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn while_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::WhileKeyword)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Condition)?),
            Some(self.select(EdgeLabel::CloseParen)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn do_while_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::DoKeyword)?),
            Some(self.select(EdgeLabel::Body)?),
            Some(self.select(EdgeLabel::WhileKeyword)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Condition)?),
            Some(self.select(EdgeLabel::CloseParen)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn continue_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ContinueKeyword)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn break_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::BreakKeyword)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn return_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ReturnKeyword)?),
            self.try_select(EdgeLabel::Expression),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn emit_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::EmitKeyword)?),
            Some(self.select(EdgeLabel::Event)?),
            Some(self.select(EdgeLabel::Arguments)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn try_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TryKeyword)?),
            Some(self.select(EdgeLabel::Expression)?),
            self.try_select(EdgeLabel::Returns),
            Some(self.select(EdgeLabel::Body)?),
            Some(self.select(EdgeLabel::CatchClauses)?),
        ])
    }
}

impl Selector {
    fn catch_clause_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::CatchKeyword)?),
            self.try_select(EdgeLabel::Error),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn catch_clause_error_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(EdgeLabel::Name),
            Some(self.select(EdgeLabel::Parameters)?),
        ])
    }
}

impl Selector {
    fn revert_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::RevertKeyword)?),
            self.try_select(EdgeLabel::Error),
            Some(self.select(EdgeLabel::Arguments)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn throw_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ThrowKeyword)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Selector {
    fn assignment_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn conditional_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::QuestionMark)?),
            Some(self.select(EdgeLabel::TrueExpression)?),
            Some(self.select(EdgeLabel::Colon)?),
            Some(self.select(EdgeLabel::FalseExpression)?),
        ])
    }
}

impl Selector {
    fn or_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn and_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn equality_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn comparison_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn bitwise_or_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn bitwise_xor_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn bitwise_and_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn shift_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn additive_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn multiplicative_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn exponentiation_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Selector {
    fn postfix_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::Operator)?),
        ])
    }
}

impl Selector {
    fn prefix_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::Operand)?),
        ])
    }
}

impl Selector {
    fn function_call_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::Arguments)?),
        ])
    }
}

impl Selector {
    fn call_options_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Options)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn member_access_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::Period)?),
            Some(self.select(EdgeLabel::Member)?),
        ])
    }
}

impl Selector {
    fn index_access_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::OpenBracket)?),
            self.try_select(EdgeLabel::Start),
            self.try_select(EdgeLabel::End),
            Some(self.select(EdgeLabel::CloseBracket)?),
        ])
    }
}

impl Selector {
    fn index_access_end_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Colon)?),
            self.try_select(EdgeLabel::End),
        ])
    }
}

impl Selector {
    fn positional_arguments_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Arguments)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn named_arguments_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            self.try_select(EdgeLabel::Arguments),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn named_argument_group_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Arguments)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn named_argument_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Colon)?),
            Some(self.select(EdgeLabel::Value)?),
        ])
    }
}

impl Selector {
    fn type_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeKeyword)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::TypeName)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn new_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::NewKeyword)?),
            Some(self.select(EdgeLabel::TypeName)?),
        ])
    }
}

impl Selector {
    fn tuple_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Items)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn tuple_value_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![self.try_select(EdgeLabel::Expression)])
    }
}

impl Selector {
    fn array_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBracket)?),
            Some(self.select(EdgeLabel::Items)?),
            Some(self.select(EdgeLabel::CloseBracket)?),
        ])
    }
}

impl Selector {
    fn hex_number_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Literal)?),
            self.try_select(EdgeLabel::Unit),
        ])
    }
}

impl Selector {
    fn decimal_number_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Literal)?),
            self.try_select(EdgeLabel::Unit),
        ])
    }
}

impl Selector {
    fn yul_block_sequence(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Statements)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Selector {
    fn yul_function_definition_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::FunctionKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Parameters)?),
            self.try_select(EdgeLabel::Returns),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn yul_parameters_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Selector {
    fn yul_returns_declaration_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::MinusGreaterThan)?),
            Some(self.select(EdgeLabel::Variables)?),
        ])
    }
}

impl Selector {
    fn yul_variable_declaration_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LetKeyword)?),
            Some(self.select(EdgeLabel::Variables)?),
            self.try_select(EdgeLabel::Value),
        ])
    }
}

impl Selector {
    fn yul_variable_declaration_value_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Assignment)?),
            Some(self.select(EdgeLabel::Expression)?),
        ])
    }
}

impl Selector {
    fn yul_variable_assignment_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Variables)?),
            Some(self.select(EdgeLabel::Assignment)?),
            Some(self.select(EdgeLabel::Expression)?),
        ])
    }
}

impl Selector {
    fn yul_stack_assignment_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Assignment)?),
            Some(self.select(EdgeLabel::Expression)?),
        ])
    }
}

impl Selector {
    fn yul_colon_equal_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Colon)?),
            Some(self.select(EdgeLabel::Equal)?),
        ])
    }
}

impl Selector {
    fn yul_if_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::IfKeyword)?),
            Some(self.select(EdgeLabel::Condition)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn yul_for_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ForKeyword)?),
            Some(self.select(EdgeLabel::Initialization)?),
            Some(self.select(EdgeLabel::Condition)?),
            Some(self.select(EdgeLabel::Iterator)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn yul_switch_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::SwitchKeyword)?),
            Some(self.select(EdgeLabel::Expression)?),
            Some(self.select(EdgeLabel::Cases)?),
        ])
    }
}

impl Selector {
    fn yul_default_case_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::DefaultKeyword)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn yul_value_case_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::CaseKeyword)?),
            Some(self.select(EdgeLabel::Value)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Selector {
    fn yul_leave_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(EdgeLabel::LeaveKeyword)?)])
    }
}

impl Selector {
    fn yul_break_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(EdgeLabel::BreakKeyword)?)])
    }
}

impl Selector {
    fn yul_continue_statement_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(EdgeLabel::ContinueKeyword)?)])
    }
}

impl Selector {
    fn yul_label_sequence(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Label)?),
            Some(self.select(EdgeLabel::Colon)?),
        ])
    }
}

impl Selector {
    fn yul_function_call_expression_sequence(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Arguments)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}
//
// Choices:
//

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn select_choice(
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Either<NonterminalNode, TerminalNode>> {
    let mut selector = Selector::new(node);

    let variant = selector.select(EdgeLabel::Variant)?;

    selector.finalize()?;
    Ok(variant)
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
    let mut selector = Selector::new(node);

    let mut items = vec![];

    while let Some(item) = selector.try_select(EdgeLabel::Item) {
        items.push(item);
    }

    selector.finalize()?;
    Ok(items)
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let mut items = vec![];
    let mut separators = vec![];

    if let Some(first) = selector.try_select(EdgeLabel::Item) {
        items.push(first);

        while let Some(separator) = selector.try_select(EdgeLabel::Separator) {
            separators.push(separator);

            items.push(selector.select(EdgeLabel::Item)?);
        }
    }

    selector.finalize()?;
    Ok(vec![items, separators])
}

//
// Common:
//

struct Selector {
    node: Rc<RustNonterminalNode>,
    index: usize,
}

impl Selector {
    fn new(node: &NonterminalNode) -> Self {
        Self {
            node: Rc::clone(&node.0),
            index: 0,
        }
    }

    fn select(&mut self, target_label: EdgeLabel) -> Result<Either<NonterminalNode, TerminalNode>> {
        match self.try_select(target_label) {
            Some(node) => Ok(node),
            None => Error::MissingChild(target_label).into(),
        }
    }

    fn try_select(
        &mut self,
        target_label: EdgeLabel,
    ) -> Option<Either<NonterminalNode, TerminalNode>> {
        let (label, node) = self.current()?;

        if label == target_label {
            self.index += 1;
            Some(node.clone().into_js_either_node())
        } else {
            None
        }
    }

    fn current(&mut self) -> Option<(EdgeLabel, RustNode)> {
        loop {
            let RustEdge { label, node } = self.node.children.get(self.index)?;

            match label {
                // Skip unlabeled nodes:
                | None
                // Skip trivia:
                | Some(EdgeLabel::LeadingTrivia | EdgeLabel::TrailingTrivia) => {
                    self.index += 1;
                    continue;
                }
                // Otherwise, return the edge:
                Some(other_label) => {
                    return Some((*other_label, node.clone()));
                }
            }
        }
    }

    fn finalize(mut self) -> Result<()> {
        match self.current() {
            Some((label, _)) => Error::UnrecognizedChild(label).into(),
            _ => Ok(()),
        }
    }
}

type Result<T> = std::result::Result<T, napi::Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    // Should not theoretically happen, since we're only called from our own generated AST types.
    #[error("Unexpected parent node with NonterminalKind '{0}'.")]
    UnexpectedParent(NonterminalKind),

    #[error("Unrecognized child with label '{0}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.")]
    UnrecognizedChild(EdgeLabel),

    #[error("Missing child with label '{0}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.")]
    MissingChild(EdgeLabel),
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(napi::Error::from_reason(error.to_string()))
    }
}
