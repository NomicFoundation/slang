// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cst::{NAPINodeExtensions, NonterminalNode, TerminalNode};
use crate::napi_interface::{
    NonterminalKind, RustEdge, RustNode, RustNonterminalNode, TerminalKind,
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonterminalKind::SourceUnit => selector.source_unit()?,
        NonterminalKind::PragmaDirective => selector.pragma_directive()?,
        NonterminalKind::ABICoderPragma => selector.abi_coder_pragma()?,
        NonterminalKind::ExperimentalPragma => selector.experimental_pragma()?,
        NonterminalKind::VersionPragma => selector.version_pragma()?,
        NonterminalKind::VersionRange => selector.version_range()?,
        NonterminalKind::VersionComparator => selector.version_comparator()?,
        NonterminalKind::ImportDirective => selector.import_directive()?,
        NonterminalKind::PathImport => selector.path_import()?,
        NonterminalKind::NamedImport => selector.named_import()?,
        NonterminalKind::ImportDeconstruction => selector.import_deconstruction()?,
        NonterminalKind::ImportDeconstructionSymbol => selector.import_deconstruction_symbol()?,
        NonterminalKind::ImportAlias => selector.import_alias()?,
        NonterminalKind::UsingDirective => selector.using_directive()?,
        NonterminalKind::UsingDeconstruction => selector.using_deconstruction()?,
        NonterminalKind::UsingDeconstructionSymbol => selector.using_deconstruction_symbol()?,
        NonterminalKind::UsingAlias => selector.using_alias()?,
        NonterminalKind::ContractDefinition => selector.contract_definition()?,
        NonterminalKind::InheritanceSpecifier => selector.inheritance_specifier()?,
        NonterminalKind::InheritanceType => selector.inheritance_type()?,
        NonterminalKind::InterfaceDefinition => selector.interface_definition()?,
        NonterminalKind::LibraryDefinition => selector.library_definition()?,
        NonterminalKind::StructDefinition => selector.struct_definition()?,
        NonterminalKind::StructMember => selector.struct_member()?,
        NonterminalKind::EnumDefinition => selector.enum_definition()?,
        NonterminalKind::ConstantDefinition => selector.constant_definition()?,
        NonterminalKind::StateVariableDefinition => selector.state_variable_definition()?,
        NonterminalKind::StateVariableDefinitionValue => {
            selector.state_variable_definition_value()?
        }
        NonterminalKind::FunctionDefinition => selector.function_definition()?,
        NonterminalKind::ParametersDeclaration => selector.parameters_declaration()?,
        NonterminalKind::Parameter => selector.parameter()?,
        NonterminalKind::OverrideSpecifier => selector.override_specifier()?,
        NonterminalKind::OverridePathsDeclaration => selector.override_paths_declaration()?,
        NonterminalKind::ReturnsDeclaration => selector.returns_declaration()?,
        NonterminalKind::ConstructorDefinition => selector.constructor_definition()?,
        NonterminalKind::UnnamedFunctionDefinition => selector.unnamed_function_definition()?,
        NonterminalKind::FallbackFunctionDefinition => selector.fallback_function_definition()?,
        NonterminalKind::ReceiveFunctionDefinition => selector.receive_function_definition()?,
        NonterminalKind::ModifierDefinition => selector.modifier_definition()?,
        NonterminalKind::ModifierInvocation => selector.modifier_invocation()?,
        NonterminalKind::EventDefinition => selector.event_definition()?,
        NonterminalKind::EventParametersDeclaration => selector.event_parameters_declaration()?,
        NonterminalKind::EventParameter => selector.event_parameter()?,
        NonterminalKind::UserDefinedValueTypeDefinition => {
            selector.user_defined_value_type_definition()?
        }
        NonterminalKind::ErrorDefinition => selector.error_definition()?,
        NonterminalKind::ErrorParametersDeclaration => selector.error_parameters_declaration()?,
        NonterminalKind::ErrorParameter => selector.error_parameter()?,
        NonterminalKind::ArrayTypeName => selector.array_type_name()?,
        NonterminalKind::FunctionType => selector.function_type()?,
        NonterminalKind::MappingType => selector.mapping_type()?,
        NonterminalKind::MappingKey => selector.mapping_key()?,
        NonterminalKind::MappingValue => selector.mapping_value()?,
        NonterminalKind::AddressType => selector.address_type()?,
        NonterminalKind::Block => selector.block()?,
        NonterminalKind::UncheckedBlock => selector.unchecked_block()?,
        NonterminalKind::ExpressionStatement => selector.expression_statement()?,
        NonterminalKind::AssemblyStatement => selector.assembly_statement()?,
        NonterminalKind::AssemblyFlagsDeclaration => selector.assembly_flags_declaration()?,
        NonterminalKind::TupleDeconstructionStatement => {
            selector.tuple_deconstruction_statement()?
        }
        NonterminalKind::TupleDeconstructionElement => selector.tuple_deconstruction_element()?,
        NonterminalKind::TypedTupleMember => selector.typed_tuple_member()?,
        NonterminalKind::UntypedTupleMember => selector.untyped_tuple_member()?,
        NonterminalKind::VariableDeclarationStatement => {
            selector.variable_declaration_statement()?
        }
        NonterminalKind::VariableDeclarationValue => selector.variable_declaration_value()?,
        NonterminalKind::IfStatement => selector.if_statement()?,
        NonterminalKind::ElseBranch => selector.else_branch()?,
        NonterminalKind::ForStatement => selector.for_statement()?,
        NonterminalKind::WhileStatement => selector.while_statement()?,
        NonterminalKind::DoWhileStatement => selector.do_while_statement()?,
        NonterminalKind::ContinueStatement => selector.continue_statement()?,
        NonterminalKind::BreakStatement => selector.break_statement()?,
        NonterminalKind::ReturnStatement => selector.return_statement()?,
        NonterminalKind::EmitStatement => selector.emit_statement()?,
        NonterminalKind::TryStatement => selector.try_statement()?,
        NonterminalKind::CatchClause => selector.catch_clause()?,
        NonterminalKind::CatchClauseError => selector.catch_clause_error()?,
        NonterminalKind::RevertStatement => selector.revert_statement()?,
        NonterminalKind::ThrowStatement => selector.throw_statement()?,
        NonterminalKind::AssignmentExpression => selector.assignment_expression()?,
        NonterminalKind::ConditionalExpression => selector.conditional_expression()?,
        NonterminalKind::OrExpression => selector.or_expression()?,
        NonterminalKind::AndExpression => selector.and_expression()?,
        NonterminalKind::EqualityExpression => selector.equality_expression()?,
        NonterminalKind::ComparisonExpression => selector.comparison_expression()?,
        NonterminalKind::BitwiseOrExpression => selector.bitwise_or_expression()?,
        NonterminalKind::BitwiseXorExpression => selector.bitwise_xor_expression()?,
        NonterminalKind::BitwiseAndExpression => selector.bitwise_and_expression()?,
        NonterminalKind::ShiftExpression => selector.shift_expression()?,
        NonterminalKind::AdditiveExpression => selector.additive_expression()?,
        NonterminalKind::MultiplicativeExpression => selector.multiplicative_expression()?,
        NonterminalKind::ExponentiationExpression => selector.exponentiation_expression()?,
        NonterminalKind::PostfixExpression => selector.postfix_expression()?,
        NonterminalKind::PrefixExpression => selector.prefix_expression()?,
        NonterminalKind::FunctionCallExpression => selector.function_call_expression()?,
        NonterminalKind::CallOptionsExpression => selector.call_options_expression()?,
        NonterminalKind::MemberAccessExpression => selector.member_access_expression()?,
        NonterminalKind::IndexAccessExpression => selector.index_access_expression()?,
        NonterminalKind::IndexAccessEnd => selector.index_access_end()?,
        NonterminalKind::PositionalArgumentsDeclaration => {
            selector.positional_arguments_declaration()?
        }
        NonterminalKind::NamedArgumentsDeclaration => selector.named_arguments_declaration()?,
        NonterminalKind::NamedArgumentGroup => selector.named_argument_group()?,
        NonterminalKind::NamedArgument => selector.named_argument()?,
        NonterminalKind::TypeExpression => selector.type_expression()?,
        NonterminalKind::NewExpression => selector.new_expression()?,
        NonterminalKind::TupleExpression => selector.tuple_expression()?,
        NonterminalKind::TupleValue => selector.tuple_value()?,
        NonterminalKind::ArrayExpression => selector.array_expression()?,
        NonterminalKind::HexNumberExpression => selector.hex_number_expression()?,
        NonterminalKind::DecimalNumberExpression => selector.decimal_number_expression()?,
        NonterminalKind::YulBlock => selector.yul_block()?,
        NonterminalKind::YulFunctionDefinition => selector.yul_function_definition()?,
        NonterminalKind::YulParametersDeclaration => selector.yul_parameters_declaration()?,
        NonterminalKind::YulReturnsDeclaration => selector.yul_returns_declaration()?,
        NonterminalKind::YulVariableDeclarationStatement => {
            selector.yul_variable_declaration_statement()?
        }
        NonterminalKind::YulVariableDeclarationValue => {
            selector.yul_variable_declaration_value()?
        }
        NonterminalKind::YulVariableAssignmentStatement => {
            selector.yul_variable_assignment_statement()?
        }
        NonterminalKind::YulStackAssignmentStatement => {
            selector.yul_stack_assignment_statement()?
        }
        NonterminalKind::YulColonEqual => selector.yul_colon_equal()?,
        NonterminalKind::YulIfStatement => selector.yul_if_statement()?,
        NonterminalKind::YulForStatement => selector.yul_for_statement()?,
        NonterminalKind::YulSwitchStatement => selector.yul_switch_statement()?,
        NonterminalKind::YulDefaultCase => selector.yul_default_case()?,
        NonterminalKind::YulValueCase => selector.yul_value_case()?,
        NonterminalKind::YulLeaveStatement => selector.yul_leave_statement()?,
        NonterminalKind::YulBreakStatement => selector.yul_break_statement()?,
        NonterminalKind::YulContinueStatement => selector.yul_continue_statement()?,
        NonterminalKind::YulLabel => selector.yul_label()?,
        NonterminalKind::YulFunctionCallExpression => selector.yul_function_call_expression()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}
impl Selector {
    fn source_unit(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::SourceUnitMembers)
        })?)])
    }
}

impl Selector {
    fn pragma_directive(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::PragmaKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Pragma))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn abi_coder_pragma(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AbicoderKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn experimental_pragma(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_terminal_with_kind(TerminalKind::ExperimentalKeyword))?,
            ),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ExperimentalFeature)
            })?),
        ])
    }
}

impl Selector {
    fn version_pragma(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::SolidityKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::VersionExpressionSets)
            })?),
        ])
    }
}

impl Selector {
    fn version_range(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::VersionExpression)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Minus))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::VersionExpression)
            })?),
        ])
    }
}

impl Selector {
    fn version_comparator(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Caret))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::VersionExpression)
            })?),
        ])
    }
}

impl Selector {
    fn import_directive(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ImportKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::ImportClause))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn path_import(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::StringLiteral))?,
            ),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::ImportAlias))?,
        ])
    }
}

impl Selector {
    fn named_import(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Asterisk))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::ImportAlias))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FromKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::StringLiteral))?,
            ),
        ])
    }
}

impl Selector {
    fn import_deconstruction(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ImportDeconstructionSymbols)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FromKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::StringLiteral))?,
            ),
        ])
    }
}

impl Selector {
    fn import_deconstruction_symbol(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::ImportAlias))?,
        ])
    }
}

impl Selector {
    fn import_alias(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AsKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn using_directive(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::UsingKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::UsingClause))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ForKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::UsingTarget))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::GlobalKeyword))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn using_deconstruction(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::UsingDeconstructionSymbols)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn using_deconstruction_symbol(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::IdentifierPath))?,
            ),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::UsingAlias))?,
        ])
    }
}

impl Selector {
    fn using_alias(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AsKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::UsingOperator))?,
            ),
        ])
    }
}

impl Selector {
    fn contract_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::AbstractKeyword))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ContractKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::InheritanceSpecifier)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::ContractMembers)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn inheritance_specifier(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::IsKeyword))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::InheritanceTypes)
                })?,
            ),
        ])
    }
}

impl Selector {
    fn inheritance_type(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::IdentifierPath))?,
            ),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ArgumentsDeclaration)
            })?,
        ])
    }
}

impl Selector {
    fn interface_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::InterfaceKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::InheritanceSpecifier)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::InterfaceMembers)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn library_definition(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::LibraryKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::LibraryMembers))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn struct_definition(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::StructKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::StructMembers))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn struct_member(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn enum_definition(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EnumKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::EnumMembers))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn constant_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ConstantKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn state_variable_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::StateVariableAttributes)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::StateVariableDefinitionValue)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn state_variable_definition_value(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FunctionKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::FunctionName))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::FunctionAttributes)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ReturnsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn parameters_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Parameters))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn parameter(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::StorageLocation)
            })?,
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn override_specifier(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OverrideKeyword))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::OverridePathsDeclaration)
            })?,
        ])
    }
}

impl Selector {
    fn override_paths_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::OverridePaths))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn returns_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ReturnsKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ParametersDeclaration)
            })?),
        ])
    }
}

impl Selector {
    fn constructor_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ConstructorKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ConstructorAttributes)
            })?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Block))?),
        ])
    }
}

impl Selector {
    fn unnamed_function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FunctionKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::UnnamedFunctionAttributes)
            })?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn fallback_function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FallbackKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::FallbackFunctionAttributes)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ReturnsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn receive_function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ReceiveKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ReceiveFunctionAttributes)
            })?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn modifier_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ModifierKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ParametersDeclaration)
            })?,
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ModifierAttributes)
            })?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::FunctionBody))?),
        ])
    }
}

impl Selector {
    fn modifier_invocation(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::IdentifierPath))?,
            ),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ArgumentsDeclaration)
            })?,
        ])
    }
}

impl Selector {
    fn event_definition(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EventKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::EventParametersDeclaration)
            })?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::AnonymousKeyword))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn event_parameters_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::EventParameters)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn event_parameter(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::IndexedKeyword))?,
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn user_defined_value_type_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::TypeKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::IsKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::ElementaryType))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn error_definition(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ErrorKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ErrorParametersDeclaration)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn error_parameters_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::ErrorParameters)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn error_parameter(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn array_type_name(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBracket))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn function_type(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::FunctionKeyword))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ParametersDeclaration)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::FunctionTypeAttributes)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ReturnsDeclaration)
            })?,
        ])
    }
}

impl Selector {
    fn mapping_type(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::MappingKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::MappingKey))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EqualGreaterThan))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::MappingValue))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn mapping_key(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::MappingKeyType))?,
            ),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn mapping_value(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
        ])
    }
}

impl Selector {
    fn address_type(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AddressKeyword))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::PayableKeyword))?,
        ])
    }
}

impl Selector {
    fn block(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Statements))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn unchecked_block(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::UncheckedKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Block))?),
        ])
    }
}

impl Selector {
    fn expression_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn assembly_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AssemblyKeyword))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::StringLiteral))?,
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::AssemblyFlagsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn assembly_flags_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::AssemblyFlags))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn tuple_deconstruction_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::VarKeyword))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::TupleDeconstructionElements)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn tuple_deconstruction_element(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::TupleMember)
        })?])
    }
}

impl Selector {
    fn typed_tuple_member(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::StorageLocation)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn untyped_tuple_member(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::StorageLocation)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
        ])
    }
}

impl Selector {
    fn variable_declaration_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::VariableDeclarationType)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::StorageLocation)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::VariableDeclarationValue)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn variable_declaration_value(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn if_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::IfKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Statement))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::ElseBranch))?,
        ])
    }
}

impl Selector {
    fn else_branch(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ElseKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Statement))?),
        ])
    }
}

impl Selector {
    fn for_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ForKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ForStatementInitialization)
            })?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ForStatementCondition)
            })?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Statement))?),
        ])
    }
}

impl Selector {
    fn while_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::WhileKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Statement))?),
        ])
    }
}

impl Selector {
    fn do_while_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::DoKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Statement))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::WhileKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn continue_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ContinueKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn break_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::BreakKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn return_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ReturnKeyword))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn emit_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EmitKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::IdentifierPath))?,
            ),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ArgumentsDeclaration)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn try_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::TryKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ReturnsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Block))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::CatchClauses))?),
        ])
    }
}

impl Selector {
    fn catch_clause(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CatchKeyword))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::CatchClauseError)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Block))?),
        ])
    }
}

impl Selector {
    fn catch_clause_error(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ParametersDeclaration)
            })?),
        ])
    }
}

impl Selector {
    fn revert_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::RevertKeyword))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::IdentifierPath))?,
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ArgumentsDeclaration)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn throw_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::ThrowKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn assignment_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn conditional_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::QuestionMark))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn or_expression(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::BarBar))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn and_expression(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AmpersandAmpersand))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn equality_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::EqualEqual))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn comparison_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::LessThan))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn bitwise_or_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Bar))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn bitwise_xor_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Caret))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn bitwise_and_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Ampersand))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn shift_expression(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::LessThanLessThan))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn additive_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Plus))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn multiplicative_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Asterisk))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn exponentiation_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::AsteriskAsterisk))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn postfix_expression(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::PlusPlus))?),
        ])
    }
}

impl Selector {
    fn prefix_expression(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::PlusPlus))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn function_call_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::ArgumentsDeclaration)
            })?),
        ])
    }
}

impl Selector {
    fn call_options_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::CallOptions))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn member_access_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Period))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::MemberAccess))?),
        ])
    }
}

impl Selector {
    fn index_access_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBracket))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?,
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::IndexAccessEnd))?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn index_access_end(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?,
        ])
    }
}

impl Selector {
    fn positional_arguments_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::PositionalArguments)
            })?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn named_arguments_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::NamedArgumentGroup)
            })?,
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn named_argument_group(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::NamedArguments))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn named_argument(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn type_expression(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::TypeKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn new_expression(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::NewKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TypeName))?),
        ])
    }
}

impl Selector {
    fn tuple_expression(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TupleValues))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn tuple_value(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::Expression)
        })?])
    }
}

impl Selector {
    fn array_expression(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBracket))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::ArrayValues))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn hex_number_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::HexLiteral))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::NumberUnit))?,
        ])
    }
}

impl Selector {
    fn decimal_number_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::DecimalLiteral))?),
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::NumberUnit))?,
        ])
    }
}

impl Selector {
    fn yul_block(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBrace))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulStatements))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBrace))?),
        ])
    }
}

impl Selector {
    fn yul_function_definition(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulFunctionKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::YulParametersDeclaration)
            })?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::YulReturnsDeclaration)
            })?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_parameters_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulParameters))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
        ])
    }
}

impl Selector {
    fn yul_returns_declaration(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::MinusGreaterThan))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::YulReturnVariables)
            })?),
        ])
    }
}

impl Selector {
    fn yul_variable_declaration_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulLetKeyword))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?),
            self.try_select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::YulVariableDeclarationValue)
            })?,
        ])
    }
}

impl Selector {
    fn yul_variable_declaration_value(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::YulAssignmentOperator)
            })?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulExpression))?,
            ),
        ])
    }
}

impl Selector {
    fn yul_variable_assignment_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulPaths))?),
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::YulAssignmentOperator)
            })?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulExpression))?,
            ),
        ])
    }
}

impl Selector {
    fn yul_stack_assignment_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| {
                node.is_nonterminal_with_kind(NonterminalKind::YulAssignmentOperator)
            })?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulExpression))?,
            ),
        ])
    }
}

impl Selector {
    fn yul_colon_equal(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Equal))?),
        ])
    }
}

impl Selector {
    fn yul_if_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIfKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulExpression))?,
            ),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_for_statement(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulForKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulBlock))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulExpression))?,
            ),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulBlock))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_switch_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulSwitchKeyword))?),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulExpression))?,
            ),
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulSwitchCases))?,
            ),
        ])
    }
}

impl Selector {
    fn yul_default_case(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulDefaultKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_value_case(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulCaseKeyword))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulLiteral))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulBlock))?),
        ])
    }
}

impl Selector {
    fn yul_leave_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_terminal_with_kind(TerminalKind::YulLeaveKeyword)
        })?)])
    }
}

impl Selector {
    fn yul_break_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_terminal_with_kind(TerminalKind::YulBreakKeyword)
        })?)])
    }
}

impl Selector {
    fn yul_continue_statement(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_terminal_with_kind(TerminalKind::YulContinueKeyword)
        })?)])
    }
}

impl Selector {
    fn yul_label(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::YulIdentifier))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Colon))?),
        ])
    }
}

impl Selector {
    fn yul_function_call_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(
                self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulExpression))?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenParen))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulArguments))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseParen))?),
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

    let result = match node.kind() {
        NonterminalKind::SourceUnitMember => selector.source_unit_member()?,
        NonterminalKind::Pragma => selector.pragma()?,
        NonterminalKind::ExperimentalFeature => selector.experimental_feature()?,
        NonterminalKind::VersionExpression => selector.version_expression()?,
        NonterminalKind::ImportClause => selector.import_clause()?,
        NonterminalKind::UsingClause => selector.using_clause()?,
        NonterminalKind::UsingOperator => selector.using_operator()?,
        NonterminalKind::UsingTarget => selector.using_target()?,
        NonterminalKind::ContractMember => selector.contract_member()?,
        NonterminalKind::StateVariableAttribute => selector.state_variable_attribute()?,
        NonterminalKind::FunctionName => selector.function_name()?,
        NonterminalKind::FunctionAttribute => selector.function_attribute()?,
        NonterminalKind::FunctionBody => selector.function_body()?,
        NonterminalKind::ConstructorAttribute => selector.constructor_attribute()?,
        NonterminalKind::UnnamedFunctionAttribute => selector.unnamed_function_attribute()?,
        NonterminalKind::FallbackFunctionAttribute => selector.fallback_function_attribute()?,
        NonterminalKind::ReceiveFunctionAttribute => selector.receive_function_attribute()?,
        NonterminalKind::ModifierAttribute => selector.modifier_attribute()?,
        NonterminalKind::TypeName => selector.type_name()?,
        NonterminalKind::FunctionTypeAttribute => selector.function_type_attribute()?,
        NonterminalKind::MappingKeyType => selector.mapping_key_type()?,
        NonterminalKind::ElementaryType => selector.elementary_type()?,
        NonterminalKind::Statement => selector.statement()?,
        NonterminalKind::TupleMember => selector.tuple_member()?,
        NonterminalKind::VariableDeclarationType => selector.variable_declaration_type()?,
        NonterminalKind::StorageLocation => selector.storage_location()?,
        NonterminalKind::ForStatementInitialization => selector.for_statement_initialization()?,
        NonterminalKind::ForStatementCondition => selector.for_statement_condition()?,
        NonterminalKind::Expression => selector.expression()?,
        NonterminalKind::MemberAccess => selector.member_access()?,
        NonterminalKind::ArgumentsDeclaration => selector.arguments_declaration()?,
        NonterminalKind::NumberUnit => selector.number_unit()?,
        NonterminalKind::StringExpression => selector.string_expression()?,
        NonterminalKind::StringLiteral => selector.string_literal()?,
        NonterminalKind::HexStringLiteral => selector.hex_string_literal()?,
        NonterminalKind::UnicodeStringLiteral => selector.unicode_string_literal()?,
        NonterminalKind::YulStatement => selector.yul_statement()?,
        NonterminalKind::YulAssignmentOperator => selector.yul_assignment_operator()?,
        NonterminalKind::YulSwitchCase => selector.yul_switch_case()?,
        NonterminalKind::YulExpression => selector.yul_expression()?,
        NonterminalKind::YulPathComponent => selector.yul_path_component()?,
        NonterminalKind::YulBuiltInFunction => selector.yul_built_in_function()?,
        NonterminalKind::YulLiteral => selector.yul_literal()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_member(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::PragmaDirective,
                NonterminalKind::ImportDirective,
                NonterminalKind::ContractDefinition,
                NonterminalKind::InterfaceDefinition,
                NonterminalKind::LibraryDefinition,
                NonterminalKind::StructDefinition,
                NonterminalKind::EnumDefinition,
                NonterminalKind::FunctionDefinition,
                NonterminalKind::ConstantDefinition,
                NonterminalKind::ErrorDefinition,
                NonterminalKind::UserDefinedValueTypeDefinition,
                NonterminalKind::UsingDirective,
                NonterminalKind::EventDefinition,
            ])
        })
    }
}

impl Selector {
    fn pragma(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::ABICoderPragma,
                NonterminalKind::ExperimentalPragma,
                NonterminalKind::VersionPragma,
            ])
        })
    }
}

impl Selector {
    fn experimental_feature(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::StringLiteral)
                || node.is_terminal_with_kind(TerminalKind::Identifier)
        })
    }
}

impl Selector {
    fn version_expression(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::VersionRange,
                NonterminalKind::VersionComparator,
                NonterminalKind::VersionSpecifiers,
            ]) || node.is_terminal_with_kinds(&[
                TerminalKind::SingleQuotedVersionLiteral,
                TerminalKind::DoubleQuotedVersionLiteral,
            ])
        })
    }
}

impl Selector {
    fn import_clause(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::PathImport,
                NonterminalKind::NamedImport,
                NonterminalKind::ImportDeconstruction,
            ])
        })
    }
}

impl Selector {
    fn using_clause(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::IdentifierPath,
                NonterminalKind::UsingDeconstruction,
            ])
        })
    }
}

impl Selector {
    fn using_operator(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
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
    fn using_target(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::TypeName)
                || node.is_terminal_with_kind(TerminalKind::Asterisk)
        })
    }
}

impl Selector {
    fn contract_member(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::UsingDirective,
                NonterminalKind::FunctionDefinition,
                NonterminalKind::ConstructorDefinition,
                NonterminalKind::ReceiveFunctionDefinition,
                NonterminalKind::FallbackFunctionDefinition,
                NonterminalKind::UnnamedFunctionDefinition,
                NonterminalKind::ModifierDefinition,
                NonterminalKind::StructDefinition,
                NonterminalKind::EnumDefinition,
                NonterminalKind::EventDefinition,
                NonterminalKind::StateVariableDefinition,
                NonterminalKind::ErrorDefinition,
                NonterminalKind::UserDefinedValueTypeDefinition,
            ])
        })
    }
}

impl Selector {
    fn state_variable_attribute(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::OverrideSpecifier)
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
    fn function_name(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
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
    fn function_attribute(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::ModifierInvocation,
                NonterminalKind::OverrideSpecifier,
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
    fn function_body(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::Block)
                || node.is_terminal_with_kind(TerminalKind::Semicolon)
        })
    }
}

impl Selector {
    fn constructor_attribute(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::ModifierInvocation)
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
    fn unnamed_function_attribute(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::ModifierInvocation)
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
    fn fallback_function_attribute(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::ModifierInvocation,
                NonterminalKind::OverrideSpecifier,
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
    fn receive_function_attribute(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::ModifierInvocation,
                NonterminalKind::OverrideSpecifier,
            ]) || node.is_terminal_with_kinds(&[
                TerminalKind::ExternalKeyword,
                TerminalKind::PayableKeyword,
                TerminalKind::VirtualKeyword,
            ])
        })
    }
}

impl Selector {
    fn modifier_attribute(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::OverrideSpecifier)
                || node.is_terminal_with_kind(TerminalKind::VirtualKeyword)
        })
    }
}

impl Selector {
    fn type_name(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::ArrayTypeName,
                NonterminalKind::FunctionType,
                NonterminalKind::MappingType,
                NonterminalKind::ElementaryType,
                NonterminalKind::IdentifierPath,
            ])
        })
    }
}

impl Selector {
    fn function_type_attribute(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
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
    fn mapping_key_type(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::ElementaryType,
                NonterminalKind::IdentifierPath,
            ])
        })
    }
}

impl Selector {
    fn elementary_type(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::AddressType)
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
    fn statement(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::ExpressionStatement,
                NonterminalKind::VariableDeclarationStatement,
                NonterminalKind::TupleDeconstructionStatement,
                NonterminalKind::IfStatement,
                NonterminalKind::ForStatement,
                NonterminalKind::WhileStatement,
                NonterminalKind::DoWhileStatement,
                NonterminalKind::ContinueStatement,
                NonterminalKind::BreakStatement,
                NonterminalKind::ReturnStatement,
                NonterminalKind::ThrowStatement,
                NonterminalKind::EmitStatement,
                NonterminalKind::TryStatement,
                NonterminalKind::RevertStatement,
                NonterminalKind::AssemblyStatement,
                NonterminalKind::Block,
                NonterminalKind::UncheckedBlock,
            ])
        })
    }
}

impl Selector {
    fn tuple_member(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::TypedTupleMember,
                NonterminalKind::UntypedTupleMember,
            ])
        })
    }
}

impl Selector {
    fn variable_declaration_type(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::TypeName)
                || node.is_terminal_with_kind(TerminalKind::VarKeyword)
        })
    }
}

impl Selector {
    fn storage_location(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
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
    fn for_statement_initialization(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::ExpressionStatement,
                NonterminalKind::VariableDeclarationStatement,
                NonterminalKind::TupleDeconstructionStatement,
            ]) || node.is_terminal_with_kind(TerminalKind::Semicolon)
        })
    }
}

impl Selector {
    fn for_statement_condition(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::ExpressionStatement)
                || node.is_terminal_with_kind(TerminalKind::Semicolon)
        })
    }
}

impl Selector {
    fn expression(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::AssignmentExpression,
                NonterminalKind::ConditionalExpression,
                NonterminalKind::OrExpression,
                NonterminalKind::AndExpression,
                NonterminalKind::EqualityExpression,
                NonterminalKind::ComparisonExpression,
                NonterminalKind::BitwiseOrExpression,
                NonterminalKind::BitwiseXorExpression,
                NonterminalKind::BitwiseAndExpression,
                NonterminalKind::ShiftExpression,
                NonterminalKind::AdditiveExpression,
                NonterminalKind::MultiplicativeExpression,
                NonterminalKind::ExponentiationExpression,
                NonterminalKind::PostfixExpression,
                NonterminalKind::PrefixExpression,
                NonterminalKind::FunctionCallExpression,
                NonterminalKind::CallOptionsExpression,
                NonterminalKind::MemberAccessExpression,
                NonterminalKind::IndexAccessExpression,
                NonterminalKind::NewExpression,
                NonterminalKind::TupleExpression,
                NonterminalKind::TypeExpression,
                NonterminalKind::ArrayExpression,
                NonterminalKind::HexNumberExpression,
                NonterminalKind::DecimalNumberExpression,
                NonterminalKind::StringExpression,
                NonterminalKind::ElementaryType,
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
    fn member_access(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[TerminalKind::Identifier, TerminalKind::AddressKeyword])
        })
    }
}

impl Selector {
    fn arguments_declaration(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::PositionalArgumentsDeclaration,
                NonterminalKind::NamedArgumentsDeclaration,
            ])
        })
    }
}

impl Selector {
    fn number_unit(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
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
    fn string_expression(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::StringLiteral,
                NonterminalKind::StringLiterals,
                NonterminalKind::HexStringLiteral,
                NonterminalKind::HexStringLiterals,
                NonterminalKind::UnicodeStringLiterals,
            ])
        })
    }
}

impl Selector {
    fn string_literal(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::SingleQuotedStringLiteral,
                TerminalKind::DoubleQuotedStringLiteral,
            ])
        })
    }
}

impl Selector {
    fn hex_string_literal(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::SingleQuotedHexStringLiteral,
                TerminalKind::DoubleQuotedHexStringLiteral,
            ])
        })
    }
}

impl Selector {
    fn unicode_string_literal(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::SingleQuotedUnicodeStringLiteral,
                TerminalKind::DoubleQuotedUnicodeStringLiteral,
            ])
        })
    }
}

impl Selector {
    fn yul_statement(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::YulBlock,
                NonterminalKind::YulFunctionDefinition,
                NonterminalKind::YulVariableDeclarationStatement,
                NonterminalKind::YulVariableAssignmentStatement,
                NonterminalKind::YulStackAssignmentStatement,
                NonterminalKind::YulIfStatement,
                NonterminalKind::YulForStatement,
                NonterminalKind::YulSwitchStatement,
                NonterminalKind::YulLeaveStatement,
                NonterminalKind::YulBreakStatement,
                NonterminalKind::YulContinueStatement,
                NonterminalKind::YulLabel,
                NonterminalKind::YulExpression,
            ])
        })
    }
}

impl Selector {
    fn yul_assignment_operator(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::YulColonEqual)
                || node.is_terminal_with_kind(TerminalKind::ColonEqual)
        })
    }
}

impl Selector {
    fn yul_switch_case(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::YulDefaultCase,
                NonterminalKind::YulValueCase,
            ])
        })
    }
}

impl Selector {
    fn yul_expression(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::YulFunctionCallExpression,
                NonterminalKind::YulLiteral,
                NonterminalKind::YulBuiltInFunction,
                NonterminalKind::YulPath,
            ])
        })
    }
}

impl Selector {
    fn yul_path_component(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_terminal_with_kinds(&[
                TerminalKind::YulIdentifier,
                TerminalKind::YulAddressKeyword,
            ])
        })
    }
}

impl Selector {
    fn yul_built_in_function(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
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
    fn yul_literal(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::HexStringLiteral,
                NonterminalKind::StringLiteral,
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonterminalKind::SourceUnitMembers => selector.source_unit_members()?,
        NonterminalKind::VersionExpressionSet => selector.version_expression_set()?,
        NonterminalKind::ContractMembers => selector.contract_members()?,
        NonterminalKind::InterfaceMembers => selector.interface_members()?,
        NonterminalKind::LibraryMembers => selector.library_members()?,
        NonterminalKind::StructMembers => selector.struct_members()?,
        NonterminalKind::StateVariableAttributes => selector.state_variable_attributes()?,
        NonterminalKind::FunctionAttributes => selector.function_attributes()?,
        NonterminalKind::ConstructorAttributes => selector.constructor_attributes()?,
        NonterminalKind::UnnamedFunctionAttributes => selector.unnamed_function_attributes()?,
        NonterminalKind::FallbackFunctionAttributes => selector.fallback_function_attributes()?,
        NonterminalKind::ReceiveFunctionAttributes => selector.receive_function_attributes()?,
        NonterminalKind::ModifierAttributes => selector.modifier_attributes()?,
        NonterminalKind::FunctionTypeAttributes => selector.function_type_attributes()?,
        NonterminalKind::Statements => selector.statements()?,
        NonterminalKind::CatchClauses => selector.catch_clauses()?,
        NonterminalKind::StringLiterals => selector.string_literals()?,
        NonterminalKind::HexStringLiterals => selector.hex_string_literals()?,
        NonterminalKind::UnicodeStringLiterals => selector.unicode_string_literals()?,
        NonterminalKind::YulStatements => selector.yul_statements()?,
        NonterminalKind::YulSwitchCases => selector.yul_switch_cases()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_members(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::SourceUnitMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn version_expression_set(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::VersionExpression))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn contract_members(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::ContractMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn interface_members(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::ContractMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn library_members(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::ContractMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn struct_members(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::StructMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn state_variable_attributes(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::StateVariableAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn function_attributes(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::FunctionAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn constructor_attributes(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::ConstructorAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn unnamed_function_attributes(
        &mut self,
    ) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::UnnamedFunctionAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn fallback_function_attributes(
        &mut self,
    ) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::FallbackFunctionAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn receive_function_attributes(
        &mut self,
    ) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::ReceiveFunctionAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn modifier_attributes(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::ModifierAttribute))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn function_type_attributes(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::FunctionTypeAttribute)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn statements(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::Statement))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn catch_clauses(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::CatchClause))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn string_literals(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::StringLiteral))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn hex_string_literals(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::HexStringLiteral))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn unicode_string_literals(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::UnicodeStringLiteral)
        })? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn yul_statements(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulStatement))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn yul_switch_cases(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulSwitchCase))?
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonterminalKind::VersionExpressionSets => selector.version_expression_sets()?,
        NonterminalKind::VersionSpecifiers => selector.version_specifiers()?,
        NonterminalKind::ImportDeconstructionSymbols => selector.import_deconstruction_symbols()?,
        NonterminalKind::UsingDeconstructionSymbols => selector.using_deconstruction_symbols()?,
        NonterminalKind::InheritanceTypes => selector.inheritance_types()?,
        NonterminalKind::EnumMembers => selector.enum_members()?,
        NonterminalKind::Parameters => selector.parameters()?,
        NonterminalKind::OverridePaths => selector.override_paths()?,
        NonterminalKind::EventParameters => selector.event_parameters()?,
        NonterminalKind::ErrorParameters => selector.error_parameters()?,
        NonterminalKind::AssemblyFlags => selector.assembly_flags()?,
        NonterminalKind::TupleDeconstructionElements => selector.tuple_deconstruction_elements()?,
        NonterminalKind::PositionalArguments => selector.positional_arguments()?,
        NonterminalKind::NamedArguments => selector.named_arguments()?,
        NonterminalKind::CallOptions => selector.call_options()?,
        NonterminalKind::TupleValues => selector.tuple_values()?,
        NonterminalKind::ArrayValues => selector.array_values()?,
        NonterminalKind::IdentifierPath => selector.identifier_path()?,
        NonterminalKind::YulParameters => selector.yul_parameters()?,
        NonterminalKind::YulReturnVariables => selector.yul_return_variables()?,
        NonterminalKind::YulArguments => selector.yul_arguments()?,
        NonterminalKind::YulPaths => selector.yul_paths()?,
        NonterminalKind::YulPath => selector.yul_path()?,
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
    ) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::VersionExpressionSet)
        })? {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::BarBar))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::VersionExpressionSet)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn version_specifiers(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
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
    ) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::ImportDeconstructionSymbol)
        })? {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::ImportDeconstructionSymbol)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn using_deconstruction_symbols(
        &mut self,
    ) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::UsingDeconstructionSymbol)
        })? {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::UsingDeconstructionSymbol)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn inheritance_types(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::InheritanceType))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::InheritanceType)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn enum_members(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
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
    fn parameters(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::Parameter))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Parameter))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn override_paths(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::IdentifierPath))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::IdentifierPath)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn event_parameters(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::EventParameter))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::EventParameter)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn error_parameters(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::ErrorParameter))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::ErrorParameter)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn assembly_flags(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::StringLiteral))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::StringLiteral)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn tuple_deconstruction_elements(
        &mut self,
    ) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self.try_select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::TupleDeconstructionElement)
        })? {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::TupleDeconstructionElement)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn positional_arguments(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn named_arguments(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::NamedArgument))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::NamedArgument)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn call_options(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::NamedArgument))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::NamedArgument)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn tuple_values(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::TupleValue))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TupleValue))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn array_values(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn identifier_path(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
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
    fn yul_parameters(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
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
    fn yul_return_variables(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
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
    fn yul_arguments(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulExpression))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::YulExpression)
                })?);
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_paths(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulPath))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Comma))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulPath))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

impl Selector {
    fn yul_path(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::YulPathComponent))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Period))?
            {
                separators.push(separator);

                separated.push(self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::YulPathComponent)
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

    fn select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Either<NonterminalNode, TerminalNode>> {
        match self.try_select(filter)? {
            Some(node) => Ok(node),
            None => Error::MissingChild(self.index).into(),
        }
    }

    fn try_select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Option<Either<NonterminalNode, TerminalNode>>> {
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
    #[error("Unexpected parent node with NonterminalKind '{0}'.")]
    UnexpectedParent(NonterminalKind),

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
