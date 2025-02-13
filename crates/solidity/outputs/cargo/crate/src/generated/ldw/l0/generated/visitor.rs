// Generated on 2025-02-13T18:37:20.977Z
use super::model::*;

pub trait Visitor {
    #[allow(unused_variables)]
    fn handle_terminal_kind(self: &mut Self, node: &TerminalKind) {}

    #[allow(unused_variables)]
    fn handle_terminal_node(self: &mut Self, node: &Box<TerminalNode>) {
        self.handle_terminal_kind(&node.kind);
    }

    #[allow(unused_variables)]
    fn handle_source_unit(self: &mut Self, node: &Box<SourceUnit>) {
        self.handle_source_unit_members(&node.members);
    }

    #[allow(unused_variables)]
    fn handle_pragma_directive(self: &mut Self, node: &Box<PragmaDirective>) {
        self.handle_terminal_node(&node.pragma_keyword);
        self.handle_pragma(&node.pragma);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_abicoder_pragma(self: &mut Self, node: &Box<AbicoderPragma>) {
        self.handle_terminal_node(&node.abicoder_keyword);
        self.handle_terminal_node(&node.version);
    }

    #[allow(unused_variables)]
    fn handle_experimental_pragma(self: &mut Self, node: &Box<ExperimentalPragma>) {
        self.handle_terminal_node(&node.experimental_keyword);
        self.handle_experimental_feature(&node.feature);
    }

    #[allow(unused_variables)]
    fn handle_version_pragma(self: &mut Self, node: &Box<VersionPragma>) {
        self.handle_terminal_node(&node.solidity_keyword);
        self.handle_version_expression_sets(&node.sets);
    }

    #[allow(unused_variables)]
    fn handle_version_range(self: &mut Self, node: &Box<VersionRange>) {
        self.handle_version_literal(&node.start);
        self.handle_terminal_node(&node.minus);
        self.handle_version_literal(&node.end);
    }

    #[allow(unused_variables)]
    fn handle_version_term(self: &mut Self, node: &Box<VersionTerm>) {
        if let Some(value) = &node.operator {
            self.handle_version_operator(&value);
        }
        self.handle_version_literal(&node.literal);
    }

    #[allow(unused_variables)]
    fn handle_import_directive(self: &mut Self, node: &Box<ImportDirective>) {
        self.handle_terminal_node(&node.import_keyword);
        self.handle_import_clause(&node.clause);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_path_import(self: &mut Self, node: &Box<PathImport>) {
        self.handle_string_literal(&node.path);
        if let Some(value) = &node.alias {
            self.handle_import_alias(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_named_import(self: &mut Self, node: &Box<NamedImport>) {
        self.handle_terminal_node(&node.asterisk);
        self.handle_import_alias(&node.alias);
        self.handle_terminal_node(&node.from_keyword);
        self.handle_string_literal(&node.path);
    }

    #[allow(unused_variables)]
    fn handle_import_deconstruction(self: &mut Self, node: &Box<ImportDeconstruction>) {
        self.handle_terminal_node(&node.open_brace);
        self.handle_import_deconstruction_symbols(&node.symbols);
        self.handle_terminal_node(&node.close_brace);
        self.handle_terminal_node(&node.from_keyword);
        self.handle_string_literal(&node.path);
    }

    #[allow(unused_variables)]
    fn handle_import_deconstruction_symbol(
        self: &mut Self,
        node: &Box<ImportDeconstructionSymbol>,
    ) {
        self.handle_terminal_node(&node.name);
        if let Some(value) = &node.alias {
            self.handle_import_alias(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_import_alias(self: &mut Self, node: &Box<ImportAlias>) {
        self.handle_terminal_node(&node.as_keyword);
        self.handle_terminal_node(&node.identifier);
    }

    #[allow(unused_variables)]
    fn handle_using_directive(self: &mut Self, node: &Box<UsingDirective>) {
        self.handle_terminal_node(&node.using_keyword);
        self.handle_using_clause(&node.clause);
        self.handle_terminal_node(&node.for_keyword);
        self.handle_using_target(&node.target);
        if let Some(value) = &node.global_keyword {
            self.handle_terminal_node(&value);
        }
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_using_deconstruction(self: &mut Self, node: &Box<UsingDeconstruction>) {
        self.handle_terminal_node(&node.open_brace);
        self.handle_using_deconstruction_symbols(&node.symbols);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_using_deconstruction_symbol(self: &mut Self, node: &Box<UsingDeconstructionSymbol>) {
        self.handle_identifier_path(&node.name);
        if let Some(value) = &node.alias {
            self.handle_using_alias(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_using_alias(self: &mut Self, node: &Box<UsingAlias>) {
        self.handle_terminal_node(&node.as_keyword);
        self.handle_using_operator(&node.operator);
    }

    #[allow(unused_variables)]
    fn handle_contract_definition(self: &mut Self, node: &Box<ContractDefinition>) {
        if let Some(value) = &node.abstract_keyword {
            self.handle_terminal_node(&value);
        }
        self.handle_terminal_node(&node.contract_keyword);
        self.handle_terminal_node(&node.name);
        if let Some(value) = &node.inheritance {
            self.handle_inheritance_specifier(&value);
        }
        self.handle_terminal_node(&node.open_brace);
        self.handle_contract_members(&node.members);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_inheritance_specifier(self: &mut Self, node: &Box<InheritanceSpecifier>) {
        self.handle_terminal_node(&node.is_keyword);
        self.handle_inheritance_types(&node.types);
    }

    #[allow(unused_variables)]
    fn handle_inheritance_type(self: &mut Self, node: &Box<InheritanceType>) {
        self.handle_identifier_path(&node.type_name);
        if let Some(value) = &node.arguments {
            self.handle_arguments_declaration(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_interface_definition(self: &mut Self, node: &Box<InterfaceDefinition>) {
        self.handle_terminal_node(&node.interface_keyword);
        self.handle_terminal_node(&node.name);
        if let Some(value) = &node.inheritance {
            self.handle_inheritance_specifier(&value);
        }
        self.handle_terminal_node(&node.open_brace);
        self.handle_interface_members(&node.members);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_library_definition(self: &mut Self, node: &Box<LibraryDefinition>) {
        self.handle_terminal_node(&node.library_keyword);
        self.handle_terminal_node(&node.name);
        self.handle_terminal_node(&node.open_brace);
        self.handle_library_members(&node.members);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_struct_definition(self: &mut Self, node: &Box<StructDefinition>) {
        self.handle_terminal_node(&node.struct_keyword);
        self.handle_terminal_node(&node.name);
        self.handle_terminal_node(&node.open_brace);
        self.handle_struct_members(&node.members);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_struct_member(self: &mut Self, node: &Box<StructMember>) {
        self.handle_type_name(&node.type_name);
        self.handle_terminal_node(&node.name);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_enum_definition(self: &mut Self, node: &Box<EnumDefinition>) {
        self.handle_terminal_node(&node.enum_keyword);
        self.handle_terminal_node(&node.name);
        self.handle_terminal_node(&node.open_brace);
        self.handle_enum_members(&node.members);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_constant_definition(self: &mut Self, node: &Box<ConstantDefinition>) {
        self.handle_type_name(&node.type_name);
        self.handle_terminal_node(&node.constant_keyword);
        self.handle_terminal_node(&node.name);
        self.handle_terminal_node(&node.equal);
        self.handle_expression(&node.value);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_state_variable_definition(self: &mut Self, node: &Box<StateVariableDefinition>) {
        self.handle_type_name(&node.type_name);
        self.handle_state_variable_attributes(&node.attributes);
        self.handle_terminal_node(&node.name);
        if let Some(value) = &node.value {
            self.handle_state_variable_definition_value(&value);
        }
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_state_variable_definition_value(
        self: &mut Self,
        node: &Box<StateVariableDefinitionValue>,
    ) {
        self.handle_terminal_node(&node.equal);
        self.handle_expression(&node.value);
    }

    #[allow(unused_variables)]
    fn handle_function_definition(self: &mut Self, node: &Box<FunctionDefinition>) {
        self.handle_terminal_node(&node.function_keyword);
        self.handle_function_name(&node.name);
        self.handle_parameters_declaration(&node.parameters);
        self.handle_function_attributes(&node.attributes);
        if let Some(value) = &node.returns {
            self.handle_returns_declaration(&value);
        }
        self.handle_function_body(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_parameters_declaration(self: &mut Self, node: &Box<ParametersDeclaration>) {
        self.handle_terminal_node(&node.open_paren);
        self.handle_parameters(&node.parameters);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_parameter(self: &mut Self, node: &Box<Parameter>) {
        self.handle_type_name(&node.type_name);
        if let Some(value) = &node.storage_location {
            self.handle_storage_location(&value);
        }
        if let Some(value) = &node.name {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_override_specifier(self: &mut Self, node: &Box<OverrideSpecifier>) {
        self.handle_terminal_node(&node.override_keyword);
        if let Some(value) = &node.overridden {
            self.handle_override_paths_declaration(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_override_paths_declaration(self: &mut Self, node: &Box<OverridePathsDeclaration>) {
        self.handle_terminal_node(&node.open_paren);
        self.handle_override_paths(&node.paths);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_returns_declaration(self: &mut Self, node: &Box<ReturnsDeclaration>) {
        self.handle_terminal_node(&node.returns_keyword);
        self.handle_parameters_declaration(&node.variables);
    }

    #[allow(unused_variables)]
    fn handle_constructor_definition(self: &mut Self, node: &Box<ConstructorDefinition>) {
        self.handle_terminal_node(&node.constructor_keyword);
        self.handle_parameters_declaration(&node.parameters);
        self.handle_constructor_attributes(&node.attributes);
        self.handle_block(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_unnamed_function_definition(self: &mut Self, node: &Box<UnnamedFunctionDefinition>) {
        self.handle_terminal_node(&node.function_keyword);
        self.handle_parameters_declaration(&node.parameters);
        self.handle_unnamed_function_attributes(&node.attributes);
        self.handle_function_body(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_fallback_function_definition(
        self: &mut Self,
        node: &Box<FallbackFunctionDefinition>,
    ) {
        self.handle_terminal_node(&node.fallback_keyword);
        self.handle_parameters_declaration(&node.parameters);
        self.handle_fallback_function_attributes(&node.attributes);
        if let Some(value) = &node.returns {
            self.handle_returns_declaration(&value);
        }
        self.handle_function_body(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_receive_function_definition(self: &mut Self, node: &Box<ReceiveFunctionDefinition>) {
        self.handle_terminal_node(&node.receive_keyword);
        self.handle_parameters_declaration(&node.parameters);
        self.handle_receive_function_attributes(&node.attributes);
        self.handle_function_body(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_modifier_definition(self: &mut Self, node: &Box<ModifierDefinition>) {
        self.handle_terminal_node(&node.modifier_keyword);
        self.handle_terminal_node(&node.name);
        if let Some(value) = &node.parameters {
            self.handle_parameters_declaration(&value);
        }
        self.handle_modifier_attributes(&node.attributes);
        self.handle_function_body(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_modifier_invocation(self: &mut Self, node: &Box<ModifierInvocation>) {
        self.handle_identifier_path(&node.name);
        if let Some(value) = &node.arguments {
            self.handle_arguments_declaration(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_event_definition(self: &mut Self, node: &Box<EventDefinition>) {
        self.handle_terminal_node(&node.event_keyword);
        self.handle_terminal_node(&node.name);
        self.handle_event_parameters_declaration(&node.parameters);
        if let Some(value) = &node.anonymous_keyword {
            self.handle_terminal_node(&value);
        }
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_event_parameters_declaration(
        self: &mut Self,
        node: &Box<EventParametersDeclaration>,
    ) {
        self.handle_terminal_node(&node.open_paren);
        self.handle_event_parameters(&node.parameters);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_event_parameter(self: &mut Self, node: &Box<EventParameter>) {
        self.handle_type_name(&node.type_name);
        if let Some(value) = &node.indexed_keyword {
            self.handle_terminal_node(&value);
        }
        if let Some(value) = &node.name {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_user_defined_value_type_definition(
        self: &mut Self,
        node: &Box<UserDefinedValueTypeDefinition>,
    ) {
        self.handle_terminal_node(&node.type_keyword);
        self.handle_terminal_node(&node.name);
        self.handle_terminal_node(&node.is_keyword);
        self.handle_elementary_type(&node.value_type);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_error_definition(self: &mut Self, node: &Box<ErrorDefinition>) {
        self.handle_terminal_node(&node.error_keyword);
        self.handle_terminal_node(&node.name);
        self.handle_error_parameters_declaration(&node.members);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_error_parameters_declaration(
        self: &mut Self,
        node: &Box<ErrorParametersDeclaration>,
    ) {
        self.handle_terminal_node(&node.open_paren);
        self.handle_error_parameters(&node.parameters);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_error_parameter(self: &mut Self, node: &Box<ErrorParameter>) {
        self.handle_type_name(&node.type_name);
        if let Some(value) = &node.name {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_array_type_name(self: &mut Self, node: &Box<ArrayTypeName>) {
        self.handle_type_name(&node.operand);
        self.handle_terminal_node(&node.open_bracket);
        if let Some(value) = &node.index {
            self.handle_expression(&value);
        }
        self.handle_terminal_node(&node.close_bracket);
    }

    #[allow(unused_variables)]
    fn handle_function_type(self: &mut Self, node: &Box<FunctionType>) {
        self.handle_terminal_node(&node.function_keyword);
        self.handle_parameters_declaration(&node.parameters);
        self.handle_function_type_attributes(&node.attributes);
        if let Some(value) = &node.returns {
            self.handle_returns_declaration(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_mapping_type(self: &mut Self, node: &Box<MappingType>) {
        self.handle_terminal_node(&node.mapping_keyword);
        self.handle_terminal_node(&node.open_paren);
        self.handle_mapping_key(&node.key_type);
        self.handle_terminal_node(&node.equal_greater_than);
        self.handle_mapping_value(&node.value_type);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_mapping_key(self: &mut Self, node: &Box<MappingKey>) {
        self.handle_mapping_key_type(&node.key_type);
        if let Some(value) = &node.name {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_mapping_value(self: &mut Self, node: &Box<MappingValue>) {
        self.handle_type_name(&node.type_name);
        if let Some(value) = &node.name {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_address_type(self: &mut Self, node: &Box<AddressType>) {
        self.handle_terminal_node(&node.address_keyword);
        if let Some(value) = &node.payable_keyword {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_block(self: &mut Self, node: &Box<Block>) {
        self.handle_terminal_node(&node.open_brace);
        self.handle_statements(&node.statements);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_unchecked_block(self: &mut Self, node: &Box<UncheckedBlock>) {
        self.handle_terminal_node(&node.unchecked_keyword);
        self.handle_block(&node.block);
    }

    #[allow(unused_variables)]
    fn handle_expression_statement(self: &mut Self, node: &Box<ExpressionStatement>) {
        self.handle_expression(&node.expression);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_assembly_statement(self: &mut Self, node: &Box<AssemblyStatement>) {
        self.handle_terminal_node(&node.assembly_keyword);
        if let Some(value) = &node.label {
            self.handle_string_literal(&value);
        }
        if let Some(value) = &node.flags {
            self.handle_assembly_flags_declaration(&value);
        }
        self.handle_yul_block(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_assembly_flags_declaration(self: &mut Self, node: &Box<AssemblyFlagsDeclaration>) {
        self.handle_terminal_node(&node.open_paren);
        self.handle_assembly_flags(&node.flags);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_tuple_deconstruction_statement(
        self: &mut Self,
        node: &Box<TupleDeconstructionStatement>,
    ) {
        if let Some(value) = &node.var_keyword {
            self.handle_terminal_node(&value);
        }
        self.handle_terminal_node(&node.open_paren);
        self.handle_tuple_deconstruction_elements(&node.elements);
        self.handle_terminal_node(&node.close_paren);
        self.handle_terminal_node(&node.equal);
        self.handle_expression(&node.expression);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_tuple_deconstruction_element(
        self: &mut Self,
        node: &Box<TupleDeconstructionElement>,
    ) {
        if let Some(value) = &node.member {
            self.handle_tuple_member(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_typed_tuple_member(self: &mut Self, node: &Box<TypedTupleMember>) {
        self.handle_type_name(&node.type_name);
        if let Some(value) = &node.storage_location {
            self.handle_storage_location(&value);
        }
        self.handle_terminal_node(&node.name);
    }

    #[allow(unused_variables)]
    fn handle_untyped_tuple_member(self: &mut Self, node: &Box<UntypedTupleMember>) {
        if let Some(value) = &node.storage_location {
            self.handle_storage_location(&value);
        }
        self.handle_terminal_node(&node.name);
    }

    #[allow(unused_variables)]
    fn handle_variable_declaration_statement(
        self: &mut Self,
        node: &Box<VariableDeclarationStatement>,
    ) {
        self.handle_variable_declaration_type(&node.variable_type);
        if let Some(value) = &node.storage_location {
            self.handle_storage_location(&value);
        }
        self.handle_terminal_node(&node.name);
        if let Some(value) = &node.value {
            self.handle_variable_declaration_value(&value);
        }
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_variable_declaration_value(self: &mut Self, node: &Box<VariableDeclarationValue>) {
        self.handle_terminal_node(&node.equal);
        self.handle_expression(&node.expression);
    }

    #[allow(unused_variables)]
    fn handle_if_statement(self: &mut Self, node: &Box<IfStatement>) {
        self.handle_terminal_node(&node.if_keyword);
        self.handle_terminal_node(&node.open_paren);
        self.handle_expression(&node.condition);
        self.handle_terminal_node(&node.close_paren);
        self.handle_statement(&node.body);
        if let Some(value) = &node.else_branch {
            self.handle_else_branch(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_else_branch(self: &mut Self, node: &Box<ElseBranch>) {
        self.handle_terminal_node(&node.else_keyword);
        self.handle_statement(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_for_statement(self: &mut Self, node: &Box<ForStatement>) {
        self.handle_terminal_node(&node.for_keyword);
        self.handle_terminal_node(&node.open_paren);
        self.handle_for_statement_initialization(&node.initialization);
        self.handle_for_statement_condition(&node.condition);
        if let Some(value) = &node.iterator {
            self.handle_expression(&value);
        }
        self.handle_terminal_node(&node.close_paren);
        self.handle_statement(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_while_statement(self: &mut Self, node: &Box<WhileStatement>) {
        self.handle_terminal_node(&node.while_keyword);
        self.handle_terminal_node(&node.open_paren);
        self.handle_expression(&node.condition);
        self.handle_terminal_node(&node.close_paren);
        self.handle_statement(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_do_while_statement(self: &mut Self, node: &Box<DoWhileStatement>) {
        self.handle_terminal_node(&node.do_keyword);
        self.handle_statement(&node.body);
        self.handle_terminal_node(&node.while_keyword);
        self.handle_terminal_node(&node.open_paren);
        self.handle_expression(&node.condition);
        self.handle_terminal_node(&node.close_paren);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_continue_statement(self: &mut Self, node: &Box<ContinueStatement>) {
        self.handle_terminal_node(&node.continue_keyword);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_break_statement(self: &mut Self, node: &Box<BreakStatement>) {
        self.handle_terminal_node(&node.break_keyword);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_return_statement(self: &mut Self, node: &Box<ReturnStatement>) {
        self.handle_terminal_node(&node.return_keyword);
        if let Some(value) = &node.expression {
            self.handle_expression(&value);
        }
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_emit_statement(self: &mut Self, node: &Box<EmitStatement>) {
        self.handle_terminal_node(&node.emit_keyword);
        self.handle_identifier_path(&node.event);
        self.handle_arguments_declaration(&node.arguments);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_try_statement(self: &mut Self, node: &Box<TryStatement>) {
        self.handle_terminal_node(&node.try_keyword);
        self.handle_expression(&node.expression);
        if let Some(value) = &node.returns {
            self.handle_returns_declaration(&value);
        }
        self.handle_block(&node.body);
        self.handle_catch_clauses(&node.catch_clauses);
    }

    #[allow(unused_variables)]
    fn handle_catch_clause(self: &mut Self, node: &Box<CatchClause>) {
        self.handle_terminal_node(&node.catch_keyword);
        if let Some(value) = &node.error {
            self.handle_catch_clause_error(&value);
        }
        self.handle_block(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_catch_clause_error(self: &mut Self, node: &Box<CatchClauseError>) {
        if let Some(value) = &node.name {
            self.handle_terminal_node(&value);
        }
        self.handle_parameters_declaration(&node.parameters);
    }

    #[allow(unused_variables)]
    fn handle_revert_statement(self: &mut Self, node: &Box<RevertStatement>) {
        self.handle_terminal_node(&node.revert_keyword);
        if let Some(value) = &node.error {
            self.handle_identifier_path(&value);
        }
        self.handle_arguments_declaration(&node.arguments);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_throw_statement(self: &mut Self, node: &Box<ThrowStatement>) {
        self.handle_terminal_node(&node.throw_keyword);
        self.handle_terminal_node(&node.semicolon);
    }

    #[allow(unused_variables)]
    fn handle_assignment_expression(self: &mut Self, node: &Box<AssignmentExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_conditional_expression(self: &mut Self, node: &Box<ConditionalExpression>) {
        self.handle_expression(&node.operand);
        self.handle_terminal_node(&node.question_mark);
        self.handle_expression(&node.true_expression);
        self.handle_terminal_node(&node.colon);
        self.handle_expression(&node.false_expression);
    }

    #[allow(unused_variables)]
    fn handle_or_expression(self: &mut Self, node: &Box<OrExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_and_expression(self: &mut Self, node: &Box<AndExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_equality_expression(self: &mut Self, node: &Box<EqualityExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_comparison_expression(self: &mut Self, node: &Box<ComparisonExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_bitwise_or_expression(self: &mut Self, node: &Box<BitwiseOrExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_bitwise_xor_expression(self: &mut Self, node: &Box<BitwiseXorExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_bitwise_and_expression(self: &mut Self, node: &Box<BitwiseAndExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_shift_expression(self: &mut Self, node: &Box<ShiftExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_additive_expression(self: &mut Self, node: &Box<AdditiveExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_multiplicative_expression(self: &mut Self, node: &Box<MultiplicativeExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_exponentiation_expression(self: &mut Self, node: &Box<ExponentiationExpression>) {
        self.handle_expression(&node.left_operand);
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.right_operand);
    }

    #[allow(unused_variables)]
    fn handle_postfix_expression(self: &mut Self, node: &Box<PostfixExpression>) {
        self.handle_expression(&node.operand);
        self.handle_terminal_node(&node.operator);
    }

    #[allow(unused_variables)]
    fn handle_prefix_expression(self: &mut Self, node: &Box<PrefixExpression>) {
        self.handle_terminal_node(&node.operator);
        self.handle_expression(&node.operand);
    }

    #[allow(unused_variables)]
    fn handle_function_call_expression(self: &mut Self, node: &Box<FunctionCallExpression>) {
        self.handle_expression(&node.operand);
        self.handle_arguments_declaration(&node.arguments);
    }

    #[allow(unused_variables)]
    fn handle_call_options_expression(self: &mut Self, node: &Box<CallOptionsExpression>) {
        self.handle_expression(&node.operand);
        self.handle_terminal_node(&node.open_brace);
        self.handle_call_options(&node.options);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_member_access_expression(self: &mut Self, node: &Box<MemberAccessExpression>) {
        self.handle_expression(&node.operand);
        self.handle_terminal_node(&node.period);
        self.handle_terminal_node(&node.member);
    }

    #[allow(unused_variables)]
    fn handle_index_access_expression(self: &mut Self, node: &Box<IndexAccessExpression>) {
        self.handle_expression(&node.operand);
        self.handle_terminal_node(&node.open_bracket);
        if let Some(value) = &node.start {
            self.handle_expression(&value);
        }
        if let Some(value) = &node.end {
            self.handle_index_access_end(&value);
        }
        self.handle_terminal_node(&node.close_bracket);
    }

    #[allow(unused_variables)]
    fn handle_index_access_end(self: &mut Self, node: &Box<IndexAccessEnd>) {
        self.handle_terminal_node(&node.colon);
        if let Some(value) = &node.end {
            self.handle_expression(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_positional_arguments_declaration(
        self: &mut Self,
        node: &Box<PositionalArgumentsDeclaration>,
    ) {
        self.handle_terminal_node(&node.open_paren);
        self.handle_positional_arguments(&node.arguments);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_named_arguments_declaration(self: &mut Self, node: &Box<NamedArgumentsDeclaration>) {
        self.handle_terminal_node(&node.open_paren);
        if let Some(value) = &node.arguments {
            self.handle_named_argument_group(&value);
        }
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_named_argument_group(self: &mut Self, node: &Box<NamedArgumentGroup>) {
        self.handle_terminal_node(&node.open_brace);
        self.handle_named_arguments(&node.arguments);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_named_argument(self: &mut Self, node: &Box<NamedArgument>) {
        self.handle_terminal_node(&node.name);
        self.handle_terminal_node(&node.colon);
        self.handle_expression(&node.value);
    }

    #[allow(unused_variables)]
    fn handle_type_expression(self: &mut Self, node: &Box<TypeExpression>) {
        self.handle_terminal_node(&node.type_keyword);
        self.handle_terminal_node(&node.open_paren);
        self.handle_type_name(&node.type_name);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_new_expression(self: &mut Self, node: &Box<NewExpression>) {
        self.handle_terminal_node(&node.new_keyword);
        self.handle_type_name(&node.type_name);
    }

    #[allow(unused_variables)]
    fn handle_tuple_expression(self: &mut Self, node: &Box<TupleExpression>) {
        self.handle_terminal_node(&node.open_paren);
        self.handle_tuple_values(&node.items);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_tuple_value(self: &mut Self, node: &Box<TupleValue>) {
        if let Some(value) = &node.expression {
            self.handle_expression(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_array_expression(self: &mut Self, node: &Box<ArrayExpression>) {
        self.handle_terminal_node(&node.open_bracket);
        self.handle_array_values(&node.items);
        self.handle_terminal_node(&node.close_bracket);
    }

    #[allow(unused_variables)]
    fn handle_hex_number_expression(self: &mut Self, node: &Box<HexNumberExpression>) {
        self.handle_terminal_node(&node.literal);
        if let Some(value) = &node.unit {
            self.handle_number_unit(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_decimal_number_expression(self: &mut Self, node: &Box<DecimalNumberExpression>) {
        self.handle_terminal_node(&node.literal);
        if let Some(value) = &node.unit {
            self.handle_number_unit(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_block(self: &mut Self, node: &Box<YulBlock>) {
        self.handle_terminal_node(&node.open_brace);
        self.handle_yul_statements(&node.statements);
        self.handle_terminal_node(&node.close_brace);
    }

    #[allow(unused_variables)]
    fn handle_yul_function_definition(self: &mut Self, node: &Box<YulFunctionDefinition>) {
        self.handle_terminal_node(&node.function_keyword);
        self.handle_terminal_node(&node.name);
        self.handle_yul_parameters_declaration(&node.parameters);
        if let Some(value) = &node.returns {
            self.handle_yul_returns_declaration(&value);
        }
        self.handle_yul_block(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_yul_parameters_declaration(self: &mut Self, node: &Box<YulParametersDeclaration>) {
        self.handle_terminal_node(&node.open_paren);
        self.handle_yul_parameters(&node.parameters);
        self.handle_terminal_node(&node.close_paren);
    }

    #[allow(unused_variables)]
    fn handle_yul_returns_declaration(self: &mut Self, node: &Box<YulReturnsDeclaration>) {
        self.handle_terminal_node(&node.minus_greater_than);
        self.handle_yul_variable_names(&node.variables);
    }

    #[allow(unused_variables)]
    fn handle_yul_variable_declaration_statement(
        self: &mut Self,
        node: &Box<YulVariableDeclarationStatement>,
    ) {
        self.handle_terminal_node(&node.let_keyword);
        self.handle_yul_variable_names(&node.variables);
        if let Some(value) = &node.value {
            self.handle_yul_variable_declaration_value(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_variable_declaration_value(
        self: &mut Self,
        node: &Box<YulVariableDeclarationValue>,
    ) {
        self.handle_yul_assignment_operator(&node.assignment);
        self.handle_yul_expression(&node.expression);
    }

    #[allow(unused_variables)]
    fn handle_yul_variable_assignment_statement(
        self: &mut Self,
        node: &Box<YulVariableAssignmentStatement>,
    ) {
        self.handle_yul_paths(&node.variables);
        self.handle_yul_assignment_operator(&node.assignment);
        self.handle_yul_expression(&node.expression);
    }

    #[allow(unused_variables)]
    fn handle_yul_colon_and_equal(self: &mut Self, node: &Box<YulColonAndEqual>) {
        self.handle_terminal_node(&node.colon);
        self.handle_terminal_node(&node.equal);
    }

    #[allow(unused_variables)]
    fn handle_yul_stack_assignment_statement(
        self: &mut Self,
        node: &Box<YulStackAssignmentStatement>,
    ) {
        self.handle_yul_stack_assignment_operator(&node.assignment);
        self.handle_terminal_node(&node.variable);
    }

    #[allow(unused_variables)]
    fn handle_yul_equal_and_colon(self: &mut Self, node: &Box<YulEqualAndColon>) {
        self.handle_terminal_node(&node.equal);
        self.handle_terminal_node(&node.colon);
    }

    #[allow(unused_variables)]
    fn handle_yul_if_statement(self: &mut Self, node: &Box<YulIfStatement>) {
        self.handle_terminal_node(&node.if_keyword);
        self.handle_yul_expression(&node.condition);
        self.handle_yul_block(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_yul_for_statement(self: &mut Self, node: &Box<YulForStatement>) {
        self.handle_terminal_node(&node.for_keyword);
        self.handle_yul_block(&node.initialization);
        self.handle_yul_expression(&node.condition);
        self.handle_yul_block(&node.iterator);
        self.handle_yul_block(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_yul_switch_statement(self: &mut Self, node: &Box<YulSwitchStatement>) {
        self.handle_terminal_node(&node.switch_keyword);
        self.handle_yul_expression(&node.expression);
        self.handle_yul_switch_cases(&node.cases);
    }

    #[allow(unused_variables)]
    fn handle_yul_default_case(self: &mut Self, node: &Box<YulDefaultCase>) {
        self.handle_terminal_node(&node.default_keyword);
        self.handle_yul_block(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_yul_value_case(self: &mut Self, node: &Box<YulValueCase>) {
        self.handle_terminal_node(&node.case_keyword);
        self.handle_yul_literal(&node.value);
        self.handle_yul_block(&node.body);
    }

    #[allow(unused_variables)]
    fn handle_yul_leave_statement(self: &mut Self, node: &Box<YulLeaveStatement>) {
        self.handle_terminal_node(&node.leave_keyword);
    }

    #[allow(unused_variables)]
    fn handle_yul_break_statement(self: &mut Self, node: &Box<YulBreakStatement>) {
        self.handle_terminal_node(&node.break_keyword);
    }

    #[allow(unused_variables)]
    fn handle_yul_continue_statement(self: &mut Self, node: &Box<YulContinueStatement>) {
        self.handle_terminal_node(&node.continue_keyword);
    }

    #[allow(unused_variables)]
    fn handle_yul_label(self: &mut Self, node: &Box<YulLabel>) {
        self.handle_terminal_node(&node.label);
        self.handle_terminal_node(&node.colon);
    }

    #[allow(unused_variables)]
    fn handle_yul_function_call_expression(self: &mut Self, node: &Box<YulFunctionCallExpression>) {
        self.handle_yul_expression(&node.operand);
        self.handle_terminal_node(&node.open_paren);
        self.handle_yul_arguments(&node.arguments);
        self.handle_terminal_node(&node.close_paren);
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
    fn handle_source_unit_members(self: &mut Self, node: &Box<SourceUnitMembers>) {
        for key in &node.items {
            self.handle_source_unit_member(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_version_expression_set(self: &mut Self, node: &Box<VersionExpressionSet>) {
        for key in &node.items {
            self.handle_version_expression(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_contract_members(self: &mut Self, node: &Box<ContractMembers>) {
        for key in &node.items {
            self.handle_contract_member(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_interface_members(self: &mut Self, node: &Box<InterfaceMembers>) {
        for key in &node.items {
            self.handle_contract_member(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_library_members(self: &mut Self, node: &Box<LibraryMembers>) {
        for key in &node.items {
            self.handle_contract_member(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_struct_members(self: &mut Self, node: &Box<StructMembers>) {
        for key in &node.items {
            self.handle_struct_member(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_state_variable_attributes(self: &mut Self, node: &Box<StateVariableAttributes>) {
        for key in &node.items {
            self.handle_state_variable_attribute(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_function_attributes(self: &mut Self, node: &Box<FunctionAttributes>) {
        for key in &node.items {
            self.handle_function_attribute(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_constructor_attributes(self: &mut Self, node: &Box<ConstructorAttributes>) {
        for key in &node.items {
            self.handle_constructor_attribute(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_unnamed_function_attributes(self: &mut Self, node: &Box<UnnamedFunctionAttributes>) {
        for key in &node.items {
            self.handle_unnamed_function_attribute(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_fallback_function_attributes(
        self: &mut Self,
        node: &Box<FallbackFunctionAttributes>,
    ) {
        for key in &node.items {
            self.handle_fallback_function_attribute(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_receive_function_attributes(self: &mut Self, node: &Box<ReceiveFunctionAttributes>) {
        for key in &node.items {
            self.handle_receive_function_attribute(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_modifier_attributes(self: &mut Self, node: &Box<ModifierAttributes>) {
        for key in &node.items {
            self.handle_modifier_attribute(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_function_type_attributes(self: &mut Self, node: &Box<FunctionTypeAttributes>) {
        for key in &node.items {
            self.handle_function_type_attribute(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_statements(self: &mut Self, node: &Box<Statements>) {
        for key in &node.items {
            self.handle_statement(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_catch_clauses(self: &mut Self, node: &Box<CatchClauses>) {
        for key in &node.items {
            self.handle_catch_clause(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_string_literals(self: &mut Self, node: &Box<StringLiterals>) {
        for key in &node.items {
            self.handle_string_literal(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_hex_string_literals(self: &mut Self, node: &Box<HexStringLiterals>) {
        for key in &node.items {
            self.handle_hex_string_literal(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_unicode_string_literals(self: &mut Self, node: &Box<UnicodeStringLiterals>) {
        for key in &node.items {
            self.handle_unicode_string_literal(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_statements(self: &mut Self, node: &Box<YulStatements>) {
        for key in &node.items {
            self.handle_yul_statement(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_switch_cases(self: &mut Self, node: &Box<YulSwitchCases>) {
        for key in &node.items {
            self.handle_yul_switch_case(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_version_expression_sets(self: &mut Self, node: &Box<VersionExpressionSets>) {
        for key in &node.items {
            self.handle_version_expression_set(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_simple_version_literal(self: &mut Self, node: &Box<SimpleVersionLiteral>) {
        for key in &node.items {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_import_deconstruction_symbols(
        self: &mut Self,
        node: &Box<ImportDeconstructionSymbols>,
    ) {
        for key in &node.items {
            self.handle_import_deconstruction_symbol(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_using_deconstruction_symbols(
        self: &mut Self,
        node: &Box<UsingDeconstructionSymbols>,
    ) {
        for key in &node.items {
            self.handle_using_deconstruction_symbol(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_inheritance_types(self: &mut Self, node: &Box<InheritanceTypes>) {
        for key in &node.items {
            self.handle_inheritance_type(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_enum_members(self: &mut Self, node: &Box<EnumMembers>) {
        for key in &node.items {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_parameters(self: &mut Self, node: &Box<Parameters>) {
        for key in &node.items {
            self.handle_parameter(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_override_paths(self: &mut Self, node: &Box<OverridePaths>) {
        for key in &node.items {
            self.handle_identifier_path(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_event_parameters(self: &mut Self, node: &Box<EventParameters>) {
        for key in &node.items {
            self.handle_event_parameter(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_error_parameters(self: &mut Self, node: &Box<ErrorParameters>) {
        for key in &node.items {
            self.handle_error_parameter(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_assembly_flags(self: &mut Self, node: &Box<AssemblyFlags>) {
        for key in &node.items {
            self.handle_string_literal(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_tuple_deconstruction_elements(
        self: &mut Self,
        node: &Box<TupleDeconstructionElements>,
    ) {
        for key in &node.items {
            self.handle_tuple_deconstruction_element(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_positional_arguments(self: &mut Self, node: &Box<PositionalArguments>) {
        for key in &node.items {
            self.handle_expression(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_named_arguments(self: &mut Self, node: &Box<NamedArguments>) {
        for key in &node.items {
            self.handle_named_argument(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_call_options(self: &mut Self, node: &Box<CallOptions>) {
        for key in &node.items {
            self.handle_named_argument(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_tuple_values(self: &mut Self, node: &Box<TupleValues>) {
        for key in &node.items {
            self.handle_tuple_value(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_array_values(self: &mut Self, node: &Box<ArrayValues>) {
        for key in &node.items {
            self.handle_expression(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_identifier_path(self: &mut Self, node: &Box<IdentifierPath>) {
        for key in &node.items {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_parameters(self: &mut Self, node: &Box<YulParameters>) {
        for key in &node.items {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_variable_names(self: &mut Self, node: &Box<YulVariableNames>) {
        for key in &node.items {
            self.handle_terminal_node(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_arguments(self: &mut Self, node: &Box<YulArguments>) {
        for key in &node.items {
            self.handle_yul_expression(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_paths(self: &mut Self, node: &Box<YulPaths>) {
        for key in &node.items {
            self.handle_yul_path(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_yul_path(self: &mut Self, node: &Box<YulPath>) {
        for key in &node.items {
            self.handle_terminal_node(&value);
        }
    }
}
