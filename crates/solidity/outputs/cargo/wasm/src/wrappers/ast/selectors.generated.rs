// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)] // large match statements for all non-terminals
#![allow(clippy::unnecessary_wraps)] // using `Result` for all functions for error handling

use std::rc::Rc;

use slang_solidity::cst::{Edge, EdgeLabel, Node, NonterminalKind, NonterminalNode};

//
// Sequences:
//

pub fn select_sequence(node: &Rc<NonterminalNode>) -> Result<Vec<Option<Node>>> {
    let mut helper = Helper::new(node);

    let result = match node.kind {
        NonterminalKind::SourceUnit => helper.source_unit_sequence()?,
        NonterminalKind::PragmaDirective => helper.pragma_directive_sequence()?,
        NonterminalKind::AbicoderPragma => helper.abicoder_pragma_sequence()?,
        NonterminalKind::ExperimentalPragma => helper.experimental_pragma_sequence()?,
        NonterminalKind::VersionPragma => helper.version_pragma_sequence()?,
        NonterminalKind::VersionRange => helper.version_range_sequence()?,
        NonterminalKind::VersionTerm => helper.version_term_sequence()?,
        NonterminalKind::ImportDirective => helper.import_directive_sequence()?,
        NonterminalKind::PathImport => helper.path_import_sequence()?,
        NonterminalKind::NamedImport => helper.named_import_sequence()?,
        NonterminalKind::ImportDeconstruction => helper.import_deconstruction_sequence()?,
        NonterminalKind::ImportDeconstructionSymbol => {
            helper.import_deconstruction_symbol_sequence()?
        }
        NonterminalKind::ImportAlias => helper.import_alias_sequence()?,
        NonterminalKind::UsingDirective => helper.using_directive_sequence()?,
        NonterminalKind::UsingDeconstruction => helper.using_deconstruction_sequence()?,
        NonterminalKind::UsingDeconstructionSymbol => {
            helper.using_deconstruction_symbol_sequence()?
        }
        NonterminalKind::UsingAlias => helper.using_alias_sequence()?,
        NonterminalKind::ContractDefinition => helper.contract_definition_sequence()?,
        NonterminalKind::InheritanceSpecifier => helper.inheritance_specifier_sequence()?,
        NonterminalKind::InheritanceType => helper.inheritance_type_sequence()?,
        NonterminalKind::StorageLayoutSpecifier => helper.storage_layout_specifier_sequence()?,
        NonterminalKind::InterfaceDefinition => helper.interface_definition_sequence()?,
        NonterminalKind::LibraryDefinition => helper.library_definition_sequence()?,
        NonterminalKind::StructDefinition => helper.struct_definition_sequence()?,
        NonterminalKind::StructMember => helper.struct_member_sequence()?,
        NonterminalKind::EnumDefinition => helper.enum_definition_sequence()?,
        NonterminalKind::ConstantDefinition => helper.constant_definition_sequence()?,
        NonterminalKind::StateVariableDefinition => helper.state_variable_definition_sequence()?,
        NonterminalKind::StateVariableDefinitionValue => {
            helper.state_variable_definition_value_sequence()?
        }
        NonterminalKind::FunctionDefinition => helper.function_definition_sequence()?,
        NonterminalKind::ParametersDeclaration => helper.parameters_declaration_sequence()?,
        NonterminalKind::Parameter => helper.parameter_sequence()?,
        NonterminalKind::OverrideSpecifier => helper.override_specifier_sequence()?,
        NonterminalKind::OverridePathsDeclaration => {
            helper.override_paths_declaration_sequence()?
        }
        NonterminalKind::ReturnsDeclaration => helper.returns_declaration_sequence()?,
        NonterminalKind::ConstructorDefinition => helper.constructor_definition_sequence()?,
        NonterminalKind::UnnamedFunctionDefinition => {
            helper.unnamed_function_definition_sequence()?
        }
        NonterminalKind::FallbackFunctionDefinition => {
            helper.fallback_function_definition_sequence()?
        }
        NonterminalKind::ReceiveFunctionDefinition => {
            helper.receive_function_definition_sequence()?
        }
        NonterminalKind::ModifierDefinition => helper.modifier_definition_sequence()?,
        NonterminalKind::ModifierInvocation => helper.modifier_invocation_sequence()?,
        NonterminalKind::EventDefinition => helper.event_definition_sequence()?,
        NonterminalKind::EventParametersDeclaration => {
            helper.event_parameters_declaration_sequence()?
        }
        NonterminalKind::EventParameter => helper.event_parameter_sequence()?,
        NonterminalKind::UserDefinedValueTypeDefinition => {
            helper.user_defined_value_type_definition_sequence()?
        }
        NonterminalKind::ErrorDefinition => helper.error_definition_sequence()?,
        NonterminalKind::ErrorParametersDeclaration => {
            helper.error_parameters_declaration_sequence()?
        }
        NonterminalKind::ErrorParameter => helper.error_parameter_sequence()?,
        NonterminalKind::ArrayTypeName => helper.array_type_name_sequence()?,
        NonterminalKind::FunctionType => helper.function_type_sequence()?,
        NonterminalKind::MappingType => helper.mapping_type_sequence()?,
        NonterminalKind::MappingKey => helper.mapping_key_sequence()?,
        NonterminalKind::MappingValue => helper.mapping_value_sequence()?,
        NonterminalKind::AddressType => helper.address_type_sequence()?,
        NonterminalKind::Block => helper.block_sequence()?,
        NonterminalKind::UncheckedBlock => helper.unchecked_block_sequence()?,
        NonterminalKind::ExpressionStatement => helper.expression_statement_sequence()?,
        NonterminalKind::AssemblyStatement => helper.assembly_statement_sequence()?,
        NonterminalKind::AssemblyFlagsDeclaration => {
            helper.assembly_flags_declaration_sequence()?
        }
        NonterminalKind::TupleDeconstructionStatement => {
            helper.tuple_deconstruction_statement_sequence()?
        }
        NonterminalKind::VarTupleDeconstructionTarget => {
            helper.var_tuple_deconstruction_target_sequence()?
        }
        NonterminalKind::UntypedTupleDeconstructionElement => {
            helper.untyped_tuple_deconstruction_element_sequence()?
        }
        NonterminalKind::TypedTupleDeconstructionTarget => {
            helper.typed_tuple_deconstruction_target_sequence()?
        }
        NonterminalKind::TypedTupleDeconstructionElement => {
            helper.typed_tuple_deconstruction_element_sequence()?
        }
        NonterminalKind::TypedTupleDeconstructionMember => {
            helper.typed_tuple_deconstruction_member_sequence()?
        }
        NonterminalKind::VariableDeclarationStatement => {
            helper.variable_declaration_statement_sequence()?
        }
        NonterminalKind::VariableDeclarationValue => {
            helper.variable_declaration_value_sequence()?
        }
        NonterminalKind::IfStatement => helper.if_statement_sequence()?,
        NonterminalKind::ElseBranch => helper.else_branch_sequence()?,
        NonterminalKind::ForStatement => helper.for_statement_sequence()?,
        NonterminalKind::WhileStatement => helper.while_statement_sequence()?,
        NonterminalKind::DoWhileStatement => helper.do_while_statement_sequence()?,
        NonterminalKind::ContinueStatement => helper.continue_statement_sequence()?,
        NonterminalKind::BreakStatement => helper.break_statement_sequence()?,
        NonterminalKind::ReturnStatement => helper.return_statement_sequence()?,
        NonterminalKind::EmitStatement => helper.emit_statement_sequence()?,
        NonterminalKind::TryStatement => helper.try_statement_sequence()?,
        NonterminalKind::CatchClause => helper.catch_clause_sequence()?,
        NonterminalKind::CatchClauseError => helper.catch_clause_error_sequence()?,
        NonterminalKind::RevertStatement => helper.revert_statement_sequence()?,
        NonterminalKind::ThrowStatement => helper.throw_statement_sequence()?,
        NonterminalKind::AssignmentExpression => helper.assignment_expression_sequence()?,
        NonterminalKind::ConditionalExpression => helper.conditional_expression_sequence()?,
        NonterminalKind::OrExpression => helper.or_expression_sequence()?,
        NonterminalKind::AndExpression => helper.and_expression_sequence()?,
        NonterminalKind::EqualityExpression => helper.equality_expression_sequence()?,
        NonterminalKind::InequalityExpression => helper.inequality_expression_sequence()?,
        NonterminalKind::BitwiseOrExpression => helper.bitwise_or_expression_sequence()?,
        NonterminalKind::BitwiseXorExpression => helper.bitwise_xor_expression_sequence()?,
        NonterminalKind::BitwiseAndExpression => helper.bitwise_and_expression_sequence()?,
        NonterminalKind::ShiftExpression => helper.shift_expression_sequence()?,
        NonterminalKind::AdditiveExpression => helper.additive_expression_sequence()?,
        NonterminalKind::MultiplicativeExpression => helper.multiplicative_expression_sequence()?,
        NonterminalKind::ExponentiationExpression => helper.exponentiation_expression_sequence()?,
        NonterminalKind::PostfixExpression => helper.postfix_expression_sequence()?,
        NonterminalKind::PrefixExpression => helper.prefix_expression_sequence()?,
        NonterminalKind::FunctionCallExpression => helper.function_call_expression_sequence()?,
        NonterminalKind::CallOptionsExpression => helper.call_options_expression_sequence()?,
        NonterminalKind::MemberAccessExpression => helper.member_access_expression_sequence()?,
        NonterminalKind::IndexAccessExpression => helper.index_access_expression_sequence()?,
        NonterminalKind::IndexAccessEnd => helper.index_access_end_sequence()?,
        NonterminalKind::PositionalArgumentsDeclaration => {
            helper.positional_arguments_declaration_sequence()?
        }
        NonterminalKind::NamedArgumentsDeclaration => {
            helper.named_arguments_declaration_sequence()?
        }
        NonterminalKind::NamedArgumentGroup => helper.named_argument_group_sequence()?,
        NonterminalKind::NamedArgument => helper.named_argument_sequence()?,
        NonterminalKind::TypeExpression => helper.type_expression_sequence()?,
        NonterminalKind::NewExpression => helper.new_expression_sequence()?,
        NonterminalKind::TupleExpression => helper.tuple_expression_sequence()?,
        NonterminalKind::TupleValue => helper.tuple_value_sequence()?,
        NonterminalKind::ArrayExpression => helper.array_expression_sequence()?,
        NonterminalKind::HexNumberExpression => helper.hex_number_expression_sequence()?,
        NonterminalKind::DecimalNumberExpression => helper.decimal_number_expression_sequence()?,
        NonterminalKind::YulBlock => helper.yul_block_sequence()?,
        NonterminalKind::YulFunctionDefinition => helper.yul_function_definition_sequence()?,
        NonterminalKind::YulParametersDeclaration => {
            helper.yul_parameters_declaration_sequence()?
        }
        NonterminalKind::YulReturnsDeclaration => helper.yul_returns_declaration_sequence()?,
        NonterminalKind::YulVariableDeclarationStatement => {
            helper.yul_variable_declaration_statement_sequence()?
        }
        NonterminalKind::YulVariableDeclarationValue => {
            helper.yul_variable_declaration_value_sequence()?
        }
        NonterminalKind::YulVariableAssignmentStatement => {
            helper.yul_variable_assignment_statement_sequence()?
        }
        NonterminalKind::YulColonAndEqual => helper.yul_colon_and_equal_sequence()?,
        NonterminalKind::YulStackAssignmentStatement => {
            helper.yul_stack_assignment_statement_sequence()?
        }
        NonterminalKind::YulEqualAndColon => helper.yul_equal_and_colon_sequence()?,
        NonterminalKind::YulIfStatement => helper.yul_if_statement_sequence()?,
        NonterminalKind::YulForStatement => helper.yul_for_statement_sequence()?,
        NonterminalKind::YulSwitchStatement => helper.yul_switch_statement_sequence()?,
        NonterminalKind::YulDefaultCase => helper.yul_default_case_sequence()?,
        NonterminalKind::YulValueCase => helper.yul_value_case_sequence()?,
        NonterminalKind::YulLeaveStatement => helper.yul_leave_statement_sequence()?,
        NonterminalKind::YulBreakStatement => helper.yul_break_statement_sequence()?,
        NonterminalKind::YulContinueStatement => helper.yul_continue_statement_sequence()?,
        NonterminalKind::YulLabel => helper.yul_label_sequence()?,
        NonterminalKind::YulFunctionCallExpression => {
            helper.yul_function_call_expression_sequence()?
        }
        _ => {
            // Should not theoretically happen, since we're only called from our own generated AST types.
            return Err(format!(
                "Unexpected parent node with NonterminalKind '{0}'.",
                node.kind
            ));
        }
    };

    helper.finalize()?;
    Ok(result)
}

impl Helper {
    fn source_unit_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![Some(self.select(EdgeLabel::Members)?)])
    }
}

impl Helper {
    fn pragma_directive_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::PragmaKeyword)?),
            Some(self.select(EdgeLabel::Pragma)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn abicoder_pragma_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AbicoderKeyword)?),
            Some(self.select(EdgeLabel::Version)?),
        ])
    }
}

impl Helper {
    fn experimental_pragma_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ExperimentalKeyword)?),
            Some(self.select(EdgeLabel::Feature)?),
        ])
    }
}

impl Helper {
    fn version_pragma_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::SolidityKeyword)?),
            Some(self.select(EdgeLabel::Sets)?),
        ])
    }
}

impl Helper {
    fn version_range_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Start)?),
            Some(self.select(EdgeLabel::Minus)?),
            Some(self.select(EdgeLabel::End)?),
        ])
    }
}

impl Helper {
    fn version_term_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            self.try_select(EdgeLabel::Operator),
            Some(self.select(EdgeLabel::Literal)?),
        ])
    }
}

impl Helper {
    fn import_directive_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ImportKeyword)?),
            Some(self.select(EdgeLabel::Clause)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn path_import_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Path)?),
            self.try_select(EdgeLabel::Alias),
        ])
    }
}

impl Helper {
    fn named_import_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Asterisk)?),
            Some(self.select(EdgeLabel::Alias)?),
            Some(self.select(EdgeLabel::FromKeyword)?),
            Some(self.select(EdgeLabel::Path)?),
        ])
    }
}

impl Helper {
    fn import_deconstruction_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Symbols)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
            Some(self.select(EdgeLabel::FromKeyword)?),
            Some(self.select(EdgeLabel::Path)?),
        ])
    }
}

impl Helper {
    fn import_deconstruction_symbol_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Alias),
        ])
    }
}

impl Helper {
    fn import_alias_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AsKeyword)?),
            Some(self.select(EdgeLabel::Identifier)?),
        ])
    }
}

impl Helper {
    fn using_directive_sequence(&mut self) -> Result<Vec<Option<Node>>> {
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

impl Helper {
    fn using_deconstruction_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Symbols)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Helper {
    fn using_deconstruction_symbol_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Alias),
        ])
    }
}

impl Helper {
    fn using_alias_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AsKeyword)?),
            Some(self.select(EdgeLabel::Operator)?),
        ])
    }
}

impl Helper {
    fn contract_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            self.try_select(EdgeLabel::AbstractKeyword),
            Some(self.select(EdgeLabel::ContractKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Specifiers)?),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Helper {
    fn inheritance_specifier_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::IsKeyword)?),
            Some(self.select(EdgeLabel::Types)?),
        ])
    }
}

impl Helper {
    fn inheritance_type_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::Arguments),
        ])
    }
}

impl Helper {
    fn storage_layout_specifier_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LayoutKeyword)?),
            Some(self.select(EdgeLabel::AtKeyword)?),
            Some(self.select(EdgeLabel::Expression)?),
        ])
    }
}

impl Helper {
    fn interface_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
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

impl Helper {
    fn library_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LibraryKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Helper {
    fn struct_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::StructKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Helper {
    fn struct_member_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn enum_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::EnumKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Helper {
    fn constant_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
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

impl Helper {
    fn state_variable_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Value),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn state_variable_definition_value_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Equal)?),
            Some(self.select(EdgeLabel::Value)?),
        ])
    }
}

impl Helper {
    fn function_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
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

impl Helper {
    fn parameters_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn parameter_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::StorageLocation),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Helper {
    fn override_specifier_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OverrideKeyword)?),
            self.try_select(EdgeLabel::Overridden),
        ])
    }
}

impl Helper {
    fn override_paths_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Paths)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn returns_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ReturnsKeyword)?),
            Some(self.select(EdgeLabel::Variables)?),
        ])
    }
}

impl Helper {
    fn constructor_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ConstructorKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn unnamed_function_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::FunctionKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn fallback_function_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::FallbackKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            self.try_select(EdgeLabel::Returns),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn receive_function_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ReceiveKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn modifier_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ModifierKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Parameters),
            Some(self.select(EdgeLabel::Attributes)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn modifier_invocation_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Arguments),
        ])
    }
}

impl Helper {
    fn event_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::EventKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Parameters)?),
            self.try_select(EdgeLabel::AnonymousKeyword),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn event_parameters_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn event_parameter_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::IndexedKeyword),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Helper {
    fn user_defined_value_type_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::IsKeyword)?),
            Some(self.select(EdgeLabel::ValueType)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn error_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ErrorKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn error_parameters_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn error_parameter_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Helper {
    fn array_type_name_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::OpenBracket)?),
            self.try_select(EdgeLabel::Index),
            Some(self.select(EdgeLabel::CloseBracket)?),
        ])
    }
}

impl Helper {
    fn function_type_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::FunctionKeyword)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::Attributes)?),
            self.try_select(EdgeLabel::Returns),
        ])
    }
}

impl Helper {
    fn mapping_type_sequence(&mut self) -> Result<Vec<Option<Node>>> {
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

impl Helper {
    fn mapping_key_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::KeyType)?),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Helper {
    fn mapping_value_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::Name),
        ])
    }
}

impl Helper {
    fn address_type_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AddressKeyword)?),
            self.try_select(EdgeLabel::PayableKeyword),
        ])
    }
}

impl Helper {
    fn block_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Statements)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Helper {
    fn unchecked_block_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::UncheckedKeyword)?),
            Some(self.select(EdgeLabel::Block)?),
        ])
    }
}

impl Helper {
    fn expression_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Expression)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn assembly_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::AssemblyKeyword)?),
            self.try_select(EdgeLabel::Label),
            self.try_select(EdgeLabel::Flags),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn assembly_flags_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Flags)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn tuple_deconstruction_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Target)?),
            Some(self.select(EdgeLabel::Equal)?),
            Some(self.select(EdgeLabel::Expression)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn var_tuple_deconstruction_target_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::VarKeyword)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Elements)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn untyped_tuple_deconstruction_element_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![self.try_select(EdgeLabel::Name)])
    }
}

impl Helper {
    fn typed_tuple_deconstruction_target_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Elements)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn typed_tuple_deconstruction_element_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![self.try_select(EdgeLabel::Member)])
    }
}

impl Helper {
    fn typed_tuple_deconstruction_member_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeName)?),
            self.try_select(EdgeLabel::StorageLocation),
            Some(self.select(EdgeLabel::Name)?),
        ])
    }
}

impl Helper {
    fn variable_declaration_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::VariableType)?),
            self.try_select(EdgeLabel::StorageLocation),
            Some(self.select(EdgeLabel::Name)?),
            self.try_select(EdgeLabel::Value),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn variable_declaration_value_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Equal)?),
            Some(self.select(EdgeLabel::Expression)?),
        ])
    }
}

impl Helper {
    fn if_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
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

impl Helper {
    fn else_branch_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ElseKeyword)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn for_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
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

impl Helper {
    fn while_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::WhileKeyword)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Condition)?),
            Some(self.select(EdgeLabel::CloseParen)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn do_while_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
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

impl Helper {
    fn continue_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ContinueKeyword)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn break_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::BreakKeyword)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn return_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ReturnKeyword)?),
            self.try_select(EdgeLabel::Expression),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn emit_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::EmitKeyword)?),
            Some(self.select(EdgeLabel::Event)?),
            Some(self.select(EdgeLabel::Arguments)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn try_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TryKeyword)?),
            Some(self.select(EdgeLabel::Expression)?),
            self.try_select(EdgeLabel::Returns),
            Some(self.select(EdgeLabel::Body)?),
            Some(self.select(EdgeLabel::CatchClauses)?),
        ])
    }
}

impl Helper {
    fn catch_clause_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::CatchKeyword)?),
            self.try_select(EdgeLabel::Error),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn catch_clause_error_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            self.try_select(EdgeLabel::Name),
            Some(self.select(EdgeLabel::Parameters)?),
        ])
    }
}

impl Helper {
    fn revert_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::RevertKeyword)?),
            Some(self.select(EdgeLabel::Error)?),
            Some(self.select(EdgeLabel::Arguments)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn throw_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ThrowKeyword)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn assignment_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn conditional_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::QuestionMark)?),
            Some(self.select(EdgeLabel::TrueExpression)?),
            Some(self.select(EdgeLabel::Colon)?),
            Some(self.select(EdgeLabel::FalseExpression)?),
        ])
    }
}

impl Helper {
    fn or_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn and_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn equality_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn inequality_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn bitwise_or_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn bitwise_xor_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn bitwise_and_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn shift_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn additive_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn multiplicative_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn exponentiation_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn postfix_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::Operator)?),
        ])
    }
}

impl Helper {
    fn prefix_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::Operand)?),
        ])
    }
}

impl Helper {
    fn function_call_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::Arguments)?),
        ])
    }
}

impl Helper {
    fn call_options_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Options)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Helper {
    fn member_access_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::Period)?),
            Some(self.select(EdgeLabel::Member)?),
        ])
    }
}

impl Helper {
    fn index_access_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::OpenBracket)?),
            self.try_select(EdgeLabel::Start),
            self.try_select(EdgeLabel::End),
            Some(self.select(EdgeLabel::CloseBracket)?),
        ])
    }
}

impl Helper {
    fn index_access_end_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Colon)?),
            self.try_select(EdgeLabel::End),
        ])
    }
}

impl Helper {
    fn positional_arguments_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Arguments)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn named_arguments_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            self.try_select(EdgeLabel::Arguments),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn named_argument_group_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Arguments)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Helper {
    fn named_argument_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Colon)?),
            Some(self.select(EdgeLabel::Value)?),
        ])
    }
}

impl Helper {
    fn type_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::TypeKeyword)?),
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::TypeName)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn new_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::NewKeyword)?),
            Some(self.select(EdgeLabel::TypeName)?),
        ])
    }
}

impl Helper {
    fn tuple_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Items)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn tuple_value_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![self.try_select(EdgeLabel::Expression)])
    }
}

impl Helper {
    fn array_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBracket)?),
            Some(self.select(EdgeLabel::Items)?),
            Some(self.select(EdgeLabel::CloseBracket)?),
        ])
    }
}

impl Helper {
    fn hex_number_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Literal)?),
            self.try_select(EdgeLabel::Unit),
        ])
    }
}

impl Helper {
    fn decimal_number_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Literal)?),
            self.try_select(EdgeLabel::Unit),
        ])
    }
}

impl Helper {
    fn yul_block_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBrace)?),
            Some(self.select(EdgeLabel::Statements)?),
            Some(self.select(EdgeLabel::CloseBrace)?),
        ])
    }
}

impl Helper {
    fn yul_function_definition_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::FunctionKeyword)?),
            Some(self.select(EdgeLabel::Name)?),
            Some(self.select(EdgeLabel::Parameters)?),
            self.try_select(EdgeLabel::Returns),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn yul_parameters_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenParen)?),
            Some(self.select(EdgeLabel::Parameters)?),
            Some(self.select(EdgeLabel::CloseParen)?),
        ])
    }
}

impl Helper {
    fn yul_returns_declaration_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::MinusGreaterThan)?),
            Some(self.select(EdgeLabel::Variables)?),
        ])
    }
}

impl Helper {
    fn yul_variable_declaration_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LetKeyword)?),
            Some(self.select(EdgeLabel::Variables)?),
            self.try_select(EdgeLabel::Value),
        ])
    }
}

impl Helper {
    fn yul_variable_declaration_value_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Assignment)?),
            Some(self.select(EdgeLabel::Expression)?),
        ])
    }
}

impl Helper {
    fn yul_variable_assignment_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Variables)?),
            Some(self.select(EdgeLabel::Assignment)?),
            Some(self.select(EdgeLabel::Expression)?),
        ])
    }
}

impl Helper {
    fn yul_colon_and_equal_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Colon)?),
            Some(self.select(EdgeLabel::Equal)?),
        ])
    }
}

impl Helper {
    fn yul_stack_assignment_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Assignment)?),
            Some(self.select(EdgeLabel::Variable)?),
        ])
    }
}

impl Helper {
    fn yul_equal_and_colon_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Equal)?),
            Some(self.select(EdgeLabel::Colon)?),
        ])
    }
}

impl Helper {
    fn yul_if_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::IfKeyword)?),
            Some(self.select(EdgeLabel::Condition)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn yul_for_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::ForKeyword)?),
            Some(self.select(EdgeLabel::Initialization)?),
            Some(self.select(EdgeLabel::Condition)?),
            Some(self.select(EdgeLabel::Iterator)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn yul_switch_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::SwitchKeyword)?),
            Some(self.select(EdgeLabel::Expression)?),
            Some(self.select(EdgeLabel::Cases)?),
        ])
    }
}

impl Helper {
    fn yul_default_case_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::DefaultKeyword)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn yul_value_case_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::CaseKeyword)?),
            Some(self.select(EdgeLabel::Value)?),
            Some(self.select(EdgeLabel::Body)?),
        ])
    }
}

impl Helper {
    fn yul_leave_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![Some(self.select(EdgeLabel::LeaveKeyword)?)])
    }
}

impl Helper {
    fn yul_break_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![Some(self.select(EdgeLabel::BreakKeyword)?)])
    }
}

impl Helper {
    fn yul_continue_statement_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![Some(self.select(EdgeLabel::ContinueKeyword)?)])
    }
}

impl Helper {
    fn yul_label_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Label)?),
            Some(self.select(EdgeLabel::Colon)?),
        ])
    }
}

impl Helper {
    fn yul_function_call_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
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

pub fn select_choice(node: &Rc<NonterminalNode>) -> Result<Node> {
    let mut helper = Helper::new(node);

    let variant = helper.select(EdgeLabel::Variant)?;

    helper.finalize()?;
    Ok(variant)
}

//
// Repeated:
//

pub fn select_repeated(node: &Rc<NonterminalNode>) -> Result<Vec<Node>> {
    let mut helper = Helper::new(node);

    let mut items = vec![];

    while let Some(item) = helper.try_select(EdgeLabel::Item) {
        items.push(item);
    }

    helper.finalize()?;
    Ok(items)
}

//
// Separated:
//

pub fn select_separated(node: &Rc<NonterminalNode>) -> Result<Vec<Vec<Node>>> {
    let mut helper = Helper::new(node);

    let mut items = vec![];
    let mut separators = vec![];

    if let Some(first) = helper.try_select(EdgeLabel::Item) {
        items.push(first);

        while let Some(separator) = helper.try_select(EdgeLabel::Separator) {
            separators.push(separator);

            items.push(helper.select(EdgeLabel::Item)?);
        }
    }

    helper.finalize()?;
    Ok(vec![items, separators])
}

//
// Common:
//

type Result<T> = std::result::Result<T, String>;

struct Helper {
    node: Rc<NonterminalNode>,
    index: usize,
}

impl Helper {
    fn new(node: &Rc<NonterminalNode>) -> Self {
        Self {
            node: Rc::clone(node),
            index: 0,
        }
    }

    fn select(&mut self, target_label: EdgeLabel) -> Result<Node> {
        match self.try_select(target_label) {
            Some(node) => {
                Ok(node)
            },
            None => {
                Err(format!("Missing child with label '{target_label}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet."))
            }
        }
    }

    fn try_select(&mut self, target_label: EdgeLabel) -> Option<Node> {
        let edge = self.current()?;

        if edge.label == target_label {
            self.index += 1;
            Some(edge.node)
        } else {
            None
        }
    }

    fn current(&mut self) -> Option<Edge> {
        loop {
            let edge = self.node.children.get(self.index)?;

            match edge.label {
                // Skip trivia:
                EdgeLabel::LeadingTrivia | EdgeLabel::TrailingTrivia => {
                    self.index += 1;
                    continue;
                }
                // Otherwise, return the edge:
                _ => {
                    return Some(edge.clone());
                }
            }
        }
    }

    fn finalize(mut self) -> Result<()> {
        match self.current() {
            Some(edge) => {
                Err(format!("Unrecognized child with label '{label}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.", label = edge.label))
            }
            None => {
                Ok(())
            },
        }
    }
}
