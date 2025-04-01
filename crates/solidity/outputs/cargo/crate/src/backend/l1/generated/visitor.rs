// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::TerminalNode;

pub trait Visitor {
    fn enter_source_unit(&mut self, _target: &SourceUnit) -> bool {
        true
    }
    fn leave_source_unit(&mut self, _target: &SourceUnit) {}

    fn enter_pragma_directive(&mut self, _target: &PragmaDirective) -> bool {
        true
    }
    fn leave_pragma_directive(&mut self, _target: &PragmaDirective) {}

    fn enter_abicoder_pragma(&mut self, _target: &AbicoderPragma) -> bool {
        true
    }
    fn leave_abicoder_pragma(&mut self, _target: &AbicoderPragma) {}

    fn enter_experimental_pragma(&mut self, _target: &ExperimentalPragma) -> bool {
        true
    }
    fn leave_experimental_pragma(&mut self, _target: &ExperimentalPragma) {}

    fn enter_version_pragma(&mut self, _target: &VersionPragma) -> bool {
        true
    }
    fn leave_version_pragma(&mut self, _target: &VersionPragma) {}

    fn enter_version_range(&mut self, _target: &VersionRange) -> bool {
        true
    }
    fn leave_version_range(&mut self, _target: &VersionRange) {}

    fn enter_version_term(&mut self, _target: &VersionTerm) -> bool {
        true
    }
    fn leave_version_term(&mut self, _target: &VersionTerm) {}

    fn enter_import_directive(&mut self, _target: &ImportDirective) -> bool {
        true
    }
    fn leave_import_directive(&mut self, _target: &ImportDirective) {}

    fn enter_path_import(&mut self, _target: &PathImport) -> bool {
        true
    }
    fn leave_path_import(&mut self, _target: &PathImport) {}

    fn enter_named_import(&mut self, _target: &NamedImport) -> bool {
        true
    }
    fn leave_named_import(&mut self, _target: &NamedImport) {}

    fn enter_import_deconstruction(&mut self, _target: &ImportDeconstruction) -> bool {
        true
    }
    fn leave_import_deconstruction(&mut self, _target: &ImportDeconstruction) {}

    fn enter_import_deconstruction_symbol(&mut self, _target: &ImportDeconstructionSymbol) -> bool {
        true
    }
    fn leave_import_deconstruction_symbol(&mut self, _target: &ImportDeconstructionSymbol) {}

    fn enter_import_alias(&mut self, _target: &ImportAlias) -> bool {
        true
    }
    fn leave_import_alias(&mut self, _target: &ImportAlias) {}

    fn enter_using_directive(&mut self, _target: &UsingDirective) -> bool {
        true
    }
    fn leave_using_directive(&mut self, _target: &UsingDirective) {}

    fn enter_using_deconstruction(&mut self, _target: &UsingDeconstruction) -> bool {
        true
    }
    fn leave_using_deconstruction(&mut self, _target: &UsingDeconstruction) {}

    fn enter_using_deconstruction_symbol(&mut self, _target: &UsingDeconstructionSymbol) -> bool {
        true
    }
    fn leave_using_deconstruction_symbol(&mut self, _target: &UsingDeconstructionSymbol) {}

    fn enter_using_alias(&mut self, _target: &UsingAlias) -> bool {
        true
    }
    fn leave_using_alias(&mut self, _target: &UsingAlias) {}

    fn enter_contract_definition(&mut self, _target: &ContractDefinition) -> bool {
        true
    }
    fn leave_contract_definition(&mut self, _target: &ContractDefinition) {}

    fn enter_inheritance_specifier(&mut self, _target: &InheritanceSpecifier) -> bool {
        true
    }
    fn leave_inheritance_specifier(&mut self, _target: &InheritanceSpecifier) {}

    fn enter_inheritance_type(&mut self, _target: &InheritanceType) -> bool {
        true
    }
    fn leave_inheritance_type(&mut self, _target: &InheritanceType) {}

    fn enter_storage_layout_specifier(&mut self, _target: &StorageLayoutSpecifier) -> bool {
        true
    }
    fn leave_storage_layout_specifier(&mut self, _target: &StorageLayoutSpecifier) {}

    fn enter_interface_definition(&mut self, _target: &InterfaceDefinition) -> bool {
        true
    }
    fn leave_interface_definition(&mut self, _target: &InterfaceDefinition) {}

    fn enter_library_definition(&mut self, _target: &LibraryDefinition) -> bool {
        true
    }
    fn leave_library_definition(&mut self, _target: &LibraryDefinition) {}

    fn enter_struct_definition(&mut self, _target: &StructDefinition) -> bool {
        true
    }
    fn leave_struct_definition(&mut self, _target: &StructDefinition) {}

    fn enter_struct_member(&mut self, _target: &StructMember) -> bool {
        true
    }
    fn leave_struct_member(&mut self, _target: &StructMember) {}

    fn enter_enum_definition(&mut self, _target: &EnumDefinition) -> bool {
        true
    }
    fn leave_enum_definition(&mut self, _target: &EnumDefinition) {}

    fn enter_constant_definition(&mut self, _target: &ConstantDefinition) -> bool {
        true
    }
    fn leave_constant_definition(&mut self, _target: &ConstantDefinition) {}

    fn enter_state_variable_definition(&mut self, _target: &StateVariableDefinition) -> bool {
        true
    }
    fn leave_state_variable_definition(&mut self, _target: &StateVariableDefinition) {}

    fn enter_state_variable_definition_value(
        &mut self,
        _target: &StateVariableDefinitionValue,
    ) -> bool {
        true
    }
    fn leave_state_variable_definition_value(&mut self, _target: &StateVariableDefinitionValue) {}

    fn enter_function_definition(&mut self, _target: &FunctionDefinition) -> bool {
        true
    }
    fn leave_function_definition(&mut self, _target: &FunctionDefinition) {}

    fn enter_parameters_declaration(&mut self, _target: &ParametersDeclaration) -> bool {
        true
    }
    fn leave_parameters_declaration(&mut self, _target: &ParametersDeclaration) {}

    fn enter_parameter(&mut self, _target: &Parameter) -> bool {
        true
    }
    fn leave_parameter(&mut self, _target: &Parameter) {}

    fn enter_override_specifier(&mut self, _target: &OverrideSpecifier) -> bool {
        true
    }
    fn leave_override_specifier(&mut self, _target: &OverrideSpecifier) {}

    fn enter_override_paths_declaration(&mut self, _target: &OverridePathsDeclaration) -> bool {
        true
    }
    fn leave_override_paths_declaration(&mut self, _target: &OverridePathsDeclaration) {}

    fn enter_returns_declaration(&mut self, _target: &ReturnsDeclaration) -> bool {
        true
    }
    fn leave_returns_declaration(&mut self, _target: &ReturnsDeclaration) {}

    fn enter_constructor_definition(&mut self, _target: &ConstructorDefinition) -> bool {
        true
    }
    fn leave_constructor_definition(&mut self, _target: &ConstructorDefinition) {}

    fn enter_fallback_function_definition(&mut self, _target: &FallbackFunctionDefinition) -> bool {
        true
    }
    fn leave_fallback_function_definition(&mut self, _target: &FallbackFunctionDefinition) {}

    fn enter_receive_function_definition(&mut self, _target: &ReceiveFunctionDefinition) -> bool {
        true
    }
    fn leave_receive_function_definition(&mut self, _target: &ReceiveFunctionDefinition) {}

    fn enter_modifier_definition(&mut self, _target: &ModifierDefinition) -> bool {
        true
    }
    fn leave_modifier_definition(&mut self, _target: &ModifierDefinition) {}

    fn enter_modifier_invocation(&mut self, _target: &ModifierInvocation) -> bool {
        true
    }
    fn leave_modifier_invocation(&mut self, _target: &ModifierInvocation) {}

    fn enter_event_definition(&mut self, _target: &EventDefinition) -> bool {
        true
    }
    fn leave_event_definition(&mut self, _target: &EventDefinition) {}

    fn enter_event_parameters_declaration(&mut self, _target: &EventParametersDeclaration) -> bool {
        true
    }
    fn leave_event_parameters_declaration(&mut self, _target: &EventParametersDeclaration) {}

    fn enter_event_parameter(&mut self, _target: &EventParameter) -> bool {
        true
    }
    fn leave_event_parameter(&mut self, _target: &EventParameter) {}

    fn enter_user_defined_value_type_definition(
        &mut self,
        _target: &UserDefinedValueTypeDefinition,
    ) -> bool {
        true
    }
    fn leave_user_defined_value_type_definition(
        &mut self,
        _target: &UserDefinedValueTypeDefinition,
    ) {
    }

    fn enter_error_definition(&mut self, _target: &ErrorDefinition) -> bool {
        true
    }
    fn leave_error_definition(&mut self, _target: &ErrorDefinition) {}

    fn enter_error_parameters_declaration(&mut self, _target: &ErrorParametersDeclaration) -> bool {
        true
    }
    fn leave_error_parameters_declaration(&mut self, _target: &ErrorParametersDeclaration) {}

    fn enter_error_parameter(&mut self, _target: &ErrorParameter) -> bool {
        true
    }
    fn leave_error_parameter(&mut self, _target: &ErrorParameter) {}

    fn enter_array_type_name(&mut self, _target: &ArrayTypeName) -> bool {
        true
    }
    fn leave_array_type_name(&mut self, _target: &ArrayTypeName) {}

    fn enter_function_type(&mut self, _target: &FunctionType) -> bool {
        true
    }
    fn leave_function_type(&mut self, _target: &FunctionType) {}

    fn enter_mapping_type(&mut self, _target: &MappingType) -> bool {
        true
    }
    fn leave_mapping_type(&mut self, _target: &MappingType) {}

    fn enter_mapping_key(&mut self, _target: &MappingKey) -> bool {
        true
    }
    fn leave_mapping_key(&mut self, _target: &MappingKey) {}

    fn enter_mapping_value(&mut self, _target: &MappingValue) -> bool {
        true
    }
    fn leave_mapping_value(&mut self, _target: &MappingValue) {}

    fn enter_address_type(&mut self, _target: &AddressType) -> bool {
        true
    }
    fn leave_address_type(&mut self, _target: &AddressType) {}

    fn enter_block(&mut self, _target: &Block) -> bool {
        true
    }
    fn leave_block(&mut self, _target: &Block) {}

    fn enter_unchecked_block(&mut self, _target: &UncheckedBlock) -> bool {
        true
    }
    fn leave_unchecked_block(&mut self, _target: &UncheckedBlock) {}

    fn enter_expression_statement(&mut self, _target: &ExpressionStatement) -> bool {
        true
    }
    fn leave_expression_statement(&mut self, _target: &ExpressionStatement) {}

    fn enter_assembly_statement(&mut self, _target: &AssemblyStatement) -> bool {
        true
    }
    fn leave_assembly_statement(&mut self, _target: &AssemblyStatement) {}

    fn enter_assembly_flags_declaration(&mut self, _target: &AssemblyFlagsDeclaration) -> bool {
        true
    }
    fn leave_assembly_flags_declaration(&mut self, _target: &AssemblyFlagsDeclaration) {}

    fn enter_tuple_deconstruction_statement(
        &mut self,
        _target: &TupleDeconstructionStatement,
    ) -> bool {
        true
    }
    fn leave_tuple_deconstruction_statement(&mut self, _target: &TupleDeconstructionStatement) {}

    fn enter_tuple_deconstruction_element(&mut self, _target: &TupleDeconstructionElement) -> bool {
        true
    }
    fn leave_tuple_deconstruction_element(&mut self, _target: &TupleDeconstructionElement) {}

    fn enter_typed_tuple_member(&mut self, _target: &TypedTupleMember) -> bool {
        true
    }
    fn leave_typed_tuple_member(&mut self, _target: &TypedTupleMember) {}

    fn enter_untyped_tuple_member(&mut self, _target: &UntypedTupleMember) -> bool {
        true
    }
    fn leave_untyped_tuple_member(&mut self, _target: &UntypedTupleMember) {}

    fn enter_variable_declaration_statement(
        &mut self,
        _target: &VariableDeclarationStatement,
    ) -> bool {
        true
    }
    fn leave_variable_declaration_statement(&mut self, _target: &VariableDeclarationStatement) {}

    fn enter_variable_declaration_value(&mut self, _target: &VariableDeclarationValue) -> bool {
        true
    }
    fn leave_variable_declaration_value(&mut self, _target: &VariableDeclarationValue) {}

    fn enter_if_statement(&mut self, _target: &IfStatement) -> bool {
        true
    }
    fn leave_if_statement(&mut self, _target: &IfStatement) {}

    fn enter_else_branch(&mut self, _target: &ElseBranch) -> bool {
        true
    }
    fn leave_else_branch(&mut self, _target: &ElseBranch) {}

    fn enter_for_statement(&mut self, _target: &ForStatement) -> bool {
        true
    }
    fn leave_for_statement(&mut self, _target: &ForStatement) {}

    fn enter_while_statement(&mut self, _target: &WhileStatement) -> bool {
        true
    }
    fn leave_while_statement(&mut self, _target: &WhileStatement) {}

    fn enter_do_while_statement(&mut self, _target: &DoWhileStatement) -> bool {
        true
    }
    fn leave_do_while_statement(&mut self, _target: &DoWhileStatement) {}

    fn enter_continue_statement(&mut self, _target: &ContinueStatement) -> bool {
        true
    }
    fn leave_continue_statement(&mut self, _target: &ContinueStatement) {}

    fn enter_break_statement(&mut self, _target: &BreakStatement) -> bool {
        true
    }
    fn leave_break_statement(&mut self, _target: &BreakStatement) {}

    fn enter_return_statement(&mut self, _target: &ReturnStatement) -> bool {
        true
    }
    fn leave_return_statement(&mut self, _target: &ReturnStatement) {}

    fn enter_emit_statement(&mut self, _target: &EmitStatement) -> bool {
        true
    }
    fn leave_emit_statement(&mut self, _target: &EmitStatement) {}

    fn enter_try_statement(&mut self, _target: &TryStatement) -> bool {
        true
    }
    fn leave_try_statement(&mut self, _target: &TryStatement) {}

    fn enter_catch_clause(&mut self, _target: &CatchClause) -> bool {
        true
    }
    fn leave_catch_clause(&mut self, _target: &CatchClause) {}

    fn enter_catch_clause_error(&mut self, _target: &CatchClauseError) -> bool {
        true
    }
    fn leave_catch_clause_error(&mut self, _target: &CatchClauseError) {}

    fn enter_revert_statement(&mut self, _target: &RevertStatement) -> bool {
        true
    }
    fn leave_revert_statement(&mut self, _target: &RevertStatement) {}

    fn enter_throw_statement(&mut self, _target: &ThrowStatement) -> bool {
        true
    }
    fn leave_throw_statement(&mut self, _target: &ThrowStatement) {}

    fn enter_assignment_expression(&mut self, _target: &AssignmentExpression) -> bool {
        true
    }
    fn leave_assignment_expression(&mut self, _target: &AssignmentExpression) {}

    fn enter_conditional_expression(&mut self, _target: &ConditionalExpression) -> bool {
        true
    }
    fn leave_conditional_expression(&mut self, _target: &ConditionalExpression) {}

    fn enter_or_expression(&mut self, _target: &OrExpression) -> bool {
        true
    }
    fn leave_or_expression(&mut self, _target: &OrExpression) {}

    fn enter_and_expression(&mut self, _target: &AndExpression) -> bool {
        true
    }
    fn leave_and_expression(&mut self, _target: &AndExpression) {}

    fn enter_equality_expression(&mut self, _target: &EqualityExpression) -> bool {
        true
    }
    fn leave_equality_expression(&mut self, _target: &EqualityExpression) {}

    fn enter_inequality_expression(&mut self, _target: &InequalityExpression) -> bool {
        true
    }
    fn leave_inequality_expression(&mut self, _target: &InequalityExpression) {}

    fn enter_bitwise_or_expression(&mut self, _target: &BitwiseOrExpression) -> bool {
        true
    }
    fn leave_bitwise_or_expression(&mut self, _target: &BitwiseOrExpression) {}

    fn enter_bitwise_xor_expression(&mut self, _target: &BitwiseXorExpression) -> bool {
        true
    }
    fn leave_bitwise_xor_expression(&mut self, _target: &BitwiseXorExpression) {}

    fn enter_bitwise_and_expression(&mut self, _target: &BitwiseAndExpression) -> bool {
        true
    }
    fn leave_bitwise_and_expression(&mut self, _target: &BitwiseAndExpression) {}

    fn enter_shift_expression(&mut self, _target: &ShiftExpression) -> bool {
        true
    }
    fn leave_shift_expression(&mut self, _target: &ShiftExpression) {}

    fn enter_additive_expression(&mut self, _target: &AdditiveExpression) -> bool {
        true
    }
    fn leave_additive_expression(&mut self, _target: &AdditiveExpression) {}

    fn enter_multiplicative_expression(&mut self, _target: &MultiplicativeExpression) -> bool {
        true
    }
    fn leave_multiplicative_expression(&mut self, _target: &MultiplicativeExpression) {}

    fn enter_exponentiation_expression(&mut self, _target: &ExponentiationExpression) -> bool {
        true
    }
    fn leave_exponentiation_expression(&mut self, _target: &ExponentiationExpression) {}

    fn enter_postfix_expression(&mut self, _target: &PostfixExpression) -> bool {
        true
    }
    fn leave_postfix_expression(&mut self, _target: &PostfixExpression) {}

    fn enter_prefix_expression(&mut self, _target: &PrefixExpression) -> bool {
        true
    }
    fn leave_prefix_expression(&mut self, _target: &PrefixExpression) {}

    fn enter_function_call_expression(&mut self, _target: &FunctionCallExpression) -> bool {
        true
    }
    fn leave_function_call_expression(&mut self, _target: &FunctionCallExpression) {}

    fn enter_call_options_expression(&mut self, _target: &CallOptionsExpression) -> bool {
        true
    }
    fn leave_call_options_expression(&mut self, _target: &CallOptionsExpression) {}

    fn enter_member_access_expression(&mut self, _target: &MemberAccessExpression) -> bool {
        true
    }
    fn leave_member_access_expression(&mut self, _target: &MemberAccessExpression) {}

    fn enter_index_access_expression(&mut self, _target: &IndexAccessExpression) -> bool {
        true
    }
    fn leave_index_access_expression(&mut self, _target: &IndexAccessExpression) {}

    fn enter_index_access_end(&mut self, _target: &IndexAccessEnd) -> bool {
        true
    }
    fn leave_index_access_end(&mut self, _target: &IndexAccessEnd) {}

    fn enter_positional_arguments_declaration(
        &mut self,
        _target: &PositionalArgumentsDeclaration,
    ) -> bool {
        true
    }
    fn leave_positional_arguments_declaration(&mut self, _target: &PositionalArgumentsDeclaration) {
    }

    fn enter_named_arguments_declaration(&mut self, _target: &NamedArgumentsDeclaration) -> bool {
        true
    }
    fn leave_named_arguments_declaration(&mut self, _target: &NamedArgumentsDeclaration) {}

    fn enter_named_argument_group(&mut self, _target: &NamedArgumentGroup) -> bool {
        true
    }
    fn leave_named_argument_group(&mut self, _target: &NamedArgumentGroup) {}

    fn enter_named_argument(&mut self, _target: &NamedArgument) -> bool {
        true
    }
    fn leave_named_argument(&mut self, _target: &NamedArgument) {}

    fn enter_type_expression(&mut self, _target: &TypeExpression) -> bool {
        true
    }
    fn leave_type_expression(&mut self, _target: &TypeExpression) {}

    fn enter_new_expression(&mut self, _target: &NewExpression) -> bool {
        true
    }
    fn leave_new_expression(&mut self, _target: &NewExpression) {}

    fn enter_tuple_expression(&mut self, _target: &TupleExpression) -> bool {
        true
    }
    fn leave_tuple_expression(&mut self, _target: &TupleExpression) {}

    fn enter_tuple_value(&mut self, _target: &TupleValue) -> bool {
        true
    }
    fn leave_tuple_value(&mut self, _target: &TupleValue) {}

    fn enter_array_expression(&mut self, _target: &ArrayExpression) -> bool {
        true
    }
    fn leave_array_expression(&mut self, _target: &ArrayExpression) {}

    fn enter_hex_number_expression(&mut self, _target: &HexNumberExpression) -> bool {
        true
    }
    fn leave_hex_number_expression(&mut self, _target: &HexNumberExpression) {}

    fn enter_decimal_number_expression(&mut self, _target: &DecimalNumberExpression) -> bool {
        true
    }
    fn leave_decimal_number_expression(&mut self, _target: &DecimalNumberExpression) {}

    fn enter_yul_block(&mut self, _target: &YulBlock) -> bool {
        true
    }
    fn leave_yul_block(&mut self, _target: &YulBlock) {}

    fn enter_yul_function_definition(&mut self, _target: &YulFunctionDefinition) -> bool {
        true
    }
    fn leave_yul_function_definition(&mut self, _target: &YulFunctionDefinition) {}

    fn enter_yul_parameters_declaration(&mut self, _target: &YulParametersDeclaration) -> bool {
        true
    }
    fn leave_yul_parameters_declaration(&mut self, _target: &YulParametersDeclaration) {}

    fn enter_yul_returns_declaration(&mut self, _target: &YulReturnsDeclaration) -> bool {
        true
    }
    fn leave_yul_returns_declaration(&mut self, _target: &YulReturnsDeclaration) {}

    fn enter_yul_variable_declaration_statement(
        &mut self,
        _target: &YulVariableDeclarationStatement,
    ) -> bool {
        true
    }
    fn leave_yul_variable_declaration_statement(
        &mut self,
        _target: &YulVariableDeclarationStatement,
    ) {
    }

    fn enter_yul_variable_declaration_value(
        &mut self,
        _target: &YulVariableDeclarationValue,
    ) -> bool {
        true
    }
    fn leave_yul_variable_declaration_value(&mut self, _target: &YulVariableDeclarationValue) {}

    fn enter_yul_variable_assignment_statement(
        &mut self,
        _target: &YulVariableAssignmentStatement,
    ) -> bool {
        true
    }
    fn leave_yul_variable_assignment_statement(
        &mut self,
        _target: &YulVariableAssignmentStatement,
    ) {
    }

    fn enter_yul_colon_and_equal(&mut self, _target: &YulColonAndEqual) -> bool {
        true
    }
    fn leave_yul_colon_and_equal(&mut self, _target: &YulColonAndEqual) {}

    fn enter_yul_stack_assignment_statement(
        &mut self,
        _target: &YulStackAssignmentStatement,
    ) -> bool {
        true
    }
    fn leave_yul_stack_assignment_statement(&mut self, _target: &YulStackAssignmentStatement) {}

    fn enter_yul_equal_and_colon(&mut self, _target: &YulEqualAndColon) -> bool {
        true
    }
    fn leave_yul_equal_and_colon(&mut self, _target: &YulEqualAndColon) {}

    fn enter_yul_if_statement(&mut self, _target: &YulIfStatement) -> bool {
        true
    }
    fn leave_yul_if_statement(&mut self, _target: &YulIfStatement) {}

    fn enter_yul_for_statement(&mut self, _target: &YulForStatement) -> bool {
        true
    }
    fn leave_yul_for_statement(&mut self, _target: &YulForStatement) {}

    fn enter_yul_switch_statement(&mut self, _target: &YulSwitchStatement) -> bool {
        true
    }
    fn leave_yul_switch_statement(&mut self, _target: &YulSwitchStatement) {}

    fn enter_yul_default_case(&mut self, _target: &YulDefaultCase) -> bool {
        true
    }
    fn leave_yul_default_case(&mut self, _target: &YulDefaultCase) {}

    fn enter_yul_value_case(&mut self, _target: &YulValueCase) -> bool {
        true
    }
    fn leave_yul_value_case(&mut self, _target: &YulValueCase) {}

    fn enter_yul_leave_statement(&mut self, _target: &YulLeaveStatement) -> bool {
        true
    }
    fn leave_yul_leave_statement(&mut self, _target: &YulLeaveStatement) {}

    fn enter_yul_break_statement(&mut self, _target: &YulBreakStatement) -> bool {
        true
    }
    fn leave_yul_break_statement(&mut self, _target: &YulBreakStatement) {}

    fn enter_yul_continue_statement(&mut self, _target: &YulContinueStatement) -> bool {
        true
    }
    fn leave_yul_continue_statement(&mut self, _target: &YulContinueStatement) {}

    fn enter_yul_label(&mut self, _target: &YulLabel) -> bool {
        true
    }
    fn leave_yul_label(&mut self, _target: &YulLabel) {}

    fn enter_yul_function_call_expression(&mut self, _target: &YulFunctionCallExpression) -> bool {
        true
    }
    fn leave_yul_function_call_expression(&mut self, _target: &YulFunctionCallExpression) {}
}

//
// Sequences:
//

pub fn accept_source_unit(target: &SourceUnit, visitor: &mut dyn Visitor) {
    if visitor.enter_source_unit(target) {
        accept_source_unit_members(&target.members, visitor);
    }
    visitor.leave_source_unit(target);
}

pub fn accept_pragma_directive(target: &PragmaDirective, visitor: &mut dyn Visitor) {
    if visitor.enter_pragma_directive(target) {
        accept_pragma(&target.pragma, visitor);
    }
    visitor.leave_pragma_directive(target);
}

pub fn accept_abicoder_pragma(target: &AbicoderPragma, visitor: &mut dyn Visitor) {
    visitor.enter_abicoder_pragma(target);
    visitor.leave_abicoder_pragma(target);
}

pub fn accept_experimental_pragma(target: &ExperimentalPragma, visitor: &mut dyn Visitor) {
    if visitor.enter_experimental_pragma(target) {
        accept_experimental_feature(&target.feature, visitor);
    }
    visitor.leave_experimental_pragma(target);
}

pub fn accept_version_pragma(target: &VersionPragma, visitor: &mut dyn Visitor) {
    if visitor.enter_version_pragma(target) {
        accept_version_expression_sets(&target.sets, visitor);
    }
    visitor.leave_version_pragma(target);
}

pub fn accept_version_range(target: &VersionRange, visitor: &mut dyn Visitor) {
    if visitor.enter_version_range(target) {
        accept_version_literal(&target.start, visitor);
        accept_version_literal(&target.end, visitor);
    }
    visitor.leave_version_range(target);
}

pub fn accept_version_term(target: &VersionTerm, visitor: &mut dyn Visitor) {
    if visitor.enter_version_term(target) {
        if let Some(ref operator) = target.operator {
            accept_version_operator(operator, visitor);
        }
        accept_version_literal(&target.literal, visitor);
    }
    visitor.leave_version_term(target);
}

pub fn accept_import_directive(target: &ImportDirective, visitor: &mut dyn Visitor) {
    if visitor.enter_import_directive(target) {
        accept_import_clause(&target.clause, visitor);
    }
    visitor.leave_import_directive(target);
}

pub fn accept_path_import(target: &PathImport, visitor: &mut dyn Visitor) {
    if visitor.enter_path_import(target) {
        accept_string_literal(&target.path, visitor);
        if let Some(ref alias) = target.alias {
            accept_import_alias(alias, visitor);
        }
    }
    visitor.leave_path_import(target);
}

pub fn accept_named_import(target: &NamedImport, visitor: &mut dyn Visitor) {
    if visitor.enter_named_import(target) {
        accept_import_alias(&target.alias, visitor);
        accept_string_literal(&target.path, visitor);
    }
    visitor.leave_named_import(target);
}

pub fn accept_import_deconstruction(target: &ImportDeconstruction, visitor: &mut dyn Visitor) {
    if visitor.enter_import_deconstruction(target) {
        accept_import_deconstruction_symbols(&target.symbols, visitor);
        accept_string_literal(&target.path, visitor);
    }
    visitor.leave_import_deconstruction(target);
}

pub fn accept_import_deconstruction_symbol(
    target: &ImportDeconstructionSymbol,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_import_deconstruction_symbol(target) {
        if let Some(ref alias) = target.alias {
            accept_import_alias(alias, visitor);
        }
    }
    visitor.leave_import_deconstruction_symbol(target);
}

pub fn accept_import_alias(target: &ImportAlias, visitor: &mut dyn Visitor) {
    visitor.enter_import_alias(target);
    visitor.leave_import_alias(target);
}

pub fn accept_using_directive(target: &UsingDirective, visitor: &mut dyn Visitor) {
    if visitor.enter_using_directive(target) {
        accept_using_clause(&target.clause, visitor);
        accept_using_target(&target.target, visitor);
    }
    visitor.leave_using_directive(target);
}

pub fn accept_using_deconstruction(target: &UsingDeconstruction, visitor: &mut dyn Visitor) {
    if visitor.enter_using_deconstruction(target) {
        accept_using_deconstruction_symbols(&target.symbols, visitor);
    }
    visitor.leave_using_deconstruction(target);
}

pub fn accept_using_deconstruction_symbol(
    target: &UsingDeconstructionSymbol,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_using_deconstruction_symbol(target) {
        accept_identifier_path(&target.name, visitor);
        if let Some(ref alias) = target.alias {
            accept_using_alias(alias, visitor);
        }
    }
    visitor.leave_using_deconstruction_symbol(target);
}

pub fn accept_using_alias(target: &UsingAlias, visitor: &mut dyn Visitor) {
    if visitor.enter_using_alias(target) {
        accept_using_operator(&target.operator, visitor);
    }
    visitor.leave_using_alias(target);
}

pub fn accept_contract_definition(target: &ContractDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_contract_definition(target) {
        accept_contract_members(&target.members, visitor);
        accept_inheritance_types(&target.inheritance_types, visitor);
        if let Some(ref storage_layout) = target.storage_layout {
            accept_storage_layout_specifier(storage_layout, visitor);
        }
    }
    visitor.leave_contract_definition(target);
}

pub fn accept_inheritance_specifier(target: &InheritanceSpecifier, visitor: &mut dyn Visitor) {
    if visitor.enter_inheritance_specifier(target) {
        accept_inheritance_types(&target.types, visitor);
    }
    visitor.leave_inheritance_specifier(target);
}

pub fn accept_inheritance_type(target: &InheritanceType, visitor: &mut dyn Visitor) {
    if visitor.enter_inheritance_type(target) {
        accept_identifier_path(&target.type_name, visitor);
        if let Some(ref arguments) = target.arguments {
            accept_arguments_declaration(arguments, visitor);
        }
    }
    visitor.leave_inheritance_type(target);
}

pub fn accept_storage_layout_specifier(target: &StorageLayoutSpecifier, visitor: &mut dyn Visitor) {
    if visitor.enter_storage_layout_specifier(target) {
        accept_expression(&target.expression, visitor);
    }
    visitor.leave_storage_layout_specifier(target);
}

pub fn accept_interface_definition(target: &InterfaceDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_interface_definition(target) {
        if let Some(ref inheritance) = target.inheritance {
            accept_inheritance_specifier(inheritance, visitor);
        }
        accept_interface_members(&target.members, visitor);
    }
    visitor.leave_interface_definition(target);
}

pub fn accept_library_definition(target: &LibraryDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_library_definition(target) {
        accept_library_members(&target.members, visitor);
    }
    visitor.leave_library_definition(target);
}

pub fn accept_struct_definition(target: &StructDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_struct_definition(target) {
        accept_struct_members(&target.members, visitor);
    }
    visitor.leave_struct_definition(target);
}

pub fn accept_struct_member(target: &StructMember, visitor: &mut dyn Visitor) {
    if visitor.enter_struct_member(target) {
        accept_type_name(&target.type_name, visitor);
    }
    visitor.leave_struct_member(target);
}

pub fn accept_enum_definition(target: &EnumDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_enum_definition(target) {
        accept_enum_members(&target.members, visitor);
    }
    visitor.leave_enum_definition(target);
}

pub fn accept_constant_definition(target: &ConstantDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_constant_definition(target) {
        accept_type_name(&target.type_name, visitor);
        accept_expression(&target.value, visitor);
    }
    visitor.leave_constant_definition(target);
}

pub fn accept_state_variable_definition(
    target: &StateVariableDefinition,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_state_variable_definition(target) {
        accept_type_name(&target.type_name, visitor);
        accept_state_variable_attributes(&target.attributes, visitor);
        if let Some(ref value) = target.value {
            accept_state_variable_definition_value(value, visitor);
        }
    }
    visitor.leave_state_variable_definition(target);
}

pub fn accept_state_variable_definition_value(
    target: &StateVariableDefinitionValue,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_state_variable_definition_value(target) {
        accept_expression(&target.value, visitor);
    }
    visitor.leave_state_variable_definition_value(target);
}

pub fn accept_function_definition(target: &FunctionDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_function_definition(target) {
        accept_function_name(&target.name, visitor);
        accept_parameters_declaration(&target.parameters, visitor);
        accept_function_attributes(&target.attributes, visitor);
        if let Some(ref returns) = target.returns {
            accept_returns_declaration(returns, visitor);
        }
        accept_function_body(&target.body, visitor);
    }
    visitor.leave_function_definition(target);
}

pub fn accept_parameters_declaration(target: &ParametersDeclaration, visitor: &mut dyn Visitor) {
    if visitor.enter_parameters_declaration(target) {
        accept_parameters(&target.parameters, visitor);
    }
    visitor.leave_parameters_declaration(target);
}

pub fn accept_parameter(target: &Parameter, visitor: &mut dyn Visitor) {
    if visitor.enter_parameter(target) {
        accept_type_name(&target.type_name, visitor);
        if let Some(ref storage_location) = target.storage_location {
            accept_storage_location(storage_location, visitor);
        }
    }
    visitor.leave_parameter(target);
}

pub fn accept_override_specifier(target: &OverrideSpecifier, visitor: &mut dyn Visitor) {
    if visitor.enter_override_specifier(target) {
        if let Some(ref overridden) = target.overridden {
            accept_override_paths_declaration(overridden, visitor);
        }
    }
    visitor.leave_override_specifier(target);
}

pub fn accept_override_paths_declaration(
    target: &OverridePathsDeclaration,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_override_paths_declaration(target) {
        accept_override_paths(&target.paths, visitor);
    }
    visitor.leave_override_paths_declaration(target);
}

pub fn accept_returns_declaration(target: &ReturnsDeclaration, visitor: &mut dyn Visitor) {
    if visitor.enter_returns_declaration(target) {
        accept_parameters_declaration(&target.variables, visitor);
    }
    visitor.leave_returns_declaration(target);
}

pub fn accept_constructor_definition(target: &ConstructorDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_constructor_definition(target) {
        accept_parameters_declaration(&target.parameters, visitor);
        accept_constructor_attributes(&target.attributes, visitor);
        accept_block(&target.body, visitor);
    }
    visitor.leave_constructor_definition(target);
}

pub fn accept_fallback_function_definition(
    target: &FallbackFunctionDefinition,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_fallback_function_definition(target) {
        accept_parameters_declaration(&target.parameters, visitor);
        accept_fallback_function_attributes(&target.attributes, visitor);
        if let Some(ref returns) = target.returns {
            accept_returns_declaration(returns, visitor);
        }
        accept_function_body(&target.body, visitor);
    }
    visitor.leave_fallback_function_definition(target);
}

pub fn accept_receive_function_definition(
    target: &ReceiveFunctionDefinition,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_receive_function_definition(target) {
        accept_parameters_declaration(&target.parameters, visitor);
        accept_receive_function_attributes(&target.attributes, visitor);
        accept_function_body(&target.body, visitor);
    }
    visitor.leave_receive_function_definition(target);
}

pub fn accept_modifier_definition(target: &ModifierDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_modifier_definition(target) {
        if let Some(ref parameters) = target.parameters {
            accept_parameters_declaration(parameters, visitor);
        }
        accept_modifier_attributes(&target.attributes, visitor);
        accept_function_body(&target.body, visitor);
    }
    visitor.leave_modifier_definition(target);
}

pub fn accept_modifier_invocation(target: &ModifierInvocation, visitor: &mut dyn Visitor) {
    if visitor.enter_modifier_invocation(target) {
        accept_identifier_path(&target.name, visitor);
        if let Some(ref arguments) = target.arguments {
            accept_arguments_declaration(arguments, visitor);
        }
    }
    visitor.leave_modifier_invocation(target);
}

pub fn accept_event_definition(target: &EventDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_event_definition(target) {
        accept_event_parameters_declaration(&target.parameters, visitor);
    }
    visitor.leave_event_definition(target);
}

pub fn accept_event_parameters_declaration(
    target: &EventParametersDeclaration,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_event_parameters_declaration(target) {
        accept_event_parameters(&target.parameters, visitor);
    }
    visitor.leave_event_parameters_declaration(target);
}

pub fn accept_event_parameter(target: &EventParameter, visitor: &mut dyn Visitor) {
    if visitor.enter_event_parameter(target) {
        accept_type_name(&target.type_name, visitor);
    }
    visitor.leave_event_parameter(target);
}

pub fn accept_user_defined_value_type_definition(
    target: &UserDefinedValueTypeDefinition,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_user_defined_value_type_definition(target) {
        accept_elementary_type(&target.value_type, visitor);
    }
    visitor.leave_user_defined_value_type_definition(target);
}

pub fn accept_error_definition(target: &ErrorDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_error_definition(target) {
        accept_error_parameters_declaration(&target.members, visitor);
    }
    visitor.leave_error_definition(target);
}

pub fn accept_error_parameters_declaration(
    target: &ErrorParametersDeclaration,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_error_parameters_declaration(target) {
        accept_error_parameters(&target.parameters, visitor);
    }
    visitor.leave_error_parameters_declaration(target);
}

pub fn accept_error_parameter(target: &ErrorParameter, visitor: &mut dyn Visitor) {
    if visitor.enter_error_parameter(target) {
        accept_type_name(&target.type_name, visitor);
    }
    visitor.leave_error_parameter(target);
}

pub fn accept_array_type_name(target: &ArrayTypeName, visitor: &mut dyn Visitor) {
    if visitor.enter_array_type_name(target) {
        accept_type_name(&target.operand, visitor);
        if let Some(ref index) = target.index {
            accept_expression(index, visitor);
        }
    }
    visitor.leave_array_type_name(target);
}

pub fn accept_function_type(target: &FunctionType, visitor: &mut dyn Visitor) {
    if visitor.enter_function_type(target) {
        accept_parameters_declaration(&target.parameters, visitor);
        accept_function_type_attributes(&target.attributes, visitor);
        if let Some(ref returns) = target.returns {
            accept_returns_declaration(returns, visitor);
        }
    }
    visitor.leave_function_type(target);
}

pub fn accept_mapping_type(target: &MappingType, visitor: &mut dyn Visitor) {
    if visitor.enter_mapping_type(target) {
        accept_mapping_key(&target.key_type, visitor);
        accept_mapping_value(&target.value_type, visitor);
    }
    visitor.leave_mapping_type(target);
}

pub fn accept_mapping_key(target: &MappingKey, visitor: &mut dyn Visitor) {
    if visitor.enter_mapping_key(target) {
        accept_mapping_key_type(&target.key_type, visitor);
    }
    visitor.leave_mapping_key(target);
}

pub fn accept_mapping_value(target: &MappingValue, visitor: &mut dyn Visitor) {
    if visitor.enter_mapping_value(target) {
        accept_type_name(&target.type_name, visitor);
    }
    visitor.leave_mapping_value(target);
}

pub fn accept_address_type(target: &AddressType, visitor: &mut dyn Visitor) {
    visitor.enter_address_type(target);
    visitor.leave_address_type(target);
}

pub fn accept_block(target: &Block, visitor: &mut dyn Visitor) {
    if visitor.enter_block(target) {
        accept_statements(&target.statements, visitor);
    }
    visitor.leave_block(target);
}

pub fn accept_unchecked_block(target: &UncheckedBlock, visitor: &mut dyn Visitor) {
    if visitor.enter_unchecked_block(target) {
        accept_block(&target.block, visitor);
    }
    visitor.leave_unchecked_block(target);
}

pub fn accept_expression_statement(target: &ExpressionStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_expression_statement(target) {
        accept_expression(&target.expression, visitor);
    }
    visitor.leave_expression_statement(target);
}

pub fn accept_assembly_statement(target: &AssemblyStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_assembly_statement(target) {
        if let Some(ref label) = target.label {
            accept_string_literal(label, visitor);
        }

        if let Some(ref flags) = target.flags {
            accept_assembly_flags_declaration(flags, visitor);
        }
        accept_yul_block(&target.body, visitor);
    }
    visitor.leave_assembly_statement(target);
}

pub fn accept_assembly_flags_declaration(
    target: &AssemblyFlagsDeclaration,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_assembly_flags_declaration(target) {
        accept_assembly_flags(&target.flags, visitor);
    }
    visitor.leave_assembly_flags_declaration(target);
}

pub fn accept_tuple_deconstruction_statement(
    target: &TupleDeconstructionStatement,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_tuple_deconstruction_statement(target) {
        accept_tuple_deconstruction_elements(&target.elements, visitor);
        accept_expression(&target.expression, visitor);
    }
    visitor.leave_tuple_deconstruction_statement(target);
}

pub fn accept_tuple_deconstruction_element(
    target: &TupleDeconstructionElement,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_tuple_deconstruction_element(target) {
        if let Some(ref member) = target.member {
            accept_tuple_member(member, visitor);
        }
    }
    visitor.leave_tuple_deconstruction_element(target);
}

pub fn accept_typed_tuple_member(target: &TypedTupleMember, visitor: &mut dyn Visitor) {
    if visitor.enter_typed_tuple_member(target) {
        accept_type_name(&target.type_name, visitor);
        if let Some(ref storage_location) = target.storage_location {
            accept_storage_location(storage_location, visitor);
        }
    }
    visitor.leave_typed_tuple_member(target);
}

pub fn accept_untyped_tuple_member(target: &UntypedTupleMember, visitor: &mut dyn Visitor) {
    if visitor.enter_untyped_tuple_member(target) {
        if let Some(ref storage_location) = target.storage_location {
            accept_storage_location(storage_location, visitor);
        }
    }
    visitor.leave_untyped_tuple_member(target);
}

pub fn accept_variable_declaration_statement(
    target: &VariableDeclarationStatement,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_variable_declaration_statement(target) {
        accept_variable_declaration_type(&target.variable_type, visitor);
        if let Some(ref storage_location) = target.storage_location {
            accept_storage_location(storage_location, visitor);
        }

        if let Some(ref value) = target.value {
            accept_variable_declaration_value(value, visitor);
        }
    }
    visitor.leave_variable_declaration_statement(target);
}

pub fn accept_variable_declaration_value(
    target: &VariableDeclarationValue,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_variable_declaration_value(target) {
        accept_expression(&target.expression, visitor);
    }
    visitor.leave_variable_declaration_value(target);
}

pub fn accept_if_statement(target: &IfStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_if_statement(target) {
        accept_expression(&target.condition, visitor);
        accept_statement(&target.body, visitor);
        if let Some(ref else_branch) = target.else_branch {
            accept_else_branch(else_branch, visitor);
        }
    }
    visitor.leave_if_statement(target);
}

pub fn accept_else_branch(target: &ElseBranch, visitor: &mut dyn Visitor) {
    if visitor.enter_else_branch(target) {
        accept_statement(&target.body, visitor);
    }
    visitor.leave_else_branch(target);
}

pub fn accept_for_statement(target: &ForStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_for_statement(target) {
        accept_for_statement_initialization(&target.initialization, visitor);
        accept_for_statement_condition(&target.condition, visitor);
        if let Some(ref iterator) = target.iterator {
            accept_expression(iterator, visitor);
        }
        accept_statement(&target.body, visitor);
    }
    visitor.leave_for_statement(target);
}

pub fn accept_while_statement(target: &WhileStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_while_statement(target) {
        accept_expression(&target.condition, visitor);
        accept_statement(&target.body, visitor);
    }
    visitor.leave_while_statement(target);
}

pub fn accept_do_while_statement(target: &DoWhileStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_do_while_statement(target) {
        accept_statement(&target.body, visitor);
        accept_expression(&target.condition, visitor);
    }
    visitor.leave_do_while_statement(target);
}

pub fn accept_continue_statement(target: &ContinueStatement, visitor: &mut dyn Visitor) {
    visitor.enter_continue_statement(target);
    visitor.leave_continue_statement(target);
}

pub fn accept_break_statement(target: &BreakStatement, visitor: &mut dyn Visitor) {
    visitor.enter_break_statement(target);
    visitor.leave_break_statement(target);
}

pub fn accept_return_statement(target: &ReturnStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_return_statement(target) {
        if let Some(ref expression) = target.expression {
            accept_expression(expression, visitor);
        }
    }
    visitor.leave_return_statement(target);
}

pub fn accept_emit_statement(target: &EmitStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_emit_statement(target) {
        accept_identifier_path(&target.event, visitor);
        accept_arguments_declaration(&target.arguments, visitor);
    }
    visitor.leave_emit_statement(target);
}

pub fn accept_try_statement(target: &TryStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_try_statement(target) {
        accept_expression(&target.expression, visitor);
        if let Some(ref returns) = target.returns {
            accept_returns_declaration(returns, visitor);
        }
        accept_block(&target.body, visitor);
        accept_catch_clauses(&target.catch_clauses, visitor);
    }
    visitor.leave_try_statement(target);
}

pub fn accept_catch_clause(target: &CatchClause, visitor: &mut dyn Visitor) {
    if visitor.enter_catch_clause(target) {
        if let Some(ref error) = target.error {
            accept_catch_clause_error(error, visitor);
        }
        accept_block(&target.body, visitor);
    }
    visitor.leave_catch_clause(target);
}

pub fn accept_catch_clause_error(target: &CatchClauseError, visitor: &mut dyn Visitor) {
    if visitor.enter_catch_clause_error(target) {
        accept_parameters_declaration(&target.parameters, visitor);
    }
    visitor.leave_catch_clause_error(target);
}

pub fn accept_revert_statement(target: &RevertStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_revert_statement(target) {
        if let Some(ref error) = target.error {
            accept_identifier_path(error, visitor);
        }
        accept_arguments_declaration(&target.arguments, visitor);
    }
    visitor.leave_revert_statement(target);
}

pub fn accept_throw_statement(target: &ThrowStatement, visitor: &mut dyn Visitor) {
    visitor.enter_throw_statement(target);
    visitor.leave_throw_statement(target);
}

pub fn accept_assignment_expression(target: &AssignmentExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_assignment_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_assignment_expression(target);
}

pub fn accept_conditional_expression(target: &ConditionalExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_conditional_expression(target) {
        accept_expression(&target.operand, visitor);
        accept_expression(&target.true_expression, visitor);
        accept_expression(&target.false_expression, visitor);
    }
    visitor.leave_conditional_expression(target);
}

pub fn accept_or_expression(target: &OrExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_or_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_or_expression(target);
}

pub fn accept_and_expression(target: &AndExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_and_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_and_expression(target);
}

pub fn accept_equality_expression(target: &EqualityExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_equality_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_equality_expression(target);
}

pub fn accept_inequality_expression(target: &InequalityExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_inequality_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_inequality_expression(target);
}

pub fn accept_bitwise_or_expression(target: &BitwiseOrExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_bitwise_or_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_bitwise_or_expression(target);
}

pub fn accept_bitwise_xor_expression(target: &BitwiseXorExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_bitwise_xor_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_bitwise_xor_expression(target);
}

pub fn accept_bitwise_and_expression(target: &BitwiseAndExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_bitwise_and_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_bitwise_and_expression(target);
}

pub fn accept_shift_expression(target: &ShiftExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_shift_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_shift_expression(target);
}

pub fn accept_additive_expression(target: &AdditiveExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_additive_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_additive_expression(target);
}

pub fn accept_multiplicative_expression(
    target: &MultiplicativeExpression,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_multiplicative_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_multiplicative_expression(target);
}

pub fn accept_exponentiation_expression(
    target: &ExponentiationExpression,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_exponentiation_expression(target) {
        accept_expression(&target.left_operand, visitor);
        accept_expression(&target.right_operand, visitor);
    }
    visitor.leave_exponentiation_expression(target);
}

pub fn accept_postfix_expression(target: &PostfixExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_postfix_expression(target) {
        accept_expression(&target.operand, visitor);
    }
    visitor.leave_postfix_expression(target);
}

pub fn accept_prefix_expression(target: &PrefixExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_prefix_expression(target) {
        accept_expression(&target.operand, visitor);
    }
    visitor.leave_prefix_expression(target);
}

pub fn accept_function_call_expression(target: &FunctionCallExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_function_call_expression(target) {
        accept_expression(&target.operand, visitor);
        accept_arguments_declaration(&target.arguments, visitor);
    }
    visitor.leave_function_call_expression(target);
}

pub fn accept_call_options_expression(target: &CallOptionsExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_call_options_expression(target) {
        accept_expression(&target.operand, visitor);
        accept_call_options(&target.options, visitor);
    }
    visitor.leave_call_options_expression(target);
}

pub fn accept_member_access_expression(target: &MemberAccessExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_member_access_expression(target) {
        accept_expression(&target.operand, visitor);
    }
    visitor.leave_member_access_expression(target);
}

pub fn accept_index_access_expression(target: &IndexAccessExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_index_access_expression(target) {
        accept_expression(&target.operand, visitor);
        if let Some(ref start) = target.start {
            accept_expression(start, visitor);
        }

        if let Some(ref end) = target.end {
            accept_index_access_end(end, visitor);
        }
    }
    visitor.leave_index_access_expression(target);
}

pub fn accept_index_access_end(target: &IndexAccessEnd, visitor: &mut dyn Visitor) {
    if visitor.enter_index_access_end(target) {
        if let Some(ref end) = target.end {
            accept_expression(end, visitor);
        }
    }
    visitor.leave_index_access_end(target);
}

pub fn accept_positional_arguments_declaration(
    target: &PositionalArgumentsDeclaration,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_positional_arguments_declaration(target) {
        accept_positional_arguments(&target.arguments, visitor);
    }
    visitor.leave_positional_arguments_declaration(target);
}

pub fn accept_named_arguments_declaration(
    target: &NamedArgumentsDeclaration,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_named_arguments_declaration(target) {
        if let Some(ref arguments) = target.arguments {
            accept_named_argument_group(arguments, visitor);
        }
    }
    visitor.leave_named_arguments_declaration(target);
}

pub fn accept_named_argument_group(target: &NamedArgumentGroup, visitor: &mut dyn Visitor) {
    if visitor.enter_named_argument_group(target) {
        accept_named_arguments(&target.arguments, visitor);
    }
    visitor.leave_named_argument_group(target);
}

pub fn accept_named_argument(target: &NamedArgument, visitor: &mut dyn Visitor) {
    if visitor.enter_named_argument(target) {
        accept_expression(&target.value, visitor);
    }
    visitor.leave_named_argument(target);
}

pub fn accept_type_expression(target: &TypeExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_type_expression(target) {
        accept_type_name(&target.type_name, visitor);
    }
    visitor.leave_type_expression(target);
}

pub fn accept_new_expression(target: &NewExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_new_expression(target) {
        accept_type_name(&target.type_name, visitor);
    }
    visitor.leave_new_expression(target);
}

pub fn accept_tuple_expression(target: &TupleExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_tuple_expression(target) {
        accept_tuple_values(&target.items, visitor);
    }
    visitor.leave_tuple_expression(target);
}

pub fn accept_tuple_value(target: &TupleValue, visitor: &mut dyn Visitor) {
    if visitor.enter_tuple_value(target) {
        if let Some(ref expression) = target.expression {
            accept_expression(expression, visitor);
        }
    }
    visitor.leave_tuple_value(target);
}

pub fn accept_array_expression(target: &ArrayExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_array_expression(target) {
        accept_array_values(&target.items, visitor);
    }
    visitor.leave_array_expression(target);
}

pub fn accept_hex_number_expression(target: &HexNumberExpression, visitor: &mut dyn Visitor) {
    if visitor.enter_hex_number_expression(target) {
        if let Some(ref unit) = target.unit {
            accept_number_unit(unit, visitor);
        }
    }
    visitor.leave_hex_number_expression(target);
}

pub fn accept_decimal_number_expression(
    target: &DecimalNumberExpression,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_decimal_number_expression(target) {
        if let Some(ref unit) = target.unit {
            accept_number_unit(unit, visitor);
        }
    }
    visitor.leave_decimal_number_expression(target);
}

pub fn accept_yul_block(target: &YulBlock, visitor: &mut dyn Visitor) {
    if visitor.enter_yul_block(target) {
        accept_yul_statements(&target.statements, visitor);
    }
    visitor.leave_yul_block(target);
}

pub fn accept_yul_function_definition(target: &YulFunctionDefinition, visitor: &mut dyn Visitor) {
    if visitor.enter_yul_function_definition(target) {
        accept_yul_parameters_declaration(&target.parameters, visitor);
        if let Some(ref returns) = target.returns {
            accept_yul_returns_declaration(returns, visitor);
        }
        accept_yul_block(&target.body, visitor);
    }
    visitor.leave_yul_function_definition(target);
}

pub fn accept_yul_parameters_declaration(
    target: &YulParametersDeclaration,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_yul_parameters_declaration(target) {
        accept_yul_parameters(&target.parameters, visitor);
    }
    visitor.leave_yul_parameters_declaration(target);
}

pub fn accept_yul_returns_declaration(target: &YulReturnsDeclaration, visitor: &mut dyn Visitor) {
    if visitor.enter_yul_returns_declaration(target) {
        accept_yul_variable_names(&target.variables, visitor);
    }
    visitor.leave_yul_returns_declaration(target);
}

pub fn accept_yul_variable_declaration_statement(
    target: &YulVariableDeclarationStatement,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_yul_variable_declaration_statement(target) {
        accept_yul_variable_names(&target.variables, visitor);
        if let Some(ref value) = target.value {
            accept_yul_variable_declaration_value(value, visitor);
        }
    }
    visitor.leave_yul_variable_declaration_statement(target);
}

pub fn accept_yul_variable_declaration_value(
    target: &YulVariableDeclarationValue,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_yul_variable_declaration_value(target) {
        accept_yul_assignment_operator(&target.assignment, visitor);
        accept_yul_expression(&target.expression, visitor);
    }
    visitor.leave_yul_variable_declaration_value(target);
}

pub fn accept_yul_variable_assignment_statement(
    target: &YulVariableAssignmentStatement,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_yul_variable_assignment_statement(target) {
        accept_yul_paths(&target.variables, visitor);
        accept_yul_assignment_operator(&target.assignment, visitor);
        accept_yul_expression(&target.expression, visitor);
    }
    visitor.leave_yul_variable_assignment_statement(target);
}

pub fn accept_yul_colon_and_equal(target: &YulColonAndEqual, visitor: &mut dyn Visitor) {
    visitor.enter_yul_colon_and_equal(target);
    visitor.leave_yul_colon_and_equal(target);
}

pub fn accept_yul_stack_assignment_statement(
    target: &YulStackAssignmentStatement,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_yul_stack_assignment_statement(target) {
        accept_yul_stack_assignment_operator(&target.assignment, visitor);
    }
    visitor.leave_yul_stack_assignment_statement(target);
}

pub fn accept_yul_equal_and_colon(target: &YulEqualAndColon, visitor: &mut dyn Visitor) {
    visitor.enter_yul_equal_and_colon(target);
    visitor.leave_yul_equal_and_colon(target);
}

pub fn accept_yul_if_statement(target: &YulIfStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_yul_if_statement(target) {
        accept_yul_expression(&target.condition, visitor);
        accept_yul_block(&target.body, visitor);
    }
    visitor.leave_yul_if_statement(target);
}

pub fn accept_yul_for_statement(target: &YulForStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_yul_for_statement(target) {
        accept_yul_block(&target.initialization, visitor);
        accept_yul_expression(&target.condition, visitor);
        accept_yul_block(&target.iterator, visitor);
        accept_yul_block(&target.body, visitor);
    }
    visitor.leave_yul_for_statement(target);
}

pub fn accept_yul_switch_statement(target: &YulSwitchStatement, visitor: &mut dyn Visitor) {
    if visitor.enter_yul_switch_statement(target) {
        accept_yul_expression(&target.expression, visitor);
        accept_yul_switch_cases(&target.cases, visitor);
    }
    visitor.leave_yul_switch_statement(target);
}

pub fn accept_yul_default_case(target: &YulDefaultCase, visitor: &mut dyn Visitor) {
    if visitor.enter_yul_default_case(target) {
        accept_yul_block(&target.body, visitor);
    }
    visitor.leave_yul_default_case(target);
}

pub fn accept_yul_value_case(target: &YulValueCase, visitor: &mut dyn Visitor) {
    if visitor.enter_yul_value_case(target) {
        accept_yul_literal(&target.value, visitor);
        accept_yul_block(&target.body, visitor);
    }
    visitor.leave_yul_value_case(target);
}

pub fn accept_yul_leave_statement(target: &YulLeaveStatement, visitor: &mut dyn Visitor) {
    visitor.enter_yul_leave_statement(target);
    visitor.leave_yul_leave_statement(target);
}

pub fn accept_yul_break_statement(target: &YulBreakStatement, visitor: &mut dyn Visitor) {
    visitor.enter_yul_break_statement(target);
    visitor.leave_yul_break_statement(target);
}

pub fn accept_yul_continue_statement(target: &YulContinueStatement, visitor: &mut dyn Visitor) {
    visitor.enter_yul_continue_statement(target);
    visitor.leave_yul_continue_statement(target);
}

pub fn accept_yul_label(target: &YulLabel, visitor: &mut dyn Visitor) {
    visitor.enter_yul_label(target);
    visitor.leave_yul_label(target);
}

pub fn accept_yul_function_call_expression(
    target: &YulFunctionCallExpression,
    visitor: &mut dyn Visitor,
) {
    if visitor.enter_yul_function_call_expression(target) {
        accept_yul_expression(&target.operand, visitor);
        accept_yul_arguments(&target.arguments, visitor);
    }
    visitor.leave_yul_function_call_expression(target);
}

//
// Choices:
//

pub fn accept_source_unit_member(target: &SourceUnitMember, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_pragma(target: &Pragma, visitor: &mut dyn Visitor) {
    match target {
        Pragma::AbicoderPragma(ref abicoder_pragma) => {
            accept_abicoder_pragma(abicoder_pragma, visitor);
        }
        Pragma::ExperimentalPragma(ref experimental_pragma) => {
            accept_experimental_pragma(experimental_pragma, visitor);
        }
        Pragma::VersionPragma(ref version_pragma) => {
            accept_version_pragma(version_pragma, visitor);
        }
    }
}

pub fn accept_experimental_feature(target: &ExperimentalFeature, visitor: &mut dyn Visitor) {
    match target {
        ExperimentalFeature::StringLiteral(ref string_literal) => {
            accept_string_literal(string_literal, visitor);
        }
        ExperimentalFeature::Identifier(_) => {}
    }
}

pub fn accept_version_expression(target: &VersionExpression, visitor: &mut dyn Visitor) {
    match target {
        VersionExpression::VersionRange(ref version_range) => {
            accept_version_range(version_range, visitor);
        }
        VersionExpression::VersionTerm(ref version_term) => {
            accept_version_term(version_term, visitor);
        }
    }
}

pub fn accept_version_operator(_target: &VersionOperator, _visitor: &mut dyn Visitor) {}

pub fn accept_version_literal(target: &VersionLiteral, visitor: &mut dyn Visitor) {
    match target {
        VersionLiteral::SimpleVersionLiteral(ref simple_version_literal) => {
            accept_simple_version_literal(simple_version_literal, visitor);
        }
        VersionLiteral::SingleQuotedVersionLiteral(_)
        | VersionLiteral::DoubleQuotedVersionLiteral(_) => {}
    }
}

pub fn accept_import_clause(target: &ImportClause, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_using_clause(target: &UsingClause, visitor: &mut dyn Visitor) {
    match target {
        UsingClause::IdentifierPath(ref identifier_path) => {
            accept_identifier_path(identifier_path, visitor);
        }
        UsingClause::UsingDeconstruction(ref using_deconstruction) => {
            accept_using_deconstruction(using_deconstruction, visitor);
        }
    }
}

pub fn accept_using_operator(_target: &UsingOperator, _visitor: &mut dyn Visitor) {}

pub fn accept_using_target(target: &UsingTarget, visitor: &mut dyn Visitor) {
    match target {
        UsingTarget::TypeName(ref type_name) => {
            accept_type_name(type_name, visitor);
        }
        UsingTarget::Asterisk => {}
    }
}

pub fn accept_contract_specifier(target: &ContractSpecifier, visitor: &mut dyn Visitor) {
    match target {
        ContractSpecifier::InheritanceSpecifier(ref inheritance_specifier) => {
            accept_inheritance_specifier(inheritance_specifier, visitor);
        }
        ContractSpecifier::StorageLayoutSpecifier(ref storage_layout_specifier) => {
            accept_storage_layout_specifier(storage_layout_specifier, visitor);
        }
    }
}

pub fn accept_contract_member(target: &ContractMember, visitor: &mut dyn Visitor) {
    match target {
        ContractMember::UsingDirective(ref using_directive) => {
            accept_using_directive(using_directive, visitor);
        }
        ContractMember::FunctionDefinition(ref function_definition) => {
            accept_function_definition(function_definition, visitor);
        }
        ContractMember::ConstructorDefinition(ref constructor_definition) => {
            accept_constructor_definition(constructor_definition, visitor);
        }
        ContractMember::ReceiveFunctionDefinition(ref receive_function_definition) => {
            accept_receive_function_definition(receive_function_definition, visitor);
        }
        ContractMember::FallbackFunctionDefinition(ref fallback_function_definition) => {
            accept_fallback_function_definition(fallback_function_definition, visitor);
        }
        ContractMember::ModifierDefinition(ref modifier_definition) => {
            accept_modifier_definition(modifier_definition, visitor);
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
}

pub fn accept_state_variable_attribute(target: &StateVariableAttribute, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_function_name(_target: &FunctionName, _visitor: &mut dyn Visitor) {}

pub fn accept_function_attribute(target: &FunctionAttribute, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_function_body(target: &FunctionBody, visitor: &mut dyn Visitor) {
    match target {
        FunctionBody::Block(ref block) => {
            accept_block(block, visitor);
        }
        FunctionBody::Semicolon => {}
    }
}

pub fn accept_constructor_attribute(target: &ConstructorAttribute, visitor: &mut dyn Visitor) {
    match target {
        ConstructorAttribute::ModifierInvocation(ref modifier_invocation) => {
            accept_modifier_invocation(modifier_invocation, visitor);
        }
        ConstructorAttribute::InternalKeyword
        | ConstructorAttribute::OverrideKeyword
        | ConstructorAttribute::PayableKeyword
        | ConstructorAttribute::PublicKeyword
        | ConstructorAttribute::VirtualKeyword => {}
    }
}

pub fn accept_unnamed_function_attribute(
    target: &UnnamedFunctionAttribute,
    visitor: &mut dyn Visitor,
) {
    match target {
        UnnamedFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
            accept_modifier_invocation(modifier_invocation, visitor);
        }
        UnnamedFunctionAttribute::ConstantKeyword
        | UnnamedFunctionAttribute::ExternalKeyword
        | UnnamedFunctionAttribute::InternalKeyword
        | UnnamedFunctionAttribute::PayableKeyword
        | UnnamedFunctionAttribute::PrivateKeyword
        | UnnamedFunctionAttribute::PublicKeyword
        | UnnamedFunctionAttribute::PureKeyword
        | UnnamedFunctionAttribute::ViewKeyword => {}
    }
}

pub fn accept_fallback_function_attribute(
    target: &FallbackFunctionAttribute,
    visitor: &mut dyn Visitor,
) {
    match target {
        FallbackFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
            accept_modifier_invocation(modifier_invocation, visitor);
        }
        FallbackFunctionAttribute::OverrideSpecifier(ref override_specifier) => {
            accept_override_specifier(override_specifier, visitor);
        }
        FallbackFunctionAttribute::ExternalKeyword
        | FallbackFunctionAttribute::PayableKeyword
        | FallbackFunctionAttribute::PureKeyword
        | FallbackFunctionAttribute::ViewKeyword
        | FallbackFunctionAttribute::VirtualKeyword => {}
    }
}

pub fn accept_receive_function_attribute(
    target: &ReceiveFunctionAttribute,
    visitor: &mut dyn Visitor,
) {
    match target {
        ReceiveFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
            accept_modifier_invocation(modifier_invocation, visitor);
        }
        ReceiveFunctionAttribute::OverrideSpecifier(ref override_specifier) => {
            accept_override_specifier(override_specifier, visitor);
        }
        ReceiveFunctionAttribute::ExternalKeyword
        | ReceiveFunctionAttribute::PayableKeyword
        | ReceiveFunctionAttribute::VirtualKeyword => {}
    }
}

pub fn accept_modifier_attribute(target: &ModifierAttribute, visitor: &mut dyn Visitor) {
    match target {
        ModifierAttribute::OverrideSpecifier(ref override_specifier) => {
            accept_override_specifier(override_specifier, visitor);
        }
        ModifierAttribute::VirtualKeyword => {}
    }
}

pub fn accept_type_name(target: &TypeName, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_function_type_attribute(_target: &FunctionTypeAttribute, _visitor: &mut dyn Visitor) {
}

pub fn accept_mapping_key_type(target: &MappingKeyType, visitor: &mut dyn Visitor) {
    match target {
        MappingKeyType::ElementaryType(ref elementary_type) => {
            accept_elementary_type(elementary_type, visitor);
        }
        MappingKeyType::IdentifierPath(ref identifier_path) => {
            accept_identifier_path(identifier_path, visitor);
        }
    }
}

pub fn accept_elementary_type(target: &ElementaryType, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_statement(target: &Statement, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_tuple_member(target: &TupleMember, visitor: &mut dyn Visitor) {
    match target {
        TupleMember::TypedTupleMember(ref typed_tuple_member) => {
            accept_typed_tuple_member(typed_tuple_member, visitor);
        }
        TupleMember::UntypedTupleMember(ref untyped_tuple_member) => {
            accept_untyped_tuple_member(untyped_tuple_member, visitor);
        }
    }
}

pub fn accept_variable_declaration_type(
    target: &VariableDeclarationType,
    visitor: &mut dyn Visitor,
) {
    match target {
        VariableDeclarationType::TypeName(ref type_name) => {
            accept_type_name(type_name, visitor);
        }
        VariableDeclarationType::VarKeyword => {}
    }
}

pub fn accept_storage_location(_target: &StorageLocation, _visitor: &mut dyn Visitor) {}

pub fn accept_for_statement_initialization(
    target: &ForStatementInitialization,
    visitor: &mut dyn Visitor,
) {
    match target {
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
}

pub fn accept_for_statement_condition(target: &ForStatementCondition, visitor: &mut dyn Visitor) {
    match target {
        ForStatementCondition::ExpressionStatement(ref expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
        ForStatementCondition::Semicolon => {}
    }
}

pub fn accept_expression(target: &Expression, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_arguments_declaration(target: &ArgumentsDeclaration, visitor: &mut dyn Visitor) {
    match target {
        ArgumentsDeclaration::PositionalArgumentsDeclaration(
            ref positional_arguments_declaration,
        ) => {
            accept_positional_arguments_declaration(positional_arguments_declaration, visitor);
        }
        ArgumentsDeclaration::NamedArgumentsDeclaration(ref named_arguments_declaration) => {
            accept_named_arguments_declaration(named_arguments_declaration, visitor);
        }
    }
}

pub fn accept_number_unit(_target: &NumberUnit, _visitor: &mut dyn Visitor) {}

pub fn accept_string_expression(target: &StringExpression, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_string_literal(_target: &StringLiteral, _visitor: &mut dyn Visitor) {}

pub fn accept_hex_string_literal(_target: &HexStringLiteral, _visitor: &mut dyn Visitor) {}

pub fn accept_unicode_string_literal(_target: &UnicodeStringLiteral, _visitor: &mut dyn Visitor) {}

pub fn accept_yul_statement(target: &YulStatement, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_yul_assignment_operator(target: &YulAssignmentOperator, visitor: &mut dyn Visitor) {
    match target {
        YulAssignmentOperator::YulColonAndEqual(ref yul_colon_and_equal) => {
            accept_yul_colon_and_equal(yul_colon_and_equal, visitor);
        }
        YulAssignmentOperator::ColonEqual => {}
    }
}

pub fn accept_yul_stack_assignment_operator(
    target: &YulStackAssignmentOperator,
    visitor: &mut dyn Visitor,
) {
    match target {
        YulStackAssignmentOperator::YulEqualAndColon(ref yul_equal_and_colon) => {
            accept_yul_equal_and_colon(yul_equal_and_colon, visitor);
        }
        YulStackAssignmentOperator::EqualColon => {}
    }
}

pub fn accept_yul_switch_case(target: &YulSwitchCase, visitor: &mut dyn Visitor) {
    match target {
        YulSwitchCase::YulDefaultCase(ref yul_default_case) => {
            accept_yul_default_case(yul_default_case, visitor);
        }
        YulSwitchCase::YulValueCase(ref yul_value_case) => {
            accept_yul_value_case(yul_value_case, visitor);
        }
    }
}

pub fn accept_yul_expression(target: &YulExpression, visitor: &mut dyn Visitor) {
    match target {
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
}

pub fn accept_yul_literal(target: &YulLiteral, visitor: &mut dyn Visitor) {
    match target {
        YulLiteral::HexStringLiteral(ref hex_string_literal) => {
            accept_hex_string_literal(hex_string_literal, visitor);
        }
        YulLiteral::StringLiteral(ref string_literal) => {
            accept_string_literal(string_literal, visitor);
        }
        YulLiteral::YulDecimalLiteral(_) | YulLiteral::YulHexLiteral(_) => {}
        YulLiteral::YulTrueKeyword | YulLiteral::YulFalseKeyword => {}
    }
}

//
// Repeated:
//

#[inline]
fn accept_source_unit_members(items: &[SourceUnitMember], visitor: &mut dyn Visitor) {
    for item in items {
        accept_source_unit_member(item, visitor);
    }
}

#[inline]
fn accept_version_expression_set(items: &[VersionExpression], visitor: &mut dyn Visitor) {
    for item in items {
        accept_version_expression(item, visitor);
    }
}

#[inline]
fn accept_contract_members(items: &[ContractMember], visitor: &mut dyn Visitor) {
    for item in items {
        accept_contract_member(item, visitor);
    }
}

#[inline]
fn accept_interface_members(items: &[ContractMember], visitor: &mut dyn Visitor) {
    for item in items {
        accept_contract_member(item, visitor);
    }
}

#[inline]
fn accept_library_members(items: &[ContractMember], visitor: &mut dyn Visitor) {
    for item in items {
        accept_contract_member(item, visitor);
    }
}

#[inline]
fn accept_struct_members(items: &[StructMember], visitor: &mut dyn Visitor) {
    for item in items {
        accept_struct_member(item, visitor);
    }
}

#[inline]
fn accept_state_variable_attributes(items: &[StateVariableAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        accept_state_variable_attribute(item, visitor);
    }
}

#[inline]
fn accept_function_attributes(items: &[FunctionAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        accept_function_attribute(item, visitor);
    }
}

#[inline]
fn accept_constructor_attributes(items: &[ConstructorAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        accept_constructor_attribute(item, visitor);
    }
}

#[inline]
fn accept_fallback_function_attributes(
    items: &[FallbackFunctionAttribute],
    visitor: &mut dyn Visitor,
) {
    for item in items {
        accept_fallback_function_attribute(item, visitor);
    }
}

#[inline]
fn accept_receive_function_attributes(
    items: &[ReceiveFunctionAttribute],
    visitor: &mut dyn Visitor,
) {
    for item in items {
        accept_receive_function_attribute(item, visitor);
    }
}

#[inline]
fn accept_modifier_attributes(items: &[ModifierAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        accept_modifier_attribute(item, visitor);
    }
}

#[inline]
fn accept_function_type_attributes(items: &[FunctionTypeAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        accept_function_type_attribute(item, visitor);
    }
}

#[inline]
fn accept_statements(items: &[Statement], visitor: &mut dyn Visitor) {
    for item in items {
        accept_statement(item, visitor);
    }
}

#[inline]
fn accept_catch_clauses(items: &[CatchClause], visitor: &mut dyn Visitor) {
    for item in items {
        accept_catch_clause(item, visitor);
    }
}

#[inline]
fn accept_string_literals(items: &[StringLiteral], visitor: &mut dyn Visitor) {
    for item in items {
        accept_string_literal(item, visitor);
    }
}

#[inline]
fn accept_hex_string_literals(items: &[HexStringLiteral], visitor: &mut dyn Visitor) {
    for item in items {
        accept_hex_string_literal(item, visitor);
    }
}

#[inline]
fn accept_unicode_string_literals(items: &[UnicodeStringLiteral], visitor: &mut dyn Visitor) {
    for item in items {
        accept_unicode_string_literal(item, visitor);
    }
}

#[inline]
fn accept_yul_statements(items: &[YulStatement], visitor: &mut dyn Visitor) {
    for item in items {
        accept_yul_statement(item, visitor);
    }
}

#[inline]
fn accept_yul_switch_cases(items: &[YulSwitchCase], visitor: &mut dyn Visitor) {
    for item in items {
        accept_yul_switch_case(item, visitor);
    }
}

//
// Separated:
//

#[inline]
fn accept_version_expression_sets(items: &[VersionExpressionSet], visitor: &mut dyn Visitor) {
    for item in items {
        accept_version_expression_set(item, visitor);
    }
}

#[inline]
fn accept_simple_version_literal(_items: &[Rc<TerminalNode>], _visitor: &mut dyn Visitor) {}

#[inline]
fn accept_import_deconstruction_symbols(
    items: &[ImportDeconstructionSymbol],
    visitor: &mut dyn Visitor,
) {
    for item in items {
        accept_import_deconstruction_symbol(item, visitor);
    }
}

#[inline]
fn accept_using_deconstruction_symbols(
    items: &[UsingDeconstructionSymbol],
    visitor: &mut dyn Visitor,
) {
    for item in items {
        accept_using_deconstruction_symbol(item, visitor);
    }
}

#[inline]
fn accept_inheritance_types(items: &[InheritanceType], visitor: &mut dyn Visitor) {
    for item in items {
        accept_inheritance_type(item, visitor);
    }
}

#[inline]
fn accept_enum_members(_items: &[Rc<TerminalNode>], _visitor: &mut dyn Visitor) {}

#[inline]
fn accept_parameters(items: &[Parameter], visitor: &mut dyn Visitor) {
    for item in items {
        accept_parameter(item, visitor);
    }
}

#[inline]
fn accept_override_paths(items: &[IdentifierPath], visitor: &mut dyn Visitor) {
    for item in items {
        accept_identifier_path(item, visitor);
    }
}

#[inline]
fn accept_event_parameters(items: &[EventParameter], visitor: &mut dyn Visitor) {
    for item in items {
        accept_event_parameter(item, visitor);
    }
}

#[inline]
fn accept_error_parameters(items: &[ErrorParameter], visitor: &mut dyn Visitor) {
    for item in items {
        accept_error_parameter(item, visitor);
    }
}

#[inline]
fn accept_assembly_flags(items: &[StringLiteral], visitor: &mut dyn Visitor) {
    for item in items {
        accept_string_literal(item, visitor);
    }
}

#[inline]
fn accept_tuple_deconstruction_elements(
    items: &[TupleDeconstructionElement],
    visitor: &mut dyn Visitor,
) {
    for item in items {
        accept_tuple_deconstruction_element(item, visitor);
    }
}

#[inline]
fn accept_positional_arguments(items: &[Expression], visitor: &mut dyn Visitor) {
    for item in items {
        accept_expression(item, visitor);
    }
}

#[inline]
fn accept_named_arguments(items: &[NamedArgument], visitor: &mut dyn Visitor) {
    for item in items {
        accept_named_argument(item, visitor);
    }
}

#[inline]
fn accept_call_options(items: &[NamedArgument], visitor: &mut dyn Visitor) {
    for item in items {
        accept_named_argument(item, visitor);
    }
}

#[inline]
fn accept_tuple_values(items: &[TupleValue], visitor: &mut dyn Visitor) {
    for item in items {
        accept_tuple_value(item, visitor);
    }
}

#[inline]
fn accept_array_values(items: &[Expression], visitor: &mut dyn Visitor) {
    for item in items {
        accept_expression(item, visitor);
    }
}

#[inline]
fn accept_identifier_path(_items: &[Rc<TerminalNode>], _visitor: &mut dyn Visitor) {}

#[inline]
fn accept_yul_parameters(_items: &[Rc<TerminalNode>], _visitor: &mut dyn Visitor) {}

#[inline]
fn accept_yul_variable_names(_items: &[Rc<TerminalNode>], _visitor: &mut dyn Visitor) {}

#[inline]
fn accept_yul_arguments(items: &[YulExpression], visitor: &mut dyn Visitor) {
    for item in items {
        accept_yul_expression(item, visitor);
    }
}

#[inline]
fn accept_yul_paths(items: &[YulPath], visitor: &mut dyn Visitor) {
    for item in items {
        accept_yul_path(item, visitor);
    }
}

#[inline]
fn accept_yul_path(_items: &[Rc<TerminalNode>], _visitor: &mut dyn Visitor) {}
