// Generated on 2025-02-13T18:37:20.966Z
use super::model::*;

pub trait Handler {
    #[allow(unused_variables)]
    fn handle_terminal_kind(self: &mut Self, node: &TerminalKind) {}

    #[allow(unused_variables)]
    fn handle_terminal_node(self: &mut Self, node: &Box<TerminalNode>) {}

    #[allow(unused_variables)]
    fn handle_source_unit(self: &mut Self, node: &Box<SourceUnit>) {}

    #[allow(unused_variables)]
    fn handle_pragma_directive(self: &mut Self, node: &Box<PragmaDirective>) {}

    #[allow(unused_variables)]
    fn handle_abicoder_pragma(self: &mut Self, node: &Box<AbicoderPragma>) {}

    #[allow(unused_variables)]
    fn handle_experimental_pragma(self: &mut Self, node: &Box<ExperimentalPragma>) {}

    #[allow(unused_variables)]
    fn handle_version_pragma(self: &mut Self, node: &Box<VersionPragma>) {}

    #[allow(unused_variables)]
    fn handle_version_range(self: &mut Self, node: &Box<VersionRange>) {}

    #[allow(unused_variables)]
    fn handle_version_term(self: &mut Self, node: &Box<VersionTerm>) {}

    #[allow(unused_variables)]
    fn handle_import_directive(self: &mut Self, node: &Box<ImportDirective>) {}

    #[allow(unused_variables)]
    fn handle_path_import(self: &mut Self, node: &Box<PathImport>) {}

    #[allow(unused_variables)]
    fn handle_named_import(self: &mut Self, node: &Box<NamedImport>) {}

    #[allow(unused_variables)]
    fn handle_import_deconstruction(self: &mut Self, node: &Box<ImportDeconstruction>) {}

    #[allow(unused_variables)]
    fn handle_import_deconstruction_symbol(
        self: &mut Self,
        node: &Box<ImportDeconstructionSymbol>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_import_alias(self: &mut Self, node: &Box<ImportAlias>) {}

    #[allow(unused_variables)]
    fn handle_using_directive(self: &mut Self, node: &Box<UsingDirective>) {}

    #[allow(unused_variables)]
    fn handle_using_deconstruction(self: &mut Self, node: &Box<UsingDeconstruction>) {}

    #[allow(unused_variables)]
    fn handle_using_deconstruction_symbol(self: &mut Self, node: &Box<UsingDeconstructionSymbol>) {}

    #[allow(unused_variables)]
    fn handle_using_alias(self: &mut Self, node: &Box<UsingAlias>) {}

    #[allow(unused_variables)]
    fn handle_contract_definition(self: &mut Self, node: &Box<ContractDefinition>) {}

    #[allow(unused_variables)]
    fn handle_inheritance_specifier(self: &mut Self, node: &Box<InheritanceSpecifier>) {}

    #[allow(unused_variables)]
    fn handle_inheritance_type(self: &mut Self, node: &Box<InheritanceType>) {}

    #[allow(unused_variables)]
    fn handle_interface_definition(self: &mut Self, node: &Box<InterfaceDefinition>) {}

    #[allow(unused_variables)]
    fn handle_library_definition(self: &mut Self, node: &Box<LibraryDefinition>) {}

    #[allow(unused_variables)]
    fn handle_struct_definition(self: &mut Self, node: &Box<StructDefinition>) {}

    #[allow(unused_variables)]
    fn handle_struct_member(self: &mut Self, node: &Box<StructMember>) {}

    #[allow(unused_variables)]
    fn handle_enum_definition(self: &mut Self, node: &Box<EnumDefinition>) {}

    #[allow(unused_variables)]
    fn handle_constant_definition(self: &mut Self, node: &Box<ConstantDefinition>) {}

    #[allow(unused_variables)]
    fn handle_state_variable_definition(self: &mut Self, node: &Box<StateVariableDefinition>) {}

    #[allow(unused_variables)]
    fn handle_state_variable_definition_value(
        self: &mut Self,
        node: &Box<StateVariableDefinitionValue>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_function_definition(self: &mut Self, node: &Box<FunctionDefinition>) {}

    #[allow(unused_variables)]
    fn handle_parameters_declaration(self: &mut Self, node: &Box<ParametersDeclaration>) {}

    #[allow(unused_variables)]
    fn handle_parameter(self: &mut Self, node: &Box<Parameter>) {}

    #[allow(unused_variables)]
    fn handle_override_specifier(self: &mut Self, node: &Box<OverrideSpecifier>) {}

    #[allow(unused_variables)]
    fn handle_override_paths_declaration(self: &mut Self, node: &Box<OverridePathsDeclaration>) {}

    #[allow(unused_variables)]
    fn handle_returns_declaration(self: &mut Self, node: &Box<ReturnsDeclaration>) {}

    #[allow(unused_variables)]
    fn handle_constructor_definition(self: &mut Self, node: &Box<ConstructorDefinition>) {}

    #[allow(unused_variables)]
    fn handle_unnamed_function_definition(self: &mut Self, node: &Box<UnnamedFunctionDefinition>) {}

    #[allow(unused_variables)]
    fn handle_fallback_function_definition(
        self: &mut Self,
        node: &Box<FallbackFunctionDefinition>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_receive_function_definition(self: &mut Self, node: &Box<ReceiveFunctionDefinition>) {}

    #[allow(unused_variables)]
    fn handle_modifier_definition(self: &mut Self, node: &Box<ModifierDefinition>) {}

    #[allow(unused_variables)]
    fn handle_modifier_invocation(self: &mut Self, node: &Box<ModifierInvocation>) {}

    #[allow(unused_variables)]
    fn handle_event_definition(self: &mut Self, node: &Box<EventDefinition>) {}

    #[allow(unused_variables)]
    fn handle_event_parameters_declaration(
        self: &mut Self,
        node: &Box<EventParametersDeclaration>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_event_parameter(self: &mut Self, node: &Box<EventParameter>) {}

    #[allow(unused_variables)]
    fn handle_user_defined_value_type_definition(
        self: &mut Self,
        node: &Box<UserDefinedValueTypeDefinition>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_error_definition(self: &mut Self, node: &Box<ErrorDefinition>) {}

    #[allow(unused_variables)]
    fn handle_error_parameters_declaration(
        self: &mut Self,
        node: &Box<ErrorParametersDeclaration>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_error_parameter(self: &mut Self, node: &Box<ErrorParameter>) {}

    #[allow(unused_variables)]
    fn handle_array_type_name(self: &mut Self, node: &Box<ArrayTypeName>) {}

    #[allow(unused_variables)]
    fn handle_function_type(self: &mut Self, node: &Box<FunctionType>) {}

    #[allow(unused_variables)]
    fn handle_mapping_type(self: &mut Self, node: &Box<MappingType>) {}

    #[allow(unused_variables)]
    fn handle_mapping_key(self: &mut Self, node: &Box<MappingKey>) {}

    #[allow(unused_variables)]
    fn handle_mapping_value(self: &mut Self, node: &Box<MappingValue>) {}

    #[allow(unused_variables)]
    fn handle_address_type(self: &mut Self, node: &Box<AddressType>) {}

    #[allow(unused_variables)]
    fn handle_block(self: &mut Self, node: &Box<Block>) {}

    #[allow(unused_variables)]
    fn handle_unchecked_block(self: &mut Self, node: &Box<UncheckedBlock>) {}

    #[allow(unused_variables)]
    fn handle_expression_statement(self: &mut Self, node: &Box<ExpressionStatement>) {}

    #[allow(unused_variables)]
    fn handle_assembly_statement(self: &mut Self, node: &Box<AssemblyStatement>) {}

    #[allow(unused_variables)]
    fn handle_assembly_flags_declaration(self: &mut Self, node: &Box<AssemblyFlagsDeclaration>) {}

    #[allow(unused_variables)]
    fn handle_tuple_deconstruction_statement(
        self: &mut Self,
        node: &Box<TupleDeconstructionStatement>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_tuple_deconstruction_element(
        self: &mut Self,
        node: &Box<TupleDeconstructionElement>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_typed_tuple_member(self: &mut Self, node: &Box<TypedTupleMember>) {}

    #[allow(unused_variables)]
    fn handle_untyped_tuple_member(self: &mut Self, node: &Box<UntypedTupleMember>) {}

    #[allow(unused_variables)]
    fn handle_variable_declaration_statement(
        self: &mut Self,
        node: &Box<VariableDeclarationStatement>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_variable_declaration_value(self: &mut Self, node: &Box<VariableDeclarationValue>) {}

    #[allow(unused_variables)]
    fn handle_if_statement(self: &mut Self, node: &Box<IfStatement>) {}

    #[allow(unused_variables)]
    fn handle_else_branch(self: &mut Self, node: &Box<ElseBranch>) {}

    #[allow(unused_variables)]
    fn handle_for_statement(self: &mut Self, node: &Box<ForStatement>) {}

    #[allow(unused_variables)]
    fn handle_while_statement(self: &mut Self, node: &Box<WhileStatement>) {}

    #[allow(unused_variables)]
    fn handle_do_while_statement(self: &mut Self, node: &Box<DoWhileStatement>) {}

    #[allow(unused_variables)]
    fn handle_continue_statement(self: &mut Self, node: &Box<ContinueStatement>) {}

    #[allow(unused_variables)]
    fn handle_break_statement(self: &mut Self, node: &Box<BreakStatement>) {}

    #[allow(unused_variables)]
    fn handle_return_statement(self: &mut Self, node: &Box<ReturnStatement>) {}

    #[allow(unused_variables)]
    fn handle_emit_statement(self: &mut Self, node: &Box<EmitStatement>) {}

    #[allow(unused_variables)]
    fn handle_try_statement(self: &mut Self, node: &Box<TryStatement>) {}

    #[allow(unused_variables)]
    fn handle_catch_clause(self: &mut Self, node: &Box<CatchClause>) {}

    #[allow(unused_variables)]
    fn handle_catch_clause_error(self: &mut Self, node: &Box<CatchClauseError>) {}

    #[allow(unused_variables)]
    fn handle_revert_statement(self: &mut Self, node: &Box<RevertStatement>) {}

    #[allow(unused_variables)]
    fn handle_throw_statement(self: &mut Self, node: &Box<ThrowStatement>) {}

    #[allow(unused_variables)]
    fn handle_assignment_expression(self: &mut Self, node: &Box<AssignmentExpression>) {}

    #[allow(unused_variables)]
    fn handle_conditional_expression(self: &mut Self, node: &Box<ConditionalExpression>) {}

    #[allow(unused_variables)]
    fn handle_or_expression(self: &mut Self, node: &Box<OrExpression>) {}

    #[allow(unused_variables)]
    fn handle_and_expression(self: &mut Self, node: &Box<AndExpression>) {}

    #[allow(unused_variables)]
    fn handle_equality_expression(self: &mut Self, node: &Box<EqualityExpression>) {}

    #[allow(unused_variables)]
    fn handle_comparison_expression(self: &mut Self, node: &Box<ComparisonExpression>) {}

    #[allow(unused_variables)]
    fn handle_bitwise_or_expression(self: &mut Self, node: &Box<BitwiseOrExpression>) {}

    #[allow(unused_variables)]
    fn handle_bitwise_xor_expression(self: &mut Self, node: &Box<BitwiseXorExpression>) {}

    #[allow(unused_variables)]
    fn handle_bitwise_and_expression(self: &mut Self, node: &Box<BitwiseAndExpression>) {}

    #[allow(unused_variables)]
    fn handle_shift_expression(self: &mut Self, node: &Box<ShiftExpression>) {}

    #[allow(unused_variables)]
    fn handle_additive_expression(self: &mut Self, node: &Box<AdditiveExpression>) {}

    #[allow(unused_variables)]
    fn handle_multiplicative_expression(self: &mut Self, node: &Box<MultiplicativeExpression>) {}

    #[allow(unused_variables)]
    fn handle_exponentiation_expression(self: &mut Self, node: &Box<ExponentiationExpression>) {}

    #[allow(unused_variables)]
    fn handle_postfix_expression(self: &mut Self, node: &Box<PostfixExpression>) {}

    #[allow(unused_variables)]
    fn handle_prefix_expression(self: &mut Self, node: &Box<PrefixExpression>) {}

    #[allow(unused_variables)]
    fn handle_function_call_expression(self: &mut Self, node: &Box<FunctionCallExpression>) {}

    #[allow(unused_variables)]
    fn handle_call_options_expression(self: &mut Self, node: &Box<CallOptionsExpression>) {}

    #[allow(unused_variables)]
    fn handle_member_access_expression(self: &mut Self, node: &Box<MemberAccessExpression>) {}

    #[allow(unused_variables)]
    fn handle_index_access_expression(self: &mut Self, node: &Box<IndexAccessExpression>) {}

    #[allow(unused_variables)]
    fn handle_index_access_end(self: &mut Self, node: &Box<IndexAccessEnd>) {}

    #[allow(unused_variables)]
    fn handle_positional_arguments_declaration(
        self: &mut Self,
        node: &Box<PositionalArgumentsDeclaration>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_named_arguments_declaration(self: &mut Self, node: &Box<NamedArgumentsDeclaration>) {}

    #[allow(unused_variables)]
    fn handle_named_argument_group(self: &mut Self, node: &Box<NamedArgumentGroup>) {}

    #[allow(unused_variables)]
    fn handle_named_argument(self: &mut Self, node: &Box<NamedArgument>) {}

    #[allow(unused_variables)]
    fn handle_type_expression(self: &mut Self, node: &Box<TypeExpression>) {}

    #[allow(unused_variables)]
    fn handle_new_expression(self: &mut Self, node: &Box<NewExpression>) {}

    #[allow(unused_variables)]
    fn handle_tuple_expression(self: &mut Self, node: &Box<TupleExpression>) {}

    #[allow(unused_variables)]
    fn handle_tuple_value(self: &mut Self, node: &Box<TupleValue>) {}

    #[allow(unused_variables)]
    fn handle_array_expression(self: &mut Self, node: &Box<ArrayExpression>) {}

    #[allow(unused_variables)]
    fn handle_hex_number_expression(self: &mut Self, node: &Box<HexNumberExpression>) {}

    #[allow(unused_variables)]
    fn handle_decimal_number_expression(self: &mut Self, node: &Box<DecimalNumberExpression>) {}

    #[allow(unused_variables)]
    fn handle_yul_block(self: &mut Self, node: &Box<YulBlock>) {}

    #[allow(unused_variables)]
    fn handle_yul_function_definition(self: &mut Self, node: &Box<YulFunctionDefinition>) {}

    #[allow(unused_variables)]
    fn handle_yul_parameters_declaration(self: &mut Self, node: &Box<YulParametersDeclaration>) {}

    #[allow(unused_variables)]
    fn handle_yul_returns_declaration(self: &mut Self, node: &Box<YulReturnsDeclaration>) {}

    #[allow(unused_variables)]
    fn handle_yul_variable_declaration_statement(
        self: &mut Self,
        node: &Box<YulVariableDeclarationStatement>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_yul_variable_declaration_value(
        self: &mut Self,
        node: &Box<YulVariableDeclarationValue>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_yul_variable_assignment_statement(
        self: &mut Self,
        node: &Box<YulVariableAssignmentStatement>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_yul_colon_and_equal(self: &mut Self, node: &Box<YulColonAndEqual>) {}

    #[allow(unused_variables)]
    fn handle_yul_stack_assignment_statement(
        self: &mut Self,
        node: &Box<YulStackAssignmentStatement>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_yul_equal_and_colon(self: &mut Self, node: &Box<YulEqualAndColon>) {}

    #[allow(unused_variables)]
    fn handle_yul_if_statement(self: &mut Self, node: &Box<YulIfStatement>) {}

    #[allow(unused_variables)]
    fn handle_yul_for_statement(self: &mut Self, node: &Box<YulForStatement>) {}

    #[allow(unused_variables)]
    fn handle_yul_switch_statement(self: &mut Self, node: &Box<YulSwitchStatement>) {}

    #[allow(unused_variables)]
    fn handle_yul_default_case(self: &mut Self, node: &Box<YulDefaultCase>) {}

    #[allow(unused_variables)]
    fn handle_yul_value_case(self: &mut Self, node: &Box<YulValueCase>) {}

    #[allow(unused_variables)]
    fn handle_yul_leave_statement(self: &mut Self, node: &Box<YulLeaveStatement>) {}

    #[allow(unused_variables)]
    fn handle_yul_break_statement(self: &mut Self, node: &Box<YulBreakStatement>) {}

    #[allow(unused_variables)]
    fn handle_yul_continue_statement(self: &mut Self, node: &Box<YulContinueStatement>) {}

    #[allow(unused_variables)]
    fn handle_yul_label(self: &mut Self, node: &Box<YulLabel>) {}

    #[allow(unused_variables)]
    fn handle_yul_function_call_expression(self: &mut Self, node: &Box<YulFunctionCallExpression>) {
    }

    #[allow(unused_variables)]
    fn handle_source_unit_member(self: &mut Self, node: &SourceUnitMember) {
        match node {
            SourceUnitMember::PragmaDirective(value) => self.handle_pragma_directive(value),
            SourceUnitMember::ImportDirective(value) => self.handle_import_directive(value),
            SourceUnitMember::ContractDefinition(value) => self.handle_contract_definition(value),
            SourceUnitMember::InterfaceDefinition(value) => self.handle_interface_definition(value),
            SourceUnitMember::LibraryDefinition(value) => self.handle_library_definition(value),
            SourceUnitMember::StructDefinition(value) => self.handle_struct_definition(value),
            SourceUnitMember::EnumDefinition(value) => self.handle_enum_definition(value),
            SourceUnitMember::FunctionDefinition(value) => self.handle_function_definition(value),
            SourceUnitMember::ErrorDefinition(value) => self.handle_error_definition(value),
            SourceUnitMember::UserDefinedValueTypeDefinition(value) => {
                self.handle_user_defined_value_type_definition(value)
            }
            SourceUnitMember::UsingDirective(value) => self.handle_using_directive(value),
            SourceUnitMember::EventDefinition(value) => self.handle_event_definition(value),
            SourceUnitMember::ConstantDefinition(value) => self.handle_constant_definition(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_pragma(self: &mut Self, node: &Pragma) {
        match node {
            Pragma::AbicoderPragma(value) => self.handle_abicoder_pragma(value),
            Pragma::ExperimentalPragma(value) => self.handle_experimental_pragma(value),
            Pragma::VersionPragma(value) => self.handle_version_pragma(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_experimental_feature(self: &mut Self, node: &ExperimentalFeature) {
        match node {
            ExperimentalFeature::StringLiteral(value) => self.handle_string_literal(value),
            ExperimentalFeature::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_version_expression(self: &mut Self, node: &VersionExpression) {
        match node {
            VersionExpression::VersionRange(value) => self.handle_version_range(value),
            VersionExpression::VersionTerm(value) => self.handle_version_term(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_version_operator(self: &mut Self, node: &VersionOperator) {
        match node {
            VersionOperator::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_version_literal(self: &mut Self, node: &VersionLiteral) {
        match node {
            VersionLiteral::SimpleVersionLiteral(value) => {
                self.handle_simple_version_literal(value)
            }
            VersionLiteral::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_import_clause(self: &mut Self, node: &ImportClause) {
        match node {
            ImportClause::PathImport(value) => self.handle_path_import(value),
            ImportClause::NamedImport(value) => self.handle_named_import(value),
            ImportClause::ImportDeconstruction(value) => self.handle_import_deconstruction(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_using_clause(self: &mut Self, node: &UsingClause) {
        match node {
            UsingClause::IdentifierPath(value) => self.handle_identifier_path(value),
            UsingClause::UsingDeconstruction(value) => self.handle_using_deconstruction(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_using_operator(self: &mut Self, node: &UsingOperator) {
        match node {
            UsingOperator::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_using_target(self: &mut Self, node: &UsingTarget) {
        match node {
            UsingTarget::TypeName(value) => self.handle_type_name(value),
            UsingTarget::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_contract_member(self: &mut Self, node: &ContractMember) {
        match node {
            ContractMember::UsingDirective(value) => self.handle_using_directive(value),
            ContractMember::FunctionDefinition(value) => self.handle_function_definition(value),
            ContractMember::ConstructorDefinition(value) => {
                self.handle_constructor_definition(value)
            }
            ContractMember::ReceiveFunctionDefinition(value) => {
                self.handle_receive_function_definition(value)
            }
            ContractMember::FallbackFunctionDefinition(value) => {
                self.handle_fallback_function_definition(value)
            }
            ContractMember::UnnamedFunctionDefinition(value) => {
                self.handle_unnamed_function_definition(value)
            }
            ContractMember::ModifierDefinition(value) => self.handle_modifier_definition(value),
            ContractMember::StructDefinition(value) => self.handle_struct_definition(value),
            ContractMember::EnumDefinition(value) => self.handle_enum_definition(value),
            ContractMember::EventDefinition(value) => self.handle_event_definition(value),
            ContractMember::ErrorDefinition(value) => self.handle_error_definition(value),
            ContractMember::UserDefinedValueTypeDefinition(value) => {
                self.handle_user_defined_value_type_definition(value)
            }
            ContractMember::StateVariableDefinition(value) => {
                self.handle_state_variable_definition(value)
            }
        }
    }

    #[allow(unused_variables)]
    fn handle_state_variable_attribute(self: &mut Self, node: &StateVariableAttribute) {
        match node {
            StateVariableAttribute::OverrideSpecifier(value) => {
                self.handle_override_specifier(value)
            }
            StateVariableAttribute::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_function_name(self: &mut Self, node: &FunctionName) {
        match node {
            FunctionName::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_function_attribute(self: &mut Self, node: &FunctionAttribute) {
        match node {
            FunctionAttribute::ModifierInvocation(value) => self.handle_modifier_invocation(value),
            FunctionAttribute::OverrideSpecifier(value) => self.handle_override_specifier(value),
            FunctionAttribute::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_function_body(self: &mut Self, node: &FunctionBody) {
        match node {
            FunctionBody::Block(value) => self.handle_block(value),
            FunctionBody::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_constructor_attribute(self: &mut Self, node: &ConstructorAttribute) {
        match node {
            ConstructorAttribute::ModifierInvocation(value) => {
                self.handle_modifier_invocation(value)
            }
            ConstructorAttribute::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_unnamed_function_attribute(self: &mut Self, node: &UnnamedFunctionAttribute) {
        match node {
            UnnamedFunctionAttribute::ModifierInvocation(value) => {
                self.handle_modifier_invocation(value)
            }
            UnnamedFunctionAttribute::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_fallback_function_attribute(self: &mut Self, node: &FallbackFunctionAttribute) {
        match node {
            FallbackFunctionAttribute::ModifierInvocation(value) => {
                self.handle_modifier_invocation(value)
            }
            FallbackFunctionAttribute::OverrideSpecifier(value) => {
                self.handle_override_specifier(value)
            }
            FallbackFunctionAttribute::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_receive_function_attribute(self: &mut Self, node: &ReceiveFunctionAttribute) {
        match node {
            ReceiveFunctionAttribute::ModifierInvocation(value) => {
                self.handle_modifier_invocation(value)
            }
            ReceiveFunctionAttribute::OverrideSpecifier(value) => {
                self.handle_override_specifier(value)
            }
            ReceiveFunctionAttribute::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_modifier_attribute(self: &mut Self, node: &ModifierAttribute) {
        match node {
            ModifierAttribute::OverrideSpecifier(value) => self.handle_override_specifier(value),
            ModifierAttribute::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_type_name(self: &mut Self, node: &TypeName) {
        match node {
            TypeName::ArrayTypeName(value) => self.handle_array_type_name(value),
            TypeName::FunctionType(value) => self.handle_function_type(value),
            TypeName::MappingType(value) => self.handle_mapping_type(value),
            TypeName::ElementaryType(value) => self.handle_elementary_type(value),
            TypeName::IdentifierPath(value) => self.handle_identifier_path(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_function_type_attribute(self: &mut Self, node: &FunctionTypeAttribute) {
        match node {
            FunctionTypeAttribute::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_mapping_key_type(self: &mut Self, node: &MappingKeyType) {
        match node {
            MappingKeyType::ElementaryType(value) => self.handle_elementary_type(value),
            MappingKeyType::IdentifierPath(value) => self.handle_identifier_path(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_elementary_type(self: &mut Self, node: &ElementaryType) {
        match node {
            ElementaryType::AddressType(value) => self.handle_address_type(value),
            ElementaryType::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_statement(self: &mut Self, node: &Statement) {
        match node {
            Statement::IfStatement(value) => self.handle_if_statement(value),
            Statement::ForStatement(value) => self.handle_for_statement(value),
            Statement::WhileStatement(value) => self.handle_while_statement(value),
            Statement::DoWhileStatement(value) => self.handle_do_while_statement(value),
            Statement::ContinueStatement(value) => self.handle_continue_statement(value),
            Statement::BreakStatement(value) => self.handle_break_statement(value),
            Statement::ReturnStatement(value) => self.handle_return_statement(value),
            Statement::ThrowStatement(value) => self.handle_throw_statement(value),
            Statement::EmitStatement(value) => self.handle_emit_statement(value),
            Statement::TryStatement(value) => self.handle_try_statement(value),
            Statement::RevertStatement(value) => self.handle_revert_statement(value),
            Statement::AssemblyStatement(value) => self.handle_assembly_statement(value),
            Statement::Block(value) => self.handle_block(value),
            Statement::UncheckedBlock(value) => self.handle_unchecked_block(value),
            Statement::TupleDeconstructionStatement(value) => {
                self.handle_tuple_deconstruction_statement(value)
            }
            Statement::VariableDeclarationStatement(value) => {
                self.handle_variable_declaration_statement(value)
            }
            Statement::ExpressionStatement(value) => self.handle_expression_statement(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_tuple_member(self: &mut Self, node: &TupleMember) {
        match node {
            TupleMember::TypedTupleMember(value) => self.handle_typed_tuple_member(value),
            TupleMember::UntypedTupleMember(value) => self.handle_untyped_tuple_member(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_variable_declaration_type(self: &mut Self, node: &VariableDeclarationType) {
        match node {
            VariableDeclarationType::TypeName(value) => self.handle_type_name(value),
            VariableDeclarationType::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_storage_location(self: &mut Self, node: &StorageLocation) {
        match node {
            StorageLocation::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_for_statement_initialization(self: &mut Self, node: &ForStatementInitialization) {
        match node {
            ForStatementInitialization::TupleDeconstructionStatement(value) => {
                self.handle_tuple_deconstruction_statement(value)
            }
            ForStatementInitialization::VariableDeclarationStatement(value) => {
                self.handle_variable_declaration_statement(value)
            }
            ForStatementInitialization::ExpressionStatement(value) => {
                self.handle_expression_statement(value)
            }
            ForStatementInitialization::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_for_statement_condition(self: &mut Self, node: &ForStatementCondition) {
        match node {
            ForStatementCondition::ExpressionStatement(value) => {
                self.handle_expression_statement(value)
            }
            ForStatementCondition::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_expression(self: &mut Self, node: &Expression) {
        match node {
            Expression::AssignmentExpression(value) => self.handle_assignment_expression(value),
            Expression::ConditionalExpression(value) => self.handle_conditional_expression(value),
            Expression::OrExpression(value) => self.handle_or_expression(value),
            Expression::AndExpression(value) => self.handle_and_expression(value),
            Expression::EqualityExpression(value) => self.handle_equality_expression(value),
            Expression::ComparisonExpression(value) => self.handle_comparison_expression(value),
            Expression::BitwiseOrExpression(value) => self.handle_bitwise_or_expression(value),
            Expression::BitwiseXorExpression(value) => self.handle_bitwise_xor_expression(value),
            Expression::BitwiseAndExpression(value) => self.handle_bitwise_and_expression(value),
            Expression::ShiftExpression(value) => self.handle_shift_expression(value),
            Expression::AdditiveExpression(value) => self.handle_additive_expression(value),
            Expression::MultiplicativeExpression(value) => {
                self.handle_multiplicative_expression(value)
            }
            Expression::ExponentiationExpression(value) => {
                self.handle_exponentiation_expression(value)
            }
            Expression::PostfixExpression(value) => self.handle_postfix_expression(value),
            Expression::PrefixExpression(value) => self.handle_prefix_expression(value),
            Expression::FunctionCallExpression(value) => {
                self.handle_function_call_expression(value)
            }
            Expression::CallOptionsExpression(value) => self.handle_call_options_expression(value),
            Expression::MemberAccessExpression(value) => {
                self.handle_member_access_expression(value)
            }
            Expression::IndexAccessExpression(value) => self.handle_index_access_expression(value),
            Expression::NewExpression(value) => self.handle_new_expression(value),
            Expression::TupleExpression(value) => self.handle_tuple_expression(value),
            Expression::TypeExpression(value) => self.handle_type_expression(value),
            Expression::ArrayExpression(value) => self.handle_array_expression(value),
            Expression::HexNumberExpression(value) => self.handle_hex_number_expression(value),
            Expression::DecimalNumberExpression(value) => {
                self.handle_decimal_number_expression(value)
            }
            Expression::StringExpression(value) => self.handle_string_expression(value),
            Expression::ElementaryType(value) => self.handle_elementary_type(value),
            Expression::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_arguments_declaration(self: &mut Self, node: &ArgumentsDeclaration) {
        match node {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(value) => {
                self.handle_positional_arguments_declaration(value)
            }
            ArgumentsDeclaration::NamedArgumentsDeclaration(value) => {
                self.handle_named_arguments_declaration(value)
            }
        }
    }

    #[allow(unused_variables)]
    fn handle_number_unit(self: &mut Self, node: &NumberUnit) {
        match node {
            NumberUnit::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_string_expression(self: &mut Self, node: &StringExpression) {
        match node {
            StringExpression::StringLiteral(value) => self.handle_string_literal(value),
            StringExpression::StringLiterals(value) => self.handle_string_literals(value),
            StringExpression::HexStringLiteral(value) => self.handle_hex_string_literal(value),
            StringExpression::HexStringLiterals(value) => self.handle_hex_string_literals(value),
            StringExpression::UnicodeStringLiterals(value) => {
                self.handle_unicode_string_literals(value)
            }
        }
    }

    #[allow(unused_variables)]
    fn handle_string_literal(self: &mut Self, node: &StringLiteral) {
        match node {
            StringLiteral::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_hex_string_literal(self: &mut Self, node: &HexStringLiteral) {
        match node {
            HexStringLiteral::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_unicode_string_literal(self: &mut Self, node: &UnicodeStringLiteral) {
        match node {
            UnicodeStringLiteral::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_statement(self: &mut Self, node: &YulStatement) {
        match node {
            YulStatement::YulBlock(value) => self.handle_yul_block(value),
            YulStatement::YulFunctionDefinition(value) => {
                self.handle_yul_function_definition(value)
            }
            YulStatement::YulStackAssignmentStatement(value) => {
                self.handle_yul_stack_assignment_statement(value)
            }
            YulStatement::YulIfStatement(value) => self.handle_yul_if_statement(value),
            YulStatement::YulForStatement(value) => self.handle_yul_for_statement(value),
            YulStatement::YulSwitchStatement(value) => self.handle_yul_switch_statement(value),
            YulStatement::YulLeaveStatement(value) => self.handle_yul_leave_statement(value),
            YulStatement::YulBreakStatement(value) => self.handle_yul_break_statement(value),
            YulStatement::YulContinueStatement(value) => self.handle_yul_continue_statement(value),
            YulStatement::YulVariableAssignmentStatement(value) => {
                self.handle_yul_variable_assignment_statement(value)
            }
            YulStatement::YulLabel(value) => self.handle_yul_label(value),
            YulStatement::YulVariableDeclarationStatement(value) => {
                self.handle_yul_variable_declaration_statement(value)
            }
            YulStatement::YulExpression(value) => self.handle_yul_expression(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_assignment_operator(self: &mut Self, node: &YulAssignmentOperator) {
        match node {
            YulAssignmentOperator::YulColonAndEqual(value) => {
                self.handle_yul_colon_and_equal(value)
            }
            YulAssignmentOperator::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_stack_assignment_operator(self: &mut Self, node: &YulStackAssignmentOperator) {
        match node {
            YulStackAssignmentOperator::YulEqualAndColon(value) => {
                self.handle_yul_equal_and_colon(value)
            }
            YulStackAssignmentOperator::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_switch_case(self: &mut Self, node: &YulSwitchCase) {
        match node {
            YulSwitchCase::YulDefaultCase(value) => self.handle_yul_default_case(value),
            YulSwitchCase::YulValueCase(value) => self.handle_yul_value_case(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_expression(self: &mut Self, node: &YulExpression) {
        match node {
            YulExpression::YulFunctionCallExpression(value) => {
                self.handle_yul_function_call_expression(value)
            }
            YulExpression::YulLiteral(value) => self.handle_yul_literal(value),
            YulExpression::YulBuiltInFunction(value) => self.handle_yul_built_in_function(value),
            YulExpression::YulPath(value) => self.handle_yul_path(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_built_in_function(self: &mut Self, node: &YulBuiltInFunction) {
        match node {
            YulBuiltInFunction::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_literal(self: &mut Self, node: &YulLiteral) {
        match node {
            YulLiteral::HexStringLiteral(value) => self.handle_hex_string_literal(value),
            YulLiteral::StringLiteral(value) => self.handle_string_literal(value),
            YulLiteral::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_source_unit_members(self: &mut Self, node: &Box<SourceUnitMembers>) {}

    #[allow(unused_variables)]
    fn handle_version_expression_set(self: &mut Self, node: &Box<VersionExpressionSet>) {}

    #[allow(unused_variables)]
    fn handle_contract_members(self: &mut Self, node: &Box<ContractMembers>) {}

    #[allow(unused_variables)]
    fn handle_interface_members(self: &mut Self, node: &Box<InterfaceMembers>) {}

    #[allow(unused_variables)]
    fn handle_library_members(self: &mut Self, node: &Box<LibraryMembers>) {}

    #[allow(unused_variables)]
    fn handle_struct_members(self: &mut Self, node: &Box<StructMembers>) {}

    #[allow(unused_variables)]
    fn handle_state_variable_attributes(self: &mut Self, node: &Box<StateVariableAttributes>) {}

    #[allow(unused_variables)]
    fn handle_function_attributes(self: &mut Self, node: &Box<FunctionAttributes>) {}

    #[allow(unused_variables)]
    fn handle_constructor_attributes(self: &mut Self, node: &Box<ConstructorAttributes>) {}

    #[allow(unused_variables)]
    fn handle_unnamed_function_attributes(self: &mut Self, node: &Box<UnnamedFunctionAttributes>) {}

    #[allow(unused_variables)]
    fn handle_fallback_function_attributes(
        self: &mut Self,
        node: &Box<FallbackFunctionAttributes>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_receive_function_attributes(self: &mut Self, node: &Box<ReceiveFunctionAttributes>) {}

    #[allow(unused_variables)]
    fn handle_modifier_attributes(self: &mut Self, node: &Box<ModifierAttributes>) {}

    #[allow(unused_variables)]
    fn handle_function_type_attributes(self: &mut Self, node: &Box<FunctionTypeAttributes>) {}

    #[allow(unused_variables)]
    fn handle_statements(self: &mut Self, node: &Box<Statements>) {}

    #[allow(unused_variables)]
    fn handle_catch_clauses(self: &mut Self, node: &Box<CatchClauses>) {}

    #[allow(unused_variables)]
    fn handle_string_literals(self: &mut Self, node: &Box<StringLiterals>) {}

    #[allow(unused_variables)]
    fn handle_hex_string_literals(self: &mut Self, node: &Box<HexStringLiterals>) {}

    #[allow(unused_variables)]
    fn handle_unicode_string_literals(self: &mut Self, node: &Box<UnicodeStringLiterals>) {}

    #[allow(unused_variables)]
    fn handle_yul_statements(self: &mut Self, node: &Box<YulStatements>) {}

    #[allow(unused_variables)]
    fn handle_yul_switch_cases(self: &mut Self, node: &Box<YulSwitchCases>) {}

    #[allow(unused_variables)]
    fn handle_version_expression_sets(self: &mut Self, node: &Box<VersionExpressionSets>) {}

    #[allow(unused_variables)]
    fn handle_simple_version_literal(self: &mut Self, node: &Box<SimpleVersionLiteral>) {}

    #[allow(unused_variables)]
    fn handle_import_deconstruction_symbols(
        self: &mut Self,
        node: &Box<ImportDeconstructionSymbols>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_using_deconstruction_symbols(
        self: &mut Self,
        node: &Box<UsingDeconstructionSymbols>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_inheritance_types(self: &mut Self, node: &Box<InheritanceTypes>) {}

    #[allow(unused_variables)]
    fn handle_enum_members(self: &mut Self, node: &Box<EnumMembers>) {}

    #[allow(unused_variables)]
    fn handle_parameters(self: &mut Self, node: &Box<Parameters>) {}

    #[allow(unused_variables)]
    fn handle_override_paths(self: &mut Self, node: &Box<OverridePaths>) {}

    #[allow(unused_variables)]
    fn handle_event_parameters(self: &mut Self, node: &Box<EventParameters>) {}

    #[allow(unused_variables)]
    fn handle_error_parameters(self: &mut Self, node: &Box<ErrorParameters>) {}

    #[allow(unused_variables)]
    fn handle_assembly_flags(self: &mut Self, node: &Box<AssemblyFlags>) {}

    #[allow(unused_variables)]
    fn handle_tuple_deconstruction_elements(
        self: &mut Self,
        node: &Box<TupleDeconstructionElements>,
    ) {
    }

    #[allow(unused_variables)]
    fn handle_positional_arguments(self: &mut Self, node: &Box<PositionalArguments>) {}

    #[allow(unused_variables)]
    fn handle_named_arguments(self: &mut Self, node: &Box<NamedArguments>) {}

    #[allow(unused_variables)]
    fn handle_call_options(self: &mut Self, node: &Box<CallOptions>) {}

    #[allow(unused_variables)]
    fn handle_tuple_values(self: &mut Self, node: &Box<TupleValues>) {}

    #[allow(unused_variables)]
    fn handle_array_values(self: &mut Self, node: &Box<ArrayValues>) {}

    #[allow(unused_variables)]
    fn handle_identifier_path(self: &mut Self, node: &Box<IdentifierPath>) {}

    #[allow(unused_variables)]
    fn handle_yul_parameters(self: &mut Self, node: &Box<YulParameters>) {}

    #[allow(unused_variables)]
    fn handle_yul_variable_names(self: &mut Self, node: &Box<YulVariableNames>) {}

    #[allow(unused_variables)]
    fn handle_yul_arguments(self: &mut Self, node: &Box<YulArguments>) {}

    #[allow(unused_variables)]
    fn handle_yul_paths(self: &mut Self, node: &Box<YulPaths>) {}

    #[allow(unused_variables)]
    fn handle_yul_path(self: &mut Self, node: &Box<YulPath>) {}
}
