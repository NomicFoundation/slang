// Generated on 2025-01-29T15:47:57.404Z
use super::model::*;

pub trait InPlaceTransformer {
    #[allow(unused_mut)]
    fn transform_terminal_kind(self: &mut Self, mut node: TerminalKind) -> TerminalKind {
        node
    }

    #[allow(unused_mut)]
    fn transform_terminal_node(self: &mut Self, mut node: TerminalNode) -> TerminalNode {
        node
    }

    #[allow(unused_mut)]
    fn transform_source_unit(self: &mut Self, mut node: Box<SourceUnit>) -> Box<SourceUnit> {
        node.members = self.transform_source_unit_members(node.members);
        node
    }

    #[allow(unused_mut)]
    fn transform_pragma_directive(
        self: &mut Self,
        mut node: Box<PragmaDirective>,
    ) -> Box<PragmaDirective> {
        node.pragma_keyword = self.transform_terminal_node(node.pragma_keyword);
        node.pragma = self.transform_pragma(node.pragma);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_abicoder_pragma(
        self: &mut Self,
        mut node: Box<AbicoderPragma>,
    ) -> Box<AbicoderPragma> {
        node.abicoder_keyword = self.transform_terminal_node(node.abicoder_keyword);
        node.version = self.transform_terminal_node(node.version);
        node
    }

    #[allow(unused_mut)]
    fn transform_experimental_pragma(
        self: &mut Self,
        mut node: Box<ExperimentalPragma>,
    ) -> Box<ExperimentalPragma> {
        node.experimental_keyword = self.transform_terminal_node(node.experimental_keyword);
        node.feature = self.transform_experimental_feature(node.feature);
        node
    }

    #[allow(unused_mut)]
    fn transform_version_pragma(
        self: &mut Self,
        mut node: Box<VersionPragma>,
    ) -> Box<VersionPragma> {
        node.solidity_keyword = self.transform_terminal_node(node.solidity_keyword);
        node.sets = self.transform_version_expression_sets(node.sets);
        node
    }

    #[allow(unused_mut)]
    fn transform_version_range(self: &mut Self, mut node: Box<VersionRange>) -> Box<VersionRange> {
        node.start = self.transform_version_literal(node.start);
        node.minus = self.transform_terminal_node(node.minus);
        node.end = self.transform_version_literal(node.end);
        node
    }

    #[allow(unused_mut)]
    fn transform_version_term(self: &mut Self, mut node: Box<VersionTerm>) -> Box<VersionTerm> {
        node.operator = node
            .operator
            .map(|value| self.transform_version_operator(value));
        node.literal = self.transform_version_literal(node.literal);
        node
    }

    #[allow(unused_mut)]
    fn transform_import_directive(
        self: &mut Self,
        mut node: Box<ImportDirective>,
    ) -> Box<ImportDirective> {
        node.import_keyword = self.transform_terminal_node(node.import_keyword);
        node.clause = self.transform_import_clause(node.clause);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_path_import(self: &mut Self, mut node: Box<PathImport>) -> Box<PathImport> {
        node.path = self.transform_string_literal(node.path);
        node.alias = node.alias.map(|value| self.transform_import_alias(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_named_import(self: &mut Self, mut node: Box<NamedImport>) -> Box<NamedImport> {
        node.asterisk = self.transform_terminal_node(node.asterisk);
        node.alias = self.transform_import_alias(node.alias);
        node.from_keyword = self.transform_terminal_node(node.from_keyword);
        node.path = self.transform_string_literal(node.path);
        node
    }

    #[allow(unused_mut)]
    fn transform_import_deconstruction(
        self: &mut Self,
        mut node: Box<ImportDeconstruction>,
    ) -> Box<ImportDeconstruction> {
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.symbols = self.transform_import_deconstruction_symbols(node.symbols);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node.from_keyword = self.transform_terminal_node(node.from_keyword);
        node.path = self.transform_string_literal(node.path);
        node
    }

    #[allow(unused_mut)]
    fn transform_import_deconstruction_symbol(
        self: &mut Self,
        mut node: Box<ImportDeconstructionSymbol>,
    ) -> Box<ImportDeconstructionSymbol> {
        node.name = self.transform_terminal_node(node.name);
        node.alias = node.alias.map(|value| self.transform_import_alias(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_import_alias(self: &mut Self, mut node: Box<ImportAlias>) -> Box<ImportAlias> {
        node.as_keyword = self.transform_terminal_node(node.as_keyword);
        node.identifier = self.transform_terminal_node(node.identifier);
        node
    }

    #[allow(unused_mut)]
    fn transform_using_directive(
        self: &mut Self,
        mut node: Box<UsingDirective>,
    ) -> Box<UsingDirective> {
        node.using_keyword = self.transform_terminal_node(node.using_keyword);
        node.clause = self.transform_using_clause(node.clause);
        node.for_keyword = self.transform_terminal_node(node.for_keyword);
        node.target = self.transform_using_target(node.target);
        node.global_keyword = node
            .global_keyword
            .map(|value| self.transform_terminal_node(value));
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_using_deconstruction(
        self: &mut Self,
        mut node: Box<UsingDeconstruction>,
    ) -> Box<UsingDeconstruction> {
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.symbols = self.transform_using_deconstruction_symbols(node.symbols);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_using_deconstruction_symbol(
        self: &mut Self,
        mut node: Box<UsingDeconstructionSymbol>,
    ) -> Box<UsingDeconstructionSymbol> {
        node.name = self.transform_identifier_path(node.name);
        node.alias = node.alias.map(|value| self.transform_using_alias(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_using_alias(self: &mut Self, mut node: Box<UsingAlias>) -> Box<UsingAlias> {
        node.as_keyword = self.transform_terminal_node(node.as_keyword);
        node.operator = self.transform_using_operator(node.operator);
        node
    }

    #[allow(unused_mut)]
    fn transform_contract_definition(
        self: &mut Self,
        mut node: Box<ContractDefinition>,
    ) -> Box<ContractDefinition> {
        node.abstract_keyword = node
            .abstract_keyword
            .map(|value| self.transform_terminal_node(value));
        node.contract_keyword = self.transform_terminal_node(node.contract_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.inheritance = node
            .inheritance
            .map(|value| self.transform_inheritance_specifier(value));
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.members = self.transform_contract_members(node.members);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_inheritance_specifier(
        self: &mut Self,
        mut node: Box<InheritanceSpecifier>,
    ) -> Box<InheritanceSpecifier> {
        node.is_keyword = self.transform_terminal_node(node.is_keyword);
        node.types = self.transform_inheritance_types(node.types);
        node
    }

    #[allow(unused_mut)]
    fn transform_inheritance_type(
        self: &mut Self,
        mut node: Box<InheritanceType>,
    ) -> Box<InheritanceType> {
        node.type_name = self.transform_identifier_path(node.type_name);
        node.arguments = node
            .arguments
            .map(|value| self.transform_arguments_declaration(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_interface_definition(
        self: &mut Self,
        mut node: Box<InterfaceDefinition>,
    ) -> Box<InterfaceDefinition> {
        node.interface_keyword = self.transform_terminal_node(node.interface_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.inheritance = node
            .inheritance
            .map(|value| self.transform_inheritance_specifier(value));
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.members = self.transform_interface_members(node.members);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_library_definition(
        self: &mut Self,
        mut node: Box<LibraryDefinition>,
    ) -> Box<LibraryDefinition> {
        node.library_keyword = self.transform_terminal_node(node.library_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.members = self.transform_library_members(node.members);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_struct_definition(
        self: &mut Self,
        mut node: Box<StructDefinition>,
    ) -> Box<StructDefinition> {
        node.struct_keyword = self.transform_terminal_node(node.struct_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.members = self.transform_struct_members(node.members);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_struct_member(self: &mut Self, mut node: Box<StructMember>) -> Box<StructMember> {
        node.type_name = self.transform_type_name(node.type_name);
        node.name = self.transform_terminal_node(node.name);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_enum_definition(
        self: &mut Self,
        mut node: Box<EnumDefinition>,
    ) -> Box<EnumDefinition> {
        node.enum_keyword = self.transform_terminal_node(node.enum_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.members = self.transform_enum_members(node.members);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_constant_definition(
        self: &mut Self,
        mut node: Box<ConstantDefinition>,
    ) -> Box<ConstantDefinition> {
        node.type_name = self.transform_type_name(node.type_name);
        node.constant_keyword = self.transform_terminal_node(node.constant_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.equal = self.transform_terminal_node(node.equal);
        node.value = self.transform_expression(node.value);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_state_variable_definition(
        self: &mut Self,
        mut node: Box<StateVariableDefinition>,
    ) -> Box<StateVariableDefinition> {
        node.type_name = self.transform_type_name(node.type_name);
        node.attributes = self.transform_state_variable_attributes(node.attributes);
        node.name = self.transform_terminal_node(node.name);
        node.value = node
            .value
            .map(|value| self.transform_state_variable_definition_value(value));
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_state_variable_definition_value(
        self: &mut Self,
        mut node: Box<StateVariableDefinitionValue>,
    ) -> Box<StateVariableDefinitionValue> {
        node.equal = self.transform_terminal_node(node.equal);
        node.value = self.transform_expression(node.value);
        node
    }

    #[allow(unused_mut)]
    fn transform_function_definition(
        self: &mut Self,
        mut node: Box<FunctionDefinition>,
    ) -> Box<FunctionDefinition> {
        node.function_keyword = self.transform_terminal_node(node.function_keyword);
        node.name = self.transform_function_name(node.name);
        node.parameters = self.transform_parameters_declaration(node.parameters);
        node.attributes = self.transform_function_attributes(node.attributes);
        node.returns = node
            .returns
            .map(|value| self.transform_returns_declaration(value));
        node.body = self.transform_function_body(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_parameters_declaration(
        self: &mut Self,
        mut node: Box<ParametersDeclaration>,
    ) -> Box<ParametersDeclaration> {
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.parameters = self.transform_parameters(node.parameters);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_parameter(self: &mut Self, mut node: Box<Parameter>) -> Box<Parameter> {
        node.type_name = self.transform_type_name(node.type_name);
        node.storage_location = node
            .storage_location
            .map(|value| self.transform_storage_location(value));
        node.name = node.name.map(|value| self.transform_terminal_node(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_override_specifier(
        self: &mut Self,
        mut node: Box<OverrideSpecifier>,
    ) -> Box<OverrideSpecifier> {
        node.override_keyword = self.transform_terminal_node(node.override_keyword);
        node.overridden = node
            .overridden
            .map(|value| self.transform_override_paths_declaration(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_override_paths_declaration(
        self: &mut Self,
        mut node: Box<OverridePathsDeclaration>,
    ) -> Box<OverridePathsDeclaration> {
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.paths = self.transform_override_paths(node.paths);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_returns_declaration(
        self: &mut Self,
        mut node: Box<ReturnsDeclaration>,
    ) -> Box<ReturnsDeclaration> {
        node.returns_keyword = self.transform_terminal_node(node.returns_keyword);
        node.variables = self.transform_parameters_declaration(node.variables);
        node
    }

    #[allow(unused_mut)]
    fn transform_constructor_definition(
        self: &mut Self,
        mut node: Box<ConstructorDefinition>,
    ) -> Box<ConstructorDefinition> {
        node.constructor_keyword = self.transform_terminal_node(node.constructor_keyword);
        node.parameters = self.transform_parameters_declaration(node.parameters);
        node.attributes = self.transform_constructor_attributes(node.attributes);
        node.body = self.transform_block(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_unnamed_function_definition(
        self: &mut Self,
        mut node: Box<UnnamedFunctionDefinition>,
    ) -> Box<UnnamedFunctionDefinition> {
        node.function_keyword = self.transform_terminal_node(node.function_keyword);
        node.parameters = self.transform_parameters_declaration(node.parameters);
        node.attributes = self.transform_unnamed_function_attributes(node.attributes);
        node.body = self.transform_function_body(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_fallback_function_definition(
        self: &mut Self,
        mut node: Box<FallbackFunctionDefinition>,
    ) -> Box<FallbackFunctionDefinition> {
        node.fallback_keyword = self.transform_terminal_node(node.fallback_keyword);
        node.parameters = self.transform_parameters_declaration(node.parameters);
        node.attributes = self.transform_fallback_function_attributes(node.attributes);
        node.returns = node
            .returns
            .map(|value| self.transform_returns_declaration(value));
        node.body = self.transform_function_body(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_receive_function_definition(
        self: &mut Self,
        mut node: Box<ReceiveFunctionDefinition>,
    ) -> Box<ReceiveFunctionDefinition> {
        node.receive_keyword = self.transform_terminal_node(node.receive_keyword);
        node.parameters = self.transform_parameters_declaration(node.parameters);
        node.attributes = self.transform_receive_function_attributes(node.attributes);
        node.body = self.transform_function_body(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_modifier_definition(
        self: &mut Self,
        mut node: Box<ModifierDefinition>,
    ) -> Box<ModifierDefinition> {
        node.modifier_keyword = self.transform_terminal_node(node.modifier_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.parameters = node
            .parameters
            .map(|value| self.transform_parameters_declaration(value));
        node.attributes = self.transform_modifier_attributes(node.attributes);
        node.body = self.transform_function_body(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_modifier_invocation(
        self: &mut Self,
        mut node: Box<ModifierInvocation>,
    ) -> Box<ModifierInvocation> {
        node.name = self.transform_identifier_path(node.name);
        node.arguments = node
            .arguments
            .map(|value| self.transform_arguments_declaration(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_event_definition(
        self: &mut Self,
        mut node: Box<EventDefinition>,
    ) -> Box<EventDefinition> {
        node.event_keyword = self.transform_terminal_node(node.event_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.parameters = self.transform_event_parameters_declaration(node.parameters);
        node.anonymous_keyword = node
            .anonymous_keyword
            .map(|value| self.transform_terminal_node(value));
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_event_parameters_declaration(
        self: &mut Self,
        mut node: Box<EventParametersDeclaration>,
    ) -> Box<EventParametersDeclaration> {
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.parameters = self.transform_event_parameters(node.parameters);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_event_parameter(
        self: &mut Self,
        mut node: Box<EventParameter>,
    ) -> Box<EventParameter> {
        node.type_name = self.transform_type_name(node.type_name);
        node.indexed_keyword = node
            .indexed_keyword
            .map(|value| self.transform_terminal_node(value));
        node.name = node.name.map(|value| self.transform_terminal_node(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_user_defined_value_type_definition(
        self: &mut Self,
        mut node: Box<UserDefinedValueTypeDefinition>,
    ) -> Box<UserDefinedValueTypeDefinition> {
        node.type_keyword = self.transform_terminal_node(node.type_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.is_keyword = self.transform_terminal_node(node.is_keyword);
        node.value_type = self.transform_elementary_type(node.value_type);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_error_definition(
        self: &mut Self,
        mut node: Box<ErrorDefinition>,
    ) -> Box<ErrorDefinition> {
        node.error_keyword = self.transform_terminal_node(node.error_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.members = self.transform_error_parameters_declaration(node.members);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_error_parameters_declaration(
        self: &mut Self,
        mut node: Box<ErrorParametersDeclaration>,
    ) -> Box<ErrorParametersDeclaration> {
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.parameters = self.transform_error_parameters(node.parameters);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_error_parameter(
        self: &mut Self,
        mut node: Box<ErrorParameter>,
    ) -> Box<ErrorParameter> {
        node.type_name = self.transform_type_name(node.type_name);
        node.name = node.name.map(|value| self.transform_terminal_node(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_array_type_name(
        self: &mut Self,
        mut node: Box<ArrayTypeName>,
    ) -> Box<ArrayTypeName> {
        node.operand = self.transform_type_name(node.operand);
        node.open_bracket = self.transform_terminal_node(node.open_bracket);
        node.index = node.index.map(|value| self.transform_expression(value));
        node.close_bracket = self.transform_terminal_node(node.close_bracket);
        node
    }

    #[allow(unused_mut)]
    fn transform_function_type(self: &mut Self, mut node: Box<FunctionType>) -> Box<FunctionType> {
        node.function_keyword = self.transform_terminal_node(node.function_keyword);
        node.parameters = self.transform_parameters_declaration(node.parameters);
        node.attributes = self.transform_function_type_attributes(node.attributes);
        node.returns = node
            .returns
            .map(|value| self.transform_returns_declaration(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_mapping_type(self: &mut Self, mut node: Box<MappingType>) -> Box<MappingType> {
        node.mapping_keyword = self.transform_terminal_node(node.mapping_keyword);
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.key_type = self.transform_mapping_key(node.key_type);
        node.equal_greater_than = self.transform_terminal_node(node.equal_greater_than);
        node.value_type = self.transform_mapping_value(node.value_type);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_mapping_key(self: &mut Self, mut node: Box<MappingKey>) -> Box<MappingKey> {
        node.key_type = self.transform_mapping_key_type(node.key_type);
        node.name = node.name.map(|value| self.transform_terminal_node(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_mapping_value(self: &mut Self, mut node: Box<MappingValue>) -> Box<MappingValue> {
        node.type_name = self.transform_type_name(node.type_name);
        node.name = node.name.map(|value| self.transform_terminal_node(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_address_type(self: &mut Self, mut node: Box<AddressType>) -> Box<AddressType> {
        node.address_keyword = self.transform_terminal_node(node.address_keyword);
        node.payable_keyword = node
            .payable_keyword
            .map(|value| self.transform_terminal_node(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_block(self: &mut Self, mut node: Box<Block>) -> Box<Block> {
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.statements = self.transform_statements(node.statements);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_unchecked_block(
        self: &mut Self,
        mut node: Box<UncheckedBlock>,
    ) -> Box<UncheckedBlock> {
        node.unchecked_keyword = self.transform_terminal_node(node.unchecked_keyword);
        node.block = self.transform_block(node.block);
        node
    }

    #[allow(unused_mut)]
    fn transform_expression_statement(
        self: &mut Self,
        mut node: Box<ExpressionStatement>,
    ) -> Box<ExpressionStatement> {
        node.expression = self.transform_expression(node.expression);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_assembly_statement(
        self: &mut Self,
        mut node: Box<AssemblyStatement>,
    ) -> Box<AssemblyStatement> {
        node.assembly_keyword = self.transform_terminal_node(node.assembly_keyword);
        node.label = node.label.map(|value| self.transform_string_literal(value));
        node.flags = node
            .flags
            .map(|value| self.transform_assembly_flags_declaration(value));
        node.body = self.transform_yul_block(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_assembly_flags_declaration(
        self: &mut Self,
        mut node: Box<AssemblyFlagsDeclaration>,
    ) -> Box<AssemblyFlagsDeclaration> {
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.flags = self.transform_assembly_flags(node.flags);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_tuple_deconstruction_statement(
        self: &mut Self,
        mut node: Box<TupleDeconstructionStatement>,
    ) -> Box<TupleDeconstructionStatement> {
        node.var_keyword = node
            .var_keyword
            .map(|value| self.transform_terminal_node(value));
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.elements = self.transform_tuple_deconstruction_elements(node.elements);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node.equal = self.transform_terminal_node(node.equal);
        node.expression = self.transform_expression(node.expression);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_tuple_deconstruction_element(
        self: &mut Self,
        mut node: Box<TupleDeconstructionElement>,
    ) -> Box<TupleDeconstructionElement> {
        node.member = node.member.map(|value| self.transform_tuple_member(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_typed_tuple_member(
        self: &mut Self,
        mut node: Box<TypedTupleMember>,
    ) -> Box<TypedTupleMember> {
        node.type_name = self.transform_type_name(node.type_name);
        node.storage_location = node
            .storage_location
            .map(|value| self.transform_storage_location(value));
        node.name = self.transform_terminal_node(node.name);
        node
    }

    #[allow(unused_mut)]
    fn transform_untyped_tuple_member(
        self: &mut Self,
        mut node: Box<UntypedTupleMember>,
    ) -> Box<UntypedTupleMember> {
        node.storage_location = node
            .storage_location
            .map(|value| self.transform_storage_location(value));
        node.name = self.transform_terminal_node(node.name);
        node
    }

    #[allow(unused_mut)]
    fn transform_variable_declaration_statement(
        self: &mut Self,
        mut node: Box<VariableDeclarationStatement>,
    ) -> Box<VariableDeclarationStatement> {
        node.variable_type = self.transform_variable_declaration_type(node.variable_type);
        node.storage_location = node
            .storage_location
            .map(|value| self.transform_storage_location(value));
        node.name = self.transform_terminal_node(node.name);
        node.value = node
            .value
            .map(|value| self.transform_variable_declaration_value(value));
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_variable_declaration_value(
        self: &mut Self,
        mut node: Box<VariableDeclarationValue>,
    ) -> Box<VariableDeclarationValue> {
        node.equal = self.transform_terminal_node(node.equal);
        node.expression = self.transform_expression(node.expression);
        node
    }

    #[allow(unused_mut)]
    fn transform_if_statement(self: &mut Self, mut node: Box<IfStatement>) -> Box<IfStatement> {
        node.if_keyword = self.transform_terminal_node(node.if_keyword);
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.condition = self.transform_expression(node.condition);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node.body = self.transform_statement(node.body);
        node.else_branch = node
            .else_branch
            .map(|value| self.transform_else_branch(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_else_branch(self: &mut Self, mut node: Box<ElseBranch>) -> Box<ElseBranch> {
        node.else_keyword = self.transform_terminal_node(node.else_keyword);
        node.body = self.transform_statement(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_for_statement(self: &mut Self, mut node: Box<ForStatement>) -> Box<ForStatement> {
        node.for_keyword = self.transform_terminal_node(node.for_keyword);
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.initialization = self.transform_for_statement_initialization(node.initialization);
        node.condition = self.transform_for_statement_condition(node.condition);
        node.iterator = node.iterator.map(|value| self.transform_expression(value));
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node.body = self.transform_statement(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_while_statement(
        self: &mut Self,
        mut node: Box<WhileStatement>,
    ) -> Box<WhileStatement> {
        node.while_keyword = self.transform_terminal_node(node.while_keyword);
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.condition = self.transform_expression(node.condition);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node.body = self.transform_statement(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_do_while_statement(
        self: &mut Self,
        mut node: Box<DoWhileStatement>,
    ) -> Box<DoWhileStatement> {
        node.do_keyword = self.transform_terminal_node(node.do_keyword);
        node.body = self.transform_statement(node.body);
        node.while_keyword = self.transform_terminal_node(node.while_keyword);
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.condition = self.transform_expression(node.condition);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_continue_statement(
        self: &mut Self,
        mut node: Box<ContinueStatement>,
    ) -> Box<ContinueStatement> {
        node.continue_keyword = self.transform_terminal_node(node.continue_keyword);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_break_statement(
        self: &mut Self,
        mut node: Box<BreakStatement>,
    ) -> Box<BreakStatement> {
        node.break_keyword = self.transform_terminal_node(node.break_keyword);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_return_statement(
        self: &mut Self,
        mut node: Box<ReturnStatement>,
    ) -> Box<ReturnStatement> {
        node.return_keyword = self.transform_terminal_node(node.return_keyword);
        node.expression = node
            .expression
            .map(|value| self.transform_expression(value));
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_emit_statement(
        self: &mut Self,
        mut node: Box<EmitStatement>,
    ) -> Box<EmitStatement> {
        node.emit_keyword = self.transform_terminal_node(node.emit_keyword);
        node.event = self.transform_identifier_path(node.event);
        node.arguments = self.transform_arguments_declaration(node.arguments);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_try_statement(self: &mut Self, mut node: Box<TryStatement>) -> Box<TryStatement> {
        node.try_keyword = self.transform_terminal_node(node.try_keyword);
        node.expression = self.transform_expression(node.expression);
        node.returns = node
            .returns
            .map(|value| self.transform_returns_declaration(value));
        node.body = self.transform_block(node.body);
        node.catch_clauses = self.transform_catch_clauses(node.catch_clauses);
        node
    }

    #[allow(unused_mut)]
    fn transform_catch_clause(self: &mut Self, mut node: Box<CatchClause>) -> Box<CatchClause> {
        node.catch_keyword = self.transform_terminal_node(node.catch_keyword);
        node.error = node
            .error
            .map(|value| self.transform_catch_clause_error(value));
        node.body = self.transform_block(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_catch_clause_error(
        self: &mut Self,
        mut node: Box<CatchClauseError>,
    ) -> Box<CatchClauseError> {
        node.name = node.name.map(|value| self.transform_terminal_node(value));
        node.parameters = self.transform_parameters_declaration(node.parameters);
        node
    }

    #[allow(unused_mut)]
    fn transform_revert_statement(
        self: &mut Self,
        mut node: Box<RevertStatement>,
    ) -> Box<RevertStatement> {
        node.revert_keyword = self.transform_terminal_node(node.revert_keyword);
        node.error = node
            .error
            .map(|value| self.transform_identifier_path(value));
        node.arguments = self.transform_arguments_declaration(node.arguments);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_throw_statement(
        self: &mut Self,
        mut node: Box<ThrowStatement>,
    ) -> Box<ThrowStatement> {
        node.throw_keyword = self.transform_terminal_node(node.throw_keyword);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_assignment_expression(
        self: &mut Self,
        mut node: Box<AssignmentExpression>,
    ) -> Box<AssignmentExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_conditional_expression(
        self: &mut Self,
        mut node: Box<ConditionalExpression>,
    ) -> Box<ConditionalExpression> {
        node.operand = self.transform_expression(node.operand);
        node.question_mark = self.transform_terminal_node(node.question_mark);
        node.true_expression = self.transform_expression(node.true_expression);
        node.colon = self.transform_terminal_node(node.colon);
        node.false_expression = self.transform_expression(node.false_expression);
        node
    }

    #[allow(unused_mut)]
    fn transform_or_expression(self: &mut Self, mut node: Box<OrExpression>) -> Box<OrExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_and_expression(
        self: &mut Self,
        mut node: Box<AndExpression>,
    ) -> Box<AndExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_equality_expression(
        self: &mut Self,
        mut node: Box<EqualityExpression>,
    ) -> Box<EqualityExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_comparison_expression(
        self: &mut Self,
        mut node: Box<ComparisonExpression>,
    ) -> Box<ComparisonExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_bitwise_or_expression(
        self: &mut Self,
        mut node: Box<BitwiseOrExpression>,
    ) -> Box<BitwiseOrExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_bitwise_xor_expression(
        self: &mut Self,
        mut node: Box<BitwiseXorExpression>,
    ) -> Box<BitwiseXorExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_bitwise_and_expression(
        self: &mut Self,
        mut node: Box<BitwiseAndExpression>,
    ) -> Box<BitwiseAndExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_shift_expression(
        self: &mut Self,
        mut node: Box<ShiftExpression>,
    ) -> Box<ShiftExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_additive_expression(
        self: &mut Self,
        mut node: Box<AdditiveExpression>,
    ) -> Box<AdditiveExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_multiplicative_expression(
        self: &mut Self,
        mut node: Box<MultiplicativeExpression>,
    ) -> Box<MultiplicativeExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_exponentiation_expression(
        self: &mut Self,
        mut node: Box<ExponentiationExpression>,
    ) -> Box<ExponentiationExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_postfix_expression(
        self: &mut Self,
        mut node: Box<PostfixExpression>,
    ) -> Box<PostfixExpression> {
        node.operand = self.transform_expression(node.operand);
        node.operator = self.transform_terminal_node(node.operator);
        node
    }

    #[allow(unused_mut)]
    fn transform_prefix_expression(
        self: &mut Self,
        mut node: Box<PrefixExpression>,
    ) -> Box<PrefixExpression> {
        node.operator = self.transform_terminal_node(node.operator);
        node.operand = self.transform_expression(node.operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_function_call_expression(
        self: &mut Self,
        mut node: Box<FunctionCallExpression>,
    ) -> Box<FunctionCallExpression> {
        node.operand = self.transform_expression(node.operand);
        node.arguments = self.transform_arguments_declaration(node.arguments);
        node
    }

    #[allow(unused_mut)]
    fn transform_call_options_expression(
        self: &mut Self,
        mut node: Box<CallOptionsExpression>,
    ) -> Box<CallOptionsExpression> {
        node.operand = self.transform_expression(node.operand);
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.options = self.transform_call_options(node.options);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_member_access_expression(
        self: &mut Self,
        mut node: Box<MemberAccessExpression>,
    ) -> Box<MemberAccessExpression> {
        node.operand = self.transform_expression(node.operand);
        node.period = self.transform_terminal_node(node.period);
        node.member = self.transform_terminal_node(node.member);
        node
    }

    #[allow(unused_mut)]
    fn transform_index_access_expression(
        self: &mut Self,
        mut node: Box<IndexAccessExpression>,
    ) -> Box<IndexAccessExpression> {
        node.operand = self.transform_expression(node.operand);
        node.open_bracket = self.transform_terminal_node(node.open_bracket);
        node.start = node.start.map(|value| self.transform_expression(value));
        node.end = node.end.map(|value| self.transform_index_access_end(value));
        node.close_bracket = self.transform_terminal_node(node.close_bracket);
        node
    }

    #[allow(unused_mut)]
    fn transform_index_access_end(
        self: &mut Self,
        mut node: Box<IndexAccessEnd>,
    ) -> Box<IndexAccessEnd> {
        node.colon = self.transform_terminal_node(node.colon);
        node.end = node.end.map(|value| self.transform_expression(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_positional_arguments_declaration(
        self: &mut Self,
        mut node: Box<PositionalArgumentsDeclaration>,
    ) -> Box<PositionalArgumentsDeclaration> {
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.arguments = self.transform_positional_arguments(node.arguments);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_named_arguments_declaration(
        self: &mut Self,
        mut node: Box<NamedArgumentsDeclaration>,
    ) -> Box<NamedArgumentsDeclaration> {
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.arguments = node
            .arguments
            .map(|value| self.transform_named_argument_group(value));
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_named_argument_group(
        self: &mut Self,
        mut node: Box<NamedArgumentGroup>,
    ) -> Box<NamedArgumentGroup> {
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.arguments = self.transform_named_arguments(node.arguments);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_named_argument(
        self: &mut Self,
        mut node: Box<NamedArgument>,
    ) -> Box<NamedArgument> {
        node.name = self.transform_terminal_node(node.name);
        node.colon = self.transform_terminal_node(node.colon);
        node.value = self.transform_expression(node.value);
        node
    }

    #[allow(unused_mut)]
    fn transform_type_expression(
        self: &mut Self,
        mut node: Box<TypeExpression>,
    ) -> Box<TypeExpression> {
        node.type_keyword = self.transform_terminal_node(node.type_keyword);
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.type_name = self.transform_type_name(node.type_name);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_new_expression(
        self: &mut Self,
        mut node: Box<NewExpression>,
    ) -> Box<NewExpression> {
        node.new_keyword = self.transform_terminal_node(node.new_keyword);
        node.type_name = self.transform_type_name(node.type_name);
        node
    }

    #[allow(unused_mut)]
    fn transform_tuple_expression(
        self: &mut Self,
        mut node: Box<TupleExpression>,
    ) -> Box<TupleExpression> {
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.items = self.transform_tuple_values(node.items);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_tuple_value(self: &mut Self, mut node: Box<TupleValue>) -> Box<TupleValue> {
        node.expression = node
            .expression
            .map(|value| self.transform_expression(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_array_expression(
        self: &mut Self,
        mut node: Box<ArrayExpression>,
    ) -> Box<ArrayExpression> {
        node.open_bracket = self.transform_terminal_node(node.open_bracket);
        node.items = self.transform_array_values(node.items);
        node.close_bracket = self.transform_terminal_node(node.close_bracket);
        node
    }

    #[allow(unused_mut)]
    fn transform_hex_number_expression(
        self: &mut Self,
        mut node: Box<HexNumberExpression>,
    ) -> Box<HexNumberExpression> {
        node.literal = self.transform_terminal_node(node.literal);
        node.unit = node.unit.map(|value| self.transform_number_unit(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_decimal_number_expression(
        self: &mut Self,
        mut node: Box<DecimalNumberExpression>,
    ) -> Box<DecimalNumberExpression> {
        node.literal = self.transform_terminal_node(node.literal);
        node.unit = node.unit.map(|value| self.transform_number_unit(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_block(self: &mut Self, mut node: Box<YulBlock>) -> Box<YulBlock> {
        node.open_brace = self.transform_terminal_node(node.open_brace);
        node.statements = self.transform_yul_statements(node.statements);
        node.close_brace = self.transform_terminal_node(node.close_brace);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_function_definition(
        self: &mut Self,
        mut node: Box<YulFunctionDefinition>,
    ) -> Box<YulFunctionDefinition> {
        node.function_keyword = self.transform_terminal_node(node.function_keyword);
        node.name = self.transform_terminal_node(node.name);
        node.parameters = self.transform_yul_parameters_declaration(node.parameters);
        node.returns = node
            .returns
            .map(|value| self.transform_yul_returns_declaration(value));
        node.body = self.transform_yul_block(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_parameters_declaration(
        self: &mut Self,
        mut node: Box<YulParametersDeclaration>,
    ) -> Box<YulParametersDeclaration> {
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.parameters = self.transform_yul_parameters(node.parameters);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_returns_declaration(
        self: &mut Self,
        mut node: Box<YulReturnsDeclaration>,
    ) -> Box<YulReturnsDeclaration> {
        node.minus_greater_than = self.transform_terminal_node(node.minus_greater_than);
        node.variables = self.transform_yul_variable_names(node.variables);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_variable_declaration_statement(
        self: &mut Self,
        mut node: Box<YulVariableDeclarationStatement>,
    ) -> Box<YulVariableDeclarationStatement> {
        node.let_keyword = self.transform_terminal_node(node.let_keyword);
        node.variables = self.transform_yul_variable_names(node.variables);
        node.value = node
            .value
            .map(|value| self.transform_yul_variable_declaration_value(value));
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_variable_declaration_value(
        self: &mut Self,
        mut node: Box<YulVariableDeclarationValue>,
    ) -> Box<YulVariableDeclarationValue> {
        node.assignment = self.transform_yul_assignment_operator(node.assignment);
        node.expression = self.transform_yul_expression(node.expression);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_variable_assignment_statement(
        self: &mut Self,
        mut node: Box<YulVariableAssignmentStatement>,
    ) -> Box<YulVariableAssignmentStatement> {
        node.variables = self.transform_yul_paths(node.variables);
        node.assignment = self.transform_yul_assignment_operator(node.assignment);
        node.expression = self.transform_yul_expression(node.expression);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_colon_and_equal(
        self: &mut Self,
        mut node: Box<YulColonAndEqual>,
    ) -> Box<YulColonAndEqual> {
        node.colon = self.transform_terminal_node(node.colon);
        node.equal = self.transform_terminal_node(node.equal);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_stack_assignment_statement(
        self: &mut Self,
        mut node: Box<YulStackAssignmentStatement>,
    ) -> Box<YulStackAssignmentStatement> {
        node.assignment = self.transform_yul_stack_assignment_operator(node.assignment);
        node.variable = self.transform_terminal_node(node.variable);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_equal_and_colon(
        self: &mut Self,
        mut node: Box<YulEqualAndColon>,
    ) -> Box<YulEqualAndColon> {
        node.equal = self.transform_terminal_node(node.equal);
        node.colon = self.transform_terminal_node(node.colon);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_if_statement(
        self: &mut Self,
        mut node: Box<YulIfStatement>,
    ) -> Box<YulIfStatement> {
        node.if_keyword = self.transform_terminal_node(node.if_keyword);
        node.condition = self.transform_yul_expression(node.condition);
        node.body = self.transform_yul_block(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_for_statement(
        self: &mut Self,
        mut node: Box<YulForStatement>,
    ) -> Box<YulForStatement> {
        node.for_keyword = self.transform_terminal_node(node.for_keyword);
        node.initialization = self.transform_yul_block(node.initialization);
        node.condition = self.transform_yul_expression(node.condition);
        node.iterator = self.transform_yul_block(node.iterator);
        node.body = self.transform_yul_block(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_switch_statement(
        self: &mut Self,
        mut node: Box<YulSwitchStatement>,
    ) -> Box<YulSwitchStatement> {
        node.switch_keyword = self.transform_terminal_node(node.switch_keyword);
        node.expression = self.transform_yul_expression(node.expression);
        node.cases = self.transform_yul_switch_cases(node.cases);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_default_case(
        self: &mut Self,
        mut node: Box<YulDefaultCase>,
    ) -> Box<YulDefaultCase> {
        node.default_keyword = self.transform_terminal_node(node.default_keyword);
        node.body = self.transform_yul_block(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_value_case(self: &mut Self, mut node: Box<YulValueCase>) -> Box<YulValueCase> {
        node.case_keyword = self.transform_terminal_node(node.case_keyword);
        node.value = self.transform_yul_literal(node.value);
        node.body = self.transform_yul_block(node.body);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_leave_statement(
        self: &mut Self,
        mut node: Box<YulLeaveStatement>,
    ) -> Box<YulLeaveStatement> {
        node.leave_keyword = self.transform_terminal_node(node.leave_keyword);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_break_statement(
        self: &mut Self,
        mut node: Box<YulBreakStatement>,
    ) -> Box<YulBreakStatement> {
        node.break_keyword = self.transform_terminal_node(node.break_keyword);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_continue_statement(
        self: &mut Self,
        mut node: Box<YulContinueStatement>,
    ) -> Box<YulContinueStatement> {
        node.continue_keyword = self.transform_terminal_node(node.continue_keyword);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_label(self: &mut Self, mut node: Box<YulLabel>) -> Box<YulLabel> {
        node.label = self.transform_terminal_node(node.label);
        node.colon = self.transform_terminal_node(node.colon);
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_function_call_expression(
        self: &mut Self,
        mut node: Box<YulFunctionCallExpression>,
    ) -> Box<YulFunctionCallExpression> {
        node.operand = self.transform_yul_expression(node.operand);
        node.open_paren = self.transform_terminal_node(node.open_paren);
        node.arguments = self.transform_yul_arguments(node.arguments);
        node.close_paren = self.transform_terminal_node(node.close_paren);
        node
    }

    #[allow(unused_mut)]
    fn transform_source_unit_member(
        self: &mut Self,
        mut node: SourceUnitMember,
    ) -> SourceUnitMember {
        match node {
            SourceUnitMember::PragmaDirective(value) => {
                SourceUnitMember::PragmaDirective(self.transform_pragma_directive(value))
            }
            SourceUnitMember::ImportDirective(value) => {
                SourceUnitMember::ImportDirective(self.transform_import_directive(value))
            }
            SourceUnitMember::ContractDefinition(value) => {
                SourceUnitMember::ContractDefinition(self.transform_contract_definition(value))
            }
            SourceUnitMember::InterfaceDefinition(value) => {
                SourceUnitMember::InterfaceDefinition(self.transform_interface_definition(value))
            }
            SourceUnitMember::LibraryDefinition(value) => {
                SourceUnitMember::LibraryDefinition(self.transform_library_definition(value))
            }
            SourceUnitMember::StructDefinition(value) => {
                SourceUnitMember::StructDefinition(self.transform_struct_definition(value))
            }
            SourceUnitMember::EnumDefinition(value) => {
                SourceUnitMember::EnumDefinition(self.transform_enum_definition(value))
            }
            SourceUnitMember::FunctionDefinition(value) => {
                SourceUnitMember::FunctionDefinition(self.transform_function_definition(value))
            }
            SourceUnitMember::ErrorDefinition(value) => {
                SourceUnitMember::ErrorDefinition(self.transform_error_definition(value))
            }
            SourceUnitMember::UserDefinedValueTypeDefinition(value) => {
                SourceUnitMember::UserDefinedValueTypeDefinition(
                    self.transform_user_defined_value_type_definition(value),
                )
            }
            SourceUnitMember::UsingDirective(value) => {
                SourceUnitMember::UsingDirective(self.transform_using_directive(value))
            }
            SourceUnitMember::EventDefinition(value) => {
                SourceUnitMember::EventDefinition(self.transform_event_definition(value))
            }
            SourceUnitMember::ConstantDefinition(value) => {
                SourceUnitMember::ConstantDefinition(self.transform_constant_definition(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_pragma(self: &mut Self, mut node: Pragma) -> Pragma {
        match node {
            Pragma::AbicoderPragma(value) => {
                Pragma::AbicoderPragma(self.transform_abicoder_pragma(value))
            }
            Pragma::ExperimentalPragma(value) => {
                Pragma::ExperimentalPragma(self.transform_experimental_pragma(value))
            }
            Pragma::VersionPragma(value) => {
                Pragma::VersionPragma(self.transform_version_pragma(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_experimental_feature(
        self: &mut Self,
        mut node: ExperimentalFeature,
    ) -> ExperimentalFeature {
        match node {
            ExperimentalFeature::StringLiteral(value) => {
                ExperimentalFeature::StringLiteral(self.transform_string_literal(value))
            }
            ExperimentalFeature::TerminalNode(value) => {
                ExperimentalFeature::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_version_expression(
        self: &mut Self,
        mut node: VersionExpression,
    ) -> VersionExpression {
        match node {
            VersionExpression::VersionRange(value) => {
                VersionExpression::VersionRange(self.transform_version_range(value))
            }
            VersionExpression::VersionTerm(value) => {
                VersionExpression::VersionTerm(self.transform_version_term(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_version_operator(self: &mut Self, mut node: VersionOperator) -> VersionOperator {
        match node {
            VersionOperator::TerminalNode(value) => {
                VersionOperator::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_version_literal(self: &mut Self, mut node: VersionLiteral) -> VersionLiteral {
        match node {
            VersionLiteral::SimpleVersionLiteral(value) => {
                VersionLiteral::SimpleVersionLiteral(self.transform_simple_version_literal(value))
            }
            VersionLiteral::TerminalNode(value) => {
                VersionLiteral::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_import_clause(self: &mut Self, mut node: ImportClause) -> ImportClause {
        match node {
            ImportClause::PathImport(value) => {
                ImportClause::PathImport(self.transform_path_import(value))
            }
            ImportClause::NamedImport(value) => {
                ImportClause::NamedImport(self.transform_named_import(value))
            }
            ImportClause::ImportDeconstruction(value) => {
                ImportClause::ImportDeconstruction(self.transform_import_deconstruction(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_using_clause(self: &mut Self, mut node: UsingClause) -> UsingClause {
        match node {
            UsingClause::IdentifierPath(value) => {
                UsingClause::IdentifierPath(self.transform_identifier_path(value))
            }
            UsingClause::UsingDeconstruction(value) => {
                UsingClause::UsingDeconstruction(self.transform_using_deconstruction(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_using_operator(self: &mut Self, mut node: UsingOperator) -> UsingOperator {
        match node {
            UsingOperator::TerminalNode(value) => {
                UsingOperator::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_using_target(self: &mut Self, mut node: UsingTarget) -> UsingTarget {
        match node {
            UsingTarget::TypeName(value) => UsingTarget::TypeName(self.transform_type_name(value)),
            UsingTarget::TerminalNode(value) => {
                UsingTarget::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_contract_member(self: &mut Self, mut node: ContractMember) -> ContractMember {
        match node {
            ContractMember::UsingDirective(value) => {
                ContractMember::UsingDirective(self.transform_using_directive(value))
            }
            ContractMember::FunctionDefinition(value) => {
                ContractMember::FunctionDefinition(self.transform_function_definition(value))
            }
            ContractMember::ConstructorDefinition(value) => {
                ContractMember::ConstructorDefinition(self.transform_constructor_definition(value))
            }
            ContractMember::ReceiveFunctionDefinition(value) => {
                ContractMember::ReceiveFunctionDefinition(
                    self.transform_receive_function_definition(value),
                )
            }
            ContractMember::FallbackFunctionDefinition(value) => {
                ContractMember::FallbackFunctionDefinition(
                    self.transform_fallback_function_definition(value),
                )
            }
            ContractMember::UnnamedFunctionDefinition(value) => {
                ContractMember::UnnamedFunctionDefinition(
                    self.transform_unnamed_function_definition(value),
                )
            }
            ContractMember::ModifierDefinition(value) => {
                ContractMember::ModifierDefinition(self.transform_modifier_definition(value))
            }
            ContractMember::StructDefinition(value) => {
                ContractMember::StructDefinition(self.transform_struct_definition(value))
            }
            ContractMember::EnumDefinition(value) => {
                ContractMember::EnumDefinition(self.transform_enum_definition(value))
            }
            ContractMember::EventDefinition(value) => {
                ContractMember::EventDefinition(self.transform_event_definition(value))
            }
            ContractMember::ErrorDefinition(value) => {
                ContractMember::ErrorDefinition(self.transform_error_definition(value))
            }
            ContractMember::UserDefinedValueTypeDefinition(value) => {
                ContractMember::UserDefinedValueTypeDefinition(
                    self.transform_user_defined_value_type_definition(value),
                )
            }
            ContractMember::StateVariableDefinition(value) => {
                ContractMember::StateVariableDefinition(
                    self.transform_state_variable_definition(value),
                )
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_state_variable_attribute(
        self: &mut Self,
        mut node: StateVariableAttribute,
    ) -> StateVariableAttribute {
        match node {
            StateVariableAttribute::OverrideSpecifier(value) => {
                StateVariableAttribute::OverrideSpecifier(self.transform_override_specifier(value))
            }
            StateVariableAttribute::TerminalNode(value) => {
                StateVariableAttribute::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_function_name(self: &mut Self, mut node: FunctionName) -> FunctionName {
        match node {
            FunctionName::TerminalNode(value) => {
                FunctionName::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_function_attribute(
        self: &mut Self,
        mut node: FunctionAttribute,
    ) -> FunctionAttribute {
        match node {
            FunctionAttribute::ModifierInvocation(value) => {
                FunctionAttribute::ModifierInvocation(self.transform_modifier_invocation(value))
            }
            FunctionAttribute::OverrideSpecifier(value) => {
                FunctionAttribute::OverrideSpecifier(self.transform_override_specifier(value))
            }
            FunctionAttribute::TerminalNode(value) => {
                FunctionAttribute::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_function_body(self: &mut Self, mut node: FunctionBody) -> FunctionBody {
        match node {
            FunctionBody::Block(value) => FunctionBody::Block(self.transform_block(value)),
            FunctionBody::TerminalNode(value) => {
                FunctionBody::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_constructor_attribute(
        self: &mut Self,
        mut node: ConstructorAttribute,
    ) -> ConstructorAttribute {
        match node {
            ConstructorAttribute::ModifierInvocation(value) => {
                ConstructorAttribute::ModifierInvocation(self.transform_modifier_invocation(value))
            }
            ConstructorAttribute::TerminalNode(value) => {
                ConstructorAttribute::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_unnamed_function_attribute(
        self: &mut Self,
        mut node: UnnamedFunctionAttribute,
    ) -> UnnamedFunctionAttribute {
        match node {
            UnnamedFunctionAttribute::ModifierInvocation(value) => {
                UnnamedFunctionAttribute::ModifierInvocation(
                    self.transform_modifier_invocation(value),
                )
            }
            UnnamedFunctionAttribute::TerminalNode(value) => {
                UnnamedFunctionAttribute::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_fallback_function_attribute(
        self: &mut Self,
        mut node: FallbackFunctionAttribute,
    ) -> FallbackFunctionAttribute {
        match node {
            FallbackFunctionAttribute::ModifierInvocation(value) => {
                FallbackFunctionAttribute::ModifierInvocation(
                    self.transform_modifier_invocation(value),
                )
            }
            FallbackFunctionAttribute::OverrideSpecifier(value) => {
                FallbackFunctionAttribute::OverrideSpecifier(
                    self.transform_override_specifier(value),
                )
            }
            FallbackFunctionAttribute::TerminalNode(value) => {
                FallbackFunctionAttribute::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_receive_function_attribute(
        self: &mut Self,
        mut node: ReceiveFunctionAttribute,
    ) -> ReceiveFunctionAttribute {
        match node {
            ReceiveFunctionAttribute::ModifierInvocation(value) => {
                ReceiveFunctionAttribute::ModifierInvocation(
                    self.transform_modifier_invocation(value),
                )
            }
            ReceiveFunctionAttribute::OverrideSpecifier(value) => {
                ReceiveFunctionAttribute::OverrideSpecifier(
                    self.transform_override_specifier(value),
                )
            }
            ReceiveFunctionAttribute::TerminalNode(value) => {
                ReceiveFunctionAttribute::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_modifier_attribute(
        self: &mut Self,
        mut node: ModifierAttribute,
    ) -> ModifierAttribute {
        match node {
            ModifierAttribute::OverrideSpecifier(value) => {
                ModifierAttribute::OverrideSpecifier(self.transform_override_specifier(value))
            }
            ModifierAttribute::TerminalNode(value) => {
                ModifierAttribute::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_type_name(self: &mut Self, mut node: TypeName) -> TypeName {
        match node {
            TypeName::ArrayTypeName(value) => {
                TypeName::ArrayTypeName(self.transform_array_type_name(value))
            }
            TypeName::FunctionType(value) => {
                TypeName::FunctionType(self.transform_function_type(value))
            }
            TypeName::MappingType(value) => {
                TypeName::MappingType(self.transform_mapping_type(value))
            }
            TypeName::ElementaryType(value) => {
                TypeName::ElementaryType(self.transform_elementary_type(value))
            }
            TypeName::IdentifierPath(value) => {
                TypeName::IdentifierPath(self.transform_identifier_path(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_function_type_attribute(
        self: &mut Self,
        mut node: FunctionTypeAttribute,
    ) -> FunctionTypeAttribute {
        match node {
            FunctionTypeAttribute::TerminalNode(value) => {
                FunctionTypeAttribute::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_mapping_key_type(self: &mut Self, mut node: MappingKeyType) -> MappingKeyType {
        match node {
            MappingKeyType::ElementaryType(value) => {
                MappingKeyType::ElementaryType(self.transform_elementary_type(value))
            }
            MappingKeyType::IdentifierPath(value) => {
                MappingKeyType::IdentifierPath(self.transform_identifier_path(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_elementary_type(self: &mut Self, mut node: ElementaryType) -> ElementaryType {
        match node {
            ElementaryType::AddressType(value) => {
                ElementaryType::AddressType(self.transform_address_type(value))
            }
            ElementaryType::TerminalNode(value) => {
                ElementaryType::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_statement(self: &mut Self, mut node: Statement) -> Statement {
        match node {
            Statement::IfStatement(value) => {
                Statement::IfStatement(self.transform_if_statement(value))
            }
            Statement::ForStatement(value) => {
                Statement::ForStatement(self.transform_for_statement(value))
            }
            Statement::WhileStatement(value) => {
                Statement::WhileStatement(self.transform_while_statement(value))
            }
            Statement::DoWhileStatement(value) => {
                Statement::DoWhileStatement(self.transform_do_while_statement(value))
            }
            Statement::ContinueStatement(value) => {
                Statement::ContinueStatement(self.transform_continue_statement(value))
            }
            Statement::BreakStatement(value) => {
                Statement::BreakStatement(self.transform_break_statement(value))
            }
            Statement::ReturnStatement(value) => {
                Statement::ReturnStatement(self.transform_return_statement(value))
            }
            Statement::ThrowStatement(value) => {
                Statement::ThrowStatement(self.transform_throw_statement(value))
            }
            Statement::EmitStatement(value) => {
                Statement::EmitStatement(self.transform_emit_statement(value))
            }
            Statement::TryStatement(value) => {
                Statement::TryStatement(self.transform_try_statement(value))
            }
            Statement::RevertStatement(value) => {
                Statement::RevertStatement(self.transform_revert_statement(value))
            }
            Statement::AssemblyStatement(value) => {
                Statement::AssemblyStatement(self.transform_assembly_statement(value))
            }
            Statement::Block(value) => Statement::Block(self.transform_block(value)),
            Statement::UncheckedBlock(value) => {
                Statement::UncheckedBlock(self.transform_unchecked_block(value))
            }
            Statement::TupleDeconstructionStatement(value) => {
                Statement::TupleDeconstructionStatement(
                    self.transform_tuple_deconstruction_statement(value),
                )
            }
            Statement::VariableDeclarationStatement(value) => {
                Statement::VariableDeclarationStatement(
                    self.transform_variable_declaration_statement(value),
                )
            }
            Statement::ExpressionStatement(value) => {
                Statement::ExpressionStatement(self.transform_expression_statement(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_tuple_member(self: &mut Self, mut node: TupleMember) -> TupleMember {
        match node {
            TupleMember::TypedTupleMember(value) => {
                TupleMember::TypedTupleMember(self.transform_typed_tuple_member(value))
            }
            TupleMember::UntypedTupleMember(value) => {
                TupleMember::UntypedTupleMember(self.transform_untyped_tuple_member(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_variable_declaration_type(
        self: &mut Self,
        mut node: VariableDeclarationType,
    ) -> VariableDeclarationType {
        match node {
            VariableDeclarationType::TypeName(value) => {
                VariableDeclarationType::TypeName(self.transform_type_name(value))
            }
            VariableDeclarationType::TerminalNode(value) => {
                VariableDeclarationType::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_storage_location(self: &mut Self, mut node: StorageLocation) -> StorageLocation {
        match node {
            StorageLocation::TerminalNode(value) => {
                StorageLocation::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_for_statement_initialization(
        self: &mut Self,
        mut node: ForStatementInitialization,
    ) -> ForStatementInitialization {
        match node {
            ForStatementInitialization::TupleDeconstructionStatement(value) => {
                ForStatementInitialization::TupleDeconstructionStatement(
                    self.transform_tuple_deconstruction_statement(value),
                )
            }
            ForStatementInitialization::VariableDeclarationStatement(value) => {
                ForStatementInitialization::VariableDeclarationStatement(
                    self.transform_variable_declaration_statement(value),
                )
            }
            ForStatementInitialization::ExpressionStatement(value) => {
                ForStatementInitialization::ExpressionStatement(
                    self.transform_expression_statement(value),
                )
            }
            ForStatementInitialization::TerminalNode(value) => {
                ForStatementInitialization::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_for_statement_condition(
        self: &mut Self,
        mut node: ForStatementCondition,
    ) -> ForStatementCondition {
        match node {
            ForStatementCondition::ExpressionStatement(value) => {
                ForStatementCondition::ExpressionStatement(
                    self.transform_expression_statement(value),
                )
            }
            ForStatementCondition::TerminalNode(value) => {
                ForStatementCondition::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_expression(self: &mut Self, mut node: Expression) -> Expression {
        match node {
            Expression::AssignmentExpression(value) => {
                Expression::AssignmentExpression(self.transform_assignment_expression(value))
            }
            Expression::ConditionalExpression(value) => {
                Expression::ConditionalExpression(self.transform_conditional_expression(value))
            }
            Expression::OrExpression(value) => {
                Expression::OrExpression(self.transform_or_expression(value))
            }
            Expression::AndExpression(value) => {
                Expression::AndExpression(self.transform_and_expression(value))
            }
            Expression::EqualityExpression(value) => {
                Expression::EqualityExpression(self.transform_equality_expression(value))
            }
            Expression::ComparisonExpression(value) => {
                Expression::ComparisonExpression(self.transform_comparison_expression(value))
            }
            Expression::BitwiseOrExpression(value) => {
                Expression::BitwiseOrExpression(self.transform_bitwise_or_expression(value))
            }
            Expression::BitwiseXorExpression(value) => {
                Expression::BitwiseXorExpression(self.transform_bitwise_xor_expression(value))
            }
            Expression::BitwiseAndExpression(value) => {
                Expression::BitwiseAndExpression(self.transform_bitwise_and_expression(value))
            }
            Expression::ShiftExpression(value) => {
                Expression::ShiftExpression(self.transform_shift_expression(value))
            }
            Expression::AdditiveExpression(value) => {
                Expression::AdditiveExpression(self.transform_additive_expression(value))
            }
            Expression::MultiplicativeExpression(value) => Expression::MultiplicativeExpression(
                self.transform_multiplicative_expression(value),
            ),
            Expression::ExponentiationExpression(value) => Expression::ExponentiationExpression(
                self.transform_exponentiation_expression(value),
            ),
            Expression::PostfixExpression(value) => {
                Expression::PostfixExpression(self.transform_postfix_expression(value))
            }
            Expression::PrefixExpression(value) => {
                Expression::PrefixExpression(self.transform_prefix_expression(value))
            }
            Expression::FunctionCallExpression(value) => {
                Expression::FunctionCallExpression(self.transform_function_call_expression(value))
            }
            Expression::CallOptionsExpression(value) => {
                Expression::CallOptionsExpression(self.transform_call_options_expression(value))
            }
            Expression::MemberAccessExpression(value) => {
                Expression::MemberAccessExpression(self.transform_member_access_expression(value))
            }
            Expression::IndexAccessExpression(value) => {
                Expression::IndexAccessExpression(self.transform_index_access_expression(value))
            }
            Expression::NewExpression(value) => {
                Expression::NewExpression(self.transform_new_expression(value))
            }
            Expression::TupleExpression(value) => {
                Expression::TupleExpression(self.transform_tuple_expression(value))
            }
            Expression::TypeExpression(value) => {
                Expression::TypeExpression(self.transform_type_expression(value))
            }
            Expression::ArrayExpression(value) => {
                Expression::ArrayExpression(self.transform_array_expression(value))
            }
            Expression::HexNumberExpression(value) => {
                Expression::HexNumberExpression(self.transform_hex_number_expression(value))
            }
            Expression::DecimalNumberExpression(value) => {
                Expression::DecimalNumberExpression(self.transform_decimal_number_expression(value))
            }
            Expression::StringExpression(value) => {
                Expression::StringExpression(self.transform_string_expression(value))
            }
            Expression::ElementaryType(value) => {
                Expression::ElementaryType(self.transform_elementary_type(value))
            }
            Expression::TerminalNode(value) => {
                Expression::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_arguments_declaration(
        self: &mut Self,
        mut node: ArgumentsDeclaration,
    ) -> ArgumentsDeclaration {
        match node {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(value) => {
                ArgumentsDeclaration::PositionalArgumentsDeclaration(
                    self.transform_positional_arguments_declaration(value),
                )
            }
            ArgumentsDeclaration::NamedArgumentsDeclaration(value) => {
                ArgumentsDeclaration::NamedArgumentsDeclaration(
                    self.transform_named_arguments_declaration(value),
                )
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_number_unit(self: &mut Self, mut node: NumberUnit) -> NumberUnit {
        match node {
            NumberUnit::TerminalNode(value) => {
                NumberUnit::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_string_expression(
        self: &mut Self,
        mut node: StringExpression,
    ) -> StringExpression {
        match node {
            StringExpression::StringLiteral(value) => {
                StringExpression::StringLiteral(self.transform_string_literal(value))
            }
            StringExpression::StringLiterals(value) => {
                StringExpression::StringLiterals(self.transform_string_literals(value))
            }
            StringExpression::HexStringLiteral(value) => {
                StringExpression::HexStringLiteral(self.transform_hex_string_literal(value))
            }
            StringExpression::HexStringLiterals(value) => {
                StringExpression::HexStringLiterals(self.transform_hex_string_literals(value))
            }
            StringExpression::UnicodeStringLiterals(value) => {
                StringExpression::UnicodeStringLiterals(
                    self.transform_unicode_string_literals(value),
                )
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_string_literal(self: &mut Self, mut node: StringLiteral) -> StringLiteral {
        match node {
            StringLiteral::TerminalNode(value) => {
                StringLiteral::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_hex_string_literal(
        self: &mut Self,
        mut node: HexStringLiteral,
    ) -> HexStringLiteral {
        match node {
            HexStringLiteral::TerminalNode(value) => {
                HexStringLiteral::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_unicode_string_literal(
        self: &mut Self,
        mut node: UnicodeStringLiteral,
    ) -> UnicodeStringLiteral {
        match node {
            UnicodeStringLiteral::TerminalNode(value) => {
                UnicodeStringLiteral::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_yul_statement(self: &mut Self, mut node: YulStatement) -> YulStatement {
        match node {
            YulStatement::YulBlock(value) => {
                YulStatement::YulBlock(self.transform_yul_block(value))
            }
            YulStatement::YulFunctionDefinition(value) => {
                YulStatement::YulFunctionDefinition(self.transform_yul_function_definition(value))
            }
            YulStatement::YulStackAssignmentStatement(value) => {
                YulStatement::YulStackAssignmentStatement(
                    self.transform_yul_stack_assignment_statement(value),
                )
            }
            YulStatement::YulIfStatement(value) => {
                YulStatement::YulIfStatement(self.transform_yul_if_statement(value))
            }
            YulStatement::YulForStatement(value) => {
                YulStatement::YulForStatement(self.transform_yul_for_statement(value))
            }
            YulStatement::YulSwitchStatement(value) => {
                YulStatement::YulSwitchStatement(self.transform_yul_switch_statement(value))
            }
            YulStatement::YulLeaveStatement(value) => {
                YulStatement::YulLeaveStatement(self.transform_yul_leave_statement(value))
            }
            YulStatement::YulBreakStatement(value) => {
                YulStatement::YulBreakStatement(self.transform_yul_break_statement(value))
            }
            YulStatement::YulContinueStatement(value) => {
                YulStatement::YulContinueStatement(self.transform_yul_continue_statement(value))
            }
            YulStatement::YulVariableAssignmentStatement(value) => {
                YulStatement::YulVariableAssignmentStatement(
                    self.transform_yul_variable_assignment_statement(value),
                )
            }
            YulStatement::YulLabel(value) => {
                YulStatement::YulLabel(self.transform_yul_label(value))
            }
            YulStatement::YulVariableDeclarationStatement(value) => {
                YulStatement::YulVariableDeclarationStatement(
                    self.transform_yul_variable_declaration_statement(value),
                )
            }
            YulStatement::YulExpression(value) => {
                YulStatement::YulExpression(self.transform_yul_expression(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_yul_assignment_operator(
        self: &mut Self,
        mut node: YulAssignmentOperator,
    ) -> YulAssignmentOperator {
        match node {
            YulAssignmentOperator::YulColonAndEqual(value) => {
                YulAssignmentOperator::YulColonAndEqual(self.transform_yul_colon_and_equal(value))
            }
            YulAssignmentOperator::TerminalNode(value) => {
                YulAssignmentOperator::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_yul_stack_assignment_operator(
        self: &mut Self,
        mut node: YulStackAssignmentOperator,
    ) -> YulStackAssignmentOperator {
        match node {
            YulStackAssignmentOperator::YulEqualAndColon(value) => {
                YulStackAssignmentOperator::YulEqualAndColon(
                    self.transform_yul_equal_and_colon(value),
                )
            }
            YulStackAssignmentOperator::TerminalNode(value) => {
                YulStackAssignmentOperator::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_yul_switch_case(self: &mut Self, mut node: YulSwitchCase) -> YulSwitchCase {
        match node {
            YulSwitchCase::YulDefaultCase(value) => {
                YulSwitchCase::YulDefaultCase(self.transform_yul_default_case(value))
            }
            YulSwitchCase::YulValueCase(value) => {
                YulSwitchCase::YulValueCase(self.transform_yul_value_case(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_yul_expression(self: &mut Self, mut node: YulExpression) -> YulExpression {
        match node {
            YulExpression::YulFunctionCallExpression(value) => {
                YulExpression::YulFunctionCallExpression(
                    self.transform_yul_function_call_expression(value),
                )
            }
            YulExpression::YulLiteral(value) => {
                YulExpression::YulLiteral(self.transform_yul_literal(value))
            }
            YulExpression::YulBuiltInFunction(value) => {
                YulExpression::YulBuiltInFunction(self.transform_yul_built_in_function(value))
            }
            YulExpression::YulPath(value) => YulExpression::YulPath(self.transform_yul_path(value)),
        }
    }

    #[allow(unused_mut)]
    fn transform_yul_built_in_function(
        self: &mut Self,
        mut node: YulBuiltInFunction,
    ) -> YulBuiltInFunction {
        match node {
            YulBuiltInFunction::TerminalNode(value) => {
                YulBuiltInFunction::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_yul_literal(self: &mut Self, mut node: YulLiteral) -> YulLiteral {
        match node {
            YulLiteral::HexStringLiteral(value) => {
                YulLiteral::HexStringLiteral(self.transform_hex_string_literal(value))
            }
            YulLiteral::StringLiteral(value) => {
                YulLiteral::StringLiteral(self.transform_string_literal(value))
            }
            YulLiteral::TerminalNode(value) => {
                YulLiteral::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_source_unit_members(
        self: &mut Self,
        mut node: Box<SourceUnitMembers>,
    ) -> Box<SourceUnitMembers> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_source_unit_member(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_version_expression_set(
        self: &mut Self,
        mut node: Box<VersionExpressionSet>,
    ) -> Box<VersionExpressionSet> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_version_expression(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_contract_members(
        self: &mut Self,
        mut node: Box<ContractMembers>,
    ) -> Box<ContractMembers> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_contract_member(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_interface_members(
        self: &mut Self,
        mut node: Box<InterfaceMembers>,
    ) -> Box<InterfaceMembers> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_contract_member(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_library_members(
        self: &mut Self,
        mut node: Box<LibraryMembers>,
    ) -> Box<LibraryMembers> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_contract_member(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_struct_members(
        self: &mut Self,
        mut node: Box<StructMembers>,
    ) -> Box<StructMembers> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_struct_member(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_state_variable_attributes(
        self: &mut Self,
        mut node: Box<StateVariableAttributes>,
    ) -> Box<StateVariableAttributes> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_state_variable_attribute(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_function_attributes(
        self: &mut Self,
        mut node: Box<FunctionAttributes>,
    ) -> Box<FunctionAttributes> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_function_attribute(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_constructor_attributes(
        self: &mut Self,
        mut node: Box<ConstructorAttributes>,
    ) -> Box<ConstructorAttributes> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_constructor_attribute(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_unnamed_function_attributes(
        self: &mut Self,
        mut node: Box<UnnamedFunctionAttributes>,
    ) -> Box<UnnamedFunctionAttributes> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_unnamed_function_attribute(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_fallback_function_attributes(
        self: &mut Self,
        mut node: Box<FallbackFunctionAttributes>,
    ) -> Box<FallbackFunctionAttributes> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_fallback_function_attribute(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_receive_function_attributes(
        self: &mut Self,
        mut node: Box<ReceiveFunctionAttributes>,
    ) -> Box<ReceiveFunctionAttributes> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_receive_function_attribute(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_modifier_attributes(
        self: &mut Self,
        mut node: Box<ModifierAttributes>,
    ) -> Box<ModifierAttributes> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_modifier_attribute(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_function_type_attributes(
        self: &mut Self,
        mut node: Box<FunctionTypeAttributes>,
    ) -> Box<FunctionTypeAttributes> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_function_type_attribute(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_statements(self: &mut Self, mut node: Box<Statements>) -> Box<Statements> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_statement(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_catch_clauses(self: &mut Self, mut node: Box<CatchClauses>) -> Box<CatchClauses> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_catch_clause(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_string_literals(
        self: &mut Self,
        mut node: Box<StringLiterals>,
    ) -> Box<StringLiterals> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_string_literal(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_hex_string_literals(
        self: &mut Self,
        mut node: Box<HexStringLiterals>,
    ) -> Box<HexStringLiterals> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_hex_string_literal(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_unicode_string_literals(
        self: &mut Self,
        mut node: Box<UnicodeStringLiterals>,
    ) -> Box<UnicodeStringLiterals> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_unicode_string_literal(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_statements(
        self: &mut Self,
        mut node: Box<YulStatements>,
    ) -> Box<YulStatements> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_yul_statement(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_switch_cases(
        self: &mut Self,
        mut node: Box<YulSwitchCases>,
    ) -> Box<YulSwitchCases> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_yul_switch_case(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_version_expression_sets(
        self: &mut Self,
        mut node: Box<VersionExpressionSets>,
    ) -> Box<VersionExpressionSets> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_version_expression_set(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_simple_version_literal(
        self: &mut Self,
        mut node: Box<SimpleVersionLiteral>,
    ) -> Box<SimpleVersionLiteral> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_terminal_node(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_import_deconstruction_symbols(
        self: &mut Self,
        mut node: Box<ImportDeconstructionSymbols>,
    ) -> Box<ImportDeconstructionSymbols> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_import_deconstruction_symbol(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_using_deconstruction_symbols(
        self: &mut Self,
        mut node: Box<UsingDeconstructionSymbols>,
    ) -> Box<UsingDeconstructionSymbols> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_using_deconstruction_symbol(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_inheritance_types(
        self: &mut Self,
        mut node: Box<InheritanceTypes>,
    ) -> Box<InheritanceTypes> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_inheritance_type(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_enum_members(self: &mut Self, mut node: Box<EnumMembers>) -> Box<EnumMembers> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_terminal_node(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_parameters(self: &mut Self, mut node: Box<Parameters>) -> Box<Parameters> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_parameter(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_override_paths(
        self: &mut Self,
        mut node: Box<OverridePaths>,
    ) -> Box<OverridePaths> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_identifier_path(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_event_parameters(
        self: &mut Self,
        mut node: Box<EventParameters>,
    ) -> Box<EventParameters> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_event_parameter(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_error_parameters(
        self: &mut Self,
        mut node: Box<ErrorParameters>,
    ) -> Box<ErrorParameters> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_error_parameter(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_assembly_flags(
        self: &mut Self,
        mut node: Box<AssemblyFlags>,
    ) -> Box<AssemblyFlags> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_string_literal(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_tuple_deconstruction_elements(
        self: &mut Self,
        mut node: Box<TupleDeconstructionElements>,
    ) -> Box<TupleDeconstructionElements> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_tuple_deconstruction_element(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_positional_arguments(
        self: &mut Self,
        mut node: Box<PositionalArguments>,
    ) -> Box<PositionalArguments> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_expression(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_named_arguments(
        self: &mut Self,
        mut node: Box<NamedArguments>,
    ) -> Box<NamedArguments> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_named_argument(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_call_options(self: &mut Self, mut node: Box<CallOptions>) -> Box<CallOptions> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_named_argument(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_tuple_values(self: &mut Self, mut node: Box<TupleValues>) -> Box<TupleValues> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_tuple_value(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_array_values(self: &mut Self, mut node: Box<ArrayValues>) -> Box<ArrayValues> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_expression(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_identifier_path(
        self: &mut Self,
        mut node: Box<IdentifierPath>,
    ) -> Box<IdentifierPath> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_terminal_node(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_parameters(
        self: &mut Self,
        mut node: Box<YulParameters>,
    ) -> Box<YulParameters> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_terminal_node(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_variable_names(
        self: &mut Self,
        mut node: Box<YulVariableNames>,
    ) -> Box<YulVariableNames> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_terminal_node(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_arguments(self: &mut Self, mut node: Box<YulArguments>) -> Box<YulArguments> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_yul_expression(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_paths(self: &mut Self, mut node: Box<YulPaths>) -> Box<YulPaths> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_yul_path(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_yul_path(self: &mut Self, mut node: Box<YulPath>) -> Box<YulPath> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_terminal_node(key))
            .collect();
        node
    }
}
