// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

#[allow(clippy::wildcard_imports)]
use super::nodes::*;

pub trait Visitor {
    fn enter_abicoder_pragma(&mut self, _node: &AbicoderPragma) -> bool {
        true
    }
    fn leave_abicoder_pragma(&mut self, _node: &AbicoderPragma) {}

    fn enter_additive_expression(&mut self, _node: &AdditiveExpression) -> bool {
        true
    }
    fn leave_additive_expression(&mut self, _node: &AdditiveExpression) {}

    fn enter_address_type(&mut self, _node: &AddressType) -> bool {
        true
    }
    fn leave_address_type(&mut self, _node: &AddressType) {}

    fn enter_and_expression(&mut self, _node: &AndExpression) -> bool {
        true
    }
    fn leave_and_expression(&mut self, _node: &AndExpression) {}

    fn enter_array_expression(&mut self, _node: &ArrayExpression) -> bool {
        true
    }
    fn leave_array_expression(&mut self, _node: &ArrayExpression) {}

    fn enter_array_type_name(&mut self, _node: &ArrayTypeName) -> bool {
        true
    }
    fn leave_array_type_name(&mut self, _node: &ArrayTypeName) {}

    fn enter_assembly_statement(&mut self, _node: &AssemblyStatement) -> bool {
        true
    }
    fn leave_assembly_statement(&mut self, _node: &AssemblyStatement) {}

    fn enter_assignment_expression(&mut self, _node: &AssignmentExpression) -> bool {
        true
    }
    fn leave_assignment_expression(&mut self, _node: &AssignmentExpression) {}

    fn enter_bitwise_and_expression(&mut self, _node: &BitwiseAndExpression) -> bool {
        true
    }
    fn leave_bitwise_and_expression(&mut self, _node: &BitwiseAndExpression) {}

    fn enter_bitwise_or_expression(&mut self, _node: &BitwiseOrExpression) -> bool {
        true
    }
    fn leave_bitwise_or_expression(&mut self, _node: &BitwiseOrExpression) {}

    fn enter_bitwise_xor_expression(&mut self, _node: &BitwiseXorExpression) -> bool {
        true
    }
    fn leave_bitwise_xor_expression(&mut self, _node: &BitwiseXorExpression) {}

    fn enter_block(&mut self, _node: &Block) -> bool {
        true
    }
    fn leave_block(&mut self, _node: &Block) {}

    fn enter_break_statement(&mut self, _node: &BreakStatement) -> bool {
        true
    }
    fn leave_break_statement(&mut self, _node: &BreakStatement) {}

    fn enter_call_options_expression(&mut self, _node: &CallOptionsExpression) -> bool {
        true
    }
    fn leave_call_options_expression(&mut self, _node: &CallOptionsExpression) {}

    fn enter_catch_clause(&mut self, _node: &CatchClause) -> bool {
        true
    }
    fn leave_catch_clause(&mut self, _node: &CatchClause) {}

    fn enter_catch_clause_error(&mut self, _node: &CatchClauseError) -> bool {
        true
    }
    fn leave_catch_clause_error(&mut self, _node: &CatchClauseError) {}

    fn enter_conditional_expression(&mut self, _node: &ConditionalExpression) -> bool {
        true
    }
    fn leave_conditional_expression(&mut self, _node: &ConditionalExpression) {}

    fn enter_constant_definition(&mut self, _node: &ConstantDefinition) -> bool {
        true
    }
    fn leave_constant_definition(&mut self, _node: &ConstantDefinition) {}

    fn enter_continue_statement(&mut self, _node: &ContinueStatement) -> bool {
        true
    }
    fn leave_continue_statement(&mut self, _node: &ContinueStatement) {}

    fn enter_contract_definition(&mut self, _node: &ContractDefinition) -> bool {
        true
    }
    fn leave_contract_definition(&mut self, _node: &ContractDefinition) {}

    fn enter_decimal_number_expression(&mut self, _node: &DecimalNumberExpression) -> bool {
        true
    }
    fn leave_decimal_number_expression(&mut self, _node: &DecimalNumberExpression) {}

    fn enter_do_while_statement(&mut self, _node: &DoWhileStatement) -> bool {
        true
    }
    fn leave_do_while_statement(&mut self, _node: &DoWhileStatement) {}

    fn enter_emit_statement(&mut self, _node: &EmitStatement) -> bool {
        true
    }
    fn leave_emit_statement(&mut self, _node: &EmitStatement) {}

    fn enter_enum_definition(&mut self, _node: &EnumDefinition) -> bool {
        true
    }
    fn leave_enum_definition(&mut self, _node: &EnumDefinition) {}

    fn enter_equality_expression(&mut self, _node: &EqualityExpression) -> bool {
        true
    }
    fn leave_equality_expression(&mut self, _node: &EqualityExpression) {}

    fn enter_error_definition(&mut self, _node: &ErrorDefinition) -> bool {
        true
    }
    fn leave_error_definition(&mut self, _node: &ErrorDefinition) {}

    fn enter_event_definition(&mut self, _node: &EventDefinition) -> bool {
        true
    }
    fn leave_event_definition(&mut self, _node: &EventDefinition) {}

    fn enter_experimental_pragma(&mut self, _node: &ExperimentalPragma) -> bool {
        true
    }
    fn leave_experimental_pragma(&mut self, _node: &ExperimentalPragma) {}

    fn enter_exponentiation_expression(&mut self, _node: &ExponentiationExpression) -> bool {
        true
    }
    fn leave_exponentiation_expression(&mut self, _node: &ExponentiationExpression) {}

    fn enter_expression_statement(&mut self, _node: &ExpressionStatement) -> bool {
        true
    }
    fn leave_expression_statement(&mut self, _node: &ExpressionStatement) {}

    fn enter_for_statement(&mut self, _node: &ForStatement) -> bool {
        true
    }
    fn leave_for_statement(&mut self, _node: &ForStatement) {}

    fn enter_function_call_expression(&mut self, _node: &FunctionCallExpression) -> bool {
        true
    }
    fn leave_function_call_expression(&mut self, _node: &FunctionCallExpression) {}

    fn enter_function_definition(&mut self, _node: &FunctionDefinition) -> bool {
        true
    }
    fn leave_function_definition(&mut self, _node: &FunctionDefinition) {}

    fn enter_function_type(&mut self, _node: &FunctionType) -> bool {
        true
    }
    fn leave_function_type(&mut self, _node: &FunctionType) {}

    fn enter_hex_number_expression(&mut self, _node: &HexNumberExpression) -> bool {
        true
    }
    fn leave_hex_number_expression(&mut self, _node: &HexNumberExpression) {}

    fn enter_if_statement(&mut self, _node: &IfStatement) -> bool {
        true
    }
    fn leave_if_statement(&mut self, _node: &IfStatement) {}

    fn enter_import_deconstruction(&mut self, _node: &ImportDeconstruction) -> bool {
        true
    }
    fn leave_import_deconstruction(&mut self, _node: &ImportDeconstruction) {}

    fn enter_import_deconstruction_symbol(&mut self, _node: &ImportDeconstructionSymbol) -> bool {
        true
    }
    fn leave_import_deconstruction_symbol(&mut self, _node: &ImportDeconstructionSymbol) {}

    fn enter_index_access_expression(&mut self, _node: &IndexAccessExpression) -> bool {
        true
    }
    fn leave_index_access_expression(&mut self, _node: &IndexAccessExpression) {}

    fn enter_inequality_expression(&mut self, _node: &InequalityExpression) -> bool {
        true
    }
    fn leave_inequality_expression(&mut self, _node: &InequalityExpression) {}

    fn enter_inheritance_type(&mut self, _node: &InheritanceType) -> bool {
        true
    }
    fn leave_inheritance_type(&mut self, _node: &InheritanceType) {}

    fn enter_interface_definition(&mut self, _node: &InterfaceDefinition) -> bool {
        true
    }
    fn leave_interface_definition(&mut self, _node: &InterfaceDefinition) {}

    fn enter_library_definition(&mut self, _node: &LibraryDefinition) -> bool {
        true
    }
    fn leave_library_definition(&mut self, _node: &LibraryDefinition) {}

    fn enter_mapping_type(&mut self, _node: &MappingType) -> bool {
        true
    }
    fn leave_mapping_type(&mut self, _node: &MappingType) {}

    fn enter_member_access_expression(&mut self, _node: &MemberAccessExpression) -> bool {
        true
    }
    fn leave_member_access_expression(&mut self, _node: &MemberAccessExpression) {}

    fn enter_modifier_invocation(&mut self, _node: &ModifierInvocation) -> bool {
        true
    }
    fn leave_modifier_invocation(&mut self, _node: &ModifierInvocation) {}

    fn enter_multi_typed_declaration(&mut self, _node: &MultiTypedDeclaration) -> bool {
        true
    }
    fn leave_multi_typed_declaration(&mut self, _node: &MultiTypedDeclaration) {}

    fn enter_multi_typed_declaration_element(
        &mut self,
        _node: &MultiTypedDeclarationElement,
    ) -> bool {
        true
    }
    fn leave_multi_typed_declaration_element(&mut self, _node: &MultiTypedDeclarationElement) {}

    fn enter_multiplicative_expression(&mut self, _node: &MultiplicativeExpression) -> bool {
        true
    }
    fn leave_multiplicative_expression(&mut self, _node: &MultiplicativeExpression) {}

    fn enter_named_argument(&mut self, _node: &NamedArgument) -> bool {
        true
    }
    fn leave_named_argument(&mut self, _node: &NamedArgument) {}

    fn enter_new_expression(&mut self, _node: &NewExpression) -> bool {
        true
    }
    fn leave_new_expression(&mut self, _node: &NewExpression) {}

    fn enter_or_expression(&mut self, _node: &OrExpression) -> bool {
        true
    }
    fn leave_or_expression(&mut self, _node: &OrExpression) {}

    fn enter_parameter(&mut self, _node: &Parameter) -> bool {
        true
    }
    fn leave_parameter(&mut self, _node: &Parameter) {}

    fn enter_path_import(&mut self, _node: &PathImport) -> bool {
        true
    }
    fn leave_path_import(&mut self, _node: &PathImport) {}

    fn enter_postfix_expression(&mut self, _node: &PostfixExpression) -> bool {
        true
    }
    fn leave_postfix_expression(&mut self, _node: &PostfixExpression) {}

    fn enter_pragma_directive(&mut self, _node: &PragmaDirective) -> bool {
        true
    }
    fn leave_pragma_directive(&mut self, _node: &PragmaDirective) {}

    fn enter_prefix_expression(&mut self, _node: &PrefixExpression) -> bool {
        true
    }
    fn leave_prefix_expression(&mut self, _node: &PrefixExpression) {}

    fn enter_return_statement(&mut self, _node: &ReturnStatement) -> bool {
        true
    }
    fn leave_return_statement(&mut self, _node: &ReturnStatement) {}

    fn enter_revert_statement(&mut self, _node: &RevertStatement) -> bool {
        true
    }
    fn leave_revert_statement(&mut self, _node: &RevertStatement) {}

    fn enter_shift_expression(&mut self, _node: &ShiftExpression) -> bool {
        true
    }
    fn leave_shift_expression(&mut self, _node: &ShiftExpression) {}

    fn enter_single_typed_declaration(&mut self, _node: &SingleTypedDeclaration) -> bool {
        true
    }
    fn leave_single_typed_declaration(&mut self, _node: &SingleTypedDeclaration) {}

    fn enter_source_unit(&mut self, _node: &SourceUnit) -> bool {
        true
    }
    fn leave_source_unit(&mut self, _node: &SourceUnit) {}

    fn enter_state_variable_definition(&mut self, _node: &StateVariableDefinition) -> bool {
        true
    }
    fn leave_state_variable_definition(&mut self, _node: &StateVariableDefinition) {}

    fn enter_struct_definition(&mut self, _node: &StructDefinition) -> bool {
        true
    }
    fn leave_struct_definition(&mut self, _node: &StructDefinition) {}

    fn enter_struct_member(&mut self, _node: &StructMember) -> bool {
        true
    }
    fn leave_struct_member(&mut self, _node: &StructMember) {}

    fn enter_try_statement(&mut self, _node: &TryStatement) -> bool {
        true
    }
    fn leave_try_statement(&mut self, _node: &TryStatement) {}

    fn enter_tuple_expression(&mut self, _node: &TupleExpression) -> bool {
        true
    }
    fn leave_tuple_expression(&mut self, _node: &TupleExpression) {}

    fn enter_tuple_value(&mut self, _node: &TupleValue) -> bool {
        true
    }
    fn leave_tuple_value(&mut self, _node: &TupleValue) {}

    fn enter_type_expression(&mut self, _node: &TypeExpression) -> bool {
        true
    }
    fn leave_type_expression(&mut self, _node: &TypeExpression) {}

    fn enter_unchecked_block(&mut self, _node: &UncheckedBlock) -> bool {
        true
    }
    fn leave_unchecked_block(&mut self, _node: &UncheckedBlock) {}

    fn enter_user_defined_value_type_definition(
        &mut self,
        _node: &UserDefinedValueTypeDefinition,
    ) -> bool {
        true
    }
    fn leave_user_defined_value_type_definition(&mut self, _node: &UserDefinedValueTypeDefinition) {
    }

    fn enter_using_deconstruction(&mut self, _node: &UsingDeconstruction) -> bool {
        true
    }
    fn leave_using_deconstruction(&mut self, _node: &UsingDeconstruction) {}

    fn enter_using_deconstruction_symbol(&mut self, _node: &UsingDeconstructionSymbol) -> bool {
        true
    }
    fn leave_using_deconstruction_symbol(&mut self, _node: &UsingDeconstructionSymbol) {}

    fn enter_using_directive(&mut self, _node: &UsingDirective) -> bool {
        true
    }
    fn leave_using_directive(&mut self, _node: &UsingDirective) {}

    fn enter_variable_declaration(&mut self, _node: &VariableDeclaration) -> bool {
        true
    }
    fn leave_variable_declaration(&mut self, _node: &VariableDeclaration) {}

    fn enter_variable_declaration_statement(
        &mut self,
        _node: &VariableDeclarationStatement,
    ) -> bool {
        true
    }
    fn leave_variable_declaration_statement(&mut self, _node: &VariableDeclarationStatement) {}

    fn enter_version_pragma(&mut self, _node: &VersionPragma) -> bool {
        true
    }
    fn leave_version_pragma(&mut self, _node: &VersionPragma) {}

    fn enter_version_range(&mut self, _node: &VersionRange) -> bool {
        true
    }
    fn leave_version_range(&mut self, _node: &VersionRange) {}

    fn enter_version_term(&mut self, _node: &VersionTerm) -> bool {
        true
    }
    fn leave_version_term(&mut self, _node: &VersionTerm) {}

    fn enter_while_statement(&mut self, _node: &WhileStatement) -> bool {
        true
    }
    fn leave_while_statement(&mut self, _node: &WhileStatement) {}

    fn enter_yul_block(&mut self, _node: &YulBlock) -> bool {
        true
    }
    fn leave_yul_block(&mut self, _node: &YulBlock) {}

    fn enter_yul_break_statement(&mut self, _node: &YulBreakStatement) -> bool {
        true
    }
    fn leave_yul_break_statement(&mut self, _node: &YulBreakStatement) {}

    fn enter_yul_continue_statement(&mut self, _node: &YulContinueStatement) -> bool {
        true
    }
    fn leave_yul_continue_statement(&mut self, _node: &YulContinueStatement) {}

    fn enter_yul_default_case(&mut self, _node: &YulDefaultCase) -> bool {
        true
    }
    fn leave_yul_default_case(&mut self, _node: &YulDefaultCase) {}

    fn enter_yul_for_statement(&mut self, _node: &YulForStatement) -> bool {
        true
    }
    fn leave_yul_for_statement(&mut self, _node: &YulForStatement) {}

    fn enter_yul_function_call_expression(&mut self, _node: &YulFunctionCallExpression) -> bool {
        true
    }
    fn leave_yul_function_call_expression(&mut self, _node: &YulFunctionCallExpression) {}

    fn enter_yul_function_definition(&mut self, _node: &YulFunctionDefinition) -> bool {
        true
    }
    fn leave_yul_function_definition(&mut self, _node: &YulFunctionDefinition) {}

    fn enter_yul_if_statement(&mut self, _node: &YulIfStatement) -> bool {
        true
    }
    fn leave_yul_if_statement(&mut self, _node: &YulIfStatement) {}

    fn enter_yul_leave_statement(&mut self, _node: &YulLeaveStatement) -> bool {
        true
    }
    fn leave_yul_leave_statement(&mut self, _node: &YulLeaveStatement) {}

    fn enter_yul_switch_statement(&mut self, _node: &YulSwitchStatement) -> bool {
        true
    }
    fn leave_yul_switch_statement(&mut self, _node: &YulSwitchStatement) {}

    fn enter_yul_value_case(&mut self, _node: &YulValueCase) -> bool {
        true
    }
    fn leave_yul_value_case(&mut self, _node: &YulValueCase) {}

    fn enter_yul_variable_assignment_statement(
        &mut self,
        _node: &YulVariableAssignmentStatement,
    ) -> bool {
        true
    }
    fn leave_yul_variable_assignment_statement(&mut self, _node: &YulVariableAssignmentStatement) {}

    fn enter_yul_variable_declaration_statement(
        &mut self,
        _node: &YulVariableDeclarationStatement,
    ) -> bool {
        true
    }
    fn leave_yul_variable_declaration_statement(
        &mut self,
        _node: &YulVariableDeclarationStatement,
    ) {
    }

    fn enter_yul_variable_declaration_value(
        &mut self,
        _node: &YulVariableDeclarationValue,
    ) -> bool {
        true
    }
    fn leave_yul_variable_declaration_value(&mut self, _node: &YulVariableDeclarationValue) {}

    fn enter_abicoder_version(&mut self, _node: &AbicoderVersion) -> bool {
        true
    }
    fn leave_abicoder_version(&mut self, _node: &AbicoderVersion) {}

    fn enter_additive_expression_operator(&mut self, _node: &AdditiveExpressionOperator) -> bool {
        true
    }
    fn leave_additive_expression_operator(&mut self, _node: &AdditiveExpressionOperator) {}

    fn enter_arguments_declaration(&mut self, _node: &ArgumentsDeclaration) -> bool {
        true
    }
    fn leave_arguments_declaration(&mut self, _node: &ArgumentsDeclaration) {}

    fn enter_assignment_expression_operator(
        &mut self,
        _node: &AssignmentExpressionOperator,
    ) -> bool {
        true
    }
    fn leave_assignment_expression_operator(&mut self, _node: &AssignmentExpressionOperator) {}

    fn enter_contract_member(&mut self, _node: &ContractMember) -> bool {
        true
    }
    fn leave_contract_member(&mut self, _node: &ContractMember) {}

    fn enter_elementary_type(&mut self, _node: &ElementaryType) -> bool {
        true
    }
    fn leave_elementary_type(&mut self, _node: &ElementaryType) {}

    fn enter_equality_expression_operator(&mut self, _node: &EqualityExpressionOperator) -> bool {
        true
    }
    fn leave_equality_expression_operator(&mut self, _node: &EqualityExpressionOperator) {}

    fn enter_experimental_feature(&mut self, _node: &ExperimentalFeature) -> bool {
        true
    }
    fn leave_experimental_feature(&mut self, _node: &ExperimentalFeature) {}

    fn enter_expression(&mut self, _node: &Expression) -> bool {
        true
    }
    fn leave_expression(&mut self, _node: &Expression) {}

    fn enter_for_statement_condition(&mut self, _node: &ForStatementCondition) -> bool {
        true
    }
    fn leave_for_statement_condition(&mut self, _node: &ForStatementCondition) {}

    fn enter_for_statement_initialization(&mut self, _node: &ForStatementInitialization) -> bool {
        true
    }
    fn leave_for_statement_initialization(&mut self, _node: &ForStatementInitialization) {}

    fn enter_function_kind(&mut self, _node: &FunctionKind) -> bool {
        true
    }
    fn leave_function_kind(&mut self, _node: &FunctionKind) {}

    fn enter_function_mutability(&mut self, _node: &FunctionMutability) -> bool {
        true
    }
    fn leave_function_mutability(&mut self, _node: &FunctionMutability) {}

    fn enter_function_visibility(&mut self, _node: &FunctionVisibility) -> bool {
        true
    }
    fn leave_function_visibility(&mut self, _node: &FunctionVisibility) {}

    fn enter_import_clause(&mut self, _node: &ImportClause) -> bool {
        true
    }
    fn leave_import_clause(&mut self, _node: &ImportClause) {}

    fn enter_inequality_expression_operator(
        &mut self,
        _node: &InequalityExpressionOperator,
    ) -> bool {
        true
    }
    fn leave_inequality_expression_operator(&mut self, _node: &InequalityExpressionOperator) {}

    fn enter_multiplicative_expression_operator(
        &mut self,
        _node: &MultiplicativeExpressionOperator,
    ) -> bool {
        true
    }
    fn leave_multiplicative_expression_operator(
        &mut self,
        _node: &MultiplicativeExpressionOperator,
    ) {
    }

    fn enter_number_unit(&mut self, _node: &NumberUnit) -> bool {
        true
    }
    fn leave_number_unit(&mut self, _node: &NumberUnit) {}

    fn enter_postfix_expression_operator(&mut self, _node: &PostfixExpressionOperator) -> bool {
        true
    }
    fn leave_postfix_expression_operator(&mut self, _node: &PostfixExpressionOperator) {}

    fn enter_pragma(&mut self, _node: &Pragma) -> bool {
        true
    }
    fn leave_pragma(&mut self, _node: &Pragma) {}

    fn enter_prefix_expression_operator(&mut self, _node: &PrefixExpressionOperator) -> bool {
        true
    }
    fn leave_prefix_expression_operator(&mut self, _node: &PrefixExpressionOperator) {}

    fn enter_shift_expression_operator(&mut self, _node: &ShiftExpressionOperator) -> bool {
        true
    }
    fn leave_shift_expression_operator(&mut self, _node: &ShiftExpressionOperator) {}

    fn enter_source_unit_member(&mut self, _node: &SourceUnitMember) -> bool {
        true
    }
    fn leave_source_unit_member(&mut self, _node: &SourceUnitMember) {}

    fn enter_state_variable_mutability(&mut self, _node: &StateVariableMutability) -> bool {
        true
    }
    fn leave_state_variable_mutability(&mut self, _node: &StateVariableMutability) {}

    fn enter_state_variable_visibility(&mut self, _node: &StateVariableVisibility) -> bool {
        true
    }
    fn leave_state_variable_visibility(&mut self, _node: &StateVariableVisibility) {}

    fn enter_statement(&mut self, _node: &Statement) -> bool {
        true
    }
    fn leave_statement(&mut self, _node: &Statement) {}

    fn enter_storage_location(&mut self, _node: &StorageLocation) -> bool {
        true
    }
    fn leave_storage_location(&mut self, _node: &StorageLocation) {}

    fn enter_string_expression(&mut self, _node: &StringExpression) -> bool {
        true
    }
    fn leave_string_expression(&mut self, _node: &StringExpression) {}

    fn enter_type_name(&mut self, _node: &TypeName) -> bool {
        true
    }
    fn leave_type_name(&mut self, _node: &TypeName) {}

    fn enter_using_clause(&mut self, _node: &UsingClause) -> bool {
        true
    }
    fn leave_using_clause(&mut self, _node: &UsingClause) {}

    fn enter_using_operator(&mut self, _node: &UsingOperator) -> bool {
        true
    }
    fn leave_using_operator(&mut self, _node: &UsingOperator) {}

    fn enter_using_target(&mut self, _node: &UsingTarget) -> bool {
        true
    }
    fn leave_using_target(&mut self, _node: &UsingTarget) {}

    fn enter_variable_declaration_target(&mut self, _node: &VariableDeclarationTarget) -> bool {
        true
    }
    fn leave_variable_declaration_target(&mut self, _node: &VariableDeclarationTarget) {}

    fn enter_version_expression(&mut self, _node: &VersionExpression) -> bool {
        true
    }
    fn leave_version_expression(&mut self, _node: &VersionExpression) {}

    fn enter_version_literal(&mut self, _node: &VersionLiteral) -> bool {
        true
    }
    fn leave_version_literal(&mut self, _node: &VersionLiteral) {}

    fn enter_version_operator(&mut self, _node: &VersionOperator) -> bool {
        true
    }
    fn leave_version_operator(&mut self, _node: &VersionOperator) {}

    fn enter_yul_expression(&mut self, _node: &YulExpression) -> bool {
        true
    }
    fn leave_yul_expression(&mut self, _node: &YulExpression) {}

    fn enter_yul_literal(&mut self, _node: &YulLiteral) -> bool {
        true
    }
    fn leave_yul_literal(&mut self, _node: &YulLiteral) {}

    fn enter_yul_statement(&mut self, _node: &YulStatement) -> bool {
        true
    }
    fn leave_yul_statement(&mut self, _node: &YulStatement) {}

    fn enter_yul_switch_case(&mut self, _node: &YulSwitchCase) -> bool {
        true
    }
    fn leave_yul_switch_case(&mut self, _node: &YulSwitchCase) {}

    fn enter_array_values(&mut self, _items: &ArrayValues) -> bool {
        true
    }
    fn leave_array_values(&mut self, _items: &ArrayValues) {}

    fn enter_call_options(&mut self, _items: &CallOptions) -> bool {
        true
    }
    fn leave_call_options(&mut self, _items: &CallOptions) {}

    fn enter_catch_clauses(&mut self, _items: &CatchClauses) -> bool {
        true
    }
    fn leave_catch_clauses(&mut self, _items: &CatchClauses) {}

    fn enter_contract_members(&mut self, _items: &ContractMembers) -> bool {
        true
    }
    fn leave_contract_members(&mut self, _items: &ContractMembers) {}

    fn enter_enum_members(&mut self, _items: &EnumMembers) -> bool {
        true
    }
    fn leave_enum_members(&mut self, _items: &EnumMembers) {}

    fn enter_hex_string_literals(&mut self, _items: &HexStringLiterals) -> bool {
        true
    }
    fn leave_hex_string_literals(&mut self, _items: &HexStringLiterals) {}

    fn enter_identifier_path(&mut self, _items: &IdentifierPath) -> bool {
        true
    }
    fn leave_identifier_path(&mut self, _items: &IdentifierPath) {}

    fn enter_import_deconstruction_symbols(
        &mut self,
        _items: &ImportDeconstructionSymbols,
    ) -> bool {
        true
    }
    fn leave_import_deconstruction_symbols(&mut self, _items: &ImportDeconstructionSymbols) {}

    fn enter_inheritance_types(&mut self, _items: &InheritanceTypes) -> bool {
        true
    }
    fn leave_inheritance_types(&mut self, _items: &InheritanceTypes) {}

    fn enter_interface_members(&mut self, _items: &InterfaceMembers) -> bool {
        true
    }
    fn leave_interface_members(&mut self, _items: &InterfaceMembers) {}

    fn enter_library_members(&mut self, _items: &LibraryMembers) -> bool {
        true
    }
    fn leave_library_members(&mut self, _items: &LibraryMembers) {}

    fn enter_modifier_invocations(&mut self, _items: &ModifierInvocations) -> bool {
        true
    }
    fn leave_modifier_invocations(&mut self, _items: &ModifierInvocations) {}

    fn enter_multi_typed_declaration_elements(
        &mut self,
        _items: &MultiTypedDeclarationElements,
    ) -> bool {
        true
    }
    fn leave_multi_typed_declaration_elements(&mut self, _items: &MultiTypedDeclarationElements) {}

    fn enter_named_arguments(&mut self, _items: &NamedArguments) -> bool {
        true
    }
    fn leave_named_arguments(&mut self, _items: &NamedArguments) {}

    fn enter_override_paths(&mut self, _items: &OverridePaths) -> bool {
        true
    }
    fn leave_override_paths(&mut self, _items: &OverridePaths) {}

    fn enter_parameters(&mut self, _items: &Parameters) -> bool {
        true
    }
    fn leave_parameters(&mut self, _items: &Parameters) {}

    fn enter_positional_arguments(&mut self, _items: &PositionalArguments) -> bool {
        true
    }
    fn leave_positional_arguments(&mut self, _items: &PositionalArguments) {}

    fn enter_simple_version_literal(&mut self, _items: &SimpleVersionLiteral) -> bool {
        true
    }
    fn leave_simple_version_literal(&mut self, _items: &SimpleVersionLiteral) {}

    fn enter_source_unit_members(&mut self, _items: &SourceUnitMembers) -> bool {
        true
    }
    fn leave_source_unit_members(&mut self, _items: &SourceUnitMembers) {}

    fn enter_statements(&mut self, _items: &Statements) -> bool {
        true
    }
    fn leave_statements(&mut self, _items: &Statements) {}

    fn enter_string_literals(&mut self, _items: &StringLiterals) -> bool {
        true
    }
    fn leave_string_literals(&mut self, _items: &StringLiterals) {}

    fn enter_struct_members(&mut self, _items: &StructMembers) -> bool {
        true
    }
    fn leave_struct_members(&mut self, _items: &StructMembers) {}

    fn enter_tuple_values(&mut self, _items: &TupleValues) -> bool {
        true
    }
    fn leave_tuple_values(&mut self, _items: &TupleValues) {}

    fn enter_unicode_string_literals(&mut self, _items: &UnicodeStringLiterals) -> bool {
        true
    }
    fn leave_unicode_string_literals(&mut self, _items: &UnicodeStringLiterals) {}

    fn enter_using_deconstruction_symbols(&mut self, _items: &UsingDeconstructionSymbols) -> bool {
        true
    }
    fn leave_using_deconstruction_symbols(&mut self, _items: &UsingDeconstructionSymbols) {}

    fn enter_version_expression_set(&mut self, _items: &VersionExpressionSet) -> bool {
        true
    }
    fn leave_version_expression_set(&mut self, _items: &VersionExpressionSet) {}

    fn enter_version_expression_sets(&mut self, _items: &VersionExpressionSets) -> bool {
        true
    }
    fn leave_version_expression_sets(&mut self, _items: &VersionExpressionSets) {}

    fn enter_yul_arguments(&mut self, _items: &YulArguments) -> bool {
        true
    }
    fn leave_yul_arguments(&mut self, _items: &YulArguments) {}

    fn enter_yul_flags(&mut self, _items: &YulFlags) -> bool {
        true
    }
    fn leave_yul_flags(&mut self, _items: &YulFlags) {}

    fn enter_yul_parameters(&mut self, _items: &YulParameters) -> bool {
        true
    }
    fn leave_yul_parameters(&mut self, _items: &YulParameters) {}

    fn enter_yul_path(&mut self, _items: &YulPath) -> bool {
        true
    }
    fn leave_yul_path(&mut self, _items: &YulPath) {}

    fn enter_yul_paths(&mut self, _items: &YulPaths) -> bool {
        true
    }
    fn leave_yul_paths(&mut self, _items: &YulPaths) {}

    fn enter_yul_statements(&mut self, _items: &YulStatements) -> bool {
        true
    }
    fn leave_yul_statements(&mut self, _items: &YulStatements) {}

    fn enter_yul_switch_cases(&mut self, _items: &YulSwitchCases) -> bool {
        true
    }
    fn leave_yul_switch_cases(&mut self, _items: &YulSwitchCases) {}

    fn enter_yul_variable_names(&mut self, _items: &YulVariableNames) -> bool {
        true
    }
    fn leave_yul_variable_names(&mut self, _items: &YulVariableNames) {}
    // Terminals are not entered or left, since there's no children to them.
    // Instead they're just visited.
    fn visit_abi_encoder_v2_keyword(&mut self, _node: &ABIEncoderV2Keyword) {}

    fn visit_abicoder_v1_keyword(&mut self, _node: &AbicoderV1Keyword) {}

    fn visit_abicoder_v2_keyword(&mut self, _node: &AbicoderV2Keyword) {}

    fn visit_abstract_keyword(&mut self, _node: &AbstractKeyword) {}

    fn visit_ampersand(&mut self, _node: &Ampersand) {}

    fn visit_ampersand_equal(&mut self, _node: &AmpersandEqual) {}

    fn visit_anonymous_keyword(&mut self, _node: &AnonymousKeyword) {}

    fn visit_asterisk(&mut self, _node: &Asterisk) {}

    fn visit_asterisk_equal(&mut self, _node: &AsteriskEqual) {}

    fn visit_bang(&mut self, _node: &Bang) {}

    fn visit_bang_equal(&mut self, _node: &BangEqual) {}

    fn visit_bar(&mut self, _node: &Bar) {}

    fn visit_bar_equal(&mut self, _node: &BarEqual) {}

    fn visit_bool_keyword(&mut self, _node: &BoolKeyword) {}

    fn visit_bytes_keyword(&mut self, _node: &BytesKeyword) {}

    fn visit_call_data_keyword(&mut self, _node: &CallDataKeyword) {}

    fn visit_caret(&mut self, _node: &Caret) {}

    fn visit_caret_equal(&mut self, _node: &CaretEqual) {}

    fn visit_days_keyword(&mut self, _node: &DaysKeyword) {}

    fn visit_decimal_literal(&mut self, _node: &DecimalLiteral) {}

    fn visit_delete_keyword(&mut self, _node: &DeleteKeyword) {}

    fn visit_equal(&mut self, _node: &Equal) {}

    fn visit_equal_equal(&mut self, _node: &EqualEqual) {}

    fn visit_ether_keyword(&mut self, _node: &EtherKeyword) {}

    fn visit_false_keyword(&mut self, _node: &FalseKeyword) {}

    fn visit_fixed_keyword(&mut self, _node: &FixedKeyword) {}

    fn visit_global_keyword(&mut self, _node: &GlobalKeyword) {}

    fn visit_greater_than(&mut self, _node: &GreaterThan) {}

    fn visit_greater_than_equal(&mut self, _node: &GreaterThanEqual) {}

    fn visit_greater_than_greater_than(&mut self, _node: &GreaterThanGreaterThan) {}

    fn visit_greater_than_greater_than_equal(&mut self, _node: &GreaterThanGreaterThanEqual) {}

    fn visit_greater_than_greater_than_greater_than(
        &mut self,
        _node: &GreaterThanGreaterThanGreaterThan,
    ) {
    }

    fn visit_greater_than_greater_than_greater_than_equal(
        &mut self,
        _node: &GreaterThanGreaterThanGreaterThanEqual,
    ) {
    }

    fn visit_gwei_keyword(&mut self, _node: &GweiKeyword) {}

    fn visit_hex_literal(&mut self, _node: &HexLiteral) {}

    fn visit_hex_string_literal(&mut self, _node: &HexStringLiteral) {}

    fn visit_hours_keyword(&mut self, _node: &HoursKeyword) {}

    fn visit_identifier(&mut self, _node: &Identifier) {}

    fn visit_indexed_keyword(&mut self, _node: &IndexedKeyword) {}

    fn visit_int_keyword(&mut self, _node: &IntKeyword) {}

    fn visit_less_than(&mut self, _node: &LessThan) {}

    fn visit_less_than_equal(&mut self, _node: &LessThanEqual) {}

    fn visit_less_than_less_than(&mut self, _node: &LessThanLessThan) {}

    fn visit_less_than_less_than_equal(&mut self, _node: &LessThanLessThanEqual) {}

    fn visit_memory_keyword(&mut self, _node: &MemoryKeyword) {}

    fn visit_minus(&mut self, _node: &Minus) {}

    fn visit_minus_equal(&mut self, _node: &MinusEqual) {}

    fn visit_minus_minus(&mut self, _node: &MinusMinus) {}

    fn visit_minutes_keyword(&mut self, _node: &MinutesKeyword) {}

    fn visit_payable_keyword(&mut self, _node: &PayableKeyword) {}

    fn visit_percent(&mut self, _node: &Percent) {}

    fn visit_percent_equal(&mut self, _node: &PercentEqual) {}

    fn visit_plus(&mut self, _node: &Plus) {}

    fn visit_plus_equal(&mut self, _node: &PlusEqual) {}

    fn visit_plus_plus(&mut self, _node: &PlusPlus) {}

    fn visit_pragma_caret(&mut self, _node: &PragmaCaret) {}

    fn visit_pragma_equal(&mut self, _node: &PragmaEqual) {}

    fn visit_pragma_greater_than(&mut self, _node: &PragmaGreaterThan) {}

    fn visit_pragma_greater_than_equal(&mut self, _node: &PragmaGreaterThanEqual) {}

    fn visit_pragma_less_than(&mut self, _node: &PragmaLessThan) {}

    fn visit_pragma_less_than_equal(&mut self, _node: &PragmaLessThanEqual) {}

    fn visit_pragma_tilde(&mut self, _node: &PragmaTilde) {}

    fn visit_smt_checker_keyword(&mut self, _node: &SMTCheckerKeyword) {}

    fn visit_seconds_keyword(&mut self, _node: &SecondsKeyword) {}

    fn visit_semicolon(&mut self, _node: &Semicolon) {}

    fn visit_slash(&mut self, _node: &Slash) {}

    fn visit_slash_equal(&mut self, _node: &SlashEqual) {}

    fn visit_storage_keyword(&mut self, _node: &StorageKeyword) {}

    fn visit_string_keyword(&mut self, _node: &StringKeyword) {}

    fn visit_string_literal(&mut self, _node: &StringLiteral) {}

    fn visit_super_keyword(&mut self, _node: &SuperKeyword) {}

    fn visit_this_keyword(&mut self, _node: &ThisKeyword) {}

    fn visit_tilde(&mut self, _node: &Tilde) {}

    fn visit_true_keyword(&mut self, _node: &TrueKeyword) {}

    fn visit_ufixed_keyword(&mut self, _node: &UfixedKeyword) {}

    fn visit_uint_keyword(&mut self, _node: &UintKeyword) {}

    fn visit_unicode_string_literal(&mut self, _node: &UnicodeStringLiteral) {}

    fn visit_version_specifier(&mut self, _node: &VersionSpecifier) {}

    fn visit_virtual_keyword(&mut self, _node: &VirtualKeyword) {}

    fn visit_weeks_keyword(&mut self, _node: &WeeksKeyword) {}

    fn visit_wei_keyword(&mut self, _node: &WeiKeyword) {}
}

//
// Sequences
//

pub fn accept_abicoder_pragma(node: &AbicoderPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_abicoder_pragma(node) {
        return;
    }
    accept_abicoder_version(&node.version, visitor);
    visitor.leave_abicoder_pragma(node);
}

pub fn accept_additive_expression(node: &AdditiveExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_additive_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_additive_expression_operator(&node.operator, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_additive_expression(node);
}

pub fn accept_address_type(node: &AddressType, visitor: &mut impl Visitor) {
    if !visitor.enter_address_type(node) {
        return;
    }
    if let Some(payable_keyword) = &node.payable_keyword {
        visitor.visit_payable_keyword(payable_keyword);
    }
    visitor.leave_address_type(node);
}

pub fn accept_and_expression(node: &AndExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_and_expression(node);
}

pub fn accept_array_expression(node: &ArrayExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_array_expression(node) {
        return;
    }
    accept_array_values(&node.items, visitor);
    visitor.leave_array_expression(node);
}

pub fn accept_array_type_name(node: &ArrayTypeName, visitor: &mut impl Visitor) {
    if !visitor.enter_array_type_name(node) {
        return;
    }
    accept_type_name(&node.operand, visitor);
    if let Some(index) = &node.index {
        accept_expression(index, visitor);
    }
    visitor.leave_array_type_name(node);
}

pub fn accept_assembly_statement(node: &AssemblyStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_assembly_statement(node) {
        return;
    }
    if let Some(label) = &node.label {
        visitor.visit_string_literal(label);
    }
    if let Some(flags) = &node.flags {
        accept_yul_flags(flags, visitor);
    }
    accept_yul_block(&node.body, visitor);
    visitor.leave_assembly_statement(node);
}

pub fn accept_assignment_expression(node: &AssignmentExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_assignment_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_assignment_expression_operator(&node.operator, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_assignment_expression(node);
}

pub fn accept_bitwise_and_expression(node: &BitwiseAndExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_bitwise_and_expression(node);
}

pub fn accept_bitwise_or_expression(node: &BitwiseOrExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_or_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_bitwise_or_expression(node);
}

pub fn accept_bitwise_xor_expression(node: &BitwiseXorExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_xor_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_bitwise_xor_expression(node);
}

pub fn accept_block(node: &Block, visitor: &mut impl Visitor) {
    if !visitor.enter_block(node) {
        return;
    }
    accept_statements(&node.statements, visitor);
    visitor.leave_block(node);
}

pub fn accept_break_statement(node: &BreakStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_break_statement(node) {
        return;
    }
    visitor.leave_break_statement(node);
}

pub fn accept_call_options_expression(node: &CallOptionsExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_call_options_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_call_options(&node.options, visitor);
    visitor.leave_call_options_expression(node);
}

pub fn accept_catch_clause(node: &CatchClause, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause(node) {
        return;
    }
    if let Some(error) = &node.error {
        accept_catch_clause_error(error, visitor);
    }
    accept_block(&node.body, visitor);
    visitor.leave_catch_clause(node);
}

pub fn accept_catch_clause_error(node: &CatchClauseError, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause_error(node) {
        return;
    }
    if let Some(name) = &node.name {
        visitor.visit_identifier(name);
    }
    accept_parameters(&node.parameters, visitor);
    visitor.leave_catch_clause_error(node);
}

pub fn accept_conditional_expression(node: &ConditionalExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_conditional_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_expression(&node.true_expression, visitor);
    accept_expression(&node.false_expression, visitor);
    visitor.leave_conditional_expression(node);
}

pub fn accept_constant_definition(node: &ConstantDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_constant_definition(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.visit_identifier(&node.name);
    if let Some(visibility) = &node.visibility {
        accept_state_variable_visibility(visibility, visitor);
    }
    if let Some(value) = &node.value {
        accept_expression(value, visitor);
    }
    visitor.leave_constant_definition(node);
}

pub fn accept_continue_statement(node: &ContinueStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_continue_statement(node) {
        return;
    }
    visitor.leave_continue_statement(node);
}

pub fn accept_contract_definition(node: &ContractDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_definition(node) {
        return;
    }
    if let Some(abstract_keyword) = &node.abstract_keyword {
        visitor.visit_abstract_keyword(abstract_keyword);
    }
    visitor.visit_identifier(&node.name);
    accept_inheritance_types(&node.inheritance_types, visitor);
    if let Some(storage_layout) = &node.storage_layout {
        accept_expression(storage_layout, visitor);
    }
    accept_contract_members(&node.members, visitor);
    visitor.leave_contract_definition(node);
}

pub fn accept_decimal_number_expression(
    node: &DecimalNumberExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_decimal_number_expression(node) {
        return;
    }
    visitor.visit_decimal_literal(&node.literal);
    if let Some(unit) = &node.unit {
        accept_number_unit(unit, visitor);
    }
    visitor.leave_decimal_number_expression(node);
}

pub fn accept_do_while_statement(node: &DoWhileStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_do_while_statement(node) {
        return;
    }
    accept_statement(&node.body, visitor);
    accept_expression(&node.condition, visitor);
    visitor.leave_do_while_statement(node);
}

pub fn accept_emit_statement(node: &EmitStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_emit_statement(node) {
        return;
    }
    accept_identifier_path(&node.event, visitor);
    accept_arguments_declaration(&node.arguments, visitor);
    visitor.leave_emit_statement(node);
}

pub fn accept_enum_definition(node: &EnumDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_enum_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    accept_enum_members(&node.members, visitor);
    visitor.leave_enum_definition(node);
}

pub fn accept_equality_expression(node: &EqualityExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_equality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_equality_expression_operator(&node.operator, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_equality_expression(node);
}

pub fn accept_error_definition(node: &ErrorDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_error_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    accept_parameters(&node.parameters, visitor);
    visitor.leave_error_definition(node);
}

pub fn accept_event_definition(node: &EventDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_event_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    if let Some(anonymous_keyword) = &node.anonymous_keyword {
        visitor.visit_anonymous_keyword(anonymous_keyword);
    }
    accept_parameters(&node.parameters, visitor);
    visitor.leave_event_definition(node);
}

pub fn accept_experimental_pragma(node: &ExperimentalPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_experimental_pragma(node) {
        return;
    }
    accept_experimental_feature(&node.feature, visitor);
    visitor.leave_experimental_pragma(node);
}

pub fn accept_exponentiation_expression(
    node: &ExponentiationExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_exponentiation_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_exponentiation_expression(node);
}

pub fn accept_expression_statement(node: &ExpressionStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_expression_statement(node) {
        return;
    }
    accept_expression(&node.expression, visitor);
    visitor.leave_expression_statement(node);
}

pub fn accept_for_statement(node: &ForStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_for_statement(node) {
        return;
    }
    accept_for_statement_initialization(&node.initialization, visitor);
    accept_for_statement_condition(&node.condition, visitor);
    if let Some(iterator) = &node.iterator {
        accept_expression(iterator, visitor);
    }
    accept_statement(&node.body, visitor);
    visitor.leave_for_statement(node);
}

pub fn accept_function_call_expression(node: &FunctionCallExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_function_call_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_arguments_declaration(&node.arguments, visitor);
    visitor.leave_function_call_expression(node);
}

pub fn accept_function_definition(node: &FunctionDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_function_definition(node) {
        return;
    }
    accept_function_kind(&node.kind, visitor);
    if let Some(name) = &node.name {
        visitor.visit_identifier(name);
    }
    accept_parameters(&node.parameters, visitor);
    accept_function_visibility(&node.visibility, visitor);
    accept_function_mutability(&node.mutability, visitor);
    if let Some(virtual_keyword) = &node.virtual_keyword {
        visitor.visit_virtual_keyword(virtual_keyword);
    }
    if let Some(override_specifier) = &node.override_specifier {
        accept_override_paths(override_specifier, visitor);
    }
    accept_modifier_invocations(&node.modifier_invocations, visitor);
    if let Some(returns) = &node.returns {
        accept_parameters(returns, visitor);
    }
    if let Some(body) = &node.body {
        accept_block(body, visitor);
    }
    visitor.leave_function_definition(node);
}

pub fn accept_function_type(node: &FunctionType, visitor: &mut impl Visitor) {
    if !visitor.enter_function_type(node) {
        return;
    }
    accept_parameters(&node.parameters, visitor);
    accept_function_visibility(&node.visibility, visitor);
    accept_function_mutability(&node.mutability, visitor);
    if let Some(returns) = &node.returns {
        accept_parameters(returns, visitor);
    }
    visitor.leave_function_type(node);
}

pub fn accept_hex_number_expression(node: &HexNumberExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_hex_number_expression(node) {
        return;
    }
    visitor.visit_hex_literal(&node.literal);
    visitor.leave_hex_number_expression(node);
}

pub fn accept_if_statement(node: &IfStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_if_statement(node) {
        return;
    }
    accept_expression(&node.condition, visitor);
    accept_statement(&node.body, visitor);
    if let Some(else_branch) = &node.else_branch {
        accept_statement(else_branch, visitor);
    }
    visitor.leave_if_statement(node);
}

pub fn accept_import_deconstruction(node: &ImportDeconstruction, visitor: &mut impl Visitor) {
    if !visitor.enter_import_deconstruction(node) {
        return;
    }
    accept_import_deconstruction_symbols(&node.symbols, visitor);
    visitor.visit_string_literal(&node.path);
    visitor.leave_import_deconstruction(node);
}

pub fn accept_import_deconstruction_symbol(
    node: &ImportDeconstructionSymbol,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_import_deconstruction_symbol(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    if let Some(alias) = &node.alias {
        visitor.visit_identifier(alias);
    }
    visitor.leave_import_deconstruction_symbol(node);
}

pub fn accept_index_access_expression(node: &IndexAccessExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_index_access_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    if let Some(start) = &node.start {
        accept_expression(start, visitor);
    }
    if let Some(end) = &node.end {
        accept_expression(end, visitor);
    }
    visitor.leave_index_access_expression(node);
}

pub fn accept_inequality_expression(node: &InequalityExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_inequality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_inequality_expression_operator(&node.operator, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_inequality_expression(node);
}

pub fn accept_inheritance_type(node: &InheritanceType, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_type(node) {
        return;
    }
    accept_identifier_path(&node.type_name, visitor);
    if let Some(arguments) = &node.arguments {
        accept_arguments_declaration(arguments, visitor);
    }
    visitor.leave_inheritance_type(node);
}

pub fn accept_interface_definition(node: &InterfaceDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_interface_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    if let Some(inheritance) = &node.inheritance {
        accept_inheritance_types(inheritance, visitor);
    }
    accept_interface_members(&node.members, visitor);
    visitor.leave_interface_definition(node);
}

pub fn accept_library_definition(node: &LibraryDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_library_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    accept_library_members(&node.members, visitor);
    visitor.leave_library_definition(node);
}

pub fn accept_mapping_type(node: &MappingType, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_type(node) {
        return;
    }
    accept_parameter(&node.key_type, visitor);
    accept_parameter(&node.value_type, visitor);
    visitor.leave_mapping_type(node);
}

pub fn accept_member_access_expression(node: &MemberAccessExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_member_access_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    visitor.visit_identifier(&node.member);
    visitor.leave_member_access_expression(node);
}

pub fn accept_modifier_invocation(node: &ModifierInvocation, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_invocation(node) {
        return;
    }
    accept_identifier_path(&node.name, visitor);
    if let Some(arguments) = &node.arguments {
        accept_arguments_declaration(arguments, visitor);
    }
    visitor.leave_modifier_invocation(node);
}

pub fn accept_multi_typed_declaration(node: &MultiTypedDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_multi_typed_declaration(node) {
        return;
    }
    accept_multi_typed_declaration_elements(&node.elements, visitor);
    accept_expression(&node.value, visitor);
    visitor.leave_multi_typed_declaration(node);
}

pub fn accept_multi_typed_declaration_element(
    node: &MultiTypedDeclarationElement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multi_typed_declaration_element(node) {
        return;
    }
    if let Some(member) = &node.member {
        accept_variable_declaration(member, visitor);
    }
    visitor.leave_multi_typed_declaration_element(node);
}

pub fn accept_multiplicative_expression(
    node: &MultiplicativeExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multiplicative_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_multiplicative_expression_operator(&node.operator, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_multiplicative_expression(node);
}

pub fn accept_named_argument(node: &NamedArgument, visitor: &mut impl Visitor) {
    if !visitor.enter_named_argument(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    accept_expression(&node.value, visitor);
    visitor.leave_named_argument(node);
}

pub fn accept_new_expression(node: &NewExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_new_expression(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_new_expression(node);
}

pub fn accept_or_expression(node: &OrExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_or_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_or_expression(node);
}

pub fn accept_parameter(node: &Parameter, visitor: &mut impl Visitor) {
    if !visitor.enter_parameter(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    if let Some(storage_location) = &node.storage_location {
        accept_storage_location(storage_location, visitor);
    }
    if let Some(name) = &node.name {
        visitor.visit_identifier(name);
    }
    if let Some(indexed) = &node.indexed {
        visitor.visit_indexed_keyword(indexed);
    }
    visitor.leave_parameter(node);
}

pub fn accept_path_import(node: &PathImport, visitor: &mut impl Visitor) {
    if !visitor.enter_path_import(node) {
        return;
    }
    visitor.visit_string_literal(&node.path);
    if let Some(alias) = &node.alias {
        visitor.visit_identifier(alias);
    }
    visitor.leave_path_import(node);
}

pub fn accept_postfix_expression(node: &PostfixExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_postfix_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_postfix_expression_operator(&node.operator, visitor);
    visitor.leave_postfix_expression(node);
}

pub fn accept_pragma_directive(node: &PragmaDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma_directive(node) {
        return;
    }
    accept_pragma(&node.pragma, visitor);
    visitor.leave_pragma_directive(node);
}

pub fn accept_prefix_expression(node: &PrefixExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_prefix_expression(node) {
        return;
    }
    accept_prefix_expression_operator(&node.operator, visitor);
    accept_expression(&node.operand, visitor);
    visitor.leave_prefix_expression(node);
}

pub fn accept_return_statement(node: &ReturnStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_return_statement(node) {
        return;
    }
    if let Some(expression) = &node.expression {
        accept_expression(expression, visitor);
    }
    visitor.leave_return_statement(node);
}

pub fn accept_revert_statement(node: &RevertStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_revert_statement(node) {
        return;
    }
    accept_identifier_path(&node.error, visitor);
    accept_arguments_declaration(&node.arguments, visitor);
    visitor.leave_revert_statement(node);
}

pub fn accept_shift_expression(node: &ShiftExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_shift_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_shift_expression_operator(&node.operator, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_shift_expression(node);
}

pub fn accept_single_typed_declaration(node: &SingleTypedDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_single_typed_declaration(node) {
        return;
    }
    accept_variable_declaration(&node.declaration, visitor);
    if let Some(value) = &node.value {
        accept_expression(value, visitor);
    }
    visitor.leave_single_typed_declaration(node);
}

pub fn accept_source_unit(node: &SourceUnit, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit(node) {
        return;
    }
    accept_source_unit_members(&node.members, visitor);
    visitor.leave_source_unit(node);
}

pub fn accept_state_variable_definition(
    node: &StateVariableDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_definition(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.visit_identifier(&node.name);
    if let Some(value) = &node.value {
        accept_expression(value, visitor);
    }
    accept_state_variable_visibility(&node.visibility, visitor);
    accept_state_variable_mutability(&node.mutability, visitor);
    if let Some(override_specifier) = &node.override_specifier {
        accept_override_paths(override_specifier, visitor);
    }
    visitor.leave_state_variable_definition(node);
}

pub fn accept_struct_definition(node: &StructDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    accept_struct_members(&node.members, visitor);
    visitor.leave_struct_definition(node);
}

pub fn accept_struct_member(node: &StructMember, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_member(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.visit_identifier(&node.name);
    visitor.leave_struct_member(node);
}

pub fn accept_try_statement(node: &TryStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_try_statement(node) {
        return;
    }
    accept_expression(&node.expression, visitor);
    if let Some(returns) = &node.returns {
        accept_parameters(returns, visitor);
    }
    accept_block(&node.body, visitor);
    accept_catch_clauses(&node.catch_clauses, visitor);
    visitor.leave_try_statement(node);
}

pub fn accept_tuple_expression(node: &TupleExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_expression(node) {
        return;
    }
    accept_tuple_values(&node.items, visitor);
    visitor.leave_tuple_expression(node);
}

pub fn accept_tuple_value(node: &TupleValue, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_value(node) {
        return;
    }
    if let Some(expression) = &node.expression {
        accept_expression(expression, visitor);
    }
    visitor.leave_tuple_value(node);
}

pub fn accept_type_expression(node: &TypeExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_type_expression(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_type_expression(node);
}

pub fn accept_unchecked_block(node: &UncheckedBlock, visitor: &mut impl Visitor) {
    if !visitor.enter_unchecked_block(node) {
        return;
    }
    accept_block(&node.block, visitor);
    visitor.leave_unchecked_block(node);
}

pub fn accept_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_user_defined_value_type_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    accept_elementary_type(&node.value_type, visitor);
    visitor.leave_user_defined_value_type_definition(node);
}

pub fn accept_using_deconstruction(node: &UsingDeconstruction, visitor: &mut impl Visitor) {
    if !visitor.enter_using_deconstruction(node) {
        return;
    }
    accept_using_deconstruction_symbols(&node.symbols, visitor);
    visitor.leave_using_deconstruction(node);
}

pub fn accept_using_deconstruction_symbol(
    node: &UsingDeconstructionSymbol,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_using_deconstruction_symbol(node) {
        return;
    }
    accept_identifier_path(&node.name, visitor);
    if let Some(alias) = &node.alias {
        accept_using_operator(alias, visitor);
    }
    visitor.leave_using_deconstruction_symbol(node);
}

pub fn accept_using_directive(node: &UsingDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_using_directive(node) {
        return;
    }
    accept_using_clause(&node.clause, visitor);
    accept_using_target(&node.target, visitor);
    if let Some(global_keyword) = &node.global_keyword {
        visitor.visit_global_keyword(global_keyword);
    }
    visitor.leave_using_directive(node);
}

pub fn accept_variable_declaration(node: &VariableDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_variable_declaration(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    if let Some(storage_location) = &node.storage_location {
        accept_storage_location(storage_location, visitor);
    }
    visitor.visit_identifier(&node.name);
    visitor.leave_variable_declaration(node);
}

pub fn accept_variable_declaration_statement(
    node: &VariableDeclarationStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_statement(node) {
        return;
    }
    accept_variable_declaration_target(&node.target, visitor);
    visitor.leave_variable_declaration_statement(node);
}

pub fn accept_version_pragma(node: &VersionPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_version_pragma(node) {
        return;
    }
    accept_version_expression_sets(&node.sets, visitor);
    visitor.leave_version_pragma(node);
}

pub fn accept_version_range(node: &VersionRange, visitor: &mut impl Visitor) {
    if !visitor.enter_version_range(node) {
        return;
    }
    accept_version_literal(&node.start, visitor);
    accept_version_literal(&node.end, visitor);
    visitor.leave_version_range(node);
}

pub fn accept_version_term(node: &VersionTerm, visitor: &mut impl Visitor) {
    if !visitor.enter_version_term(node) {
        return;
    }
    if let Some(operator) = &node.operator {
        accept_version_operator(operator, visitor);
    }
    accept_version_literal(&node.literal, visitor);
    visitor.leave_version_term(node);
}

pub fn accept_while_statement(node: &WhileStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_while_statement(node) {
        return;
    }
    accept_expression(&node.condition, visitor);
    accept_statement(&node.body, visitor);
    visitor.leave_while_statement(node);
}

pub fn accept_yul_block(node: &YulBlock, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_block(node) {
        return;
    }
    accept_yul_statements(&node.statements, visitor);
    visitor.leave_yul_block(node);
}

pub fn accept_yul_break_statement(node: &YulBreakStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_break_statement(node) {
        return;
    }
    visitor.leave_yul_break_statement(node);
}

pub fn accept_yul_continue_statement(node: &YulContinueStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_continue_statement(node) {
        return;
    }
    visitor.leave_yul_continue_statement(node);
}

pub fn accept_yul_default_case(node: &YulDefaultCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_default_case(node) {
        return;
    }
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_default_case(node);
}

pub fn accept_yul_for_statement(node: &YulForStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_for_statement(node) {
        return;
    }
    accept_yul_block(&node.initialization, visitor);
    accept_yul_expression(&node.condition, visitor);
    accept_yul_block(&node.iterator, visitor);
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_for_statement(node);
}

pub fn accept_yul_function_call_expression(
    node: &YulFunctionCallExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_function_call_expression(node) {
        return;
    }
    accept_yul_expression(&node.operand, visitor);
    accept_yul_arguments(&node.arguments, visitor);
    visitor.leave_yul_function_call_expression(node);
}

pub fn accept_yul_function_definition(node: &YulFunctionDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_function_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name);
    accept_yul_parameters(&node.parameters, visitor);
    if let Some(returns) = &node.returns {
        accept_yul_variable_names(returns, visitor);
    }
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_function_definition(node);
}

pub fn accept_yul_if_statement(node: &YulIfStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_if_statement(node) {
        return;
    }
    accept_yul_expression(&node.condition, visitor);
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_if_statement(node);
}

pub fn accept_yul_leave_statement(node: &YulLeaveStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_leave_statement(node) {
        return;
    }
    visitor.leave_yul_leave_statement(node);
}

pub fn accept_yul_switch_statement(node: &YulSwitchStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_statement(node) {
        return;
    }
    accept_yul_expression(&node.expression, visitor);
    accept_yul_switch_cases(&node.cases, visitor);
    visitor.leave_yul_switch_statement(node);
}

pub fn accept_yul_value_case(node: &YulValueCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_value_case(node) {
        return;
    }
    accept_yul_literal(&node.value, visitor);
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_value_case(node);
}

pub fn accept_yul_variable_assignment_statement(
    node: &YulVariableAssignmentStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_assignment_statement(node) {
        return;
    }
    accept_yul_paths(&node.variables, visitor);
    accept_yul_expression(&node.expression, visitor);
    visitor.leave_yul_variable_assignment_statement(node);
}

pub fn accept_yul_variable_declaration_statement(
    node: &YulVariableDeclarationStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_declaration_statement(node) {
        return;
    }
    accept_yul_variable_names(&node.variables, visitor);
    if let Some(value) = &node.value {
        accept_yul_variable_declaration_value(value, visitor);
    }
    visitor.leave_yul_variable_declaration_statement(node);
}

pub fn accept_yul_variable_declaration_value(
    node: &YulVariableDeclarationValue,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_declaration_value(node) {
        return;
    }
    accept_yul_expression(&node.expression, visitor);
    visitor.leave_yul_variable_declaration_value(node);
}

//
// Choices
//

pub fn accept_abicoder_version(node: &AbicoderVersion, visitor: &mut impl Visitor) {
    if !visitor.enter_abicoder_version(node) {
        return;
    }
    match &node {
        AbicoderVersion::AbicoderV1Keyword(abicoder_v1_keyword) => {
            visitor.visit_abicoder_v1_keyword(abicoder_v1_keyword);
        }
        AbicoderVersion::AbicoderV2Keyword(abicoder_v2_keyword) => {
            visitor.visit_abicoder_v2_keyword(abicoder_v2_keyword);
        }
    }
    visitor.leave_abicoder_version(node);
}

pub fn accept_additive_expression_operator(
    node: &AdditiveExpressionOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_additive_expression_operator(node) {
        return;
    }
    match &node {
        AdditiveExpressionOperator::Minus(minus) => {
            visitor.visit_minus(minus);
        }
        AdditiveExpressionOperator::Plus(plus) => {
            visitor.visit_plus(plus);
        }
    }
    visitor.leave_additive_expression_operator(node);
}

pub fn accept_arguments_declaration(node: &ArgumentsDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_arguments_declaration(node) {
        return;
    }
    match &node {
        ArgumentsDeclaration::PositionalArguments(positional_arguments) => {
            accept_positional_arguments(positional_arguments, visitor);
        }
        ArgumentsDeclaration::NamedArguments(named_arguments) => {
            accept_named_arguments(named_arguments, visitor);
        }
    }
    visitor.leave_arguments_declaration(node);
}

pub fn accept_assignment_expression_operator(
    node: &AssignmentExpressionOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_assignment_expression_operator(node) {
        return;
    }
    match &node {
        AssignmentExpressionOperator::AmpersandEqual(ampersand_equal) => {
            visitor.visit_ampersand_equal(ampersand_equal);
        }
        AssignmentExpressionOperator::AsteriskEqual(asterisk_equal) => {
            visitor.visit_asterisk_equal(asterisk_equal);
        }
        AssignmentExpressionOperator::BarEqual(bar_equal) => {
            visitor.visit_bar_equal(bar_equal);
        }
        AssignmentExpressionOperator::CaretEqual(caret_equal) => {
            visitor.visit_caret_equal(caret_equal);
        }
        AssignmentExpressionOperator::Equal(equal) => {
            visitor.visit_equal(equal);
        }
        AssignmentExpressionOperator::GreaterThanGreaterThanEqual(
            greater_than_greater_than_equal,
        ) => {
            visitor.visit_greater_than_greater_than_equal(greater_than_greater_than_equal);
        }
        AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(
            greater_than_greater_than_greater_than_equal,
        ) => {
            visitor.visit_greater_than_greater_than_greater_than_equal(
                greater_than_greater_than_greater_than_equal,
            );
        }
        AssignmentExpressionOperator::LessThanLessThanEqual(less_than_less_than_equal) => {
            visitor.visit_less_than_less_than_equal(less_than_less_than_equal);
        }
        AssignmentExpressionOperator::MinusEqual(minus_equal) => {
            visitor.visit_minus_equal(minus_equal);
        }
        AssignmentExpressionOperator::PercentEqual(percent_equal) => {
            visitor.visit_percent_equal(percent_equal);
        }
        AssignmentExpressionOperator::PlusEqual(plus_equal) => {
            visitor.visit_plus_equal(plus_equal);
        }
        AssignmentExpressionOperator::SlashEqual(slash_equal) => {
            visitor.visit_slash_equal(slash_equal);
        }
    }
    visitor.leave_assignment_expression_operator(node);
}

pub fn accept_contract_member(node: &ContractMember, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_member(node) {
        return;
    }
    match &node {
        ContractMember::UsingDirective(using_directive) => {
            accept_using_directive(using_directive, visitor);
        }
        ContractMember::FunctionDefinition(function_definition) => {
            accept_function_definition(function_definition, visitor);
        }
        ContractMember::StructDefinition(struct_definition) => {
            accept_struct_definition(struct_definition, visitor);
        }
        ContractMember::EnumDefinition(enum_definition) => {
            accept_enum_definition(enum_definition, visitor);
        }
        ContractMember::EventDefinition(event_definition) => {
            accept_event_definition(event_definition, visitor);
        }
        ContractMember::ErrorDefinition(error_definition) => {
            accept_error_definition(error_definition, visitor);
        }
        ContractMember::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
            accept_user_defined_value_type_definition(user_defined_value_type_definition, visitor);
        }
        ContractMember::StateVariableDefinition(state_variable_definition) => {
            accept_state_variable_definition(state_variable_definition, visitor);
        }
        ContractMember::ConstantDefinition(constant_definition) => {
            accept_constant_definition(constant_definition, visitor);
        }
    }
    visitor.leave_contract_member(node);
}

pub fn accept_elementary_type(node: &ElementaryType, visitor: &mut impl Visitor) {
    if !visitor.enter_elementary_type(node) {
        return;
    }
    match &node {
        ElementaryType::BoolKeyword(bool_keyword) => {
            visitor.visit_bool_keyword(bool_keyword);
        }
        ElementaryType::StringKeyword(string_keyword) => {
            visitor.visit_string_keyword(string_keyword);
        }
        ElementaryType::AddressType(address_type) => {
            accept_address_type(address_type, visitor);
        }
        ElementaryType::BytesKeyword(bytes_keyword) => {
            visitor.visit_bytes_keyword(bytes_keyword);
        }
        ElementaryType::IntKeyword(int_keyword) => {
            visitor.visit_int_keyword(int_keyword);
        }
        ElementaryType::UintKeyword(uint_keyword) => {
            visitor.visit_uint_keyword(uint_keyword);
        }
        ElementaryType::FixedKeyword(fixed_keyword) => {
            visitor.visit_fixed_keyword(fixed_keyword);
        }
        ElementaryType::UfixedKeyword(ufixed_keyword) => {
            visitor.visit_ufixed_keyword(ufixed_keyword);
        }
    }
    visitor.leave_elementary_type(node);
}

pub fn accept_equality_expression_operator(
    node: &EqualityExpressionOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_equality_expression_operator(node) {
        return;
    }
    match &node {
        EqualityExpressionOperator::BangEqual(bang_equal) => {
            visitor.visit_bang_equal(bang_equal);
        }
        EqualityExpressionOperator::EqualEqual(equal_equal) => {
            visitor.visit_equal_equal(equal_equal);
        }
    }
    visitor.leave_equality_expression_operator(node);
}

pub fn accept_experimental_feature(node: &ExperimentalFeature, visitor: &mut impl Visitor) {
    if !visitor.enter_experimental_feature(node) {
        return;
    }
    match &node {
        ExperimentalFeature::ABIEncoderV2Keyword(abi_encoder_v2_keyword) => {
            visitor.visit_abi_encoder_v2_keyword(abi_encoder_v2_keyword);
        }
        ExperimentalFeature::SMTCheckerKeyword(smt_checker_keyword) => {
            visitor.visit_smt_checker_keyword(smt_checker_keyword);
        }
        ExperimentalFeature::StringLiteral(string_literal) => {
            visitor.visit_string_literal(string_literal);
        }
    }
    visitor.leave_experimental_feature(node);
}

pub fn accept_expression(node: &Expression, visitor: &mut impl Visitor) {
    if !visitor.enter_expression(node) {
        return;
    }
    match &node {
        Expression::AssignmentExpression(assignment_expression) => {
            accept_assignment_expression(assignment_expression, visitor);
        }
        Expression::ConditionalExpression(conditional_expression) => {
            accept_conditional_expression(conditional_expression, visitor);
        }
        Expression::OrExpression(or_expression) => {
            accept_or_expression(or_expression, visitor);
        }
        Expression::AndExpression(and_expression) => {
            accept_and_expression(and_expression, visitor);
        }
        Expression::EqualityExpression(equality_expression) => {
            accept_equality_expression(equality_expression, visitor);
        }
        Expression::InequalityExpression(inequality_expression) => {
            accept_inequality_expression(inequality_expression, visitor);
        }
        Expression::BitwiseOrExpression(bitwise_or_expression) => {
            accept_bitwise_or_expression(bitwise_or_expression, visitor);
        }
        Expression::BitwiseXorExpression(bitwise_xor_expression) => {
            accept_bitwise_xor_expression(bitwise_xor_expression, visitor);
        }
        Expression::BitwiseAndExpression(bitwise_and_expression) => {
            accept_bitwise_and_expression(bitwise_and_expression, visitor);
        }
        Expression::ShiftExpression(shift_expression) => {
            accept_shift_expression(shift_expression, visitor);
        }
        Expression::AdditiveExpression(additive_expression) => {
            accept_additive_expression(additive_expression, visitor);
        }
        Expression::MultiplicativeExpression(multiplicative_expression) => {
            accept_multiplicative_expression(multiplicative_expression, visitor);
        }
        Expression::ExponentiationExpression(exponentiation_expression) => {
            accept_exponentiation_expression(exponentiation_expression, visitor);
        }
        Expression::PostfixExpression(postfix_expression) => {
            accept_postfix_expression(postfix_expression, visitor);
        }
        Expression::PrefixExpression(prefix_expression) => {
            accept_prefix_expression(prefix_expression, visitor);
        }
        Expression::FunctionCallExpression(function_call_expression) => {
            accept_function_call_expression(function_call_expression, visitor);
        }
        Expression::CallOptionsExpression(call_options_expression) => {
            accept_call_options_expression(call_options_expression, visitor);
        }
        Expression::MemberAccessExpression(member_access_expression) => {
            accept_member_access_expression(member_access_expression, visitor);
        }
        Expression::IndexAccessExpression(index_access_expression) => {
            accept_index_access_expression(index_access_expression, visitor);
        }
        Expression::NewExpression(new_expression) => {
            accept_new_expression(new_expression, visitor);
        }
        Expression::TupleExpression(tuple_expression) => {
            accept_tuple_expression(tuple_expression, visitor);
        }
        Expression::TypeExpression(type_expression) => {
            accept_type_expression(type_expression, visitor);
        }
        Expression::ArrayExpression(array_expression) => {
            accept_array_expression(array_expression, visitor);
        }
        Expression::HexNumberExpression(hex_number_expression) => {
            accept_hex_number_expression(hex_number_expression, visitor);
        }
        Expression::DecimalNumberExpression(decimal_number_expression) => {
            accept_decimal_number_expression(decimal_number_expression, visitor);
        }
        Expression::StringExpression(string_expression) => {
            accept_string_expression(string_expression, visitor);
        }
        Expression::ElementaryType(elementary_type) => {
            accept_elementary_type(elementary_type, visitor);
        }
        Expression::PayableKeyword(payable_keyword) => {
            visitor.visit_payable_keyword(payable_keyword);
        }
        Expression::ThisKeyword(this_keyword) => {
            visitor.visit_this_keyword(this_keyword);
        }
        Expression::SuperKeyword(super_keyword) => {
            visitor.visit_super_keyword(super_keyword);
        }
        Expression::TrueKeyword(true_keyword) => {
            visitor.visit_true_keyword(true_keyword);
        }
        Expression::FalseKeyword(false_keyword) => {
            visitor.visit_false_keyword(false_keyword);
        }
        Expression::Identifier(identifier) => {
            visitor.visit_identifier(identifier);
        }
    }
    visitor.leave_expression(node);
}

pub fn accept_for_statement_condition(node: &ForStatementCondition, visitor: &mut impl Visitor) {
    if !visitor.enter_for_statement_condition(node) {
        return;
    }
    match &node {
        ForStatementCondition::ExpressionStatement(expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
        ForStatementCondition::Semicolon(semicolon) => {
            visitor.visit_semicolon(semicolon);
        }
    }
    visitor.leave_for_statement_condition(node);
}

pub fn accept_for_statement_initialization(
    node: &ForStatementInitialization,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_for_statement_initialization(node) {
        return;
    }
    match &node {
        ForStatementInitialization::VariableDeclarationStatement(
            variable_declaration_statement,
        ) => {
            accept_variable_declaration_statement(variable_declaration_statement, visitor);
        }
        ForStatementInitialization::ExpressionStatement(expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
        ForStatementInitialization::Semicolon(semicolon) => {
            visitor.visit_semicolon(semicolon);
        }
    }
    visitor.leave_for_statement_initialization(node);
}

pub fn accept_function_kind(node: &FunctionKind, visitor: &mut impl Visitor) {
    if !visitor.enter_function_kind(node) {
        return;
    }
    match &node {
        FunctionKind::Regular => {}
        FunctionKind::Constructor => {}
        FunctionKind::Fallback => {}
        FunctionKind::Receive => {}
        FunctionKind::Modifier => {}
    }
    visitor.leave_function_kind(node);
}

pub fn accept_function_mutability(node: &FunctionMutability, visitor: &mut impl Visitor) {
    if !visitor.enter_function_mutability(node) {
        return;
    }
    match &node {
        FunctionMutability::Pure => {}
        FunctionMutability::View => {}
        FunctionMutability::NonPayable => {}
        FunctionMutability::Payable => {}
    }
    visitor.leave_function_mutability(node);
}

pub fn accept_function_visibility(node: &FunctionVisibility, visitor: &mut impl Visitor) {
    if !visitor.enter_function_visibility(node) {
        return;
    }
    match &node {
        FunctionVisibility::Public => {}
        FunctionVisibility::Private => {}
        FunctionVisibility::Internal => {}
        FunctionVisibility::External => {}
    }
    visitor.leave_function_visibility(node);
}

pub fn accept_import_clause(node: &ImportClause, visitor: &mut impl Visitor) {
    if !visitor.enter_import_clause(node) {
        return;
    }
    match &node {
        ImportClause::PathImport(path_import) => {
            accept_path_import(path_import, visitor);
        }
        ImportClause::ImportDeconstruction(import_deconstruction) => {
            accept_import_deconstruction(import_deconstruction, visitor);
        }
    }
    visitor.leave_import_clause(node);
}

pub fn accept_inequality_expression_operator(
    node: &InequalityExpressionOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_inequality_expression_operator(node) {
        return;
    }
    match &node {
        InequalityExpressionOperator::GreaterThan(greater_than) => {
            visitor.visit_greater_than(greater_than);
        }
        InequalityExpressionOperator::GreaterThanEqual(greater_than_equal) => {
            visitor.visit_greater_than_equal(greater_than_equal);
        }
        InequalityExpressionOperator::LessThan(less_than) => {
            visitor.visit_less_than(less_than);
        }
        InequalityExpressionOperator::LessThanEqual(less_than_equal) => {
            visitor.visit_less_than_equal(less_than_equal);
        }
    }
    visitor.leave_inequality_expression_operator(node);
}

pub fn accept_multiplicative_expression_operator(
    node: &MultiplicativeExpressionOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multiplicative_expression_operator(node) {
        return;
    }
    match &node {
        MultiplicativeExpressionOperator::Asterisk(asterisk) => {
            visitor.visit_asterisk(asterisk);
        }
        MultiplicativeExpressionOperator::Percent(percent) => {
            visitor.visit_percent(percent);
        }
        MultiplicativeExpressionOperator::Slash(slash) => {
            visitor.visit_slash(slash);
        }
    }
    visitor.leave_multiplicative_expression_operator(node);
}

pub fn accept_number_unit(node: &NumberUnit, visitor: &mut impl Visitor) {
    if !visitor.enter_number_unit(node) {
        return;
    }
    match &node {
        NumberUnit::WeiKeyword(wei_keyword) => {
            visitor.visit_wei_keyword(wei_keyword);
        }
        NumberUnit::GweiKeyword(gwei_keyword) => {
            visitor.visit_gwei_keyword(gwei_keyword);
        }
        NumberUnit::EtherKeyword(ether_keyword) => {
            visitor.visit_ether_keyword(ether_keyword);
        }
        NumberUnit::SecondsKeyword(seconds_keyword) => {
            visitor.visit_seconds_keyword(seconds_keyword);
        }
        NumberUnit::MinutesKeyword(minutes_keyword) => {
            visitor.visit_minutes_keyword(minutes_keyword);
        }
        NumberUnit::HoursKeyword(hours_keyword) => {
            visitor.visit_hours_keyword(hours_keyword);
        }
        NumberUnit::DaysKeyword(days_keyword) => {
            visitor.visit_days_keyword(days_keyword);
        }
        NumberUnit::WeeksKeyword(weeks_keyword) => {
            visitor.visit_weeks_keyword(weeks_keyword);
        }
    }
    visitor.leave_number_unit(node);
}

pub fn accept_postfix_expression_operator(
    node: &PostfixExpressionOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_postfix_expression_operator(node) {
        return;
    }
    match &node {
        PostfixExpressionOperator::MinusMinus(minus_minus) => {
            visitor.visit_minus_minus(minus_minus);
        }
        PostfixExpressionOperator::PlusPlus(plus_plus) => {
            visitor.visit_plus_plus(plus_plus);
        }
    }
    visitor.leave_postfix_expression_operator(node);
}

pub fn accept_pragma(node: &Pragma, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma(node) {
        return;
    }
    match &node {
        Pragma::VersionPragma(version_pragma) => {
            accept_version_pragma(version_pragma, visitor);
        }
        Pragma::AbicoderPragma(abicoder_pragma) => {
            accept_abicoder_pragma(abicoder_pragma, visitor);
        }
        Pragma::ExperimentalPragma(experimental_pragma) => {
            accept_experimental_pragma(experimental_pragma, visitor);
        }
    }
    visitor.leave_pragma(node);
}

pub fn accept_prefix_expression_operator(
    node: &PrefixExpressionOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_prefix_expression_operator(node) {
        return;
    }
    match &node {
        PrefixExpressionOperator::Bang(bang) => {
            visitor.visit_bang(bang);
        }
        PrefixExpressionOperator::DeleteKeyword(delete_keyword) => {
            visitor.visit_delete_keyword(delete_keyword);
        }
        PrefixExpressionOperator::Minus(minus) => {
            visitor.visit_minus(minus);
        }
        PrefixExpressionOperator::MinusMinus(minus_minus) => {
            visitor.visit_minus_minus(minus_minus);
        }
        PrefixExpressionOperator::PlusPlus(plus_plus) => {
            visitor.visit_plus_plus(plus_plus);
        }
        PrefixExpressionOperator::Tilde(tilde) => {
            visitor.visit_tilde(tilde);
        }
    }
    visitor.leave_prefix_expression_operator(node);
}

pub fn accept_shift_expression_operator(
    node: &ShiftExpressionOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_shift_expression_operator(node) {
        return;
    }
    match &node {
        ShiftExpressionOperator::GreaterThanGreaterThan(greater_than_greater_than) => {
            visitor.visit_greater_than_greater_than(greater_than_greater_than);
        }
        ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(
            greater_than_greater_than_greater_than,
        ) => {
            visitor.visit_greater_than_greater_than_greater_than(
                greater_than_greater_than_greater_than,
            );
        }
        ShiftExpressionOperator::LessThanLessThan(less_than_less_than) => {
            visitor.visit_less_than_less_than(less_than_less_than);
        }
    }
    visitor.leave_shift_expression_operator(node);
}

pub fn accept_source_unit_member(node: &SourceUnitMember, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit_member(node) {
        return;
    }
    match &node {
        SourceUnitMember::PragmaDirective(pragma_directive) => {
            accept_pragma_directive(pragma_directive, visitor);
        }
        SourceUnitMember::ImportClause(import_clause) => {
            accept_import_clause(import_clause, visitor);
        }
        SourceUnitMember::ContractDefinition(contract_definition) => {
            accept_contract_definition(contract_definition, visitor);
        }
        SourceUnitMember::InterfaceDefinition(interface_definition) => {
            accept_interface_definition(interface_definition, visitor);
        }
        SourceUnitMember::LibraryDefinition(library_definition) => {
            accept_library_definition(library_definition, visitor);
        }
        SourceUnitMember::StructDefinition(struct_definition) => {
            accept_struct_definition(struct_definition, visitor);
        }
        SourceUnitMember::EnumDefinition(enum_definition) => {
            accept_enum_definition(enum_definition, visitor);
        }
        SourceUnitMember::FunctionDefinition(function_definition) => {
            accept_function_definition(function_definition, visitor);
        }
        SourceUnitMember::ErrorDefinition(error_definition) => {
            accept_error_definition(error_definition, visitor);
        }
        SourceUnitMember::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
            accept_user_defined_value_type_definition(user_defined_value_type_definition, visitor);
        }
        SourceUnitMember::UsingDirective(using_directive) => {
            accept_using_directive(using_directive, visitor);
        }
        SourceUnitMember::EventDefinition(event_definition) => {
            accept_event_definition(event_definition, visitor);
        }
        SourceUnitMember::ConstantDefinition(constant_definition) => {
            accept_constant_definition(constant_definition, visitor);
        }
    }
    visitor.leave_source_unit_member(node);
}

pub fn accept_state_variable_mutability(
    node: &StateVariableMutability,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_mutability(node) {
        return;
    }
    match &node {
        StateVariableMutability::Mutable => {}
        StateVariableMutability::Constant => {}
        StateVariableMutability::Immutable => {}
        StateVariableMutability::Transient => {}
    }
    visitor.leave_state_variable_mutability(node);
}

pub fn accept_state_variable_visibility(
    node: &StateVariableVisibility,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_visibility(node) {
        return;
    }
    match &node {
        StateVariableVisibility::Public => {}
        StateVariableVisibility::Private => {}
        StateVariableVisibility::Internal => {}
    }
    visitor.leave_state_variable_visibility(node);
}

pub fn accept_statement(node: &Statement, visitor: &mut impl Visitor) {
    if !visitor.enter_statement(node) {
        return;
    }
    match &node {
        Statement::IfStatement(if_statement) => {
            accept_if_statement(if_statement, visitor);
        }
        Statement::ForStatement(for_statement) => {
            accept_for_statement(for_statement, visitor);
        }
        Statement::WhileStatement(while_statement) => {
            accept_while_statement(while_statement, visitor);
        }
        Statement::DoWhileStatement(do_while_statement) => {
            accept_do_while_statement(do_while_statement, visitor);
        }
        Statement::ContinueStatement(continue_statement) => {
            accept_continue_statement(continue_statement, visitor);
        }
        Statement::BreakStatement(break_statement) => {
            accept_break_statement(break_statement, visitor);
        }
        Statement::ReturnStatement(return_statement) => {
            accept_return_statement(return_statement, visitor);
        }
        Statement::EmitStatement(emit_statement) => {
            accept_emit_statement(emit_statement, visitor);
        }
        Statement::TryStatement(try_statement) => {
            accept_try_statement(try_statement, visitor);
        }
        Statement::RevertStatement(revert_statement) => {
            accept_revert_statement(revert_statement, visitor);
        }
        Statement::AssemblyStatement(assembly_statement) => {
            accept_assembly_statement(assembly_statement, visitor);
        }
        Statement::Block(block) => {
            accept_block(block, visitor);
        }
        Statement::UncheckedBlock(unchecked_block) => {
            accept_unchecked_block(unchecked_block, visitor);
        }
        Statement::VariableDeclarationStatement(variable_declaration_statement) => {
            accept_variable_declaration_statement(variable_declaration_statement, visitor);
        }
        Statement::ExpressionStatement(expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
    }
    visitor.leave_statement(node);
}

pub fn accept_storage_location(node: &StorageLocation, visitor: &mut impl Visitor) {
    if !visitor.enter_storage_location(node) {
        return;
    }
    match &node {
        StorageLocation::MemoryKeyword(memory_keyword) => {
            visitor.visit_memory_keyword(memory_keyword);
        }
        StorageLocation::StorageKeyword(storage_keyword) => {
            visitor.visit_storage_keyword(storage_keyword);
        }
        StorageLocation::CallDataKeyword(call_data_keyword) => {
            visitor.visit_call_data_keyword(call_data_keyword);
        }
    }
    visitor.leave_storage_location(node);
}

pub fn accept_string_expression(node: &StringExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_string_expression(node) {
        return;
    }
    match &node {
        StringExpression::StringLiterals(string_literals) => {
            accept_string_literals(string_literals, visitor);
        }
        StringExpression::HexStringLiterals(hex_string_literals) => {
            accept_hex_string_literals(hex_string_literals, visitor);
        }
        StringExpression::UnicodeStringLiterals(unicode_string_literals) => {
            accept_unicode_string_literals(unicode_string_literals, visitor);
        }
    }
    visitor.leave_string_expression(node);
}

pub fn accept_type_name(node: &TypeName, visitor: &mut impl Visitor) {
    if !visitor.enter_type_name(node) {
        return;
    }
    match &node {
        TypeName::ArrayTypeName(array_type_name) => {
            accept_array_type_name(array_type_name, visitor);
        }
        TypeName::FunctionType(function_type) => {
            accept_function_type(function_type, visitor);
        }
        TypeName::MappingType(mapping_type) => {
            accept_mapping_type(mapping_type, visitor);
        }
        TypeName::ElementaryType(elementary_type) => {
            accept_elementary_type(elementary_type, visitor);
        }
        TypeName::IdentifierPath(identifier_path) => {
            accept_identifier_path(identifier_path, visitor);
        }
    }
    visitor.leave_type_name(node);
}

pub fn accept_using_clause(node: &UsingClause, visitor: &mut impl Visitor) {
    if !visitor.enter_using_clause(node) {
        return;
    }
    match &node {
        UsingClause::IdentifierPath(identifier_path) => {
            accept_identifier_path(identifier_path, visitor);
        }
        UsingClause::UsingDeconstruction(using_deconstruction) => {
            accept_using_deconstruction(using_deconstruction, visitor);
        }
    }
    visitor.leave_using_clause(node);
}

pub fn accept_using_operator(node: &UsingOperator, visitor: &mut impl Visitor) {
    if !visitor.enter_using_operator(node) {
        return;
    }
    match &node {
        UsingOperator::Ampersand(ampersand) => {
            visitor.visit_ampersand(ampersand);
        }
        UsingOperator::Asterisk(asterisk) => {
            visitor.visit_asterisk(asterisk);
        }
        UsingOperator::BangEqual(bang_equal) => {
            visitor.visit_bang_equal(bang_equal);
        }
        UsingOperator::Bar(bar) => {
            visitor.visit_bar(bar);
        }
        UsingOperator::Caret(caret) => {
            visitor.visit_caret(caret);
        }
        UsingOperator::EqualEqual(equal_equal) => {
            visitor.visit_equal_equal(equal_equal);
        }
        UsingOperator::GreaterThan(greater_than) => {
            visitor.visit_greater_than(greater_than);
        }
        UsingOperator::GreaterThanEqual(greater_than_equal) => {
            visitor.visit_greater_than_equal(greater_than_equal);
        }
        UsingOperator::LessThan(less_than) => {
            visitor.visit_less_than(less_than);
        }
        UsingOperator::LessThanEqual(less_than_equal) => {
            visitor.visit_less_than_equal(less_than_equal);
        }
        UsingOperator::Minus(minus) => {
            visitor.visit_minus(minus);
        }
        UsingOperator::Percent(percent) => {
            visitor.visit_percent(percent);
        }
        UsingOperator::Plus(plus) => {
            visitor.visit_plus(plus);
        }
        UsingOperator::Slash(slash) => {
            visitor.visit_slash(slash);
        }
        UsingOperator::Tilde(tilde) => {
            visitor.visit_tilde(tilde);
        }
    }
    visitor.leave_using_operator(node);
}

pub fn accept_using_target(node: &UsingTarget, visitor: &mut impl Visitor) {
    if !visitor.enter_using_target(node) {
        return;
    }
    match &node {
        UsingTarget::TypeName(type_name) => {
            accept_type_name(type_name, visitor);
        }
        UsingTarget::Asterisk(asterisk) => {
            visitor.visit_asterisk(asterisk);
        }
    }
    visitor.leave_using_target(node);
}

pub fn accept_variable_declaration_target(
    node: &VariableDeclarationTarget,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_target(node) {
        return;
    }
    match &node {
        VariableDeclarationTarget::SingleTypedDeclaration(single_typed_declaration) => {
            accept_single_typed_declaration(single_typed_declaration, visitor);
        }
        VariableDeclarationTarget::MultiTypedDeclaration(multi_typed_declaration) => {
            accept_multi_typed_declaration(multi_typed_declaration, visitor);
        }
    }
    visitor.leave_variable_declaration_target(node);
}

pub fn accept_version_expression(node: &VersionExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression(node) {
        return;
    }
    match &node {
        VersionExpression::VersionRange(version_range) => {
            accept_version_range(version_range, visitor);
        }
        VersionExpression::VersionTerm(version_term) => {
            accept_version_term(version_term, visitor);
        }
    }
    visitor.leave_version_expression(node);
}

pub fn accept_version_literal(node: &VersionLiteral, visitor: &mut impl Visitor) {
    if !visitor.enter_version_literal(node) {
        return;
    }
    match &node {
        VersionLiteral::SimpleVersionLiteral(simple_version_literal) => {
            accept_simple_version_literal(simple_version_literal, visitor);
        }
        VersionLiteral::StringLiteral(string_literal) => {
            visitor.visit_string_literal(string_literal);
        }
    }
    visitor.leave_version_literal(node);
}

pub fn accept_version_operator(node: &VersionOperator, visitor: &mut impl Visitor) {
    if !visitor.enter_version_operator(node) {
        return;
    }
    match &node {
        VersionOperator::PragmaCaret(pragma_caret) => {
            visitor.visit_pragma_caret(pragma_caret);
        }
        VersionOperator::PragmaTilde(pragma_tilde) => {
            visitor.visit_pragma_tilde(pragma_tilde);
        }
        VersionOperator::PragmaEqual(pragma_equal) => {
            visitor.visit_pragma_equal(pragma_equal);
        }
        VersionOperator::PragmaLessThan(pragma_less_than) => {
            visitor.visit_pragma_less_than(pragma_less_than);
        }
        VersionOperator::PragmaGreaterThan(pragma_greater_than) => {
            visitor.visit_pragma_greater_than(pragma_greater_than);
        }
        VersionOperator::PragmaLessThanEqual(pragma_less_than_equal) => {
            visitor.visit_pragma_less_than_equal(pragma_less_than_equal);
        }
        VersionOperator::PragmaGreaterThanEqual(pragma_greater_than_equal) => {
            visitor.visit_pragma_greater_than_equal(pragma_greater_than_equal);
        }
    }
    visitor.leave_version_operator(node);
}

pub fn accept_yul_expression(node: &YulExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_expression(node) {
        return;
    }
    match &node {
        YulExpression::YulFunctionCallExpression(yul_function_call_expression) => {
            accept_yul_function_call_expression(yul_function_call_expression, visitor);
        }
        YulExpression::YulLiteral(yul_literal) => {
            accept_yul_literal(yul_literal, visitor);
        }
        YulExpression::YulPath(yul_path) => {
            accept_yul_path(yul_path, visitor);
        }
    }
    visitor.leave_yul_expression(node);
}

pub fn accept_yul_literal(node: &YulLiteral, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_literal(node) {
        return;
    }
    match &node {
        YulLiteral::TrueKeyword(true_keyword) => {
            visitor.visit_true_keyword(true_keyword);
        }
        YulLiteral::FalseKeyword(false_keyword) => {
            visitor.visit_false_keyword(false_keyword);
        }
        YulLiteral::DecimalLiteral(decimal_literal) => {
            visitor.visit_decimal_literal(decimal_literal);
        }
        YulLiteral::HexLiteral(hex_literal) => {
            visitor.visit_hex_literal(hex_literal);
        }
        YulLiteral::HexStringLiteral(hex_string_literal) => {
            visitor.visit_hex_string_literal(hex_string_literal);
        }
        YulLiteral::StringLiteral(string_literal) => {
            visitor.visit_string_literal(string_literal);
        }
    }
    visitor.leave_yul_literal(node);
}

pub fn accept_yul_statement(node: &YulStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_statement(node) {
        return;
    }
    match &node {
        YulStatement::YulBlock(yul_block) => {
            accept_yul_block(yul_block, visitor);
        }
        YulStatement::YulFunctionDefinition(yul_function_definition) => {
            accept_yul_function_definition(yul_function_definition, visitor);
        }
        YulStatement::YulIfStatement(yul_if_statement) => {
            accept_yul_if_statement(yul_if_statement, visitor);
        }
        YulStatement::YulForStatement(yul_for_statement) => {
            accept_yul_for_statement(yul_for_statement, visitor);
        }
        YulStatement::YulSwitchStatement(yul_switch_statement) => {
            accept_yul_switch_statement(yul_switch_statement, visitor);
        }
        YulStatement::YulLeaveStatement(yul_leave_statement) => {
            accept_yul_leave_statement(yul_leave_statement, visitor);
        }
        YulStatement::YulBreakStatement(yul_break_statement) => {
            accept_yul_break_statement(yul_break_statement, visitor);
        }
        YulStatement::YulContinueStatement(yul_continue_statement) => {
            accept_yul_continue_statement(yul_continue_statement, visitor);
        }
        YulStatement::YulVariableAssignmentStatement(yul_variable_assignment_statement) => {
            accept_yul_variable_assignment_statement(yul_variable_assignment_statement, visitor);
        }
        YulStatement::YulVariableDeclarationStatement(yul_variable_declaration_statement) => {
            accept_yul_variable_declaration_statement(yul_variable_declaration_statement, visitor);
        }
        YulStatement::YulExpression(yul_expression) => {
            accept_yul_expression(yul_expression, visitor);
        }
    }
    visitor.leave_yul_statement(node);
}

pub fn accept_yul_switch_case(node: &YulSwitchCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_case(node) {
        return;
    }
    match &node {
        YulSwitchCase::YulDefaultCase(yul_default_case) => {
            accept_yul_default_case(yul_default_case, visitor);
        }
        YulSwitchCase::YulValueCase(yul_value_case) => {
            accept_yul_value_case(yul_value_case, visitor);
        }
    }
    visitor.leave_yul_switch_case(node);
}

//
// Repeated & Separated
//

#[inline]
fn accept_array_values(items: &Vec<Expression>, visitor: &mut impl Visitor) {
    if !visitor.enter_array_values(items) {
        return;
    }

    for item in items {
        accept_expression(item, visitor);
    }

    visitor.leave_array_values(items);
}
#[inline]
fn accept_call_options(items: &Vec<NamedArgument>, visitor: &mut impl Visitor) {
    if !visitor.enter_call_options(items) {
        return;
    }

    for item in items {
        accept_named_argument(item, visitor);
    }

    visitor.leave_call_options(items);
}
#[inline]
fn accept_catch_clauses(items: &Vec<CatchClause>, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clauses(items) {
        return;
    }

    for item in items {
        accept_catch_clause(item, visitor);
    }

    visitor.leave_catch_clauses(items);
}
#[inline]
fn accept_contract_members(items: &Vec<ContractMember>, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_members(items) {
        return;
    }

    for item in items {
        accept_contract_member(item, visitor);
    }

    visitor.leave_contract_members(items);
}
#[inline]
fn accept_enum_members(items: &Vec<Identifier>, visitor: &mut impl Visitor) {
    if !visitor.enter_enum_members(items) {
        return;
    }

    for item in items {
        visitor.visit_identifier(item);
    }

    visitor.leave_enum_members(items);
}
#[inline]
fn accept_hex_string_literals(items: &Vec<HexStringLiteral>, visitor: &mut impl Visitor) {
    if !visitor.enter_hex_string_literals(items) {
        return;
    }

    for item in items {
        visitor.visit_hex_string_literal(item);
    }

    visitor.leave_hex_string_literals(items);
}
#[inline]
fn accept_identifier_path(items: &Vec<Identifier>, visitor: &mut impl Visitor) {
    if !visitor.enter_identifier_path(items) {
        return;
    }

    for item in items {
        visitor.visit_identifier(item);
    }

    visitor.leave_identifier_path(items);
}
#[inline]
fn accept_import_deconstruction_symbols(
    items: &Vec<ImportDeconstructionSymbol>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_import_deconstruction_symbols(items) {
        return;
    }

    for item in items {
        accept_import_deconstruction_symbol(item, visitor);
    }

    visitor.leave_import_deconstruction_symbols(items);
}
#[inline]
fn accept_inheritance_types(items: &Vec<InheritanceType>, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_types(items) {
        return;
    }

    for item in items {
        accept_inheritance_type(item, visitor);
    }

    visitor.leave_inheritance_types(items);
}
#[inline]
fn accept_interface_members(items: &Vec<ContractMember>, visitor: &mut impl Visitor) {
    if !visitor.enter_interface_members(items) {
        return;
    }

    for item in items {
        accept_contract_member(item, visitor);
    }

    visitor.leave_interface_members(items);
}
#[inline]
fn accept_library_members(items: &Vec<ContractMember>, visitor: &mut impl Visitor) {
    if !visitor.enter_library_members(items) {
        return;
    }

    for item in items {
        accept_contract_member(item, visitor);
    }

    visitor.leave_library_members(items);
}
#[inline]
fn accept_modifier_invocations(items: &Vec<ModifierInvocation>, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_invocations(items) {
        return;
    }

    for item in items {
        accept_modifier_invocation(item, visitor);
    }

    visitor.leave_modifier_invocations(items);
}
#[inline]
fn accept_multi_typed_declaration_elements(
    items: &Vec<MultiTypedDeclarationElement>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multi_typed_declaration_elements(items) {
        return;
    }

    for item in items {
        accept_multi_typed_declaration_element(item, visitor);
    }

    visitor.leave_multi_typed_declaration_elements(items);
}
#[inline]
fn accept_named_arguments(items: &Vec<NamedArgument>, visitor: &mut impl Visitor) {
    if !visitor.enter_named_arguments(items) {
        return;
    }

    for item in items {
        accept_named_argument(item, visitor);
    }

    visitor.leave_named_arguments(items);
}
#[inline]
fn accept_override_paths(items: &Vec<IdentifierPath>, visitor: &mut impl Visitor) {
    if !visitor.enter_override_paths(items) {
        return;
    }

    for item in items {
        accept_identifier_path(item, visitor);
    }

    visitor.leave_override_paths(items);
}
#[inline]
fn accept_parameters(items: &Vec<Parameter>, visitor: &mut impl Visitor) {
    if !visitor.enter_parameters(items) {
        return;
    }

    for item in items {
        accept_parameter(item, visitor);
    }

    visitor.leave_parameters(items);
}
#[inline]
fn accept_positional_arguments(items: &Vec<Expression>, visitor: &mut impl Visitor) {
    if !visitor.enter_positional_arguments(items) {
        return;
    }

    for item in items {
        accept_expression(item, visitor);
    }

    visitor.leave_positional_arguments(items);
}
#[inline]
fn accept_simple_version_literal(items: &Vec<VersionSpecifier>, visitor: &mut impl Visitor) {
    if !visitor.enter_simple_version_literal(items) {
        return;
    }

    for item in items {
        visitor.visit_version_specifier(item);
    }

    visitor.leave_simple_version_literal(items);
}
#[inline]
fn accept_source_unit_members(items: &Vec<SourceUnitMember>, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit_members(items) {
        return;
    }

    for item in items {
        accept_source_unit_member(item, visitor);
    }

    visitor.leave_source_unit_members(items);
}
#[inline]
fn accept_statements(items: &Vec<Statement>, visitor: &mut impl Visitor) {
    if !visitor.enter_statements(items) {
        return;
    }

    for item in items {
        accept_statement(item, visitor);
    }

    visitor.leave_statements(items);
}
#[inline]
fn accept_string_literals(items: &Vec<StringLiteral>, visitor: &mut impl Visitor) {
    if !visitor.enter_string_literals(items) {
        return;
    }

    for item in items {
        visitor.visit_string_literal(item);
    }

    visitor.leave_string_literals(items);
}
#[inline]
fn accept_struct_members(items: &Vec<StructMember>, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_members(items) {
        return;
    }

    for item in items {
        accept_struct_member(item, visitor);
    }

    visitor.leave_struct_members(items);
}
#[inline]
fn accept_tuple_values(items: &Vec<TupleValue>, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_values(items) {
        return;
    }

    for item in items {
        accept_tuple_value(item, visitor);
    }

    visitor.leave_tuple_values(items);
}
#[inline]
fn accept_unicode_string_literals(items: &Vec<UnicodeStringLiteral>, visitor: &mut impl Visitor) {
    if !visitor.enter_unicode_string_literals(items) {
        return;
    }

    for item in items {
        visitor.visit_unicode_string_literal(item);
    }

    visitor.leave_unicode_string_literals(items);
}
#[inline]
fn accept_using_deconstruction_symbols(
    items: &Vec<UsingDeconstructionSymbol>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_using_deconstruction_symbols(items) {
        return;
    }

    for item in items {
        accept_using_deconstruction_symbol(item, visitor);
    }

    visitor.leave_using_deconstruction_symbols(items);
}
#[inline]
fn accept_version_expression_set(items: &Vec<VersionExpression>, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression_set(items) {
        return;
    }

    for item in items {
        accept_version_expression(item, visitor);
    }

    visitor.leave_version_expression_set(items);
}
#[inline]
fn accept_version_expression_sets(items: &Vec<VersionExpressionSet>, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression_sets(items) {
        return;
    }

    for item in items {
        accept_version_expression_set(item, visitor);
    }

    visitor.leave_version_expression_sets(items);
}
#[inline]
fn accept_yul_arguments(items: &Vec<YulExpression>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_arguments(items) {
        return;
    }

    for item in items {
        accept_yul_expression(item, visitor);
    }

    visitor.leave_yul_arguments(items);
}
#[inline]
fn accept_yul_flags(items: &Vec<StringLiteral>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_flags(items) {
        return;
    }

    for item in items {
        visitor.visit_string_literal(item);
    }

    visitor.leave_yul_flags(items);
}
#[inline]
fn accept_yul_parameters(items: &Vec<Identifier>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_parameters(items) {
        return;
    }

    for item in items {
        visitor.visit_identifier(item);
    }

    visitor.leave_yul_parameters(items);
}
#[inline]
fn accept_yul_path(items: &Vec<Identifier>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_path(items) {
        return;
    }

    for item in items {
        visitor.visit_identifier(item);
    }

    visitor.leave_yul_path(items);
}
#[inline]
fn accept_yul_paths(items: &Vec<YulPath>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_paths(items) {
        return;
    }

    for item in items {
        accept_yul_path(item, visitor);
    }

    visitor.leave_yul_paths(items);
}
#[inline]
fn accept_yul_statements(items: &Vec<YulStatement>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_statements(items) {
        return;
    }

    for item in items {
        accept_yul_statement(item, visitor);
    }

    visitor.leave_yul_statements(items);
}
#[inline]
fn accept_yul_switch_cases(items: &Vec<YulSwitchCase>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_cases(items) {
        return;
    }

    for item in items {
        accept_yul_switch_case(item, visitor);
    }

    visitor.leave_yul_switch_cases(items);
}
#[inline]
fn accept_yul_variable_names(items: &Vec<Identifier>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_variable_names(items) {
        return;
    }

    for item in items {
        visitor.visit_identifier(item);
    }

    visitor.leave_yul_variable_names(items);
}
