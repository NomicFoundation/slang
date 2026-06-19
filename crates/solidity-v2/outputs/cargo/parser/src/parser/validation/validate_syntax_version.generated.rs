// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::needless_return)]
#![allow(clippy::wildcard_imports)]

use slang_solidity_v2_common::diagnostics::kinds::syntax::IncompatibleSyntaxVersion;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};
use slang_solidity_v2_cst::structured_cst::nodes::*;
use slang_solidity_v2_cst::structured_cst::TextRange;

/// Validate that all nodes in the given `SourceUnit` are valid for the given language version.
pub fn validate_syntax_version(
    root: &SourceUnit,
    version: LanguageVersion,
    file_id: &FileId,
    diagnostics: &mut DiagnosticCollection,
) {
    let mut validator = SyntaxVersionValidator {
        version,
        file_id,
        diagnostics,
    };
    validator.check_source_unit(root);
}

struct SyntaxVersionValidator<'a> {
    version: LanguageVersion,
    file_id: &'a FileId,
    diagnostics: &'a mut DiagnosticCollection,
}

impl SyntaxVersionValidator<'_> {
    fn check(&mut self, node: &impl TextRange, compatible_in: LanguageVersionSpecifier) -> bool {
        if compatible_in.contains(self.version) {
            return true;
        }

        self.diagnostics.push(
            self.file_id.to_owned(),
            node.calculate_text_range()
                .expect("Structured CST node should have a range"),
            IncompatibleSyntaxVersion { compatible_in },
        );

        false
    }

    //
    // Sequence validators
    //

    fn check_additive_expression(&mut self, node: &AdditiveExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_and_expression(&mut self, node: &AndExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_array_expression(&mut self, node: &ArrayExpression) {
        self.check_array_values(&node.items);
    }

    fn check_array_type_name(&mut self, node: &ArrayTypeName) {
        self.check_type_name(&node.operand);

        if let Some(ref child) = node.index {
            self.check_expression(child);
        }
    }

    fn check_assembly_statement(&mut self, node: &AssemblyStatement) {
        if let Some(ref child) = node.flags {
            if self.check(
                child,
                LanguageVersionSpecifier::from(LanguageVersion::V0_8_13),
            ) {
                self.check_yul_flags_declaration(child);
            }
        }
    }

    fn check_assignment_expression(&mut self, node: &AssignmentExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_bitwise_and_expression(&mut self, node: &BitwiseAndExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_bitwise_or_expression(&mut self, node: &BitwiseOrExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_bitwise_xor_expression(&mut self, node: &BitwiseXorExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_block(&mut self, node: &Block) {
        self.check_statements(&node.statements);
    }

    fn check_call_options_expression(&mut self, node: &CallOptionsExpression) {
        self.check_expression(&node.operand);

        self.check_call_options(&node.options);
    }

    fn check_catch_clause(&mut self, node: &CatchClause) {
        if let Some(ref child) = node.error {
            self.check_catch_clause_error(child);
        }

        self.check_block(&node.body);
    }

    fn check_catch_clause_error(&mut self, node: &CatchClauseError) {
        self.check_parameters_declaration(&node.parameters);
    }

    fn check_conditional_expression(&mut self, node: &ConditionalExpression) {
        self.check_expression(&node.operand);

        self.check_expression(&node.true_expression);

        self.check_expression(&node.false_expression);
    }

    fn check_constant_definition(&mut self, node: &ConstantDefinition) {
        self.check_type_name(&node.type_name);

        self.check_expression(&node.value);
    }

    fn check_constructor_definition(&mut self, node: &ConstructorDefinition) {
        self.check_parameters_declaration(&node.parameters);

        self.check_constructor_attributes(&node.attributes);

        self.check_block(&node.body);
    }

    fn check_contract_definition(&mut self, node: &ContractDefinition) {
        self.check_contract_specifiers(&node.specifiers);

        self.check_contract_members(&node.members);
    }

    fn check_do_while_statement(&mut self, node: &DoWhileStatement) {
        self.check_statement(&node.body);

        self.check_expression(&node.condition);
    }

    fn check_else_branch(&mut self, node: &ElseBranch) {
        self.check_statement(&node.body);
    }

    fn check_emit_statement(&mut self, node: &EmitStatement) {
        self.check_arguments_declaration(&node.arguments);
    }

    fn check_equality_expression(&mut self, node: &EqualityExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_error_definition(&mut self, node: &ErrorDefinition) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
        ) {
            return;
        }

        self.check_error_parameters_declaration(&node.members);
    }

    fn check_error_parameter(&mut self, node: &ErrorParameter) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
        ) {
            return;
        }

        self.check_type_name(&node.type_name);
    }

    fn check_error_parameters_declaration(&mut self, node: &ErrorParametersDeclaration) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
        ) {
            return;
        }

        self.check_error_parameters(&node.parameters);
    }

    fn check_event_definition(&mut self, node: &EventDefinition) {
        self.check_event_parameters_declaration(&node.parameters);
    }

    fn check_event_parameter(&mut self, node: &EventParameter) {
        self.check_type_name(&node.type_name);
    }

    fn check_event_parameters_declaration(&mut self, node: &EventParametersDeclaration) {
        self.check_event_parameters(&node.parameters);
    }

    fn check_exponentiation_expression(&mut self, node: &ExponentiationExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_expression_statement(&mut self, node: &ExpressionStatement) {
        self.check_expression(&node.expression);
    }

    fn check_fallback_function_definition(&mut self, node: &FallbackFunctionDefinition) {
        self.check_parameters_declaration(&node.parameters);

        self.check_fallback_function_attributes(&node.attributes);
        if let Some(ref child) = node.returns {
            self.check_returns_declaration(child);
        }

        self.check_function_body(&node.body);
    }

    fn check_for_statement(&mut self, node: &ForStatement) {
        self.check_for_statement_initialization(&node.initialization);

        self.check_for_statement_condition(&node.condition);
        if let Some(ref child) = node.iterator {
            self.check_expression(child);
        }

        self.check_statement(&node.body);
    }

    fn check_function_call_expression(&mut self, node: &FunctionCallExpression) {
        self.check_expression(&node.operand);

        self.check_arguments_declaration(&node.arguments);
    }

    fn check_function_definition(&mut self, node: &FunctionDefinition) {
        self.check_parameters_declaration(&node.parameters);

        self.check_function_attributes(&node.attributes);
        if let Some(ref child) = node.returns {
            self.check_returns_declaration(child);
        }

        self.check_function_body(&node.body);
    }

    fn check_function_type(&mut self, node: &FunctionType) {
        self.check_parameters_declaration(&node.parameters);

        if let Some(ref child) = node.returns {
            self.check_returns_declaration(child);
        }
    }

    fn check_if_statement(&mut self, node: &IfStatement) {
        self.check_expression(&node.condition);

        self.check_statement(&node.body);
        if let Some(ref child) = node.else_branch {
            self.check_else_branch(child);
        }
    }

    fn check_index_access_end(&mut self, node: &IndexAccessEnd) {
        if let Some(ref child) = node.end {
            self.check_expression(child);
        }
    }

    fn check_index_access_expression(&mut self, node: &IndexAccessExpression) {
        self.check_expression(&node.operand);

        if let Some(ref child) = node.start {
            self.check_expression(child);
        }
        if let Some(ref child) = node.end {
            self.check_index_access_end(child);
        }
    }

    fn check_inequality_expression(&mut self, node: &InequalityExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_inheritance_specifier(&mut self, node: &InheritanceSpecifier) {
        self.check_inheritance_types(&node.types);
    }

    fn check_inheritance_type(&mut self, node: &InheritanceType) {
        if let Some(ref child) = node.arguments {
            self.check_arguments_declaration(child);
        }
    }

    fn check_interface_definition(&mut self, node: &InterfaceDefinition) {
        if let Some(ref child) = node.inheritance {
            self.check_inheritance_specifier(child);
        }

        self.check_interface_members(&node.members);
    }

    fn check_library_definition(&mut self, node: &LibraryDefinition) {
        self.check_library_members(&node.members);
    }

    fn check_mapping_key(&mut self, node: &MappingKey) {
        if let Some(ref child) = node.name {
            self.check(
                child,
                LanguageVersionSpecifier::from(LanguageVersion::V0_8_18),
            );
        }
    }

    fn check_mapping_type(&mut self, node: &MappingType) {
        self.check_mapping_key(&node.key_type);

        self.check_mapping_value(&node.value_type);
    }

    fn check_mapping_value(&mut self, node: &MappingValue) {
        self.check_type_name(&node.type_name);
        if let Some(ref child) = node.name {
            self.check(
                child,
                LanguageVersionSpecifier::from(LanguageVersion::V0_8_18),
            );
        }
    }

    fn check_member_access_expression(&mut self, node: &MemberAccessExpression) {
        self.check_expression(&node.operand);
    }

    fn check_modifier_definition(&mut self, node: &ModifierDefinition) {
        if let Some(ref child) = node.parameters {
            self.check_parameters_declaration(child);
        }

        self.check_function_body(&node.body);
    }

    fn check_modifier_invocation(&mut self, node: &ModifierInvocation) {
        if let Some(ref child) = node.arguments {
            self.check_arguments_declaration(child);
        }
    }

    fn check_multi_typed_declaration(&mut self, node: &MultiTypedDeclaration) {
        self.check_multi_typed_declaration_elements(&node.elements);

        self.check_variable_declaration_value(&node.value);
    }

    fn check_multi_typed_declaration_element(&mut self, node: &MultiTypedDeclarationElement) {
        if let Some(ref child) = node.member {
            self.check_variable_declaration(child);
        }
    }

    fn check_multiplicative_expression(&mut self, node: &MultiplicativeExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_named_argument(&mut self, node: &NamedArgument) {
        self.check_expression(&node.value);
    }

    fn check_named_argument_group(&mut self, node: &NamedArgumentGroup) {
        self.check_named_arguments(&node.arguments);
    }

    fn check_named_arguments_declaration(&mut self, node: &NamedArgumentsDeclaration) {
        self.check_named_argument_group(&node.arguments);
    }

    fn check_new_expression(&mut self, node: &NewExpression) {
        self.check_type_name(&node.type_name);
    }

    fn check_or_expression(&mut self, node: &OrExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_parameter(&mut self, node: &Parameter) {
        self.check_type_name(&node.type_name);
    }

    fn check_parameters_declaration(&mut self, node: &ParametersDeclaration) {
        self.check_parameters(&node.parameters);
    }

    fn check_positional_arguments_declaration(&mut self, node: &PositionalArgumentsDeclaration) {
        self.check_positional_arguments(&node.arguments);
    }

    fn check_postfix_expression(&mut self, node: &PostfixExpression) {
        self.check_expression(&node.operand);
    }

    fn check_prefix_expression(&mut self, node: &PrefixExpression) {
        self.check_expression(&node.operand);
    }

    fn check_receive_function_definition(&mut self, node: &ReceiveFunctionDefinition) {
        self.check_parameters_declaration(&node.parameters);

        self.check_receive_function_attributes(&node.attributes);

        self.check_function_body(&node.body);
    }

    fn check_return_statement(&mut self, node: &ReturnStatement) {
        if let Some(ref child) = node.expression {
            self.check_expression(child);
        }
    }

    fn check_returns_declaration(&mut self, node: &ReturnsDeclaration) {
        self.check_parameters_declaration(&node.variables);
    }

    fn check_revert_statement(&mut self, node: &RevertStatement) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
        ) {
            return;
        }

        self.check_arguments_declaration(&node.arguments);
    }

    fn check_shift_expression(&mut self, node: &ShiftExpression) {
        self.check_expression(&node.left_operand);

        self.check_expression(&node.right_operand);
    }

    fn check_single_typed_declaration(&mut self, node: &SingleTypedDeclaration) {
        self.check_variable_declaration(&node.declaration);
        if let Some(ref child) = node.value {
            self.check_variable_declaration_value(child);
        }
    }

    fn check_source_unit(&mut self, node: &SourceUnit) {
        self.check_source_unit_members(&node.members);
    }

    fn check_state_variable_definition(&mut self, node: &StateVariableDefinition) {
        self.check_type_name(&node.type_name);

        self.check_state_variable_attributes(&node.attributes);

        if let Some(ref child) = node.value {
            self.check_state_variable_definition_value(child);
        }
    }

    fn check_state_variable_definition_value(&mut self, node: &StateVariableDefinitionValue) {
        self.check_expression(&node.value);
    }

    fn check_storage_layout_specifier(&mut self, node: &StorageLayoutSpecifier) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_29),
        ) {
            return;
        }

        self.check_expression(&node.expression);
    }

    fn check_struct_definition(&mut self, node: &StructDefinition) {
        self.check_struct_members(&node.members);
    }

    fn check_struct_member(&mut self, node: &StructMember) {
        self.check_type_name(&node.type_name);
    }

    fn check_try_statement(&mut self, node: &TryStatement) {
        self.check_expression(&node.expression);
        if let Some(ref child) = node.returns {
            self.check_returns_declaration(child);
        }

        self.check_block(&node.body);

        self.check_catch_clauses(&node.catch_clauses);
    }

    fn check_tuple_expression(&mut self, node: &TupleExpression) {
        self.check_tuple_values(&node.items);
    }

    fn check_tuple_value(&mut self, node: &TupleValue) {
        if let Some(ref child) = node.expression {
            self.check_expression(child);
        }
    }

    fn check_type_expression(&mut self, node: &TypeExpression) {
        self.check_type_name(&node.type_name);
    }

    fn check_unchecked_block(&mut self, node: &UncheckedBlock) {
        self.check_block(&node.block);
    }

    fn check_user_defined_value_type_definition(&mut self, node: &UserDefinedValueTypeDefinition) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
        ) {
            return;
        }
    }

    fn check_using_alias(&mut self, node: &UsingAlias) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_19),
        ) {
            return;
        }

        self.check_using_operator(&node.operator);
    }

    fn check_using_deconstruction(&mut self, node: &UsingDeconstruction) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_13),
        ) {
            return;
        }

        self.check_using_deconstruction_symbols(&node.symbols);
    }

    fn check_using_deconstruction_symbol(&mut self, node: &UsingDeconstructionSymbol) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_13),
        ) {
            return;
        }

        if let Some(ref child) = node.alias {
            if self.check(
                child,
                LanguageVersionSpecifier::from(LanguageVersion::V0_8_19),
            ) {
                self.check_using_alias(child);
            }
        }
    }

    fn check_using_directive(&mut self, node: &UsingDirective) {
        self.check_using_clause(&node.clause);

        self.check_using_target(&node.target);
        if let Some(ref child) = node.global_keyword {
            self.check(
                child,
                LanguageVersionSpecifier::from(LanguageVersion::V0_8_13),
            );
        }
    }

    fn check_variable_declaration(&mut self, node: &VariableDeclaration) {
        self.check_type_name(&node.type_name);
    }

    fn check_variable_declaration_statement(&mut self, node: &VariableDeclarationStatement) {
        self.check_variable_declaration_target(&node.target);
    }

    fn check_variable_declaration_value(&mut self, node: &VariableDeclarationValue) {
        self.check_expression(&node.expression);
    }

    fn check_while_statement(&mut self, node: &WhileStatement) {
        self.check_expression(&node.condition);

        self.check_statement(&node.body);
    }

    fn check_yul_flags_declaration(&mut self, node: &YulFlagsDeclaration) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_13),
        ) {
            return;
        }

        self.check_yul_flags(&node.flags);
    }

    //
    // Choice validators
    //

    fn check_arguments_declaration(&mut self, node: &ArgumentsDeclaration) {
        match node {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(child) => {
                self.check_positional_arguments_declaration(child);
            }
            ArgumentsDeclaration::NamedArgumentsDeclaration(child) => {
                self.check_named_arguments_declaration(child);
            }
        }
    }

    fn check_constructor_attribute(&mut self, node: &ConstructorAttribute) {
        match node {
            ConstructorAttribute::ModifierInvocation(child) => {
                self.check_modifier_invocation(child);
            }
            ConstructorAttribute::InternalKeyword(_) => {}
            ConstructorAttribute::PayableKeyword(_) => {}
            ConstructorAttribute::PublicKeyword(_) => {}
        }
    }

    fn check_contract_member(&mut self, node: &ContractMember) {
        match node {
            ContractMember::UsingDirective(child) => {
                self.check_using_directive(child);
            }
            ContractMember::FunctionDefinition(child) => {
                self.check_function_definition(child);
            }
            ContractMember::ConstructorDefinition(child) => {
                self.check_constructor_definition(child);
            }
            ContractMember::ReceiveFunctionDefinition(child) => {
                self.check_receive_function_definition(child);
            }
            ContractMember::FallbackFunctionDefinition(child) => {
                self.check_fallback_function_definition(child);
            }
            ContractMember::ModifierDefinition(child) => {
                self.check_modifier_definition(child);
            }
            ContractMember::StructDefinition(child) => {
                self.check_struct_definition(child);
            }
            ContractMember::EnumDefinition(_) => {}
            ContractMember::EventDefinition(child) => {
                self.check_event_definition(child);
            }
            ContractMember::ErrorDefinition(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
                ) {
                    return;
                }
                self.check_error_definition(child);
            }
            ContractMember::UserDefinedValueTypeDefinition(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                ) {
                    return;
                }
                self.check_user_defined_value_type_definition(child);
            }
            ContractMember::StateVariableDefinition(child) => {
                self.check_state_variable_definition(child);
            }
        }
    }

    fn check_contract_specifier(&mut self, node: &ContractSpecifier) {
        match node {
            ContractSpecifier::InheritanceSpecifier(child) => {
                self.check_inheritance_specifier(child);
            }
            ContractSpecifier::StorageLayoutSpecifier(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_29),
                ) {
                    return;
                }
                self.check_storage_layout_specifier(child);
            }
        }
    }

    fn check_expression(&mut self, node: &Expression) {
        match node {
            Expression::AssignmentExpression(child) => {
                self.check_assignment_expression(child);
            }
            Expression::ConditionalExpression(child) => {
                self.check_conditional_expression(child);
            }
            Expression::OrExpression(child) => {
                self.check_or_expression(child);
            }
            Expression::AndExpression(child) => {
                self.check_and_expression(child);
            }
            Expression::EqualityExpression(child) => {
                self.check_equality_expression(child);
            }
            Expression::InequalityExpression(child) => {
                self.check_inequality_expression(child);
            }
            Expression::BitwiseOrExpression(child) => {
                self.check_bitwise_or_expression(child);
            }
            Expression::BitwiseXorExpression(child) => {
                self.check_bitwise_xor_expression(child);
            }
            Expression::BitwiseAndExpression(child) => {
                self.check_bitwise_and_expression(child);
            }
            Expression::ShiftExpression(child) => {
                self.check_shift_expression(child);
            }
            Expression::AdditiveExpression(child) => {
                self.check_additive_expression(child);
            }
            Expression::MultiplicativeExpression(child) => {
                self.check_multiplicative_expression(child);
            }
            Expression::ExponentiationExpression(child) => {
                self.check_exponentiation_expression(child);
            }
            Expression::PostfixExpression(child) => {
                self.check_postfix_expression(child);
            }
            Expression::PrefixExpression(child) => {
                self.check_prefix_expression(child);
            }
            Expression::FunctionCallExpression(child) => {
                self.check_function_call_expression(child);
            }
            Expression::CallOptionsExpression(child) => {
                self.check_call_options_expression(child);
            }
            Expression::MemberAccessExpression(child) => {
                self.check_member_access_expression(child);
            }
            Expression::IndexAccessExpression(child) => {
                self.check_index_access_expression(child);
            }
            Expression::NewExpression(child) => {
                self.check_new_expression(child);
            }
            Expression::TupleExpression(child) => {
                self.check_tuple_expression(child);
            }
            Expression::TypeExpression(child) => {
                self.check_type_expression(child);
            }
            Expression::ArrayExpression(child) => {
                self.check_array_expression(child);
            }
            Expression::HexNumberExpression(_) => {}
            Expression::DecimalNumberExpression(_) => {}
            Expression::StringExpression(_) => {}
            Expression::ElementaryType(_) => {}
            Expression::PayableKeyword(_) => {}
            Expression::ThisKeyword(_) => {}
            Expression::SuperKeyword(_) => {}
            Expression::TrueKeyword(_) => {}
            Expression::FalseKeyword(_) => {}
            Expression::Identifier(_) => {}
        }
    }

    fn check_fallback_function_attribute(&mut self, node: &FallbackFunctionAttribute) {
        match node {
            FallbackFunctionAttribute::ModifierInvocation(child) => {
                self.check_modifier_invocation(child);
            }
            FallbackFunctionAttribute::OverrideSpecifier(_) => {}
            FallbackFunctionAttribute::ExternalKeyword(_) => {}
            FallbackFunctionAttribute::PayableKeyword(_) => {}
            FallbackFunctionAttribute::PureKeyword(_) => {}
            FallbackFunctionAttribute::ViewKeyword(_) => {}
            FallbackFunctionAttribute::VirtualKeyword(_) => {}
        }
    }

    fn check_for_statement_condition(&mut self, node: &ForStatementCondition) {
        match node {
            ForStatementCondition::ExpressionStatement(child) => {
                self.check_expression_statement(child);
            }
            ForStatementCondition::Semicolon(_) => {}
        }
    }

    fn check_for_statement_initialization(&mut self, node: &ForStatementInitialization) {
        match node {
            ForStatementInitialization::VariableDeclarationStatement(child) => {
                self.check_variable_declaration_statement(child);
            }
            ForStatementInitialization::ExpressionStatement(child) => {
                self.check_expression_statement(child);
            }
            ForStatementInitialization::Semicolon(_) => {}
        }
    }

    fn check_function_attribute(&mut self, node: &FunctionAttribute) {
        match node {
            FunctionAttribute::ModifierInvocation(child) => {
                self.check_modifier_invocation(child);
            }
            FunctionAttribute::OverrideSpecifier(_) => {}
            FunctionAttribute::ExternalKeyword(_) => {}
            FunctionAttribute::InternalKeyword(_) => {}
            FunctionAttribute::PayableKeyword(_) => {}
            FunctionAttribute::PrivateKeyword(_) => {}
            FunctionAttribute::PublicKeyword(_) => {}
            FunctionAttribute::PureKeyword(_) => {}
            FunctionAttribute::ViewKeyword(_) => {}
            FunctionAttribute::VirtualKeyword(_) => {}
        }
    }

    fn check_function_body(&mut self, node: &FunctionBody) {
        match node {
            FunctionBody::Block(child) => {
                self.check_block(child);
            }
            FunctionBody::Semicolon(_) => {}
        }
    }

    fn check_receive_function_attribute(&mut self, node: &ReceiveFunctionAttribute) {
        match node {
            ReceiveFunctionAttribute::ModifierInvocation(child) => {
                self.check_modifier_invocation(child);
            }
            ReceiveFunctionAttribute::OverrideSpecifier(_) => {}
            ReceiveFunctionAttribute::ExternalKeyword(_) => {}
            ReceiveFunctionAttribute::PayableKeyword(_) => {}
            ReceiveFunctionAttribute::VirtualKeyword(_) => {}
        }
    }

    fn check_source_unit_member(&mut self, node: &SourceUnitMember) {
        match node {
            SourceUnitMember::PragmaDirective(_) => {}
            SourceUnitMember::ImportDirective(_) => {}
            SourceUnitMember::ContractDefinition(child) => {
                self.check_contract_definition(child);
            }
            SourceUnitMember::InterfaceDefinition(child) => {
                self.check_interface_definition(child);
            }
            SourceUnitMember::LibraryDefinition(child) => {
                self.check_library_definition(child);
            }
            SourceUnitMember::StructDefinition(child) => {
                self.check_struct_definition(child);
            }
            SourceUnitMember::EnumDefinition(_) => {}
            SourceUnitMember::FunctionDefinition(child) => {
                self.check_function_definition(child);
            }
            SourceUnitMember::ErrorDefinition(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
                ) {
                    return;
                }
                self.check_error_definition(child);
            }
            SourceUnitMember::UserDefinedValueTypeDefinition(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                ) {
                    return;
                }
                self.check_user_defined_value_type_definition(child);
            }
            SourceUnitMember::UsingDirective(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_13),
                ) {
                    return;
                }
                self.check_using_directive(child);
            }
            SourceUnitMember::EventDefinition(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_22),
                ) {
                    return;
                }
                self.check_event_definition(child);
            }
            SourceUnitMember::ConstantDefinition(child) => {
                self.check_constant_definition(child);
            }
        }
    }

    fn check_state_variable_attribute(&mut self, node: &StateVariableAttribute) {
        match node {
            StateVariableAttribute::OverrideSpecifier(_) => {}
            StateVariableAttribute::ConstantKeyword(_) => {}
            StateVariableAttribute::InternalKeyword(_) => {}
            StateVariableAttribute::PrivateKeyword(_) => {}
            StateVariableAttribute::PublicKeyword(_) => {}
            StateVariableAttribute::ImmutableKeyword(_) => {}
            StateVariableAttribute::TransientKeyword(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_27),
                ) {
                    return;
                }
            }
        }
    }

    fn check_statement(&mut self, node: &Statement) {
        match node {
            Statement::IfStatement(child) => {
                self.check_if_statement(child);
            }
            Statement::ForStatement(child) => {
                self.check_for_statement(child);
            }
            Statement::WhileStatement(child) => {
                self.check_while_statement(child);
            }
            Statement::DoWhileStatement(child) => {
                self.check_do_while_statement(child);
            }
            Statement::ContinueStatement(_) => {}
            Statement::BreakStatement(_) => {}
            Statement::ReturnStatement(child) => {
                self.check_return_statement(child);
            }
            Statement::EmitStatement(child) => {
                self.check_emit_statement(child);
            }
            Statement::TryStatement(child) => {
                self.check_try_statement(child);
            }
            Statement::RevertStatement(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
                ) {
                    return;
                }
                self.check_revert_statement(child);
            }
            Statement::AssemblyStatement(child) => {
                self.check_assembly_statement(child);
            }
            Statement::Block(child) => {
                self.check_block(child);
            }
            Statement::UncheckedBlock(child) => {
                self.check_unchecked_block(child);
            }
            Statement::VariableDeclarationStatement(child) => {
                self.check_variable_declaration_statement(child);
            }
            Statement::ExpressionStatement(child) => {
                self.check_expression_statement(child);
            }
        }
    }

    fn check_type_name(&mut self, node: &TypeName) {
        match node {
            TypeName::ArrayTypeName(child) => {
                self.check_array_type_name(child);
            }
            TypeName::FunctionType(child) => {
                self.check_function_type(child);
            }
            TypeName::MappingType(child) => {
                self.check_mapping_type(child);
            }
            TypeName::ElementaryType(_) => {}
            TypeName::IdentifierPath(_) => {}
        }
    }

    fn check_using_clause(&mut self, node: &UsingClause) {
        match node {
            UsingClause::IdentifierPath(_) => {}
            UsingClause::UsingDeconstruction(child) => {
                if !self.check(
                    child,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_13),
                ) {
                    return;
                }
                self.check_using_deconstruction(child);
            }
        }
    }

    fn check_using_operator(&mut self, node: &UsingOperator) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_19),
        ) {
            return;
        }

        match node {
            UsingOperator::Ampersand(_) => {}
            UsingOperator::Asterisk(_) => {}
            UsingOperator::BangEqual(_) => {}
            UsingOperator::Bar(_) => {}
            UsingOperator::Caret(_) => {}
            UsingOperator::EqualEqual(_) => {}
            UsingOperator::GreaterThan(_) => {}
            UsingOperator::GreaterThanEqual(_) => {}
            UsingOperator::LessThan(_) => {}
            UsingOperator::LessThanEqual(_) => {}
            UsingOperator::Minus(_) => {}
            UsingOperator::Percent(_) => {}
            UsingOperator::Plus(_) => {}
            UsingOperator::Slash(_) => {}
            UsingOperator::Tilde(_) => {}
        }
    }

    fn check_using_target(&mut self, node: &UsingTarget) {
        match node {
            UsingTarget::TypeName(child) => {
                self.check_type_name(child);
            }
            UsingTarget::Asterisk(_) => {}
        }
    }

    fn check_variable_declaration_target(&mut self, node: &VariableDeclarationTarget) {
        match node {
            VariableDeclarationTarget::SingleTypedDeclaration(child) => {
                self.check_single_typed_declaration(child);
            }
            VariableDeclarationTarget::MultiTypedDeclaration(child) => {
                self.check_multi_typed_declaration(child);
            }
        }
    }

    //
    // Collection validators
    //

    fn check_array_values(&mut self, node: &ArrayValues) {
        for child in &node.elements {
            self.check_expression(child);
        }
    }

    fn check_call_options(&mut self, node: &CallOptions) {
        for child in &node.elements {
            self.check_named_argument(child);
        }
    }

    fn check_catch_clauses(&mut self, node: &CatchClauses) {
        for child in &node.elements {
            self.check_catch_clause(child);
        }
    }

    fn check_constructor_attributes(&mut self, node: &ConstructorAttributes) {
        for child in &node.elements {
            self.check_constructor_attribute(child);
        }
    }

    fn check_contract_members(&mut self, node: &ContractMembers) {
        for child in &node.elements {
            self.check_contract_member(child);
        }
    }

    fn check_contract_specifiers(&mut self, node: &ContractSpecifiers) {
        for child in &node.elements {
            self.check_contract_specifier(child);
        }
    }

    fn check_error_parameters(&mut self, node: &ErrorParameters) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
        ) {
            return;
        }
        for child in &node.elements {
            self.check_error_parameter(child);
        }
    }

    fn check_event_parameters(&mut self, node: &EventParameters) {
        for child in &node.elements {
            self.check_event_parameter(child);
        }
    }

    fn check_fallback_function_attributes(&mut self, node: &FallbackFunctionAttributes) {
        for child in &node.elements {
            self.check_fallback_function_attribute(child);
        }
    }

    fn check_function_attributes(&mut self, node: &FunctionAttributes) {
        for child in &node.elements {
            self.check_function_attribute(child);
        }
    }

    fn check_inheritance_types(&mut self, node: &InheritanceTypes) {
        for child in &node.elements {
            self.check_inheritance_type(child);
        }
    }

    fn check_interface_members(&mut self, node: &InterfaceMembers) {
        for child in &node.elements {
            self.check_contract_member(child);
        }
    }

    fn check_library_members(&mut self, node: &LibraryMembers) {
        for child in &node.elements {
            self.check_contract_member(child);
        }
    }

    fn check_multi_typed_declaration_elements(&mut self, node: &MultiTypedDeclarationElements) {
        for child in &node.elements {
            self.check_multi_typed_declaration_element(child);
        }
    }

    fn check_named_arguments(&mut self, node: &NamedArguments) {
        for child in &node.elements {
            self.check_named_argument(child);
        }
    }

    fn check_parameters(&mut self, node: &Parameters) {
        for child in &node.elements {
            self.check_parameter(child);
        }
    }

    fn check_positional_arguments(&mut self, node: &PositionalArguments) {
        for child in &node.elements {
            self.check_expression(child);
        }
    }

    fn check_receive_function_attributes(&mut self, node: &ReceiveFunctionAttributes) {
        for child in &node.elements {
            self.check_receive_function_attribute(child);
        }
    }

    fn check_source_unit_members(&mut self, node: &SourceUnitMembers) {
        for child in &node.elements {
            self.check_source_unit_member(child);
        }
    }

    fn check_state_variable_attributes(&mut self, node: &StateVariableAttributes) {
        for child in &node.elements {
            self.check_state_variable_attribute(child);
        }
    }

    fn check_statements(&mut self, node: &Statements) {
        for child in &node.elements {
            self.check_statement(child);
        }
    }

    fn check_struct_members(&mut self, node: &StructMembers) {
        for child in &node.elements {
            self.check_struct_member(child);
        }
    }

    fn check_tuple_values(&mut self, node: &TupleValues) {
        for child in &node.elements {
            self.check_tuple_value(child);
        }
    }

    fn check_using_deconstruction_symbols(&mut self, node: &UsingDeconstructionSymbols) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_13),
        ) {
            return;
        }
        for child in &node.elements {
            self.check_using_deconstruction_symbol(child);
        }
    }

    fn check_yul_flags(&mut self, node: &YulFlags) {
        if !self.check(
            node,
            LanguageVersionSpecifier::from(LanguageVersion::V0_8_13),
        ) {
            return;
        }
    }
}
