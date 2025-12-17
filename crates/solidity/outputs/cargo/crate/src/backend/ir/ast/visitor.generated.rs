// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use super::input as input_ir;
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

    fn enter_path_import(&mut self, _node: &PathImport) -> bool {
        true
    }
    fn leave_path_import(&mut self, _node: &PathImport) {}

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

    fn enter_type_name(&mut self, _node: &TypeName) -> bool {
        true
    }
    fn leave_type_name(&mut self, _node: &TypeName) {}

    fn enter_elementary_type(&mut self, _node: &ElementaryType) -> bool {
        true
    }
    fn leave_elementary_type(&mut self, _node: &ElementaryType) {}

    fn enter_statement(&mut self, _node: &Statement) -> bool {
        true
    }
    fn leave_statement(&mut self, _node: &Statement) {}

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

    fn enter_function_visibility(&mut self, _node: &FunctionVisibility) -> bool {
        true
    }
    fn leave_function_visibility(&mut self, _node: &FunctionVisibility) {}

    fn enter_function_mutability(&mut self, _node: &FunctionMutability) -> bool {
        true
    }
    fn leave_function_mutability(&mut self, _node: &FunctionMutability) {}

    fn enter_state_variable_visibility(&mut self, _node: &StateVariableVisibility) -> bool {
        true
    }
    fn leave_state_variable_visibility(&mut self, _node: &StateVariableVisibility) {}

    fn enter_state_variable_mutability(&mut self, _node: &StateVariableMutability) -> bool {
        true
    }
    fn leave_state_variable_mutability(&mut self, _node: &StateVariableMutability) {}

    fn enter_tuple_deconstruction_member(&mut self, _node: &TupleDeconstructionMember) -> bool {
        true
    }
    fn leave_tuple_deconstruction_member(&mut self, _node: &TupleDeconstructionMember) {}

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

    fn enter_simple_version_literal(&mut self, _items: &[Rc<TerminalNode>]) -> bool {
        true
    }
    fn leave_simple_version_literal(&mut self, _items: &[Rc<TerminalNode>]) {}

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

    fn enter_parameters(&mut self, _items: &Parameters) -> bool {
        true
    }
    fn leave_parameters(&mut self, _items: &Parameters) {}

    fn enter_override_paths(&mut self, _items: &OverridePaths) -> bool {
        true
    }
    fn leave_override_paths(&mut self, _items: &OverridePaths) {}

    fn enter_statements(&mut self, _items: &Statements) -> bool {
        true
    }
    fn leave_statements(&mut self, _items: &Statements) {}

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

    fn enter_modifier_invocations(&mut self, _items: &ModifierInvocations) -> bool {
        true
    }
    fn leave_modifier_invocations(&mut self, _items: &ModifierInvocations) {}

    fn enter_strings(&mut self, _items: &[Rc<TerminalNode>]) -> bool {
        true
    }
    fn leave_strings(&mut self, _items: &[Rc<TerminalNode>]) {}

    fn enter_hex_strings(&mut self, _items: &[Rc<TerminalNode>]) -> bool {
        true
    }
    fn leave_hex_strings(&mut self, _items: &[Rc<TerminalNode>]) {}

    fn enter_unicode_strings(&mut self, _items: &[Rc<TerminalNode>]) -> bool {
        true
    }
    fn leave_unicode_strings(&mut self, _items: &[Rc<TerminalNode>]) {}

    fn enter_assembly_flags(&mut self, _items: &[Rc<TerminalNode>]) -> bool {
        true
    }
    fn leave_assembly_flags(&mut self, _items: &[Rc<TerminalNode>]) {}

    fn enter_tuple_deconstruction_members(&mut self, _items: &TupleDeconstructionMembers) -> bool {
        true
    }
    fn leave_tuple_deconstruction_members(&mut self, _items: &TupleDeconstructionMembers) {}
    fn visit_identifier(&mut self, _node: &Identifier) {}
    fn visit_yul_identifier(&mut self, _node: &YulIdentifier) {}
}

pub fn accept_identifier(node: &Identifier, visitor: &mut impl Visitor) {
    visitor.visit_identifier(node);
}

pub fn accept_yul_identifier(node: &YulIdentifier, visitor: &mut impl Visitor) {
    visitor.visit_yul_identifier(node);
}

//
// Sequences:
//

pub fn accept_source_unit(node: &SourceUnit, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit(node) {
        return;
    }
    accept_source_unit_members(&node.members(), visitor);
    visitor.leave_source_unit(node);
}

pub fn accept_pragma_directive(node: &PragmaDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma_directive(node) {
        return;
    }
    accept_pragma(&node.pragma(), visitor);
    visitor.leave_pragma_directive(node);
}

pub fn accept_abicoder_pragma(node: &AbicoderPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_abicoder_pragma(node) {
        return;
    }
    accept_abicoder_version(&node.version(), visitor);
    visitor.leave_abicoder_pragma(node);
}

pub fn accept_experimental_pragma(node: &ExperimentalPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_experimental_pragma(node) {
        return;
    }
    accept_experimental_feature(&node.feature(), visitor);
    visitor.leave_experimental_pragma(node);
}

pub fn accept_version_pragma(node: &VersionPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_version_pragma(node) {
        return;
    }
    accept_version_expression_sets(&node.sets(), visitor);
    visitor.leave_version_pragma(node);
}

pub fn accept_version_range(node: &VersionRange, visitor: &mut impl Visitor) {
    if !visitor.enter_version_range(node) {
        return;
    }
    accept_version_literal(&node.start(), visitor);
    accept_version_literal(&node.end(), visitor);
    visitor.leave_version_range(node);
}

pub fn accept_version_term(node: &VersionTerm, visitor: &mut impl Visitor) {
    if !visitor.enter_version_term(node) {
        return;
    }
    if let Some(ref operator) = node.operator() {
        accept_version_operator(operator, visitor);
    }
    accept_version_literal(&node.literal(), visitor);
    visitor.leave_version_term(node);
}

pub fn accept_path_import(node: &PathImport, visitor: &mut impl Visitor) {
    if !visitor.enter_path_import(node) {
        return;
    }
    if let Some(ref alias) = node.alias() {
        accept_identifier(alias, visitor);
    }
    visitor.leave_path_import(node);
}

pub fn accept_import_deconstruction(node: &ImportDeconstruction, visitor: &mut impl Visitor) {
    if !visitor.enter_import_deconstruction(node) {
        return;
    }
    accept_import_deconstruction_symbols(&node.symbols(), visitor);
    visitor.leave_import_deconstruction(node);
}

pub fn accept_import_deconstruction_symbol(
    node: &ImportDeconstructionSymbol,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_import_deconstruction_symbol(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    if let Some(ref alias) = node.alias() {
        accept_identifier(alias, visitor);
    }
    visitor.leave_import_deconstruction_symbol(node);
}

pub fn accept_using_directive(node: &UsingDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_using_directive(node) {
        return;
    }
    accept_using_clause(&node.clause(), visitor);
    accept_using_target(&node.target(), visitor);
    visitor.leave_using_directive(node);
}

pub fn accept_using_deconstruction(node: &UsingDeconstruction, visitor: &mut impl Visitor) {
    if !visitor.enter_using_deconstruction(node) {
        return;
    }
    accept_using_deconstruction_symbols(&node.symbols(), visitor);
    visitor.leave_using_deconstruction(node);
}

pub fn accept_using_deconstruction_symbol(
    node: &UsingDeconstructionSymbol,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_using_deconstruction_symbol(node) {
        return;
    }
    accept_identifier_path(&node.name(), visitor);
    if let Some(ref alias) = node.alias() {
        accept_using_operator(alias, visitor);
    }
    visitor.leave_using_deconstruction_symbol(node);
}

pub fn accept_contract_definition(node: &ContractDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_definition(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    accept_contract_members(&node.members(), visitor);
    accept_inheritance_types(&node.inheritance_types(), visitor);
    if let Some(ref storage_layout) = node.storage_layout() {
        accept_expression(storage_layout, visitor);
    }
    visitor.leave_contract_definition(node);
}

pub fn accept_inheritance_type(node: &InheritanceType, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_type(node) {
        return;
    }
    accept_identifier_path(&node.type_name(), visitor);
    if let Some(ref arguments) = node.arguments() {
        accept_arguments_declaration(arguments, visitor);
    }
    visitor.leave_inheritance_type(node);
}

pub fn accept_interface_definition(node: &InterfaceDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_interface_definition(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    if let Some(ref inheritance) = node.inheritance() {
        accept_inheritance_types(inheritance, visitor);
    }
    accept_interface_members(&node.members(), visitor);
    visitor.leave_interface_definition(node);
}

pub fn accept_library_definition(node: &LibraryDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_library_definition(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    accept_library_members(&node.members(), visitor);
    visitor.leave_library_definition(node);
}

pub fn accept_struct_definition(node: &StructDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_definition(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    accept_struct_members(&node.members(), visitor);
    visitor.leave_struct_definition(node);
}

pub fn accept_struct_member(node: &StructMember, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_member(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    accept_identifier(&node.name(), visitor);
    visitor.leave_struct_member(node);
}

pub fn accept_enum_definition(node: &EnumDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_enum_definition(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    accept_enum_members(&node.members(), visitor);
    visitor.leave_enum_definition(node);
}

pub fn accept_constant_definition(node: &ConstantDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_constant_definition(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    accept_identifier(&node.name(), visitor);
    if let Some(ref visibility) = node.visibility() {
        accept_state_variable_visibility(visibility, visitor);
    }
    if let Some(ref value) = node.value() {
        accept_expression(value, visitor);
    }
    visitor.leave_constant_definition(node);
}

pub fn accept_state_variable_definition(
    node: &StateVariableDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_definition(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    accept_identifier(&node.name(), visitor);
    if let Some(ref value) = node.value() {
        accept_expression(value, visitor);
    }
    accept_state_variable_visibility(&node.visibility(), visitor);
    accept_state_variable_mutability(&node.mutability(), visitor);
    if let Some(ref override_specifier) = node.override_specifier() {
        accept_override_paths(override_specifier, visitor);
    }
    visitor.leave_state_variable_definition(node);
}

pub fn accept_function_definition(node: &FunctionDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_function_definition(node) {
        return;
    }
    accept_parameters(&node.parameters(), visitor);
    if let Some(ref returns) = node.returns() {
        accept_parameters(returns, visitor);
    }
    accept_function_kind(&node.kind(), visitor);
    if let Some(ref name) = node.name() {
        accept_identifier(name, visitor);
    }
    if let Some(ref body) = node.body() {
        accept_block(body, visitor);
    }
    accept_function_visibility(&node.visibility(), visitor);
    accept_function_mutability(&node.mutability(), visitor);
    if let Some(ref override_specifier) = node.override_specifier() {
        accept_override_paths(override_specifier, visitor);
    }
    accept_modifier_invocations(&node.modifier_invocations(), visitor);
    visitor.leave_function_definition(node);
}

pub fn accept_parameter(node: &Parameter, visitor: &mut impl Visitor) {
    if !visitor.enter_parameter(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    if let Some(ref storage_location) = node.storage_location() {
        accept_storage_location(storage_location, visitor);
    }
    if let Some(ref name) = node.name() {
        accept_identifier(name, visitor);
    }
    visitor.leave_parameter(node);
}

pub fn accept_override_specifier(node: &OverrideSpecifier, visitor: &mut impl Visitor) {
    if !visitor.enter_override_specifier(node) {
        return;
    }
    if let Some(ref overridden) = node.overridden() {
        accept_override_paths(overridden, visitor);
    }
    visitor.leave_override_specifier(node);
}

pub fn accept_modifier_invocation(node: &ModifierInvocation, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_invocation(node) {
        return;
    }
    accept_identifier_path(&node.name(), visitor);
    if let Some(ref arguments) = node.arguments() {
        accept_arguments_declaration(arguments, visitor);
    }
    visitor.leave_modifier_invocation(node);
}

pub fn accept_event_definition(node: &EventDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_event_definition(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    accept_parameters(&node.parameters(), visitor);
    visitor.leave_event_definition(node);
}

pub fn accept_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_user_defined_value_type_definition(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    accept_elementary_type(&node.value_type(), visitor);
    visitor.leave_user_defined_value_type_definition(node);
}

pub fn accept_error_definition(node: &ErrorDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_error_definition(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    accept_parameters(&node.parameters(), visitor);
    visitor.leave_error_definition(node);
}

pub fn accept_array_type_name(node: &ArrayTypeName, visitor: &mut impl Visitor) {
    if !visitor.enter_array_type_name(node) {
        return;
    }
    accept_type_name(&node.operand(), visitor);
    if let Some(ref index) = node.index() {
        accept_expression(index, visitor);
    }
    visitor.leave_array_type_name(node);
}

pub fn accept_function_type(node: &FunctionType, visitor: &mut impl Visitor) {
    if !visitor.enter_function_type(node) {
        return;
    }
    accept_parameters(&node.parameters(), visitor);
    if let Some(ref returns) = node.returns() {
        accept_parameters(returns, visitor);
    }
    accept_function_visibility(&node.visibility(), visitor);
    accept_function_mutability(&node.mutability(), visitor);
    visitor.leave_function_type(node);
}

pub fn accept_mapping_type(node: &MappingType, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_type(node) {
        return;
    }
    accept_parameter(&node.key_type(), visitor);
    accept_parameter(&node.value_type(), visitor);
    visitor.leave_mapping_type(node);
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
    accept_statements(&node.statements(), visitor);
    visitor.leave_block(node);
}

pub fn accept_unchecked_block(node: &UncheckedBlock, visitor: &mut impl Visitor) {
    if !visitor.enter_unchecked_block(node) {
        return;
    }
    accept_block(&node.block(), visitor);
    visitor.leave_unchecked_block(node);
}

pub fn accept_expression_statement(node: &ExpressionStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_expression_statement(node) {
        return;
    }
    accept_expression(&node.expression(), visitor);
    visitor.leave_expression_statement(node);
}

pub fn accept_assembly_statement(node: &AssemblyStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_assembly_statement(node) {
        return;
    }
    accept_yul_block(&node.body(), visitor);
    accept_assembly_flags(&node.flags(), visitor);
    visitor.leave_assembly_statement(node);
}

pub fn accept_tuple_deconstruction_statement(
    node: &TupleDeconstructionStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_tuple_deconstruction_statement(node) {
        return;
    }
    accept_expression(&node.expression(), visitor);
    accept_tuple_deconstruction_members(&node.members(), visitor);
    visitor.leave_tuple_deconstruction_statement(node);
}

pub fn accept_variable_declaration_statement(
    node: &VariableDeclarationStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_statement(node) {
        return;
    }
    if let Some(ref storage_location) = node.storage_location() {
        accept_storage_location(storage_location, visitor);
    }
    accept_identifier(&node.name(), visitor);
    if let Some(ref value) = node.value() {
        accept_expression(value, visitor);
    }
    if let Some(ref type_name) = node.type_name() {
        accept_type_name(type_name, visitor);
    }
    visitor.leave_variable_declaration_statement(node);
}

pub fn accept_if_statement(node: &IfStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_if_statement(node) {
        return;
    }
    accept_expression(&node.condition(), visitor);
    accept_statement(&node.body(), visitor);
    if let Some(ref else_branch) = node.else_branch() {
        accept_statement(else_branch, visitor);
    }
    visitor.leave_if_statement(node);
}

pub fn accept_for_statement(node: &ForStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_for_statement(node) {
        return;
    }
    accept_for_statement_initialization(&node.initialization(), visitor);
    accept_for_statement_condition(&node.condition(), visitor);
    if let Some(ref iterator) = node.iterator() {
        accept_expression(iterator, visitor);
    }
    accept_statement(&node.body(), visitor);
    visitor.leave_for_statement(node);
}

pub fn accept_while_statement(node: &WhileStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_while_statement(node) {
        return;
    }
    accept_expression(&node.condition(), visitor);
    accept_statement(&node.body(), visitor);
    visitor.leave_while_statement(node);
}

pub fn accept_do_while_statement(node: &DoWhileStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_do_while_statement(node) {
        return;
    }
    accept_statement(&node.body(), visitor);
    accept_expression(&node.condition(), visitor);
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
    if let Some(ref expression) = node.expression() {
        accept_expression(expression, visitor);
    }
    visitor.leave_return_statement(node);
}

pub fn accept_emit_statement(node: &EmitStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_emit_statement(node) {
        return;
    }
    accept_identifier_path(&node.event(), visitor);
    accept_arguments_declaration(&node.arguments(), visitor);
    visitor.leave_emit_statement(node);
}

pub fn accept_try_statement(node: &TryStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_try_statement(node) {
        return;
    }
    accept_expression(&node.expression(), visitor);
    if let Some(ref returns) = node.returns() {
        accept_parameters(returns, visitor);
    }
    accept_block(&node.body(), visitor);
    accept_catch_clauses(&node.catch_clauses(), visitor);
    visitor.leave_try_statement(node);
}

pub fn accept_catch_clause(node: &CatchClause, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause(node) {
        return;
    }
    if let Some(ref error) = node.error() {
        accept_catch_clause_error(error, visitor);
    }
    accept_block(&node.body(), visitor);
    visitor.leave_catch_clause(node);
}

pub fn accept_catch_clause_error(node: &CatchClauseError, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause_error(node) {
        return;
    }
    if let Some(ref name) = node.name() {
        accept_identifier(name, visitor);
    }
    accept_parameters(&node.parameters(), visitor);
    visitor.leave_catch_clause_error(node);
}

pub fn accept_revert_statement(node: &RevertStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_revert_statement(node) {
        return;
    }
    if let Some(ref error) = node.error() {
        accept_identifier_path(error, visitor);
    }
    accept_arguments_declaration(&node.arguments(), visitor);
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
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_assignment_expression(node);
}

pub fn accept_conditional_expression(node: &ConditionalExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_conditional_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    accept_expression(&node.true_expression(), visitor);
    accept_expression(&node.false_expression(), visitor);
    visitor.leave_conditional_expression(node);
}

pub fn accept_or_expression(node: &OrExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_or_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_or_expression(node);
}

pub fn accept_and_expression(node: &AndExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_and_expression(node);
}

pub fn accept_equality_expression(node: &EqualityExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_equality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_equality_expression(node);
}

pub fn accept_inequality_expression(node: &InequalityExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_inequality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_inequality_expression(node);
}

pub fn accept_bitwise_or_expression(node: &BitwiseOrExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_or_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_bitwise_or_expression(node);
}

pub fn accept_bitwise_xor_expression(node: &BitwiseXorExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_xor_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_bitwise_xor_expression(node);
}

pub fn accept_bitwise_and_expression(node: &BitwiseAndExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_bitwise_and_expression(node);
}

pub fn accept_shift_expression(node: &ShiftExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_shift_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_shift_expression(node);
}

pub fn accept_additive_expression(node: &AdditiveExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_additive_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_additive_expression(node);
}

pub fn accept_multiplicative_expression(
    node: &MultiplicativeExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multiplicative_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_multiplicative_expression(node);
}

pub fn accept_exponentiation_expression(
    node: &ExponentiationExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_exponentiation_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_exponentiation_expression(node);
}

pub fn accept_postfix_expression(node: &PostfixExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_postfix_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    visitor.leave_postfix_expression(node);
}

pub fn accept_prefix_expression(node: &PrefixExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_prefix_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    visitor.leave_prefix_expression(node);
}

pub fn accept_function_call_expression(node: &FunctionCallExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_function_call_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    accept_arguments_declaration(&node.arguments(), visitor);
    visitor.leave_function_call_expression(node);
}

pub fn accept_call_options_expression(node: &CallOptionsExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_call_options_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    accept_call_options(&node.options(), visitor);
    visitor.leave_call_options_expression(node);
}

pub fn accept_member_access_expression(node: &MemberAccessExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_member_access_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    accept_identifier(&node.member(), visitor);
    visitor.leave_member_access_expression(node);
}

pub fn accept_index_access_expression(node: &IndexAccessExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_index_access_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    if let Some(ref start) = node.start() {
        accept_expression(start, visitor);
    }
    if let Some(ref end) = node.end() {
        accept_expression(end, visitor);
    }
    visitor.leave_index_access_expression(node);
}

pub fn accept_named_argument(node: &NamedArgument, visitor: &mut impl Visitor) {
    if !visitor.enter_named_argument(node) {
        return;
    }
    accept_identifier(&node.name(), visitor);
    accept_expression(&node.value(), visitor);
    visitor.leave_named_argument(node);
}

pub fn accept_type_expression(node: &TypeExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_type_expression(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    visitor.leave_type_expression(node);
}

pub fn accept_new_expression(node: &NewExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_new_expression(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    visitor.leave_new_expression(node);
}

pub fn accept_tuple_expression(node: &TupleExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_expression(node) {
        return;
    }
    accept_tuple_values(&node.items(), visitor);
    visitor.leave_tuple_expression(node);
}

pub fn accept_tuple_value(node: &TupleValue, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_value(node) {
        return;
    }
    if let Some(ref expression) = node.expression() {
        accept_expression(expression, visitor);
    }
    visitor.leave_tuple_value(node);
}

pub fn accept_array_expression(node: &ArrayExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_array_expression(node) {
        return;
    }
    accept_array_values(&node.items(), visitor);
    visitor.leave_array_expression(node);
}

pub fn accept_hex_number_expression(node: &HexNumberExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_hex_number_expression(node) {
        return;
    }
    if let Some(ref unit) = node.unit() {
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
    if let Some(ref unit) = node.unit() {
        accept_number_unit(unit, visitor);
    }
    visitor.leave_decimal_number_expression(node);
}

pub fn accept_yul_block(node: &YulBlock, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_block(node) {
        return;
    }
    accept_yul_statements(&node.statements(), visitor);
    visitor.leave_yul_block(node);
}

pub fn accept_yul_function_definition(node: &YulFunctionDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_function_definition(node) {
        return;
    }
    accept_yul_identifier(&node.name(), visitor);
    accept_yul_parameters(&node.parameters(), visitor);
    if let Some(ref returns) = node.returns() {
        accept_yul_variable_names(returns, visitor);
    }
    accept_yul_block(&node.body(), visitor);
    visitor.leave_yul_function_definition(node);
}

pub fn accept_yul_variable_declaration_statement(
    node: &YulVariableDeclarationStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_declaration_statement(node) {
        return;
    }
    accept_yul_variable_names(&node.variables(), visitor);
    if let Some(ref value) = node.value() {
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
    accept_yul_assignment_operator(&node.assignment(), visitor);
    accept_yul_expression(&node.expression(), visitor);
    visitor.leave_yul_variable_declaration_value(node);
}

pub fn accept_yul_variable_assignment_statement(
    node: &YulVariableAssignmentStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_assignment_statement(node) {
        return;
    }
    accept_yul_paths(&node.variables(), visitor);
    accept_yul_assignment_operator(&node.assignment(), visitor);
    accept_yul_expression(&node.expression(), visitor);
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
    accept_yul_stack_assignment_operator(&node.assignment(), visitor);
    accept_yul_identifier(&node.variable(), visitor);
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
    accept_yul_expression(&node.condition(), visitor);
    accept_yul_block(&node.body(), visitor);
    visitor.leave_yul_if_statement(node);
}

pub fn accept_yul_for_statement(node: &YulForStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_for_statement(node) {
        return;
    }
    accept_yul_block(&node.initialization(), visitor);
    accept_yul_expression(&node.condition(), visitor);
    accept_yul_block(&node.iterator(), visitor);
    accept_yul_block(&node.body(), visitor);
    visitor.leave_yul_for_statement(node);
}

pub fn accept_yul_switch_statement(node: &YulSwitchStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_statement(node) {
        return;
    }
    accept_yul_expression(&node.expression(), visitor);
    accept_yul_switch_cases(&node.cases(), visitor);
    visitor.leave_yul_switch_statement(node);
}

pub fn accept_yul_default_case(node: &YulDefaultCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_default_case(node) {
        return;
    }
    accept_yul_block(&node.body(), visitor);
    visitor.leave_yul_default_case(node);
}

pub fn accept_yul_value_case(node: &YulValueCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_value_case(node) {
        return;
    }
    accept_yul_literal(&node.value(), visitor);
    accept_yul_block(&node.body(), visitor);
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
    accept_yul_identifier(&node.label(), visitor);
    visitor.leave_yul_label(node);
}

pub fn accept_yul_function_call_expression(
    node: &YulFunctionCallExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_function_call_expression(node) {
        return;
    }
    accept_yul_expression(&node.operand(), visitor);
    accept_yul_arguments(&node.arguments(), visitor);
    visitor.leave_yul_function_call_expression(node);
}

//
// Choices:
//

#[allow(clippy::too_many_lines)]
pub fn accept_source_unit_member(node: &SourceUnitMember, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit_member(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::SourceUnitMember::PragmaDirective(_) => {
            accept_pragma_directive(&node.as_pragma_directive().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::ContractDefinition(_) => {
            accept_contract_definition(&node.as_contract_definition().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::InterfaceDefinition(_) => {
            accept_interface_definition(&node.as_interface_definition().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::LibraryDefinition(_) => {
            accept_library_definition(&node.as_library_definition().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::StructDefinition(_) => {
            accept_struct_definition(&node.as_struct_definition().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::EnumDefinition(_) => {
            accept_enum_definition(&node.as_enum_definition().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::FunctionDefinition(_) => {
            accept_function_definition(&node.as_function_definition().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::ErrorDefinition(_) => {
            accept_error_definition(&node.as_error_definition().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::UserDefinedValueTypeDefinition(_) => {
            accept_user_defined_value_type_definition(
                &node.as_user_defined_value_type_definition().unwrap(),
                visitor,
            );
        }
        input_ir::SourceUnitMember::UsingDirective(_) => {
            accept_using_directive(&node.as_using_directive().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::EventDefinition(_) => {
            accept_event_definition(&node.as_event_definition().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::ConstantDefinition(_) => {
            accept_constant_definition(&node.as_constant_definition().unwrap(), visitor);
        }
        input_ir::SourceUnitMember::ImportClause(_) => {
            accept_import_clause(&node.as_import_clause().unwrap(), visitor);
        }
    }
    visitor.leave_source_unit_member(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_pragma(node: &Pragma, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::Pragma::VersionPragma(_) => {
            accept_version_pragma(&node.as_version_pragma().unwrap(), visitor);
        }
        input_ir::Pragma::AbicoderPragma(_) => {
            accept_abicoder_pragma(&node.as_abicoder_pragma().unwrap(), visitor);
        }
        input_ir::Pragma::ExperimentalPragma(_) => {
            accept_experimental_pragma(&node.as_experimental_pragma().unwrap(), visitor);
        }
    }
    visitor.leave_pragma(node);
}

pub fn accept_abicoder_version(_node: &AbicoderVersion, _visitor: &mut impl Visitor) {}

#[allow(clippy::too_many_lines)]
pub fn accept_experimental_feature(node: &ExperimentalFeature, visitor: &mut impl Visitor) {
    if !visitor.enter_experimental_feature(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::ExperimentalFeature::StringLiteral(_) => {}
        _ => {}
    }
    visitor.leave_experimental_feature(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_version_expression(node: &VersionExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::VersionExpression::VersionRange(_) => {
            accept_version_range(&node.as_version_range().unwrap(), visitor);
        }
        input_ir::VersionExpression::VersionTerm(_) => {
            accept_version_term(&node.as_version_term().unwrap(), visitor);
        }
    }
    visitor.leave_version_expression(node);
}

pub fn accept_version_operator(_node: &VersionOperator, _visitor: &mut impl Visitor) {}

#[allow(clippy::too_many_lines)]
pub fn accept_version_literal(node: &VersionLiteral, visitor: &mut impl Visitor) {
    if !visitor.enter_version_literal(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::VersionLiteral::SimpleVersionLiteral(_) => {}
        input_ir::VersionLiteral::SingleQuotedVersionLiteral(_) => {}
        input_ir::VersionLiteral::DoubleQuotedVersionLiteral(_) => {}
    }
    visitor.leave_version_literal(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_import_clause(node: &ImportClause, visitor: &mut impl Visitor) {
    if !visitor.enter_import_clause(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::ImportClause::PathImport(_) => {
            accept_path_import(&node.as_path_import().unwrap(), visitor);
        }
        input_ir::ImportClause::ImportDeconstruction(_) => {
            accept_import_deconstruction(&node.as_import_deconstruction().unwrap(), visitor);
        }
    }
    visitor.leave_import_clause(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_using_clause(node: &UsingClause, visitor: &mut impl Visitor) {
    if !visitor.enter_using_clause(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::UsingClause::IdentifierPath(_) => {
            accept_identifier_path(&node.as_identifier_path().unwrap(), visitor);
        }
        input_ir::UsingClause::UsingDeconstruction(_) => {
            accept_using_deconstruction(&node.as_using_deconstruction().unwrap(), visitor);
        }
    }
    visitor.leave_using_clause(node);
}

pub fn accept_using_operator(_node: &UsingOperator, _visitor: &mut impl Visitor) {}

#[allow(clippy::too_many_lines)]
pub fn accept_using_target(node: &UsingTarget, visitor: &mut impl Visitor) {
    if !visitor.enter_using_target(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::UsingTarget::TypeName(_) => {
            accept_type_name(&node.as_type_name().unwrap(), visitor);
        }

        _ => {}
    }
    visitor.leave_using_target(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_contract_member(node: &ContractMember, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_member(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::ContractMember::UsingDirective(_) => {
            accept_using_directive(&node.as_using_directive().unwrap(), visitor);
        }
        input_ir::ContractMember::FunctionDefinition(_) => {
            accept_function_definition(&node.as_function_definition().unwrap(), visitor);
        }
        input_ir::ContractMember::StructDefinition(_) => {
            accept_struct_definition(&node.as_struct_definition().unwrap(), visitor);
        }
        input_ir::ContractMember::EnumDefinition(_) => {
            accept_enum_definition(&node.as_enum_definition().unwrap(), visitor);
        }
        input_ir::ContractMember::EventDefinition(_) => {
            accept_event_definition(&node.as_event_definition().unwrap(), visitor);
        }
        input_ir::ContractMember::ErrorDefinition(_) => {
            accept_error_definition(&node.as_error_definition().unwrap(), visitor);
        }
        input_ir::ContractMember::UserDefinedValueTypeDefinition(_) => {
            accept_user_defined_value_type_definition(
                &node.as_user_defined_value_type_definition().unwrap(),
                visitor,
            );
        }
        input_ir::ContractMember::StateVariableDefinition(_) => {
            accept_state_variable_definition(
                &node.as_state_variable_definition().unwrap(),
                visitor,
            );
        }
        input_ir::ContractMember::ConstantDefinition(_) => {
            accept_constant_definition(&node.as_constant_definition().unwrap(), visitor);
        }
    }
    visitor.leave_contract_member(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_type_name(node: &TypeName, visitor: &mut impl Visitor) {
    if !visitor.enter_type_name(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::TypeName::ArrayTypeName(_) => {
            accept_array_type_name(&node.as_array_type_name().unwrap(), visitor);
        }
        input_ir::TypeName::FunctionType(_) => {
            accept_function_type(&node.as_function_type().unwrap(), visitor);
        }
        input_ir::TypeName::MappingType(_) => {
            accept_mapping_type(&node.as_mapping_type().unwrap(), visitor);
        }
        input_ir::TypeName::ElementaryType(_) => {
            accept_elementary_type(&node.as_elementary_type().unwrap(), visitor);
        }
        input_ir::TypeName::IdentifierPath(_) => {
            accept_identifier_path(&node.as_identifier_path().unwrap(), visitor);
        }
    }
    visitor.leave_type_name(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_elementary_type(node: &ElementaryType, visitor: &mut impl Visitor) {
    if !visitor.enter_elementary_type(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::ElementaryType::AddressType(_) => {
            accept_address_type(&node.as_address_type().unwrap(), visitor);
        }
        input_ir::ElementaryType::BytesKeyword(_) => {}
        input_ir::ElementaryType::IntKeyword(_) => {}
        input_ir::ElementaryType::UintKeyword(_) => {}
        input_ir::ElementaryType::FixedKeyword(_) => {}
        input_ir::ElementaryType::UfixedKeyword(_) => {}
        _ => {}
    }
    visitor.leave_elementary_type(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_statement(node: &Statement, visitor: &mut impl Visitor) {
    if !visitor.enter_statement(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::Statement::IfStatement(_) => {
            accept_if_statement(&node.as_if_statement().unwrap(), visitor);
        }
        input_ir::Statement::ForStatement(_) => {
            accept_for_statement(&node.as_for_statement().unwrap(), visitor);
        }
        input_ir::Statement::WhileStatement(_) => {
            accept_while_statement(&node.as_while_statement().unwrap(), visitor);
        }
        input_ir::Statement::DoWhileStatement(_) => {
            accept_do_while_statement(&node.as_do_while_statement().unwrap(), visitor);
        }
        input_ir::Statement::ContinueStatement(_) => {
            accept_continue_statement(&node.as_continue_statement().unwrap(), visitor);
        }
        input_ir::Statement::BreakStatement(_) => {
            accept_break_statement(&node.as_break_statement().unwrap(), visitor);
        }
        input_ir::Statement::ReturnStatement(_) => {
            accept_return_statement(&node.as_return_statement().unwrap(), visitor);
        }
        input_ir::Statement::ThrowStatement(_) => {
            accept_throw_statement(&node.as_throw_statement().unwrap(), visitor);
        }
        input_ir::Statement::EmitStatement(_) => {
            accept_emit_statement(&node.as_emit_statement().unwrap(), visitor);
        }
        input_ir::Statement::TryStatement(_) => {
            accept_try_statement(&node.as_try_statement().unwrap(), visitor);
        }
        input_ir::Statement::RevertStatement(_) => {
            accept_revert_statement(&node.as_revert_statement().unwrap(), visitor);
        }
        input_ir::Statement::AssemblyStatement(_) => {
            accept_assembly_statement(&node.as_assembly_statement().unwrap(), visitor);
        }
        input_ir::Statement::Block(_) => {
            accept_block(&node.as_block().unwrap(), visitor);
        }
        input_ir::Statement::UncheckedBlock(_) => {
            accept_unchecked_block(&node.as_unchecked_block().unwrap(), visitor);
        }
        input_ir::Statement::TupleDeconstructionStatement(_) => {
            accept_tuple_deconstruction_statement(
                &node.as_tuple_deconstruction_statement().unwrap(),
                visitor,
            );
        }
        input_ir::Statement::VariableDeclarationStatement(_) => {
            accept_variable_declaration_statement(
                &node.as_variable_declaration_statement().unwrap(),
                visitor,
            );
        }
        input_ir::Statement::ExpressionStatement(_) => {
            accept_expression_statement(&node.as_expression_statement().unwrap(), visitor);
        }
    }
    visitor.leave_statement(node);
}

pub fn accept_storage_location(_node: &StorageLocation, _visitor: &mut impl Visitor) {}

#[allow(clippy::too_many_lines)]
pub fn accept_for_statement_initialization(
    node: &ForStatementInitialization,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_for_statement_initialization(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::ForStatementInitialization::TupleDeconstructionStatement(_) => {
            accept_tuple_deconstruction_statement(
                &node.as_tuple_deconstruction_statement().unwrap(),
                visitor,
            );
        }
        input_ir::ForStatementInitialization::VariableDeclarationStatement(_) => {
            accept_variable_declaration_statement(
                &node.as_variable_declaration_statement().unwrap(),
                visitor,
            );
        }
        input_ir::ForStatementInitialization::ExpressionStatement(_) => {
            accept_expression_statement(&node.as_expression_statement().unwrap(), visitor);
        }

        _ => {}
    }
    visitor.leave_for_statement_initialization(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_for_statement_condition(node: &ForStatementCondition, visitor: &mut impl Visitor) {
    if !visitor.enter_for_statement_condition(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::ForStatementCondition::ExpressionStatement(_) => {
            accept_expression_statement(&node.as_expression_statement().unwrap(), visitor);
        }

        _ => {}
    }
    visitor.leave_for_statement_condition(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_expression(node: &Expression, visitor: &mut impl Visitor) {
    if !visitor.enter_expression(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::Expression::AssignmentExpression(_) => {
            accept_assignment_expression(&node.as_assignment_expression().unwrap(), visitor);
        }
        input_ir::Expression::ConditionalExpression(_) => {
            accept_conditional_expression(&node.as_conditional_expression().unwrap(), visitor);
        }
        input_ir::Expression::OrExpression(_) => {
            accept_or_expression(&node.as_or_expression().unwrap(), visitor);
        }
        input_ir::Expression::AndExpression(_) => {
            accept_and_expression(&node.as_and_expression().unwrap(), visitor);
        }
        input_ir::Expression::EqualityExpression(_) => {
            accept_equality_expression(&node.as_equality_expression().unwrap(), visitor);
        }
        input_ir::Expression::InequalityExpression(_) => {
            accept_inequality_expression(&node.as_inequality_expression().unwrap(), visitor);
        }
        input_ir::Expression::BitwiseOrExpression(_) => {
            accept_bitwise_or_expression(&node.as_bitwise_or_expression().unwrap(), visitor);
        }
        input_ir::Expression::BitwiseXorExpression(_) => {
            accept_bitwise_xor_expression(&node.as_bitwise_xor_expression().unwrap(), visitor);
        }
        input_ir::Expression::BitwiseAndExpression(_) => {
            accept_bitwise_and_expression(&node.as_bitwise_and_expression().unwrap(), visitor);
        }
        input_ir::Expression::ShiftExpression(_) => {
            accept_shift_expression(&node.as_shift_expression().unwrap(), visitor);
        }
        input_ir::Expression::AdditiveExpression(_) => {
            accept_additive_expression(&node.as_additive_expression().unwrap(), visitor);
        }
        input_ir::Expression::MultiplicativeExpression(_) => {
            accept_multiplicative_expression(
                &node.as_multiplicative_expression().unwrap(),
                visitor,
            );
        }
        input_ir::Expression::ExponentiationExpression(_) => {
            accept_exponentiation_expression(
                &node.as_exponentiation_expression().unwrap(),
                visitor,
            );
        }
        input_ir::Expression::PostfixExpression(_) => {
            accept_postfix_expression(&node.as_postfix_expression().unwrap(), visitor);
        }
        input_ir::Expression::PrefixExpression(_) => {
            accept_prefix_expression(&node.as_prefix_expression().unwrap(), visitor);
        }
        input_ir::Expression::FunctionCallExpression(_) => {
            accept_function_call_expression(&node.as_function_call_expression().unwrap(), visitor);
        }
        input_ir::Expression::CallOptionsExpression(_) => {
            accept_call_options_expression(&node.as_call_options_expression().unwrap(), visitor);
        }
        input_ir::Expression::MemberAccessExpression(_) => {
            accept_member_access_expression(&node.as_member_access_expression().unwrap(), visitor);
        }
        input_ir::Expression::IndexAccessExpression(_) => {
            accept_index_access_expression(&node.as_index_access_expression().unwrap(), visitor);
        }
        input_ir::Expression::NewExpression(_) => {
            accept_new_expression(&node.as_new_expression().unwrap(), visitor);
        }
        input_ir::Expression::TupleExpression(_) => {
            accept_tuple_expression(&node.as_tuple_expression().unwrap(), visitor);
        }
        input_ir::Expression::TypeExpression(_) => {
            accept_type_expression(&node.as_type_expression().unwrap(), visitor);
        }
        input_ir::Expression::ArrayExpression(_) => {
            accept_array_expression(&node.as_array_expression().unwrap(), visitor);
        }
        input_ir::Expression::HexNumberExpression(_) => {
            accept_hex_number_expression(&node.as_hex_number_expression().unwrap(), visitor);
        }
        input_ir::Expression::DecimalNumberExpression(_) => {
            accept_decimal_number_expression(
                &node.as_decimal_number_expression().unwrap(),
                visitor,
            );
        }
        input_ir::Expression::StringExpression(_) => {
            accept_string_expression(&node.as_string_expression().unwrap(), visitor);
        }
        input_ir::Expression::ElementaryType(_) => {
            accept_elementary_type(&node.as_elementary_type().unwrap(), visitor);
        }
        input_ir::Expression::Identifier(_) => {
            accept_identifier(&node.as_identifier().unwrap(), visitor);
        }
        _ => {}
    }
    visitor.leave_expression(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_arguments_declaration(node: &ArgumentsDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_arguments_declaration(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::ArgumentsDeclaration::PositionalArguments(_) => {
            accept_positional_arguments(&node.as_positional_arguments().unwrap(), visitor);
        }
        input_ir::ArgumentsDeclaration::NamedArguments(_) => {
            accept_named_arguments(&node.as_named_arguments().unwrap(), visitor);
        }
    }
    visitor.leave_arguments_declaration(node);
}

pub fn accept_number_unit(_node: &NumberUnit, _visitor: &mut impl Visitor) {}

#[allow(clippy::too_many_lines)]
pub fn accept_string_expression(node: &StringExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_string_expression(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::StringExpression::Strings(_) => {}
        input_ir::StringExpression::HexStrings(_) => {}
        input_ir::StringExpression::UnicodeStrings(_) => {}
    }
    visitor.leave_string_expression(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_yul_statement(node: &YulStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_statement(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::YulStatement::YulBlock(_) => {
            accept_yul_block(&node.as_yul_block().unwrap(), visitor);
        }
        input_ir::YulStatement::YulFunctionDefinition(_) => {
            accept_yul_function_definition(&node.as_yul_function_definition().unwrap(), visitor);
        }
        input_ir::YulStatement::YulStackAssignmentStatement(_) => {
            accept_yul_stack_assignment_statement(
                &node.as_yul_stack_assignment_statement().unwrap(),
                visitor,
            );
        }
        input_ir::YulStatement::YulIfStatement(_) => {
            accept_yul_if_statement(&node.as_yul_if_statement().unwrap(), visitor);
        }
        input_ir::YulStatement::YulForStatement(_) => {
            accept_yul_for_statement(&node.as_yul_for_statement().unwrap(), visitor);
        }
        input_ir::YulStatement::YulSwitchStatement(_) => {
            accept_yul_switch_statement(&node.as_yul_switch_statement().unwrap(), visitor);
        }
        input_ir::YulStatement::YulLeaveStatement(_) => {
            accept_yul_leave_statement(&node.as_yul_leave_statement().unwrap(), visitor);
        }
        input_ir::YulStatement::YulBreakStatement(_) => {
            accept_yul_break_statement(&node.as_yul_break_statement().unwrap(), visitor);
        }
        input_ir::YulStatement::YulContinueStatement(_) => {
            accept_yul_continue_statement(&node.as_yul_continue_statement().unwrap(), visitor);
        }
        input_ir::YulStatement::YulVariableAssignmentStatement(_) => {
            accept_yul_variable_assignment_statement(
                &node.as_yul_variable_assignment_statement().unwrap(),
                visitor,
            );
        }
        input_ir::YulStatement::YulLabel(_) => {
            accept_yul_label(&node.as_yul_label().unwrap(), visitor);
        }
        input_ir::YulStatement::YulVariableDeclarationStatement(_) => {
            accept_yul_variable_declaration_statement(
                &node.as_yul_variable_declaration_statement().unwrap(),
                visitor,
            );
        }
        input_ir::YulStatement::YulExpression(_) => {
            accept_yul_expression(&node.as_yul_expression().unwrap(), visitor);
        }
    }
    visitor.leave_yul_statement(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_yul_assignment_operator(node: &YulAssignmentOperator, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_assignment_operator(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::YulAssignmentOperator::YulColonAndEqual(_) => {
            accept_yul_colon_and_equal(&node.as_yul_colon_and_equal().unwrap(), visitor);
        }

        _ => {}
    }
    visitor.leave_yul_assignment_operator(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_yul_stack_assignment_operator(
    node: &YulStackAssignmentOperator,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_stack_assignment_operator(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::YulStackAssignmentOperator::YulEqualAndColon(_) => {
            accept_yul_equal_and_colon(&node.as_yul_equal_and_colon().unwrap(), visitor);
        }

        _ => {}
    }
    visitor.leave_yul_stack_assignment_operator(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_yul_switch_case(node: &YulSwitchCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_case(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::YulSwitchCase::YulDefaultCase(_) => {
            accept_yul_default_case(&node.as_yul_default_case().unwrap(), visitor);
        }
        input_ir::YulSwitchCase::YulValueCase(_) => {
            accept_yul_value_case(&node.as_yul_value_case().unwrap(), visitor);
        }
    }
    visitor.leave_yul_switch_case(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_yul_expression(node: &YulExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_expression(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::YulExpression::YulFunctionCallExpression(_) => {
            accept_yul_function_call_expression(
                &node.as_yul_function_call_expression().unwrap(),
                visitor,
            );
        }
        input_ir::YulExpression::YulLiteral(_) => {
            accept_yul_literal(&node.as_yul_literal().unwrap(), visitor);
        }
        input_ir::YulExpression::YulPath(_) => {
            accept_yul_path(&node.as_yul_path().unwrap(), visitor);
        }
    }
    visitor.leave_yul_expression(node);
}

#[allow(clippy::too_many_lines)]
pub fn accept_yul_literal(node: &YulLiteral, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_literal(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::YulLiteral::YulDecimalLiteral(_) => {}
        input_ir::YulLiteral::YulHexLiteral(_) => {}
        input_ir::YulLiteral::StringLiteral(_) => {}
        input_ir::YulLiteral::HexStringLiteral(_) => {}
        _ => {}
    }
    visitor.leave_yul_literal(node);
}

pub fn accept_function_kind(_node: &FunctionKind, _visitor: &mut impl Visitor) {}

pub fn accept_function_visibility(_node: &FunctionVisibility, _visitor: &mut impl Visitor) {}

pub fn accept_function_mutability(_node: &FunctionMutability, _visitor: &mut impl Visitor) {}

pub fn accept_state_variable_visibility(
    _node: &StateVariableVisibility,
    _visitor: &mut impl Visitor,
) {
}

pub fn accept_state_variable_mutability(
    _node: &StateVariableMutability,
    _visitor: &mut impl Visitor,
) {
}

#[allow(clippy::too_many_lines)]
pub fn accept_tuple_deconstruction_member(
    node: &TupleDeconstructionMember,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_tuple_deconstruction_member(node) {
        return;
    }
    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    match &node.ir_node {
        input_ir::TupleDeconstructionMember::VariableDeclarationStatement(_) => {
            accept_variable_declaration_statement(
                &node.as_variable_declaration_statement().unwrap(),
                visitor,
            );
        }
        input_ir::TupleDeconstructionMember::Identifier(_) => {
            accept_identifier(&node.as_identifier().unwrap(), visitor);
        }
        _ => {}
    }
    visitor.leave_tuple_deconstruction_member(node);
}

//
// Repeated & Separated
//

pub fn accept_source_unit_members(items: &SourceUnitMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit_members(items) {
        return;
    }
    for item in items.iter() {
        accept_source_unit_member(&item, visitor);
    }
    visitor.leave_source_unit_members(items);
}

pub fn accept_version_expression_sets(items: &VersionExpressionSets, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression_sets(items) {
        return;
    }
    for item in items.iter() {
        accept_version_expression_set(&item, visitor);
    }
    visitor.leave_version_expression_sets(items);
}

pub fn accept_version_expression_set(items: &VersionExpressionSet, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression_set(items) {
        return;
    }
    for item in items.iter() {
        accept_version_expression(&item, visitor);
    }
    visitor.leave_version_expression_set(items);
}

pub fn accept_simple_version_literal(items: &[Rc<TerminalNode>], visitor: &mut impl Visitor) {
    if !visitor.enter_simple_version_literal(items) {
        return;
    }
    visitor.leave_simple_version_literal(items);
}

pub fn accept_import_deconstruction_symbols(
    items: &ImportDeconstructionSymbols,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_import_deconstruction_symbols(items) {
        return;
    }
    for item in items.iter() {
        accept_import_deconstruction_symbol(&item, visitor);
    }
    visitor.leave_import_deconstruction_symbols(items);
}

pub fn accept_using_deconstruction_symbols(
    items: &UsingDeconstructionSymbols,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_using_deconstruction_symbols(items) {
        return;
    }
    for item in items.iter() {
        accept_using_deconstruction_symbol(&item, visitor);
    }
    visitor.leave_using_deconstruction_symbols(items);
}

pub fn accept_inheritance_types(items: &InheritanceTypes, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_types(items) {
        return;
    }
    for item in items.iter() {
        accept_inheritance_type(&item, visitor);
    }
    visitor.leave_inheritance_types(items);
}

pub fn accept_contract_members(items: &ContractMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_members(items) {
        return;
    }
    for item in items.iter() {
        accept_contract_member(&item, visitor);
    }
    visitor.leave_contract_members(items);
}

pub fn accept_interface_members(items: &InterfaceMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_interface_members(items) {
        return;
    }
    for item in items.iter() {
        accept_contract_member(&item, visitor);
    }
    visitor.leave_interface_members(items);
}

pub fn accept_library_members(items: &LibraryMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_library_members(items) {
        return;
    }
    for item in items.iter() {
        accept_contract_member(&item, visitor);
    }
    visitor.leave_library_members(items);
}

pub fn accept_struct_members(items: &StructMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_members(items) {
        return;
    }
    for item in items.iter() {
        accept_struct_member(&item, visitor);
    }
    visitor.leave_struct_members(items);
}

pub fn accept_enum_members(items: &EnumMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_enum_members(items) {
        return;
    }
    for item in items.iter() {
        accept_identifier(&item, visitor);
    }
    visitor.leave_enum_members(items);
}

pub fn accept_parameters(items: &Parameters, visitor: &mut impl Visitor) {
    if !visitor.enter_parameters(items) {
        return;
    }
    for item in items.iter() {
        accept_parameter(&item, visitor);
    }
    visitor.leave_parameters(items);
}

pub fn accept_override_paths(items: &OverridePaths, visitor: &mut impl Visitor) {
    if !visitor.enter_override_paths(items) {
        return;
    }
    for item in items.iter() {
        accept_identifier_path(&item, visitor);
    }
    visitor.leave_override_paths(items);
}

pub fn accept_statements(items: &Statements, visitor: &mut impl Visitor) {
    if !visitor.enter_statements(items) {
        return;
    }
    for item in items.iter() {
        accept_statement(&item, visitor);
    }
    visitor.leave_statements(items);
}

pub fn accept_catch_clauses(items: &CatchClauses, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clauses(items) {
        return;
    }
    for item in items.iter() {
        accept_catch_clause(&item, visitor);
    }
    visitor.leave_catch_clauses(items);
}

pub fn accept_positional_arguments(items: &PositionalArguments, visitor: &mut impl Visitor) {
    if !visitor.enter_positional_arguments(items) {
        return;
    }
    for item in items.iter() {
        accept_expression(&item, visitor);
    }
    visitor.leave_positional_arguments(items);
}

pub fn accept_named_arguments(items: &NamedArguments, visitor: &mut impl Visitor) {
    if !visitor.enter_named_arguments(items) {
        return;
    }
    for item in items.iter() {
        accept_named_argument(&item, visitor);
    }
    visitor.leave_named_arguments(items);
}

pub fn accept_call_options(items: &CallOptions, visitor: &mut impl Visitor) {
    if !visitor.enter_call_options(items) {
        return;
    }
    for item in items.iter() {
        accept_named_argument(&item, visitor);
    }
    visitor.leave_call_options(items);
}

pub fn accept_tuple_values(items: &TupleValues, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_values(items) {
        return;
    }
    for item in items.iter() {
        accept_tuple_value(&item, visitor);
    }
    visitor.leave_tuple_values(items);
}

pub fn accept_array_values(items: &ArrayValues, visitor: &mut impl Visitor) {
    if !visitor.enter_array_values(items) {
        return;
    }
    for item in items.iter() {
        accept_expression(&item, visitor);
    }
    visitor.leave_array_values(items);
}

pub fn accept_identifier_path(items: &IdentifierPath, visitor: &mut impl Visitor) {
    if !visitor.enter_identifier_path(items) {
        return;
    }
    for item in items.iter() {
        accept_identifier(&item, visitor);
    }
    visitor.leave_identifier_path(items);
}

pub fn accept_yul_statements(items: &YulStatements, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_statements(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_statement(&item, visitor);
    }
    visitor.leave_yul_statements(items);
}

pub fn accept_yul_parameters(items: &YulParameters, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_parameters(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_identifier(&item, visitor);
    }
    visitor.leave_yul_parameters(items);
}

pub fn accept_yul_variable_names(items: &YulVariableNames, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_variable_names(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_identifier(&item, visitor);
    }
    visitor.leave_yul_variable_names(items);
}

pub fn accept_yul_switch_cases(items: &YulSwitchCases, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_cases(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_switch_case(&item, visitor);
    }
    visitor.leave_yul_switch_cases(items);
}

pub fn accept_yul_arguments(items: &YulArguments, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_arguments(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_expression(&item, visitor);
    }
    visitor.leave_yul_arguments(items);
}

pub fn accept_yul_paths(items: &YulPaths, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_paths(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_path(&item, visitor);
    }
    visitor.leave_yul_paths(items);
}

pub fn accept_yul_path(items: &YulPath, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_path(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_identifier(&item, visitor);
    }
    visitor.leave_yul_path(items);
}

pub fn accept_modifier_invocations(items: &ModifierInvocations, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_invocations(items) {
        return;
    }
    for item in items.iter() {
        accept_modifier_invocation(&item, visitor);
    }
    visitor.leave_modifier_invocations(items);
}

pub fn accept_strings(items: &[Rc<TerminalNode>], visitor: &mut impl Visitor) {
    if !visitor.enter_strings(items) {
        return;
    }
    visitor.leave_strings(items);
}

pub fn accept_hex_strings(items: &[Rc<TerminalNode>], visitor: &mut impl Visitor) {
    if !visitor.enter_hex_strings(items) {
        return;
    }
    visitor.leave_hex_strings(items);
}

pub fn accept_unicode_strings(items: &[Rc<TerminalNode>], visitor: &mut impl Visitor) {
    if !visitor.enter_unicode_strings(items) {
        return;
    }
    visitor.leave_unicode_strings(items);
}

pub fn accept_assembly_flags(items: &[Rc<TerminalNode>], visitor: &mut impl Visitor) {
    if !visitor.enter_assembly_flags(items) {
        return;
    }
    visitor.leave_assembly_flags(items);
}

pub fn accept_tuple_deconstruction_members(
    items: &TupleDeconstructionMembers,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_tuple_deconstruction_members(items) {
        return;
    }
    for item in items.iter() {
        accept_tuple_deconstruction_member(&item, visitor);
    }
    visitor.leave_tuple_deconstruction_members(items);
}
