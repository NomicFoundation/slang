// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cst::{NAPINodeExtensions, NonTerminalNode, TerminalNode};
use crate::napi_interface::{
    NonTerminalKind, RustEdge, RustNode, RustNonTerminalNode, TerminalKind,
};

//
// Sequences:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node | null>",
    catch_unwind
)]
pub fn select_sequence(
    #[napi(ts_arg_type = "cst.NonTerminalNode")] node: &NonTerminalNode,
) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonTerminalKind::SourceUnit => selector.source_unit()?,
        NonTerminalKind::PragmaDirective => selector.pragma_directive()?,
        NonTerminalKind::ABICoderPragma => selector.abi_coder_pragma()?,
        NonTerminalKind::ExperimentalPragma => selector.experimental_pragma()?,
        NonTerminalKind::VersionPragma => selector.version_pragma()?,
        NonTerminalKind::VersionRange => selector.version_range()?,
        NonTerminalKind::VersionComparator => selector.version_comparator()?,
        NonTerminalKind::ImportDirective => selector.import_directive()?,
        NonTerminalKind::PathImport => selector.path_import()?,
        NonTerminalKind::NamedImport => selector.named_import()?,
        NonTerminalKind::ImportDeconstruction => selector.import_deconstruction()?,
        NonTerminalKind::ImportDeconstructionSymbol => selector.import_deconstruction_symbol()?,
        NonTerminalKind::ImportAlias => selector.import_alias()?,
        NonTerminalKind::UsingDirective => selector.using_directive()?,
        NonTerminalKind::UsingDeconstruction => selector.using_deconstruction()?,
        NonTerminalKind::UsingDeconstructionSymbol => selector.using_deconstruction_symbol()?,
        NonTerminalKind::UsingAlias => selector.using_alias()?,
        NonTerminalKind::ContractDefinition => selector.contract_definition()?,
        NonTerminalKind::InheritanceSpecifier => selector.inheritance_specifier()?,
        NonTerminalKind::InheritanceType => selector.inheritance_type()?,
        NonTerminalKind::InterfaceDefinition => selector.interface_definition()?,
        NonTerminalKind::LibraryDefinition => selector.library_definition()?,
        NonTerminalKind::StructDefinition => selector.struct_definition()?,
        NonTerminalKind::StructMember => selector.struct_member()?,
        NonTerminalKind::EnumDefinition => selector.enum_definition()?,
        NonTerminalKind::ConstantDefinition => selector.constant_definition()?,
        NonTerminalKind::StateVariableDefinition => selector.state_variable_definition()?,
        NonTerminalKind::StateVariableDefinitionValue => {
            selector.state_variable_definition_value()?
        }
        NonTerminalKind::FunctionDefinition => selector.function_definition()?,
        NonTerminalKind::ParametersDeclaration => selector.parameters_declaration()?,
        NonTerminalKind::Parameter => selector.parameter()?,
        NonTerminalKind::OverrideSpecifier => selector.override_specifier()?,
        NonTerminalKind::OverridePathsDeclaration => selector.override_paths_declaration()?,
        NonTerminalKind::ReturnsDeclaration => selector.returns_declaration()?,
        NonTerminalKind::ConstructorDefinition => selector.constructor_definition()?,
        NonTerminalKind::UnnamedFunctionDefinition => selector.unnamed_function_definition()?,
        NonTerminalKind::FallbackFunctionDefinition => selector.fallback_function_definition()?,
        NonTerminalKind::ReceiveFunctionDefinition => selector.receive_function_definition()?,
        NonTerminalKind::ModifierDefinition => selector.modifier_definition()?,
        NonTerminalKind::ModifierInvocation => selector.modifier_invocation()?,
        NonTerminalKind::EventDefinition => selector.event_definition()?,
        NonTerminalKind::EventParametersDeclaration => selector.event_parameters_declaration()?,
        NonTerminalKind::EventParameter => selector.event_parameter()?,
        NonTerminalKind::UserDefinedValueTypeDefinition => {
            selector.user_defined_value_type_definition()?
        }
        NonTerminalKind::ErrorDefinition => selector.error_definition()?,
        NonTerminalKind::ErrorParametersDeclaration => selector.error_parameters_declaration()?,
        NonTerminalKind::ErrorParameter => selector.error_parameter()?,
        NonTerminalKind::ArrayTypeName => selector.array_type_name()?,
        NonTerminalKind::FunctionType => selector.function_type()?,
        NonTerminalKind::MappingType => selector.mapping_type()?,
        NonTerminalKind::MappingKey => selector.mapping_key()?,
        NonTerminalKind::MappingValue => selector.mapping_value()?,
        NonTerminalKind::AddressType => selector.address_type()?,
        NonTerminalKind::Block => selector.block()?,
        NonTerminalKind::UncheckedBlock => selector.unchecked_block()?,
        NonTerminalKind::ExpressionStatement => selector.expression_statement()?,
        NonTerminalKind::AssemblyStatement => selector.assembly_statement()?,
        NonTerminalKind::AssemblyFlagsDeclaration => selector.assembly_flags_declaration()?,
        NonTerminalKind::TupleDeconstructionStatement => {
            selector.tuple_deconstruction_statement()?
        }
        NonTerminalKind::TupleDeconstructionElement => selector.tuple_deconstruction_element()?,
        NonTerminalKind::TypedTupleMember => selector.typed_tuple_member()?,
        NonTerminalKind::UntypedTupleMember => selector.untyped_tuple_member()?,
        NonTerminalKind::VariableDeclarationStatement => {
            selector.variable_declaration_statement()?
        }
        NonTerminalKind::VariableDeclarationValue => selector.variable_declaration_value()?,
        NonTerminalKind::IfStatement => selector.if_statement()?,
        NonTerminalKind::ElseBranch => selector.else_branch()?,
        NonTerminalKind::ForStatement => selector.for_statement()?,
        NonTerminalKind::WhileStatement => selector.while_statement()?,
        NonTerminalKind::DoWhileStatement => selector.do_while_statement()?,
        NonTerminalKind::ContinueStatement => selector.continue_statement()?,
        NonTerminalKind::BreakStatement => selector.break_statement()?,
        NonTerminalKind::ReturnStatement => selector.return_statement()?,
        NonTerminalKind::EmitStatement => selector.emit_statement()?,
        NonTerminalKind::TryStatement => selector.try_statement()?,
        NonTerminalKind::CatchClause => selector.catch_clause()?,
        NonTerminalKind::CatchClauseError => selector.catch_clause_error()?,
        NonTerminalKind::RevertStatement => selector.revert_statement()?,
        NonTerminalKind::ThrowStatement => selector.throw_statement()?,
        NonTerminalKind::AssignmentExpression => selector.assignment_expression()?,
        NonTerminalKind::ConditionalExpression => selector.conditional_expression()?,
        NonTerminalKind::OrExpression => selector.or_expression()?,
        NonTerminalKind::AndExpression => selector.and_expression()?,
        NonTerminalKind::EqualityExpression => selector.equality_expression()?,
        NonTerminalKind::ComparisonExpression => selector.comparison_expression()?,
        NonTerminalKind::BitwiseOrExpression => selector.bitwise_or_expression()?,
        NonTerminalKind::BitwiseXorExpression => selector.bitwise_xor_expression()?,
        NonTerminalKind::BitwiseAndExpression => selector.bitwise_and_expression()?,
        NonTerminalKind::ShiftExpression => selector.shift_expression()?,
        NonTerminalKind::AdditiveExpression => selector.additive_expression()?,
        NonTerminalKind::MultiplicativeExpression => selector.multiplicative_expression()?,
        NonTerminalKind::ExponentiationExpression => selector.exponentiation_expression()?,
        NonTerminalKind::PostfixExpression => selector.postfix_expression()?,
        NonTerminalKind::PrefixExpression => selector.prefix_expression()?,
        NonTerminalKind::FunctionCallExpression => selector.function_call_expression()?,
        NonTerminalKind::CallOptionsExpression => selector.call_options_expression()?,
        NonTerminalKind::MemberAccessExpression => selector.member_access_expression()?,
        NonTerminalKind::IndexAccessExpression => selector.index_access_expression()?,
        NonTerminalKind::IndexAccessEnd => selector.index_access_end()?,
        NonTerminalKind::PositionalArgumentsDeclaration => {
            selector.positional_arguments_declaration()?
        }
        NonTerminalKind::NamedArgumentsDeclaration => selector.named_arguments_declaration()?,
        NonTerminalKind::NamedArgumentGroup => selector.named_argument_group()?,
        NonTerminalKind::NamedArgument => selector.named_argument()?,
        NonTerminalKind::TypeExpression => selector.type_expression()?,
        NonTerminalKind::NewExpression => selector.new_expression()?,
        NonTerminalKind::TupleExpression => selector.tuple_expression()?,
        NonTerminalKind::TupleValue => selector.tuple_value()?,
        NonTerminalKind::ArrayExpression => selector.array_expression()?,
        NonTerminalKind::HexNumberExpression => selector.hex_number_expression()?,
        NonTerminalKind::DecimalNumberExpression => selector.decimal_number_expression()?,
        NonTerminalKind::YulBlock => selector.yul_block()?,
        NonTerminalKind::YulFunctionDefinition => selector.yul_function_definition()?,
        NonTerminalKind::YulParametersDeclaration => selector.yul_parameters_declaration()?,
        NonTerminalKind::YulReturnsDeclaration => selector.yul_returns_declaration()?,
        NonTerminalKind::YulVariableDeclarationStatement => {
            selector.yul_variable_declaration_statement()?
        }
        NonTerminalKind::YulVariableDeclarationValue => {
            selector.yul_variable_declaration_value()?
        }
        NonTerminalKind::YulAssignmentStatement => selector.yul_assignment_statement()?,
        NonTerminalKind::YulColonAndEqual => selector.yul_colon_and_equal()?,
        NonTerminalKind::YulIfStatement => selector.yul_if_statement()?,
        NonTerminalKind::YulForStatement => selector.yul_for_statement()?,
        NonTerminalKind::YulSwitchStatement => selector.yul_switch_statement()?,
        NonTerminalKind::YulDefaultCase => selector.yul_default_case()?,
        NonTerminalKind::YulValueCase => selector.yul_value_case()?,
        NonTerminalKind::YulLeaveStatement => selector.yul_leave_statement()?,
        NonTerminalKind::YulBreakStatement => selector.yul_break_statement()?,
        NonTerminalKind::YulContinueStatement => selector.yul_continue_statement()?,
        NonTerminalKind::YulLabel => selector.yul_label()?,
        NonTerminalKind::YulFunctionCallExpression => selector.yul_function_call_expression()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}
impl Selector {
    fn source_unit(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::SourceUnitMembers)
        })?)])
    }
}

impl Selector {
    fn pragma_directive(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::PragmaKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Pragma))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn abi_coder_pragma(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AbicoderKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn experimental_pragma(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_terminal_with_kind(TerminalKind::ExperimentalKeyword))?,
            ),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ExperimentalFeature)
            })?),
        ])
    }
}

impl Selector {
    fn version_pragma(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::SolidityKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::VersionExpressionSets)
            })?),
        ])
    }
}

impl Selector {
    fn version_range(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::VersionExpression)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Minus))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::VersionExpression)
            })?),
        ])
    }
}

impl Selector {
    fn version_comparator(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Caret))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::VersionExpression)
            })?),
        ])
    }
}

impl Selector {
    fn import_directive(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ImportKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ImportClause))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn path_import(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::StringLiteral))?,
            ),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ImportAlias))?,
        ])
    }
}

impl Selector {
    fn named_import(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Asterisk))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ImportAlias))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FromKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::StringLiteral))?,
            ),
        ])
    }
}

impl Selector {
    fn import_deconstruction(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ImportDeconstructionSymbols)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FromKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::StringLiteral))?,
            ),
        ])
    }
}

impl Selector {
    fn import_deconstruction_symbol(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ImportAlias))?,
        ])
    }
}

impl Selector {
    fn import_alias(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AsKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn using_directive(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::UsingKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::UsingClause))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ForKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::UsingTarget))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::GlobalKeyword))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn using_deconstruction(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::UsingDeconstructionSymbols)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn using_deconstruction_symbol(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::IdentifierPath))?,
            ),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::UsingAlias))?,
        ])
    }
}

impl Selector {
    fn using_alias(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AsKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::UsingOperator))?,
            ),
        ])
    }
}

impl Selector {
    fn contract_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::AbstractKeyword))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ContractKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::InheritanceSpecifier)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::ContractMembers)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn inheritance_specifier(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::IsKeyword))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::InheritanceTypes)
                })?,
            ),
        ])
    }
}

impl Selector {
    fn inheritance_type(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::IdentifierPath))?,
            ),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ArgumentsDeclaration)
            })?,
        ])
    }
}

impl Selector {
    fn interface_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::InterfaceKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::InheritanceSpecifier)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::InterfaceMembers)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn library_definition(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::LibraryKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::LibraryMembers))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn struct_definition(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::StructKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::StructMembers))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn struct_member(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn enum_definition(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EnumKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::EnumMembers))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn constant_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ConstantKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn state_variable_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::StateVariableAttributes)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::StateVariableDefinitionValue)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn state_variable_definition_value(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FunctionKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::FunctionName))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::FunctionAttributes)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ReturnsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn parameters_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Parameters))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn parameter(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::StorageLocation)
            })?,
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn override_specifier(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OverrideKeyword))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::OverridePathsDeclaration)
            })?,
        ])
    }
}

impl Selector {
    fn override_paths_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::OverridePaths))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn returns_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ReturnsKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ParametersDeclaration)
            })?),
        ])
    }
}

impl Selector {
    fn constructor_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ConstructorKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ConstructorAttributes)
            })?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Block))?),
        ])
    }
}

impl Selector {
    fn unnamed_function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FunctionKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::UnnamedFunctionAttributes)
            })?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn fallback_function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FallbackKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::FallbackFunctionAttributes)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ReturnsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn receive_function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ReceiveKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ReceiveFunctionAttributes)
            })?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn modifier_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ModifierKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ParametersDeclaration)
            })?,
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ModifierAttributes)
            })?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn modifier_invocation(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::IdentifierPath))?,
            ),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ArgumentsDeclaration)
            })?,
        ])
    }
}

impl Selector {
    fn event_definition(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EventKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::EventParametersDeclaration)
            })?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::AnonymousKeyword))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn event_parameters_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::EventParameters)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn event_parameter(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::IndexedKeyword))?,
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn user_defined_value_type_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::TypeKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::IsKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ElementaryType))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn error_definition(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ErrorKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ErrorParametersDeclaration)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn error_parameters_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::ErrorParameters)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn error_parameter(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn array_type_name(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBracket))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn function_type(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FunctionKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::FunctionTypeAttributes)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ReturnsDeclaration)
            })?,
        ])
    }
}

impl Selector {
    fn mapping_type(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::MappingKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::MappingKey))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EqualGreaterThan))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::MappingValue))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn mapping_key(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::MappingKeyType))?,
            ),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn mapping_value(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn address_type(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AddressKeyword))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::PayableKeyword))?,
        ])
    }
}

impl Selector {
    fn block(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Statements))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn unchecked_block(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::UncheckedKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Block))?),
        ])
    }
}

impl Selector {
    fn expression_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn assembly_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AssemblyKeyword))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::StringLiteral))?,
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::AssemblyFlagsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn assembly_flags_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::AssemblyFlags))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn tuple_deconstruction_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::VarKeyword))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::TupleDeconstructionElements)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn tuple_deconstruction_element(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::TupleMember)
        })?])
    }
}

impl Selector {
    fn typed_tuple_member(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::StorageLocation)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn untyped_tuple_member(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::StorageLocation)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn variable_declaration_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::VariableDeclarationType)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::StorageLocation)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::VariableDeclarationValue)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn variable_declaration_value(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn if_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::IfKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Statement))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ElseBranch))?,
        ])
    }
}

impl Selector {
    fn else_branch(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ElseKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Statement))?),
        ])
    }
}

impl Selector {
    fn for_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ForKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ForStatementInitialization)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ForStatementCondition)
            })?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Statement))?),
        ])
    }
}

impl Selector {
    fn while_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::WhileKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Statement))?),
        ])
    }
}

impl Selector {
    fn do_while_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::DoKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Statement))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::WhileKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn continue_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ContinueKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn break_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::BreakKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn return_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ReturnKeyword))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn emit_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EmitKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::IdentifierPath))?,
            ),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ArgumentsDeclaration)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn try_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::TryKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ReturnsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Block))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::CatchClauses))?),
        ])
    }
}

impl Selector {
    fn catch_clause(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CatchKeyword))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::CatchClauseError)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Block))?),
        ])
    }
}

impl Selector {
    fn catch_clause_error(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ParametersDeclaration)
            })?),
        ])
    }
}

impl Selector {
    fn revert_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::RevertKeyword))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::IdentifierPath))?,
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ArgumentsDeclaration)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn throw_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ThrowKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn assignment_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn conditional_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::QuestionMark))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn or_expression(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::BarBar))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn and_expression(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AmpersandAmpersand))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn equality_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EqualEqual))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn comparison_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::LessThan))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn bitwise_or_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Bar))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn bitwise_xor_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Caret))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn bitwise_and_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Ampersand))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn shift_expression(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::LessThanLessThan))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn additive_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Plus))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn multiplicative_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Asterisk))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn exponentiation_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AsteriskAsterisk))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn postfix_expression(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::PlusPlus))?),
        ])
    }
}

impl Selector {
    fn prefix_expression(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::PlusPlus))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn function_call_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::ArgumentsDeclaration)
            })?),
        ])
    }
}

impl Selector {
    fn call_options_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::CallOptions))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn member_access_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Period))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::MemberAccess))?),
        ])
    }
}

impl Selector {
    fn index_access_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBracket))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?,
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::IndexAccessEnd))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn index_access_end(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?,
        ])
    }
}

impl Selector {
    fn positional_arguments_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::PositionalArguments)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn named_arguments_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::NamedArgumentGroup)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn named_argument_group(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::NamedArguments))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn named_argument(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn type_expression(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::TypeKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn new_expression(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::NewKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TypeName))?),
        ])
    }
}

impl Selector {
    fn tuple_expression(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TupleValues))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn tuple_value(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::Expression)
        })?])
    }
}

impl Selector {
    fn array_expression(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBracket))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ArrayValues))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn hex_number_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::HexLiteral))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::NumberUnit))?,
        ])
    }
}

impl Selector {
    fn decimal_number_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::DecimalLiteral))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::NumberUnit))?,
        ])
    }
}

impl Selector {
    fn yul_block(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulStatements))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn yul_function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulFunctionKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::YulParametersDeclaration)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::YulReturnsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_parameters_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulParameters))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn yul_returns_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::MinusGreaterThan))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::YulReturnVariables)
            })?),
        ])
    }
}

impl Selector {
    fn yul_variable_declaration_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulLetKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::YulVariableDeclarationValue)
            })?,
        ])
    }
}

impl Selector {
    fn yul_variable_declaration_value(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::YulAssignmentOperator)
            })?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulExpression))?,
            ),
        ])
    }
}

impl Selector {
    fn yul_assignment_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulPaths))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonTerminalKind::YulAssignmentOperator)
            })?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulExpression))?,
            ),
        ])
    }
}

impl Selector {
    fn yul_colon_and_equal(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
        ])
    }
}

impl Selector {
    fn yul_if_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIfKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulExpression))?,
            ),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_for_statement(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulForKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulBlock))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulExpression))?,
            ),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulBlock))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_switch_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulSwitchKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulExpression))?,
            ),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulSwitchCases))?,
            ),
        ])
    }
}

impl Selector {
    fn yul_default_case(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulDefaultKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_value_case(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulCaseKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulLiteral))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_leave_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_terminal_with_kind(TerminalKind::YulLeaveKeyword)
        })?)])
    }
}

impl Selector {
    fn yul_break_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_terminal_with_kind(TerminalKind::YulBreakKeyword)
        })?)])
    }
}

impl Selector {
    fn yul_continue_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_terminal_with_kind(TerminalKind::YulContinueKeyword)
        })?)])
    }
}

impl Selector {
    fn yul_label(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
        ])
    }
}

impl Selector {
    fn yul_function_call_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulExpression))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulArguments))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}
//
// Choices:
//

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn select_choice(
    #[napi(ts_arg_type = "cst.NonTerminalNode")] node: &NonTerminalNode,
) -> Result<Either<NonTerminalNode, TerminalNode>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonTerminalKind::SourceUnitMember => selector.source_unit_member()?,
        NonTerminalKind::Pragma => selector.pragma()?,
        NonTerminalKind::ExperimentalFeature => selector.experimental_feature()?,
        NonTerminalKind::VersionExpression => selector.version_expression()?,
        NonTerminalKind::ImportClause => selector.import_clause()?,
        NonTerminalKind::UsingClause => selector.using_clause()?,
        NonTerminalKind::UsingOperator => selector.using_operator()?,
        NonTerminalKind::UsingTarget => selector.using_target()?,
        NonTerminalKind::ContractMember => selector.contract_member()?,
        NonTerminalKind::StateVariableAttribute => selector.state_variable_attribute()?,
        NonTerminalKind::FunctionName => selector.function_name()?,
        NonTerminalKind::FunctionAttribute => selector.function_attribute()?,
        NonTerminalKind::FunctionBody => selector.function_body()?,
        NonTerminalKind::ConstructorAttribute => selector.constructor_attribute()?,
        NonTerminalKind::UnnamedFunctionAttribute => selector.unnamed_function_attribute()?,
        NonTerminalKind::FallbackFunctionAttribute => selector.fallback_function_attribute()?,
        NonTerminalKind::ReceiveFunctionAttribute => selector.receive_function_attribute()?,
        NonTerminalKind::ModifierAttribute => selector.modifier_attribute()?,
        NonTerminalKind::TypeName => selector.type_name()?,
        NonTerminalKind::FunctionTypeAttribute => selector.function_type_attribute()?,
        NonTerminalKind::MappingKeyType => selector.mapping_key_type()?,
        NonTerminalKind::ElementaryType => selector.elementary_type()?,
        NonTerminalKind::Statement => selector.statement()?,
        NonTerminalKind::TupleMember => selector.tuple_member()?,
        NonTerminalKind::VariableDeclarationType => selector.variable_declaration_type()?,
        NonTerminalKind::StorageLocation => selector.storage_location()?,
        NonTerminalKind::ForStatementInitialization => selector.for_statement_initialization()?,
        NonTerminalKind::ForStatementCondition => selector.for_statement_condition()?,
        NonTerminalKind::Expression => selector.expression()?,
        NonTerminalKind::MemberAccess => selector.member_access()?,
        NonTerminalKind::ArgumentsDeclaration => selector.arguments_declaration()?,
        NonTerminalKind::NumberUnit => selector.number_unit()?,
        NonTerminalKind::StringExpression => selector.string_expression()?,
        NonTerminalKind::StringLiteral => selector.string_literal()?,
        NonTerminalKind::HexStringLiteral => selector.hex_string_literal()?,
        NonTerminalKind::UnicodeStringLiteral => selector.unicode_string_literal()?,
        NonTerminalKind::YulStatement => selector.yul_statement()?,
        NonTerminalKind::YulAssignmentOperator => selector.yul_assignment_operator()?,
        NonTerminalKind::YulSwitchCase => selector.yul_switch_case()?,
        NonTerminalKind::YulExpression => selector.yul_expression()?,
        NonTerminalKind::YulPathComponent => selector.yul_path_component()?,
        NonTerminalKind::YulBuiltInFunction => selector.yul_built_in_function()?,
        NonTerminalKind::YulLiteral => selector.yul_literal()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_member(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::PragmaDirective,
                NonTerminalKind::ImportDirective,
                NonTerminalKind::ContractDefinition,
                NonTerminalKind::InterfaceDefinition,
                NonTerminalKind::LibraryDefinition,
                NonTerminalKind::StructDefinition,
                NonTerminalKind::EnumDefinition,
                NonTerminalKind::FunctionDefinition,
                NonTerminalKind::ConstantDefinition,
                NonTerminalKind::ErrorDefinition,
                NonTerminalKind::UserDefinedValueTypeDefinition,
                NonTerminalKind::UsingDirective,
                NonTerminalKind::EventDefinition,
            ])
        })
    }
}

impl Selector {
    fn pragma(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::ABICoderPragma,
                NonTerminalKind::ExperimentalPragma,
                NonTerminalKind::VersionPragma,
            ])
        })
    }
}

impl Selector {
    fn experimental_feature(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::StringLiteral)
                || node.is_terminal_with_kind(TerminalKind::Identifier)
        })
    }
}

impl Selector {
    fn version_expression(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::VersionRange,
                NonTerminalKind::VersionComparator,
                NonTerminalKind::VersionSpecifiers,
            ]) || node.is_terminal_with_kinds(&[
                TerminalKind::SingleQuotedVersionLiteral,
                TerminalKind::DoubleQuotedVersionLiteral,
            ])
        })
    }
}

impl Selector {
    fn import_clause(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::PathImport,
                NonTerminalKind::NamedImport,
                NonTerminalKind::ImportDeconstruction,
            ])
        })
    }
}

impl Selector {
    fn using_clause(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::IdentifierPath,
                NonTerminalKind::UsingDeconstruction,
            ])
        })
    }
}

impl Selector {
    fn using_operator(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::Ampersand,
                TerminalKind::Asterisk,
                TerminalKind::BangEqual,
                TerminalKind::Bar,
                TerminalKind::Caret,
                TerminalKind::EqualEqual,
                TerminalKind::GreaterThan,
                TerminalKind::GreaterThanEqual,
                TerminalKind::LessThan,
                TerminalKind::LessThanEqual,
                TerminalKind::Minus,
                TerminalKind::Percent,
                TerminalKind::Plus,
                TerminalKind::Slash,
                TerminalKind::Tilde,
            ])
        })
    }
}

impl Selector {
    fn using_target(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::TypeName)
                || node.is_terminal_with_kind(TerminalKind::Asterisk)
        })
    }
}

impl Selector {
    fn contract_member(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::UsingDirective,
                NonTerminalKind::FunctionDefinition,
                NonTerminalKind::ConstructorDefinition,
                NonTerminalKind::ReceiveFunctionDefinition,
                NonTerminalKind::FallbackFunctionDefinition,
                NonTerminalKind::UnnamedFunctionDefinition,
                NonTerminalKind::ModifierDefinition,
                NonTerminalKind::StructDefinition,
                NonTerminalKind::EnumDefinition,
                NonTerminalKind::EventDefinition,
                NonTerminalKind::StateVariableDefinition,
                NonTerminalKind::ErrorDefinition,
                NonTerminalKind::UserDefinedValueTypeDefinition,
            ])
        })
    }
}

impl Selector {
    fn state_variable_attribute(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::OverrideSpecifier)
                || node.is_terminal_with_kinds(&[
                    TerminalKind::ConstantKeyword,
                    TerminalKind::InternalKeyword,
                    TerminalKind::PrivateKeyword,
                    TerminalKind::PublicKeyword,
                    TerminalKind::ImmutableKeyword,
                ])
        })
    }
}

impl Selector {
    fn function_name(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::Identifier,
                TerminalKind::FallbackKeyword,
                TerminalKind::ReceiveKeyword,
            ])
        })
    }
}

impl Selector {
    fn function_attribute(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::ModifierInvocation,
                NonTerminalKind::OverrideSpecifier,
            ]) || node.is_terminal_with_kinds(&[
                TerminalKind::ConstantKeyword,
                TerminalKind::ExternalKeyword,
                TerminalKind::InternalKeyword,
                TerminalKind::PayableKeyword,
                TerminalKind::PrivateKeyword,
                TerminalKind::PublicKeyword,
                TerminalKind::PureKeyword,
                TerminalKind::ViewKeyword,
                TerminalKind::VirtualKeyword,
            ])
        })
    }
}

impl Selector {
    fn function_body(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::Block)
                || node.is_terminal_with_kind(TerminalKind::Semicolon)
        })
    }
}

impl Selector {
    fn constructor_attribute(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::ModifierInvocation)
                || node.is_terminal_with_kinds(&[
                    TerminalKind::InternalKeyword,
                    TerminalKind::OverrideKeyword,
                    TerminalKind::PayableKeyword,
                    TerminalKind::PublicKeyword,
                    TerminalKind::VirtualKeyword,
                ])
        })
    }
}

impl Selector {
    fn unnamed_function_attribute(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::ModifierInvocation)
                || node.is_terminal_with_kinds(&[
                    TerminalKind::ConstantKeyword,
                    TerminalKind::ExternalKeyword,
                    TerminalKind::InternalKeyword,
                    TerminalKind::PayableKeyword,
                    TerminalKind::PrivateKeyword,
                    TerminalKind::PublicKeyword,
                    TerminalKind::PureKeyword,
                    TerminalKind::ViewKeyword,
                ])
        })
    }
}

impl Selector {
    fn fallback_function_attribute(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::ModifierInvocation,
                NonTerminalKind::OverrideSpecifier,
            ]) || node.is_terminal_with_kinds(&[
                TerminalKind::ExternalKeyword,
                TerminalKind::PayableKeyword,
                TerminalKind::PureKeyword,
                TerminalKind::ViewKeyword,
                TerminalKind::VirtualKeyword,
            ])
        })
    }
}

impl Selector {
    fn receive_function_attribute(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::ModifierInvocation,
                NonTerminalKind::OverrideSpecifier,
            ]) || node.is_terminal_with_kinds(&[
                TerminalKind::ExternalKeyword,
                TerminalKind::PayableKeyword,
                TerminalKind::VirtualKeyword,
            ])
        })
    }
}

impl Selector {
    fn modifier_attribute(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::OverrideSpecifier)
                || node.is_terminal_with_kind(TerminalKind::VirtualKeyword)
        })
    }
}

impl Selector {
    fn type_name(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::ArrayTypeName,
                NonTerminalKind::FunctionType,
                NonTerminalKind::MappingType,
                NonTerminalKind::ElementaryType,
                NonTerminalKind::IdentifierPath,
            ])
        })
    }
}

impl Selector {
    fn function_type_attribute(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::InternalKeyword,
                TerminalKind::ExternalKeyword,
                TerminalKind::PrivateKeyword,
                TerminalKind::PublicKeyword,
                TerminalKind::ConstantKeyword,
                TerminalKind::PureKeyword,
                TerminalKind::ViewKeyword,
                TerminalKind::PayableKeyword,
            ])
        })
    }
}

impl Selector {
    fn mapping_key_type(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::ElementaryType,
                NonTerminalKind::IdentifierPath,
            ])
        })
    }
}

impl Selector {
    fn elementary_type(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::AddressType)
                || node.is_terminal_with_kinds(&[
                    TerminalKind::BoolKeyword,
                    TerminalKind::ByteKeyword,
                    TerminalKind::StringKeyword,
                    TerminalKind::BytesKeyword,
                    TerminalKind::IntKeyword,
                    TerminalKind::UintKeyword,
                    TerminalKind::FixedKeyword,
                    TerminalKind::UfixedKeyword,
                ])
        })
    }
}

impl Selector {
    fn statement(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::ExpressionStatement,
                NonTerminalKind::VariableDeclarationStatement,
                NonTerminalKind::TupleDeconstructionStatement,
                NonTerminalKind::IfStatement,
                NonTerminalKind::ForStatement,
                NonTerminalKind::WhileStatement,
                NonTerminalKind::DoWhileStatement,
                NonTerminalKind::ContinueStatement,
                NonTerminalKind::BreakStatement,
                NonTerminalKind::ReturnStatement,
                NonTerminalKind::ThrowStatement,
                NonTerminalKind::EmitStatement,
                NonTerminalKind::TryStatement,
                NonTerminalKind::RevertStatement,
                NonTerminalKind::AssemblyStatement,
                NonTerminalKind::Block,
                NonTerminalKind::UncheckedBlock,
            ])
        })
    }
}

impl Selector {
    fn tuple_member(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::TypedTupleMember,
                NonTerminalKind::UntypedTupleMember,
            ])
        })
    }
}

impl Selector {
    fn variable_declaration_type(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::TypeName)
                || node.is_terminal_with_kind(TerminalKind::VarKeyword)
        })
    }
}

impl Selector {
    fn storage_location(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::MemoryKeyword,
                TerminalKind::StorageKeyword,
                TerminalKind::CallDataKeyword,
            ])
        })
    }
}

impl Selector {
    fn for_statement_initialization(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::ExpressionStatement,
                NonTerminalKind::VariableDeclarationStatement,
                NonTerminalKind::TupleDeconstructionStatement,
            ]) || node.is_terminal_with_kind(TerminalKind::Semicolon)
        })
    }
}

impl Selector {
    fn for_statement_condition(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::ExpressionStatement)
                || node.is_terminal_with_kind(TerminalKind::Semicolon)
        })
    }
}

impl Selector {
    fn expression(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::AssignmentExpression,
                NonTerminalKind::ConditionalExpression,
                NonTerminalKind::OrExpression,
                NonTerminalKind::AndExpression,
                NonTerminalKind::EqualityExpression,
                NonTerminalKind::ComparisonExpression,
                NonTerminalKind::BitwiseOrExpression,
                NonTerminalKind::BitwiseXorExpression,
                NonTerminalKind::BitwiseAndExpression,
                NonTerminalKind::ShiftExpression,
                NonTerminalKind::AdditiveExpression,
                NonTerminalKind::MultiplicativeExpression,
                NonTerminalKind::ExponentiationExpression,
                NonTerminalKind::PostfixExpression,
                NonTerminalKind::PrefixExpression,
                NonTerminalKind::FunctionCallExpression,
                NonTerminalKind::CallOptionsExpression,
                NonTerminalKind::MemberAccessExpression,
                NonTerminalKind::IndexAccessExpression,
                NonTerminalKind::NewExpression,
                NonTerminalKind::TupleExpression,
                NonTerminalKind::TypeExpression,
                NonTerminalKind::ArrayExpression,
                NonTerminalKind::HexNumberExpression,
                NonTerminalKind::DecimalNumberExpression,
                NonTerminalKind::StringExpression,
                NonTerminalKind::ElementaryType,
            ]) || node.is_terminal_with_kinds(&[
                TerminalKind::PayableKeyword,
                TerminalKind::TrueKeyword,
                TerminalKind::FalseKeyword,
                TerminalKind::Identifier,
            ])
        })
    }
}

impl Selector {
    fn member_access(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[TerminalKind::Identifier, TerminalKind::AddressKeyword])
        })
    }
}

impl Selector {
    fn arguments_declaration(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::PositionalArgumentsDeclaration,
                NonTerminalKind::NamedArgumentsDeclaration,
            ])
        })
    }
}

impl Selector {
    fn number_unit(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::WeiKeyword,
                TerminalKind::GweiKeyword,
                TerminalKind::SzaboKeyword,
                TerminalKind::FinneyKeyword,
                TerminalKind::EtherKeyword,
                TerminalKind::SecondsKeyword,
                TerminalKind::MinutesKeyword,
                TerminalKind::HoursKeyword,
                TerminalKind::DaysKeyword,
                TerminalKind::WeeksKeyword,
                TerminalKind::YearsKeyword,
            ])
        })
    }
}

impl Selector {
    fn string_expression(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::StringLiteral,
                NonTerminalKind::StringLiterals,
                NonTerminalKind::HexStringLiteral,
                NonTerminalKind::HexStringLiterals,
                NonTerminalKind::UnicodeStringLiterals,
            ])
        })
    }
}

impl Selector {
    fn string_literal(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::SingleQuotedStringLiteral,
                TerminalKind::DoubleQuotedStringLiteral,
            ])
        })
    }
}

impl Selector {
    fn hex_string_literal(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::SingleQuotedHexStringLiteral,
                TerminalKind::DoubleQuotedHexStringLiteral,
            ])
        })
    }
}

impl Selector {
    fn unicode_string_literal(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::SingleQuotedUnicodeStringLiteral,
                TerminalKind::DoubleQuotedUnicodeStringLiteral,
            ])
        })
    }
}

impl Selector {
    fn yul_statement(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::YulBlock,
                NonTerminalKind::YulFunctionDefinition,
                NonTerminalKind::YulVariableDeclarationStatement,
                NonTerminalKind::YulAssignmentStatement,
                NonTerminalKind::YulIfStatement,
                NonTerminalKind::YulForStatement,
                NonTerminalKind::YulSwitchStatement,
                NonTerminalKind::YulLeaveStatement,
                NonTerminalKind::YulBreakStatement,
                NonTerminalKind::YulContinueStatement,
                NonTerminalKind::YulLabel,
                NonTerminalKind::YulExpression,
            ])
        })
    }
}

impl Selector {
    fn yul_assignment_operator(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::YulColonAndEqual)
                || node.is_terminal_with_kind(TerminalKind::ColonEqual)
        })
    }
}

impl Selector {
    fn yul_switch_case(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::YulDefaultCase,
                NonTerminalKind::YulValueCase,
            ])
        })
    }
}

impl Selector {
    fn yul_expression(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::YulFunctionCallExpression,
                NonTerminalKind::YulLiteral,
                NonTerminalKind::YulBuiltInFunction,
                NonTerminalKind::YulPath,
            ])
        })
    }
}

impl Selector {
    fn yul_path_component(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::YulIdentifier,
                TerminalKind::YulAddressKeyword,
            ])
        })
    }
}

impl Selector {
    fn yul_built_in_function(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::YulAddKeyword,
                TerminalKind::YulAddModKeyword,
                TerminalKind::YulAddressKeyword,
                TerminalKind::YulAndKeyword,
                TerminalKind::YulBalanceKeyword,
                TerminalKind::YulBlockHashKeyword,
                TerminalKind::YulByteKeyword,
                TerminalKind::YulCallCodeKeyword,
                TerminalKind::YulCallDataCopyKeyword,
                TerminalKind::YulCallDataLoadKeyword,
                TerminalKind::YulCallDataSizeKeyword,
                TerminalKind::YulCallerKeyword,
                TerminalKind::YulCallKeyword,
                TerminalKind::YulCallValueKeyword,
                TerminalKind::YulCoinBaseKeyword,
                TerminalKind::YulCreateKeyword,
                TerminalKind::YulDelegateCallKeyword,
                TerminalKind::YulDivKeyword,
                TerminalKind::YulEqKeyword,
                TerminalKind::YulExpKeyword,
                TerminalKind::YulExtCodeCopyKeyword,
                TerminalKind::YulExtCodeSizeKeyword,
                TerminalKind::YulGasKeyword,
                TerminalKind::YulGasLimitKeyword,
                TerminalKind::YulGasPriceKeyword,
                TerminalKind::YulGtKeyword,
                TerminalKind::YulInvalidKeyword,
                TerminalKind::YulIsZeroKeyword,
                TerminalKind::YulLog0Keyword,
                TerminalKind::YulLog1Keyword,
                TerminalKind::YulLog2Keyword,
                TerminalKind::YulLog3Keyword,
                TerminalKind::YulLog4Keyword,
                TerminalKind::YulLtKeyword,
                TerminalKind::YulMLoadKeyword,
                TerminalKind::YulModKeyword,
                TerminalKind::YulMSizeKeyword,
                TerminalKind::YulMStore8Keyword,
                TerminalKind::YulMStoreKeyword,
                TerminalKind::YulMulKeyword,
                TerminalKind::YulMulModKeyword,
                TerminalKind::YulNotKeyword,
                TerminalKind::YulNumberKeyword,
                TerminalKind::YulOriginKeyword,
                TerminalKind::YulOrKeyword,
                TerminalKind::YulPopKeyword,
                TerminalKind::YulReturnKeyword,
                TerminalKind::YulRevertKeyword,
                TerminalKind::YulSDivKeyword,
                TerminalKind::YulSelfDestructKeyword,
                TerminalKind::YulSgtKeyword,
                TerminalKind::YulSignExtendKeyword,
                TerminalKind::YulSLoadKeyword,
                TerminalKind::YulSltKeyword,
                TerminalKind::YulSModKeyword,
                TerminalKind::YulSStoreKeyword,
                TerminalKind::YulStopKeyword,
                TerminalKind::YulSubKeyword,
                TerminalKind::YulTimestampKeyword,
                TerminalKind::YulXorKeyword,
                TerminalKind::YulKeccak256Keyword,
                TerminalKind::YulSha3Keyword,
                TerminalKind::YulSuicideKeyword,
                TerminalKind::YulReturnDataCopyKeyword,
                TerminalKind::YulReturnDataSizeKeyword,
                TerminalKind::YulStaticCallKeyword,
                TerminalKind::YulCreate2Keyword,
                TerminalKind::YulExtCodeHashKeyword,
                TerminalKind::YulSarKeyword,
                TerminalKind::YulShlKeyword,
                TerminalKind::YulShrKeyword,
                TerminalKind::YulChainIdKeyword,
                TerminalKind::YulSelfBalanceKeyword,
                TerminalKind::YulBaseFeeKeyword,
                TerminalKind::YulDifficultyKeyword,
                TerminalKind::YulPrevRandaoKeyword,
                TerminalKind::YulBlobBaseFeeKeyword,
                TerminalKind::YulBlobHashKeyword,
                TerminalKind::YulTLoadKeyword,
                TerminalKind::YulTStoreKeyword,
                TerminalKind::YulMCopyKeyword,
            ])
        })
    }
}

impl Selector {
    fn yul_literal(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::HexStringLiteral,
                NonTerminalKind::StringLiteral,
            ]) || node.is_terminal_with_kinds(&[
                TerminalKind::YulTrueKeyword,
                TerminalKind::YulFalseKeyword,
                TerminalKind::YulDecimalLiteral,
                TerminalKind::YulHexLiteral,
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
    #[napi(ts_arg_type = "cst.NonTerminalNode")] node: &NonTerminalNode,
) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonTerminalKind::SourceUnitMembers => selector.source_unit_members()?,
        NonTerminalKind::VersionExpressionSet => selector.version_expression_set()?,
        NonTerminalKind::ContractMembers => selector.contract_members()?,
        NonTerminalKind::InterfaceMembers => selector.interface_members()?,
        NonTerminalKind::LibraryMembers => selector.library_members()?,
        NonTerminalKind::StructMembers => selector.struct_members()?,
        NonTerminalKind::StateVariableAttributes => selector.state_variable_attributes()?,
        NonTerminalKind::FunctionAttributes => selector.function_attributes()?,
        NonTerminalKind::ConstructorAttributes => selector.constructor_attributes()?,
        NonTerminalKind::UnnamedFunctionAttributes => selector.unnamed_function_attributes()?,
        NonTerminalKind::FallbackFunctionAttributes => selector.fallback_function_attributes()?,
        NonTerminalKind::ReceiveFunctionAttributes => selector.receive_function_attributes()?,
        NonTerminalKind::ModifierAttributes => selector.modifier_attributes()?,
        NonTerminalKind::FunctionTypeAttributes => selector.function_type_attributes()?,
        NonTerminalKind::Statements => selector.statements()?,
        NonTerminalKind::CatchClauses => selector.catch_clauses()?,
        NonTerminalKind::StringLiterals => selector.string_literals()?,
        NonTerminalKind::HexStringLiterals => selector.hex_string_literals()?,
        NonTerminalKind::UnicodeStringLiterals => selector.unicode_string_literals()?,
        NonTerminalKind::YulStatements => selector.yul_statements()?,
        NonTerminalKind::YulSwitchCases => selector.yul_switch_cases()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_members(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::SourceUnitMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn version_expression_set(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::VersionExpression))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn contract_members(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ContractMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn interface_members(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ContractMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn library_members(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ContractMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn struct_members(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::StructMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn state_variable_attributes(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::StateVariableAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn function_attributes(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::FunctionAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn constructor_attributes(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::ConstructorAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn unnamed_function_attributes(
        &mut self,
    ) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::UnnamedFunctionAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn fallback_function_attributes(
        &mut self,
    ) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::FallbackFunctionAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn receive_function_attributes(
        &mut self,
    ) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::ReceiveFunctionAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn modifier_attributes(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ModifierAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn function_type_attributes(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::FunctionTypeAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn statements(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Statement))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn catch_clauses(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::CatchClause))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn string_literals(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::StringLiteral))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn hex_string_literals(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::HexStringLiteral))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn unicode_string_literals(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::UnicodeStringLiteral)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn yul_statements(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulStatement))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn yul_switch_cases(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulSwitchCase))?
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
    #[napi(ts_arg_type = "cst.NonTerminalNode")] node: &NonTerminalNode,
) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonTerminalKind::VersionExpressionSets => selector.version_expression_sets()?,
        NonTerminalKind::VersionSpecifiers => selector.version_specifiers()?,
        NonTerminalKind::ImportDeconstructionSymbols => selector.import_deconstruction_symbols()?,
        NonTerminalKind::UsingDeconstructionSymbols => selector.using_deconstruction_symbols()?,
        NonTerminalKind::InheritanceTypes => selector.inheritance_types()?,
        NonTerminalKind::EnumMembers => selector.enum_members()?,
        NonTerminalKind::Parameters => selector.parameters()?,
        NonTerminalKind::OverridePaths => selector.override_paths()?,
        NonTerminalKind::EventParameters => selector.event_parameters()?,
        NonTerminalKind::ErrorParameters => selector.error_parameters()?,
        NonTerminalKind::AssemblyFlags => selector.assembly_flags()?,
        NonTerminalKind::TupleDeconstructionElements => selector.tuple_deconstruction_elements()?,
        NonTerminalKind::PositionalArguments => selector.positional_arguments()?,
        NonTerminalKind::NamedArguments => selector.named_arguments()?,
        NonTerminalKind::CallOptions => selector.call_options()?,
        NonTerminalKind::TupleValues => selector.tuple_values()?,
        NonTerminalKind::ArrayValues => selector.array_values()?,
        NonTerminalKind::IdentifierPath => selector.identifier_path()?,
        NonTerminalKind::YulParameters => selector.yul_parameters()?,
        NonTerminalKind::YulReturnVariables => selector.yul_return_variables()?,
        NonTerminalKind::YulArguments => selector.yul_arguments()?,
        NonTerminalKind::YulPaths => selector.yul_paths()?,
        NonTerminalKind::YulPath => selector.yul_path()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn version_expression_sets(
        &mut self,
    ) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::VersionExpressionSet)
        })? {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::BarBar))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::VersionExpressionSet)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn version_specifiers(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::VersionSpecifier))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Period))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_terminal_with_kind(TerminalKind::VersionSpecifier))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn import_deconstruction_symbols(
        &mut self,
    ) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::ImportDeconstructionSymbol)
        })? {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::ImportDeconstructionSymbol)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn using_deconstruction_symbols(
        &mut self,
    ) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::UsingDeconstructionSymbol)
        })? {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::UsingDeconstructionSymbol)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn inheritance_types(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::InheritanceType))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::InheritanceType)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn enum_members(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn parameters(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Parameter))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Parameter))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn override_paths(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::IdentifierPath))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::IdentifierPath)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn event_parameters(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::EventParameter))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::EventParameter)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn error_parameters(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::ErrorParameter))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::ErrorParameter)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn assembly_flags(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::StringLiteral))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::StringLiteral)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn tuple_deconstruction_elements(
        &mut self,
    ) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::TupleDeconstructionElement)
        })? {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::TupleDeconstructionElement)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn positional_arguments(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn named_arguments(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::NamedArgument))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::NamedArgument)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn call_options(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::NamedArgument))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::NamedArgument)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn tuple_values(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TupleValue))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TupleValue))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn array_values(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn identifier_path(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Period))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_parameters(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_return_variables(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_arguments(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulExpression))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::YulExpression)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_paths(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulPath))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulPath))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_path(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::YulPathComponent))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Period))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::YulPathComponent)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

//
// Common:
//

struct Selector {
    node: Rc<RustNonTerminalNode>,
    index: usize,
}

impl Selector {
    fn new(node: &NonTerminalNode) -> Self {
        Self {
            node: Rc::clone(&node.0),
            index: 0,
        }
    }

    fn select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Either<NonTerminalNode, TerminalNode>> {
        match self.try_select(filter)? {
            Some(node) => Ok(node),
            None => Error::MissingChild(self.index).into(),
        }
    }

    fn try_select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Option<Either<NonTerminalNode, TerminalNode>>> {
        while let Some(child) = self.node.children.get(self.index) {
            match child {
                node if node.is_trivia() => {
                    // skip trivia, since it's not part of the AST
                    self.index += 1;
                    continue;
                }
                RustEdge {
                    node: RustNode::Terminal(terminal),
                    ..
                } if matches!(terminal.kind, TerminalKind::SKIPPED) => {
                    return Error::SkippedTerminal(self.index).into();
                }
                labeled if filter(labeled) => {
                    self.index += 1;
                    return Ok(Some(labeled.node.clone().into_js_either_node()));
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
    #[error("Unexpected parent node with NonTerminalKind '{0}'.")]
    UnexpectedParent(NonTerminalKind),

    // Should not theoretically happen, since we're only called from our own generated AST types.
    #[error("Unexpected trailing children at index '{0}'.")]
    UnexpectedTrailing(usize),

    // Should not theoretically happen, unless AST error recovery was changed.
    #[error("Missing child node at index '{0}'.")]
    MissingChild(usize),

    // Can happen if the user decided to use an incorrect/incomplete CST node.
    #[error("Unexpected SKIPPED terminal at index '{0}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.")]
    SkippedTerminal(usize),
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(napi::Error::from_reason(error.to_string()))
    }
}
