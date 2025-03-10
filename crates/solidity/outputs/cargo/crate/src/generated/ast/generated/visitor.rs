// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::TerminalNode;

pub trait Visitor {
    fn visit_source_unit(&mut self, _target: &SourceUnit) {}
    fn visit_pragma_directive(&mut self, _target: &PragmaDirective) {}
    fn visit_abicoder_pragma(&mut self, _target: &AbicoderPragma) {}
    fn visit_experimental_pragma(&mut self, _target: &ExperimentalPragma) {}
    fn visit_version_pragma(&mut self, _target: &VersionPragma) {}
    fn visit_version_range(&mut self, _target: &VersionRange) {}
    fn visit_version_term(&mut self, _target: &VersionTerm) {}
    fn visit_import_directive(&mut self, _target: &ImportDirective) {}
    fn visit_path_import(&mut self, _target: &PathImport) {}
    fn visit_named_import(&mut self, _target: &NamedImport) {}
    fn visit_import_deconstruction(&mut self, _target: &ImportDeconstruction) {}
    fn visit_import_deconstruction_symbol(&mut self, _target: &ImportDeconstructionSymbol) {}
    fn visit_import_alias(&mut self, _target: &ImportAlias) {}
    fn visit_using_directive(&mut self, _target: &UsingDirective) {}
    fn visit_using_deconstruction(&mut self, _target: &UsingDeconstruction) {}
    fn visit_using_deconstruction_symbol(&mut self, _target: &UsingDeconstructionSymbol) {}
    fn visit_using_alias(&mut self, _target: &UsingAlias) {}
    fn visit_contract_definition(&mut self, _target: &ContractDefinition) {}
    fn visit_inheritance_specifier(&mut self, _target: &InheritanceSpecifier) {}
    fn visit_inheritance_type(&mut self, _target: &InheritanceType) {}
    fn visit_interface_definition(&mut self, _target: &InterfaceDefinition) {}
    fn visit_library_definition(&mut self, _target: &LibraryDefinition) {}
    fn visit_struct_definition(&mut self, _target: &StructDefinition) {}
    fn visit_struct_member(&mut self, _target: &StructMember) {}
    fn visit_enum_definition(&mut self, _target: &EnumDefinition) {}
    fn visit_constant_definition(&mut self, _target: &ConstantDefinition) {}
    fn visit_state_variable_definition(&mut self, _target: &StateVariableDefinition) {}
    fn visit_state_variable_definition_value(&mut self, _target: &StateVariableDefinitionValue) {}
    fn visit_function_definition(&mut self, _target: &FunctionDefinition) {}
    fn visit_parameters_declaration(&mut self, _target: &ParametersDeclaration) {}
    fn visit_parameter(&mut self, _target: &Parameter) {}
    fn visit_override_specifier(&mut self, _target: &OverrideSpecifier) {}
    fn visit_override_paths_declaration(&mut self, _target: &OverridePathsDeclaration) {}
    fn visit_returns_declaration(&mut self, _target: &ReturnsDeclaration) {}
    fn visit_constructor_definition(&mut self, _target: &ConstructorDefinition) {}
    fn visit_unnamed_function_definition(&mut self, _target: &UnnamedFunctionDefinition) {}
    fn visit_fallback_function_definition(&mut self, _target: &FallbackFunctionDefinition) {}
    fn visit_receive_function_definition(&mut self, _target: &ReceiveFunctionDefinition) {}
    fn visit_modifier_definition(&mut self, _target: &ModifierDefinition) {}
    fn visit_modifier_invocation(&mut self, _target: &ModifierInvocation) {}
    fn visit_event_definition(&mut self, _target: &EventDefinition) {}
    fn visit_event_parameters_declaration(&mut self, _target: &EventParametersDeclaration) {}
    fn visit_event_parameter(&mut self, _target: &EventParameter) {}
    fn visit_user_defined_value_type_definition(
        &mut self,
        _target: &UserDefinedValueTypeDefinition,
    ) {
    }
    fn visit_error_definition(&mut self, _target: &ErrorDefinition) {}
    fn visit_error_parameters_declaration(&mut self, _target: &ErrorParametersDeclaration) {}
    fn visit_error_parameter(&mut self, _target: &ErrorParameter) {}
    fn visit_array_type_name(&mut self, _target: &ArrayTypeName) {}
    fn visit_function_type(&mut self, _target: &FunctionType) {}
    fn visit_mapping_type(&mut self, _target: &MappingType) {}
    fn visit_mapping_key(&mut self, _target: &MappingKey) {}
    fn visit_mapping_value(&mut self, _target: &MappingValue) {}
    fn visit_address_type(&mut self, _target: &AddressType) {}
    fn visit_block(&mut self, _target: &Block) {}
    fn visit_unchecked_block(&mut self, _target: &UncheckedBlock) {}
    fn visit_expression_statement(&mut self, _target: &ExpressionStatement) {}
    fn visit_assembly_statement(&mut self, _target: &AssemblyStatement) {}
    fn visit_assembly_flags_declaration(&mut self, _target: &AssemblyFlagsDeclaration) {}
    fn visit_tuple_deconstruction_statement(&mut self, _target: &TupleDeconstructionStatement) {}
    fn visit_tuple_deconstruction_element(&mut self, _target: &TupleDeconstructionElement) {}
    fn visit_typed_tuple_member(&mut self, _target: &TypedTupleMember) {}
    fn visit_untyped_tuple_member(&mut self, _target: &UntypedTupleMember) {}
    fn visit_variable_declaration_statement(&mut self, _target: &VariableDeclarationStatement) {}
    fn visit_variable_declaration_value(&mut self, _target: &VariableDeclarationValue) {}
    fn visit_if_statement(&mut self, _target: &IfStatement) {}
    fn visit_else_branch(&mut self, _target: &ElseBranch) {}
    fn visit_for_statement(&mut self, _target: &ForStatement) {}
    fn visit_while_statement(&mut self, _target: &WhileStatement) {}
    fn visit_do_while_statement(&mut self, _target: &DoWhileStatement) {}
    fn visit_continue_statement(&mut self, _target: &ContinueStatement) {}
    fn visit_break_statement(&mut self, _target: &BreakStatement) {}
    fn visit_return_statement(&mut self, _target: &ReturnStatement) {}
    fn visit_emit_statement(&mut self, _target: &EmitStatement) {}
    fn visit_try_statement(&mut self, _target: &TryStatement) {}
    fn visit_catch_clause(&mut self, _target: &CatchClause) {}
    fn visit_catch_clause_error(&mut self, _target: &CatchClauseError) {}
    fn visit_revert_statement(&mut self, _target: &RevertStatement) {}
    fn visit_throw_statement(&mut self, _target: &ThrowStatement) {}
    fn visit_assignment_expression(&mut self, _target: &AssignmentExpression) {}
    fn visit_conditional_expression(&mut self, _target: &ConditionalExpression) {}
    fn visit_or_expression(&mut self, _target: &OrExpression) {}
    fn visit_and_expression(&mut self, _target: &AndExpression) {}
    fn visit_equality_expression(&mut self, _target: &EqualityExpression) {}
    fn visit_inequality_expression(&mut self, _target: &InequalityExpression) {}
    fn visit_bitwise_or_expression(&mut self, _target: &BitwiseOrExpression) {}
    fn visit_bitwise_xor_expression(&mut self, _target: &BitwiseXorExpression) {}
    fn visit_bitwise_and_expression(&mut self, _target: &BitwiseAndExpression) {}
    fn visit_shift_expression(&mut self, _target: &ShiftExpression) {}
    fn visit_additive_expression(&mut self, _target: &AdditiveExpression) {}
    fn visit_multiplicative_expression(&mut self, _target: &MultiplicativeExpression) {}
    fn visit_exponentiation_expression(&mut self, _target: &ExponentiationExpression) {}
    fn visit_postfix_expression(&mut self, _target: &PostfixExpression) {}
    fn visit_prefix_expression(&mut self, _target: &PrefixExpression) {}
    fn visit_function_call_expression(&mut self, _target: &FunctionCallExpression) {}
    fn visit_call_options_expression(&mut self, _target: &CallOptionsExpression) {}
    fn visit_member_access_expression(&mut self, _target: &MemberAccessExpression) {}
    fn visit_index_access_expression(&mut self, _target: &IndexAccessExpression) {}
    fn visit_index_access_end(&mut self, _target: &IndexAccessEnd) {}
    fn visit_positional_arguments_declaration(&mut self, _target: &PositionalArgumentsDeclaration) {
    }
    fn visit_named_arguments_declaration(&mut self, _target: &NamedArgumentsDeclaration) {}
    fn visit_named_argument_group(&mut self, _target: &NamedArgumentGroup) {}
    fn visit_named_argument(&mut self, _target: &NamedArgument) {}
    fn visit_type_expression(&mut self, _target: &TypeExpression) {}
    fn visit_new_expression(&mut self, _target: &NewExpression) {}
    fn visit_tuple_expression(&mut self, _target: &TupleExpression) {}
    fn visit_tuple_value(&mut self, _target: &TupleValue) {}
    fn visit_array_expression(&mut self, _target: &ArrayExpression) {}
    fn visit_hex_number_expression(&mut self, _target: &HexNumberExpression) {}
    fn visit_decimal_number_expression(&mut self, _target: &DecimalNumberExpression) {}
    fn visit_yul_block(&mut self, _target: &YulBlock) {}
    fn visit_yul_function_definition(&mut self, _target: &YulFunctionDefinition) {}
    fn visit_yul_parameters_declaration(&mut self, _target: &YulParametersDeclaration) {}
    fn visit_yul_returns_declaration(&mut self, _target: &YulReturnsDeclaration) {}
    fn visit_yul_variable_declaration_statement(
        &mut self,
        _target: &YulVariableDeclarationStatement,
    ) {
    }
    fn visit_yul_variable_declaration_value(&mut self, _target: &YulVariableDeclarationValue) {}
    fn visit_yul_variable_assignment_statement(
        &mut self,
        _target: &YulVariableAssignmentStatement,
    ) {
    }
    fn visit_yul_colon_and_equal(&mut self, _target: &YulColonAndEqual) {}
    fn visit_yul_stack_assignment_statement(&mut self, _target: &YulStackAssignmentStatement) {}
    fn visit_yul_equal_and_colon(&mut self, _target: &YulEqualAndColon) {}
    fn visit_yul_if_statement(&mut self, _target: &YulIfStatement) {}
    fn visit_yul_for_statement(&mut self, _target: &YulForStatement) {}
    fn visit_yul_switch_statement(&mut self, _target: &YulSwitchStatement) {}
    fn visit_yul_default_case(&mut self, _target: &YulDefaultCase) {}
    fn visit_yul_value_case(&mut self, _target: &YulValueCase) {}
    fn visit_yul_leave_statement(&mut self, _target: &YulLeaveStatement) {}
    fn visit_yul_break_statement(&mut self, _target: &YulBreakStatement) {}
    fn visit_yul_continue_statement(&mut self, _target: &YulContinueStatement) {}
    fn visit_yul_label(&mut self, _target: &YulLabel) {}
    fn visit_yul_function_call_expression(&mut self, _target: &YulFunctionCallExpression) {}
}

//
// Sequences:
//

impl SourceUnitStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_source_unit_members(&self.members, visitor);
        visitor.visit_source_unit(self);
    }
}

impl PragmaDirectiveStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.pragma.accept(visitor);
        visitor.visit_pragma_directive(self);
    }
}

impl AbicoderPragmaStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_abicoder_pragma(self);
    }
}

impl ExperimentalPragmaStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.feature.accept(visitor);
        visitor.visit_experimental_pragma(self);
    }
}

impl VersionPragmaStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_version_expression_sets(&self.sets, visitor);
        visitor.visit_version_pragma(self);
    }
}

impl VersionRangeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.start.accept(visitor);

        self.end.accept(visitor);
        visitor.visit_version_range(self);
    }
}

impl VersionTermStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref operator) = self.operator {
            operator.accept(visitor);
        }

        self.literal.accept(visitor);
        visitor.visit_version_term(self);
    }
}

impl ImportDirectiveStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.clause.accept(visitor);
        visitor.visit_import_directive(self);
    }
}

impl PathImportStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.path.accept(visitor);

        if let Some(ref alias) = self.alias {
            alias.accept(visitor);
        }
        visitor.visit_path_import(self);
    }
}

impl NamedImportStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.alias.accept(visitor);

        self.path.accept(visitor);
        visitor.visit_named_import(self);
    }
}

impl ImportDeconstructionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_import_deconstruction_symbols(&self.symbols, visitor);

        self.path.accept(visitor);
        visitor.visit_import_deconstruction(self);
    }
}

impl ImportDeconstructionSymbolStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref alias) = self.alias {
            alias.accept(visitor);
        }
        visitor.visit_import_deconstruction_symbol(self);
    }
}

impl ImportAliasStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_import_alias(self);
    }
}

impl UsingDirectiveStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.clause.accept(visitor);

        self.target.accept(visitor);
        visitor.visit_using_directive(self);
    }
}

impl UsingDeconstructionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_using_deconstruction_symbols(&self.symbols, visitor);
        visitor.visit_using_deconstruction(self);
    }
}

impl UsingDeconstructionSymbolStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_identifier_path(&self.name, visitor);

        if let Some(ref alias) = self.alias {
            alias.accept(visitor);
        }
        visitor.visit_using_deconstruction_symbol(self);
    }
}

impl UsingAliasStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operator.accept(visitor);
        visitor.visit_using_alias(self);
    }
}

impl ContractDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref inheritance) = self.inheritance {
            inheritance.accept(visitor);
        }

        accept_contract_members(&self.members, visitor);
        visitor.visit_contract_definition(self);
    }
}

impl InheritanceSpecifierStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_inheritance_types(&self.types, visitor);
        visitor.visit_inheritance_specifier(self);
    }
}

impl InheritanceTypeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_identifier_path(&self.type_name, visitor);

        if let Some(ref arguments) = self.arguments {
            arguments.accept(visitor);
        }
        visitor.visit_inheritance_type(self);
    }
}

impl InterfaceDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref inheritance) = self.inheritance {
            inheritance.accept(visitor);
        }

        accept_interface_members(&self.members, visitor);
        visitor.visit_interface_definition(self);
    }
}

impl LibraryDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_library_members(&self.members, visitor);
        visitor.visit_library_definition(self);
    }
}

impl StructDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_struct_members(&self.members, visitor);
        visitor.visit_struct_definition(self);
    }
}

impl StructMemberStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);
        visitor.visit_struct_member(self);
    }
}

impl EnumDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_enum_members(&self.members, visitor);
        visitor.visit_enum_definition(self);
    }
}

impl ConstantDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);

        self.value.accept(visitor);
        visitor.visit_constant_definition(self);
    }
}

impl StateVariableDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);

        accept_state_variable_attributes(&self.attributes, visitor);

        if let Some(ref value) = self.value {
            value.accept(visitor);
        }
        visitor.visit_state_variable_definition(self);
    }
}

impl StateVariableDefinitionValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.value.accept(visitor);
        visitor.visit_state_variable_definition_value(self);
    }
}

impl FunctionDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.name.accept(visitor);

        self.parameters.accept(visitor);

        accept_function_attributes(&self.attributes, visitor);

        if let Some(ref returns) = self.returns {
            returns.accept(visitor);
        }

        self.body.accept(visitor);
        visitor.visit_function_definition(self);
    }
}

impl ParametersDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_parameters(&self.parameters, visitor);
        visitor.visit_parameters_declaration(self);
    }
}

impl ParameterStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);

        if let Some(ref storage_location) = self.storage_location {
            storage_location.accept(visitor);
        }
        visitor.visit_parameter(self);
    }
}

impl OverrideSpecifierStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref overridden) = self.overridden {
            overridden.accept(visitor);
        }
        visitor.visit_override_specifier(self);
    }
}

impl OverridePathsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_override_paths(&self.paths, visitor);
        visitor.visit_override_paths_declaration(self);
    }
}

impl ReturnsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.variables.accept(visitor);
        visitor.visit_returns_declaration(self);
    }
}

impl ConstructorDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.parameters.accept(visitor);

        accept_constructor_attributes(&self.attributes, visitor);

        self.body.accept(visitor);
        visitor.visit_constructor_definition(self);
    }
}

impl UnnamedFunctionDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.parameters.accept(visitor);

        accept_unnamed_function_attributes(&self.attributes, visitor);

        self.body.accept(visitor);
        visitor.visit_unnamed_function_definition(self);
    }
}

impl FallbackFunctionDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.parameters.accept(visitor);

        accept_fallback_function_attributes(&self.attributes, visitor);

        if let Some(ref returns) = self.returns {
            returns.accept(visitor);
        }

        self.body.accept(visitor);
        visitor.visit_fallback_function_definition(self);
    }
}

impl ReceiveFunctionDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.parameters.accept(visitor);

        accept_receive_function_attributes(&self.attributes, visitor);

        self.body.accept(visitor);
        visitor.visit_receive_function_definition(self);
    }
}

impl ModifierDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref parameters) = self.parameters {
            parameters.accept(visitor);
        }

        accept_modifier_attributes(&self.attributes, visitor);

        self.body.accept(visitor);
        visitor.visit_modifier_definition(self);
    }
}

impl ModifierInvocationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_identifier_path(&self.name, visitor);

        if let Some(ref arguments) = self.arguments {
            arguments.accept(visitor);
        }
        visitor.visit_modifier_invocation(self);
    }
}

impl EventDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.parameters.accept(visitor);
        visitor.visit_event_definition(self);
    }
}

impl EventParametersDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_event_parameters(&self.parameters, visitor);
        visitor.visit_event_parameters_declaration(self);
    }
}

impl EventParameterStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);
        visitor.visit_event_parameter(self);
    }
}

impl UserDefinedValueTypeDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.value_type.accept(visitor);
        visitor.visit_user_defined_value_type_definition(self);
    }
}

impl ErrorDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.members.accept(visitor);
        visitor.visit_error_definition(self);
    }
}

impl ErrorParametersDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_error_parameters(&self.parameters, visitor);
        visitor.visit_error_parameters_declaration(self);
    }
}

impl ErrorParameterStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);
        visitor.visit_error_parameter(self);
    }
}

impl ArrayTypeNameStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operand.accept(visitor);

        if let Some(ref index) = self.index {
            index.accept(visitor);
        }
        visitor.visit_array_type_name(self);
    }
}

impl FunctionTypeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.parameters.accept(visitor);

        accept_function_type_attributes(&self.attributes, visitor);

        if let Some(ref returns) = self.returns {
            returns.accept(visitor);
        }
        visitor.visit_function_type(self);
    }
}

impl MappingTypeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.key_type.accept(visitor);

        self.value_type.accept(visitor);
        visitor.visit_mapping_type(self);
    }
}

impl MappingKeyStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.key_type.accept(visitor);
        visitor.visit_mapping_key(self);
    }
}

impl MappingValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);
        visitor.visit_mapping_value(self);
    }
}

impl AddressTypeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_address_type(self);
    }
}

impl BlockStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_statements(&self.statements, visitor);
        visitor.visit_block(self);
    }
}

impl UncheckedBlockStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.block.accept(visitor);
        visitor.visit_unchecked_block(self);
    }
}

impl ExpressionStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.expression.accept(visitor);
        visitor.visit_expression_statement(self);
    }
}

impl AssemblyStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref label) = self.label {
            label.accept(visitor);
        }

        if let Some(ref flags) = self.flags {
            flags.accept(visitor);
        }

        self.body.accept(visitor);
        visitor.visit_assembly_statement(self);
    }
}

impl AssemblyFlagsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_assembly_flags(&self.flags, visitor);
        visitor.visit_assembly_flags_declaration(self);
    }
}

impl TupleDeconstructionStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_tuple_deconstruction_elements(&self.elements, visitor);

        self.expression.accept(visitor);
        visitor.visit_tuple_deconstruction_statement(self);
    }
}

impl TupleDeconstructionElementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref member) = self.member {
            member.accept(visitor);
        }
        visitor.visit_tuple_deconstruction_element(self);
    }
}

impl TypedTupleMemberStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);

        if let Some(ref storage_location) = self.storage_location {
            storage_location.accept(visitor);
        }
        visitor.visit_typed_tuple_member(self);
    }
}

impl UntypedTupleMemberStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref storage_location) = self.storage_location {
            storage_location.accept(visitor);
        }
        visitor.visit_untyped_tuple_member(self);
    }
}

impl VariableDeclarationStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.variable_type.accept(visitor);

        if let Some(ref storage_location) = self.storage_location {
            storage_location.accept(visitor);
        }

        if let Some(ref value) = self.value {
            value.accept(visitor);
        }
        visitor.visit_variable_declaration_statement(self);
    }
}

impl VariableDeclarationValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.expression.accept(visitor);
        visitor.visit_variable_declaration_value(self);
    }
}

impl IfStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.condition.accept(visitor);

        self.body.accept(visitor);

        if let Some(ref else_branch) = self.else_branch {
            else_branch.accept(visitor);
        }
        visitor.visit_if_statement(self);
    }
}

impl ElseBranchStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.body.accept(visitor);
        visitor.visit_else_branch(self);
    }
}

impl ForStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.initialization.accept(visitor);

        self.condition.accept(visitor);

        if let Some(ref iterator) = self.iterator {
            iterator.accept(visitor);
        }

        self.body.accept(visitor);
        visitor.visit_for_statement(self);
    }
}

impl WhileStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.condition.accept(visitor);

        self.body.accept(visitor);
        visitor.visit_while_statement(self);
    }
}

impl DoWhileStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.body.accept(visitor);

        self.condition.accept(visitor);
        visitor.visit_do_while_statement(self);
    }
}

impl ContinueStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_continue_statement(self);
    }
}

impl BreakStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_break_statement(self);
    }
}

impl ReturnStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref expression) = self.expression {
            expression.accept(visitor);
        }
        visitor.visit_return_statement(self);
    }
}

impl EmitStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_identifier_path(&self.event, visitor);

        self.arguments.accept(visitor);
        visitor.visit_emit_statement(self);
    }
}

impl TryStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.expression.accept(visitor);

        if let Some(ref returns) = self.returns {
            returns.accept(visitor);
        }

        self.body.accept(visitor);

        accept_catch_clauses(&self.catch_clauses, visitor);
        visitor.visit_try_statement(self);
    }
}

impl CatchClauseStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref error) = self.error {
            error.accept(visitor);
        }

        self.body.accept(visitor);
        visitor.visit_catch_clause(self);
    }
}

impl CatchClauseErrorStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.parameters.accept(visitor);
        visitor.visit_catch_clause_error(self);
    }
}

impl RevertStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref error) = self.error {
            accept_identifier_path(error, visitor);
        }

        self.arguments.accept(visitor);
        visitor.visit_revert_statement(self);
    }
}

impl ThrowStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_throw_statement(self);
    }
}

impl AssignmentExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_assignment_expression(self);
    }
}

impl ConditionalExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operand.accept(visitor);

        self.true_expression.accept(visitor);

        self.false_expression.accept(visitor);
        visitor.visit_conditional_expression(self);
    }
}

impl OrExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_or_expression(self);
    }
}

impl AndExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_and_expression(self);
    }
}

impl EqualityExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_equality_expression(self);
    }
}

impl InequalityExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_inequality_expression(self);
    }
}

impl BitwiseOrExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_bitwise_or_expression(self);
    }
}

impl BitwiseXorExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_bitwise_xor_expression(self);
    }
}

impl BitwiseAndExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_bitwise_and_expression(self);
    }
}

impl ShiftExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_shift_expression(self);
    }
}

impl AdditiveExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_additive_expression(self);
    }
}

impl MultiplicativeExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_multiplicative_expression(self);
    }
}

impl ExponentiationExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_exponentiation_expression(self);
    }
}

impl PostfixExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operand.accept(visitor);
        visitor.visit_postfix_expression(self);
    }
}

impl PrefixExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operand.accept(visitor);
        visitor.visit_prefix_expression(self);
    }
}

impl FunctionCallExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operand.accept(visitor);

        self.arguments.accept(visitor);
        visitor.visit_function_call_expression(self);
    }
}

impl CallOptionsExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operand.accept(visitor);

        accept_call_options(&self.options, visitor);
        visitor.visit_call_options_expression(self);
    }
}

impl MemberAccessExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operand.accept(visitor);
        visitor.visit_member_access_expression(self);
    }
}

impl IndexAccessExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operand.accept(visitor);

        if let Some(ref start) = self.start {
            start.accept(visitor);
        }

        if let Some(ref end) = self.end {
            end.accept(visitor);
        }
        visitor.visit_index_access_expression(self);
    }
}

impl IndexAccessEndStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref end) = self.end {
            end.accept(visitor);
        }
        visitor.visit_index_access_end(self);
    }
}

impl PositionalArgumentsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_positional_arguments(&self.arguments, visitor);
        visitor.visit_positional_arguments_declaration(self);
    }
}

impl NamedArgumentsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref arguments) = self.arguments {
            arguments.accept(visitor);
        }
        visitor.visit_named_arguments_declaration(self);
    }
}

impl NamedArgumentGroupStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_named_arguments(&self.arguments, visitor);
        visitor.visit_named_argument_group(self);
    }
}

impl NamedArgumentStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.value.accept(visitor);
        visitor.visit_named_argument(self);
    }
}

impl TypeExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);
        visitor.visit_type_expression(self);
    }
}

impl NewExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.type_name.accept(visitor);
        visitor.visit_new_expression(self);
    }
}

impl TupleExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_tuple_values(&self.items, visitor);
        visitor.visit_tuple_expression(self);
    }
}

impl TupleValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref expression) = self.expression {
            expression.accept(visitor);
        }
        visitor.visit_tuple_value(self);
    }
}

impl ArrayExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_array_values(&self.items, visitor);
        visitor.visit_array_expression(self);
    }
}

impl HexNumberExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref unit) = self.unit {
            unit.accept(visitor);
        }
        visitor.visit_hex_number_expression(self);
    }
}

impl DecimalNumberExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        if let Some(ref unit) = self.unit {
            unit.accept(visitor);
        }
        visitor.visit_decimal_number_expression(self);
    }
}

impl YulBlockStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_yul_statements(&self.statements, visitor);
        visitor.visit_yul_block(self);
    }
}

impl YulFunctionDefinitionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.parameters.accept(visitor);

        if let Some(ref returns) = self.returns {
            returns.accept(visitor);
        }

        self.body.accept(visitor);
        visitor.visit_yul_function_definition(self);
    }
}

impl YulParametersDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_yul_parameters(&self.parameters, visitor);
        visitor.visit_yul_parameters_declaration(self);
    }
}

impl YulReturnsDeclarationStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_yul_variable_names(&self.variables, visitor);
        visitor.visit_yul_returns_declaration(self);
    }
}

impl YulVariableDeclarationStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_yul_variable_names(&self.variables, visitor);

        if let Some(ref value) = self.value {
            value.accept(visitor);
        }
        visitor.visit_yul_variable_declaration_statement(self);
    }
}

impl YulVariableDeclarationValueStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.assignment.accept(visitor);

        self.expression.accept(visitor);
        visitor.visit_yul_variable_declaration_value(self);
    }
}

impl YulVariableAssignmentStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        accept_yul_paths(&self.variables, visitor);

        self.assignment.accept(visitor);

        self.expression.accept(visitor);
        visitor.visit_yul_variable_assignment_statement(self);
    }
}

impl YulColonAndEqualStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_yul_colon_and_equal(self);
    }
}

impl YulStackAssignmentStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.assignment.accept(visitor);
        visitor.visit_yul_stack_assignment_statement(self);
    }
}

impl YulEqualAndColonStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_yul_equal_and_colon(self);
    }
}

impl YulIfStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.condition.accept(visitor);

        self.body.accept(visitor);
        visitor.visit_yul_if_statement(self);
    }
}

impl YulForStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.initialization.accept(visitor);

        self.condition.accept(visitor);

        self.iterator.accept(visitor);

        self.body.accept(visitor);
        visitor.visit_yul_for_statement(self);
    }
}

impl YulSwitchStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.expression.accept(visitor);

        accept_yul_switch_cases(&self.cases, visitor);
        visitor.visit_yul_switch_statement(self);
    }
}

impl YulDefaultCaseStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.body.accept(visitor);
        visitor.visit_yul_default_case(self);
    }
}

impl YulValueCaseStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.value.accept(visitor);

        self.body.accept(visitor);
        visitor.visit_yul_value_case(self);
    }
}

impl YulLeaveStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_yul_leave_statement(self);
    }
}

impl YulBreakStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_yul_break_statement(self);
    }
}

impl YulContinueStatementStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_yul_continue_statement(self);
    }
}

impl YulLabelStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        visitor.visit_yul_label(self);
    }
}

impl YulFunctionCallExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &mut dyn Visitor) {
        self.operand.accept(visitor);

        accept_yul_arguments(&self.arguments, visitor);
        visitor.visit_yul_function_call_expression(self);
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
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
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
            Self::UnnamedFunctionDefinition(ref unnamed_function_definition) => {
                unnamed_function_definition.accept(visitor);
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
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
        }
    }
}

impl FunctionBody {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::Block(ref block) => {
                block.accept(visitor);
            }
            Self::TerminalNode(_) => {}
        }
    }
}

impl ConstructorAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ModifierInvocation(ref modifier_invocation) => {
                modifier_invocation.accept(visitor);
            }
            Self::TerminalNode(_) => {}
        }
    }
}

impl UnnamedFunctionAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ModifierInvocation(ref modifier_invocation) => {
                modifier_invocation.accept(visitor);
            }
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
        }
    }
}

impl ModifierAttribute {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::OverrideSpecifier(ref override_specifier) => {
                override_specifier.accept(visitor);
            }
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
        }
    }
}

impl ForStatementCondition {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::ExpressionStatement(ref expression_statement) => {
                expression_statement.accept(visitor);
            }
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
        }
    }
}

impl YulStackAssignmentOperator {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::YulEqualAndColon(ref yul_equal_and_colon) => {
                yul_equal_and_colon.accept(visitor);
            }
            Self::TerminalNode(_) => {}
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
            Self::TerminalNode(_) => {}
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
fn accept_unnamed_function_attributes(
    items: &[UnnamedFunctionAttribute],
    visitor: &mut dyn Visitor,
) {
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
