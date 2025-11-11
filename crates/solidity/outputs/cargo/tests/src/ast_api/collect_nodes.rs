use std::collections::HashMap;
use std::rc::Rc;

use slang_solidity::backend::l2_flat_contracts::visitor::Visitor;
use slang_solidity::backend::l2_flat_contracts::*;
use slang_solidity::cst::NodeId;

#[derive(Default, Debug)]
pub(crate) struct NodesMap {
    pub(crate) map: HashMap<NodeId, Node>,
}

impl Visitor for NodesMap {
    fn enter_source_unit(&mut self, node: &SourceUnit) -> bool {
        self.map
            .insert(node.node_id, Node::SourceUnit(Rc::clone(node)));
        true
    }

    fn enter_pragma_directive(&mut self, node: &PragmaDirective) -> bool {
        self.map
            .insert(node.node_id, Node::PragmaDirective(Rc::clone(node)));
        true
    }

    fn enter_abicoder_pragma(&mut self, node: &AbicoderPragma) -> bool {
        self.map
            .insert(node.node_id, Node::AbicoderPragma(Rc::clone(node)));
        true
    }

    fn enter_experimental_pragma(&mut self, node: &ExperimentalPragma) -> bool {
        self.map
            .insert(node.node_id, Node::ExperimentalPragma(Rc::clone(node)));
        true
    }

    fn enter_version_pragma(&mut self, node: &VersionPragma) -> bool {
        self.map
            .insert(node.node_id, Node::VersionPragma(Rc::clone(node)));
        true
    }

    fn enter_version_range(&mut self, node: &VersionRange) -> bool {
        self.map
            .insert(node.node_id, Node::VersionRange(Rc::clone(node)));
        true
    }

    fn enter_version_term(&mut self, node: &VersionTerm) -> bool {
        self.map
            .insert(node.node_id, Node::VersionTerm(Rc::clone(node)));
        true
    }

    fn enter_import_directive(&mut self, node: &ImportDirective) -> bool {
        self.map
            .insert(node.node_id, Node::ImportDirective(Rc::clone(node)));
        true
    }

    fn enter_path_import(&mut self, node: &PathImport) -> bool {
        self.map
            .insert(node.node_id, Node::PathImport(Rc::clone(node)));
        true
    }

    fn enter_named_import(&mut self, node: &NamedImport) -> bool {
        self.map
            .insert(node.node_id, Node::NamedImport(Rc::clone(node)));
        true
    }

    fn enter_import_deconstruction(&mut self, node: &ImportDeconstruction) -> bool {
        self.map
            .insert(node.node_id, Node::ImportDeconstruction(Rc::clone(node)));
        true
    }

    fn enter_import_deconstruction_symbol(&mut self, node: &ImportDeconstructionSymbol) -> bool {
        self.map.insert(
            node.node_id,
            Node::ImportDeconstructionSymbol(Rc::clone(node)),
        );
        true
    }

    fn enter_import_alias(&mut self, node: &ImportAlias) -> bool {
        self.map
            .insert(node.node_id, Node::ImportAlias(Rc::clone(node)));
        true
    }

    fn enter_using_directive(&mut self, node: &UsingDirective) -> bool {
        self.map
            .insert(node.node_id, Node::UsingDirective(Rc::clone(node)));
        true
    }

    fn enter_using_deconstruction(&mut self, node: &UsingDeconstruction) -> bool {
        self.map
            .insert(node.node_id, Node::UsingDeconstruction(Rc::clone(node)));
        true
    }

    fn enter_using_deconstruction_symbol(&mut self, node: &UsingDeconstructionSymbol) -> bool {
        self.map.insert(
            node.node_id,
            Node::UsingDeconstructionSymbol(Rc::clone(node)),
        );
        true
    }

    fn enter_using_alias(&mut self, node: &UsingAlias) -> bool {
        self.map
            .insert(node.node_id, Node::UsingAlias(Rc::clone(node)));
        true
    }

    fn enter_contract_definition(&mut self, node: &ContractDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::ContractDefinition(Rc::clone(node)));
        true
    }

    fn enter_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> bool {
        self.map
            .insert(node.node_id, Node::InheritanceSpecifier(Rc::clone(node)));
        true
    }

    fn enter_inheritance_type(&mut self, node: &InheritanceType) -> bool {
        self.map
            .insert(node.node_id, Node::InheritanceType(Rc::clone(node)));
        true
    }

    fn enter_storage_layout_specifier(&mut self, node: &StorageLayoutSpecifier) -> bool {
        self.map
            .insert(node.node_id, Node::StorageLayoutSpecifier(Rc::clone(node)));
        true
    }

    fn enter_interface_definition(&mut self, node: &InterfaceDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::InterfaceDefinition(Rc::clone(node)));
        true
    }

    fn enter_library_definition(&mut self, node: &LibraryDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::LibraryDefinition(Rc::clone(node)));
        true
    }

    fn enter_struct_definition(&mut self, node: &StructDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::StructDefinition(Rc::clone(node)));
        true
    }

    fn enter_struct_member(&mut self, node: &StructMember) -> bool {
        self.map
            .insert(node.node_id, Node::StructMember(Rc::clone(node)));
        true
    }

    fn enter_enum_definition(&mut self, node: &EnumDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::EnumDefinition(Rc::clone(node)));
        true
    }

    fn enter_constant_definition(&mut self, node: &ConstantDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::ConstantDefinition(Rc::clone(node)));
        true
    }

    fn enter_state_variable_definition(&mut self, node: &StateVariableDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::StateVariableDefinition(Rc::clone(node)));
        true
    }

    fn enter_state_variable_definition_value(
        &mut self,
        node: &StateVariableDefinitionValue,
    ) -> bool {
        self.map.insert(
            node.node_id,
            Node::StateVariableDefinitionValue(Rc::clone(node)),
        );
        true
    }

    fn enter_function_definition(&mut self, node: &FunctionDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::FunctionDefinition(Rc::clone(node)));
        true
    }

    fn enter_parameters_declaration(&mut self, node: &ParametersDeclaration) -> bool {
        self.map
            .insert(node.node_id, Node::ParametersDeclaration(Rc::clone(node)));
        true
    }

    fn enter_parameter(&mut self, node: &Parameter) -> bool {
        self.map
            .insert(node.node_id, Node::Parameter(Rc::clone(node)));
        true
    }

    fn enter_override_specifier(&mut self, node: &OverrideSpecifier) -> bool {
        self.map
            .insert(node.node_id, Node::OverrideSpecifier(Rc::clone(node)));
        true
    }

    fn enter_override_paths_declaration(&mut self, node: &OverridePathsDeclaration) -> bool {
        self.map.insert(
            node.node_id,
            Node::OverridePathsDeclaration(Rc::clone(node)),
        );
        true
    }

    fn enter_returns_declaration(&mut self, node: &ReturnsDeclaration) -> bool {
        self.map
            .insert(node.node_id, Node::ReturnsDeclaration(Rc::clone(node)));
        true
    }

    fn enter_constructor_definition(&mut self, node: &ConstructorDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::ConstructorDefinition(Rc::clone(node)));
        true
    }

    fn enter_unnamed_function_definition(&mut self, node: &UnnamedFunctionDefinition) -> bool {
        self.map.insert(
            node.node_id,
            Node::UnnamedFunctionDefinition(Rc::clone(node)),
        );
        true
    }

    fn enter_fallback_function_definition(&mut self, node: &FallbackFunctionDefinition) -> bool {
        self.map.insert(
            node.node_id,
            Node::FallbackFunctionDefinition(Rc::clone(node)),
        );
        true
    }

    fn enter_receive_function_definition(&mut self, node: &ReceiveFunctionDefinition) -> bool {
        self.map.insert(
            node.node_id,
            Node::ReceiveFunctionDefinition(Rc::clone(node)),
        );
        true
    }

    fn enter_modifier_definition(&mut self, node: &ModifierDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::ModifierDefinition(Rc::clone(node)));
        true
    }

    fn enter_modifier_invocation(&mut self, node: &ModifierInvocation) -> bool {
        self.map
            .insert(node.node_id, Node::ModifierInvocation(Rc::clone(node)));
        true
    }

    fn enter_event_definition(&mut self, node: &EventDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::EventDefinition(Rc::clone(node)));
        true
    }

    fn enter_event_parameters_declaration(&mut self, node: &EventParametersDeclaration) -> bool {
        self.map.insert(
            node.node_id,
            Node::EventParametersDeclaration(Rc::clone(node)),
        );
        true
    }

    fn enter_event_parameter(&mut self, node: &EventParameter) -> bool {
        self.map
            .insert(node.node_id, Node::EventParameter(Rc::clone(node)));
        true
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> bool {
        self.map.insert(
            node.node_id,
            Node::UserDefinedValueTypeDefinition(Rc::clone(node)),
        );
        true
    }

    fn enter_error_definition(&mut self, node: &ErrorDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::ErrorDefinition(Rc::clone(node)));
        true
    }

    fn enter_error_parameters_declaration(&mut self, node: &ErrorParametersDeclaration) -> bool {
        self.map.insert(
            node.node_id,
            Node::ErrorParametersDeclaration(Rc::clone(node)),
        );
        true
    }

    fn enter_error_parameter(&mut self, node: &ErrorParameter) -> bool {
        self.map
            .insert(node.node_id, Node::ErrorParameter(Rc::clone(node)));
        true
    }

    fn enter_array_type_name(&mut self, node: &ArrayTypeName) -> bool {
        self.map
            .insert(node.node_id, Node::ArrayTypeName(Rc::clone(node)));
        true
    }

    fn enter_function_type(&mut self, node: &FunctionType) -> bool {
        self.map
            .insert(node.node_id, Node::FunctionType(Rc::clone(node)));
        true
    }

    fn enter_mapping_type(&mut self, node: &MappingType) -> bool {
        self.map
            .insert(node.node_id, Node::MappingType(Rc::clone(node)));
        true
    }

    fn enter_mapping_key(&mut self, node: &MappingKey) -> bool {
        self.map
            .insert(node.node_id, Node::MappingKey(Rc::clone(node)));
        true
    }

    fn enter_mapping_value(&mut self, node: &MappingValue) -> bool {
        self.map
            .insert(node.node_id, Node::MappingValue(Rc::clone(node)));
        true
    }

    fn enter_address_type(&mut self, node: &AddressType) -> bool {
        self.map
            .insert(node.node_id, Node::AddressType(Rc::clone(node)));
        true
    }

    fn enter_block(&mut self, node: &Block) -> bool {
        self.map.insert(node.node_id, Node::Block(Rc::clone(node)));
        true
    }

    fn enter_unchecked_block(&mut self, node: &UncheckedBlock) -> bool {
        self.map
            .insert(node.node_id, Node::UncheckedBlock(Rc::clone(node)));
        true
    }

    fn enter_expression_statement(&mut self, node: &ExpressionStatement) -> bool {
        self.map
            .insert(node.node_id, Node::ExpressionStatement(Rc::clone(node)));
        true
    }

    fn enter_assembly_statement(&mut self, node: &AssemblyStatement) -> bool {
        self.map
            .insert(node.node_id, Node::AssemblyStatement(Rc::clone(node)));
        true
    }

    fn enter_assembly_flags_declaration(&mut self, node: &AssemblyFlagsDeclaration) -> bool {
        self.map.insert(
            node.node_id,
            Node::AssemblyFlagsDeclaration(Rc::clone(node)),
        );
        true
    }

    fn enter_tuple_deconstruction_statement(
        &mut self,
        node: &TupleDeconstructionStatement,
    ) -> bool {
        self.map.insert(
            node.node_id,
            Node::TupleDeconstructionStatement(Rc::clone(node)),
        );
        true
    }

    fn enter_tuple_deconstruction_element(&mut self, node: &TupleDeconstructionElement) -> bool {
        self.map.insert(
            node.node_id,
            Node::TupleDeconstructionElement(Rc::clone(node)),
        );
        true
    }

    fn enter_typed_tuple_member(&mut self, node: &TypedTupleMember) -> bool {
        self.map
            .insert(node.node_id, Node::TypedTupleMember(Rc::clone(node)));
        true
    }

    fn enter_untyped_tuple_member(&mut self, node: &UntypedTupleMember) -> bool {
        self.map
            .insert(node.node_id, Node::UntypedTupleMember(Rc::clone(node)));
        true
    }

    fn enter_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> bool {
        self.map.insert(
            node.node_id,
            Node::VariableDeclarationStatement(Rc::clone(node)),
        );
        true
    }

    fn enter_variable_declaration_value(&mut self, node: &VariableDeclarationValue) -> bool {
        self.map.insert(
            node.node_id,
            Node::VariableDeclarationValue(Rc::clone(node)),
        );
        true
    }

    fn enter_if_statement(&mut self, node: &IfStatement) -> bool {
        self.map
            .insert(node.node_id, Node::IfStatement(Rc::clone(node)));
        true
    }

    fn enter_else_branch(&mut self, node: &ElseBranch) -> bool {
        self.map
            .insert(node.node_id, Node::ElseBranch(Rc::clone(node)));
        true
    }

    fn enter_for_statement(&mut self, node: &ForStatement) -> bool {
        self.map
            .insert(node.node_id, Node::ForStatement(Rc::clone(node)));
        true
    }

    fn enter_while_statement(&mut self, node: &WhileStatement) -> bool {
        self.map
            .insert(node.node_id, Node::WhileStatement(Rc::clone(node)));
        true
    }

    fn enter_do_while_statement(&mut self, node: &DoWhileStatement) -> bool {
        self.map
            .insert(node.node_id, Node::DoWhileStatement(Rc::clone(node)));
        true
    }

    fn enter_continue_statement(&mut self, node: &ContinueStatement) -> bool {
        self.map
            .insert(node.node_id, Node::ContinueStatement(Rc::clone(node)));
        true
    }

    fn enter_break_statement(&mut self, node: &BreakStatement) -> bool {
        self.map
            .insert(node.node_id, Node::BreakStatement(Rc::clone(node)));
        true
    }

    fn enter_return_statement(&mut self, node: &ReturnStatement) -> bool {
        self.map
            .insert(node.node_id, Node::ReturnStatement(Rc::clone(node)));
        true
    }

    fn enter_emit_statement(&mut self, node: &EmitStatement) -> bool {
        self.map
            .insert(node.node_id, Node::EmitStatement(Rc::clone(node)));
        true
    }

    fn enter_try_statement(&mut self, node: &TryStatement) -> bool {
        self.map
            .insert(node.node_id, Node::TryStatement(Rc::clone(node)));
        true
    }

    fn enter_catch_clause(&mut self, node: &CatchClause) -> bool {
        self.map
            .insert(node.node_id, Node::CatchClause(Rc::clone(node)));
        true
    }

    fn enter_catch_clause_error(&mut self, node: &CatchClauseError) -> bool {
        self.map
            .insert(node.node_id, Node::CatchClauseError(Rc::clone(node)));
        true
    }

    fn enter_revert_statement(&mut self, node: &RevertStatement) -> bool {
        self.map
            .insert(node.node_id, Node::RevertStatement(Rc::clone(node)));
        true
    }

    fn enter_throw_statement(&mut self, node: &ThrowStatement) -> bool {
        self.map
            .insert(node.node_id, Node::ThrowStatement(Rc::clone(node)));
        true
    }

    fn enter_assignment_expression(&mut self, node: &AssignmentExpression) -> bool {
        self.map
            .insert(node.node_id, Node::AssignmentExpression(Rc::clone(node)));
        true
    }

    fn enter_conditional_expression(&mut self, node: &ConditionalExpression) -> bool {
        self.map
            .insert(node.node_id, Node::ConditionalExpression(Rc::clone(node)));
        true
    }

    fn enter_or_expression(&mut self, node: &OrExpression) -> bool {
        self.map
            .insert(node.node_id, Node::OrExpression(Rc::clone(node)));
        true
    }

    fn enter_and_expression(&mut self, node: &AndExpression) -> bool {
        self.map
            .insert(node.node_id, Node::AndExpression(Rc::clone(node)));
        true
    }

    fn enter_equality_expression(&mut self, node: &EqualityExpression) -> bool {
        self.map
            .insert(node.node_id, Node::EqualityExpression(Rc::clone(node)));
        true
    }

    fn enter_inequality_expression(&mut self, node: &InequalityExpression) -> bool {
        self.map
            .insert(node.node_id, Node::InequalityExpression(Rc::clone(node)));
        true
    }

    fn enter_bitwise_or_expression(&mut self, node: &BitwiseOrExpression) -> bool {
        self.map
            .insert(node.node_id, Node::BitwiseOrExpression(Rc::clone(node)));
        true
    }

    fn enter_bitwise_xor_expression(&mut self, node: &BitwiseXorExpression) -> bool {
        self.map
            .insert(node.node_id, Node::BitwiseXorExpression(Rc::clone(node)));
        true
    }

    fn enter_bitwise_and_expression(&mut self, node: &BitwiseAndExpression) -> bool {
        self.map
            .insert(node.node_id, Node::BitwiseAndExpression(Rc::clone(node)));
        true
    }

    fn enter_shift_expression(&mut self, node: &ShiftExpression) -> bool {
        self.map
            .insert(node.node_id, Node::ShiftExpression(Rc::clone(node)));
        true
    }

    fn enter_additive_expression(&mut self, node: &AdditiveExpression) -> bool {
        self.map
            .insert(node.node_id, Node::AdditiveExpression(Rc::clone(node)));
        true
    }

    fn enter_multiplicative_expression(&mut self, node: &MultiplicativeExpression) -> bool {
        self.map.insert(
            node.node_id,
            Node::MultiplicativeExpression(Rc::clone(node)),
        );
        true
    }

    fn enter_exponentiation_expression(&mut self, node: &ExponentiationExpression) -> bool {
        self.map.insert(
            node.node_id,
            Node::ExponentiationExpression(Rc::clone(node)),
        );
        true
    }

    fn enter_postfix_expression(&mut self, node: &PostfixExpression) -> bool {
        self.map
            .insert(node.node_id, Node::PostfixExpression(Rc::clone(node)));
        true
    }

    fn enter_prefix_expression(&mut self, node: &PrefixExpression) -> bool {
        self.map
            .insert(node.node_id, Node::PrefixExpression(Rc::clone(node)));
        true
    }

    fn enter_function_call_expression(&mut self, node: &FunctionCallExpression) -> bool {
        self.map
            .insert(node.node_id, Node::FunctionCallExpression(Rc::clone(node)));
        true
    }

    fn enter_call_options_expression(&mut self, node: &CallOptionsExpression) -> bool {
        self.map
            .insert(node.node_id, Node::CallOptionsExpression(Rc::clone(node)));
        true
    }

    fn enter_member_access_expression(&mut self, node: &MemberAccessExpression) -> bool {
        self.map
            .insert(node.node_id, Node::MemberAccessExpression(Rc::clone(node)));
        true
    }

    fn enter_index_access_expression(&mut self, node: &IndexAccessExpression) -> bool {
        self.map
            .insert(node.node_id, Node::IndexAccessExpression(Rc::clone(node)));
        true
    }

    fn enter_index_access_end(&mut self, node: &IndexAccessEnd) -> bool {
        self.map
            .insert(node.node_id, Node::IndexAccessEnd(Rc::clone(node)));
        true
    }

    fn enter_positional_arguments_declaration(
        &mut self,
        node: &PositionalArgumentsDeclaration,
    ) -> bool {
        self.map.insert(
            node.node_id,
            Node::PositionalArgumentsDeclaration(Rc::clone(node)),
        );
        true
    }

    fn enter_named_arguments_declaration(&mut self, node: &NamedArgumentsDeclaration) -> bool {
        self.map.insert(
            node.node_id,
            Node::NamedArgumentsDeclaration(Rc::clone(node)),
        );
        true
    }

    fn enter_named_argument_group(&mut self, node: &NamedArgumentGroup) -> bool {
        self.map
            .insert(node.node_id, Node::NamedArgumentGroup(Rc::clone(node)));
        true
    }

    fn enter_named_argument(&mut self, node: &NamedArgument) -> bool {
        self.map
            .insert(node.node_id, Node::NamedArgument(Rc::clone(node)));
        true
    }

    fn enter_type_expression(&mut self, node: &TypeExpression) -> bool {
        self.map
            .insert(node.node_id, Node::TypeExpression(Rc::clone(node)));
        true
    }

    fn enter_new_expression(&mut self, node: &NewExpression) -> bool {
        self.map
            .insert(node.node_id, Node::NewExpression(Rc::clone(node)));
        true
    }

    fn enter_tuple_expression(&mut self, node: &TupleExpression) -> bool {
        self.map
            .insert(node.node_id, Node::TupleExpression(Rc::clone(node)));
        true
    }

    fn enter_tuple_value(&mut self, node: &TupleValue) -> bool {
        self.map
            .insert(node.node_id, Node::TupleValue(Rc::clone(node)));
        true
    }

    fn enter_array_expression(&mut self, node: &ArrayExpression) -> bool {
        self.map
            .insert(node.node_id, Node::ArrayExpression(Rc::clone(node)));
        true
    }

    fn enter_hex_number_expression(&mut self, node: &HexNumberExpression) -> bool {
        self.map
            .insert(node.node_id, Node::HexNumberExpression(Rc::clone(node)));
        true
    }

    fn enter_decimal_number_expression(&mut self, node: &DecimalNumberExpression) -> bool {
        self.map
            .insert(node.node_id, Node::DecimalNumberExpression(Rc::clone(node)));
        true
    }

    fn enter_yul_block(&mut self, node: &YulBlock) -> bool {
        self.map
            .insert(node.node_id, Node::YulBlock(Rc::clone(node)));
        true
    }

    fn enter_yul_function_definition(&mut self, node: &YulFunctionDefinition) -> bool {
        self.map
            .insert(node.node_id, Node::YulFunctionDefinition(Rc::clone(node)));
        true
    }

    fn enter_yul_parameters_declaration(&mut self, node: &YulParametersDeclaration) -> bool {
        self.map.insert(
            node.node_id,
            Node::YulParametersDeclaration(Rc::clone(node)),
        );
        true
    }

    fn enter_yul_returns_declaration(&mut self, node: &YulReturnsDeclaration) -> bool {
        self.map
            .insert(node.node_id, Node::YulReturnsDeclaration(Rc::clone(node)));
        true
    }

    fn enter_yul_variable_declaration_statement(
        &mut self,
        node: &YulVariableDeclarationStatement,
    ) -> bool {
        self.map.insert(
            node.node_id,
            Node::YulVariableDeclarationStatement(Rc::clone(node)),
        );
        true
    }

    fn enter_yul_variable_declaration_value(&mut self, node: &YulVariableDeclarationValue) -> bool {
        self.map.insert(
            node.node_id,
            Node::YulVariableDeclarationValue(Rc::clone(node)),
        );
        true
    }

    fn enter_yul_variable_assignment_statement(
        &mut self,
        node: &YulVariableAssignmentStatement,
    ) -> bool {
        self.map.insert(
            node.node_id,
            Node::YulVariableAssignmentStatement(Rc::clone(node)),
        );
        true
    }

    fn enter_yul_colon_and_equal(&mut self, node: &YulColonAndEqual) -> bool {
        self.map
            .insert(node.node_id, Node::YulColonAndEqual(Rc::clone(node)));
        true
    }

    fn enter_yul_stack_assignment_statement(&mut self, node: &YulStackAssignmentStatement) -> bool {
        self.map.insert(
            node.node_id,
            Node::YulStackAssignmentStatement(Rc::clone(node)),
        );
        true
    }

    fn enter_yul_equal_and_colon(&mut self, node: &YulEqualAndColon) -> bool {
        self.map
            .insert(node.node_id, Node::YulEqualAndColon(Rc::clone(node)));
        true
    }

    fn enter_yul_if_statement(&mut self, node: &YulIfStatement) -> bool {
        self.map
            .insert(node.node_id, Node::YulIfStatement(Rc::clone(node)));
        true
    }

    fn enter_yul_for_statement(&mut self, node: &YulForStatement) -> bool {
        self.map
            .insert(node.node_id, Node::YulForStatement(Rc::clone(node)));
        true
    }

    fn enter_yul_switch_statement(&mut self, node: &YulSwitchStatement) -> bool {
        self.map
            .insert(node.node_id, Node::YulSwitchStatement(Rc::clone(node)));
        true
    }

    fn enter_yul_default_case(&mut self, node: &YulDefaultCase) -> bool {
        self.map
            .insert(node.node_id, Node::YulDefaultCase(Rc::clone(node)));
        true
    }

    fn enter_yul_value_case(&mut self, node: &YulValueCase) -> bool {
        self.map
            .insert(node.node_id, Node::YulValueCase(Rc::clone(node)));
        true
    }

    fn enter_yul_leave_statement(&mut self, node: &YulLeaveStatement) -> bool {
        self.map
            .insert(node.node_id, Node::YulLeaveStatement(Rc::clone(node)));
        true
    }

    fn enter_yul_break_statement(&mut self, node: &YulBreakStatement) -> bool {
        self.map
            .insert(node.node_id, Node::YulBreakStatement(Rc::clone(node)));
        true
    }

    fn enter_yul_continue_statement(&mut self, node: &YulContinueStatement) -> bool {
        self.map
            .insert(node.node_id, Node::YulContinueStatement(Rc::clone(node)));
        true
    }

    fn enter_yul_label(&mut self, node: &YulLabel) -> bool {
        self.map
            .insert(node.node_id, Node::YulLabel(Rc::clone(node)));
        true
    }

    fn enter_yul_function_call_expression(&mut self, node: &YulFunctionCallExpression) -> bool {
        self.map.insert(
            node.node_id,
            Node::YulFunctionCallExpression(Rc::clone(node)),
        );
        true
    }

    fn enter_source_unit_member(&mut self, _node: &SourceUnitMember) -> bool {
        true
    }
    fn enter_pragma(&mut self, _node: &Pragma) -> bool {
        true
    }
    fn enter_abicoder_version(&mut self, _node: &AbicoderVersion) -> bool {
        true
    }
    fn enter_experimental_feature(&mut self, _node: &ExperimentalFeature) -> bool {
        true
    }
    fn enter_version_expression(&mut self, _node: &VersionExpression) -> bool {
        true
    }
    fn enter_version_operator(&mut self, _node: &VersionOperator) -> bool {
        true
    }
    fn enter_version_literal(&mut self, _node: &VersionLiteral) -> bool {
        true
    }
    fn enter_import_clause(&mut self, _node: &ImportClause) -> bool {
        true
    }
    fn enter_using_clause(&mut self, _node: &UsingClause) -> bool {
        true
    }
    fn enter_using_operator(&mut self, _node: &UsingOperator) -> bool {
        true
    }
    fn enter_using_target(&mut self, _node: &UsingTarget) -> bool {
        true
    }
    fn enter_contract_specifier(&mut self, _node: &ContractSpecifier) -> bool {
        true
    }
    fn enter_contract_member(&mut self, _node: &ContractMember) -> bool {
        true
    }
    fn enter_state_variable_attribute(&mut self, _node: &StateVariableAttribute) -> bool {
        true
    }

    fn enter_function_name(&mut self, _node: &FunctionName) -> bool {
        true
    }
    fn enter_function_attribute(&mut self, _node: &FunctionAttribute) -> bool {
        true
    }
    fn enter_function_body(&mut self, _node: &FunctionBody) -> bool {
        true
    }
    fn enter_constructor_attribute(&mut self, _node: &ConstructorAttribute) -> bool {
        true
    }
    fn enter_unnamed_function_attribute(&mut self, _node: &UnnamedFunctionAttribute) -> bool {
        true
    }
    fn enter_fallback_function_attribute(&mut self, _node: &FallbackFunctionAttribute) -> bool {
        true
    }
    fn enter_receive_function_attribute(&mut self, _node: &ReceiveFunctionAttribute) -> bool {
        true
    }
    fn enter_modifier_attribute(&mut self, _node: &ModifierAttribute) -> bool {
        true
    }
    fn enter_type_name(&mut self, _node: &TypeName) -> bool {
        true
    }
    fn enter_function_type_attribute(&mut self, _node: &FunctionTypeAttribute) -> bool {
        true
    }
    fn enter_mapping_key_type(&mut self, _node: &MappingKeyType) -> bool {
        true
    }
    fn enter_elementary_type(&mut self, _node: &ElementaryType) -> bool {
        true
    }
    fn enter_statement(&mut self, _node: &Statement) -> bool {
        true
    }
    fn enter_tuple_member(&mut self, _node: &TupleMember) -> bool {
        true
    }
    fn enter_variable_declaration_type(&mut self, _node: &VariableDeclarationType) -> bool {
        true
    }
    fn enter_storage_location(&mut self, _node: &StorageLocation) -> bool {
        true
    }
    fn enter_for_statement_initialization(&mut self, _node: &ForStatementInitialization) -> bool {
        true
    }
    fn enter_for_statement_condition(&mut self, _node: &ForStatementCondition) -> bool {
        true
    }
    fn enter_expression(&mut self, _node: &Expression) -> bool {
        true
    }
    fn enter_arguments_declaration(&mut self, _node: &ArgumentsDeclaration) -> bool {
        true
    }
    fn enter_number_unit(&mut self, _node: &NumberUnit) -> bool {
        true
    }
    fn enter_string_expression(&mut self, _node: &StringExpression) -> bool {
        true
    }
    fn enter_string_literal(&mut self, _node: &StringLiteral) -> bool {
        true
    }
    fn enter_hex_string_literal(&mut self, _node: &HexStringLiteral) -> bool {
        true
    }
    fn enter_unicode_string_literal(&mut self, _node: &UnicodeStringLiteral) -> bool {
        true
    }
    fn enter_yul_statement(&mut self, _node: &YulStatement) -> bool {
        true
    }
    fn enter_yul_assignment_operator(&mut self, _node: &YulAssignmentOperator) -> bool {
        true
    }
    fn enter_yul_stack_assignment_operator(&mut self, _node: &YulStackAssignmentOperator) -> bool {
        true
    }
    fn enter_yul_switch_case(&mut self, _node: &YulSwitchCase) -> bool {
        true
    }
    fn enter_yul_expression(&mut self, _node: &YulExpression) -> bool {
        true
    }
    fn enter_yul_literal(&mut self, _node: &YulLiteral) -> bool {
        true
    }
    fn enter_source_unit_members(&mut self, _items: &SourceUnitMembers) -> bool {
        true
    }
    fn enter_version_expression_sets(&mut self, _items: &VersionExpressionSets) -> bool {
        true
    }
    fn enter_version_expression_set(&mut self, _items: &VersionExpressionSet) -> bool {
        true
    }
    fn enter_simple_version_literal(&mut self, _items: &SimpleVersionLiteral) -> bool {
        true
    }
    fn enter_import_deconstruction_symbols(
        &mut self,
        _items: &ImportDeconstructionSymbols,
    ) -> bool {
        true
    }
    fn enter_using_deconstruction_symbols(&mut self, _items: &UsingDeconstructionSymbols) -> bool {
        true
    }
    fn enter_inheritance_types(&mut self, _items: &InheritanceTypes) -> bool {
        true
    }
    fn enter_contract_members(&mut self, _items: &ContractMembers) -> bool {
        true
    }
    fn enter_interface_members(&mut self, _items: &InterfaceMembers) -> bool {
        true
    }
    fn enter_library_members(&mut self, _items: &LibraryMembers) -> bool {
        true
    }
    fn enter_struct_members(&mut self, _items: &StructMembers) -> bool {
        true
    }
    fn enter_enum_members(&mut self, _items: &EnumMembers) -> bool {
        true
    }
    fn enter_state_variable_attributes(&mut self, _items: &StateVariableAttributes) -> bool {
        true
    }
    fn enter_parameters(&mut self, _items: &Parameters) -> bool {
        true
    }
    fn enter_function_attributes(&mut self, _items: &FunctionAttributes) -> bool {
        true
    }
    fn enter_override_paths(&mut self, _items: &OverridePaths) -> bool {
        true
    }
    fn enter_constructor_attributes(&mut self, _items: &ConstructorAttributes) -> bool {
        true
    }
    fn enter_unnamed_function_attributes(&mut self, _items: &UnnamedFunctionAttributes) -> bool {
        true
    }
    fn enter_fallback_function_attributes(&mut self, _items: &FallbackFunctionAttributes) -> bool {
        true
    }
    fn enter_receive_function_attributes(&mut self, _items: &ReceiveFunctionAttributes) -> bool {
        true
    }
    fn enter_modifier_attributes(&mut self, _items: &ModifierAttributes) -> bool {
        true
    }
    fn enter_event_parameters(&mut self, _items: &EventParameters) -> bool {
        true
    }
    fn enter_error_parameters(&mut self, _items: &ErrorParameters) -> bool {
        true
    }
    fn enter_function_type_attributes(&mut self, _items: &FunctionTypeAttributes) -> bool {
        true
    }
    fn enter_statements(&mut self, _items: &Statements) -> bool {
        true
    }
    fn enter_assembly_flags(&mut self, _items: &AssemblyFlags) -> bool {
        true
    }
    fn enter_tuple_deconstruction_elements(
        &mut self,
        _items: &TupleDeconstructionElements,
    ) -> bool {
        true
    }
    fn enter_catch_clauses(&mut self, _items: &CatchClauses) -> bool {
        true
    }
    fn enter_positional_arguments(&mut self, _items: &PositionalArguments) -> bool {
        true
    }
    fn enter_named_arguments(&mut self, _items: &NamedArguments) -> bool {
        true
    }
    fn enter_call_options(&mut self, _items: &CallOptions) -> bool {
        true
    }
    fn enter_tuple_values(&mut self, _items: &TupleValues) -> bool {
        true
    }
    fn enter_array_values(&mut self, _items: &ArrayValues) -> bool {
        true
    }
    fn enter_string_literals(&mut self, _items: &StringLiterals) -> bool {
        true
    }
    fn enter_hex_string_literals(&mut self, _items: &HexStringLiterals) -> bool {
        true
    }
    fn enter_unicode_string_literals(&mut self, _items: &UnicodeStringLiterals) -> bool {
        true
    }
    fn enter_identifier_path(&mut self, _items: &IdentifierPath) -> bool {
        true
    }
    fn enter_yul_statements(&mut self, _items: &YulStatements) -> bool {
        true
    }
    fn enter_yul_parameters(&mut self, _items: &YulParameters) -> bool {
        true
    }
    fn enter_yul_variable_names(&mut self, _items: &YulVariableNames) -> bool {
        true
    }
    fn enter_yul_switch_cases(&mut self, _items: &YulSwitchCases) -> bool {
        true
    }
    fn enter_yul_arguments(&mut self, _items: &YulArguments) -> bool {
        true
    }
    fn enter_yul_paths(&mut self, _items: &YulPaths) -> bool {
        true
    }
    fn enter_yul_path(&mut self, _items: &YulPath) -> bool {
        true
    }
}
