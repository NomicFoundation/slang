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

impl SourceUnitStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_source_unit(self) {
            accept_source_unit_members(&self.members, visitor);
        }
        visitor.leave_source_unit(self);
    }
}

impl PragmaDirectiveStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_pragma_directive(self) {
            self.pragma.accept(visitor);
        }
        visitor.leave_pragma_directive(self);
    }
}

impl AbicoderPragmaStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_abicoder_pragma(self);
        visitor.leave_abicoder_pragma(self);
    }
}

impl ExperimentalPragmaStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_experimental_pragma(self) {
            self.feature.accept(visitor);
        }
        visitor.leave_experimental_pragma(self);
    }
}

impl VersionPragmaStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_version_pragma(self) {
            accept_version_expression_sets(&self.sets, visitor);
        }
        visitor.leave_version_pragma(self);
    }
}

impl VersionRangeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_version_range(self) {
            self.start.accept(visitor);

            self.end.accept(visitor);
        }
        visitor.leave_version_range(self);
    }
}

impl VersionTermStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_version_term(self) {
            if let Some(ref operator) = self.operator {
                operator.accept(visitor);
            }

            self.literal.accept(visitor);
        }
        visitor.leave_version_term(self);
    }
}

impl ImportDirectiveStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_import_directive(self) {
            self.clause.accept(visitor);
        }
        visitor.leave_import_directive(self);
    }
}

impl PathImportStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_path_import(self) {
            self.path.accept(visitor);

            if let Some(ref alias) = self.alias {
                alias.accept(visitor);
            }
        }
        visitor.leave_path_import(self);
    }
}

impl NamedImportStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_named_import(self) {
            self.alias.accept(visitor);

            self.path.accept(visitor);
        }
        visitor.leave_named_import(self);
    }
}

impl ImportDeconstructionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_import_deconstruction(self) {
            accept_import_deconstruction_symbols(&self.symbols, visitor);

            self.path.accept(visitor);
        }
        visitor.leave_import_deconstruction(self);
    }
}

impl ImportDeconstructionSymbolStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_import_deconstruction_symbol(self) {
            if let Some(ref alias) = self.alias {
                alias.accept(visitor);
            }
        }
        visitor.leave_import_deconstruction_symbol(self);
    }
}

impl ImportAliasStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_import_alias(self);
        visitor.leave_import_alias(self);
    }
}

impl UsingDirectiveStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_using_directive(self) {
            self.clause.accept(visitor);

            self.target.accept(visitor);
        }
        visitor.leave_using_directive(self);
    }
}

impl UsingDeconstructionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_using_deconstruction(self) {
            accept_using_deconstruction_symbols(&self.symbols, visitor);
        }
        visitor.leave_using_deconstruction(self);
    }
}

impl UsingDeconstructionSymbolStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_using_deconstruction_symbol(self) {
            accept_identifier_path(&self.name, visitor);

            if let Some(ref alias) = self.alias {
                alias.accept(visitor);
            }
        }
        visitor.leave_using_deconstruction_symbol(self);
    }
}

impl UsingAliasStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_using_alias(self) {
            self.operator.accept(visitor);
        }
        visitor.leave_using_alias(self);
    }
}

impl ContractDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_contract_definition(self) {
            accept_contract_members(&self.members, visitor);

            accept_inheritance_types(&self.inheritance_types, visitor);
        }
        visitor.leave_contract_definition(self);
    }
}

impl InheritanceSpecifierStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_inheritance_specifier(self) {
            accept_inheritance_types(&self.types, visitor);
        }
        visitor.leave_inheritance_specifier(self);
    }
}

impl InheritanceTypeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_inheritance_type(self) {
            accept_identifier_path(&self.type_name, visitor);

            if let Some(ref arguments) = self.arguments {
                arguments.accept(visitor);
            }
        }
        visitor.leave_inheritance_type(self);
    }
}

impl InterfaceDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_interface_definition(self) {
            if let Some(ref inheritance) = self.inheritance {
                inheritance.accept(visitor);
            }

            accept_interface_members(&self.members, visitor);
        }
        visitor.leave_interface_definition(self);
    }
}

impl LibraryDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_library_definition(self) {
            accept_library_members(&self.members, visitor);
        }
        visitor.leave_library_definition(self);
    }
}

impl StructDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_struct_definition(self) {
            accept_struct_members(&self.members, visitor);
        }
        visitor.leave_struct_definition(self);
    }
}

impl StructMemberStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_struct_member(self) {
            self.type_name.accept(visitor);
        }
        visitor.leave_struct_member(self);
    }
}

impl EnumDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_enum_definition(self) {
            accept_enum_members(&self.members, visitor);
        }
        visitor.leave_enum_definition(self);
    }
}

impl ConstantDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_constant_definition(self) {
            self.type_name.accept(visitor);

            self.value.accept(visitor);
        }
        visitor.leave_constant_definition(self);
    }
}

impl StateVariableDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_state_variable_definition(self) {
            self.type_name.accept(visitor);

            accept_state_variable_attributes(&self.attributes, visitor);

            if let Some(ref value) = self.value {
                value.accept(visitor);
            }
        }
        visitor.leave_state_variable_definition(self);
    }
}

impl StateVariableDefinitionValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_state_variable_definition_value(self) {
            self.value.accept(visitor);
        }
        visitor.leave_state_variable_definition_value(self);
    }
}

impl FunctionDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_function_definition(self) {
            self.name.accept(visitor);

            self.parameters.accept(visitor);

            accept_function_attributes(&self.attributes, visitor);

            if let Some(ref returns) = self.returns {
                returns.accept(visitor);
            }

            self.body.accept(visitor);
        }
        visitor.leave_function_definition(self);
    }
}

impl ParametersDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_parameters_declaration(self) {
            accept_parameters(&self.parameters, visitor);
        }
        visitor.leave_parameters_declaration(self);
    }
}

impl ParameterStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_parameter(self) {
            self.type_name.accept(visitor);

            if let Some(ref storage_location) = self.storage_location {
                storage_location.accept(visitor);
            }
        }
        visitor.leave_parameter(self);
    }
}

impl OverrideSpecifierStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_override_specifier(self) {
            if let Some(ref overridden) = self.overridden {
                overridden.accept(visitor);
            }
        }
        visitor.leave_override_specifier(self);
    }
}

impl OverridePathsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_override_paths_declaration(self) {
            accept_override_paths(&self.paths, visitor);
        }
        visitor.leave_override_paths_declaration(self);
    }
}

impl ReturnsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_returns_declaration(self) {
            self.variables.accept(visitor);
        }
        visitor.leave_returns_declaration(self);
    }
}

impl ConstructorDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_constructor_definition(self) {
            self.parameters.accept(visitor);

            accept_constructor_attributes(&self.attributes, visitor);

            self.body.accept(visitor);
        }
        visitor.leave_constructor_definition(self);
    }
}

impl FallbackFunctionDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_fallback_function_definition(self) {
            self.parameters.accept(visitor);

            accept_fallback_function_attributes(&self.attributes, visitor);

            if let Some(ref returns) = self.returns {
                returns.accept(visitor);
            }

            self.body.accept(visitor);
        }
        visitor.leave_fallback_function_definition(self);
    }
}

impl ReceiveFunctionDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_receive_function_definition(self) {
            self.parameters.accept(visitor);

            accept_receive_function_attributes(&self.attributes, visitor);

            self.body.accept(visitor);
        }
        visitor.leave_receive_function_definition(self);
    }
}

impl ModifierDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_modifier_definition(self) {
            if let Some(ref parameters) = self.parameters {
                parameters.accept(visitor);
            }

            accept_modifier_attributes(&self.attributes, visitor);

            self.body.accept(visitor);
        }
        visitor.leave_modifier_definition(self);
    }
}

impl ModifierInvocationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_modifier_invocation(self) {
            accept_identifier_path(&self.name, visitor);

            if let Some(ref arguments) = self.arguments {
                arguments.accept(visitor);
            }
        }
        visitor.leave_modifier_invocation(self);
    }
}

impl EventDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_event_definition(self) {
            self.parameters.accept(visitor);
        }
        visitor.leave_event_definition(self);
    }
}

impl EventParametersDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_event_parameters_declaration(self) {
            accept_event_parameters(&self.parameters, visitor);
        }
        visitor.leave_event_parameters_declaration(self);
    }
}

impl EventParameterStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_event_parameter(self) {
            self.type_name.accept(visitor);
        }
        visitor.leave_event_parameter(self);
    }
}

impl UserDefinedValueTypeDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_user_defined_value_type_definition(self) {
            self.value_type.accept(visitor);
        }
        visitor.leave_user_defined_value_type_definition(self);
    }
}

impl ErrorDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_error_definition(self) {
            self.members.accept(visitor);
        }
        visitor.leave_error_definition(self);
    }
}

impl ErrorParametersDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_error_parameters_declaration(self) {
            accept_error_parameters(&self.parameters, visitor);
        }
        visitor.leave_error_parameters_declaration(self);
    }
}

impl ErrorParameterStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_error_parameter(self) {
            self.type_name.accept(visitor);
        }
        visitor.leave_error_parameter(self);
    }
}

impl ArrayTypeNameStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_array_type_name(self) {
            self.operand.accept(visitor);

            if let Some(ref index) = self.index {
                index.accept(visitor);
            }
        }
        visitor.leave_array_type_name(self);
    }
}

impl FunctionTypeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_function_type(self) {
            self.parameters.accept(visitor);

            accept_function_type_attributes(&self.attributes, visitor);

            if let Some(ref returns) = self.returns {
                returns.accept(visitor);
            }
        }
        visitor.leave_function_type(self);
    }
}

impl MappingTypeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_mapping_type(self) {
            self.key_type.accept(visitor);

            self.value_type.accept(visitor);
        }
        visitor.leave_mapping_type(self);
    }
}

impl MappingKeyStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_mapping_key(self) {
            self.key_type.accept(visitor);
        }
        visitor.leave_mapping_key(self);
    }
}

impl MappingValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_mapping_value(self) {
            self.type_name.accept(visitor);
        }
        visitor.leave_mapping_value(self);
    }
}

impl AddressTypeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_address_type(self);
        visitor.leave_address_type(self);
    }
}

impl BlockStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_block(self) {
            accept_statements(&self.statements, visitor);
        }
        visitor.leave_block(self);
    }
}

impl UncheckedBlockStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_unchecked_block(self) {
            self.block.accept(visitor);
        }
        visitor.leave_unchecked_block(self);
    }
}

impl ExpressionStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_expression_statement(self) {
            self.expression.accept(visitor);
        }
        visitor.leave_expression_statement(self);
    }
}

impl AssemblyStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_assembly_statement(self) {
            if let Some(ref label) = self.label {
                label.accept(visitor);
            }

            if let Some(ref flags) = self.flags {
                flags.accept(visitor);
            }

            self.body.accept(visitor);
        }
        visitor.leave_assembly_statement(self);
    }
}

impl AssemblyFlagsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_assembly_flags_declaration(self) {
            accept_assembly_flags(&self.flags, visitor);
        }
        visitor.leave_assembly_flags_declaration(self);
    }
}

impl TupleDeconstructionStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_tuple_deconstruction_statement(self) {
            accept_tuple_deconstruction_elements(&self.elements, visitor);

            self.expression.accept(visitor);
        }
        visitor.leave_tuple_deconstruction_statement(self);
    }
}

impl TupleDeconstructionElementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_tuple_deconstruction_element(self) {
            if let Some(ref member) = self.member {
                member.accept(visitor);
            }
        }
        visitor.leave_tuple_deconstruction_element(self);
    }
}

impl TypedTupleMemberStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_typed_tuple_member(self) {
            self.type_name.accept(visitor);

            if let Some(ref storage_location) = self.storage_location {
                storage_location.accept(visitor);
            }
        }
        visitor.leave_typed_tuple_member(self);
    }
}

impl UntypedTupleMemberStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_untyped_tuple_member(self) {
            if let Some(ref storage_location) = self.storage_location {
                storage_location.accept(visitor);
            }
        }
        visitor.leave_untyped_tuple_member(self);
    }
}

impl VariableDeclarationStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_variable_declaration_statement(self) {
            self.variable_type.accept(visitor);

            if let Some(ref storage_location) = self.storage_location {
                storage_location.accept(visitor);
            }

            if let Some(ref value) = self.value {
                value.accept(visitor);
            }
        }
        visitor.leave_variable_declaration_statement(self);
    }
}

impl VariableDeclarationValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_variable_declaration_value(self) {
            self.expression.accept(visitor);
        }
        visitor.leave_variable_declaration_value(self);
    }
}

impl IfStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_if_statement(self) {
            self.condition.accept(visitor);

            self.body.accept(visitor);

            if let Some(ref else_branch) = self.else_branch {
                else_branch.accept(visitor);
            }
        }
        visitor.leave_if_statement(self);
    }
}

impl ElseBranchStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_else_branch(self) {
            self.body.accept(visitor);
        }
        visitor.leave_else_branch(self);
    }
}

impl ForStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_for_statement(self) {
            self.initialization.accept(visitor);

            self.condition.accept(visitor);

            if let Some(ref iterator) = self.iterator {
                iterator.accept(visitor);
            }

            self.body.accept(visitor);
        }
        visitor.leave_for_statement(self);
    }
}

impl WhileStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_while_statement(self) {
            self.condition.accept(visitor);

            self.body.accept(visitor);
        }
        visitor.leave_while_statement(self);
    }
}

impl DoWhileStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_do_while_statement(self) {
            self.body.accept(visitor);

            self.condition.accept(visitor);
        }
        visitor.leave_do_while_statement(self);
    }
}

impl ContinueStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_continue_statement(self);
        visitor.leave_continue_statement(self);
    }
}

impl BreakStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_break_statement(self);
        visitor.leave_break_statement(self);
    }
}

impl ReturnStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_return_statement(self) {
            if let Some(ref expression) = self.expression {
                expression.accept(visitor);
            }
        }
        visitor.leave_return_statement(self);
    }
}

impl EmitStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_emit_statement(self) {
            accept_identifier_path(&self.event, visitor);

            self.arguments.accept(visitor);
        }
        visitor.leave_emit_statement(self);
    }
}

impl TryStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_try_statement(self) {
            self.expression.accept(visitor);

            if let Some(ref returns) = self.returns {
                returns.accept(visitor);
            }

            self.body.accept(visitor);

            accept_catch_clauses(&self.catch_clauses, visitor);
        }
        visitor.leave_try_statement(self);
    }
}

impl CatchClauseStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_catch_clause(self) {
            if let Some(ref error) = self.error {
                error.accept(visitor);
            }

            self.body.accept(visitor);
        }
        visitor.leave_catch_clause(self);
    }
}

impl CatchClauseErrorStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_catch_clause_error(self) {
            self.parameters.accept(visitor);
        }
        visitor.leave_catch_clause_error(self);
    }
}

impl RevertStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_revert_statement(self) {
            if let Some(ref error) = self.error {
                accept_identifier_path(error, visitor);
            }

            self.arguments.accept(visitor);
        }
        visitor.leave_revert_statement(self);
    }
}

impl ThrowStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_throw_statement(self);
        visitor.leave_throw_statement(self);
    }
}

impl AssignmentExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_assignment_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_assignment_expression(self);
    }
}

impl ConditionalExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_conditional_expression(self) {
            self.operand.accept(visitor);

            self.true_expression.accept(visitor);

            self.false_expression.accept(visitor);
        }
        visitor.leave_conditional_expression(self);
    }
}

impl OrExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_or_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_or_expression(self);
    }
}

impl AndExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_and_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_and_expression(self);
    }
}

impl EqualityExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_equality_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_equality_expression(self);
    }
}

impl InequalityExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_inequality_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_inequality_expression(self);
    }
}

impl BitwiseOrExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_bitwise_or_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_bitwise_or_expression(self);
    }
}

impl BitwiseXorExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_bitwise_xor_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_bitwise_xor_expression(self);
    }
}

impl BitwiseAndExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_bitwise_and_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_bitwise_and_expression(self);
    }
}

impl ShiftExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_shift_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_shift_expression(self);
    }
}

impl AdditiveExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_additive_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_additive_expression(self);
    }
}

impl MultiplicativeExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_multiplicative_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_multiplicative_expression(self);
    }
}

impl ExponentiationExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_exponentiation_expression(self) {
            self.left_operand.accept(visitor);

            self.right_operand.accept(visitor);
        }
        visitor.leave_exponentiation_expression(self);
    }
}

impl PostfixExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_postfix_expression(self) {
            self.operand.accept(visitor);
        }
        visitor.leave_postfix_expression(self);
    }
}

impl PrefixExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_prefix_expression(self) {
            self.operand.accept(visitor);
        }
        visitor.leave_prefix_expression(self);
    }
}

impl FunctionCallExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_function_call_expression(self) {
            self.operand.accept(visitor);

            self.arguments.accept(visitor);
        }
        visitor.leave_function_call_expression(self);
    }
}

impl CallOptionsExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_call_options_expression(self) {
            self.operand.accept(visitor);

            accept_call_options(&self.options, visitor);
        }
        visitor.leave_call_options_expression(self);
    }
}

impl MemberAccessExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_member_access_expression(self) {
            self.operand.accept(visitor);
        }
        visitor.leave_member_access_expression(self);
    }
}

impl IndexAccessExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_index_access_expression(self) {
            self.operand.accept(visitor);

            if let Some(ref start) = self.start {
                start.accept(visitor);
            }

            if let Some(ref end) = self.end {
                end.accept(visitor);
            }
        }
        visitor.leave_index_access_expression(self);
    }
}

impl IndexAccessEndStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_index_access_end(self) {
            if let Some(ref end) = self.end {
                end.accept(visitor);
            }
        }
        visitor.leave_index_access_end(self);
    }
}

impl PositionalArgumentsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_positional_arguments_declaration(self) {
            accept_positional_arguments(&self.arguments, visitor);
        }
        visitor.leave_positional_arguments_declaration(self);
    }
}

impl NamedArgumentsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_named_arguments_declaration(self) {
            if let Some(ref arguments) = self.arguments {
                arguments.accept(visitor);
            }
        }
        visitor.leave_named_arguments_declaration(self);
    }
}

impl NamedArgumentGroupStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_named_argument_group(self) {
            accept_named_arguments(&self.arguments, visitor);
        }
        visitor.leave_named_argument_group(self);
    }
}

impl NamedArgumentStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_named_argument(self) {
            self.value.accept(visitor);
        }
        visitor.leave_named_argument(self);
    }
}

impl TypeExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_type_expression(self) {
            self.type_name.accept(visitor);
        }
        visitor.leave_type_expression(self);
    }
}

impl NewExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_new_expression(self) {
            self.type_name.accept(visitor);
        }
        visitor.leave_new_expression(self);
    }
}

impl TupleExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_tuple_expression(self) {
            accept_tuple_values(&self.items, visitor);
        }
        visitor.leave_tuple_expression(self);
    }
}

impl TupleValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_tuple_value(self) {
            if let Some(ref expression) = self.expression {
                expression.accept(visitor);
            }
        }
        visitor.leave_tuple_value(self);
    }
}

impl ArrayExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_array_expression(self) {
            accept_array_values(&self.items, visitor);
        }
        visitor.leave_array_expression(self);
    }
}

impl HexNumberExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_hex_number_expression(self) {
            if let Some(ref unit) = self.unit {
                unit.accept(visitor);
            }
        }
        visitor.leave_hex_number_expression(self);
    }
}

impl DecimalNumberExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_decimal_number_expression(self) {
            if let Some(ref unit) = self.unit {
                unit.accept(visitor);
            }
        }
        visitor.leave_decimal_number_expression(self);
    }
}

impl YulBlockStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_block(self) {
            accept_yul_statements(&self.statements, visitor);
        }
        visitor.leave_yul_block(self);
    }
}

impl YulFunctionDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_function_definition(self) {
            self.parameters.accept(visitor);

            if let Some(ref returns) = self.returns {
                returns.accept(visitor);
            }

            self.body.accept(visitor);
        }
        visitor.leave_yul_function_definition(self);
    }
}

impl YulParametersDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_parameters_declaration(self) {
            accept_yul_parameters(&self.parameters, visitor);
        }
        visitor.leave_yul_parameters_declaration(self);
    }
}

impl YulReturnsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_returns_declaration(self) {
            accept_yul_variable_names(&self.variables, visitor);
        }
        visitor.leave_yul_returns_declaration(self);
    }
}

impl YulVariableDeclarationStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_variable_declaration_statement(self) {
            accept_yul_variable_names(&self.variables, visitor);

            if let Some(ref value) = self.value {
                value.accept(visitor);
            }
        }
        visitor.leave_yul_variable_declaration_statement(self);
    }
}

impl YulVariableDeclarationValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_variable_declaration_value(self) {
            self.assignment.accept(visitor);

            self.expression.accept(visitor);
        }
        visitor.leave_yul_variable_declaration_value(self);
    }
}

impl YulVariableAssignmentStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_variable_assignment_statement(self) {
            accept_yul_paths(&self.variables, visitor);

            self.assignment.accept(visitor);

            self.expression.accept(visitor);
        }
        visitor.leave_yul_variable_assignment_statement(self);
    }
}

impl YulColonAndEqualStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_yul_colon_and_equal(self);
        visitor.leave_yul_colon_and_equal(self);
    }
}

impl YulStackAssignmentStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_stack_assignment_statement(self) {
            self.assignment.accept(visitor);
        }
        visitor.leave_yul_stack_assignment_statement(self);
    }
}

impl YulEqualAndColonStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_yul_equal_and_colon(self);
        visitor.leave_yul_equal_and_colon(self);
    }
}

impl YulIfStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_if_statement(self) {
            self.condition.accept(visitor);

            self.body.accept(visitor);
        }
        visitor.leave_yul_if_statement(self);
    }
}

impl YulForStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_for_statement(self) {
            self.initialization.accept(visitor);

            self.condition.accept(visitor);

            self.iterator.accept(visitor);

            self.body.accept(visitor);
        }
        visitor.leave_yul_for_statement(self);
    }
}

impl YulSwitchStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_switch_statement(self) {
            self.expression.accept(visitor);

            accept_yul_switch_cases(&self.cases, visitor);
        }
        visitor.leave_yul_switch_statement(self);
    }
}

impl YulDefaultCaseStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_default_case(self) {
            self.body.accept(visitor);
        }
        visitor.leave_yul_default_case(self);
    }
}

impl YulValueCaseStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_value_case(self) {
            self.value.accept(visitor);

            self.body.accept(visitor);
        }
        visitor.leave_yul_value_case(self);
    }
}

impl YulLeaveStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_yul_leave_statement(self);
        visitor.leave_yul_leave_statement(self);
    }
}

impl YulBreakStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_yul_break_statement(self);
        visitor.leave_yul_break_statement(self);
    }
}

impl YulContinueStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_yul_continue_statement(self);
        visitor.leave_yul_continue_statement(self);
    }
}

impl YulLabelStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.enter_yul_label(self);
        visitor.leave_yul_label(self);
    }
}

impl YulFunctionCallExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if visitor.enter_yul_function_call_expression(self) {
            self.operand.accept(visitor);

            accept_yul_arguments(&self.arguments, visitor);
        }
        visitor.leave_yul_function_call_expression(self);
    }
}

//
// Choices:
//

impl SourceUnitMember {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::PragmaDirective(ref pragma_directive) => {
                pragma_directive.accept(visitor);
            }
            Self::ImportDirective(ref import_directive) => {
                import_directive.accept(visitor);
            }
            Self::ContractDefinition(ref contract_definition) => {
                contract_definition.accept(visitor);
            }
            Self::InterfaceDefinition(ref interface_definition) => {
                interface_definition.accept(visitor);
            }
            Self::LibraryDefinition(ref library_definition) => {
                library_definition.accept(visitor);
            }
            Self::StructDefinition(ref struct_definition) => {
                struct_definition.accept(visitor);
            }
            Self::EnumDefinition(ref enum_definition) => {
                enum_definition.accept(visitor);
            }
            Self::FunctionDefinition(ref function_definition) => {
                function_definition.accept(visitor);
            }
            Self::ErrorDefinition(ref error_definition) => {
                error_definition.accept(visitor);
            }
            Self::UserDefinedValueTypeDefinition(ref user_defined_value_type_definition) => {
                user_defined_value_type_definition.accept(visitor);
            }
            Self::UsingDirective(ref using_directive) => {
                using_directive.accept(visitor);
            }
            Self::EventDefinition(ref event_definition) => {
                event_definition.accept(visitor);
            }
            Self::ConstantDefinition(ref constant_definition) => {
                constant_definition.accept(visitor);
            }
        }
    }
}

impl Pragma {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::AbicoderPragma(ref abicoder_pragma) => {
                abicoder_pragma.accept(visitor);
            }
            Self::ExperimentalPragma(ref experimental_pragma) => {
                experimental_pragma.accept(visitor);
            }
            Self::VersionPragma(ref version_pragma) => {
                version_pragma.accept(visitor);
            }
        }
    }
}

impl ExperimentalFeature {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::StringLiteral(ref string_literal) => {
                string_literal.accept(visitor);
            }
            Self::Identifier(_) => {}
        }
    }
}

impl VersionExpression {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::VersionRange(ref version_range) => {
                version_range.accept(visitor);
            }
            Self::VersionTerm(ref version_term) => {
                version_term.accept(visitor);
            }
        }
    }
}

impl VersionOperator {
    pub fn accept(&self, _visitor: &mut dyn Visitor) {}
}

impl VersionLiteral {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::SimpleVersionLiteral(ref simple_version_literal) => {
                accept_simple_version_literal(simple_version_literal, visitor);
            }
            Self::SingleQuotedVersionLiteral(_) | Self::DoubleQuotedVersionLiteral(_) => {}
        }
    }
}

impl ImportClause {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::PathImport(ref path_import) => {
                path_import.accept(visitor);
            }
            Self::NamedImport(ref named_import) => {
                named_import.accept(visitor);
            }
            Self::ImportDeconstruction(ref import_deconstruction) => {
                import_deconstruction.accept(visitor);
            }
        }
    }
}

impl UsingClause {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::IdentifierPath(ref identifier_path) => {
                accept_identifier_path(identifier_path, visitor);
            }
            Self::UsingDeconstruction(ref using_deconstruction) => {
                using_deconstruction.accept(visitor);
            }
        }
    }
}

impl UsingOperator {
    pub fn accept(&self, _visitor: &mut dyn Visitor) {}
}

impl UsingTarget {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::TypeName(ref type_name) => {
                type_name.accept(visitor);
            }
            Self::Asterisk => {}
        }
    }
}

impl ContractMember {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::UsingDirective(ref using_directive) => {
                using_directive.accept(visitor);
            }
            Self::FunctionDefinition(ref function_definition) => {
                function_definition.accept(visitor);
            }
            Self::ConstructorDefinition(ref constructor_definition) => {
                constructor_definition.accept(visitor);
            }
            Self::ReceiveFunctionDefinition(ref receive_function_definition) => {
                receive_function_definition.accept(visitor);
            }
            Self::FallbackFunctionDefinition(ref fallback_function_definition) => {
                fallback_function_definition.accept(visitor);
            }
            Self::ModifierDefinition(ref modifier_definition) => {
                modifier_definition.accept(visitor);
            }
            Self::StructDefinition(ref struct_definition) => {
                struct_definition.accept(visitor);
            }
            Self::EnumDefinition(ref enum_definition) => {
                enum_definition.accept(visitor);
            }
            Self::EventDefinition(ref event_definition) => {
                event_definition.accept(visitor);
            }
            Self::ErrorDefinition(ref error_definition) => {
                error_definition.accept(visitor);
            }
            Self::UserDefinedValueTypeDefinition(ref user_defined_value_type_definition) => {
                user_defined_value_type_definition.accept(visitor);
            }
            Self::StateVariableDefinition(ref state_variable_definition) => {
                state_variable_definition.accept(visitor);
            }
        }
    }
}

impl StateVariableAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::OverrideSpecifier(ref override_specifier) => {
                override_specifier.accept(visitor);
            }
            Self::ConstantKeyword
            | Self::InternalKeyword
            | Self::PrivateKeyword
            | Self::PublicKeyword
            | Self::ImmutableKeyword
            | Self::TransientKeyword => {}
        }
    }
}

impl FunctionName {
    pub fn accept(&self, _visitor: &mut dyn Visitor) {}
}

impl FunctionAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ModifierInvocation(ref modifier_invocation) => {
                modifier_invocation.accept(visitor);
            }
            Self::OverrideSpecifier(ref override_specifier) => {
                override_specifier.accept(visitor);
            }
            Self::ConstantKeyword
            | Self::ExternalKeyword
            | Self::InternalKeyword
            | Self::PayableKeyword
            | Self::PrivateKeyword
            | Self::PublicKeyword
            | Self::PureKeyword
            | Self::ViewKeyword
            | Self::VirtualKeyword => {}
        }
    }
}

impl FunctionBody {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::Block(ref block) => {
                block.accept(visitor);
            }
            Self::Semicolon => {}
        }
    }
}

impl ConstructorAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ModifierInvocation(ref modifier_invocation) => {
                modifier_invocation.accept(visitor);
            }
            Self::InternalKeyword
            | Self::OverrideKeyword
            | Self::PayableKeyword
            | Self::PublicKeyword
            | Self::VirtualKeyword => {}
        }
    }
}

impl UnnamedFunctionAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ModifierInvocation(ref modifier_invocation) => {
                modifier_invocation.accept(visitor);
            }
            Self::ConstantKeyword
            | Self::ExternalKeyword
            | Self::InternalKeyword
            | Self::PayableKeyword
            | Self::PrivateKeyword
            | Self::PublicKeyword
            | Self::PureKeyword
            | Self::ViewKeyword => {}
        }
    }
}

impl FallbackFunctionAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ModifierInvocation(ref modifier_invocation) => {
                modifier_invocation.accept(visitor);
            }
            Self::OverrideSpecifier(ref override_specifier) => {
                override_specifier.accept(visitor);
            }
            Self::ExternalKeyword
            | Self::PayableKeyword
            | Self::PureKeyword
            | Self::ViewKeyword
            | Self::VirtualKeyword => {}
        }
    }
}

impl ReceiveFunctionAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ModifierInvocation(ref modifier_invocation) => {
                modifier_invocation.accept(visitor);
            }
            Self::OverrideSpecifier(ref override_specifier) => {
                override_specifier.accept(visitor);
            }
            Self::ExternalKeyword | Self::PayableKeyword | Self::VirtualKeyword => {}
        }
    }
}

impl ModifierAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::OverrideSpecifier(ref override_specifier) => {
                override_specifier.accept(visitor);
            }
            Self::VirtualKeyword => {}
        }
    }
}

impl TypeName {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ArrayTypeName(ref array_type_name) => {
                array_type_name.accept(visitor);
            }
            Self::FunctionType(ref function_type) => {
                function_type.accept(visitor);
            }
            Self::MappingType(ref mapping_type) => {
                mapping_type.accept(visitor);
            }
            Self::ElementaryType(ref elementary_type) => {
                elementary_type.accept(visitor);
            }
            Self::IdentifierPath(ref identifier_path) => {
                accept_identifier_path(identifier_path, visitor);
            }
        }
    }
}

impl FunctionTypeAttribute {
    pub fn accept(&self, _visitor: &mut dyn Visitor) {}
}

impl MappingKeyType {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ElementaryType(ref elementary_type) => {
                elementary_type.accept(visitor);
            }
            Self::IdentifierPath(ref identifier_path) => {
                accept_identifier_path(identifier_path, visitor);
            }
        }
    }
}

impl ElementaryType {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::AddressType(ref address_type) => {
                address_type.accept(visitor);
            }
            Self::BoolKeyword
            | Self::ByteKeyword
            | Self::StringKeyword
            | Self::BytesKeyword(_)
            | Self::IntKeyword(_)
            | Self::UintKeyword(_)
            | Self::FixedKeyword(_)
            | Self::UfixedKeyword(_) => {}
        }
    }
}

impl Statement {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::IfStatement(ref if_statement) => {
                if_statement.accept(visitor);
            }
            Self::ForStatement(ref for_statement) => {
                for_statement.accept(visitor);
            }
            Self::WhileStatement(ref while_statement) => {
                while_statement.accept(visitor);
            }
            Self::DoWhileStatement(ref do_while_statement) => {
                do_while_statement.accept(visitor);
            }
            Self::ContinueStatement(ref continue_statement) => {
                continue_statement.accept(visitor);
            }
            Self::BreakStatement(ref break_statement) => {
                break_statement.accept(visitor);
            }
            Self::ReturnStatement(ref return_statement) => {
                return_statement.accept(visitor);
            }
            Self::ThrowStatement(ref throw_statement) => {
                throw_statement.accept(visitor);
            }
            Self::EmitStatement(ref emit_statement) => {
                emit_statement.accept(visitor);
            }
            Self::TryStatement(ref try_statement) => {
                try_statement.accept(visitor);
            }
            Self::RevertStatement(ref revert_statement) => {
                revert_statement.accept(visitor);
            }
            Self::AssemblyStatement(ref assembly_statement) => {
                assembly_statement.accept(visitor);
            }
            Self::Block(ref block) => {
                block.accept(visitor);
            }
            Self::UncheckedBlock(ref unchecked_block) => {
                unchecked_block.accept(visitor);
            }
            Self::TupleDeconstructionStatement(ref tuple_deconstruction_statement) => {
                tuple_deconstruction_statement.accept(visitor);
            }
            Self::VariableDeclarationStatement(ref variable_declaration_statement) => {
                variable_declaration_statement.accept(visitor);
            }
            Self::ExpressionStatement(ref expression_statement) => {
                expression_statement.accept(visitor);
            }
        }
    }
}

impl TupleMember {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::TypedTupleMember(ref typed_tuple_member) => {
                typed_tuple_member.accept(visitor);
            }
            Self::UntypedTupleMember(ref untyped_tuple_member) => {
                untyped_tuple_member.accept(visitor);
            }
        }
    }
}

impl VariableDeclarationType {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::TypeName(ref type_name) => {
                type_name.accept(visitor);
            }
            Self::VarKeyword => {}
        }
    }
}

impl StorageLocation {
    pub fn accept(&self, _visitor: &mut dyn Visitor) {}
}

impl ForStatementInitialization {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::TupleDeconstructionStatement(ref tuple_deconstruction_statement) => {
                tuple_deconstruction_statement.accept(visitor);
            }
            Self::VariableDeclarationStatement(ref variable_declaration_statement) => {
                variable_declaration_statement.accept(visitor);
            }
            Self::ExpressionStatement(ref expression_statement) => {
                expression_statement.accept(visitor);
            }
            Self::Semicolon => {}
        }
    }
}

impl ForStatementCondition {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ExpressionStatement(ref expression_statement) => {
                expression_statement.accept(visitor);
            }
            Self::Semicolon => {}
        }
    }
}

impl Expression {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::AssignmentExpression(ref assignment_expression) => {
                assignment_expression.accept(visitor);
            }
            Self::ConditionalExpression(ref conditional_expression) => {
                conditional_expression.accept(visitor);
            }
            Self::OrExpression(ref or_expression) => {
                or_expression.accept(visitor);
            }
            Self::AndExpression(ref and_expression) => {
                and_expression.accept(visitor);
            }
            Self::EqualityExpression(ref equality_expression) => {
                equality_expression.accept(visitor);
            }
            Self::InequalityExpression(ref inequality_expression) => {
                inequality_expression.accept(visitor);
            }
            Self::BitwiseOrExpression(ref bitwise_or_expression) => {
                bitwise_or_expression.accept(visitor);
            }
            Self::BitwiseXorExpression(ref bitwise_xor_expression) => {
                bitwise_xor_expression.accept(visitor);
            }
            Self::BitwiseAndExpression(ref bitwise_and_expression) => {
                bitwise_and_expression.accept(visitor);
            }
            Self::ShiftExpression(ref shift_expression) => {
                shift_expression.accept(visitor);
            }
            Self::AdditiveExpression(ref additive_expression) => {
                additive_expression.accept(visitor);
            }
            Self::MultiplicativeExpression(ref multiplicative_expression) => {
                multiplicative_expression.accept(visitor);
            }
            Self::ExponentiationExpression(ref exponentiation_expression) => {
                exponentiation_expression.accept(visitor);
            }
            Self::PostfixExpression(ref postfix_expression) => {
                postfix_expression.accept(visitor);
            }
            Self::PrefixExpression(ref prefix_expression) => {
                prefix_expression.accept(visitor);
            }
            Self::FunctionCallExpression(ref function_call_expression) => {
                function_call_expression.accept(visitor);
            }
            Self::CallOptionsExpression(ref call_options_expression) => {
                call_options_expression.accept(visitor);
            }
            Self::MemberAccessExpression(ref member_access_expression) => {
                member_access_expression.accept(visitor);
            }
            Self::IndexAccessExpression(ref index_access_expression) => {
                index_access_expression.accept(visitor);
            }
            Self::NewExpression(ref new_expression) => {
                new_expression.accept(visitor);
            }
            Self::TupleExpression(ref tuple_expression) => {
                tuple_expression.accept(visitor);
            }
            Self::TypeExpression(ref type_expression) => {
                type_expression.accept(visitor);
            }
            Self::ArrayExpression(ref array_expression) => {
                array_expression.accept(visitor);
            }
            Self::HexNumberExpression(ref hex_number_expression) => {
                hex_number_expression.accept(visitor);
            }
            Self::DecimalNumberExpression(ref decimal_number_expression) => {
                decimal_number_expression.accept(visitor);
            }
            Self::StringExpression(ref string_expression) => {
                string_expression.accept(visitor);
            }
            Self::ElementaryType(ref elementary_type) => {
                elementary_type.accept(visitor);
            }
            Self::PayableKeyword
            | Self::ThisKeyword
            | Self::SuperKeyword
            | Self::TrueKeyword
            | Self::FalseKeyword
            | Self::Identifier(_) => {}
        }
    }
}

impl ArgumentsDeclaration {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::PositionalArgumentsDeclaration(ref positional_arguments_declaration) => {
                positional_arguments_declaration.accept(visitor);
            }
            Self::NamedArgumentsDeclaration(ref named_arguments_declaration) => {
                named_arguments_declaration.accept(visitor);
            }
        }
    }
}

impl NumberUnit {
    pub fn accept(&self, _visitor: &mut dyn Visitor) {}
}

impl StringExpression {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::StringLiteral(ref string_literal) => {
                string_literal.accept(visitor);
            }
            Self::StringLiterals(ref string_literals) => {
                accept_string_literals(string_literals, visitor);
            }
            Self::HexStringLiteral(ref hex_string_literal) => {
                hex_string_literal.accept(visitor);
            }
            Self::HexStringLiterals(ref hex_string_literals) => {
                accept_hex_string_literals(hex_string_literals, visitor);
            }
            Self::UnicodeStringLiterals(ref unicode_string_literals) => {
                accept_unicode_string_literals(unicode_string_literals, visitor);
            }
        }
    }
}

impl StringLiteral {
    pub fn accept(&self, _visitor: &mut dyn Visitor) {}
}

impl HexStringLiteral {
    pub fn accept(&self, _visitor: &mut dyn Visitor) {}
}

impl UnicodeStringLiteral {
    pub fn accept(&self, _visitor: &mut dyn Visitor) {}
}

impl YulStatement {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::YulBlock(ref yul_block) => {
                yul_block.accept(visitor);
            }
            Self::YulFunctionDefinition(ref yul_function_definition) => {
                yul_function_definition.accept(visitor);
            }
            Self::YulStackAssignmentStatement(ref yul_stack_assignment_statement) => {
                yul_stack_assignment_statement.accept(visitor);
            }
            Self::YulIfStatement(ref yul_if_statement) => {
                yul_if_statement.accept(visitor);
            }
            Self::YulForStatement(ref yul_for_statement) => {
                yul_for_statement.accept(visitor);
            }
            Self::YulSwitchStatement(ref yul_switch_statement) => {
                yul_switch_statement.accept(visitor);
            }
            Self::YulLeaveStatement(ref yul_leave_statement) => {
                yul_leave_statement.accept(visitor);
            }
            Self::YulBreakStatement(ref yul_break_statement) => {
                yul_break_statement.accept(visitor);
            }
            Self::YulContinueStatement(ref yul_continue_statement) => {
                yul_continue_statement.accept(visitor);
            }
            Self::YulVariableAssignmentStatement(ref yul_variable_assignment_statement) => {
                yul_variable_assignment_statement.accept(visitor);
            }
            Self::YulLabel(ref yul_label) => {
                yul_label.accept(visitor);
            }
            Self::YulVariableDeclarationStatement(ref yul_variable_declaration_statement) => {
                yul_variable_declaration_statement.accept(visitor);
            }
            Self::YulExpression(ref yul_expression) => {
                yul_expression.accept(visitor);
            }
        }
    }
}

impl YulAssignmentOperator {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::YulColonAndEqual(ref yul_colon_and_equal) => {
                yul_colon_and_equal.accept(visitor);
            }
            Self::ColonEqual => {}
        }
    }
}

impl YulStackAssignmentOperator {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::YulEqualAndColon(ref yul_equal_and_colon) => {
                yul_equal_and_colon.accept(visitor);
            }
            Self::EqualColon => {}
        }
    }
}

impl YulSwitchCase {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::YulDefaultCase(ref yul_default_case) => {
                yul_default_case.accept(visitor);
            }
            Self::YulValueCase(ref yul_value_case) => {
                yul_value_case.accept(visitor);
            }
        }
    }
}

impl YulExpression {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::YulFunctionCallExpression(ref yul_function_call_expression) => {
                yul_function_call_expression.accept(visitor);
            }
            Self::YulLiteral(ref yul_literal) => {
                yul_literal.accept(visitor);
            }
            Self::YulPath(ref yul_path) => {
                accept_yul_path(yul_path, visitor);
            }
        }
    }
}

impl YulLiteral {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::HexStringLiteral(ref hex_string_literal) => {
                hex_string_literal.accept(visitor);
            }
            Self::StringLiteral(ref string_literal) => {
                string_literal.accept(visitor);
            }
            Self::YulTrueKeyword
            | Self::YulFalseKeyword
            | Self::YulDecimalLiteral(_)
            | Self::YulHexLiteral(_) => {}
        }
    }
}

//
// Repeated:
//

#[inline]
fn accept_source_unit_members(items: &[SourceUnitMember], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_version_expression_set(items: &[VersionExpression], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_contract_members(items: &[ContractMember], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_interface_members(items: &[ContractMember], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_library_members(items: &[ContractMember], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_struct_members(items: &[StructMember], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_state_variable_attributes(items: &[StateVariableAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_function_attributes(items: &[FunctionAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_constructor_attributes(items: &[ConstructorAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_fallback_function_attributes(
    items: &[FallbackFunctionAttribute],
    visitor: &mut dyn Visitor,
) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_receive_function_attributes(
    items: &[ReceiveFunctionAttribute],
    visitor: &mut dyn Visitor,
) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_modifier_attributes(items: &[ModifierAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_function_type_attributes(items: &[FunctionTypeAttribute], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_statements(items: &[Statement], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_catch_clauses(items: &[CatchClause], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_string_literals(items: &[StringLiteral], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_hex_string_literals(items: &[HexStringLiteral], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_unicode_string_literals(items: &[UnicodeStringLiteral], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_yul_statements(items: &[YulStatement], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_yul_switch_cases(items: &[YulSwitchCase], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
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
        item.accept(visitor);
    }
}

#[inline]
fn accept_using_deconstruction_symbols(
    items: &[UsingDeconstructionSymbol],
    visitor: &mut dyn Visitor,
) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_inheritance_types(items: &[InheritanceType], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_enum_members(_items: &[Rc<TerminalNode>], _visitor: &mut dyn Visitor) {}

#[inline]
fn accept_parameters(items: &[Parameter], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
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
        item.accept(visitor);
    }
}

#[inline]
fn accept_error_parameters(items: &[ErrorParameter], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_assembly_flags(items: &[StringLiteral], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_tuple_deconstruction_elements(
    items: &[TupleDeconstructionElement],
    visitor: &mut dyn Visitor,
) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_positional_arguments(items: &[Expression], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_named_arguments(items: &[NamedArgument], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_call_options(items: &[NamedArgument], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_tuple_values(items: &[TupleValue], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_array_values(items: &[Expression], visitor: &mut dyn Visitor) {
    for item in items {
        item.accept(visitor);
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
        item.accept(visitor);
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
