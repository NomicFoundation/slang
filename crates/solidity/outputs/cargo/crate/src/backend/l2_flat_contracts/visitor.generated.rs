// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::TerminalNode;

pub trait Visitor {
    fn enter_source_unit(&mut self, _node: &SourceUnit) -> bool {
        true
    }
    fn leave_source_unit(&mut self, _node: &SourceUnit) {}

    fn enter_pragma_directive(&mut self, _node: &PragmaDirective) -> bool {
        true
    }
    fn leave_pragma_directive(&mut self, _node: &PragmaDirective) {}

    fn enter_abicoder_pragma(&mut self, _node: &AbicoderPragma) -> bool {
        true
    }
    fn leave_abicoder_pragma(&mut self, _node: &AbicoderPragma) {}

    fn enter_experimental_pragma(&mut self, _node: &ExperimentalPragma) -> bool {
        true
    }
    fn leave_experimental_pragma(&mut self, _node: &ExperimentalPragma) {}

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

    fn enter_import_directive(&mut self, _node: &ImportDirective) -> bool {
        true
    }
    fn leave_import_directive(&mut self, _node: &ImportDirective) {}

    fn enter_path_import(&mut self, _node: &PathImport) -> bool {
        true
    }
    fn leave_path_import(&mut self, _node: &PathImport) {}

    fn enter_named_import(&mut self, _node: &NamedImport) -> bool {
        true
    }
    fn leave_named_import(&mut self, _node: &NamedImport) {}

    fn enter_import_deconstruction(&mut self, _node: &ImportDeconstruction) -> bool {
        true
    }
    fn leave_import_deconstruction(&mut self, _node: &ImportDeconstruction) {}

    fn enter_import_deconstruction_symbol(&mut self, _node: &ImportDeconstructionSymbol) -> bool {
        true
    }
    fn leave_import_deconstruction_symbol(&mut self, _node: &ImportDeconstructionSymbol) {}

    fn enter_using_directive(&mut self, _node: &UsingDirective) -> bool {
        true
    }
    fn leave_using_directive(&mut self, _node: &UsingDirective) {}

    fn enter_using_deconstruction(&mut self, _node: &UsingDeconstruction) -> bool {
        true
    }
    fn leave_using_deconstruction(&mut self, _node: &UsingDeconstruction) {}

    fn enter_using_deconstruction_symbol(&mut self, _node: &UsingDeconstructionSymbol) -> bool {
        true
    }
    fn leave_using_deconstruction_symbol(&mut self, _node: &UsingDeconstructionSymbol) {}

    fn enter_contract_definition(&mut self, _node: &ContractDefinition) -> bool {
        true
    }
    fn leave_contract_definition(&mut self, _node: &ContractDefinition) {}

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

    fn enter_struct_definition(&mut self, _node: &StructDefinition) -> bool {
        true
    }
    fn leave_struct_definition(&mut self, _node: &StructDefinition) {}

    fn enter_struct_member(&mut self, _node: &StructMember) -> bool {
        true
    }
    fn leave_struct_member(&mut self, _node: &StructMember) {}

    fn enter_enum_definition(&mut self, _node: &EnumDefinition) -> bool {
        true
    }
    fn leave_enum_definition(&mut self, _node: &EnumDefinition) {}

    fn enter_constant_definition(&mut self, _node: &ConstantDefinition) -> bool {
        true
    }
    fn leave_constant_definition(&mut self, _node: &ConstantDefinition) {}

    fn enter_state_variable_definition(&mut self, _node: &StateVariableDefinition) -> bool {
        true
    }
    fn leave_state_variable_definition(&mut self, _node: &StateVariableDefinition) {}

    fn enter_function_definition(&mut self, _node: &FunctionDefinition) -> bool {
        true
    }
    fn leave_function_definition(&mut self, _node: &FunctionDefinition) {}

    fn enter_parameter(&mut self, _node: &Parameter) -> bool {
        true
    }
    fn leave_parameter(&mut self, _node: &Parameter) {}

    fn enter_override_specifier(&mut self, _node: &OverrideSpecifier) -> bool {
        true
    }
    fn leave_override_specifier(&mut self, _node: &OverrideSpecifier) {}

    fn enter_modifier_invocation(&mut self, _node: &ModifierInvocation) -> bool {
        true
    }
    fn leave_modifier_invocation(&mut self, _node: &ModifierInvocation) {}

    fn enter_event_definition(&mut self, _node: &EventDefinition) -> bool {
        true
    }
    fn leave_event_definition(&mut self, _node: &EventDefinition) {}

    fn enter_event_parameter(&mut self, _node: &EventParameter) -> bool {
        true
    }
    fn leave_event_parameter(&mut self, _node: &EventParameter) {}

    fn enter_user_defined_value_type_definition(
        &mut self,
        _node: &UserDefinedValueTypeDefinition,
    ) -> bool {
        true
    }
    fn leave_user_defined_value_type_definition(&mut self, _node: &UserDefinedValueTypeDefinition) {
    }

    fn enter_error_definition(&mut self, _node: &ErrorDefinition) -> bool {
        true
    }
    fn leave_error_definition(&mut self, _node: &ErrorDefinition) {}

    fn enter_error_parameter(&mut self, _node: &ErrorParameter) -> bool {
        true
    }
    fn leave_error_parameter(&mut self, _node: &ErrorParameter) {}

    fn enter_array_type_name(&mut self, _node: &ArrayTypeName) -> bool {
        true
    }
    fn leave_array_type_name(&mut self, _node: &ArrayTypeName) {}

    fn enter_function_type(&mut self, _node: &FunctionType) -> bool {
        true
    }
    fn leave_function_type(&mut self, _node: &FunctionType) {}

    fn enter_mapping_type(&mut self, _node: &MappingType) -> bool {
        true
    }
    fn leave_mapping_type(&mut self, _node: &MappingType) {}

    fn enter_mapping_key(&mut self, _node: &MappingKey) -> bool {
        true
    }
    fn leave_mapping_key(&mut self, _node: &MappingKey) {}

    fn enter_mapping_value(&mut self, _node: &MappingValue) -> bool {
        true
    }
    fn leave_mapping_value(&mut self, _node: &MappingValue) {}

    fn enter_address_type(&mut self, _node: &AddressType) -> bool {
        true
    }
    fn leave_address_type(&mut self, _node: &AddressType) {}

    fn enter_block(&mut self, _node: &Block) -> bool {
        true
    }
    fn leave_block(&mut self, _node: &Block) {}

    fn enter_unchecked_block(&mut self, _node: &UncheckedBlock) -> bool {
        true
    }
    fn leave_unchecked_block(&mut self, _node: &UncheckedBlock) {}

    fn enter_expression_statement(&mut self, _node: &ExpressionStatement) -> bool {
        true
    }
    fn leave_expression_statement(&mut self, _node: &ExpressionStatement) {}

    fn enter_assembly_statement(&mut self, _node: &AssemblyStatement) -> bool {
        true
    }
    fn leave_assembly_statement(&mut self, _node: &AssemblyStatement) {}

    fn enter_tuple_deconstruction_statement(
        &mut self,
        _node: &TupleDeconstructionStatement,
    ) -> bool {
        true
    }
    fn leave_tuple_deconstruction_statement(&mut self, _node: &TupleDeconstructionStatement) {}

    fn enter_tuple_deconstruction_element(&mut self, _node: &TupleDeconstructionElement) -> bool {
        true
    }
    fn leave_tuple_deconstruction_element(&mut self, _node: &TupleDeconstructionElement) {}

    fn enter_typed_tuple_member(&mut self, _node: &TypedTupleMember) -> bool {
        true
    }
    fn leave_typed_tuple_member(&mut self, _node: &TypedTupleMember) {}

    fn enter_untyped_tuple_member(&mut self, _node: &UntypedTupleMember) -> bool {
        true
    }
    fn leave_untyped_tuple_member(&mut self, _node: &UntypedTupleMember) {}

    fn enter_variable_declaration_statement(
        &mut self,
        _node: &VariableDeclarationStatement,
    ) -> bool {
        true
    }
    fn leave_variable_declaration_statement(&mut self, _node: &VariableDeclarationStatement) {}

    fn enter_if_statement(&mut self, _node: &IfStatement) -> bool {
        true
    }
    fn leave_if_statement(&mut self, _node: &IfStatement) {}

    fn enter_for_statement(&mut self, _node: &ForStatement) -> bool {
        true
    }
    fn leave_for_statement(&mut self, _node: &ForStatement) {}

    fn enter_while_statement(&mut self, _node: &WhileStatement) -> bool {
        true
    }
    fn leave_while_statement(&mut self, _node: &WhileStatement) {}

    fn enter_do_while_statement(&mut self, _node: &DoWhileStatement) -> bool {
        true
    }
    fn leave_do_while_statement(&mut self, _node: &DoWhileStatement) {}

    fn enter_continue_statement(&mut self, _node: &ContinueStatement) -> bool {
        true
    }
    fn leave_continue_statement(&mut self, _node: &ContinueStatement) {}

    fn enter_break_statement(&mut self, _node: &BreakStatement) -> bool {
        true
    }
    fn leave_break_statement(&mut self, _node: &BreakStatement) {}

    fn enter_return_statement(&mut self, _node: &ReturnStatement) -> bool {
        true
    }
    fn leave_return_statement(&mut self, _node: &ReturnStatement) {}

    fn enter_emit_statement(&mut self, _node: &EmitStatement) -> bool {
        true
    }
    fn leave_emit_statement(&mut self, _node: &EmitStatement) {}

    fn enter_try_statement(&mut self, _node: &TryStatement) -> bool {
        true
    }
    fn leave_try_statement(&mut self, _node: &TryStatement) {}

    fn enter_catch_clause(&mut self, _node: &CatchClause) -> bool {
        true
    }
    fn leave_catch_clause(&mut self, _node: &CatchClause) {}

    fn enter_catch_clause_error(&mut self, _node: &CatchClauseError) -> bool {
        true
    }
    fn leave_catch_clause_error(&mut self, _node: &CatchClauseError) {}

    fn enter_revert_statement(&mut self, _node: &RevertStatement) -> bool {
        true
    }
    fn leave_revert_statement(&mut self, _node: &RevertStatement) {}

    fn enter_throw_statement(&mut self, _node: &ThrowStatement) -> bool {
        true
    }
    fn leave_throw_statement(&mut self, _node: &ThrowStatement) {}

    fn enter_assignment_expression(&mut self, _node: &AssignmentExpression) -> bool {
        true
    }
    fn leave_assignment_expression(&mut self, _node: &AssignmentExpression) {}

    fn enter_conditional_expression(&mut self, _node: &ConditionalExpression) -> bool {
        true
    }
    fn leave_conditional_expression(&mut self, _node: &ConditionalExpression) {}

    fn enter_or_expression(&mut self, _node: &OrExpression) -> bool {
        true
    }
    fn leave_or_expression(&mut self, _node: &OrExpression) {}

    fn enter_and_expression(&mut self, _node: &AndExpression) -> bool {
        true
    }
    fn leave_and_expression(&mut self, _node: &AndExpression) {}

    fn enter_equality_expression(&mut self, _node: &EqualityExpression) -> bool {
        true
    }
    fn leave_equality_expression(&mut self, _node: &EqualityExpression) {}

    fn enter_inequality_expression(&mut self, _node: &InequalityExpression) -> bool {
        true
    }
    fn leave_inequality_expression(&mut self, _node: &InequalityExpression) {}

    fn enter_bitwise_or_expression(&mut self, _node: &BitwiseOrExpression) -> bool {
        true
    }
    fn leave_bitwise_or_expression(&mut self, _node: &BitwiseOrExpression) {}

    fn enter_bitwise_xor_expression(&mut self, _node: &BitwiseXorExpression) -> bool {
        true
    }
    fn leave_bitwise_xor_expression(&mut self, _node: &BitwiseXorExpression) {}

    fn enter_bitwise_and_expression(&mut self, _node: &BitwiseAndExpression) -> bool {
        true
    }
    fn leave_bitwise_and_expression(&mut self, _node: &BitwiseAndExpression) {}

    fn enter_shift_expression(&mut self, _node: &ShiftExpression) -> bool {
        true
    }
    fn leave_shift_expression(&mut self, _node: &ShiftExpression) {}

    fn enter_additive_expression(&mut self, _node: &AdditiveExpression) -> bool {
        true
    }
    fn leave_additive_expression(&mut self, _node: &AdditiveExpression) {}

    fn enter_multiplicative_expression(&mut self, _node: &MultiplicativeExpression) -> bool {
        true
    }
    fn leave_multiplicative_expression(&mut self, _node: &MultiplicativeExpression) {}

    fn enter_exponentiation_expression(&mut self, _node: &ExponentiationExpression) -> bool {
        true
    }
    fn leave_exponentiation_expression(&mut self, _node: &ExponentiationExpression) {}

    fn enter_postfix_expression(&mut self, _node: &PostfixExpression) -> bool {
        true
    }
    fn leave_postfix_expression(&mut self, _node: &PostfixExpression) {}

    fn enter_prefix_expression(&mut self, _node: &PrefixExpression) -> bool {
        true
    }
    fn leave_prefix_expression(&mut self, _node: &PrefixExpression) {}

    fn enter_function_call_expression(&mut self, _node: &FunctionCallExpression) -> bool {
        true
    }
    fn leave_function_call_expression(&mut self, _node: &FunctionCallExpression) {}

    fn enter_call_options_expression(&mut self, _node: &CallOptionsExpression) -> bool {
        true
    }
    fn leave_call_options_expression(&mut self, _node: &CallOptionsExpression) {}

    fn enter_member_access_expression(&mut self, _node: &MemberAccessExpression) -> bool {
        true
    }
    fn leave_member_access_expression(&mut self, _node: &MemberAccessExpression) {}

    fn enter_index_access_expression(&mut self, _node: &IndexAccessExpression) -> bool {
        true
    }
    fn leave_index_access_expression(&mut self, _node: &IndexAccessExpression) {}

    fn enter_named_argument(&mut self, _node: &NamedArgument) -> bool {
        true
    }
    fn leave_named_argument(&mut self, _node: &NamedArgument) {}

    fn enter_type_expression(&mut self, _node: &TypeExpression) -> bool {
        true
    }
    fn leave_type_expression(&mut self, _node: &TypeExpression) {}

    fn enter_new_expression(&mut self, _node: &NewExpression) -> bool {
        true
    }
    fn leave_new_expression(&mut self, _node: &NewExpression) {}

    fn enter_tuple_expression(&mut self, _node: &TupleExpression) -> bool {
        true
    }
    fn leave_tuple_expression(&mut self, _node: &TupleExpression) {}

    fn enter_tuple_value(&mut self, _node: &TupleValue) -> bool {
        true
    }
    fn leave_tuple_value(&mut self, _node: &TupleValue) {}

    fn enter_array_expression(&mut self, _node: &ArrayExpression) -> bool {
        true
    }
    fn leave_array_expression(&mut self, _node: &ArrayExpression) {}

    fn enter_hex_number_expression(&mut self, _node: &HexNumberExpression) -> bool {
        true
    }
    fn leave_hex_number_expression(&mut self, _node: &HexNumberExpression) {}

    fn enter_decimal_number_expression(&mut self, _node: &DecimalNumberExpression) -> bool {
        true
    }
    fn leave_decimal_number_expression(&mut self, _node: &DecimalNumberExpression) {}

    fn enter_yul_block(&mut self, _node: &YulBlock) -> bool {
        true
    }
    fn leave_yul_block(&mut self, _node: &YulBlock) {}

    fn enter_yul_function_definition(&mut self, _node: &YulFunctionDefinition) -> bool {
        true
    }
    fn leave_yul_function_definition(&mut self, _node: &YulFunctionDefinition) {}

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

    fn enter_yul_variable_assignment_statement(
        &mut self,
        _node: &YulVariableAssignmentStatement,
    ) -> bool {
        true
    }
    fn leave_yul_variable_assignment_statement(&mut self, _node: &YulVariableAssignmentStatement) {}

    fn enter_yul_colon_and_equal(&mut self, _node: &YulColonAndEqual) -> bool {
        true
    }
    fn leave_yul_colon_and_equal(&mut self, _node: &YulColonAndEqual) {}

    fn enter_yul_stack_assignment_statement(
        &mut self,
        _node: &YulStackAssignmentStatement,
    ) -> bool {
        true
    }
    fn leave_yul_stack_assignment_statement(&mut self, _node: &YulStackAssignmentStatement) {}

    fn enter_yul_equal_and_colon(&mut self, _node: &YulEqualAndColon) -> bool {
        true
    }
    fn leave_yul_equal_and_colon(&mut self, _node: &YulEqualAndColon) {}

    fn enter_yul_if_statement(&mut self, _node: &YulIfStatement) -> bool {
        true
    }
    fn leave_yul_if_statement(&mut self, _node: &YulIfStatement) {}

    fn enter_yul_for_statement(&mut self, _node: &YulForStatement) -> bool {
        true
    }
    fn leave_yul_for_statement(&mut self, _node: &YulForStatement) {}

    fn enter_yul_switch_statement(&mut self, _node: &YulSwitchStatement) -> bool {
        true
    }
    fn leave_yul_switch_statement(&mut self, _node: &YulSwitchStatement) {}

    fn enter_yul_default_case(&mut self, _node: &YulDefaultCase) -> bool {
        true
    }
    fn leave_yul_default_case(&mut self, _node: &YulDefaultCase) {}

    fn enter_yul_value_case(&mut self, _node: &YulValueCase) -> bool {
        true
    }
    fn leave_yul_value_case(&mut self, _node: &YulValueCase) {}

    fn enter_yul_leave_statement(&mut self, _node: &YulLeaveStatement) -> bool {
        true
    }
    fn leave_yul_leave_statement(&mut self, _node: &YulLeaveStatement) {}

    fn enter_yul_break_statement(&mut self, _node: &YulBreakStatement) -> bool {
        true
    }
    fn leave_yul_break_statement(&mut self, _node: &YulBreakStatement) {}

    fn enter_yul_continue_statement(&mut self, _node: &YulContinueStatement) -> bool {
        true
    }
    fn leave_yul_continue_statement(&mut self, _node: &YulContinueStatement) {}

    fn enter_yul_label(&mut self, _node: &YulLabel) -> bool {
        true
    }
    fn leave_yul_label(&mut self, _node: &YulLabel) {}

    fn enter_yul_function_call_expression(&mut self, _node: &YulFunctionCallExpression) -> bool {
        true
    }
    fn leave_yul_function_call_expression(&mut self, _node: &YulFunctionCallExpression) {}

    fn enter_source_unit_member(&mut self, _node: &SourceUnitMember) -> bool {
        true
    }
    fn leave_source_unit_member(&mut self, _node: &SourceUnitMember) {}

    fn enter_pragma(&mut self, _node: &Pragma) -> bool {
        true
    }
    fn leave_pragma(&mut self, _node: &Pragma) {}

    fn enter_abicoder_version(&mut self, _node: &AbicoderVersion) -> bool {
        true
    }
    fn leave_abicoder_version(&mut self, _node: &AbicoderVersion) {}

    fn enter_experimental_feature(&mut self, _node: &ExperimentalFeature) -> bool {
        true
    }
    fn leave_experimental_feature(&mut self, _node: &ExperimentalFeature) {}

    fn enter_version_expression(&mut self, _node: &VersionExpression) -> bool {
        true
    }
    fn leave_version_expression(&mut self, _node: &VersionExpression) {}

    fn enter_version_operator(&mut self, _node: &VersionOperator) -> bool {
        true
    }
    fn leave_version_operator(&mut self, _node: &VersionOperator) {}

    fn enter_version_literal(&mut self, _node: &VersionLiteral) -> bool {
        true
    }
    fn leave_version_literal(&mut self, _node: &VersionLiteral) {}

    fn enter_import_clause(&mut self, _node: &ImportClause) -> bool {
        true
    }
    fn leave_import_clause(&mut self, _node: &ImportClause) {}

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

    fn enter_contract_member(&mut self, _node: &ContractMember) -> bool {
        true
    }
    fn leave_contract_member(&mut self, _node: &ContractMember) {}

    fn enter_state_variable_attribute(&mut self, _node: &StateVariableAttribute) -> bool {
        true
    }
    fn leave_state_variable_attribute(&mut self, _node: &StateVariableAttribute) {}

    fn enter_function_attribute(&mut self, _node: &FunctionAttribute) -> bool {
        true
    }
    fn leave_function_attribute(&mut self, _node: &FunctionAttribute) {}

    fn enter_type_name(&mut self, _node: &TypeName) -> bool {
        true
    }
    fn leave_type_name(&mut self, _node: &TypeName) {}

    fn enter_function_type_attribute(&mut self, _node: &FunctionTypeAttribute) -> bool {
        true
    }
    fn leave_function_type_attribute(&mut self, _node: &FunctionTypeAttribute) {}

    fn enter_mapping_key_type(&mut self, _node: &MappingKeyType) -> bool {
        true
    }
    fn leave_mapping_key_type(&mut self, _node: &MappingKeyType) {}

    fn enter_elementary_type(&mut self, _node: &ElementaryType) -> bool {
        true
    }
    fn leave_elementary_type(&mut self, _node: &ElementaryType) {}

    fn enter_statement(&mut self, _node: &Statement) -> bool {
        true
    }
    fn leave_statement(&mut self, _node: &Statement) {}

    fn enter_tuple_member(&mut self, _node: &TupleMember) -> bool {
        true
    }
    fn leave_tuple_member(&mut self, _node: &TupleMember) {}

    fn enter_variable_declaration_type(&mut self, _node: &VariableDeclarationType) -> bool {
        true
    }
    fn leave_variable_declaration_type(&mut self, _node: &VariableDeclarationType) {}

    fn enter_storage_location(&mut self, _node: &StorageLocation) -> bool {
        true
    }
    fn leave_storage_location(&mut self, _node: &StorageLocation) {}

    fn enter_for_statement_initialization(&mut self, _node: &ForStatementInitialization) -> bool {
        true
    }
    fn leave_for_statement_initialization(&mut self, _node: &ForStatementInitialization) {}

    fn enter_for_statement_condition(&mut self, _node: &ForStatementCondition) -> bool {
        true
    }
    fn leave_for_statement_condition(&mut self, _node: &ForStatementCondition) {}

    fn enter_expression(&mut self, _node: &Expression) -> bool {
        true
    }
    fn leave_expression(&mut self, _node: &Expression) {}

    fn enter_arguments_declaration(&mut self, _node: &ArgumentsDeclaration) -> bool {
        true
    }
    fn leave_arguments_declaration(&mut self, _node: &ArgumentsDeclaration) {}

    fn enter_number_unit(&mut self, _node: &NumberUnit) -> bool {
        true
    }
    fn leave_number_unit(&mut self, _node: &NumberUnit) {}

    fn enter_string_expression(&mut self, _node: &StringExpression) -> bool {
        true
    }
    fn leave_string_expression(&mut self, _node: &StringExpression) {}

    fn enter_string_literal(&mut self, _node: &StringLiteral) -> bool {
        true
    }
    fn leave_string_literal(&mut self, _node: &StringLiteral) {}

    fn enter_hex_string_literal(&mut self, _node: &HexStringLiteral) -> bool {
        true
    }
    fn leave_hex_string_literal(&mut self, _node: &HexStringLiteral) {}

    fn enter_unicode_string_literal(&mut self, _node: &UnicodeStringLiteral) -> bool {
        true
    }
    fn leave_unicode_string_literal(&mut self, _node: &UnicodeStringLiteral) {}

    fn enter_yul_statement(&mut self, _node: &YulStatement) -> bool {
        true
    }
    fn leave_yul_statement(&mut self, _node: &YulStatement) {}

    fn enter_yul_assignment_operator(&mut self, _node: &YulAssignmentOperator) -> bool {
        true
    }
    fn leave_yul_assignment_operator(&mut self, _node: &YulAssignmentOperator) {}

    fn enter_yul_stack_assignment_operator(&mut self, _node: &YulStackAssignmentOperator) -> bool {
        true
    }
    fn leave_yul_stack_assignment_operator(&mut self, _node: &YulStackAssignmentOperator) {}

    fn enter_yul_switch_case(&mut self, _node: &YulSwitchCase) -> bool {
        true
    }
    fn leave_yul_switch_case(&mut self, _node: &YulSwitchCase) {}

    fn enter_yul_expression(&mut self, _node: &YulExpression) -> bool {
        true
    }
    fn leave_yul_expression(&mut self, _node: &YulExpression) {}

    fn enter_yul_literal(&mut self, _node: &YulLiteral) -> bool {
        true
    }
    fn leave_yul_literal(&mut self, _node: &YulLiteral) {}

    fn enter_function_kind(&mut self, _node: &FunctionKind) -> bool {
        true
    }
    fn leave_function_kind(&mut self, _node: &FunctionKind) {}

    fn enter_source_unit_members(&mut self, _items: &SourceUnitMembers) -> bool {
        true
    }
    fn leave_source_unit_members(&mut self, _items: &SourceUnitMembers) {}

    fn enter_version_expression_sets(&mut self, _items: &VersionExpressionSets) -> bool {
        true
    }
    fn leave_version_expression_sets(&mut self, _items: &VersionExpressionSets) {}

    fn enter_version_expression_set(&mut self, _items: &VersionExpressionSet) -> bool {
        true
    }
    fn leave_version_expression_set(&mut self, _items: &VersionExpressionSet) {}

    fn enter_simple_version_literal(&mut self, _items: &SimpleVersionLiteral) -> bool {
        true
    }
    fn leave_simple_version_literal(&mut self, _items: &SimpleVersionLiteral) {}

    fn enter_import_deconstruction_symbols(
        &mut self,
        _items: &ImportDeconstructionSymbols,
    ) -> bool {
        true
    }
    fn leave_import_deconstruction_symbols(&mut self, _items: &ImportDeconstructionSymbols) {}

    fn enter_using_deconstruction_symbols(&mut self, _items: &UsingDeconstructionSymbols) -> bool {
        true
    }
    fn leave_using_deconstruction_symbols(&mut self, _items: &UsingDeconstructionSymbols) {}

    fn enter_inheritance_types(&mut self, _items: &InheritanceTypes) -> bool {
        true
    }
    fn leave_inheritance_types(&mut self, _items: &InheritanceTypes) {}

    fn enter_contract_members(&mut self, _items: &ContractMembers) -> bool {
        true
    }
    fn leave_contract_members(&mut self, _items: &ContractMembers) {}

    fn enter_interface_members(&mut self, _items: &InterfaceMembers) -> bool {
        true
    }
    fn leave_interface_members(&mut self, _items: &InterfaceMembers) {}

    fn enter_library_members(&mut self, _items: &LibraryMembers) -> bool {
        true
    }
    fn leave_library_members(&mut self, _items: &LibraryMembers) {}

    fn enter_struct_members(&mut self, _items: &StructMembers) -> bool {
        true
    }
    fn leave_struct_members(&mut self, _items: &StructMembers) {}

    fn enter_enum_members(&mut self, _items: &EnumMembers) -> bool {
        true
    }
    fn leave_enum_members(&mut self, _items: &EnumMembers) {}

    fn enter_state_variable_attributes(&mut self, _items: &StateVariableAttributes) -> bool {
        true
    }
    fn leave_state_variable_attributes(&mut self, _items: &StateVariableAttributes) {}

    fn enter_parameters(&mut self, _items: &Parameters) -> bool {
        true
    }
    fn leave_parameters(&mut self, _items: &Parameters) {}

    fn enter_function_attributes(&mut self, _items: &FunctionAttributes) -> bool {
        true
    }
    fn leave_function_attributes(&mut self, _items: &FunctionAttributes) {}

    fn enter_override_paths(&mut self, _items: &OverridePaths) -> bool {
        true
    }
    fn leave_override_paths(&mut self, _items: &OverridePaths) {}

    fn enter_event_parameters(&mut self, _items: &EventParameters) -> bool {
        true
    }
    fn leave_event_parameters(&mut self, _items: &EventParameters) {}

    fn enter_error_parameters(&mut self, _items: &ErrorParameters) -> bool {
        true
    }
    fn leave_error_parameters(&mut self, _items: &ErrorParameters) {}

    fn enter_function_type_attributes(&mut self, _items: &FunctionTypeAttributes) -> bool {
        true
    }
    fn leave_function_type_attributes(&mut self, _items: &FunctionTypeAttributes) {}

    fn enter_statements(&mut self, _items: &Statements) -> bool {
        true
    }
    fn leave_statements(&mut self, _items: &Statements) {}

    fn enter_assembly_flags(&mut self, _items: &AssemblyFlags) -> bool {
        true
    }
    fn leave_assembly_flags(&mut self, _items: &AssemblyFlags) {}

    fn enter_tuple_deconstruction_elements(
        &mut self,
        _items: &TupleDeconstructionElements,
    ) -> bool {
        true
    }
    fn leave_tuple_deconstruction_elements(&mut self, _items: &TupleDeconstructionElements) {}

    fn enter_catch_clauses(&mut self, _items: &CatchClauses) -> bool {
        true
    }
    fn leave_catch_clauses(&mut self, _items: &CatchClauses) {}

    fn enter_positional_arguments(&mut self, _items: &PositionalArguments) -> bool {
        true
    }
    fn leave_positional_arguments(&mut self, _items: &PositionalArguments) {}

    fn enter_named_arguments(&mut self, _items: &NamedArguments) -> bool {
        true
    }
    fn leave_named_arguments(&mut self, _items: &NamedArguments) {}

    fn enter_call_options(&mut self, _items: &CallOptions) -> bool {
        true
    }
    fn leave_call_options(&mut self, _items: &CallOptions) {}

    fn enter_tuple_values(&mut self, _items: &TupleValues) -> bool {
        true
    }
    fn leave_tuple_values(&mut self, _items: &TupleValues) {}

    fn enter_array_values(&mut self, _items: &ArrayValues) -> bool {
        true
    }
    fn leave_array_values(&mut self, _items: &ArrayValues) {}

    fn enter_string_literals(&mut self, _items: &StringLiterals) -> bool {
        true
    }
    fn leave_string_literals(&mut self, _items: &StringLiterals) {}

    fn enter_hex_string_literals(&mut self, _items: &HexStringLiterals) -> bool {
        true
    }
    fn leave_hex_string_literals(&mut self, _items: &HexStringLiterals) {}

    fn enter_unicode_string_literals(&mut self, _items: &UnicodeStringLiterals) -> bool {
        true
    }
    fn leave_unicode_string_literals(&mut self, _items: &UnicodeStringLiterals) {}

    fn enter_identifier_path(&mut self, _items: &IdentifierPath) -> bool {
        true
    }
    fn leave_identifier_path(&mut self, _items: &IdentifierPath) {}

    fn enter_yul_statements(&mut self, _items: &YulStatements) -> bool {
        true
    }
    fn leave_yul_statements(&mut self, _items: &YulStatements) {}

    fn enter_yul_parameters(&mut self, _items: &YulParameters) -> bool {
        true
    }
    fn leave_yul_parameters(&mut self, _items: &YulParameters) {}

    fn enter_yul_variable_names(&mut self, _items: &YulVariableNames) -> bool {
        true
    }
    fn leave_yul_variable_names(&mut self, _items: &YulVariableNames) {}

    fn enter_yul_switch_cases(&mut self, _items: &YulSwitchCases) -> bool {
        true
    }
    fn leave_yul_switch_cases(&mut self, _items: &YulSwitchCases) {}

    fn enter_yul_arguments(&mut self, _items: &YulArguments) -> bool {
        true
    }
    fn leave_yul_arguments(&mut self, _items: &YulArguments) {}

    fn enter_yul_paths(&mut self, _items: &YulPaths) -> bool {
        true
    }
    fn leave_yul_paths(&mut self, _items: &YulPaths) {}

    fn enter_yul_path(&mut self, _items: &YulPath) -> bool {
        true
    }
    fn leave_yul_path(&mut self, _items: &YulPath) {}
}

//
// Sequences:
//

pub fn accept_source_unit(node: &SourceUnit, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit(node) {
        return;
    }
    accept_source_unit_members(&node.members, visitor);
    visitor.leave_source_unit(node);
}

pub fn accept_pragma_directive(node: &PragmaDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma_directive(node) {
        return;
    }
    accept_pragma(&node.pragma, visitor);
    visitor.leave_pragma_directive(node);
}

pub fn accept_abicoder_pragma(node: &AbicoderPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_abicoder_pragma(node) {
        return;
    }
    accept_abicoder_version(&node.version, visitor);
    visitor.leave_abicoder_pragma(node);
}

pub fn accept_experimental_pragma(node: &ExperimentalPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_experimental_pragma(node) {
        return;
    }
    accept_experimental_feature(&node.feature, visitor);
    visitor.leave_experimental_pragma(node);
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
    if let Some(ref operator) = node.operator {
        accept_version_operator(operator, visitor);
    }
    accept_version_literal(&node.literal, visitor);
    visitor.leave_version_term(node);
}

pub fn accept_import_directive(node: &ImportDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_import_directive(node) {
        return;
    }
    accept_import_clause(&node.clause, visitor);
    visitor.leave_import_directive(node);
}

pub fn accept_path_import(node: &PathImport, visitor: &mut impl Visitor) {
    if !visitor.enter_path_import(node) {
        return;
    }
    accept_string_literal(&node.path, visitor);
    visitor.leave_path_import(node);
}

pub fn accept_named_import(node: &NamedImport, visitor: &mut impl Visitor) {
    if !visitor.enter_named_import(node) {
        return;
    }
    accept_string_literal(&node.path, visitor);
    visitor.leave_named_import(node);
}

pub fn accept_import_deconstruction(node: &ImportDeconstruction, visitor: &mut impl Visitor) {
    if !visitor.enter_import_deconstruction(node) {
        return;
    }
    accept_import_deconstruction_symbols(&node.symbols, visitor);
    accept_string_literal(&node.path, visitor);
    visitor.leave_import_deconstruction(node);
}

pub fn accept_import_deconstruction_symbol(
    node: &ImportDeconstructionSymbol,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_import_deconstruction_symbol(node) {
        return;
    }
    visitor.leave_import_deconstruction_symbol(node);
}

pub fn accept_using_directive(node: &UsingDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_using_directive(node) {
        return;
    }
    accept_using_clause(&node.clause, visitor);
    accept_using_target(&node.target, visitor);
    visitor.leave_using_directive(node);
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
    if let Some(ref alias) = node.alias {
        accept_using_operator(alias, visitor);
    }
    visitor.leave_using_deconstruction_symbol(node);
}

pub fn accept_contract_definition(node: &ContractDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_definition(node) {
        return;
    }
    accept_contract_members(&node.members, visitor);
    accept_inheritance_types(&node.inheritance_types, visitor);
    if let Some(ref storage_layout) = node.storage_layout {
        accept_expression(storage_layout, visitor);
    }
    visitor.leave_contract_definition(node);
}

pub fn accept_inheritance_type(node: &InheritanceType, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_type(node) {
        return;
    }
    accept_identifier_path(&node.type_name, visitor);
    if let Some(ref arguments) = node.arguments {
        accept_arguments_declaration(arguments, visitor);
    }
    visitor.leave_inheritance_type(node);
}

pub fn accept_interface_definition(node: &InterfaceDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_interface_definition(node) {
        return;
    }
    if let Some(ref inheritance) = node.inheritance {
        accept_inheritance_types(inheritance, visitor);
    }
    accept_interface_members(&node.members, visitor);
    visitor.leave_interface_definition(node);
}

pub fn accept_library_definition(node: &LibraryDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_library_definition(node) {
        return;
    }
    accept_library_members(&node.members, visitor);
    visitor.leave_library_definition(node);
}

pub fn accept_struct_definition(node: &StructDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_definition(node) {
        return;
    }
    accept_struct_members(&node.members, visitor);
    visitor.leave_struct_definition(node);
}

pub fn accept_struct_member(node: &StructMember, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_member(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_struct_member(node);
}

pub fn accept_enum_definition(node: &EnumDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_enum_definition(node) {
        return;
    }
    accept_enum_members(&node.members, visitor);
    visitor.leave_enum_definition(node);
}

pub fn accept_constant_definition(node: &ConstantDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_constant_definition(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    accept_expression(&node.value, visitor);
    visitor.leave_constant_definition(node);
}

pub fn accept_state_variable_definition(
    node: &StateVariableDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_definition(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    accept_state_variable_attributes(&node.attributes, visitor);
    if let Some(ref value) = node.value {
        accept_expression(value, visitor);
    }
    visitor.leave_state_variable_definition(node);
}

pub fn accept_function_definition(node: &FunctionDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_function_definition(node) {
        return;
    }
    accept_parameters(&node.parameters, visitor);
    accept_function_attributes(&node.attributes, visitor);
    if let Some(ref returns) = node.returns {
        accept_parameters(returns, visitor);
    }
    accept_function_kind(&node.kind, visitor);
    if let Some(ref body) = node.body {
        accept_block(body, visitor);
    }
    visitor.leave_function_definition(node);
}

pub fn accept_parameter(node: &Parameter, visitor: &mut impl Visitor) {
    if !visitor.enter_parameter(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    if let Some(ref storage_location) = node.storage_location {
        accept_storage_location(storage_location, visitor);
    }
    visitor.leave_parameter(node);
}

pub fn accept_override_specifier(node: &OverrideSpecifier, visitor: &mut impl Visitor) {
    if !visitor.enter_override_specifier(node) {
        return;
    }
    if let Some(ref overridden) = node.overridden {
        accept_override_paths(overridden, visitor);
    }
    visitor.leave_override_specifier(node);
}

pub fn accept_modifier_invocation(node: &ModifierInvocation, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_invocation(node) {
        return;
    }
    accept_identifier_path(&node.name, visitor);
    if let Some(ref arguments) = node.arguments {
        accept_arguments_declaration(arguments, visitor);
    }
    visitor.leave_modifier_invocation(node);
}

pub fn accept_event_definition(node: &EventDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_event_definition(node) {
        return;
    }
    accept_event_parameters(&node.parameters, visitor);
    visitor.leave_event_definition(node);
}

pub fn accept_event_parameter(node: &EventParameter, visitor: &mut impl Visitor) {
    if !visitor.enter_event_parameter(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_event_parameter(node);
}

pub fn accept_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_user_defined_value_type_definition(node) {
        return;
    }
    accept_elementary_type(&node.value_type, visitor);
    visitor.leave_user_defined_value_type_definition(node);
}

pub fn accept_error_definition(node: &ErrorDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_error_definition(node) {
        return;
    }
    accept_error_parameters(&node.members, visitor);
    visitor.leave_error_definition(node);
}

pub fn accept_error_parameter(node: &ErrorParameter, visitor: &mut impl Visitor) {
    if !visitor.enter_error_parameter(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_error_parameter(node);
}

pub fn accept_array_type_name(node: &ArrayTypeName, visitor: &mut impl Visitor) {
    if !visitor.enter_array_type_name(node) {
        return;
    }
    accept_type_name(&node.operand, visitor);
    if let Some(ref index) = node.index {
        accept_expression(index, visitor);
    }
    visitor.leave_array_type_name(node);
}

pub fn accept_function_type(node: &FunctionType, visitor: &mut impl Visitor) {
    if !visitor.enter_function_type(node) {
        return;
    }
    accept_parameters(&node.parameters, visitor);
    accept_function_type_attributes(&node.attributes, visitor);
    if let Some(ref returns) = node.returns {
        accept_parameters(returns, visitor);
    }
    visitor.leave_function_type(node);
}

pub fn accept_mapping_type(node: &MappingType, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_type(node) {
        return;
    }
    accept_mapping_key(&node.key_type, visitor);
    accept_mapping_value(&node.value_type, visitor);
    visitor.leave_mapping_type(node);
}

pub fn accept_mapping_key(node: &MappingKey, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_key(node) {
        return;
    }
    accept_mapping_key_type(&node.key_type, visitor);
    visitor.leave_mapping_key(node);
}

pub fn accept_mapping_value(node: &MappingValue, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_value(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_mapping_value(node);
}

pub fn accept_address_type(node: &AddressType, visitor: &mut impl Visitor) {
    if !visitor.enter_address_type(node) {
        return;
    }
    visitor.leave_address_type(node);
}

pub fn accept_block(node: &Block, visitor: &mut impl Visitor) {
    if !visitor.enter_block(node) {
        return;
    }
    accept_statements(&node.statements, visitor);
    visitor.leave_block(node);
}

pub fn accept_unchecked_block(node: &UncheckedBlock, visitor: &mut impl Visitor) {
    if !visitor.enter_unchecked_block(node) {
        return;
    }
    accept_block(&node.block, visitor);
    visitor.leave_unchecked_block(node);
}

pub fn accept_expression_statement(node: &ExpressionStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_expression_statement(node) {
        return;
    }
    accept_expression(&node.expression, visitor);
    visitor.leave_expression_statement(node);
}

pub fn accept_assembly_statement(node: &AssemblyStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_assembly_statement(node) {
        return;
    }
    if let Some(ref label) = node.label {
        accept_string_literal(label, visitor);
    }
    if let Some(ref flags) = node.flags {
        accept_assembly_flags(flags, visitor);
    }
    accept_yul_block(&node.body, visitor);
    visitor.leave_assembly_statement(node);
}

pub fn accept_tuple_deconstruction_statement(
    node: &TupleDeconstructionStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_tuple_deconstruction_statement(node) {
        return;
    }
    accept_tuple_deconstruction_elements(&node.elements, visitor);
    accept_expression(&node.expression, visitor);
    visitor.leave_tuple_deconstruction_statement(node);
}

pub fn accept_tuple_deconstruction_element(
    node: &TupleDeconstructionElement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_tuple_deconstruction_element(node) {
        return;
    }
    if let Some(ref member) = node.member {
        accept_tuple_member(member, visitor);
    }
    visitor.leave_tuple_deconstruction_element(node);
}

pub fn accept_typed_tuple_member(node: &TypedTupleMember, visitor: &mut impl Visitor) {
    if !visitor.enter_typed_tuple_member(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    if let Some(ref storage_location) = node.storage_location {
        accept_storage_location(storage_location, visitor);
    }
    visitor.leave_typed_tuple_member(node);
}

pub fn accept_untyped_tuple_member(node: &UntypedTupleMember, visitor: &mut impl Visitor) {
    if !visitor.enter_untyped_tuple_member(node) {
        return;
    }
    if let Some(ref storage_location) = node.storage_location {
        accept_storage_location(storage_location, visitor);
    }
    visitor.leave_untyped_tuple_member(node);
}

pub fn accept_variable_declaration_statement(
    node: &VariableDeclarationStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_statement(node) {
        return;
    }
    accept_variable_declaration_type(&node.variable_type, visitor);
    if let Some(ref storage_location) = node.storage_location {
        accept_storage_location(storage_location, visitor);
    }
    if let Some(ref value) = node.value {
        accept_expression(value, visitor);
    }
    visitor.leave_variable_declaration_statement(node);
}

pub fn accept_if_statement(node: &IfStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_if_statement(node) {
        return;
    }
    accept_expression(&node.condition, visitor);
    accept_statement(&node.body, visitor);
    if let Some(ref else_branch) = node.else_branch {
        accept_statement(else_branch, visitor);
    }
    visitor.leave_if_statement(node);
}

pub fn accept_for_statement(node: &ForStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_for_statement(node) {
        return;
    }
    accept_for_statement_initialization(&node.initialization, visitor);
    accept_for_statement_condition(&node.condition, visitor);
    if let Some(ref iterator) = node.iterator {
        accept_expression(iterator, visitor);
    }
    accept_statement(&node.body, visitor);
    visitor.leave_for_statement(node);
}

pub fn accept_while_statement(node: &WhileStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_while_statement(node) {
        return;
    }
    accept_expression(&node.condition, visitor);
    accept_statement(&node.body, visitor);
    visitor.leave_while_statement(node);
}

pub fn accept_do_while_statement(node: &DoWhileStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_do_while_statement(node) {
        return;
    }
    accept_statement(&node.body, visitor);
    accept_expression(&node.condition, visitor);
    visitor.leave_do_while_statement(node);
}

pub fn accept_continue_statement(node: &ContinueStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_continue_statement(node) {
        return;
    }
    visitor.leave_continue_statement(node);
}

pub fn accept_break_statement(node: &BreakStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_break_statement(node) {
        return;
    }
    visitor.leave_break_statement(node);
}

pub fn accept_return_statement(node: &ReturnStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_return_statement(node) {
        return;
    }
    if let Some(ref expression) = node.expression {
        accept_expression(expression, visitor);
    }
    visitor.leave_return_statement(node);
}

pub fn accept_emit_statement(node: &EmitStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_emit_statement(node) {
        return;
    }
    accept_identifier_path(&node.event, visitor);
    accept_arguments_declaration(&node.arguments, visitor);
    visitor.leave_emit_statement(node);
}

pub fn accept_try_statement(node: &TryStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_try_statement(node) {
        return;
    }
    accept_expression(&node.expression, visitor);
    if let Some(ref returns) = node.returns {
        accept_parameters(returns, visitor);
    }
    accept_block(&node.body, visitor);
    accept_catch_clauses(&node.catch_clauses, visitor);
    visitor.leave_try_statement(node);
}

pub fn accept_catch_clause(node: &CatchClause, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause(node) {
        return;
    }
    if let Some(ref error) = node.error {
        accept_catch_clause_error(error, visitor);
    }
    accept_block(&node.body, visitor);
    visitor.leave_catch_clause(node);
}

pub fn accept_catch_clause_error(node: &CatchClauseError, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause_error(node) {
        return;
    }
    accept_parameters(&node.parameters, visitor);
    visitor.leave_catch_clause_error(node);
}

pub fn accept_revert_statement(node: &RevertStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_revert_statement(node) {
        return;
    }
    if let Some(ref error) = node.error {
        accept_identifier_path(error, visitor);
    }
    accept_arguments_declaration(&node.arguments, visitor);
    visitor.leave_revert_statement(node);
}

pub fn accept_throw_statement(node: &ThrowStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_throw_statement(node) {
        return;
    }
    visitor.leave_throw_statement(node);
}

pub fn accept_assignment_expression(node: &AssignmentExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_assignment_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_assignment_expression(node);
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

pub fn accept_or_expression(node: &OrExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_or_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_or_expression(node);
}

pub fn accept_and_expression(node: &AndExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_and_expression(node);
}

pub fn accept_equality_expression(node: &EqualityExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_equality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_equality_expression(node);
}

pub fn accept_inequality_expression(node: &InequalityExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_inequality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_inequality_expression(node);
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

pub fn accept_bitwise_and_expression(node: &BitwiseAndExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_bitwise_and_expression(node);
}

pub fn accept_shift_expression(node: &ShiftExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_shift_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_shift_expression(node);
}

pub fn accept_additive_expression(node: &AdditiveExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_additive_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_additive_expression(node);
}

pub fn accept_multiplicative_expression(
    node: &MultiplicativeExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multiplicative_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_multiplicative_expression(node);
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

pub fn accept_postfix_expression(node: &PostfixExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_postfix_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    visitor.leave_postfix_expression(node);
}

pub fn accept_prefix_expression(node: &PrefixExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_prefix_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    visitor.leave_prefix_expression(node);
}

pub fn accept_function_call_expression(node: &FunctionCallExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_function_call_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_arguments_declaration(&node.arguments, visitor);
    visitor.leave_function_call_expression(node);
}

pub fn accept_call_options_expression(node: &CallOptionsExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_call_options_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_call_options(&node.options, visitor);
    visitor.leave_call_options_expression(node);
}

pub fn accept_member_access_expression(node: &MemberAccessExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_member_access_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    visitor.leave_member_access_expression(node);
}

pub fn accept_index_access_expression(node: &IndexAccessExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_index_access_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    if let Some(ref start) = node.start {
        accept_expression(start, visitor);
    }
    if let Some(ref end) = node.end {
        accept_expression(end, visitor);
    }
    visitor.leave_index_access_expression(node);
}

pub fn accept_named_argument(node: &NamedArgument, visitor: &mut impl Visitor) {
    if !visitor.enter_named_argument(node) {
        return;
    }
    accept_expression(&node.value, visitor);
    visitor.leave_named_argument(node);
}

pub fn accept_type_expression(node: &TypeExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_type_expression(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_type_expression(node);
}

pub fn accept_new_expression(node: &NewExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_new_expression(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_new_expression(node);
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
    if let Some(ref expression) = node.expression {
        accept_expression(expression, visitor);
    }
    visitor.leave_tuple_value(node);
}

pub fn accept_array_expression(node: &ArrayExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_array_expression(node) {
        return;
    }
    accept_array_values(&node.items, visitor);
    visitor.leave_array_expression(node);
}

pub fn accept_hex_number_expression(node: &HexNumberExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_hex_number_expression(node) {
        return;
    }
    if let Some(ref unit) = node.unit {
        accept_number_unit(unit, visitor);
    }
    visitor.leave_hex_number_expression(node);
}

pub fn accept_decimal_number_expression(
    node: &DecimalNumberExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_decimal_number_expression(node) {
        return;
    }
    if let Some(ref unit) = node.unit {
        accept_number_unit(unit, visitor);
    }
    visitor.leave_decimal_number_expression(node);
}

pub fn accept_yul_block(node: &YulBlock, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_block(node) {
        return;
    }
    accept_yul_statements(&node.statements, visitor);
    visitor.leave_yul_block(node);
}

pub fn accept_yul_function_definition(node: &YulFunctionDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_function_definition(node) {
        return;
    }
    accept_yul_parameters(&node.parameters, visitor);
    if let Some(ref returns) = node.returns {
        accept_yul_variable_names(returns, visitor);
    }
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_function_definition(node);
}

pub fn accept_yul_variable_declaration_statement(
    node: &YulVariableDeclarationStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_declaration_statement(node) {
        return;
    }
    accept_yul_variable_names(&node.variables, visitor);
    if let Some(ref value) = node.value {
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
    accept_yul_assignment_operator(&node.assignment, visitor);
    accept_yul_expression(&node.expression, visitor);
    visitor.leave_yul_variable_declaration_value(node);
}

pub fn accept_yul_variable_assignment_statement(
    node: &YulVariableAssignmentStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_assignment_statement(node) {
        return;
    }
    accept_yul_paths(&node.variables, visitor);
    accept_yul_assignment_operator(&node.assignment, visitor);
    accept_yul_expression(&node.expression, visitor);
    visitor.leave_yul_variable_assignment_statement(node);
}

pub fn accept_yul_colon_and_equal(node: &YulColonAndEqual, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_colon_and_equal(node) {
        return;
    }
    visitor.leave_yul_colon_and_equal(node);
}

pub fn accept_yul_stack_assignment_statement(
    node: &YulStackAssignmentStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_stack_assignment_statement(node) {
        return;
    }
    accept_yul_stack_assignment_operator(&node.assignment, visitor);
    visitor.leave_yul_stack_assignment_statement(node);
}

pub fn accept_yul_equal_and_colon(node: &YulEqualAndColon, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_equal_and_colon(node) {
        return;
    }
    visitor.leave_yul_equal_and_colon(node);
}

pub fn accept_yul_if_statement(node: &YulIfStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_if_statement(node) {
        return;
    }
    accept_yul_expression(&node.condition, visitor);
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_if_statement(node);
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

pub fn accept_yul_switch_statement(node: &YulSwitchStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_statement(node) {
        return;
    }
    accept_yul_expression(&node.expression, visitor);
    accept_yul_switch_cases(&node.cases, visitor);
    visitor.leave_yul_switch_statement(node);
}

pub fn accept_yul_default_case(node: &YulDefaultCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_default_case(node) {
        return;
    }
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_default_case(node);
}

pub fn accept_yul_value_case(node: &YulValueCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_value_case(node) {
        return;
    }
    accept_yul_literal(&node.value, visitor);
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_value_case(node);
}

pub fn accept_yul_leave_statement(node: &YulLeaveStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_leave_statement(node) {
        return;
    }
    visitor.leave_yul_leave_statement(node);
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

pub fn accept_yul_label(node: &YulLabel, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_label(node) {
        return;
    }
    visitor.leave_yul_label(node);
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

//
// Choices:
//

pub fn accept_source_unit_member(node: &SourceUnitMember, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit_member(node) {
        return;
    }
    match node {
        SourceUnitMember::PragmaDirective(ref pragma_directive) => {
            accept_pragma_directive(pragma_directive, visitor);
        }
        SourceUnitMember::ImportDirective(ref import_directive) => {
            accept_import_directive(import_directive, visitor);
        }
        SourceUnitMember::ContractDefinition(ref contract_definition) => {
            accept_contract_definition(contract_definition, visitor);
        }
        SourceUnitMember::InterfaceDefinition(ref interface_definition) => {
            accept_interface_definition(interface_definition, visitor);
        }
        SourceUnitMember::LibraryDefinition(ref library_definition) => {
            accept_library_definition(library_definition, visitor);
        }
        SourceUnitMember::StructDefinition(ref struct_definition) => {
            accept_struct_definition(struct_definition, visitor);
        }
        SourceUnitMember::EnumDefinition(ref enum_definition) => {
            accept_enum_definition(enum_definition, visitor);
        }
        SourceUnitMember::FunctionDefinition(ref function_definition) => {
            accept_function_definition(function_definition, visitor);
        }
        SourceUnitMember::ErrorDefinition(ref error_definition) => {
            accept_error_definition(error_definition, visitor);
        }
        SourceUnitMember::UserDefinedValueTypeDefinition(
            ref user_defined_value_type_definition,
        ) => {
            accept_user_defined_value_type_definition(user_defined_value_type_definition, visitor);
        }
        SourceUnitMember::UsingDirective(ref using_directive) => {
            accept_using_directive(using_directive, visitor);
        }
        SourceUnitMember::EventDefinition(ref event_definition) => {
            accept_event_definition(event_definition, visitor);
        }
        SourceUnitMember::ConstantDefinition(ref constant_definition) => {
            accept_constant_definition(constant_definition, visitor);
        }
    }
    visitor.leave_source_unit_member(node);
}

pub fn accept_pragma(node: &Pragma, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma(node) {
        return;
    }
    match node {
        Pragma::VersionPragma(ref version_pragma) => {
            accept_version_pragma(version_pragma, visitor);
        }
        Pragma::AbicoderPragma(ref abicoder_pragma) => {
            accept_abicoder_pragma(abicoder_pragma, visitor);
        }
        Pragma::ExperimentalPragma(ref experimental_pragma) => {
            accept_experimental_pragma(experimental_pragma, visitor);
        }
    }
    visitor.leave_pragma(node);
}

pub fn accept_abicoder_version(_node: &AbicoderVersion, _visitor: &mut impl Visitor) {}

pub fn accept_experimental_feature(node: &ExperimentalFeature, visitor: &mut impl Visitor) {
    if !visitor.enter_experimental_feature(node) {
        return;
    }
    match node {
        ExperimentalFeature::StringLiteral(ref string_literal) => {
            accept_string_literal(string_literal, visitor);
        }
        ExperimentalFeature::ABIEncoderV2Keyword | ExperimentalFeature::SMTCheckerKeyword => {}
    }
    visitor.leave_experimental_feature(node);
}

pub fn accept_version_expression(node: &VersionExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression(node) {
        return;
    }
    match node {
        VersionExpression::VersionRange(ref version_range) => {
            accept_version_range(version_range, visitor);
        }
        VersionExpression::VersionTerm(ref version_term) => {
            accept_version_term(version_term, visitor);
        }
    }
    visitor.leave_version_expression(node);
}

pub fn accept_version_operator(_node: &VersionOperator, _visitor: &mut impl Visitor) {}

pub fn accept_version_literal(node: &VersionLiteral, visitor: &mut impl Visitor) {
    if !visitor.enter_version_literal(node) {
        return;
    }
    match node {
        VersionLiteral::SimpleVersionLiteral(ref simple_version_literal) => {
            accept_simple_version_literal(simple_version_literal, visitor);
        }
        VersionLiteral::SingleQuotedVersionLiteral(_)
        | VersionLiteral::DoubleQuotedVersionLiteral(_) => {}
    }
    visitor.leave_version_literal(node);
}

pub fn accept_import_clause(node: &ImportClause, visitor: &mut impl Visitor) {
    if !visitor.enter_import_clause(node) {
        return;
    }
    match node {
        ImportClause::PathImport(ref path_import) => {
            accept_path_import(path_import, visitor);
        }
        ImportClause::NamedImport(ref named_import) => {
            accept_named_import(named_import, visitor);
        }
        ImportClause::ImportDeconstruction(ref import_deconstruction) => {
            accept_import_deconstruction(import_deconstruction, visitor);
        }
    }
    visitor.leave_import_clause(node);
}

pub fn accept_using_clause(node: &UsingClause, visitor: &mut impl Visitor) {
    if !visitor.enter_using_clause(node) {
        return;
    }
    match node {
        UsingClause::IdentifierPath(ref identifier_path) => {
            accept_identifier_path(identifier_path, visitor);
        }
        UsingClause::UsingDeconstruction(ref using_deconstruction) => {
            accept_using_deconstruction(using_deconstruction, visitor);
        }
    }
    visitor.leave_using_clause(node);
}

pub fn accept_using_operator(_node: &UsingOperator, _visitor: &mut impl Visitor) {}

pub fn accept_using_target(node: &UsingTarget, visitor: &mut impl Visitor) {
    if !visitor.enter_using_target(node) {
        return;
    }
    match node {
        UsingTarget::TypeName(ref type_name) => {
            accept_type_name(type_name, visitor);
        }
        UsingTarget::Asterisk => {}
    }
    visitor.leave_using_target(node);
}

pub fn accept_contract_member(node: &ContractMember, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_member(node) {
        return;
    }
    match node {
        ContractMember::UsingDirective(ref using_directive) => {
            accept_using_directive(using_directive, visitor);
        }
        ContractMember::FunctionDefinition(ref function_definition) => {
            accept_function_definition(function_definition, visitor);
        }
        ContractMember::StructDefinition(ref struct_definition) => {
            accept_struct_definition(struct_definition, visitor);
        }
        ContractMember::EnumDefinition(ref enum_definition) => {
            accept_enum_definition(enum_definition, visitor);
        }
        ContractMember::EventDefinition(ref event_definition) => {
            accept_event_definition(event_definition, visitor);
        }
        ContractMember::ErrorDefinition(ref error_definition) => {
            accept_error_definition(error_definition, visitor);
        }
        ContractMember::UserDefinedValueTypeDefinition(ref user_defined_value_type_definition) => {
            accept_user_defined_value_type_definition(user_defined_value_type_definition, visitor);
        }
        ContractMember::StateVariableDefinition(ref state_variable_definition) => {
            accept_state_variable_definition(state_variable_definition, visitor);
        }
    }
    visitor.leave_contract_member(node);
}

pub fn accept_state_variable_attribute(node: &StateVariableAttribute, visitor: &mut impl Visitor) {
    if !visitor.enter_state_variable_attribute(node) {
        return;
    }
    match node {
        StateVariableAttribute::OverrideSpecifier(ref override_specifier) => {
            accept_override_specifier(override_specifier, visitor);
        }
        StateVariableAttribute::ConstantKeyword
        | StateVariableAttribute::InternalKeyword
        | StateVariableAttribute::PrivateKeyword
        | StateVariableAttribute::PublicKeyword
        | StateVariableAttribute::ImmutableKeyword
        | StateVariableAttribute::TransientKeyword => {}
    }
    visitor.leave_state_variable_attribute(node);
}

pub fn accept_function_attribute(node: &FunctionAttribute, visitor: &mut impl Visitor) {
    if !visitor.enter_function_attribute(node) {
        return;
    }
    match node {
        FunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
            accept_modifier_invocation(modifier_invocation, visitor);
        }
        FunctionAttribute::OverrideSpecifier(ref override_specifier) => {
            accept_override_specifier(override_specifier, visitor);
        }
        FunctionAttribute::ConstantKeyword
        | FunctionAttribute::ExternalKeyword
        | FunctionAttribute::InternalKeyword
        | FunctionAttribute::PayableKeyword
        | FunctionAttribute::PrivateKeyword
        | FunctionAttribute::PublicKeyword
        | FunctionAttribute::PureKeyword
        | FunctionAttribute::ViewKeyword
        | FunctionAttribute::VirtualKeyword => {}
    }
    visitor.leave_function_attribute(node);
}

pub fn accept_type_name(node: &TypeName, visitor: &mut impl Visitor) {
    if !visitor.enter_type_name(node) {
        return;
    }
    match node {
        TypeName::ArrayTypeName(ref array_type_name) => {
            accept_array_type_name(array_type_name, visitor);
        }
        TypeName::FunctionType(ref function_type) => {
            accept_function_type(function_type, visitor);
        }
        TypeName::MappingType(ref mapping_type) => {
            accept_mapping_type(mapping_type, visitor);
        }
        TypeName::ElementaryType(ref elementary_type) => {
            accept_elementary_type(elementary_type, visitor);
        }
        TypeName::IdentifierPath(ref identifier_path) => {
            accept_identifier_path(identifier_path, visitor);
        }
    }
    visitor.leave_type_name(node);
}

pub fn accept_function_type_attribute(_node: &FunctionTypeAttribute, _visitor: &mut impl Visitor) {}

pub fn accept_mapping_key_type(node: &MappingKeyType, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_key_type(node) {
        return;
    }
    match node {
        MappingKeyType::ElementaryType(ref elementary_type) => {
            accept_elementary_type(elementary_type, visitor);
        }
        MappingKeyType::IdentifierPath(ref identifier_path) => {
            accept_identifier_path(identifier_path, visitor);
        }
    }
    visitor.leave_mapping_key_type(node);
}

pub fn accept_elementary_type(node: &ElementaryType, visitor: &mut impl Visitor) {
    if !visitor.enter_elementary_type(node) {
        return;
    }
    match node {
        ElementaryType::AddressType(ref address_type) => {
            accept_address_type(address_type, visitor);
        }
        ElementaryType::BytesKeyword(_)
        | ElementaryType::IntKeyword(_)
        | ElementaryType::UintKeyword(_)
        | ElementaryType::FixedKeyword(_)
        | ElementaryType::UfixedKeyword(_) => {}
        ElementaryType::BoolKeyword
        | ElementaryType::ByteKeyword
        | ElementaryType::StringKeyword => {}
    }
    visitor.leave_elementary_type(node);
}

pub fn accept_statement(node: &Statement, visitor: &mut impl Visitor) {
    if !visitor.enter_statement(node) {
        return;
    }
    match node {
        Statement::IfStatement(ref if_statement) => {
            accept_if_statement(if_statement, visitor);
        }
        Statement::ForStatement(ref for_statement) => {
            accept_for_statement(for_statement, visitor);
        }
        Statement::WhileStatement(ref while_statement) => {
            accept_while_statement(while_statement, visitor);
        }
        Statement::DoWhileStatement(ref do_while_statement) => {
            accept_do_while_statement(do_while_statement, visitor);
        }
        Statement::ContinueStatement(ref continue_statement) => {
            accept_continue_statement(continue_statement, visitor);
        }
        Statement::BreakStatement(ref break_statement) => {
            accept_break_statement(break_statement, visitor);
        }
        Statement::ReturnStatement(ref return_statement) => {
            accept_return_statement(return_statement, visitor);
        }
        Statement::ThrowStatement(ref throw_statement) => {
            accept_throw_statement(throw_statement, visitor);
        }
        Statement::EmitStatement(ref emit_statement) => {
            accept_emit_statement(emit_statement, visitor);
        }
        Statement::TryStatement(ref try_statement) => {
            accept_try_statement(try_statement, visitor);
        }
        Statement::RevertStatement(ref revert_statement) => {
            accept_revert_statement(revert_statement, visitor);
        }
        Statement::AssemblyStatement(ref assembly_statement) => {
            accept_assembly_statement(assembly_statement, visitor);
        }
        Statement::Block(ref block) => {
            accept_block(block, visitor);
        }
        Statement::UncheckedBlock(ref unchecked_block) => {
            accept_unchecked_block(unchecked_block, visitor);
        }
        Statement::TupleDeconstructionStatement(ref tuple_deconstruction_statement) => {
            accept_tuple_deconstruction_statement(tuple_deconstruction_statement, visitor);
        }
        Statement::VariableDeclarationStatement(ref variable_declaration_statement) => {
            accept_variable_declaration_statement(variable_declaration_statement, visitor);
        }
        Statement::ExpressionStatement(ref expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
    }
    visitor.leave_statement(node);
}

pub fn accept_tuple_member(node: &TupleMember, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_member(node) {
        return;
    }
    match node {
        TupleMember::TypedTupleMember(ref typed_tuple_member) => {
            accept_typed_tuple_member(typed_tuple_member, visitor);
        }
        TupleMember::UntypedTupleMember(ref untyped_tuple_member) => {
            accept_untyped_tuple_member(untyped_tuple_member, visitor);
        }
    }
    visitor.leave_tuple_member(node);
}

pub fn accept_variable_declaration_type(
    node: &VariableDeclarationType,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_type(node) {
        return;
    }
    match node {
        VariableDeclarationType::TypeName(ref type_name) => {
            accept_type_name(type_name, visitor);
        }
        VariableDeclarationType::VarKeyword => {}
    }
    visitor.leave_variable_declaration_type(node);
}

pub fn accept_storage_location(_node: &StorageLocation, _visitor: &mut impl Visitor) {}

pub fn accept_for_statement_initialization(
    node: &ForStatementInitialization,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_for_statement_initialization(node) {
        return;
    }
    match node {
        ForStatementInitialization::TupleDeconstructionStatement(
            ref tuple_deconstruction_statement,
        ) => {
            accept_tuple_deconstruction_statement(tuple_deconstruction_statement, visitor);
        }
        ForStatementInitialization::VariableDeclarationStatement(
            ref variable_declaration_statement,
        ) => {
            accept_variable_declaration_statement(variable_declaration_statement, visitor);
        }
        ForStatementInitialization::ExpressionStatement(ref expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
        ForStatementInitialization::Semicolon => {}
    }
    visitor.leave_for_statement_initialization(node);
}

pub fn accept_for_statement_condition(node: &ForStatementCondition, visitor: &mut impl Visitor) {
    if !visitor.enter_for_statement_condition(node) {
        return;
    }
    match node {
        ForStatementCondition::ExpressionStatement(ref expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
        ForStatementCondition::Semicolon => {}
    }
    visitor.leave_for_statement_condition(node);
}

pub fn accept_expression(node: &Expression, visitor: &mut impl Visitor) {
    if !visitor.enter_expression(node) {
        return;
    }
    match node {
        Expression::AssignmentExpression(ref assignment_expression) => {
            accept_assignment_expression(assignment_expression, visitor);
        }
        Expression::ConditionalExpression(ref conditional_expression) => {
            accept_conditional_expression(conditional_expression, visitor);
        }
        Expression::OrExpression(ref or_expression) => {
            accept_or_expression(or_expression, visitor);
        }
        Expression::AndExpression(ref and_expression) => {
            accept_and_expression(and_expression, visitor);
        }
        Expression::EqualityExpression(ref equality_expression) => {
            accept_equality_expression(equality_expression, visitor);
        }
        Expression::InequalityExpression(ref inequality_expression) => {
            accept_inequality_expression(inequality_expression, visitor);
        }
        Expression::BitwiseOrExpression(ref bitwise_or_expression) => {
            accept_bitwise_or_expression(bitwise_or_expression, visitor);
        }
        Expression::BitwiseXorExpression(ref bitwise_xor_expression) => {
            accept_bitwise_xor_expression(bitwise_xor_expression, visitor);
        }
        Expression::BitwiseAndExpression(ref bitwise_and_expression) => {
            accept_bitwise_and_expression(bitwise_and_expression, visitor);
        }
        Expression::ShiftExpression(ref shift_expression) => {
            accept_shift_expression(shift_expression, visitor);
        }
        Expression::AdditiveExpression(ref additive_expression) => {
            accept_additive_expression(additive_expression, visitor);
        }
        Expression::MultiplicativeExpression(ref multiplicative_expression) => {
            accept_multiplicative_expression(multiplicative_expression, visitor);
        }
        Expression::ExponentiationExpression(ref exponentiation_expression) => {
            accept_exponentiation_expression(exponentiation_expression, visitor);
        }
        Expression::PostfixExpression(ref postfix_expression) => {
            accept_postfix_expression(postfix_expression, visitor);
        }
        Expression::PrefixExpression(ref prefix_expression) => {
            accept_prefix_expression(prefix_expression, visitor);
        }
        Expression::FunctionCallExpression(ref function_call_expression) => {
            accept_function_call_expression(function_call_expression, visitor);
        }
        Expression::CallOptionsExpression(ref call_options_expression) => {
            accept_call_options_expression(call_options_expression, visitor);
        }
        Expression::MemberAccessExpression(ref member_access_expression) => {
            accept_member_access_expression(member_access_expression, visitor);
        }
        Expression::IndexAccessExpression(ref index_access_expression) => {
            accept_index_access_expression(index_access_expression, visitor);
        }
        Expression::NewExpression(ref new_expression) => {
            accept_new_expression(new_expression, visitor);
        }
        Expression::TupleExpression(ref tuple_expression) => {
            accept_tuple_expression(tuple_expression, visitor);
        }
        Expression::TypeExpression(ref type_expression) => {
            accept_type_expression(type_expression, visitor);
        }
        Expression::ArrayExpression(ref array_expression) => {
            accept_array_expression(array_expression, visitor);
        }
        Expression::HexNumberExpression(ref hex_number_expression) => {
            accept_hex_number_expression(hex_number_expression, visitor);
        }
        Expression::DecimalNumberExpression(ref decimal_number_expression) => {
            accept_decimal_number_expression(decimal_number_expression, visitor);
        }
        Expression::StringExpression(ref string_expression) => {
            accept_string_expression(string_expression, visitor);
        }
        Expression::ElementaryType(ref elementary_type) => {
            accept_elementary_type(elementary_type, visitor);
        }
        Expression::Identifier(_) => {}
        Expression::PayableKeyword
        | Expression::ThisKeyword
        | Expression::SuperKeyword
        | Expression::TrueKeyword
        | Expression::FalseKeyword => {}
    }
    visitor.leave_expression(node);
}

pub fn accept_arguments_declaration(node: &ArgumentsDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_arguments_declaration(node) {
        return;
    }
    match node {
        ArgumentsDeclaration::PositionalArguments(ref positional_arguments) => {
            accept_positional_arguments(positional_arguments, visitor);
        }
        ArgumentsDeclaration::NamedArguments(ref named_arguments) => {
            accept_named_arguments(named_arguments, visitor);
        }
    }
    visitor.leave_arguments_declaration(node);
}

pub fn accept_number_unit(_node: &NumberUnit, _visitor: &mut impl Visitor) {}

pub fn accept_string_expression(node: &StringExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_string_expression(node) {
        return;
    }
    match node {
        StringExpression::StringLiteral(ref string_literal) => {
            accept_string_literal(string_literal, visitor);
        }
        StringExpression::StringLiterals(ref string_literals) => {
            accept_string_literals(string_literals, visitor);
        }
        StringExpression::HexStringLiteral(ref hex_string_literal) => {
            accept_hex_string_literal(hex_string_literal, visitor);
        }
        StringExpression::HexStringLiterals(ref hex_string_literals) => {
            accept_hex_string_literals(hex_string_literals, visitor);
        }
        StringExpression::UnicodeStringLiterals(ref unicode_string_literals) => {
            accept_unicode_string_literals(unicode_string_literals, visitor);
        }
    }
    visitor.leave_string_expression(node);
}

pub fn accept_string_literal(_node: &StringLiteral, _visitor: &mut impl Visitor) {}

pub fn accept_hex_string_literal(_node: &HexStringLiteral, _visitor: &mut impl Visitor) {}

pub fn accept_unicode_string_literal(_node: &UnicodeStringLiteral, _visitor: &mut impl Visitor) {}

pub fn accept_yul_statement(node: &YulStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_statement(node) {
        return;
    }
    match node {
        YulStatement::YulBlock(ref yul_block) => {
            accept_yul_block(yul_block, visitor);
        }
        YulStatement::YulFunctionDefinition(ref yul_function_definition) => {
            accept_yul_function_definition(yul_function_definition, visitor);
        }
        YulStatement::YulStackAssignmentStatement(ref yul_stack_assignment_statement) => {
            accept_yul_stack_assignment_statement(yul_stack_assignment_statement, visitor);
        }
        YulStatement::YulIfStatement(ref yul_if_statement) => {
            accept_yul_if_statement(yul_if_statement, visitor);
        }
        YulStatement::YulForStatement(ref yul_for_statement) => {
            accept_yul_for_statement(yul_for_statement, visitor);
        }
        YulStatement::YulSwitchStatement(ref yul_switch_statement) => {
            accept_yul_switch_statement(yul_switch_statement, visitor);
        }
        YulStatement::YulLeaveStatement(ref yul_leave_statement) => {
            accept_yul_leave_statement(yul_leave_statement, visitor);
        }
        YulStatement::YulBreakStatement(ref yul_break_statement) => {
            accept_yul_break_statement(yul_break_statement, visitor);
        }
        YulStatement::YulContinueStatement(ref yul_continue_statement) => {
            accept_yul_continue_statement(yul_continue_statement, visitor);
        }
        YulStatement::YulVariableAssignmentStatement(ref yul_variable_assignment_statement) => {
            accept_yul_variable_assignment_statement(yul_variable_assignment_statement, visitor);
        }
        YulStatement::YulLabel(ref yul_label) => {
            accept_yul_label(yul_label, visitor);
        }
        YulStatement::YulVariableDeclarationStatement(ref yul_variable_declaration_statement) => {
            accept_yul_variable_declaration_statement(yul_variable_declaration_statement, visitor);
        }
        YulStatement::YulExpression(ref yul_expression) => {
            accept_yul_expression(yul_expression, visitor);
        }
    }
    visitor.leave_yul_statement(node);
}

pub fn accept_yul_assignment_operator(node: &YulAssignmentOperator, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_assignment_operator(node) {
        return;
    }
    match node {
        YulAssignmentOperator::YulColonAndEqual(ref yul_colon_and_equal) => {
            accept_yul_colon_and_equal(yul_colon_and_equal, visitor);
        }
        YulAssignmentOperator::ColonEqual => {}
    }
    visitor.leave_yul_assignment_operator(node);
}

pub fn accept_yul_stack_assignment_operator(
    node: &YulStackAssignmentOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_stack_assignment_operator(node) {
        return;
    }
    match node {
        YulStackAssignmentOperator::YulEqualAndColon(ref yul_equal_and_colon) => {
            accept_yul_equal_and_colon(yul_equal_and_colon, visitor);
        }
        YulStackAssignmentOperator::EqualColon => {}
    }
    visitor.leave_yul_stack_assignment_operator(node);
}

pub fn accept_yul_switch_case(node: &YulSwitchCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_case(node) {
        return;
    }
    match node {
        YulSwitchCase::YulDefaultCase(ref yul_default_case) => {
            accept_yul_default_case(yul_default_case, visitor);
        }
        YulSwitchCase::YulValueCase(ref yul_value_case) => {
            accept_yul_value_case(yul_value_case, visitor);
        }
    }
    visitor.leave_yul_switch_case(node);
}

pub fn accept_yul_expression(node: &YulExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_expression(node) {
        return;
    }
    match node {
        YulExpression::YulFunctionCallExpression(ref yul_function_call_expression) => {
            accept_yul_function_call_expression(yul_function_call_expression, visitor);
        }
        YulExpression::YulLiteral(ref yul_literal) => {
            accept_yul_literal(yul_literal, visitor);
        }
        YulExpression::YulPath(ref yul_path) => {
            accept_yul_path(yul_path, visitor);
        }
    }
    visitor.leave_yul_expression(node);
}

pub fn accept_yul_literal(node: &YulLiteral, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_literal(node) {
        return;
    }
    match node {
        YulLiteral::HexStringLiteral(ref hex_string_literal) => {
            accept_hex_string_literal(hex_string_literal, visitor);
        }
        YulLiteral::StringLiteral(ref string_literal) => {
            accept_string_literal(string_literal, visitor);
        }
        YulLiteral::YulDecimalLiteral(_) | YulLiteral::YulHexLiteral(_) => {}
        YulLiteral::YulTrueKeyword | YulLiteral::YulFalseKeyword => {}
    }
    visitor.leave_yul_literal(node);
}

pub fn accept_function_kind(_node: &FunctionKind, _visitor: &mut impl Visitor) {}

//
// Repeated & Separated
//

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
fn accept_simple_version_literal(items: &Vec<Rc<TerminalNode>>, visitor: &mut impl Visitor) {
    if visitor.enter_simple_version_literal(items) {
        visitor.leave_simple_version_literal(items);
    }
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
fn accept_enum_members(items: &Vec<Rc<TerminalNode>>, visitor: &mut impl Visitor) {
    if visitor.enter_enum_members(items) {
        visitor.leave_enum_members(items);
    }
}

#[inline]
fn accept_state_variable_attributes(
    items: &Vec<StateVariableAttribute>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_attributes(items) {
        return;
    }
    for item in items {
        accept_state_variable_attribute(item, visitor);
    }
    visitor.leave_state_variable_attributes(items);
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
fn accept_function_attributes(items: &Vec<FunctionAttribute>, visitor: &mut impl Visitor) {
    if !visitor.enter_function_attributes(items) {
        return;
    }
    for item in items {
        accept_function_attribute(item, visitor);
    }
    visitor.leave_function_attributes(items);
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
fn accept_event_parameters(items: &Vec<EventParameter>, visitor: &mut impl Visitor) {
    if !visitor.enter_event_parameters(items) {
        return;
    }
    for item in items {
        accept_event_parameter(item, visitor);
    }
    visitor.leave_event_parameters(items);
}

#[inline]
fn accept_error_parameters(items: &Vec<ErrorParameter>, visitor: &mut impl Visitor) {
    if !visitor.enter_error_parameters(items) {
        return;
    }
    for item in items {
        accept_error_parameter(item, visitor);
    }
    visitor.leave_error_parameters(items);
}

#[inline]
fn accept_function_type_attributes(items: &Vec<FunctionTypeAttribute>, visitor: &mut impl Visitor) {
    if !visitor.enter_function_type_attributes(items) {
        return;
    }
    for item in items {
        accept_function_type_attribute(item, visitor);
    }
    visitor.leave_function_type_attributes(items);
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
fn accept_assembly_flags(items: &Vec<StringLiteral>, visitor: &mut impl Visitor) {
    if !visitor.enter_assembly_flags(items) {
        return;
    }
    for item in items {
        accept_string_literal(item, visitor);
    }
    visitor.leave_assembly_flags(items);
}

#[inline]
fn accept_tuple_deconstruction_elements(
    items: &Vec<TupleDeconstructionElement>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_tuple_deconstruction_elements(items) {
        return;
    }
    for item in items {
        accept_tuple_deconstruction_element(item, visitor);
    }
    visitor.leave_tuple_deconstruction_elements(items);
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
fn accept_string_literals(items: &Vec<StringLiteral>, visitor: &mut impl Visitor) {
    if !visitor.enter_string_literals(items) {
        return;
    }
    for item in items {
        accept_string_literal(item, visitor);
    }
    visitor.leave_string_literals(items);
}

#[inline]
fn accept_hex_string_literals(items: &Vec<HexStringLiteral>, visitor: &mut impl Visitor) {
    if !visitor.enter_hex_string_literals(items) {
        return;
    }
    for item in items {
        accept_hex_string_literal(item, visitor);
    }
    visitor.leave_hex_string_literals(items);
}

#[inline]
fn accept_unicode_string_literals(items: &Vec<UnicodeStringLiteral>, visitor: &mut impl Visitor) {
    if !visitor.enter_unicode_string_literals(items) {
        return;
    }
    for item in items {
        accept_unicode_string_literal(item, visitor);
    }
    visitor.leave_unicode_string_literals(items);
}

#[inline]
fn accept_identifier_path(items: &Vec<Rc<TerminalNode>>, visitor: &mut impl Visitor) {
    if visitor.enter_identifier_path(items) {
        visitor.leave_identifier_path(items);
    }
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
fn accept_yul_parameters(items: &Vec<Rc<TerminalNode>>, visitor: &mut impl Visitor) {
    if visitor.enter_yul_parameters(items) {
        visitor.leave_yul_parameters(items);
    }
}

#[inline]
fn accept_yul_variable_names(items: &Vec<Rc<TerminalNode>>, visitor: &mut impl Visitor) {
    if visitor.enter_yul_variable_names(items) {
        visitor.leave_yul_variable_names(items);
    }
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
fn accept_yul_path(items: &Vec<Rc<TerminalNode>>, visitor: &mut impl Visitor) {
    if visitor.enter_yul_path(items) {
        visitor.leave_yul_path(items);
    }
}
