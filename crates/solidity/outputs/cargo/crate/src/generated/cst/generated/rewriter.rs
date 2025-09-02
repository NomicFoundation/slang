// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use crate::cst::{Edge, Node, NonterminalKind, NonterminalNode, TerminalKind, TerminalNode};

/// Trait to rewrite a CST.
pub trait BaseRewriter {
    /// Replaces the `node` with a new node. If the result is `None`, the node is removed from the tree.
    /// This function is typically the entry point of the rewrite operation.
    fn rewrite_node(&mut self, node: &Node) -> Option<Node> {
        match node {
            Node::Terminal(node) => self.rewrite_terminal_node(node),
            Node::Nonterminal(node) => self.rewrite_nonterminal_node(node),
        }
    }

    /// Rewrites a non-terminal node. Typically called from `rewrite_node`.
    #[allow(clippy::too_many_lines)]
    fn rewrite_nonterminal_node(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        match node.kind {
            NonterminalKind::AbicoderPragma => self.rewrite_abicoder_pragma(node),

            NonterminalKind::AbicoderVersion => self.rewrite_abicoder_version(node),

            NonterminalKind::AdditiveExpression => self.rewrite_additive_expression(node),

            NonterminalKind::AddressType => self.rewrite_address_type(node),

            NonterminalKind::AndExpression => self.rewrite_and_expression(node),

            NonterminalKind::ArgumentsDeclaration => self.rewrite_arguments_declaration(node),

            NonterminalKind::ArrayExpression => self.rewrite_array_expression(node),

            NonterminalKind::ArrayTypeName => self.rewrite_array_type_name(node),

            NonterminalKind::ArrayValues => self.rewrite_array_values(node),

            NonterminalKind::AssemblyFlags => self.rewrite_assembly_flags(node),

            NonterminalKind::AssemblyFlagsDeclaration => {
                self.rewrite_assembly_flags_declaration(node)
            }

            NonterminalKind::AssemblyStatement => self.rewrite_assembly_statement(node),

            NonterminalKind::AssignmentExpression => self.rewrite_assignment_expression(node),

            NonterminalKind::BitwiseAndExpression => self.rewrite_bitwise_and_expression(node),

            NonterminalKind::BitwiseOrExpression => self.rewrite_bitwise_or_expression(node),

            NonterminalKind::BitwiseXorExpression => self.rewrite_bitwise_xor_expression(node),

            NonterminalKind::Block => self.rewrite_block(node),

            NonterminalKind::BreakStatement => self.rewrite_break_statement(node),

            NonterminalKind::CallOptions => self.rewrite_call_options(node),

            NonterminalKind::CallOptionsExpression => self.rewrite_call_options_expression(node),

            NonterminalKind::CatchClause => self.rewrite_catch_clause(node),

            NonterminalKind::CatchClauseError => self.rewrite_catch_clause_error(node),

            NonterminalKind::CatchClauses => self.rewrite_catch_clauses(node),

            NonterminalKind::ConditionalExpression => self.rewrite_conditional_expression(node),

            NonterminalKind::ConstantDefinition => self.rewrite_constant_definition(node),

            NonterminalKind::ConstructorAttribute => self.rewrite_constructor_attribute(node),

            NonterminalKind::ConstructorAttributes => self.rewrite_constructor_attributes(node),

            NonterminalKind::ConstructorDefinition => self.rewrite_constructor_definition(node),

            NonterminalKind::ContinueStatement => self.rewrite_continue_statement(node),

            NonterminalKind::ContractDefinition => self.rewrite_contract_definition(node),

            NonterminalKind::ContractMember => self.rewrite_contract_member(node),

            NonterminalKind::ContractMembers => self.rewrite_contract_members(node),

            NonterminalKind::ContractSpecifier => self.rewrite_contract_specifier(node),

            NonterminalKind::ContractSpecifiers => self.rewrite_contract_specifiers(node),

            NonterminalKind::DecimalNumberExpression => {
                self.rewrite_decimal_number_expression(node)
            }

            NonterminalKind::DoWhileStatement => self.rewrite_do_while_statement(node),

            NonterminalKind::ElementaryType => self.rewrite_elementary_type(node),

            NonterminalKind::ElseBranch => self.rewrite_else_branch(node),

            NonterminalKind::EmitStatement => self.rewrite_emit_statement(node),

            NonterminalKind::EnumDefinition => self.rewrite_enum_definition(node),

            NonterminalKind::EnumMembers => self.rewrite_enum_members(node),

            NonterminalKind::EqualityExpression => self.rewrite_equality_expression(node),

            NonterminalKind::ErrorDefinition => self.rewrite_error_definition(node),

            NonterminalKind::ErrorParameter => self.rewrite_error_parameter(node),

            NonterminalKind::ErrorParameters => self.rewrite_error_parameters(node),

            NonterminalKind::ErrorParametersDeclaration => {
                self.rewrite_error_parameters_declaration(node)
            }

            NonterminalKind::EventDefinition => self.rewrite_event_definition(node),

            NonterminalKind::EventParameter => self.rewrite_event_parameter(node),

            NonterminalKind::EventParameters => self.rewrite_event_parameters(node),

            NonterminalKind::EventParametersDeclaration => {
                self.rewrite_event_parameters_declaration(node)
            }

            NonterminalKind::ExperimentalFeature => self.rewrite_experimental_feature(node),

            NonterminalKind::ExperimentalPragma => self.rewrite_experimental_pragma(node),

            NonterminalKind::ExponentiationExpression => {
                self.rewrite_exponentiation_expression(node)
            }

            NonterminalKind::Expression => self.rewrite_expression(node),

            NonterminalKind::ExpressionStatement => self.rewrite_expression_statement(node),

            NonterminalKind::FallbackFunctionAttribute => {
                self.rewrite_fallback_function_attribute(node)
            }

            NonterminalKind::FallbackFunctionAttributes => {
                self.rewrite_fallback_function_attributes(node)
            }

            NonterminalKind::FallbackFunctionDefinition => {
                self.rewrite_fallback_function_definition(node)
            }

            NonterminalKind::ForStatement => self.rewrite_for_statement(node),

            NonterminalKind::ForStatementCondition => self.rewrite_for_statement_condition(node),

            NonterminalKind::ForStatementInitialization => {
                self.rewrite_for_statement_initialization(node)
            }

            NonterminalKind::FunctionAttribute => self.rewrite_function_attribute(node),

            NonterminalKind::FunctionAttributes => self.rewrite_function_attributes(node),

            NonterminalKind::FunctionBody => self.rewrite_function_body(node),

            NonterminalKind::FunctionCallExpression => self.rewrite_function_call_expression(node),

            NonterminalKind::FunctionDefinition => self.rewrite_function_definition(node),

            NonterminalKind::FunctionName => self.rewrite_function_name(node),

            NonterminalKind::FunctionType => self.rewrite_function_type(node),

            NonterminalKind::FunctionTypeAttribute => self.rewrite_function_type_attribute(node),

            NonterminalKind::FunctionTypeAttributes => self.rewrite_function_type_attributes(node),

            NonterminalKind::HexNumberExpression => self.rewrite_hex_number_expression(node),

            NonterminalKind::HexStringLiteral => self.rewrite_hex_string_literal(node),

            NonterminalKind::HexStringLiterals => self.rewrite_hex_string_literals(node),

            NonterminalKind::IdentifierPath => self.rewrite_identifier_path(node),

            NonterminalKind::IfStatement => self.rewrite_if_statement(node),

            NonterminalKind::ImportAlias => self.rewrite_import_alias(node),

            NonterminalKind::ImportClause => self.rewrite_import_clause(node),

            NonterminalKind::ImportDeconstruction => self.rewrite_import_deconstruction(node),

            NonterminalKind::ImportDeconstructionSymbol => {
                self.rewrite_import_deconstruction_symbol(node)
            }

            NonterminalKind::ImportDeconstructionSymbols => {
                self.rewrite_import_deconstruction_symbols(node)
            }

            NonterminalKind::ImportDirective => self.rewrite_import_directive(node),

            NonterminalKind::IndexAccessEnd => self.rewrite_index_access_end(node),

            NonterminalKind::IndexAccessExpression => self.rewrite_index_access_expression(node),

            NonterminalKind::InequalityExpression => self.rewrite_inequality_expression(node),

            NonterminalKind::InheritanceSpecifier => self.rewrite_inheritance_specifier(node),

            NonterminalKind::InheritanceType => self.rewrite_inheritance_type(node),

            NonterminalKind::InheritanceTypes => self.rewrite_inheritance_types(node),

            NonterminalKind::InterfaceDefinition => self.rewrite_interface_definition(node),

            NonterminalKind::InterfaceMembers => self.rewrite_interface_members(node),

            NonterminalKind::LibraryDefinition => self.rewrite_library_definition(node),

            NonterminalKind::LibraryMembers => self.rewrite_library_members(node),

            NonterminalKind::MappingKey => self.rewrite_mapping_key(node),

            NonterminalKind::MappingKeyType => self.rewrite_mapping_key_type(node),

            NonterminalKind::MappingType => self.rewrite_mapping_type(node),

            NonterminalKind::MappingValue => self.rewrite_mapping_value(node),

            NonterminalKind::MemberAccessExpression => self.rewrite_member_access_expression(node),

            NonterminalKind::ModifierAttribute => self.rewrite_modifier_attribute(node),

            NonterminalKind::ModifierAttributes => self.rewrite_modifier_attributes(node),

            NonterminalKind::ModifierDefinition => self.rewrite_modifier_definition(node),

            NonterminalKind::ModifierInvocation => self.rewrite_modifier_invocation(node),

            NonterminalKind::MultiplicativeExpression => {
                self.rewrite_multiplicative_expression(node)
            }

            NonterminalKind::NamedArgument => self.rewrite_named_argument(node),

            NonterminalKind::NamedArgumentGroup => self.rewrite_named_argument_group(node),

            NonterminalKind::NamedArguments => self.rewrite_named_arguments(node),

            NonterminalKind::NamedArgumentsDeclaration => {
                self.rewrite_named_arguments_declaration(node)
            }

            NonterminalKind::NamedImport => self.rewrite_named_import(node),

            NonterminalKind::NewExpression => self.rewrite_new_expression(node),

            NonterminalKind::NumberUnit => self.rewrite_number_unit(node),

            NonterminalKind::OrExpression => self.rewrite_or_expression(node),

            NonterminalKind::OverridePaths => self.rewrite_override_paths(node),

            NonterminalKind::OverridePathsDeclaration => {
                self.rewrite_override_paths_declaration(node)
            }

            NonterminalKind::OverrideSpecifier => self.rewrite_override_specifier(node),

            NonterminalKind::Parameter => self.rewrite_parameter(node),

            NonterminalKind::Parameters => self.rewrite_parameters(node),

            NonterminalKind::ParametersDeclaration => self.rewrite_parameters_declaration(node),

            NonterminalKind::PathImport => self.rewrite_path_import(node),

            NonterminalKind::PositionalArguments => self.rewrite_positional_arguments(node),

            NonterminalKind::PositionalArgumentsDeclaration => {
                self.rewrite_positional_arguments_declaration(node)
            }

            NonterminalKind::PostfixExpression => self.rewrite_postfix_expression(node),

            NonterminalKind::Pragma => self.rewrite_pragma(node),

            NonterminalKind::PragmaDirective => self.rewrite_pragma_directive(node),

            NonterminalKind::PrefixExpression => self.rewrite_prefix_expression(node),

            NonterminalKind::ReceiveFunctionAttribute => {
                self.rewrite_receive_function_attribute(node)
            }

            NonterminalKind::ReceiveFunctionAttributes => {
                self.rewrite_receive_function_attributes(node)
            }

            NonterminalKind::ReceiveFunctionDefinition => {
                self.rewrite_receive_function_definition(node)
            }

            NonterminalKind::ReturnStatement => self.rewrite_return_statement(node),

            NonterminalKind::ReturnsDeclaration => self.rewrite_returns_declaration(node),

            NonterminalKind::RevertStatement => self.rewrite_revert_statement(node),

            NonterminalKind::ShiftExpression => self.rewrite_shift_expression(node),

            NonterminalKind::SimpleVersionLiteral => self.rewrite_simple_version_literal(node),

            NonterminalKind::SourceUnit => self.rewrite_source_unit(node),

            NonterminalKind::SourceUnitMember => self.rewrite_source_unit_member(node),

            NonterminalKind::SourceUnitMembers => self.rewrite_source_unit_members(node),

            NonterminalKind::StateVariableAttribute => self.rewrite_state_variable_attribute(node),

            NonterminalKind::StateVariableAttributes => {
                self.rewrite_state_variable_attributes(node)
            }

            NonterminalKind::StateVariableDefinition => {
                self.rewrite_state_variable_definition(node)
            }

            NonterminalKind::StateVariableDefinitionValue => {
                self.rewrite_state_variable_definition_value(node)
            }

            NonterminalKind::Statement => self.rewrite_statement(node),

            NonterminalKind::Statements => self.rewrite_statements(node),

            NonterminalKind::StorageLayoutSpecifier => self.rewrite_storage_layout_specifier(node),

            NonterminalKind::StorageLocation => self.rewrite_storage_location(node),

            NonterminalKind::StringExpression => self.rewrite_string_expression(node),

            NonterminalKind::StringLiteral => self.rewrite_string_literal(node),

            NonterminalKind::StringLiterals => self.rewrite_string_literals(node),

            NonterminalKind::StructDefinition => self.rewrite_struct_definition(node),

            NonterminalKind::StructMember => self.rewrite_struct_member(node),

            NonterminalKind::StructMembers => self.rewrite_struct_members(node),

            NonterminalKind::ThrowStatement => self.rewrite_throw_statement(node),

            NonterminalKind::TryStatement => self.rewrite_try_statement(node),

            NonterminalKind::TupleDeconstructionElement => {
                self.rewrite_tuple_deconstruction_element(node)
            }

            NonterminalKind::TupleDeconstructionElements => {
                self.rewrite_tuple_deconstruction_elements(node)
            }

            NonterminalKind::TupleDeconstructionStatement => {
                self.rewrite_tuple_deconstruction_statement(node)
            }

            NonterminalKind::TupleExpression => self.rewrite_tuple_expression(node),

            NonterminalKind::TupleMember => self.rewrite_tuple_member(node),

            NonterminalKind::TupleValue => self.rewrite_tuple_value(node),

            NonterminalKind::TupleValues => self.rewrite_tuple_values(node),

            NonterminalKind::TypeExpression => self.rewrite_type_expression(node),

            NonterminalKind::TypeName => self.rewrite_type_name(node),

            NonterminalKind::TypedTupleMember => self.rewrite_typed_tuple_member(node),

            NonterminalKind::UncheckedBlock => self.rewrite_unchecked_block(node),

            NonterminalKind::UnicodeStringLiteral => self.rewrite_unicode_string_literal(node),

            NonterminalKind::UnicodeStringLiterals => self.rewrite_unicode_string_literals(node),

            NonterminalKind::UnnamedFunctionAttribute => {
                self.rewrite_unnamed_function_attribute(node)
            }

            NonterminalKind::UnnamedFunctionAttributes => {
                self.rewrite_unnamed_function_attributes(node)
            }

            NonterminalKind::UnnamedFunctionDefinition => {
                self.rewrite_unnamed_function_definition(node)
            }

            NonterminalKind::UntypedTupleMember => self.rewrite_untyped_tuple_member(node),

            NonterminalKind::UserDefinedValueTypeDefinition => {
                self.rewrite_user_defined_value_type_definition(node)
            }

            NonterminalKind::UsingAlias => self.rewrite_using_alias(node),

            NonterminalKind::UsingClause => self.rewrite_using_clause(node),

            NonterminalKind::UsingDeconstruction => self.rewrite_using_deconstruction(node),

            NonterminalKind::UsingDeconstructionSymbol => {
                self.rewrite_using_deconstruction_symbol(node)
            }

            NonterminalKind::UsingDeconstructionSymbols => {
                self.rewrite_using_deconstruction_symbols(node)
            }

            NonterminalKind::UsingDirective => self.rewrite_using_directive(node),

            NonterminalKind::UsingOperator => self.rewrite_using_operator(node),

            NonterminalKind::UsingTarget => self.rewrite_using_target(node),

            NonterminalKind::VariableDeclarationStatement => {
                self.rewrite_variable_declaration_statement(node)
            }

            NonterminalKind::VariableDeclarationType => {
                self.rewrite_variable_declaration_type(node)
            }

            NonterminalKind::VariableDeclarationValue => {
                self.rewrite_variable_declaration_value(node)
            }

            NonterminalKind::VersionExpression => self.rewrite_version_expression(node),

            NonterminalKind::VersionExpressionSet => self.rewrite_version_expression_set(node),

            NonterminalKind::VersionExpressionSets => self.rewrite_version_expression_sets(node),

            NonterminalKind::VersionLiteral => self.rewrite_version_literal(node),

            NonterminalKind::VersionOperator => self.rewrite_version_operator(node),

            NonterminalKind::VersionPragma => self.rewrite_version_pragma(node),

            NonterminalKind::VersionRange => self.rewrite_version_range(node),

            NonterminalKind::VersionTerm => self.rewrite_version_term(node),

            NonterminalKind::WhileStatement => self.rewrite_while_statement(node),

            NonterminalKind::YulArguments => self.rewrite_yul_arguments(node),

            NonterminalKind::YulAssignmentOperator => self.rewrite_yul_assignment_operator(node),

            NonterminalKind::YulBlock => self.rewrite_yul_block(node),

            NonterminalKind::YulBreakStatement => self.rewrite_yul_break_statement(node),

            NonterminalKind::YulColonAndEqual => self.rewrite_yul_colon_and_equal(node),

            NonterminalKind::YulContinueStatement => self.rewrite_yul_continue_statement(node),

            NonterminalKind::YulDefaultCase => self.rewrite_yul_default_case(node),

            NonterminalKind::YulEqualAndColon => self.rewrite_yul_equal_and_colon(node),

            NonterminalKind::YulExpression => self.rewrite_yul_expression(node),

            NonterminalKind::YulForStatement => self.rewrite_yul_for_statement(node),

            NonterminalKind::YulFunctionCallExpression => {
                self.rewrite_yul_function_call_expression(node)
            }

            NonterminalKind::YulFunctionDefinition => self.rewrite_yul_function_definition(node),

            NonterminalKind::YulIfStatement => self.rewrite_yul_if_statement(node),

            NonterminalKind::YulLabel => self.rewrite_yul_label(node),

            NonterminalKind::YulLeaveStatement => self.rewrite_yul_leave_statement(node),

            NonterminalKind::YulLiteral => self.rewrite_yul_literal(node),

            NonterminalKind::YulParameters => self.rewrite_yul_parameters(node),

            NonterminalKind::YulParametersDeclaration => {
                self.rewrite_yul_parameters_declaration(node)
            }

            NonterminalKind::YulPath => self.rewrite_yul_path(node),

            NonterminalKind::YulPaths => self.rewrite_yul_paths(node),

            NonterminalKind::YulReturnsDeclaration => self.rewrite_yul_returns_declaration(node),

            NonterminalKind::YulStackAssignmentOperator => {
                self.rewrite_yul_stack_assignment_operator(node)
            }

            NonterminalKind::YulStackAssignmentStatement => {
                self.rewrite_yul_stack_assignment_statement(node)
            }

            NonterminalKind::YulStatement => self.rewrite_yul_statement(node),

            NonterminalKind::YulStatements => self.rewrite_yul_statements(node),

            NonterminalKind::YulSwitchCase => self.rewrite_yul_switch_case(node),

            NonterminalKind::YulSwitchCases => self.rewrite_yul_switch_cases(node),

            NonterminalKind::YulSwitchStatement => self.rewrite_yul_switch_statement(node),

            NonterminalKind::YulValueCase => self.rewrite_yul_value_case(node),

            NonterminalKind::YulVariableAssignmentStatement => {
                self.rewrite_yul_variable_assignment_statement(node)
            }

            NonterminalKind::YulVariableDeclarationStatement => {
                self.rewrite_yul_variable_declaration_statement(node)
            }

            NonterminalKind::YulVariableDeclarationValue => {
                self.rewrite_yul_variable_declaration_value(node)
            }

            NonterminalKind::YulVariableNames => self.rewrite_yul_variable_names(node),
        }
    }

    /// Rewrites a terminal node. Typically called from `rewrite_node`.
    #[allow(clippy::too_many_lines)]
    fn rewrite_terminal_node(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        match node.kind {
            TerminalKind::ABIEncoderV2Keyword => self.rewrite_abi_encoder_v2_keyword(node),

            TerminalKind::AbicoderKeyword => self.rewrite_abicoder_keyword(node),

            TerminalKind::AbicoderV1Keyword => self.rewrite_abicoder_v1_keyword(node),

            TerminalKind::AbicoderV2Keyword => self.rewrite_abicoder_v2_keyword(node),

            TerminalKind::AbstractKeyword => self.rewrite_abstract_keyword(node),

            TerminalKind::AddressKeyword => self.rewrite_address_keyword(node),

            TerminalKind::AfterKeyword => self.rewrite_after_keyword(node),

            TerminalKind::AliasKeyword => self.rewrite_alias_keyword(node),

            TerminalKind::Ampersand => self.rewrite_ampersand(node),

            TerminalKind::AmpersandAmpersand => self.rewrite_ampersand_ampersand(node),

            TerminalKind::AmpersandEqual => self.rewrite_ampersand_equal(node),

            TerminalKind::AnonymousKeyword => self.rewrite_anonymous_keyword(node),

            TerminalKind::ApplyKeyword => self.rewrite_apply_keyword(node),

            TerminalKind::AsKeyword => self.rewrite_as_keyword(node),

            TerminalKind::AssemblyKeyword => self.rewrite_assembly_keyword(node),

            TerminalKind::Asterisk => self.rewrite_asterisk(node),

            TerminalKind::AsteriskAsterisk => self.rewrite_asterisk_asterisk(node),

            TerminalKind::AsteriskEqual => self.rewrite_asterisk_equal(node),

            TerminalKind::AtKeyword => self.rewrite_at_keyword(node),

            TerminalKind::AutoKeyword => self.rewrite_auto_keyword(node),

            TerminalKind::Bang => self.rewrite_bang(node),

            TerminalKind::BangEqual => self.rewrite_bang_equal(node),

            TerminalKind::Bar => self.rewrite_bar(node),

            TerminalKind::BarBar => self.rewrite_bar_bar(node),

            TerminalKind::BarEqual => self.rewrite_bar_equal(node),

            TerminalKind::BoolKeyword => self.rewrite_bool_keyword(node),

            TerminalKind::BreakKeyword => self.rewrite_break_keyword(node),

            TerminalKind::ByteKeyword => self.rewrite_byte_keyword(node),

            TerminalKind::BytesKeyword => self.rewrite_bytes_keyword(node),

            TerminalKind::CallDataKeyword => self.rewrite_call_data_keyword(node),

            TerminalKind::Caret => self.rewrite_caret(node),

            TerminalKind::CaretEqual => self.rewrite_caret_equal(node),

            TerminalKind::CaseKeyword => self.rewrite_case_keyword(node),

            TerminalKind::CatchKeyword => self.rewrite_catch_keyword(node),

            TerminalKind::CloseBrace => self.rewrite_close_brace(node),

            TerminalKind::CloseBracket => self.rewrite_close_bracket(node),

            TerminalKind::CloseParen => self.rewrite_close_paren(node),

            TerminalKind::Colon => self.rewrite_colon(node),

            TerminalKind::ColonEqual => self.rewrite_colon_equal(node),

            TerminalKind::Comma => self.rewrite_comma(node),

            TerminalKind::ConstantKeyword => self.rewrite_constant_keyword(node),

            TerminalKind::ConstructorKeyword => self.rewrite_constructor_keyword(node),

            TerminalKind::ContinueKeyword => self.rewrite_continue_keyword(node),

            TerminalKind::ContractKeyword => self.rewrite_contract_keyword(node),

            TerminalKind::CopyOfKeyword => self.rewrite_copy_of_keyword(node),

            TerminalKind::DaysKeyword => self.rewrite_days_keyword(node),

            TerminalKind::DecimalLiteral => self.rewrite_decimal_literal(node),

            TerminalKind::DefaultKeyword => self.rewrite_default_keyword(node),

            TerminalKind::DefineKeyword => self.rewrite_define_keyword(node),

            TerminalKind::DeleteKeyword => self.rewrite_delete_keyword(node),

            TerminalKind::DoKeyword => self.rewrite_do_keyword(node),

            TerminalKind::DoubleQuotedHexStringLiteral => {
                self.rewrite_double_quoted_hex_string_literal(node)
            }

            TerminalKind::DoubleQuotedStringLiteral => {
                self.rewrite_double_quoted_string_literal(node)
            }

            TerminalKind::DoubleQuotedUnicodeStringLiteral => {
                self.rewrite_double_quoted_unicode_string_literal(node)
            }

            TerminalKind::DoubleQuotedVersionLiteral => {
                self.rewrite_double_quoted_version_literal(node)
            }

            TerminalKind::ElseKeyword => self.rewrite_else_keyword(node),

            TerminalKind::EmitKeyword => self.rewrite_emit_keyword(node),

            TerminalKind::EndOfLine => self.rewrite_end_of_line(node),

            TerminalKind::EnumKeyword => self.rewrite_enum_keyword(node),

            TerminalKind::Equal => self.rewrite_equal(node),

            TerminalKind::EqualColon => self.rewrite_equal_colon(node),

            TerminalKind::EqualEqual => self.rewrite_equal_equal(node),

            TerminalKind::EqualGreaterThan => self.rewrite_equal_greater_than(node),

            TerminalKind::ErrorKeyword => self.rewrite_error_keyword(node),

            TerminalKind::EtherKeyword => self.rewrite_ether_keyword(node),

            TerminalKind::EventKeyword => self.rewrite_event_keyword(node),

            TerminalKind::ExperimentalKeyword => self.rewrite_experimental_keyword(node),

            TerminalKind::ExternalKeyword => self.rewrite_external_keyword(node),

            TerminalKind::FallbackKeyword => self.rewrite_fallback_keyword(node),

            TerminalKind::FalseKeyword => self.rewrite_false_keyword(node),

            TerminalKind::FinalKeyword => self.rewrite_final_keyword(node),

            TerminalKind::FinneyKeyword => self.rewrite_finney_keyword(node),

            TerminalKind::FixedKeyword => self.rewrite_fixed_keyword(node),

            TerminalKind::ForKeyword => self.rewrite_for_keyword(node),

            TerminalKind::FromKeyword => self.rewrite_from_keyword(node),

            TerminalKind::FunctionKeyword => self.rewrite_function_keyword(node),

            TerminalKind::GlobalKeyword => self.rewrite_global_keyword(node),

            TerminalKind::GreaterThan => self.rewrite_greater_than(node),

            TerminalKind::GreaterThanEqual => self.rewrite_greater_than_equal(node),

            TerminalKind::GreaterThanGreaterThan => self.rewrite_greater_than_greater_than(node),

            TerminalKind::GreaterThanGreaterThanEqual => {
                self.rewrite_greater_than_greater_than_equal(node)
            }

            TerminalKind::GreaterThanGreaterThanGreaterThan => {
                self.rewrite_greater_than_greater_than_greater_than(node)
            }

            TerminalKind::GreaterThanGreaterThanGreaterThanEqual => {
                self.rewrite_greater_than_greater_than_greater_than_equal(node)
            }

            TerminalKind::GweiKeyword => self.rewrite_gwei_keyword(node),

            TerminalKind::HexKeyword => self.rewrite_hex_keyword(node),

            TerminalKind::HexLiteral => self.rewrite_hex_literal(node),

            TerminalKind::HoursKeyword => self.rewrite_hours_keyword(node),

            TerminalKind::Identifier => self.rewrite_identifier(node),

            TerminalKind::IfKeyword => self.rewrite_if_keyword(node),

            TerminalKind::ImmutableKeyword => self.rewrite_immutable_keyword(node),

            TerminalKind::ImplementsKeyword => self.rewrite_implements_keyword(node),

            TerminalKind::ImportKeyword => self.rewrite_import_keyword(node),

            TerminalKind::InKeyword => self.rewrite_in_keyword(node),

            TerminalKind::IndexedKeyword => self.rewrite_indexed_keyword(node),

            TerminalKind::InlineKeyword => self.rewrite_inline_keyword(node),

            TerminalKind::IntKeyword => self.rewrite_int_keyword(node),

            TerminalKind::InterfaceKeyword => self.rewrite_interface_keyword(node),

            TerminalKind::InternalKeyword => self.rewrite_internal_keyword(node),

            TerminalKind::IsKeyword => self.rewrite_is_keyword(node),

            TerminalKind::LayoutKeyword => self.rewrite_layout_keyword(node),

            TerminalKind::LessThan => self.rewrite_less_than(node),

            TerminalKind::LessThanEqual => self.rewrite_less_than_equal(node),

            TerminalKind::LessThanLessThan => self.rewrite_less_than_less_than(node),

            TerminalKind::LessThanLessThanEqual => self.rewrite_less_than_less_than_equal(node),

            TerminalKind::LetKeyword => self.rewrite_let_keyword(node),

            TerminalKind::LibraryKeyword => self.rewrite_library_keyword(node),

            TerminalKind::MacroKeyword => self.rewrite_macro_keyword(node),

            TerminalKind::MappingKeyword => self.rewrite_mapping_keyword(node),

            TerminalKind::MatchKeyword => self.rewrite_match_keyword(node),

            TerminalKind::MemoryKeyword => self.rewrite_memory_keyword(node),

            TerminalKind::Minus => self.rewrite_minus(node),

            TerminalKind::MinusEqual => self.rewrite_minus_equal(node),

            TerminalKind::MinusGreaterThan => self.rewrite_minus_greater_than(node),

            TerminalKind::MinusMinus => self.rewrite_minus_minus(node),

            TerminalKind::MinutesKeyword => self.rewrite_minutes_keyword(node),

            TerminalKind::ModifierKeyword => self.rewrite_modifier_keyword(node),

            TerminalKind::MultiLineComment => self.rewrite_multi_line_comment(node),

            TerminalKind::MultiLineNatSpecComment => self.rewrite_multi_line_nat_spec_comment(node),

            TerminalKind::MutableKeyword => self.rewrite_mutable_keyword(node),

            TerminalKind::NewKeyword => self.rewrite_new_keyword(node),

            TerminalKind::NullKeyword => self.rewrite_null_keyword(node),

            TerminalKind::OfKeyword => self.rewrite_of_keyword(node),

            TerminalKind::OpenBrace => self.rewrite_open_brace(node),

            TerminalKind::OpenBracket => self.rewrite_open_bracket(node),

            TerminalKind::OpenParen => self.rewrite_open_paren(node),

            TerminalKind::OverrideKeyword => self.rewrite_override_keyword(node),

            TerminalKind::PartialKeyword => self.rewrite_partial_keyword(node),

            TerminalKind::PayableKeyword => self.rewrite_payable_keyword(node),

            TerminalKind::Percent => self.rewrite_percent(node),

            TerminalKind::PercentEqual => self.rewrite_percent_equal(node),

            TerminalKind::Period => self.rewrite_period(node),

            TerminalKind::Plus => self.rewrite_plus(node),

            TerminalKind::PlusEqual => self.rewrite_plus_equal(node),

            TerminalKind::PlusPlus => self.rewrite_plus_plus(node),

            TerminalKind::PragmaKeyword => self.rewrite_pragma_keyword(node),

            TerminalKind::PrivateKeyword => self.rewrite_private_keyword(node),

            TerminalKind::PromiseKeyword => self.rewrite_promise_keyword(node),

            TerminalKind::PublicKeyword => self.rewrite_public_keyword(node),

            TerminalKind::PureKeyword => self.rewrite_pure_keyword(node),

            TerminalKind::QuestionMark => self.rewrite_question_mark(node),

            TerminalKind::ReceiveKeyword => self.rewrite_receive_keyword(node),

            TerminalKind::ReferenceKeyword => self.rewrite_reference_keyword(node),

            TerminalKind::RelocatableKeyword => self.rewrite_relocatable_keyword(node),

            TerminalKind::ReturnKeyword => self.rewrite_return_keyword(node),

            TerminalKind::ReturnsKeyword => self.rewrite_returns_keyword(node),

            TerminalKind::RevertKeyword => self.rewrite_revert_keyword(node),

            TerminalKind::SMTCheckerKeyword => self.rewrite_smt_checker_keyword(node),

            TerminalKind::SealedKeyword => self.rewrite_sealed_keyword(node),

            TerminalKind::SecondsKeyword => self.rewrite_seconds_keyword(node),

            TerminalKind::Semicolon => self.rewrite_semicolon(node),

            TerminalKind::SingleLineComment => self.rewrite_single_line_comment(node),

            TerminalKind::SingleLineNatSpecComment => {
                self.rewrite_single_line_nat_spec_comment(node)
            }

            TerminalKind::SingleQuotedHexStringLiteral => {
                self.rewrite_single_quoted_hex_string_literal(node)
            }

            TerminalKind::SingleQuotedStringLiteral => {
                self.rewrite_single_quoted_string_literal(node)
            }

            TerminalKind::SingleQuotedUnicodeStringLiteral => {
                self.rewrite_single_quoted_unicode_string_literal(node)
            }

            TerminalKind::SingleQuotedVersionLiteral => {
                self.rewrite_single_quoted_version_literal(node)
            }

            TerminalKind::SizeOfKeyword => self.rewrite_size_of_keyword(node),

            TerminalKind::Slash => self.rewrite_slash(node),

            TerminalKind::SlashEqual => self.rewrite_slash_equal(node),

            TerminalKind::SolidityKeyword => self.rewrite_solidity_keyword(node),

            TerminalKind::StaticKeyword => self.rewrite_static_keyword(node),

            TerminalKind::StorageKeyword => self.rewrite_storage_keyword(node),

            TerminalKind::StringKeyword => self.rewrite_string_keyword(node),

            TerminalKind::StructKeyword => self.rewrite_struct_keyword(node),

            TerminalKind::SuperKeyword => self.rewrite_super_keyword(node),

            TerminalKind::SupportsKeyword => self.rewrite_supports_keyword(node),

            TerminalKind::SwitchKeyword => self.rewrite_switch_keyword(node),

            TerminalKind::SzaboKeyword => self.rewrite_szabo_keyword(node),

            TerminalKind::ThisKeyword => self.rewrite_this_keyword(node),

            TerminalKind::ThrowKeyword => self.rewrite_throw_keyword(node),

            TerminalKind::Tilde => self.rewrite_tilde(node),

            TerminalKind::TransientKeyword => self.rewrite_transient_keyword(node),

            TerminalKind::TrueKeyword => self.rewrite_true_keyword(node),

            TerminalKind::TryKeyword => self.rewrite_try_keyword(node),

            TerminalKind::TypeDefKeyword => self.rewrite_type_def_keyword(node),

            TerminalKind::TypeKeyword => self.rewrite_type_keyword(node),

            TerminalKind::TypeOfKeyword => self.rewrite_type_of_keyword(node),

            TerminalKind::UfixedKeyword => self.rewrite_ufixed_keyword(node),

            TerminalKind::UintKeyword => self.rewrite_uint_keyword(node),

            TerminalKind::UncheckedKeyword => self.rewrite_unchecked_keyword(node),

            TerminalKind::UsingKeyword => self.rewrite_using_keyword(node),

            TerminalKind::VarKeyword => self.rewrite_var_keyword(node),

            TerminalKind::VersionSpecifier => self.rewrite_version_specifier(node),

            TerminalKind::ViewKeyword => self.rewrite_view_keyword(node),

            TerminalKind::VirtualKeyword => self.rewrite_virtual_keyword(node),

            TerminalKind::WeeksKeyword => self.rewrite_weeks_keyword(node),

            TerminalKind::WeiKeyword => self.rewrite_wei_keyword(node),

            TerminalKind::WhileKeyword => self.rewrite_while_keyword(node),

            TerminalKind::Whitespace => self.rewrite_whitespace(node),

            TerminalKind::YearsKeyword => self.rewrite_years_keyword(node),

            TerminalKind::YulAbstractKeyword => self.rewrite_yul_abstract_keyword(node),

            TerminalKind::YulAfterKeyword => self.rewrite_yul_after_keyword(node),

            TerminalKind::YulAliasKeyword => self.rewrite_yul_alias_keyword(node),

            TerminalKind::YulAnonymousKeyword => self.rewrite_yul_anonymous_keyword(node),

            TerminalKind::YulApplyKeyword => self.rewrite_yul_apply_keyword(node),

            TerminalKind::YulAsKeyword => self.rewrite_yul_as_keyword(node),

            TerminalKind::YulAssemblyKeyword => self.rewrite_yul_assembly_keyword(node),

            TerminalKind::YulAutoKeyword => self.rewrite_yul_auto_keyword(node),

            TerminalKind::YulBoolKeyword => self.rewrite_yul_bool_keyword(node),

            TerminalKind::YulBreakKeyword => self.rewrite_yul_break_keyword(node),

            TerminalKind::YulBytesKeyword => self.rewrite_yul_bytes_keyword(node),

            TerminalKind::YulCallDataKeyword => self.rewrite_yul_call_data_keyword(node),

            TerminalKind::YulCaseKeyword => self.rewrite_yul_case_keyword(node),

            TerminalKind::YulCatchKeyword => self.rewrite_yul_catch_keyword(node),

            TerminalKind::YulConstantKeyword => self.rewrite_yul_constant_keyword(node),

            TerminalKind::YulConstructorKeyword => self.rewrite_yul_constructor_keyword(node),

            TerminalKind::YulContinueKeyword => self.rewrite_yul_continue_keyword(node),

            TerminalKind::YulContractKeyword => self.rewrite_yul_contract_keyword(node),

            TerminalKind::YulCopyOfKeyword => self.rewrite_yul_copy_of_keyword(node),

            TerminalKind::YulDaysKeyword => self.rewrite_yul_days_keyword(node),

            TerminalKind::YulDecimalLiteral => self.rewrite_yul_decimal_literal(node),

            TerminalKind::YulDefaultKeyword => self.rewrite_yul_default_keyword(node),

            TerminalKind::YulDefineKeyword => self.rewrite_yul_define_keyword(node),

            TerminalKind::YulDeleteKeyword => self.rewrite_yul_delete_keyword(node),

            TerminalKind::YulDoKeyword => self.rewrite_yul_do_keyword(node),

            TerminalKind::YulElseKeyword => self.rewrite_yul_else_keyword(node),

            TerminalKind::YulEmitKeyword => self.rewrite_yul_emit_keyword(node),

            TerminalKind::YulEnumKeyword => self.rewrite_yul_enum_keyword(node),

            TerminalKind::YulEtherKeyword => self.rewrite_yul_ether_keyword(node),

            TerminalKind::YulEventKeyword => self.rewrite_yul_event_keyword(node),

            TerminalKind::YulExternalKeyword => self.rewrite_yul_external_keyword(node),

            TerminalKind::YulFallbackKeyword => self.rewrite_yul_fallback_keyword(node),

            TerminalKind::YulFalseKeyword => self.rewrite_yul_false_keyword(node),

            TerminalKind::YulFinalKeyword => self.rewrite_yul_final_keyword(node),

            TerminalKind::YulFinneyKeyword => self.rewrite_yul_finney_keyword(node),

            TerminalKind::YulFixedKeyword => self.rewrite_yul_fixed_keyword(node),

            TerminalKind::YulForKeyword => self.rewrite_yul_for_keyword(node),

            TerminalKind::YulFunctionKeyword => self.rewrite_yul_function_keyword(node),

            TerminalKind::YulGweiKeyword => self.rewrite_yul_gwei_keyword(node),

            TerminalKind::YulHexKeyword => self.rewrite_yul_hex_keyword(node),

            TerminalKind::YulHexLiteral => self.rewrite_yul_hex_literal(node),

            TerminalKind::YulHoursKeyword => self.rewrite_yul_hours_keyword(node),

            TerminalKind::YulIdentifier => self.rewrite_yul_identifier(node),

            TerminalKind::YulIfKeyword => self.rewrite_yul_if_keyword(node),

            TerminalKind::YulImmutableKeyword => self.rewrite_yul_immutable_keyword(node),

            TerminalKind::YulImplementsKeyword => self.rewrite_yul_implements_keyword(node),

            TerminalKind::YulImportKeyword => self.rewrite_yul_import_keyword(node),

            TerminalKind::YulInKeyword => self.rewrite_yul_in_keyword(node),

            TerminalKind::YulIndexedKeyword => self.rewrite_yul_indexed_keyword(node),

            TerminalKind::YulInlineKeyword => self.rewrite_yul_inline_keyword(node),

            TerminalKind::YulIntKeyword => self.rewrite_yul_int_keyword(node),

            TerminalKind::YulInterfaceKeyword => self.rewrite_yul_interface_keyword(node),

            TerminalKind::YulInternalKeyword => self.rewrite_yul_internal_keyword(node),

            TerminalKind::YulIsKeyword => self.rewrite_yul_is_keyword(node),

            TerminalKind::YulLeaveKeyword => self.rewrite_yul_leave_keyword(node),

            TerminalKind::YulLetKeyword => self.rewrite_yul_let_keyword(node),

            TerminalKind::YulLibraryKeyword => self.rewrite_yul_library_keyword(node),

            TerminalKind::YulMacroKeyword => self.rewrite_yul_macro_keyword(node),

            TerminalKind::YulMappingKeyword => self.rewrite_yul_mapping_keyword(node),

            TerminalKind::YulMatchKeyword => self.rewrite_yul_match_keyword(node),

            TerminalKind::YulMemoryKeyword => self.rewrite_yul_memory_keyword(node),

            TerminalKind::YulMinutesKeyword => self.rewrite_yul_minutes_keyword(node),

            TerminalKind::YulModifierKeyword => self.rewrite_yul_modifier_keyword(node),

            TerminalKind::YulMutableKeyword => self.rewrite_yul_mutable_keyword(node),

            TerminalKind::YulNewKeyword => self.rewrite_yul_new_keyword(node),

            TerminalKind::YulNullKeyword => self.rewrite_yul_null_keyword(node),

            TerminalKind::YulOfKeyword => self.rewrite_yul_of_keyword(node),

            TerminalKind::YulOverrideKeyword => self.rewrite_yul_override_keyword(node),

            TerminalKind::YulPartialKeyword => self.rewrite_yul_partial_keyword(node),

            TerminalKind::YulPayableKeyword => self.rewrite_yul_payable_keyword(node),

            TerminalKind::YulPragmaKeyword => self.rewrite_yul_pragma_keyword(node),

            TerminalKind::YulPrivateKeyword => self.rewrite_yul_private_keyword(node),

            TerminalKind::YulPromiseKeyword => self.rewrite_yul_promise_keyword(node),

            TerminalKind::YulPublicKeyword => self.rewrite_yul_public_keyword(node),

            TerminalKind::YulPureKeyword => self.rewrite_yul_pure_keyword(node),

            TerminalKind::YulReceiveKeyword => self.rewrite_yul_receive_keyword(node),

            TerminalKind::YulReferenceKeyword => self.rewrite_yul_reference_keyword(node),

            TerminalKind::YulRelocatableKeyword => self.rewrite_yul_relocatable_keyword(node),

            TerminalKind::YulReturnsKeyword => self.rewrite_yul_returns_keyword(node),

            TerminalKind::YulSealedKeyword => self.rewrite_yul_sealed_keyword(node),

            TerminalKind::YulSecondsKeyword => self.rewrite_yul_seconds_keyword(node),

            TerminalKind::YulSizeOfKeyword => self.rewrite_yul_size_of_keyword(node),

            TerminalKind::YulStaticKeyword => self.rewrite_yul_static_keyword(node),

            TerminalKind::YulStorageKeyword => self.rewrite_yul_storage_keyword(node),

            TerminalKind::YulStringKeyword => self.rewrite_yul_string_keyword(node),

            TerminalKind::YulStructKeyword => self.rewrite_yul_struct_keyword(node),

            TerminalKind::YulSuperKeyword => self.rewrite_yul_super_keyword(node),

            TerminalKind::YulSupportsKeyword => self.rewrite_yul_supports_keyword(node),

            TerminalKind::YulSwitchKeyword => self.rewrite_yul_switch_keyword(node),

            TerminalKind::YulSzaboKeyword => self.rewrite_yul_szabo_keyword(node),

            TerminalKind::YulThisKeyword => self.rewrite_yul_this_keyword(node),

            TerminalKind::YulThrowKeyword => self.rewrite_yul_throw_keyword(node),

            TerminalKind::YulTrueKeyword => self.rewrite_yul_true_keyword(node),

            TerminalKind::YulTryKeyword => self.rewrite_yul_try_keyword(node),

            TerminalKind::YulTypeDefKeyword => self.rewrite_yul_type_def_keyword(node),

            TerminalKind::YulTypeKeyword => self.rewrite_yul_type_keyword(node),

            TerminalKind::YulTypeOfKeyword => self.rewrite_yul_type_of_keyword(node),

            TerminalKind::YulUfixedKeyword => self.rewrite_yul_ufixed_keyword(node),

            TerminalKind::YulUintKeyword => self.rewrite_yul_uint_keyword(node),

            TerminalKind::YulUncheckedKeyword => self.rewrite_yul_unchecked_keyword(node),

            TerminalKind::YulUsingKeyword => self.rewrite_yul_using_keyword(node),

            TerminalKind::YulVarKeyword => self.rewrite_yul_var_keyword(node),

            TerminalKind::YulViewKeyword => self.rewrite_yul_view_keyword(node),

            TerminalKind::YulVirtualKeyword => self.rewrite_yul_virtual_keyword(node),

            TerminalKind::YulWeeksKeyword => self.rewrite_yul_weeks_keyword(node),

            TerminalKind::YulWeiKeyword => self.rewrite_yul_wei_keyword(node),

            TerminalKind::YulWhileKeyword => self.rewrite_yul_while_keyword(node),

            TerminalKind::YulYearsKeyword => self.rewrite_yul_years_keyword(node),

            TerminalKind::UNRECOGNIZED => self.rewrite_unrecognized(node),

            TerminalKind::MISSING => self.rewrite_missing(node),
        }
    }

    /// Rewrites a `AbicoderPragma` node, recursively traversing the children (unless overriden).
    fn rewrite_abicoder_pragma(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `AbicoderVersion` node, recursively traversing the children (unless overriden).
    fn rewrite_abicoder_version(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `AdditiveExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_additive_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `AddressType` node, recursively traversing the children (unless overriden).
    fn rewrite_address_type(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `AndExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_and_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ArgumentsDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_arguments_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ArrayExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_array_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ArrayTypeName` node, recursively traversing the children (unless overriden).
    fn rewrite_array_type_name(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ArrayValues` node, recursively traversing the children (unless overriden).
    fn rewrite_array_values(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `AssemblyFlags` node, recursively traversing the children (unless overriden).
    fn rewrite_assembly_flags(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `AssemblyFlagsDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_assembly_flags_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `AssemblyStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_assembly_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `AssignmentExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_assignment_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `BitwiseAndExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_bitwise_and_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `BitwiseOrExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_bitwise_or_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `BitwiseXorExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_bitwise_xor_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `Block` node, recursively traversing the children (unless overriden).
    fn rewrite_block(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `BreakStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_break_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `CallOptions` node, recursively traversing the children (unless overriden).
    fn rewrite_call_options(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `CallOptionsExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_call_options_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `CatchClause` node, recursively traversing the children (unless overriden).
    fn rewrite_catch_clause(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `CatchClauseError` node, recursively traversing the children (unless overriden).
    fn rewrite_catch_clause_error(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `CatchClauses` node, recursively traversing the children (unless overriden).
    fn rewrite_catch_clauses(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ConditionalExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_conditional_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ConstantDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_constant_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ConstructorAttribute` node, recursively traversing the children (unless overriden).
    fn rewrite_constructor_attribute(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ConstructorAttributes` node, recursively traversing the children (unless overriden).
    fn rewrite_constructor_attributes(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ConstructorDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_constructor_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ContinueStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_continue_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ContractDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_contract_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ContractMember` node, recursively traversing the children (unless overriden).
    fn rewrite_contract_member(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ContractMembers` node, recursively traversing the children (unless overriden).
    fn rewrite_contract_members(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ContractSpecifier` node, recursively traversing the children (unless overriden).
    fn rewrite_contract_specifier(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ContractSpecifiers` node, recursively traversing the children (unless overriden).
    fn rewrite_contract_specifiers(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `DecimalNumberExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_decimal_number_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `DoWhileStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_do_while_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ElementaryType` node, recursively traversing the children (unless overriden).
    fn rewrite_elementary_type(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ElseBranch` node, recursively traversing the children (unless overriden).
    fn rewrite_else_branch(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `EmitStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_emit_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `EnumDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_enum_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `EnumMembers` node, recursively traversing the children (unless overriden).
    fn rewrite_enum_members(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `EqualityExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_equality_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ErrorDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_error_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ErrorParameter` node, recursively traversing the children (unless overriden).
    fn rewrite_error_parameter(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ErrorParameters` node, recursively traversing the children (unless overriden).
    fn rewrite_error_parameters(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ErrorParametersDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_error_parameters_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `EventDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_event_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `EventParameter` node, recursively traversing the children (unless overriden).
    fn rewrite_event_parameter(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `EventParameters` node, recursively traversing the children (unless overriden).
    fn rewrite_event_parameters(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `EventParametersDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_event_parameters_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ExperimentalFeature` node, recursively traversing the children (unless overriden).
    fn rewrite_experimental_feature(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ExperimentalPragma` node, recursively traversing the children (unless overriden).
    fn rewrite_experimental_pragma(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ExponentiationExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_exponentiation_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `Expression` node, recursively traversing the children (unless overriden).
    fn rewrite_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ExpressionStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_expression_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FallbackFunctionAttribute` node, recursively traversing the children (unless overriden).
    fn rewrite_fallback_function_attribute(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FallbackFunctionAttributes` node, recursively traversing the children (unless overriden).
    fn rewrite_fallback_function_attributes(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FallbackFunctionDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_fallback_function_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ForStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_for_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ForStatementCondition` node, recursively traversing the children (unless overriden).
    fn rewrite_for_statement_condition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ForStatementInitialization` node, recursively traversing the children (unless overriden).
    fn rewrite_for_statement_initialization(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FunctionAttribute` node, recursively traversing the children (unless overriden).
    fn rewrite_function_attribute(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FunctionAttributes` node, recursively traversing the children (unless overriden).
    fn rewrite_function_attributes(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FunctionBody` node, recursively traversing the children (unless overriden).
    fn rewrite_function_body(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FunctionCallExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_function_call_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FunctionDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_function_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FunctionName` node, recursively traversing the children (unless overriden).
    fn rewrite_function_name(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FunctionType` node, recursively traversing the children (unless overriden).
    fn rewrite_function_type(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FunctionTypeAttribute` node, recursively traversing the children (unless overriden).
    fn rewrite_function_type_attribute(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `FunctionTypeAttributes` node, recursively traversing the children (unless overriden).
    fn rewrite_function_type_attributes(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `HexNumberExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_hex_number_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `HexStringLiteral` node, recursively traversing the children (unless overriden).
    fn rewrite_hex_string_literal(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `HexStringLiterals` node, recursively traversing the children (unless overriden).
    fn rewrite_hex_string_literals(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `IdentifierPath` node, recursively traversing the children (unless overriden).
    fn rewrite_identifier_path(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `IfStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_if_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ImportAlias` node, recursively traversing the children (unless overriden).
    fn rewrite_import_alias(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ImportClause` node, recursively traversing the children (unless overriden).
    fn rewrite_import_clause(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ImportDeconstruction` node, recursively traversing the children (unless overriden).
    fn rewrite_import_deconstruction(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ImportDeconstructionSymbol` node, recursively traversing the children (unless overriden).
    fn rewrite_import_deconstruction_symbol(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ImportDeconstructionSymbols` node, recursively traversing the children (unless overriden).
    fn rewrite_import_deconstruction_symbols(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ImportDirective` node, recursively traversing the children (unless overriden).
    fn rewrite_import_directive(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `IndexAccessEnd` node, recursively traversing the children (unless overriden).
    fn rewrite_index_access_end(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `IndexAccessExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_index_access_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `InequalityExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_inequality_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `InheritanceSpecifier` node, recursively traversing the children (unless overriden).
    fn rewrite_inheritance_specifier(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `InheritanceType` node, recursively traversing the children (unless overriden).
    fn rewrite_inheritance_type(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `InheritanceTypes` node, recursively traversing the children (unless overriden).
    fn rewrite_inheritance_types(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `InterfaceDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_interface_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `InterfaceMembers` node, recursively traversing the children (unless overriden).
    fn rewrite_interface_members(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `LibraryDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_library_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `LibraryMembers` node, recursively traversing the children (unless overriden).
    fn rewrite_library_members(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `MappingKey` node, recursively traversing the children (unless overriden).
    fn rewrite_mapping_key(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `MappingKeyType` node, recursively traversing the children (unless overriden).
    fn rewrite_mapping_key_type(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `MappingType` node, recursively traversing the children (unless overriden).
    fn rewrite_mapping_type(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `MappingValue` node, recursively traversing the children (unless overriden).
    fn rewrite_mapping_value(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `MemberAccessExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_member_access_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ModifierAttribute` node, recursively traversing the children (unless overriden).
    fn rewrite_modifier_attribute(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ModifierAttributes` node, recursively traversing the children (unless overriden).
    fn rewrite_modifier_attributes(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ModifierDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_modifier_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ModifierInvocation` node, recursively traversing the children (unless overriden).
    fn rewrite_modifier_invocation(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `MultiplicativeExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_multiplicative_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `NamedArgument` node, recursively traversing the children (unless overriden).
    fn rewrite_named_argument(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `NamedArgumentGroup` node, recursively traversing the children (unless overriden).
    fn rewrite_named_argument_group(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `NamedArguments` node, recursively traversing the children (unless overriden).
    fn rewrite_named_arguments(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `NamedArgumentsDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_named_arguments_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `NamedImport` node, recursively traversing the children (unless overriden).
    fn rewrite_named_import(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `NewExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_new_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `NumberUnit` node, recursively traversing the children (unless overriden).
    fn rewrite_number_unit(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `OrExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_or_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `OverridePaths` node, recursively traversing the children (unless overriden).
    fn rewrite_override_paths(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `OverridePathsDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_override_paths_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `OverrideSpecifier` node, recursively traversing the children (unless overriden).
    fn rewrite_override_specifier(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `Parameter` node, recursively traversing the children (unless overriden).
    fn rewrite_parameter(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `Parameters` node, recursively traversing the children (unless overriden).
    fn rewrite_parameters(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ParametersDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_parameters_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `PathImport` node, recursively traversing the children (unless overriden).
    fn rewrite_path_import(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `PositionalArguments` node, recursively traversing the children (unless overriden).
    fn rewrite_positional_arguments(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `PositionalArgumentsDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_positional_arguments_declaration(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `PostfixExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_postfix_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `Pragma` node, recursively traversing the children (unless overriden).
    fn rewrite_pragma(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `PragmaDirective` node, recursively traversing the children (unless overriden).
    fn rewrite_pragma_directive(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `PrefixExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_prefix_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ReceiveFunctionAttribute` node, recursively traversing the children (unless overriden).
    fn rewrite_receive_function_attribute(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ReceiveFunctionAttributes` node, recursively traversing the children (unless overriden).
    fn rewrite_receive_function_attributes(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ReceiveFunctionDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_receive_function_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ReturnStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_return_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ReturnsDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_returns_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `RevertStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_revert_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ShiftExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_shift_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `SimpleVersionLiteral` node, recursively traversing the children (unless overriden).
    fn rewrite_simple_version_literal(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `SourceUnit` node, recursively traversing the children (unless overriden).
    fn rewrite_source_unit(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `SourceUnitMember` node, recursively traversing the children (unless overriden).
    fn rewrite_source_unit_member(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `SourceUnitMembers` node, recursively traversing the children (unless overriden).
    fn rewrite_source_unit_members(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StateVariableAttribute` node, recursively traversing the children (unless overriden).
    fn rewrite_state_variable_attribute(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StateVariableAttributes` node, recursively traversing the children (unless overriden).
    fn rewrite_state_variable_attributes(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StateVariableDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_state_variable_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StateVariableDefinitionValue` node, recursively traversing the children (unless overriden).
    fn rewrite_state_variable_definition_value(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `Statement` node, recursively traversing the children (unless overriden).
    fn rewrite_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `Statements` node, recursively traversing the children (unless overriden).
    fn rewrite_statements(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StorageLayoutSpecifier` node, recursively traversing the children (unless overriden).
    fn rewrite_storage_layout_specifier(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StorageLocation` node, recursively traversing the children (unless overriden).
    fn rewrite_storage_location(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StringExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_string_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StringLiteral` node, recursively traversing the children (unless overriden).
    fn rewrite_string_literal(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StringLiterals` node, recursively traversing the children (unless overriden).
    fn rewrite_string_literals(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StructDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_struct_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StructMember` node, recursively traversing the children (unless overriden).
    fn rewrite_struct_member(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `StructMembers` node, recursively traversing the children (unless overriden).
    fn rewrite_struct_members(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ThrowStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_throw_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TryStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_try_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TupleDeconstructionElement` node, recursively traversing the children (unless overriden).
    fn rewrite_tuple_deconstruction_element(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TupleDeconstructionElements` node, recursively traversing the children (unless overriden).
    fn rewrite_tuple_deconstruction_elements(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TupleDeconstructionStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_tuple_deconstruction_statement(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TupleExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_tuple_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TupleMember` node, recursively traversing the children (unless overriden).
    fn rewrite_tuple_member(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TupleValue` node, recursively traversing the children (unless overriden).
    fn rewrite_tuple_value(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TupleValues` node, recursively traversing the children (unless overriden).
    fn rewrite_tuple_values(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TypeExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_type_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TypeName` node, recursively traversing the children (unless overriden).
    fn rewrite_type_name(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `TypedTupleMember` node, recursively traversing the children (unless overriden).
    fn rewrite_typed_tuple_member(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UncheckedBlock` node, recursively traversing the children (unless overriden).
    fn rewrite_unchecked_block(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UnicodeStringLiteral` node, recursively traversing the children (unless overriden).
    fn rewrite_unicode_string_literal(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UnicodeStringLiterals` node, recursively traversing the children (unless overriden).
    fn rewrite_unicode_string_literals(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UnnamedFunctionAttribute` node, recursively traversing the children (unless overriden).
    fn rewrite_unnamed_function_attribute(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UnnamedFunctionAttributes` node, recursively traversing the children (unless overriden).
    fn rewrite_unnamed_function_attributes(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UnnamedFunctionDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_unnamed_function_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UntypedTupleMember` node, recursively traversing the children (unless overriden).
    fn rewrite_untyped_tuple_member(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UserDefinedValueTypeDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_user_defined_value_type_definition(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UsingAlias` node, recursively traversing the children (unless overriden).
    fn rewrite_using_alias(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UsingClause` node, recursively traversing the children (unless overriden).
    fn rewrite_using_clause(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UsingDeconstruction` node, recursively traversing the children (unless overriden).
    fn rewrite_using_deconstruction(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UsingDeconstructionSymbol` node, recursively traversing the children (unless overriden).
    fn rewrite_using_deconstruction_symbol(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UsingDeconstructionSymbols` node, recursively traversing the children (unless overriden).
    fn rewrite_using_deconstruction_symbols(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UsingDirective` node, recursively traversing the children (unless overriden).
    fn rewrite_using_directive(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UsingOperator` node, recursively traversing the children (unless overriden).
    fn rewrite_using_operator(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `UsingTarget` node, recursively traversing the children (unless overriden).
    fn rewrite_using_target(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VariableDeclarationStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_variable_declaration_statement(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VariableDeclarationType` node, recursively traversing the children (unless overriden).
    fn rewrite_variable_declaration_type(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VariableDeclarationValue` node, recursively traversing the children (unless overriden).
    fn rewrite_variable_declaration_value(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VersionExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_version_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VersionExpressionSet` node, recursively traversing the children (unless overriden).
    fn rewrite_version_expression_set(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VersionExpressionSets` node, recursively traversing the children (unless overriden).
    fn rewrite_version_expression_sets(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VersionLiteral` node, recursively traversing the children (unless overriden).
    fn rewrite_version_literal(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VersionOperator` node, recursively traversing the children (unless overriden).
    fn rewrite_version_operator(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VersionPragma` node, recursively traversing the children (unless overriden).
    fn rewrite_version_pragma(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VersionRange` node, recursively traversing the children (unless overriden).
    fn rewrite_version_range(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `VersionTerm` node, recursively traversing the children (unless overriden).
    fn rewrite_version_term(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `WhileStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_while_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulArguments` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_arguments(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulAssignmentOperator` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_assignment_operator(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulBlock` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_block(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulBreakStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_break_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulColonAndEqual` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_colon_and_equal(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulContinueStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_continue_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulDefaultCase` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_default_case(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulEqualAndColon` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_equal_and_colon(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulForStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_for_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulFunctionCallExpression` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_function_call_expression(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulFunctionDefinition` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_function_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulIfStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_if_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulLabel` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_label(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulLeaveStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_leave_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulLiteral` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_literal(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulParameters` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_parameters(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulParametersDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_parameters_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulPath` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_path(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulPaths` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_paths(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulReturnsDeclaration` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_returns_declaration(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulStackAssignmentOperator` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_stack_assignment_operator(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulStackAssignmentStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_stack_assignment_statement(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulStatements` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_statements(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulSwitchCase` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_switch_case(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulSwitchCases` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_switch_cases(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulSwitchStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_switch_statement(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulValueCase` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_value_case(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulVariableAssignmentStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_variable_assignment_statement(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulVariableDeclarationStatement` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_variable_declaration_statement(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulVariableDeclarationValue` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_variable_declaration_value(
        &mut self,
        node: &Rc<NonterminalNode>,
    ) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `YulVariableNames` node, recursively traversing the children (unless overriden).
    fn rewrite_yul_variable_names(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
        Some(self.rewrite_children(node))
    }

    /// Rewrites a `ABIEncoderV2Keyword` node.
    fn rewrite_abi_encoder_v2_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AbicoderKeyword` node.
    fn rewrite_abicoder_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AbicoderV1Keyword` node.
    fn rewrite_abicoder_v1_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AbicoderV2Keyword` node.
    fn rewrite_abicoder_v2_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AbstractKeyword` node.
    fn rewrite_abstract_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AddressKeyword` node.
    fn rewrite_address_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AfterKeyword` node.
    fn rewrite_after_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AliasKeyword` node.
    fn rewrite_alias_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Ampersand` node.
    fn rewrite_ampersand(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AmpersandAmpersand` node.
    fn rewrite_ampersand_ampersand(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AmpersandEqual` node.
    fn rewrite_ampersand_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AnonymousKeyword` node.
    fn rewrite_anonymous_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ApplyKeyword` node.
    fn rewrite_apply_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AsKeyword` node.
    fn rewrite_as_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AssemblyKeyword` node.
    fn rewrite_assembly_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Asterisk` node.
    fn rewrite_asterisk(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AsteriskAsterisk` node.
    fn rewrite_asterisk_asterisk(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AsteriskEqual` node.
    fn rewrite_asterisk_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AtKeyword` node.
    fn rewrite_at_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `AutoKeyword` node.
    fn rewrite_auto_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Bang` node.
    fn rewrite_bang(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `BangEqual` node.
    fn rewrite_bang_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Bar` node.
    fn rewrite_bar(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `BarBar` node.
    fn rewrite_bar_bar(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `BarEqual` node.
    fn rewrite_bar_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `BoolKeyword` node.
    fn rewrite_bool_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `BreakKeyword` node.
    fn rewrite_break_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ByteKeyword` node.
    fn rewrite_byte_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `BytesKeyword` node.
    fn rewrite_bytes_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `CallDataKeyword` node.
    fn rewrite_call_data_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Caret` node.
    fn rewrite_caret(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `CaretEqual` node.
    fn rewrite_caret_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `CaseKeyword` node.
    fn rewrite_case_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `CatchKeyword` node.
    fn rewrite_catch_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `CloseBrace` node.
    fn rewrite_close_brace(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `CloseBracket` node.
    fn rewrite_close_bracket(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `CloseParen` node.
    fn rewrite_close_paren(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Colon` node.
    fn rewrite_colon(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ColonEqual` node.
    fn rewrite_colon_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Comma` node.
    fn rewrite_comma(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ConstantKeyword` node.
    fn rewrite_constant_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ConstructorKeyword` node.
    fn rewrite_constructor_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ContinueKeyword` node.
    fn rewrite_continue_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ContractKeyword` node.
    fn rewrite_contract_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `CopyOfKeyword` node.
    fn rewrite_copy_of_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DaysKeyword` node.
    fn rewrite_days_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DecimalLiteral` node.
    fn rewrite_decimal_literal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DefaultKeyword` node.
    fn rewrite_default_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DefineKeyword` node.
    fn rewrite_define_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DeleteKeyword` node.
    fn rewrite_delete_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DoKeyword` node.
    fn rewrite_do_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DoubleQuotedHexStringLiteral` node.
    fn rewrite_double_quoted_hex_string_literal(
        &mut self,
        node: &Rc<TerminalNode>,
    ) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DoubleQuotedStringLiteral` node.
    fn rewrite_double_quoted_string_literal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DoubleQuotedUnicodeStringLiteral` node.
    fn rewrite_double_quoted_unicode_string_literal(
        &mut self,
        node: &Rc<TerminalNode>,
    ) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `DoubleQuotedVersionLiteral` node.
    fn rewrite_double_quoted_version_literal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ElseKeyword` node.
    fn rewrite_else_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `EmitKeyword` node.
    fn rewrite_emit_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `EndOfLine` node.
    fn rewrite_end_of_line(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `EnumKeyword` node.
    fn rewrite_enum_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Equal` node.
    fn rewrite_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `EqualColon` node.
    fn rewrite_equal_colon(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `EqualEqual` node.
    fn rewrite_equal_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `EqualGreaterThan` node.
    fn rewrite_equal_greater_than(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ErrorKeyword` node.
    fn rewrite_error_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `EtherKeyword` node.
    fn rewrite_ether_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `EventKeyword` node.
    fn rewrite_event_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ExperimentalKeyword` node.
    fn rewrite_experimental_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ExternalKeyword` node.
    fn rewrite_external_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `FallbackKeyword` node.
    fn rewrite_fallback_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `FalseKeyword` node.
    fn rewrite_false_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `FinalKeyword` node.
    fn rewrite_final_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `FinneyKeyword` node.
    fn rewrite_finney_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `FixedKeyword` node.
    fn rewrite_fixed_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ForKeyword` node.
    fn rewrite_for_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `FromKeyword` node.
    fn rewrite_from_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `FunctionKeyword` node.
    fn rewrite_function_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `GlobalKeyword` node.
    fn rewrite_global_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `GreaterThan` node.
    fn rewrite_greater_than(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `GreaterThanEqual` node.
    fn rewrite_greater_than_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `GreaterThanGreaterThan` node.
    fn rewrite_greater_than_greater_than(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `GreaterThanGreaterThanEqual` node.
    fn rewrite_greater_than_greater_than_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `GreaterThanGreaterThanGreaterThan` node.
    fn rewrite_greater_than_greater_than_greater_than(
        &mut self,
        node: &Rc<TerminalNode>,
    ) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `GreaterThanGreaterThanGreaterThanEqual` node.
    fn rewrite_greater_than_greater_than_greater_than_equal(
        &mut self,
        node: &Rc<TerminalNode>,
    ) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `GweiKeyword` node.
    fn rewrite_gwei_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `HexKeyword` node.
    fn rewrite_hex_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `HexLiteral` node.
    fn rewrite_hex_literal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `HoursKeyword` node.
    fn rewrite_hours_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Identifier` node.
    fn rewrite_identifier(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `IfKeyword` node.
    fn rewrite_if_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ImmutableKeyword` node.
    fn rewrite_immutable_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ImplementsKeyword` node.
    fn rewrite_implements_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ImportKeyword` node.
    fn rewrite_import_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `InKeyword` node.
    fn rewrite_in_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `IndexedKeyword` node.
    fn rewrite_indexed_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `InlineKeyword` node.
    fn rewrite_inline_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `IntKeyword` node.
    fn rewrite_int_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `InterfaceKeyword` node.
    fn rewrite_interface_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `InternalKeyword` node.
    fn rewrite_internal_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `IsKeyword` node.
    fn rewrite_is_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `LayoutKeyword` node.
    fn rewrite_layout_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `LessThan` node.
    fn rewrite_less_than(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `LessThanEqual` node.
    fn rewrite_less_than_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `LessThanLessThan` node.
    fn rewrite_less_than_less_than(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `LessThanLessThanEqual` node.
    fn rewrite_less_than_less_than_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `LetKeyword` node.
    fn rewrite_let_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `LibraryKeyword` node.
    fn rewrite_library_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MacroKeyword` node.
    fn rewrite_macro_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MappingKeyword` node.
    fn rewrite_mapping_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MatchKeyword` node.
    fn rewrite_match_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MemoryKeyword` node.
    fn rewrite_memory_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Minus` node.
    fn rewrite_minus(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MinusEqual` node.
    fn rewrite_minus_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MinusGreaterThan` node.
    fn rewrite_minus_greater_than(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MinusMinus` node.
    fn rewrite_minus_minus(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MinutesKeyword` node.
    fn rewrite_minutes_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ModifierKeyword` node.
    fn rewrite_modifier_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MultiLineComment` node.
    fn rewrite_multi_line_comment(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MultiLineNatSpecComment` node.
    fn rewrite_multi_line_nat_spec_comment(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `MutableKeyword` node.
    fn rewrite_mutable_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `NewKeyword` node.
    fn rewrite_new_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `NullKeyword` node.
    fn rewrite_null_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `OfKeyword` node.
    fn rewrite_of_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `OpenBrace` node.
    fn rewrite_open_brace(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `OpenBracket` node.
    fn rewrite_open_bracket(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `OpenParen` node.
    fn rewrite_open_paren(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `OverrideKeyword` node.
    fn rewrite_override_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PartialKeyword` node.
    fn rewrite_partial_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PayableKeyword` node.
    fn rewrite_payable_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Percent` node.
    fn rewrite_percent(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PercentEqual` node.
    fn rewrite_percent_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Period` node.
    fn rewrite_period(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Plus` node.
    fn rewrite_plus(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PlusEqual` node.
    fn rewrite_plus_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PlusPlus` node.
    fn rewrite_plus_plus(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PragmaKeyword` node.
    fn rewrite_pragma_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PrivateKeyword` node.
    fn rewrite_private_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PromiseKeyword` node.
    fn rewrite_promise_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PublicKeyword` node.
    fn rewrite_public_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `PureKeyword` node.
    fn rewrite_pure_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `QuestionMark` node.
    fn rewrite_question_mark(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ReceiveKeyword` node.
    fn rewrite_receive_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ReferenceKeyword` node.
    fn rewrite_reference_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `RelocatableKeyword` node.
    fn rewrite_relocatable_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ReturnKeyword` node.
    fn rewrite_return_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ReturnsKeyword` node.
    fn rewrite_returns_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `RevertKeyword` node.
    fn rewrite_revert_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SMTCheckerKeyword` node.
    fn rewrite_smt_checker_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SealedKeyword` node.
    fn rewrite_sealed_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SecondsKeyword` node.
    fn rewrite_seconds_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Semicolon` node.
    fn rewrite_semicolon(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SingleLineComment` node.
    fn rewrite_single_line_comment(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SingleLineNatSpecComment` node.
    fn rewrite_single_line_nat_spec_comment(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SingleQuotedHexStringLiteral` node.
    fn rewrite_single_quoted_hex_string_literal(
        &mut self,
        node: &Rc<TerminalNode>,
    ) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SingleQuotedStringLiteral` node.
    fn rewrite_single_quoted_string_literal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SingleQuotedUnicodeStringLiteral` node.
    fn rewrite_single_quoted_unicode_string_literal(
        &mut self,
        node: &Rc<TerminalNode>,
    ) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SingleQuotedVersionLiteral` node.
    fn rewrite_single_quoted_version_literal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SizeOfKeyword` node.
    fn rewrite_size_of_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Slash` node.
    fn rewrite_slash(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SlashEqual` node.
    fn rewrite_slash_equal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SolidityKeyword` node.
    fn rewrite_solidity_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `StaticKeyword` node.
    fn rewrite_static_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `StorageKeyword` node.
    fn rewrite_storage_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `StringKeyword` node.
    fn rewrite_string_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `StructKeyword` node.
    fn rewrite_struct_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SuperKeyword` node.
    fn rewrite_super_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SupportsKeyword` node.
    fn rewrite_supports_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SwitchKeyword` node.
    fn rewrite_switch_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `SzaboKeyword` node.
    fn rewrite_szabo_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ThisKeyword` node.
    fn rewrite_this_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ThrowKeyword` node.
    fn rewrite_throw_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Tilde` node.
    fn rewrite_tilde(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `TransientKeyword` node.
    fn rewrite_transient_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `TrueKeyword` node.
    fn rewrite_true_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `TryKeyword` node.
    fn rewrite_try_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `TypeDefKeyword` node.
    fn rewrite_type_def_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `TypeKeyword` node.
    fn rewrite_type_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `TypeOfKeyword` node.
    fn rewrite_type_of_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `UfixedKeyword` node.
    fn rewrite_ufixed_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `UintKeyword` node.
    fn rewrite_uint_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `UncheckedKeyword` node.
    fn rewrite_unchecked_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `UsingKeyword` node.
    fn rewrite_using_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `VarKeyword` node.
    fn rewrite_var_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `VersionSpecifier` node.
    fn rewrite_version_specifier(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `ViewKeyword` node.
    fn rewrite_view_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `VirtualKeyword` node.
    fn rewrite_virtual_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `WeeksKeyword` node.
    fn rewrite_weeks_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `WeiKeyword` node.
    fn rewrite_wei_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `WhileKeyword` node.
    fn rewrite_while_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Whitespace` node.
    fn rewrite_whitespace(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YearsKeyword` node.
    fn rewrite_years_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulAbstractKeyword` node.
    fn rewrite_yul_abstract_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulAfterKeyword` node.
    fn rewrite_yul_after_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulAliasKeyword` node.
    fn rewrite_yul_alias_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulAnonymousKeyword` node.
    fn rewrite_yul_anonymous_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulApplyKeyword` node.
    fn rewrite_yul_apply_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulAsKeyword` node.
    fn rewrite_yul_as_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulAssemblyKeyword` node.
    fn rewrite_yul_assembly_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulAutoKeyword` node.
    fn rewrite_yul_auto_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulBoolKeyword` node.
    fn rewrite_yul_bool_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulBreakKeyword` node.
    fn rewrite_yul_break_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulBytesKeyword` node.
    fn rewrite_yul_bytes_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulCallDataKeyword` node.
    fn rewrite_yul_call_data_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulCaseKeyword` node.
    fn rewrite_yul_case_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulCatchKeyword` node.
    fn rewrite_yul_catch_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulConstantKeyword` node.
    fn rewrite_yul_constant_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulConstructorKeyword` node.
    fn rewrite_yul_constructor_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulContinueKeyword` node.
    fn rewrite_yul_continue_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulContractKeyword` node.
    fn rewrite_yul_contract_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulCopyOfKeyword` node.
    fn rewrite_yul_copy_of_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulDaysKeyword` node.
    fn rewrite_yul_days_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulDecimalLiteral` node.
    fn rewrite_yul_decimal_literal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulDefaultKeyword` node.
    fn rewrite_yul_default_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulDefineKeyword` node.
    fn rewrite_yul_define_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulDeleteKeyword` node.
    fn rewrite_yul_delete_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulDoKeyword` node.
    fn rewrite_yul_do_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulElseKeyword` node.
    fn rewrite_yul_else_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulEmitKeyword` node.
    fn rewrite_yul_emit_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulEnumKeyword` node.
    fn rewrite_yul_enum_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulEtherKeyword` node.
    fn rewrite_yul_ether_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulEventKeyword` node.
    fn rewrite_yul_event_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulExternalKeyword` node.
    fn rewrite_yul_external_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulFallbackKeyword` node.
    fn rewrite_yul_fallback_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulFalseKeyword` node.
    fn rewrite_yul_false_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulFinalKeyword` node.
    fn rewrite_yul_final_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulFinneyKeyword` node.
    fn rewrite_yul_finney_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulFixedKeyword` node.
    fn rewrite_yul_fixed_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulForKeyword` node.
    fn rewrite_yul_for_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulFunctionKeyword` node.
    fn rewrite_yul_function_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulGweiKeyword` node.
    fn rewrite_yul_gwei_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulHexKeyword` node.
    fn rewrite_yul_hex_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulHexLiteral` node.
    fn rewrite_yul_hex_literal(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulHoursKeyword` node.
    fn rewrite_yul_hours_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulIdentifier` node.
    fn rewrite_yul_identifier(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulIfKeyword` node.
    fn rewrite_yul_if_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulImmutableKeyword` node.
    fn rewrite_yul_immutable_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulImplementsKeyword` node.
    fn rewrite_yul_implements_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulImportKeyword` node.
    fn rewrite_yul_import_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulInKeyword` node.
    fn rewrite_yul_in_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulIndexedKeyword` node.
    fn rewrite_yul_indexed_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulInlineKeyword` node.
    fn rewrite_yul_inline_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulIntKeyword` node.
    fn rewrite_yul_int_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulInterfaceKeyword` node.
    fn rewrite_yul_interface_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulInternalKeyword` node.
    fn rewrite_yul_internal_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulIsKeyword` node.
    fn rewrite_yul_is_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulLeaveKeyword` node.
    fn rewrite_yul_leave_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulLetKeyword` node.
    fn rewrite_yul_let_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulLibraryKeyword` node.
    fn rewrite_yul_library_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulMacroKeyword` node.
    fn rewrite_yul_macro_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulMappingKeyword` node.
    fn rewrite_yul_mapping_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulMatchKeyword` node.
    fn rewrite_yul_match_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulMemoryKeyword` node.
    fn rewrite_yul_memory_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulMinutesKeyword` node.
    fn rewrite_yul_minutes_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulModifierKeyword` node.
    fn rewrite_yul_modifier_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulMutableKeyword` node.
    fn rewrite_yul_mutable_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulNewKeyword` node.
    fn rewrite_yul_new_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulNullKeyword` node.
    fn rewrite_yul_null_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulOfKeyword` node.
    fn rewrite_yul_of_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulOverrideKeyword` node.
    fn rewrite_yul_override_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulPartialKeyword` node.
    fn rewrite_yul_partial_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulPayableKeyword` node.
    fn rewrite_yul_payable_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulPragmaKeyword` node.
    fn rewrite_yul_pragma_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulPrivateKeyword` node.
    fn rewrite_yul_private_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulPromiseKeyword` node.
    fn rewrite_yul_promise_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulPublicKeyword` node.
    fn rewrite_yul_public_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulPureKeyword` node.
    fn rewrite_yul_pure_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulReceiveKeyword` node.
    fn rewrite_yul_receive_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulReferenceKeyword` node.
    fn rewrite_yul_reference_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulRelocatableKeyword` node.
    fn rewrite_yul_relocatable_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulReturnsKeyword` node.
    fn rewrite_yul_returns_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulSealedKeyword` node.
    fn rewrite_yul_sealed_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulSecondsKeyword` node.
    fn rewrite_yul_seconds_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulSizeOfKeyword` node.
    fn rewrite_yul_size_of_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulStaticKeyword` node.
    fn rewrite_yul_static_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulStorageKeyword` node.
    fn rewrite_yul_storage_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulStringKeyword` node.
    fn rewrite_yul_string_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulStructKeyword` node.
    fn rewrite_yul_struct_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulSuperKeyword` node.
    fn rewrite_yul_super_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulSupportsKeyword` node.
    fn rewrite_yul_supports_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulSwitchKeyword` node.
    fn rewrite_yul_switch_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulSzaboKeyword` node.
    fn rewrite_yul_szabo_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulThisKeyword` node.
    fn rewrite_yul_this_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulThrowKeyword` node.
    fn rewrite_yul_throw_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulTrueKeyword` node.
    fn rewrite_yul_true_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulTryKeyword` node.
    fn rewrite_yul_try_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulTypeDefKeyword` node.
    fn rewrite_yul_type_def_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulTypeKeyword` node.
    fn rewrite_yul_type_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulTypeOfKeyword` node.
    fn rewrite_yul_type_of_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulUfixedKeyword` node.
    fn rewrite_yul_ufixed_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulUintKeyword` node.
    fn rewrite_yul_uint_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulUncheckedKeyword` node.
    fn rewrite_yul_unchecked_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulUsingKeyword` node.
    fn rewrite_yul_using_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulVarKeyword` node.
    fn rewrite_yul_var_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulViewKeyword` node.
    fn rewrite_yul_view_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulVirtualKeyword` node.
    fn rewrite_yul_virtual_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulWeeksKeyword` node.
    fn rewrite_yul_weeks_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulWeiKeyword` node.
    fn rewrite_yul_wei_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulWhileKeyword` node.
    fn rewrite_yul_while_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `YulYearsKeyword` node.
    fn rewrite_yul_years_keyword(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites an `Unrecognized` node.
    fn rewrite_unrecognized(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites a `Missing` node.
    fn rewrite_missing(&mut self, node: &Rc<TerminalNode>) -> Option<Node> {
        Some(Node::Terminal(Rc::clone(node)))
    }

    /// Rewrites all the children of a given non-terminal node.
    fn rewrite_children(&mut self, node: &Rc<NonterminalNode>) -> Node {
        let mut new_children: Option<Vec<Edge>> = None;

        for (index, child) in node.children.iter().enumerate() {
            if let Some(new_child_node) = self.rewrite_node(&child.node) {
                if new_child_node.id() == child.node.id() {
                    if let Some(ref mut new_children) = new_children {
                        new_children.push(Edge {
                            label: child.label,
                            node: new_child_node,
                        });
                    }
                } else {
                    // node has changed, produce new edge
                    let edge = Edge {
                        label: child.label,
                        node: new_child_node,
                    };
                    if new_children.is_none() {
                        new_children = Some(node.children[..index].to_vec());
                    }
                    new_children.as_mut().unwrap().push(edge);
                }
            } else {
                // node was removed. if `new_children` is set, just skip this one
                // otherwise, copy the first ones from `children` (but not the last)
                if new_children.is_none() {
                    new_children = Some(node.children[..index].to_vec());
                }
            }
        }

        if let Some(nc) = new_children {
            Node::nonterminal(node.kind, nc)
        } else {
            Node::Nonterminal(Rc::clone(node))
        }
    }
}
