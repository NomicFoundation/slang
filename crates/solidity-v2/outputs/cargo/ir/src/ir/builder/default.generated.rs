// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// Builder from StructuredCstModel
#![allow(clippy::too_many_lines)]
#![allow(unused_variables)]

use std::sync::Arc;

use slang_solidity_v2_cst::structured_cst::nodes as input;
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use super::CstToIrBuilder;
use crate::ir::{nodes as output, Source};

impl<S: Source> CstToIrBuilder<'_, S> {
    //
    // Sequences
    //

    pub(super) fn build_abicoder_pragma(
        &mut self,
        source: &input::AbicoderPragma,
    ) -> output::AbicoderPragma {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let version = self.build_abicoder_version(&source.version);

        Arc::new(output::AbicoderPragmaStruct { id, range, version })
    }

    pub(super) fn build_additive_expression(
        &mut self,
        source: &input::AdditiveExpression,
    ) -> output::AdditiveExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let operator = self.build_expression_additive_expression_operator(
            &source.expression_additive_expression_operator,
        );
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::AdditiveExpressionStruct {
            id,
            range,
            left_operand,
            operator,
            right_operand,
        })
    }

    pub(super) fn build_address_type(
        &mut self,
        source: &input::AddressType,
    ) -> output::AddressType {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let payable_keyword = source
            .payable_keyword
            .as_ref()
            .map(|value| self.build_payable_keyword(value));

        Arc::new(output::AddressTypeStruct {
            id,
            range,
            payable_keyword,
        })
    }

    pub(super) fn build_and_expression(
        &mut self,
        source: &input::AndExpression,
    ) -> output::AndExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::AndExpressionStruct {
            id,
            range,
            left_operand,
            right_operand,
        })
    }

    pub(super) fn build_array_expression(
        &mut self,
        source: &input::ArrayExpression,
    ) -> output::ArrayExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let items = self.build_array_values(&source.items);

        Arc::new(output::ArrayExpressionStruct { id, range, items })
    }

    pub(super) fn build_array_type_name(
        &mut self,
        source: &input::ArrayTypeName,
    ) -> output::ArrayTypeName {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operand = self.build_type_name(&source.operand);
        let index = source
            .index
            .as_ref()
            .map(|value| self.build_expression(value));

        Arc::new(output::ArrayTypeNameStruct {
            id,
            range,
            operand,
            index,
        })
    }

    pub(super) fn build_assembly_statement(
        &mut self,
        source: &input::AssemblyStatement,
    ) -> output::AssemblyStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let label = source
            .label
            .as_ref()
            .map(|value| self.build_yul_string_literal(value));
        let flags = source
            .flags
            .as_ref()
            .map(|value| self.build_yul_flags_declaration(value));
        let body = self.build_yul_block(&source.body);

        Arc::new(output::AssemblyStatementStruct {
            id,
            range,
            label,
            flags,
            body,
        })
    }

    pub(super) fn build_assignment_expression(
        &mut self,
        source: &input::AssignmentExpression,
    ) -> output::AssignmentExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let operator = self.build_expression_assignment_expression_operator(
            &source.expression_assignment_expression_operator,
        );
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::AssignmentExpressionStruct {
            id,
            range,
            left_operand,
            operator,
            right_operand,
        })
    }

    pub(super) fn build_bitwise_and_expression(
        &mut self,
        source: &input::BitwiseAndExpression,
    ) -> output::BitwiseAndExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::BitwiseAndExpressionStruct {
            id,
            range,
            left_operand,
            right_operand,
        })
    }

    pub(super) fn build_bitwise_or_expression(
        &mut self,
        source: &input::BitwiseOrExpression,
    ) -> output::BitwiseOrExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::BitwiseOrExpressionStruct {
            id,
            range,
            left_operand,
            right_operand,
        })
    }

    pub(super) fn build_bitwise_xor_expression(
        &mut self,
        source: &input::BitwiseXorExpression,
    ) -> output::BitwiseXorExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::BitwiseXorExpressionStruct {
            id,
            range,
            left_operand,
            right_operand,
        })
    }

    pub(super) fn build_block(&mut self, source: &input::Block) -> output::Block {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let statements = self.build_statements(&source.statements);

        Arc::new(output::BlockStruct {
            id,
            range,
            statements,
        })
    }

    pub(super) fn build_break_statement(
        &mut self,
        source: &input::BreakStatement,
    ) -> output::BreakStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();

        Arc::new(output::BreakStatementStruct { id, range })
    }

    pub(super) fn build_call_options_expression(
        &mut self,
        source: &input::CallOptionsExpression,
    ) -> output::CallOptionsExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let options = self.build_call_options(&source.options);

        Arc::new(output::CallOptionsExpressionStruct {
            id,
            range,
            operand,
            options,
        })
    }

    pub(super) fn build_catch_clause(
        &mut self,
        source: &input::CatchClause,
    ) -> output::CatchClause {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let error = source
            .error
            .as_ref()
            .map(|value| self.build_catch_clause_error(value));
        let body = self.build_block(&source.body);

        Arc::new(output::CatchClauseStruct {
            id,
            range,
            error,
            body,
        })
    }

    pub(super) fn build_catch_clause_error(
        &mut self,
        source: &input::CatchClauseError,
    ) -> output::CatchClauseError {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = source
            .name
            .as_ref()
            .map(|value| self.build_identifier(value));
        let parameters = self.build_parameters_declaration(&source.parameters);

        Arc::new(output::CatchClauseErrorStruct {
            id,
            range,
            name,
            parameters,
        })
    }

    pub(super) fn build_conditional_expression(
        &mut self,
        source: &input::ConditionalExpression,
    ) -> output::ConditionalExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let true_expression = self.build_expression(&source.true_expression);
        let false_expression = self.build_expression(&source.false_expression);

        Arc::new(output::ConditionalExpressionStruct {
            id,
            range,
            operand,
            true_expression,
            false_expression,
        })
    }

    pub(super) fn build_continue_statement(
        &mut self,
        source: &input::ContinueStatement,
    ) -> output::ContinueStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();

        Arc::new(output::ContinueStatementStruct { id, range })
    }

    pub(super) fn build_decimal_number_expression(
        &mut self,
        source: &input::DecimalNumberExpression,
    ) -> output::DecimalNumberExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let literal = self.build_decimal_literal(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.build_number_unit(value));

        Arc::new(output::DecimalNumberExpressionStruct {
            id,
            range,
            literal,
            unit,
        })
    }

    pub(super) fn build_do_while_statement(
        &mut self,
        source: &input::DoWhileStatement,
    ) -> output::DoWhileStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let body = self.build_statement(&source.body);
        let condition = self.build_expression(&source.condition);

        Arc::new(output::DoWhileStatementStruct {
            id,
            range,
            body,
            condition,
        })
    }

    pub(super) fn build_emit_statement(
        &mut self,
        source: &input::EmitStatement,
    ) -> output::EmitStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let event = self.build_identifier_path(&source.event);
        let arguments = self.build_arguments_declaration(&source.arguments);

        Arc::new(output::EmitStatementStruct {
            id,
            range,
            event,
            arguments,
        })
    }

    pub(super) fn build_enum_definition(
        &mut self,
        source: &input::EnumDefinition,
    ) -> output::EnumDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let members = self.build_enum_members(&source.members);

        Arc::new(output::EnumDefinitionStruct {
            id,
            range,
            name,
            members,
        })
    }

    pub(super) fn build_equality_expression(
        &mut self,
        source: &input::EqualityExpression,
    ) -> output::EqualityExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let operator = self.build_expression_equality_expression_operator(
            &source.expression_equality_expression_operator,
        );
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::EqualityExpressionStruct {
            id,
            range,
            left_operand,
            operator,
            right_operand,
        })
    }

    pub(super) fn build_experimental_pragma(
        &mut self,
        source: &input::ExperimentalPragma,
    ) -> output::ExperimentalPragma {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let feature = self.build_experimental_feature(&source.feature);

        Arc::new(output::ExperimentalPragmaStruct { id, range, feature })
    }

    pub(super) fn build_exponentiation_expression(
        &mut self,
        source: &input::ExponentiationExpression,
    ) -> output::ExponentiationExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::ExponentiationExpressionStruct {
            id,
            range,
            left_operand,
            right_operand,
        })
    }

    pub(super) fn build_expression_statement(
        &mut self,
        source: &input::ExpressionStatement,
    ) -> output::ExpressionStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let expression = self.build_expression(&source.expression);

        Arc::new(output::ExpressionStatementStruct {
            id,
            range,
            expression,
        })
    }

    pub(super) fn build_for_statement(
        &mut self,
        source: &input::ForStatement,
    ) -> output::ForStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let initialization = self.build_for_statement_initialization(&source.initialization);
        let condition = self.build_for_statement_condition(&source.condition);
        let iterator = source
            .iterator
            .as_ref()
            .map(|value| self.build_expression(value));
        let body = self.build_statement(&source.body);

        Arc::new(output::ForStatementStruct {
            id,
            range,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    pub(super) fn build_function_call_expression(
        &mut self,
        source: &input::FunctionCallExpression,
    ) -> output::FunctionCallExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let arguments = self.build_arguments_declaration(&source.arguments);

        Arc::new(output::FunctionCallExpressionStruct {
            id,
            range,
            operand,
            arguments,
        })
    }

    pub(super) fn build_hex_number_expression(
        &mut self,
        source: &input::HexNumberExpression,
    ) -> output::HexNumberExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let literal = self.build_hex_literal(&source.literal);

        Arc::new(output::HexNumberExpressionStruct { id, range, literal })
    }

    pub(super) fn build_if_statement(
        &mut self,
        source: &input::IfStatement,
    ) -> output::IfStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let condition = self.build_expression(&source.condition);
        let body = self.build_statement(&source.body);
        let else_branch = source
            .else_branch
            .as_ref()
            .map(|value| self.build_else_branch(value));

        Arc::new(output::IfStatementStruct {
            id,
            range,
            condition,
            body,
            else_branch,
        })
    }

    pub(super) fn build_import_deconstruction(
        &mut self,
        source: &input::ImportDeconstruction,
    ) -> output::ImportDeconstruction {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let symbols = self.build_import_deconstruction_symbols(&source.symbols);
        let path = self.build_string_literal(&source.path);

        Arc::new(output::ImportDeconstructionStruct {
            id,
            range,
            symbols,
            path,
        })
    }

    pub(super) fn build_import_deconstruction_symbol(
        &mut self,
        source: &input::ImportDeconstructionSymbol,
    ) -> output::ImportDeconstructionSymbol {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.build_import_alias(value));

        Arc::new(output::ImportDeconstructionSymbolStruct {
            id,
            range,
            name,
            alias,
        })
    }

    pub(super) fn build_inequality_expression(
        &mut self,
        source: &input::InequalityExpression,
    ) -> output::InequalityExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let operator = self.build_expression_inequality_expression_operator(
            &source.expression_inequality_expression_operator,
        );
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::InequalityExpressionStruct {
            id,
            range,
            left_operand,
            operator,
            right_operand,
        })
    }

    pub(super) fn build_inheritance_type(
        &mut self,
        source: &input::InheritanceType,
    ) -> output::InheritanceType {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_identifier_path(&source.type_name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.build_arguments_declaration(value));

        Arc::new(output::InheritanceTypeStruct {
            id,
            range,
            type_name,
            arguments,
        })
    }

    pub(super) fn build_interface_definition(
        &mut self,
        source: &input::InterfaceDefinition,
    ) -> output::InterfaceDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let inheritance = source
            .inheritance
            .as_ref()
            .map(|value| self.build_inheritance_specifier(value));
        let members = self.build_interface_members(&source.members);

        Arc::new(output::InterfaceDefinitionStruct {
            id,
            range,
            name,
            inheritance,
            members,
        })
    }

    pub(super) fn build_library_definition(
        &mut self,
        source: &input::LibraryDefinition,
    ) -> output::LibraryDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let members = self.build_library_members(&source.members);

        Arc::new(output::LibraryDefinitionStruct {
            id,
            range,
            name,
            members,
        })
    }

    pub(super) fn build_member_access_expression(
        &mut self,
        source: &input::MemberAccessExpression,
    ) -> output::MemberAccessExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let member = self.build_identifier_path_element(&source.member);

        Arc::new(output::MemberAccessExpressionStruct {
            id,
            range,
            operand,
            member,
        })
    }

    pub(super) fn build_modifier_invocation(
        &mut self,
        source: &input::ModifierInvocation,
    ) -> output::ModifierInvocation {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier_path(&source.name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.build_arguments_declaration(value));

        Arc::new(output::ModifierInvocationStruct {
            id,
            range,
            name,
            arguments,
        })
    }

    pub(super) fn build_multi_typed_declaration(
        &mut self,
        source: &input::MultiTypedDeclaration,
    ) -> output::MultiTypedDeclaration {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let elements = self.build_multi_typed_declaration_elements(&source.elements);
        let value = self.build_variable_declaration_value(&source.value);

        Arc::new(output::MultiTypedDeclarationStruct {
            id,
            range,
            elements,
            value,
        })
    }

    pub(super) fn build_multi_typed_declaration_element(
        &mut self,
        source: &input::MultiTypedDeclarationElement,
    ) -> output::MultiTypedDeclarationElement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let member = source
            .member
            .as_ref()
            .map(|value| self.build_variable_declaration(value));

        Arc::new(output::MultiTypedDeclarationElementStruct { id, range, member })
    }

    pub(super) fn build_multiplicative_expression(
        &mut self,
        source: &input::MultiplicativeExpression,
    ) -> output::MultiplicativeExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let operator = self.build_expression_multiplicative_expression_operator(
            &source.expression_multiplicative_expression_operator,
        );
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::MultiplicativeExpressionStruct {
            id,
            range,
            left_operand,
            operator,
            right_operand,
        })
    }

    pub(super) fn build_named_argument(
        &mut self,
        source: &input::NamedArgument,
    ) -> output::NamedArgument {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let value = self.build_expression(&source.value);

        Arc::new(output::NamedArgumentStruct {
            id,
            range,
            name,
            value,
        })
    }

    pub(super) fn build_new_expression(
        &mut self,
        source: &input::NewExpression,
    ) -> output::NewExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);

        Arc::new(output::NewExpressionStruct {
            id,
            range,
            type_name,
        })
    }

    pub(super) fn build_or_expression(
        &mut self,
        source: &input::OrExpression,
    ) -> output::OrExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::OrExpressionStruct {
            id,
            range,
            left_operand,
            right_operand,
        })
    }

    pub(super) fn build_path_import(&mut self, source: &input::PathImport) -> output::PathImport {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let path = self.build_string_literal(&source.path);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.build_import_alias(value));

        Arc::new(output::PathImportStruct {
            id,
            range,
            path,
            alias,
        })
    }

    pub(super) fn build_postfix_expression(
        &mut self,
        source: &input::PostfixExpression,
    ) -> output::PostfixExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let operator = self.build_expression_postfix_expression_operator(
            &source.expression_postfix_expression_operator,
        );

        Arc::new(output::PostfixExpressionStruct {
            id,
            range,
            operand,
            operator,
        })
    }

    pub(super) fn build_pragma_directive(
        &mut self,
        source: &input::PragmaDirective,
    ) -> output::PragmaDirective {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let pragma = self.build_pragma(&source.pragma);

        Arc::new(output::PragmaDirectiveStruct { id, range, pragma })
    }

    pub(super) fn build_prefix_expression(
        &mut self,
        source: &input::PrefixExpression,
    ) -> output::PrefixExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operator = self.build_expression_prefix_expression_operator(
            &source.expression_prefix_expression_operator,
        );
        let operand = self.build_expression(&source.operand);

        Arc::new(output::PrefixExpressionStruct {
            id,
            range,
            operator,
            operand,
        })
    }

    pub(super) fn build_return_statement(
        &mut self,
        source: &input::ReturnStatement,
    ) -> output::ReturnStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.build_expression(value));

        Arc::new(output::ReturnStatementStruct {
            id,
            range,
            expression,
        })
    }

    pub(super) fn build_revert_statement(
        &mut self,
        source: &input::RevertStatement,
    ) -> output::RevertStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let error = self.build_identifier_path(&source.error);
        let arguments = self.build_arguments_declaration(&source.arguments);

        Arc::new(output::RevertStatementStruct {
            id,
            range,
            error,
            arguments,
        })
    }

    pub(super) fn build_shift_expression(
        &mut self,
        source: &input::ShiftExpression,
    ) -> output::ShiftExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let operator = self.build_expression_shift_expression_operator(
            &source.expression_shift_expression_operator,
        );
        let right_operand = self.build_expression(&source.right_operand);

        Arc::new(output::ShiftExpressionStruct {
            id,
            range,
            left_operand,
            operator,
            right_operand,
        })
    }

    pub(super) fn build_single_typed_declaration(
        &mut self,
        source: &input::SingleTypedDeclaration,
    ) -> output::SingleTypedDeclaration {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let declaration = self.build_variable_declaration(&source.declaration);
        let value = source
            .value
            .as_ref()
            .map(|value| self.build_variable_declaration_value(value));

        Arc::new(output::SingleTypedDeclarationStruct {
            id,
            range,
            declaration,
            value,
        })
    }

    pub(super) fn build_source_unit(&mut self, source: &input::SourceUnit) -> output::SourceUnit {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let members = self.build_source_unit_members(&source.members);

        Arc::new(output::SourceUnitStruct { id, range, members })
    }

    pub(super) fn build_struct_definition(
        &mut self,
        source: &input::StructDefinition,
    ) -> output::StructDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let members = self.build_struct_members(&source.members);

        Arc::new(output::StructDefinitionStruct {
            id,
            range,
            name,
            members,
        })
    }

    pub(super) fn build_struct_member(
        &mut self,
        source: &input::StructMember,
    ) -> output::StructMember {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let name = self.build_identifier(&source.name);

        Arc::new(output::StructMemberStruct {
            id,
            range,
            type_name,
            name,
        })
    }

    pub(super) fn build_try_statement(
        &mut self,
        source: &input::TryStatement,
    ) -> output::TryStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let expression = self.build_expression(&source.expression);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.build_returns_declaration(value));
        let body = self.build_block(&source.body);
        let catch_clauses = self.build_catch_clauses(&source.catch_clauses);

        Arc::new(output::TryStatementStruct {
            id,
            range,
            expression,
            returns,
            body,
            catch_clauses,
        })
    }

    pub(super) fn build_tuple_expression(
        &mut self,
        source: &input::TupleExpression,
    ) -> output::TupleExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let items = self.build_tuple_values(&source.items);

        Arc::new(output::TupleExpressionStruct { id, range, items })
    }

    pub(super) fn build_tuple_value(&mut self, source: &input::TupleValue) -> output::TupleValue {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.build_expression(value));

        Arc::new(output::TupleValueStruct {
            id,
            range,
            expression,
        })
    }

    pub(super) fn build_type_expression(
        &mut self,
        source: &input::TypeExpression,
    ) -> output::TypeExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);

        Arc::new(output::TypeExpressionStruct {
            id,
            range,
            type_name,
        })
    }

    pub(super) fn build_unchecked_block(
        &mut self,
        source: &input::UncheckedBlock,
    ) -> output::UncheckedBlock {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let block = self.build_block(&source.block);

        Arc::new(output::UncheckedBlockStruct { id, range, block })
    }

    pub(super) fn build_user_defined_value_type_definition(
        &mut self,
        source: &input::UserDefinedValueTypeDefinition,
    ) -> output::UserDefinedValueTypeDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let value_type = self.build_elementary_type(&source.value_type);

        Arc::new(output::UserDefinedValueTypeDefinitionStruct {
            id,
            range,
            name,
            value_type,
        })
    }

    pub(super) fn build_using_deconstruction(
        &mut self,
        source: &input::UsingDeconstruction,
    ) -> output::UsingDeconstruction {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let symbols = self.build_using_deconstruction_symbols(&source.symbols);

        Arc::new(output::UsingDeconstructionStruct { id, range, symbols })
    }

    pub(super) fn build_using_deconstruction_symbol(
        &mut self,
        source: &input::UsingDeconstructionSymbol,
    ) -> output::UsingDeconstructionSymbol {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_identifier_path(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.build_using_alias(value));

        Arc::new(output::UsingDeconstructionSymbolStruct {
            id,
            range,
            name,
            alias,
        })
    }

    pub(super) fn build_using_directive(
        &mut self,
        source: &input::UsingDirective,
    ) -> output::UsingDirective {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let clause = self.build_using_clause(&source.clause);
        let target = self.build_using_target(&source.target);
        let global_keyword = source
            .global_keyword
            .as_ref()
            .map(|value| self.build_global_keyword(value));

        Arc::new(output::UsingDirectiveStruct {
            id,
            range,
            clause,
            target,
            global_keyword,
        })
    }

    pub(super) fn build_variable_declaration(
        &mut self,
        source: &input::VariableDeclaration,
    ) -> output::VariableDeclaration {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.build_storage_location(value));
        let name = self.build_identifier(&source.name);

        Arc::new(output::VariableDeclarationStruct {
            id,
            range,
            type_name,
            storage_location,
            name,
        })
    }

    pub(super) fn build_variable_declaration_statement(
        &mut self,
        source: &input::VariableDeclarationStatement,
    ) -> output::VariableDeclarationStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let target = self.build_variable_declaration_target(&source.target);

        Arc::new(output::VariableDeclarationStatementStruct { id, range, target })
    }

    pub(super) fn build_version_pragma(
        &mut self,
        source: &input::VersionPragma,
    ) -> output::VersionPragma {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let sets = self.build_version_expression_sets(&source.sets);

        Arc::new(output::VersionPragmaStruct { id, range, sets })
    }

    pub(super) fn build_version_range(
        &mut self,
        source: &input::VersionRange,
    ) -> output::VersionRange {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let start = self.build_version_literal(&source.start);
        let end = self.build_version_literal(&source.end);

        Arc::new(output::VersionRangeStruct {
            id,
            range,
            start,
            end,
        })
    }

    pub(super) fn build_version_term(
        &mut self,
        source: &input::VersionTerm,
    ) -> output::VersionTerm {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operator = source
            .operator
            .as_ref()
            .map(|value| self.build_version_operator(value));
        let literal = self.build_version_literal(&source.literal);

        Arc::new(output::VersionTermStruct {
            id,
            range,
            operator,
            literal,
        })
    }

    pub(super) fn build_while_statement(
        &mut self,
        source: &input::WhileStatement,
    ) -> output::WhileStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let condition = self.build_expression(&source.condition);
        let body = self.build_statement(&source.body);

        Arc::new(output::WhileStatementStruct {
            id,
            range,
            condition,
            body,
        })
    }

    pub(super) fn build_yul_block(&mut self, source: &input::YulBlock) -> output::YulBlock {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let statements = self.build_yul_statements(&source.statements);

        Arc::new(output::YulBlockStruct {
            id,
            range,
            statements,
        })
    }

    pub(super) fn build_yul_break_statement(
        &mut self,
        source: &input::YulBreakStatement,
    ) -> output::YulBreakStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();

        Arc::new(output::YulBreakStatementStruct { id, range })
    }

    pub(super) fn build_yul_continue_statement(
        &mut self,
        source: &input::YulContinueStatement,
    ) -> output::YulContinueStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();

        Arc::new(output::YulContinueStatementStruct { id, range })
    }

    pub(super) fn build_yul_default_case(
        &mut self,
        source: &input::YulDefaultCase,
    ) -> output::YulDefaultCase {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let body = self.build_yul_block(&source.body);

        Arc::new(output::YulDefaultCaseStruct { id, range, body })
    }

    pub(super) fn build_yul_for_statement(
        &mut self,
        source: &input::YulForStatement,
    ) -> output::YulForStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let initialization = self.build_yul_block(&source.initialization);
        let condition = self.build_yul_expression(&source.condition);
        let iterator = self.build_yul_block(&source.iterator);
        let body = self.build_yul_block(&source.body);

        Arc::new(output::YulForStatementStruct {
            id,
            range,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    pub(super) fn build_yul_function_call_expression(
        &mut self,
        source: &input::YulFunctionCallExpression,
    ) -> output::YulFunctionCallExpression {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let operand = self.build_yul_expression(&source.operand);
        let arguments = self.build_yul_arguments(&source.arguments);

        Arc::new(output::YulFunctionCallExpressionStruct {
            id,
            range,
            operand,
            arguments,
        })
    }

    pub(super) fn build_yul_function_definition(
        &mut self,
        source: &input::YulFunctionDefinition,
    ) -> output::YulFunctionDefinition {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let name = self.build_yul_identifier(&source.name);
        let parameters = self.build_yul_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.build_yul_returns_declaration(value));
        let body = self.build_yul_block(&source.body);

        Arc::new(output::YulFunctionDefinitionStruct {
            id,
            range,
            name,
            parameters,
            returns,
            body,
        })
    }

    pub(super) fn build_yul_if_statement(
        &mut self,
        source: &input::YulIfStatement,
    ) -> output::YulIfStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let condition = self.build_yul_expression(&source.condition);
        let body = self.build_yul_block(&source.body);

        Arc::new(output::YulIfStatementStruct {
            id,
            range,
            condition,
            body,
        })
    }

    pub(super) fn build_yul_leave_statement(
        &mut self,
        source: &input::YulLeaveStatement,
    ) -> output::YulLeaveStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();

        Arc::new(output::YulLeaveStatementStruct { id, range })
    }

    pub(super) fn build_yul_switch_statement(
        &mut self,
        source: &input::YulSwitchStatement,
    ) -> output::YulSwitchStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let expression = self.build_yul_expression(&source.expression);
        let cases = self.build_yul_switch_cases(&source.cases);

        Arc::new(output::YulSwitchStatementStruct {
            id,
            range,
            expression,
            cases,
        })
    }

    pub(super) fn build_yul_value_case(
        &mut self,
        source: &input::YulValueCase,
    ) -> output::YulValueCase {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let value = self.build_yul_literal(&source.value);
        let body = self.build_yul_block(&source.body);

        Arc::new(output::YulValueCaseStruct {
            id,
            range,
            value,
            body,
        })
    }

    pub(super) fn build_yul_variable_assignment_statement(
        &mut self,
        source: &input::YulVariableAssignmentStatement,
    ) -> output::YulVariableAssignmentStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let variables = self.build_yul_paths(&source.variables);
        let expression = self.build_yul_expression(&source.expression);

        Arc::new(output::YulVariableAssignmentStatementStruct {
            id,
            range,
            variables,
            expression,
        })
    }

    pub(super) fn build_yul_variable_declaration_statement(
        &mut self,
        source: &input::YulVariableDeclarationStatement,
    ) -> output::YulVariableDeclarationStatement {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let variables = self.build_yul_variable_names(&source.variables);
        let value = source
            .value
            .as_ref()
            .map(|value| self.build_yul_variable_declaration_value(value));

        Arc::new(output::YulVariableDeclarationStatementStruct {
            id,
            range,
            variables,
            value,
        })
    }

    pub(super) fn build_yul_variable_declaration_value(
        &mut self,
        source: &input::YulVariableDeclarationValue,
    ) -> output::YulVariableDeclarationValue {
        let id = self.next_id();
        let range = source.calculate_text_range().unwrap_or_default();
        let expression = self.build_yul_expression(&source.expression);

        Arc::new(output::YulVariableDeclarationValueStruct {
            id,
            range,
            expression,
        })
    }

    //
    // Collapsed sequences
    //

    pub(super) fn build_else_branch(&mut self, source: &input::ElseBranch) -> output::Statement {
        self.build_statement(&source.body)
    }

    pub(super) fn build_import_alias(&mut self, source: &input::ImportAlias) -> output::Identifier {
        self.build_identifier(&source.identifier)
    }

    pub(super) fn build_import_directive(
        &mut self,
        source: &input::ImportDirective,
    ) -> output::ImportClause {
        self.build_import_clause(&source.clause)
    }

    pub(super) fn build_inheritance_specifier(
        &mut self,
        source: &input::InheritanceSpecifier,
    ) -> output::InheritanceTypes {
        self.build_inheritance_types(&source.types)
    }

    pub(super) fn build_named_argument_group(
        &mut self,
        source: &input::NamedArgumentGroup,
    ) -> output::NamedArguments {
        self.build_named_arguments(&source.arguments)
    }

    pub(super) fn build_override_paths_declaration(
        &mut self,
        source: &input::OverridePathsDeclaration,
    ) -> output::OverridePaths {
        self.build_override_paths(&source.paths)
    }

    pub(super) fn build_parameters_declaration(
        &mut self,
        source: &input::ParametersDeclaration,
    ) -> output::Parameters {
        self.build_parameters(&source.parameters)
    }

    pub(super) fn build_returns_declaration(
        &mut self,
        source: &input::ReturnsDeclaration,
    ) -> output::Parameters {
        self.build_parameters_declaration(&source.variables)
    }

    pub(super) fn build_state_variable_definition_value(
        &mut self,
        source: &input::StateVariableDefinitionValue,
    ) -> output::Expression {
        self.build_expression(&source.value)
    }

    pub(super) fn build_storage_layout_specifier(
        &mut self,
        source: &input::StorageLayoutSpecifier,
    ) -> output::Expression {
        self.build_expression(&source.expression)
    }

    pub(super) fn build_using_alias(
        &mut self,
        source: &input::UsingAlias,
    ) -> output::UsingOperator {
        self.build_using_operator(&source.operator)
    }

    pub(super) fn build_variable_declaration_value(
        &mut self,
        source: &input::VariableDeclarationValue,
    ) -> output::Expression {
        self.build_expression(&source.expression)
    }

    pub(super) fn build_yul_flags_declaration(
        &mut self,
        source: &input::YulFlagsDeclaration,
    ) -> output::YulFlags {
        self.build_yul_flags(&source.flags)
    }

    pub(super) fn build_yul_parameters_declaration(
        &mut self,
        source: &input::YulParametersDeclaration,
    ) -> output::YulParameters {
        self.build_yul_parameters(&source.parameters)
    }

    pub(super) fn build_yul_returns_declaration(
        &mut self,
        source: &input::YulReturnsDeclaration,
    ) -> output::YulVariableNames {
        self.build_yul_variable_names(&source.variables)
    }

    //
    // Choices
    //

    #[allow(clippy::unused_self)]
    pub(super) fn build_abicoder_version(
        &mut self,
        source: &input::AbicoderVersion,
    ) -> output::AbicoderVersion {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::AbicoderVersion::AbicoderV1Keyword(ref abicoder_v1_keyword) => {
                output::AbicoderVersion::AbicoderV1Keyword(
                    self.build_abicoder_v1_keyword(abicoder_v1_keyword),
                )
            }
            input::AbicoderVersion::AbicoderV2Keyword(ref abicoder_v2_keyword) => {
                output::AbicoderVersion::AbicoderV2Keyword(
                    self.build_abicoder_v2_keyword(abicoder_v2_keyword),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_additive_expression_operator(
        &mut self,
        source: &input::Expression_AdditiveExpression_Operator,
    ) -> output::AdditiveExpressionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_AdditiveExpression_Operator::Minus(ref minus) => {
                output::AdditiveExpressionOperator::Minus(self.build_minus(minus))
            }
            input::Expression_AdditiveExpression_Operator::Plus(ref plus) => {
                output::AdditiveExpressionOperator::Plus(self.build_plus(plus))
            }
        }
    }

    #[allow(clippy::unused_self)]
    #[allow(dead_code)]
    pub(super) fn default_build_arguments_declaration(
        &mut self,
        source: &input::ArgumentsDeclaration,
    ) -> output::ArgumentsDeclaration {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            _ => panic!("Unexpected variant {source:?}"),
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_assignment_expression_operator(
        &mut self,
        source: &input::Expression_AssignmentExpression_Operator,
    ) -> output::AssignmentExpressionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
          input::Expression_AssignmentExpression_Operator::AmpersandEqual(ref ampersand_equal) => {
                output::AssignmentExpressionOperator::AmpersandEqual(self.build_ampersand_equal(ampersand_equal))
              }input::Expression_AssignmentExpression_Operator::AsteriskEqual(ref asterisk_equal) => {
                output::AssignmentExpressionOperator::AsteriskEqual(self.build_asterisk_equal(asterisk_equal))
              }input::Expression_AssignmentExpression_Operator::BarEqual(ref bar_equal) => {
                output::AssignmentExpressionOperator::BarEqual(self.build_bar_equal(bar_equal))
              }input::Expression_AssignmentExpression_Operator::CaretEqual(ref caret_equal) => {
                output::AssignmentExpressionOperator::CaretEqual(self.build_caret_equal(caret_equal))
              }input::Expression_AssignmentExpression_Operator::Equal(ref equal) => {
                output::AssignmentExpressionOperator::Equal(self.build_equal(equal))
              }input::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(ref greater_than_greater_than_equal) => {
                output::AssignmentExpressionOperator::GreaterThanGreaterThanEqual(self.build_greater_than_greater_than_equal(greater_than_greater_than_equal))
              }input::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(ref greater_than_greater_than_greater_than_equal) => {
                output::AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(self.build_greater_than_greater_than_greater_than_equal(greater_than_greater_than_greater_than_equal))
              }input::Expression_AssignmentExpression_Operator::LessThanLessThanEqual(ref less_than_less_than_equal) => {
                output::AssignmentExpressionOperator::LessThanLessThanEqual(self.build_less_than_less_than_equal(less_than_less_than_equal))
              }input::Expression_AssignmentExpression_Operator::MinusEqual(ref minus_equal) => {
                output::AssignmentExpressionOperator::MinusEqual(self.build_minus_equal(minus_equal))
              }input::Expression_AssignmentExpression_Operator::PercentEqual(ref percent_equal) => {
                output::AssignmentExpressionOperator::PercentEqual(self.build_percent_equal(percent_equal))
              }input::Expression_AssignmentExpression_Operator::PlusEqual(ref plus_equal) => {
                output::AssignmentExpressionOperator::PlusEqual(self.build_plus_equal(plus_equal))
              }input::Expression_AssignmentExpression_Operator::SlashEqual(ref slash_equal) => {
                output::AssignmentExpressionOperator::SlashEqual(self.build_slash_equal(slash_equal))
              }}
    }

    #[allow(clippy::unused_self)]
    #[allow(dead_code)]
    pub(super) fn default_build_contract_member(
        &mut self,
        source: &input::ContractMember,
    ) -> output::ContractMember {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::ContractMember::UsingDirective(ref using_directive) => {
                output::ContractMember::UsingDirective(self.build_using_directive(using_directive))
            }
            input::ContractMember::FunctionDefinition(ref function_definition) => {
                output::ContractMember::FunctionDefinition(
                    self.build_function_definition(function_definition),
                )
            }
            input::ContractMember::StructDefinition(ref struct_definition) => {
                output::ContractMember::StructDefinition(
                    self.build_struct_definition(struct_definition),
                )
            }
            input::ContractMember::EnumDefinition(ref enum_definition) => {
                output::ContractMember::EnumDefinition(self.build_enum_definition(enum_definition))
            }
            input::ContractMember::EventDefinition(ref event_definition) => {
                output::ContractMember::EventDefinition(
                    self.build_event_definition(event_definition),
                )
            }
            input::ContractMember::ErrorDefinition(ref error_definition) => {
                output::ContractMember::ErrorDefinition(
                    self.build_error_definition(error_definition),
                )
            }
            input::ContractMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => output::ContractMember::UserDefinedValueTypeDefinition(
                self.build_user_defined_value_type_definition(user_defined_value_type_definition),
            ),
            input::ContractMember::StateVariableDefinition(ref state_variable_definition) => {
                output::ContractMember::StateVariableDefinition(
                    self.build_state_variable_definition(state_variable_definition),
                )
            }
            _ => panic!("Unexpected variant {source:?}"),
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_elementary_type(
        &mut self,
        source: &input::ElementaryType,
    ) -> output::ElementaryType {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::ElementaryType::BoolKeyword(ref bool_keyword) => {
                output::ElementaryType::BoolKeyword(self.build_bool_keyword(bool_keyword))
            }
            input::ElementaryType::StringKeyword(ref string_keyword) => {
                output::ElementaryType::StringKeyword(self.build_string_keyword(string_keyword))
            }
            input::ElementaryType::AddressType(ref address_type) => {
                output::ElementaryType::AddressType(self.build_address_type(address_type))
            }
            input::ElementaryType::BytesKeyword(ref bytes_keyword) => {
                output::ElementaryType::BytesKeyword(self.build_bytes_keyword(bytes_keyword))
            }
            input::ElementaryType::IntKeyword(ref int_keyword) => {
                output::ElementaryType::IntKeyword(self.build_int_keyword(int_keyword))
            }
            input::ElementaryType::UintKeyword(ref uint_keyword) => {
                output::ElementaryType::UintKeyword(self.build_uint_keyword(uint_keyword))
            }
            input::ElementaryType::FixedKeyword(ref fixed_keyword) => {
                output::ElementaryType::FixedKeyword(self.build_fixed_keyword(fixed_keyword))
            }
            input::ElementaryType::UfixedKeyword(ref ufixed_keyword) => {
                output::ElementaryType::UfixedKeyword(self.build_ufixed_keyword(ufixed_keyword))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_equality_expression_operator(
        &mut self,
        source: &input::Expression_EqualityExpression_Operator,
    ) -> output::EqualityExpressionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_EqualityExpression_Operator::BangEqual(ref bang_equal) => {
                output::EqualityExpressionOperator::BangEqual(self.build_bang_equal(bang_equal))
            }
            input::Expression_EqualityExpression_Operator::EqualEqual(ref equal_equal) => {
                output::EqualityExpressionOperator::EqualEqual(self.build_equal_equal(equal_equal))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_experimental_feature(
        &mut self,
        source: &input::ExperimentalFeature,
    ) -> output::ExperimentalFeature {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::ExperimentalFeature::ABIEncoderV2Keyword(ref abi_encoder_v2_keyword) => {
                output::ExperimentalFeature::ABIEncoderV2Keyword(
                    self.build_abi_encoder_v2_keyword(abi_encoder_v2_keyword),
                )
            }
            input::ExperimentalFeature::SMTCheckerKeyword(ref smt_checker_keyword) => {
                output::ExperimentalFeature::SMTCheckerKeyword(
                    self.build_smt_checker_keyword(smt_checker_keyword),
                )
            }
            input::ExperimentalFeature::PragmaStringLiteral(ref pragma_string_literal) => {
                output::ExperimentalFeature::StringLiteral(
                    self.build_pragma_string_literal(pragma_string_literal),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression(&mut self, source: &input::Expression) -> output::Expression {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression::AssignmentExpression(ref assignment_expression) => {
                output::Expression::AssignmentExpression(
                    self.build_assignment_expression(assignment_expression),
                )
            }
            input::Expression::ConditionalExpression(ref conditional_expression) => {
                output::Expression::ConditionalExpression(
                    self.build_conditional_expression(conditional_expression),
                )
            }
            input::Expression::OrExpression(ref or_expression) => {
                output::Expression::OrExpression(self.build_or_expression(or_expression))
            }
            input::Expression::AndExpression(ref and_expression) => {
                output::Expression::AndExpression(self.build_and_expression(and_expression))
            }
            input::Expression::EqualityExpression(ref equality_expression) => {
                output::Expression::EqualityExpression(
                    self.build_equality_expression(equality_expression),
                )
            }
            input::Expression::InequalityExpression(ref inequality_expression) => {
                output::Expression::InequalityExpression(
                    self.build_inequality_expression(inequality_expression),
                )
            }
            input::Expression::BitwiseOrExpression(ref bitwise_or_expression) => {
                output::Expression::BitwiseOrExpression(
                    self.build_bitwise_or_expression(bitwise_or_expression),
                )
            }
            input::Expression::BitwiseXorExpression(ref bitwise_xor_expression) => {
                output::Expression::BitwiseXorExpression(
                    self.build_bitwise_xor_expression(bitwise_xor_expression),
                )
            }
            input::Expression::BitwiseAndExpression(ref bitwise_and_expression) => {
                output::Expression::BitwiseAndExpression(
                    self.build_bitwise_and_expression(bitwise_and_expression),
                )
            }
            input::Expression::ShiftExpression(ref shift_expression) => {
                output::Expression::ShiftExpression(self.build_shift_expression(shift_expression))
            }
            input::Expression::AdditiveExpression(ref additive_expression) => {
                output::Expression::AdditiveExpression(
                    self.build_additive_expression(additive_expression),
                )
            }
            input::Expression::MultiplicativeExpression(ref multiplicative_expression) => {
                output::Expression::MultiplicativeExpression(
                    self.build_multiplicative_expression(multiplicative_expression),
                )
            }
            input::Expression::ExponentiationExpression(ref exponentiation_expression) => {
                output::Expression::ExponentiationExpression(
                    self.build_exponentiation_expression(exponentiation_expression),
                )
            }
            input::Expression::PostfixExpression(ref postfix_expression) => {
                output::Expression::PostfixExpression(
                    self.build_postfix_expression(postfix_expression),
                )
            }
            input::Expression::PrefixExpression(ref prefix_expression) => {
                output::Expression::PrefixExpression(
                    self.build_prefix_expression(prefix_expression),
                )
            }
            input::Expression::FunctionCallExpression(ref function_call_expression) => {
                output::Expression::FunctionCallExpression(
                    self.build_function_call_expression(function_call_expression),
                )
            }
            input::Expression::CallOptionsExpression(ref call_options_expression) => {
                output::Expression::CallOptionsExpression(
                    self.build_call_options_expression(call_options_expression),
                )
            }
            input::Expression::MemberAccessExpression(ref member_access_expression) => {
                output::Expression::MemberAccessExpression(
                    self.build_member_access_expression(member_access_expression),
                )
            }
            input::Expression::IndexAccessExpression(ref index_access_expression) => {
                output::Expression::IndexAccessExpression(
                    self.build_index_access_expression(index_access_expression),
                )
            }
            input::Expression::NewExpression(ref new_expression) => {
                output::Expression::NewExpression(self.build_new_expression(new_expression))
            }
            input::Expression::TupleExpression(ref tuple_expression) => {
                output::Expression::TupleExpression(self.build_tuple_expression(tuple_expression))
            }
            input::Expression::TypeExpression(ref type_expression) => {
                output::Expression::TypeExpression(self.build_type_expression(type_expression))
            }
            input::Expression::ArrayExpression(ref array_expression) => {
                output::Expression::ArrayExpression(self.build_array_expression(array_expression))
            }
            input::Expression::HexNumberExpression(ref hex_number_expression) => {
                output::Expression::HexNumberExpression(
                    self.build_hex_number_expression(hex_number_expression),
                )
            }
            input::Expression::DecimalNumberExpression(ref decimal_number_expression) => {
                output::Expression::DecimalNumberExpression(
                    self.build_decimal_number_expression(decimal_number_expression),
                )
            }
            input::Expression::StringExpression(ref string_expression) => {
                output::Expression::StringExpression(
                    self.build_string_expression(string_expression),
                )
            }
            input::Expression::ElementaryType(ref elementary_type) => {
                output::Expression::ElementaryType(self.build_elementary_type(elementary_type))
            }
            input::Expression::PayableKeyword(ref payable_keyword) => {
                output::Expression::PayableKeyword(self.build_payable_keyword(payable_keyword))
            }
            input::Expression::ThisKeyword(ref this_keyword) => {
                output::Expression::ThisKeyword(self.build_this_keyword(this_keyword))
            }
            input::Expression::SuperKeyword(ref super_keyword) => {
                output::Expression::SuperKeyword(self.build_super_keyword(super_keyword))
            }
            input::Expression::TrueKeyword(ref true_keyword) => {
                output::Expression::TrueKeyword(self.build_true_keyword(true_keyword))
            }
            input::Expression::FalseKeyword(ref false_keyword) => {
                output::Expression::FalseKeyword(self.build_false_keyword(false_keyword))
            }
            input::Expression::Identifier(ref identifier) => {
                output::Expression::Identifier(self.build_identifier(identifier))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_for_statement_condition(
        &mut self,
        source: &input::ForStatementCondition,
    ) -> output::ForStatementCondition {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::ForStatementCondition::ExpressionStatement(ref expression_statement) => {
                output::ForStatementCondition::ExpressionStatement(
                    self.build_expression_statement(expression_statement),
                )
            }
            input::ForStatementCondition::Semicolon(ref semicolon) => {
                output::ForStatementCondition::Semicolon(self.build_semicolon(semicolon))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_for_statement_initialization(
        &mut self,
        source: &input::ForStatementInitialization,
    ) -> output::ForStatementInitialization {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::ForStatementInitialization::VariableDeclarationStatement(
                ref variable_declaration_statement,
            ) => output::ForStatementInitialization::VariableDeclarationStatement(
                self.build_variable_declaration_statement(variable_declaration_statement),
            ),
            input::ForStatementInitialization::ExpressionStatement(ref expression_statement) => {
                output::ForStatementInitialization::ExpressionStatement(
                    self.build_expression_statement(expression_statement),
                )
            }
            input::ForStatementInitialization::Semicolon(ref semicolon) => {
                output::ForStatementInitialization::Semicolon(self.build_semicolon(semicolon))
            }
        }
    }

    #[allow(clippy::unused_self)]
    #[allow(dead_code)]
    pub(super) fn default_build_import_clause(
        &mut self,
        source: &input::ImportClause,
    ) -> output::ImportClause {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::ImportClause::PathImport(ref path_import) => {
                output::ImportClause::PathImport(self.build_path_import(path_import))
            }
            input::ImportClause::ImportDeconstruction(ref import_deconstruction) => {
                output::ImportClause::ImportDeconstruction(
                    self.build_import_deconstruction(import_deconstruction),
                )
            }
            _ => panic!("Unexpected variant {source:?}"),
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_inequality_expression_operator(
        &mut self,
        source: &input::Expression_InequalityExpression_Operator,
    ) -> output::InequalityExpressionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_InequalityExpression_Operator::GreaterThan(ref greater_than) => {
                output::InequalityExpressionOperator::GreaterThan(
                    self.build_greater_than(greater_than),
                )
            }
            input::Expression_InequalityExpression_Operator::GreaterThanEqual(
                ref greater_than_equal,
            ) => output::InequalityExpressionOperator::GreaterThanEqual(
                self.build_greater_than_equal(greater_than_equal),
            ),
            input::Expression_InequalityExpression_Operator::LessThan(ref less_than) => {
                output::InequalityExpressionOperator::LessThan(self.build_less_than(less_than))
            }
            input::Expression_InequalityExpression_Operator::LessThanEqual(ref less_than_equal) => {
                output::InequalityExpressionOperator::LessThanEqual(
                    self.build_less_than_equal(less_than_equal),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_multiplicative_expression_operator(
        &mut self,
        source: &input::Expression_MultiplicativeExpression_Operator,
    ) -> output::MultiplicativeExpressionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_MultiplicativeExpression_Operator::Asterisk(ref asterisk) => {
                output::MultiplicativeExpressionOperator::Asterisk(self.build_asterisk(asterisk))
            }
            input::Expression_MultiplicativeExpression_Operator::Percent(ref percent) => {
                output::MultiplicativeExpressionOperator::Percent(self.build_percent(percent))
            }
            input::Expression_MultiplicativeExpression_Operator::Slash(ref slash) => {
                output::MultiplicativeExpressionOperator::Slash(self.build_slash(slash))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_number_unit(&mut self, source: &input::NumberUnit) -> output::NumberUnit {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::NumberUnit::WeiKeyword(ref wei_keyword) => {
                output::NumberUnit::WeiKeyword(self.build_wei_keyword(wei_keyword))
            }
            input::NumberUnit::GweiKeyword(ref gwei_keyword) => {
                output::NumberUnit::GweiKeyword(self.build_gwei_keyword(gwei_keyword))
            }
            input::NumberUnit::EtherKeyword(ref ether_keyword) => {
                output::NumberUnit::EtherKeyword(self.build_ether_keyword(ether_keyword))
            }
            input::NumberUnit::SecondsKeyword(ref seconds_keyword) => {
                output::NumberUnit::SecondsKeyword(self.build_seconds_keyword(seconds_keyword))
            }
            input::NumberUnit::MinutesKeyword(ref minutes_keyword) => {
                output::NumberUnit::MinutesKeyword(self.build_minutes_keyword(minutes_keyword))
            }
            input::NumberUnit::HoursKeyword(ref hours_keyword) => {
                output::NumberUnit::HoursKeyword(self.build_hours_keyword(hours_keyword))
            }
            input::NumberUnit::DaysKeyword(ref days_keyword) => {
                output::NumberUnit::DaysKeyword(self.build_days_keyword(days_keyword))
            }
            input::NumberUnit::WeeksKeyword(ref weeks_keyword) => {
                output::NumberUnit::WeeksKeyword(self.build_weeks_keyword(weeks_keyword))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_postfix_expression_operator(
        &mut self,
        source: &input::Expression_PostfixExpression_Operator,
    ) -> output::PostfixExpressionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_PostfixExpression_Operator::MinusMinus(ref minus_minus) => {
                output::PostfixExpressionOperator::MinusMinus(self.build_minus_minus(minus_minus))
            }
            input::Expression_PostfixExpression_Operator::PlusPlus(ref plus_plus) => {
                output::PostfixExpressionOperator::PlusPlus(self.build_plus_plus(plus_plus))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_pragma(&mut self, source: &input::Pragma) -> output::Pragma {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Pragma::VersionPragma(ref version_pragma) => {
                output::Pragma::VersionPragma(self.build_version_pragma(version_pragma))
            }
            input::Pragma::AbicoderPragma(ref abicoder_pragma) => {
                output::Pragma::AbicoderPragma(self.build_abicoder_pragma(abicoder_pragma))
            }
            input::Pragma::ExperimentalPragma(ref experimental_pragma) => {
                output::Pragma::ExperimentalPragma(
                    self.build_experimental_pragma(experimental_pragma),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_prefix_expression_operator(
        &mut self,
        source: &input::Expression_PrefixExpression_Operator,
    ) -> output::PrefixExpressionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_PrefixExpression_Operator::Bang(ref bang) => {
                output::PrefixExpressionOperator::Bang(self.build_bang(bang))
            }
            input::Expression_PrefixExpression_Operator::DeleteKeyword(ref delete_keyword) => {
                output::PrefixExpressionOperator::DeleteKeyword(
                    self.build_delete_keyword(delete_keyword),
                )
            }
            input::Expression_PrefixExpression_Operator::Minus(ref minus) => {
                output::PrefixExpressionOperator::Minus(self.build_minus(minus))
            }
            input::Expression_PrefixExpression_Operator::MinusMinus(ref minus_minus) => {
                output::PrefixExpressionOperator::MinusMinus(self.build_minus_minus(minus_minus))
            }
            input::Expression_PrefixExpression_Operator::PlusPlus(ref plus_plus) => {
                output::PrefixExpressionOperator::PlusPlus(self.build_plus_plus(plus_plus))
            }
            input::Expression_PrefixExpression_Operator::Tilde(ref tilde) => {
                output::PrefixExpressionOperator::Tilde(self.build_tilde(tilde))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_shift_expression_operator(
        &mut self,
        source: &input::Expression_ShiftExpression_Operator,
    ) -> output::ShiftExpressionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_ShiftExpression_Operator::GreaterThanGreaterThan(
                ref greater_than_greater_than,
            ) => output::ShiftExpressionOperator::GreaterThanGreaterThan(
                self.build_greater_than_greater_than(greater_than_greater_than),
            ),
            input::Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(
                ref greater_than_greater_than_greater_than,
            ) => output::ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(
                self.build_greater_than_greater_than_greater_than(
                    greater_than_greater_than_greater_than,
                ),
            ),
            input::Expression_ShiftExpression_Operator::LessThanLessThan(
                ref less_than_less_than,
            ) => output::ShiftExpressionOperator::LessThanLessThan(
                self.build_less_than_less_than(less_than_less_than),
            ),
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_source_unit_member(
        &mut self,
        source: &input::SourceUnitMember,
    ) -> output::SourceUnitMember {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::SourceUnitMember::PragmaDirective(ref pragma_directive) => {
                output::SourceUnitMember::PragmaDirective(
                    self.build_pragma_directive(pragma_directive),
                )
            }
            input::SourceUnitMember::ImportDirective(ref import_directive) => {
                output::SourceUnitMember::ImportClause(
                    self.build_import_directive(import_directive),
                )
            }
            input::SourceUnitMember::ContractDefinition(ref contract_definition) => {
                output::SourceUnitMember::ContractDefinition(
                    self.build_contract_definition(contract_definition),
                )
            }
            input::SourceUnitMember::InterfaceDefinition(ref interface_definition) => {
                output::SourceUnitMember::InterfaceDefinition(
                    self.build_interface_definition(interface_definition),
                )
            }
            input::SourceUnitMember::LibraryDefinition(ref library_definition) => {
                output::SourceUnitMember::LibraryDefinition(
                    self.build_library_definition(library_definition),
                )
            }
            input::SourceUnitMember::StructDefinition(ref struct_definition) => {
                output::SourceUnitMember::StructDefinition(
                    self.build_struct_definition(struct_definition),
                )
            }
            input::SourceUnitMember::EnumDefinition(ref enum_definition) => {
                output::SourceUnitMember::EnumDefinition(
                    self.build_enum_definition(enum_definition),
                )
            }
            input::SourceUnitMember::FunctionDefinition(ref function_definition) => {
                output::SourceUnitMember::FunctionDefinition(
                    self.build_function_definition(function_definition),
                )
            }
            input::SourceUnitMember::ErrorDefinition(ref error_definition) => {
                output::SourceUnitMember::ErrorDefinition(
                    self.build_error_definition(error_definition),
                )
            }
            input::SourceUnitMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => output::SourceUnitMember::UserDefinedValueTypeDefinition(
                self.build_user_defined_value_type_definition(user_defined_value_type_definition),
            ),
            input::SourceUnitMember::UsingDirective(ref using_directive) => {
                output::SourceUnitMember::UsingDirective(
                    self.build_using_directive(using_directive),
                )
            }
            input::SourceUnitMember::EventDefinition(ref event_definition) => {
                output::SourceUnitMember::EventDefinition(
                    self.build_event_definition(event_definition),
                )
            }
            input::SourceUnitMember::ConstantDefinition(ref constant_definition) => {
                output::SourceUnitMember::ConstantDefinition(
                    self.build_constant_definition(constant_definition),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_statement(&mut self, source: &input::Statement) -> output::Statement {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Statement::IfStatement(ref if_statement) => {
                output::Statement::IfStatement(self.build_if_statement(if_statement))
            }
            input::Statement::ForStatement(ref for_statement) => {
                output::Statement::ForStatement(self.build_for_statement(for_statement))
            }
            input::Statement::WhileStatement(ref while_statement) => {
                output::Statement::WhileStatement(self.build_while_statement(while_statement))
            }
            input::Statement::DoWhileStatement(ref do_while_statement) => {
                output::Statement::DoWhileStatement(
                    self.build_do_while_statement(do_while_statement),
                )
            }
            input::Statement::ContinueStatement(ref continue_statement) => {
                output::Statement::ContinueStatement(
                    self.build_continue_statement(continue_statement),
                )
            }
            input::Statement::BreakStatement(ref break_statement) => {
                output::Statement::BreakStatement(self.build_break_statement(break_statement))
            }
            input::Statement::ReturnStatement(ref return_statement) => {
                output::Statement::ReturnStatement(self.build_return_statement(return_statement))
            }
            input::Statement::EmitStatement(ref emit_statement) => {
                output::Statement::EmitStatement(self.build_emit_statement(emit_statement))
            }
            input::Statement::TryStatement(ref try_statement) => {
                output::Statement::TryStatement(self.build_try_statement(try_statement))
            }
            input::Statement::RevertStatement(ref revert_statement) => {
                output::Statement::RevertStatement(self.build_revert_statement(revert_statement))
            }
            input::Statement::AssemblyStatement(ref assembly_statement) => {
                output::Statement::AssemblyStatement(
                    self.build_assembly_statement(assembly_statement),
                )
            }
            input::Statement::Block(ref block) => output::Statement::Block(self.build_block(block)),
            input::Statement::UncheckedBlock(ref unchecked_block) => {
                output::Statement::UncheckedBlock(self.build_unchecked_block(unchecked_block))
            }
            input::Statement::VariableDeclarationStatement(ref variable_declaration_statement) => {
                output::Statement::VariableDeclarationStatement(
                    self.build_variable_declaration_statement(variable_declaration_statement),
                )
            }
            input::Statement::ExpressionStatement(ref expression_statement) => {
                output::Statement::ExpressionStatement(
                    self.build_expression_statement(expression_statement),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_storage_location(
        &mut self,
        source: &input::StorageLocation,
    ) -> output::StorageLocation {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::StorageLocation::MemoryKeyword(ref memory_keyword) => {
                output::StorageLocation::MemoryKeyword(self.build_memory_keyword(memory_keyword))
            }
            input::StorageLocation::StorageKeyword(ref storage_keyword) => {
                output::StorageLocation::StorageKeyword(self.build_storage_keyword(storage_keyword))
            }
            input::StorageLocation::CallDataKeyword(ref call_data_keyword) => {
                output::StorageLocation::CallDataKeyword(
                    self.build_call_data_keyword(call_data_keyword),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_string_expression(
        &mut self,
        source: &input::StringExpression,
    ) -> output::StringExpression {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::StringExpression::StringLiterals(ref string_literals) => {
                output::StringExpression::StringLiterals(
                    self.build_string_literals(string_literals),
                )
            }
            input::StringExpression::HexStringLiterals(ref hex_string_literals) => {
                output::StringExpression::HexStringLiterals(
                    self.build_hex_string_literals(hex_string_literals),
                )
            }
            input::StringExpression::UnicodeStringLiterals(ref unicode_string_literals) => {
                output::StringExpression::UnicodeStringLiterals(
                    self.build_unicode_string_literals(unicode_string_literals),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_type_name(&mut self, source: &input::TypeName) -> output::TypeName {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::TypeName::ArrayTypeName(ref array_type_name) => {
                output::TypeName::ArrayTypeName(self.build_array_type_name(array_type_name))
            }
            input::TypeName::FunctionType(ref function_type) => {
                output::TypeName::FunctionType(self.build_function_type(function_type))
            }
            input::TypeName::MappingType(ref mapping_type) => {
                output::TypeName::MappingType(self.build_mapping_type(mapping_type))
            }
            input::TypeName::ElementaryType(ref elementary_type) => {
                output::TypeName::ElementaryType(self.build_elementary_type(elementary_type))
            }
            input::TypeName::IdentifierPath(ref identifier_path) => {
                output::TypeName::IdentifierPath(self.build_identifier_path(identifier_path))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_using_clause(
        &mut self,
        source: &input::UsingClause,
    ) -> output::UsingClause {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::UsingClause::IdentifierPath(ref identifier_path) => {
                output::UsingClause::IdentifierPath(self.build_identifier_path(identifier_path))
            }
            input::UsingClause::UsingDeconstruction(ref using_deconstruction) => {
                output::UsingClause::UsingDeconstruction(
                    self.build_using_deconstruction(using_deconstruction),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_using_operator(
        &mut self,
        source: &input::UsingOperator,
    ) -> output::UsingOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::UsingOperator::Ampersand(ref ampersand) => {
                output::UsingOperator::Ampersand(self.build_ampersand(ampersand))
            }
            input::UsingOperator::Asterisk(ref asterisk) => {
                output::UsingOperator::Asterisk(self.build_asterisk(asterisk))
            }
            input::UsingOperator::BangEqual(ref bang_equal) => {
                output::UsingOperator::BangEqual(self.build_bang_equal(bang_equal))
            }
            input::UsingOperator::Bar(ref bar) => output::UsingOperator::Bar(self.build_bar(bar)),
            input::UsingOperator::Caret(ref caret) => {
                output::UsingOperator::Caret(self.build_caret(caret))
            }
            input::UsingOperator::EqualEqual(ref equal_equal) => {
                output::UsingOperator::EqualEqual(self.build_equal_equal(equal_equal))
            }
            input::UsingOperator::GreaterThan(ref greater_than) => {
                output::UsingOperator::GreaterThan(self.build_greater_than(greater_than))
            }
            input::UsingOperator::GreaterThanEqual(ref greater_than_equal) => {
                output::UsingOperator::GreaterThanEqual(
                    self.build_greater_than_equal(greater_than_equal),
                )
            }
            input::UsingOperator::LessThan(ref less_than) => {
                output::UsingOperator::LessThan(self.build_less_than(less_than))
            }
            input::UsingOperator::LessThanEqual(ref less_than_equal) => {
                output::UsingOperator::LessThanEqual(self.build_less_than_equal(less_than_equal))
            }
            input::UsingOperator::Minus(ref minus) => {
                output::UsingOperator::Minus(self.build_minus(minus))
            }
            input::UsingOperator::Percent(ref percent) => {
                output::UsingOperator::Percent(self.build_percent(percent))
            }
            input::UsingOperator::Plus(ref plus) => {
                output::UsingOperator::Plus(self.build_plus(plus))
            }
            input::UsingOperator::Slash(ref slash) => {
                output::UsingOperator::Slash(self.build_slash(slash))
            }
            input::UsingOperator::Tilde(ref tilde) => {
                output::UsingOperator::Tilde(self.build_tilde(tilde))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_using_target(
        &mut self,
        source: &input::UsingTarget,
    ) -> output::UsingTarget {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::UsingTarget::TypeName(ref type_name) => {
                output::UsingTarget::TypeName(self.build_type_name(type_name))
            }
            input::UsingTarget::Asterisk(ref asterisk) => {
                output::UsingTarget::Asterisk(self.build_asterisk(asterisk))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_variable_declaration_target(
        &mut self,
        source: &input::VariableDeclarationTarget,
    ) -> output::VariableDeclarationTarget {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::VariableDeclarationTarget::SingleTypedDeclaration(
                ref single_typed_declaration,
            ) => output::VariableDeclarationTarget::SingleTypedDeclaration(
                self.build_single_typed_declaration(single_typed_declaration),
            ),
            input::VariableDeclarationTarget::MultiTypedDeclaration(
                ref multi_typed_declaration,
            ) => output::VariableDeclarationTarget::MultiTypedDeclaration(
                self.build_multi_typed_declaration(multi_typed_declaration),
            ),
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_version_expression(
        &mut self,
        source: &input::VersionExpression,
    ) -> output::VersionExpression {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::VersionExpression::VersionRange(ref version_range) => {
                output::VersionExpression::VersionRange(self.build_version_range(version_range))
            }
            input::VersionExpression::VersionTerm(ref version_term) => {
                output::VersionExpression::VersionTerm(self.build_version_term(version_term))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_version_literal(
        &mut self,
        source: &input::VersionLiteral,
    ) -> output::VersionLiteral {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::VersionLiteral::SimpleVersionLiteral(ref simple_version_literal) => {
                output::VersionLiteral::SimpleVersionLiteral(
                    self.build_simple_version_literal(simple_version_literal),
                )
            }
            input::VersionLiteral::PragmaStringLiteral(ref pragma_string_literal) => {
                output::VersionLiteral::StringLiteral(
                    self.build_pragma_string_literal(pragma_string_literal),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_version_operator(
        &mut self,
        source: &input::VersionOperator,
    ) -> output::VersionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::VersionOperator::PragmaCaret(ref pragma_caret) => {
                output::VersionOperator::PragmaCaret(self.build_pragma_caret(pragma_caret))
            }
            input::VersionOperator::PragmaTilde(ref pragma_tilde) => {
                output::VersionOperator::PragmaTilde(self.build_pragma_tilde(pragma_tilde))
            }
            input::VersionOperator::PragmaEqual(ref pragma_equal) => {
                output::VersionOperator::PragmaEqual(self.build_pragma_equal(pragma_equal))
            }
            input::VersionOperator::PragmaLessThan(ref pragma_less_than) => {
                output::VersionOperator::PragmaLessThan(
                    self.build_pragma_less_than(pragma_less_than),
                )
            }
            input::VersionOperator::PragmaGreaterThan(ref pragma_greater_than) => {
                output::VersionOperator::PragmaGreaterThan(
                    self.build_pragma_greater_than(pragma_greater_than),
                )
            }
            input::VersionOperator::PragmaLessThanEqual(ref pragma_less_than_equal) => {
                output::VersionOperator::PragmaLessThanEqual(
                    self.build_pragma_less_than_equal(pragma_less_than_equal),
                )
            }
            input::VersionOperator::PragmaGreaterThanEqual(ref pragma_greater_than_equal) => {
                output::VersionOperator::PragmaGreaterThanEqual(
                    self.build_pragma_greater_than_equal(pragma_greater_than_equal),
                )
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_yul_expression(
        &mut self,
        source: &input::YulExpression,
    ) -> output::YulExpression {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::YulExpression::YulFunctionCallExpression(ref yul_function_call_expression) => {
                output::YulExpression::YulFunctionCallExpression(
                    self.build_yul_function_call_expression(yul_function_call_expression),
                )
            }
            input::YulExpression::YulLiteral(ref yul_literal) => {
                output::YulExpression::YulLiteral(self.build_yul_literal(yul_literal))
            }
            input::YulExpression::YulPath(ref yul_path) => {
                output::YulExpression::YulPath(self.build_yul_path(yul_path))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_yul_literal(&mut self, source: &input::YulLiteral) -> output::YulLiteral {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::YulLiteral::YulTrueKeyword(ref yul_true_keyword) => {
                output::YulLiteral::TrueKeyword(self.build_yul_true_keyword(yul_true_keyword))
            }
            input::YulLiteral::YulFalseKeyword(ref yul_false_keyword) => {
                output::YulLiteral::FalseKeyword(self.build_yul_false_keyword(yul_false_keyword))
            }
            input::YulLiteral::YulDecimalLiteral(ref yul_decimal_literal) => {
                output::YulLiteral::DecimalLiteral(
                    self.build_yul_decimal_literal(yul_decimal_literal),
                )
            }
            input::YulLiteral::YulHexLiteral(ref yul_hex_literal) => {
                output::YulLiteral::HexLiteral(self.build_yul_hex_literal(yul_hex_literal))
            }
            input::YulLiteral::YulHexStringLiteral(ref yul_hex_string_literal) => {
                output::YulLiteral::HexStringLiteral(
                    self.build_yul_hex_string_literal(yul_hex_string_literal),
                )
            }
            input::YulLiteral::YulStringLiteral(ref yul_string_literal) => {
                output::YulLiteral::StringLiteral(self.build_yul_string_literal(yul_string_literal))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_yul_statement(
        &mut self,
        source: &input::YulStatement,
    ) -> output::YulStatement {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::YulStatement::YulBlock(ref yul_block) => {
                output::YulStatement::YulBlock(self.build_yul_block(yul_block))
            }
            input::YulStatement::YulFunctionDefinition(ref yul_function_definition) => {
                output::YulStatement::YulFunctionDefinition(
                    self.build_yul_function_definition(yul_function_definition),
                )
            }
            input::YulStatement::YulIfStatement(ref yul_if_statement) => {
                output::YulStatement::YulIfStatement(self.build_yul_if_statement(yul_if_statement))
            }
            input::YulStatement::YulForStatement(ref yul_for_statement) => {
                output::YulStatement::YulForStatement(
                    self.build_yul_for_statement(yul_for_statement),
                )
            }
            input::YulStatement::YulSwitchStatement(ref yul_switch_statement) => {
                output::YulStatement::YulSwitchStatement(
                    self.build_yul_switch_statement(yul_switch_statement),
                )
            }
            input::YulStatement::YulLeaveStatement(ref yul_leave_statement) => {
                output::YulStatement::YulLeaveStatement(
                    self.build_yul_leave_statement(yul_leave_statement),
                )
            }
            input::YulStatement::YulBreakStatement(ref yul_break_statement) => {
                output::YulStatement::YulBreakStatement(
                    self.build_yul_break_statement(yul_break_statement),
                )
            }
            input::YulStatement::YulContinueStatement(ref yul_continue_statement) => {
                output::YulStatement::YulContinueStatement(
                    self.build_yul_continue_statement(yul_continue_statement),
                )
            }
            input::YulStatement::YulVariableAssignmentStatement(
                ref yul_variable_assignment_statement,
            ) => output::YulStatement::YulVariableAssignmentStatement(
                self.build_yul_variable_assignment_statement(yul_variable_assignment_statement),
            ),
            input::YulStatement::YulVariableDeclarationStatement(
                ref yul_variable_declaration_statement,
            ) => output::YulStatement::YulVariableDeclarationStatement(
                self.build_yul_variable_declaration_statement(yul_variable_declaration_statement),
            ),
            input::YulStatement::YulExpression(ref yul_expression) => {
                output::YulStatement::YulExpression(self.build_yul_expression(yul_expression))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_yul_switch_case(
        &mut self,
        source: &input::YulSwitchCase,
    ) -> output::YulSwitchCase {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::YulSwitchCase::YulDefaultCase(ref yul_default_case) => {
                output::YulSwitchCase::YulDefaultCase(self.build_yul_default_case(yul_default_case))
            }
            input::YulSwitchCase::YulValueCase(ref yul_value_case) => {
                output::YulSwitchCase::YulValueCase(self.build_yul_value_case(yul_value_case))
            }
        }
    }

    //
    // Collapsed choices
    //

    pub(super) fn build_identifier_path_element(
        &mut self,
        source: &input::IdentifierPathElement,
    ) -> output::Identifier {
        match source {
            input::IdentifierPathElement::Identifier(terminal) => {
                let text = self.unparse_range(terminal.range.clone());
                Arc::new(output::IdentifierStruct {
                    id: self.next_id(),
                    range: terminal.range.clone(),
                    text,
                })
            }
            input::IdentifierPathElement::AddressKeyword(terminal) => {
                let text = self.unparse_range(terminal.range.clone());
                Arc::new(output::IdentifierStruct {
                    id: self.next_id(),
                    range: terminal.range.clone(),
                    text,
                })
            }
        }
    }

    //
    // Repeated & Separated
    //

    pub(super) fn build_array_values(
        &mut self,
        source: &input::ArrayValues,
    ) -> output::ArrayValues {
        source
            .elements
            .iter()
            .map(|item| self.build_expression(item))
            .collect()
    }

    pub(super) fn build_call_options(
        &mut self,
        source: &input::CallOptions,
    ) -> output::CallOptions {
        source
            .elements
            .iter()
            .map(|item| self.build_named_argument(item))
            .collect()
    }

    pub(super) fn build_catch_clauses(
        &mut self,
        source: &input::CatchClauses,
    ) -> output::CatchClauses {
        source
            .elements
            .iter()
            .map(|item| self.build_catch_clause(item))
            .collect()
    }

    pub(super) fn build_contract_members(
        &mut self,
        source: &input::ContractMembers,
    ) -> output::ContractMembers {
        source
            .elements
            .iter()
            .map(|item| self.build_contract_member(item))
            .collect()
    }

    pub(super) fn build_enum_members(
        &mut self,
        source: &input::EnumMembers,
    ) -> output::EnumMembers {
        source
            .elements
            .iter()
            .map(|item| self.build_identifier(item))
            .collect()
    }

    pub(super) fn build_hex_string_literals(
        &mut self,
        source: &input::HexStringLiterals,
    ) -> output::HexStringLiterals {
        source
            .elements
            .iter()
            .map(|item| self.build_hex_string_literal(item))
            .collect()
    }

    pub(super) fn build_identifier_path(
        &mut self,
        source: &input::IdentifierPath,
    ) -> output::IdentifierPath {
        source
            .elements
            .iter()
            .map(|item| self.build_identifier_path_element(item))
            .collect()
    }

    pub(super) fn build_import_deconstruction_symbols(
        &mut self,
        source: &input::ImportDeconstructionSymbols,
    ) -> output::ImportDeconstructionSymbols {
        source
            .elements
            .iter()
            .map(|item| self.build_import_deconstruction_symbol(item))
            .collect()
    }

    pub(super) fn build_inheritance_types(
        &mut self,
        source: &input::InheritanceTypes,
    ) -> output::InheritanceTypes {
        source
            .elements
            .iter()
            .map(|item| self.build_inheritance_type(item))
            .collect()
    }

    pub(super) fn build_interface_members(
        &mut self,
        source: &input::InterfaceMembers,
    ) -> output::InterfaceMembers {
        source
            .elements
            .iter()
            .map(|item| self.build_contract_member(item))
            .collect()
    }

    pub(super) fn build_library_members(
        &mut self,
        source: &input::LibraryMembers,
    ) -> output::LibraryMembers {
        source
            .elements
            .iter()
            .map(|item| self.build_contract_member(item))
            .collect()
    }

    pub(super) fn build_multi_typed_declaration_elements(
        &mut self,
        source: &input::MultiTypedDeclarationElements,
    ) -> output::MultiTypedDeclarationElements {
        source
            .elements
            .iter()
            .map(|item| self.build_multi_typed_declaration_element(item))
            .collect()
    }

    pub(super) fn build_named_arguments(
        &mut self,
        source: &input::NamedArguments,
    ) -> output::NamedArguments {
        source
            .elements
            .iter()
            .map(|item| self.build_named_argument(item))
            .collect()
    }

    pub(super) fn build_override_paths(
        &mut self,
        source: &input::OverridePaths,
    ) -> output::OverridePaths {
        source
            .elements
            .iter()
            .map(|item| self.build_identifier_path(item))
            .collect()
    }

    pub(super) fn build_parameters(&mut self, source: &input::Parameters) -> output::Parameters {
        source
            .elements
            .iter()
            .map(|item| self.build_parameter(item))
            .collect()
    }

    pub(super) fn build_positional_arguments(
        &mut self,
        source: &input::PositionalArguments,
    ) -> output::PositionalArguments {
        source
            .elements
            .iter()
            .map(|item| self.build_expression(item))
            .collect()
    }

    pub(super) fn build_simple_version_literal(
        &mut self,
        source: &input::SimpleVersionLiteral,
    ) -> output::SimpleVersionLiteral {
        source
            .elements
            .iter()
            .map(|item| self.build_version_specifier(item))
            .collect()
    }

    pub(super) fn build_source_unit_members(
        &mut self,
        source: &input::SourceUnitMembers,
    ) -> output::SourceUnitMembers {
        source
            .elements
            .iter()
            .map(|item| self.build_source_unit_member(item))
            .collect()
    }

    pub(super) fn build_statements(&mut self, source: &input::Statements) -> output::Statements {
        source
            .elements
            .iter()
            .map(|item| self.build_statement(item))
            .collect()
    }

    pub(super) fn build_string_literals(
        &mut self,
        source: &input::StringLiterals,
    ) -> output::StringLiterals {
        source
            .elements
            .iter()
            .map(|item| self.build_string_literal(item))
            .collect()
    }

    pub(super) fn build_struct_members(
        &mut self,
        source: &input::StructMembers,
    ) -> output::StructMembers {
        source
            .elements
            .iter()
            .map(|item| self.build_struct_member(item))
            .collect()
    }

    pub(super) fn build_tuple_values(
        &mut self,
        source: &input::TupleValues,
    ) -> output::TupleValues {
        source
            .elements
            .iter()
            .map(|item| self.build_tuple_value(item))
            .collect()
    }

    pub(super) fn build_unicode_string_literals(
        &mut self,
        source: &input::UnicodeStringLiterals,
    ) -> output::UnicodeStringLiterals {
        source
            .elements
            .iter()
            .map(|item| self.build_unicode_string_literal(item))
            .collect()
    }

    pub(super) fn build_using_deconstruction_symbols(
        &mut self,
        source: &input::UsingDeconstructionSymbols,
    ) -> output::UsingDeconstructionSymbols {
        source
            .elements
            .iter()
            .map(|item| self.build_using_deconstruction_symbol(item))
            .collect()
    }

    pub(super) fn build_version_expression_set(
        &mut self,
        source: &input::VersionExpressionSet,
    ) -> output::VersionExpressionSet {
        source
            .elements
            .iter()
            .map(|item| self.build_version_expression(item))
            .collect()
    }

    pub(super) fn build_version_expression_sets(
        &mut self,
        source: &input::VersionExpressionSets,
    ) -> output::VersionExpressionSets {
        source
            .elements
            .iter()
            .map(|item| self.build_version_expression_set(item))
            .collect()
    }

    pub(super) fn build_yul_arguments(
        &mut self,
        source: &input::YulArguments,
    ) -> output::YulArguments {
        source
            .elements
            .iter()
            .map(|item| self.build_yul_expression(item))
            .collect()
    }

    pub(super) fn build_yul_flags(&mut self, source: &input::YulFlags) -> output::YulFlags {
        source
            .elements
            .iter()
            .map(|item| self.build_yul_string_literal(item))
            .collect()
    }

    pub(super) fn build_yul_parameters(
        &mut self,
        source: &input::YulParameters,
    ) -> output::YulParameters {
        source
            .elements
            .iter()
            .map(|item| self.build_yul_identifier(item))
            .collect()
    }

    pub(super) fn build_yul_path(&mut self, source: &input::YulPath) -> output::YulPath {
        source
            .elements
            .iter()
            .map(|item| self.build_yul_identifier(item))
            .collect()
    }

    pub(super) fn build_yul_paths(&mut self, source: &input::YulPaths) -> output::YulPaths {
        source
            .elements
            .iter()
            .map(|item| self.build_yul_path(item))
            .collect()
    }

    pub(super) fn build_yul_statements(
        &mut self,
        source: &input::YulStatements,
    ) -> output::YulStatements {
        source
            .elements
            .iter()
            .map(|item| self.build_yul_statement(item))
            .collect()
    }

    pub(super) fn build_yul_switch_cases(
        &mut self,
        source: &input::YulSwitchCases,
    ) -> output::YulSwitchCases {
        source
            .elements
            .iter()
            .map(|item| self.build_yul_switch_case(item))
            .collect()
    }

    pub(super) fn build_yul_variable_names(
        &mut self,
        source: &input::YulVariableNames,
    ) -> output::YulVariableNames {
        source
            .elements
            .iter()
            .map(|item| self.build_yul_identifier(item))
            .collect()
    }

    //
    // Terminals
    //

    pub(super) fn build_abi_encoder_v2_keyword(
        &mut self,
        source: &input::ABIEncoderV2Keyword,
    ) -> output::ABIEncoderV2Keyword {
        Arc::new(output::ABIEncoderV2KeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_abicoder_v1_keyword(
        &mut self,
        source: &input::AbicoderV1Keyword,
    ) -> output::AbicoderV1Keyword {
        Arc::new(output::AbicoderV1KeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_abicoder_v2_keyword(
        &mut self,
        source: &input::AbicoderV2Keyword,
    ) -> output::AbicoderV2Keyword {
        Arc::new(output::AbicoderV2KeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_abstract_keyword(
        &mut self,
        source: &input::AbstractKeyword,
    ) -> output::AbstractKeyword {
        Arc::new(output::AbstractKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_ampersand(&mut self, source: &input::Ampersand) -> output::Ampersand {
        Arc::new(output::AmpersandStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_ampersand_equal(
        &mut self,
        source: &input::AmpersandEqual,
    ) -> output::AmpersandEqual {
        Arc::new(output::AmpersandEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_anonymous_keyword(
        &mut self,
        source: &input::AnonymousKeyword,
    ) -> output::AnonymousKeyword {
        Arc::new(output::AnonymousKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_asterisk(&mut self, source: &input::Asterisk) -> output::Asterisk {
        Arc::new(output::AsteriskStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_asterisk_equal(
        &mut self,
        source: &input::AsteriskEqual,
    ) -> output::AsteriskEqual {
        Arc::new(output::AsteriskEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_bang(&mut self, source: &input::Bang) -> output::Bang {
        Arc::new(output::BangStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_bang_equal(&mut self, source: &input::BangEqual) -> output::BangEqual {
        Arc::new(output::BangEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_bar(&mut self, source: &input::Bar) -> output::Bar {
        Arc::new(output::BarStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_bar_equal(&mut self, source: &input::BarEqual) -> output::BarEqual {
        Arc::new(output::BarEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_bool_keyword(
        &mut self,
        source: &input::BoolKeyword,
    ) -> output::BoolKeyword {
        Arc::new(output::BoolKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_bytes_keyword(
        &mut self,
        source: &input::BytesKeyword,
    ) -> output::BytesKeyword {
        Arc::new(output::BytesKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_call_data_keyword(
        &mut self,
        source: &input::CallDataKeyword,
    ) -> output::CallDataKeyword {
        Arc::new(output::CallDataKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_caret(&mut self, source: &input::Caret) -> output::Caret {
        Arc::new(output::CaretStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_caret_equal(&mut self, source: &input::CaretEqual) -> output::CaretEqual {
        Arc::new(output::CaretEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_days_keyword(
        &mut self,
        source: &input::DaysKeyword,
    ) -> output::DaysKeyword {
        Arc::new(output::DaysKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_decimal_literal(
        &mut self,
        source: &input::DecimalLiteral,
    ) -> output::DecimalLiteral {
        Arc::new(output::DecimalLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_delete_keyword(
        &mut self,
        source: &input::DeleteKeyword,
    ) -> output::DeleteKeyword {
        Arc::new(output::DeleteKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_equal(&mut self, source: &input::Equal) -> output::Equal {
        Arc::new(output::EqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_equal_equal(&mut self, source: &input::EqualEqual) -> output::EqualEqual {
        Arc::new(output::EqualEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_ether_keyword(
        &mut self,
        source: &input::EtherKeyword,
    ) -> output::EtherKeyword {
        Arc::new(output::EtherKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_false_keyword(
        &mut self,
        source: &input::FalseKeyword,
    ) -> output::FalseKeyword {
        Arc::new(output::FalseKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_fixed_keyword(
        &mut self,
        source: &input::FixedKeyword,
    ) -> output::FixedKeyword {
        Arc::new(output::FixedKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_global_keyword(
        &mut self,
        source: &input::GlobalKeyword,
    ) -> output::GlobalKeyword {
        Arc::new(output::GlobalKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_greater_than(
        &mut self,
        source: &input::GreaterThan,
    ) -> output::GreaterThan {
        Arc::new(output::GreaterThanStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_greater_than_equal(
        &mut self,
        source: &input::GreaterThanEqual,
    ) -> output::GreaterThanEqual {
        Arc::new(output::GreaterThanEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_greater_than_greater_than(
        &mut self,
        source: &input::GreaterThanGreaterThan,
    ) -> output::GreaterThanGreaterThan {
        Arc::new(output::GreaterThanGreaterThanStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_greater_than_greater_than_equal(
        &mut self,
        source: &input::GreaterThanGreaterThanEqual,
    ) -> output::GreaterThanGreaterThanEqual {
        Arc::new(output::GreaterThanGreaterThanEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_greater_than_greater_than_greater_than(
        &mut self,
        source: &input::GreaterThanGreaterThanGreaterThan,
    ) -> output::GreaterThanGreaterThanGreaterThan {
        Arc::new(output::GreaterThanGreaterThanGreaterThanStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_greater_than_greater_than_greater_than_equal(
        &mut self,
        source: &input::GreaterThanGreaterThanGreaterThanEqual,
    ) -> output::GreaterThanGreaterThanGreaterThanEqual {
        Arc::new(output::GreaterThanGreaterThanGreaterThanEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_gwei_keyword(
        &mut self,
        source: &input::GweiKeyword,
    ) -> output::GweiKeyword {
        Arc::new(output::GweiKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_hex_literal(&mut self, source: &input::HexLiteral) -> output::HexLiteral {
        Arc::new(output::HexLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_hex_string_literal(
        &mut self,
        source: &input::HexStringLiteral,
    ) -> output::HexStringLiteral {
        Arc::new(output::HexStringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_hours_keyword(
        &mut self,
        source: &input::HoursKeyword,
    ) -> output::HoursKeyword {
        Arc::new(output::HoursKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_identifier(&mut self, source: &input::Identifier) -> output::Identifier {
        Arc::new(output::IdentifierStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_indexed_keyword(
        &mut self,
        source: &input::IndexedKeyword,
    ) -> output::IndexedKeyword {
        Arc::new(output::IndexedKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_int_keyword(&mut self, source: &input::IntKeyword) -> output::IntKeyword {
        Arc::new(output::IntKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_less_than(&mut self, source: &input::LessThan) -> output::LessThan {
        Arc::new(output::LessThanStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_less_than_equal(
        &mut self,
        source: &input::LessThanEqual,
    ) -> output::LessThanEqual {
        Arc::new(output::LessThanEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_less_than_less_than(
        &mut self,
        source: &input::LessThanLessThan,
    ) -> output::LessThanLessThan {
        Arc::new(output::LessThanLessThanStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_less_than_less_than_equal(
        &mut self,
        source: &input::LessThanLessThanEqual,
    ) -> output::LessThanLessThanEqual {
        Arc::new(output::LessThanLessThanEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_memory_keyword(
        &mut self,
        source: &input::MemoryKeyword,
    ) -> output::MemoryKeyword {
        Arc::new(output::MemoryKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_minus(&mut self, source: &input::Minus) -> output::Minus {
        Arc::new(output::MinusStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_minus_equal(&mut self, source: &input::MinusEqual) -> output::MinusEqual {
        Arc::new(output::MinusEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_minus_minus(&mut self, source: &input::MinusMinus) -> output::MinusMinus {
        Arc::new(output::MinusMinusStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_minutes_keyword(
        &mut self,
        source: &input::MinutesKeyword,
    ) -> output::MinutesKeyword {
        Arc::new(output::MinutesKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_payable_keyword(
        &mut self,
        source: &input::PayableKeyword,
    ) -> output::PayableKeyword {
        Arc::new(output::PayableKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_percent(&mut self, source: &input::Percent) -> output::Percent {
        Arc::new(output::PercentStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_percent_equal(
        &mut self,
        source: &input::PercentEqual,
    ) -> output::PercentEqual {
        Arc::new(output::PercentEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_plus(&mut self, source: &input::Plus) -> output::Plus {
        Arc::new(output::PlusStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_plus_equal(&mut self, source: &input::PlusEqual) -> output::PlusEqual {
        Arc::new(output::PlusEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_plus_plus(&mut self, source: &input::PlusPlus) -> output::PlusPlus {
        Arc::new(output::PlusPlusStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_pragma_caret(
        &mut self,
        source: &input::PragmaCaret,
    ) -> output::PragmaCaret {
        Arc::new(output::PragmaCaretStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_pragma_equal(
        &mut self,
        source: &input::PragmaEqual,
    ) -> output::PragmaEqual {
        Arc::new(output::PragmaEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_pragma_greater_than(
        &mut self,
        source: &input::PragmaGreaterThan,
    ) -> output::PragmaGreaterThan {
        Arc::new(output::PragmaGreaterThanStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_pragma_greater_than_equal(
        &mut self,
        source: &input::PragmaGreaterThanEqual,
    ) -> output::PragmaGreaterThanEqual {
        Arc::new(output::PragmaGreaterThanEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_pragma_less_than(
        &mut self,
        source: &input::PragmaLessThan,
    ) -> output::PragmaLessThan {
        Arc::new(output::PragmaLessThanStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_pragma_less_than_equal(
        &mut self,
        source: &input::PragmaLessThanEqual,
    ) -> output::PragmaLessThanEqual {
        Arc::new(output::PragmaLessThanEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_pragma_tilde(
        &mut self,
        source: &input::PragmaTilde,
    ) -> output::PragmaTilde {
        Arc::new(output::PragmaTildeStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_smt_checker_keyword(
        &mut self,
        source: &input::SMTCheckerKeyword,
    ) -> output::SMTCheckerKeyword {
        Arc::new(output::SMTCheckerKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_seconds_keyword(
        &mut self,
        source: &input::SecondsKeyword,
    ) -> output::SecondsKeyword {
        Arc::new(output::SecondsKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_semicolon(&mut self, source: &input::Semicolon) -> output::Semicolon {
        Arc::new(output::SemicolonStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_slash(&mut self, source: &input::Slash) -> output::Slash {
        Arc::new(output::SlashStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_slash_equal(&mut self, source: &input::SlashEqual) -> output::SlashEqual {
        Arc::new(output::SlashEqualStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_storage_keyword(
        &mut self,
        source: &input::StorageKeyword,
    ) -> output::StorageKeyword {
        Arc::new(output::StorageKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_string_keyword(
        &mut self,
        source: &input::StringKeyword,
    ) -> output::StringKeyword {
        Arc::new(output::StringKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_string_literal(
        &mut self,
        source: &input::StringLiteral,
    ) -> output::StringLiteral {
        Arc::new(output::StringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_super_keyword(
        &mut self,
        source: &input::SuperKeyword,
    ) -> output::SuperKeyword {
        Arc::new(output::SuperKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_this_keyword(
        &mut self,
        source: &input::ThisKeyword,
    ) -> output::ThisKeyword {
        Arc::new(output::ThisKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_tilde(&mut self, source: &input::Tilde) -> output::Tilde {
        Arc::new(output::TildeStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_true_keyword(
        &mut self,
        source: &input::TrueKeyword,
    ) -> output::TrueKeyword {
        Arc::new(output::TrueKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_ufixed_keyword(
        &mut self,
        source: &input::UfixedKeyword,
    ) -> output::UfixedKeyword {
        Arc::new(output::UfixedKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_uint_keyword(
        &mut self,
        source: &input::UintKeyword,
    ) -> output::UintKeyword {
        Arc::new(output::UintKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_unicode_string_literal(
        &mut self,
        source: &input::UnicodeStringLiteral,
    ) -> output::UnicodeStringLiteral {
        Arc::new(output::UnicodeStringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_version_specifier(
        &mut self,
        source: &input::VersionSpecifier,
    ) -> output::VersionSpecifier {
        Arc::new(output::VersionSpecifierStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_virtual_keyword(
        &mut self,
        source: &input::VirtualKeyword,
    ) -> output::VirtualKeyword {
        Arc::new(output::VirtualKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_weeks_keyword(
        &mut self,
        source: &input::WeeksKeyword,
    ) -> output::WeeksKeyword {
        Arc::new(output::WeeksKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_wei_keyword(&mut self, source: &input::WeiKeyword) -> output::WeiKeyword {
        Arc::new(output::WeiKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    //
    // Normalized Terminals
    //

    pub(super) fn build_pragma_string_literal(
        &mut self,
        source: &input::PragmaStringLiteral,
    ) -> output::StringLiteral {
        Arc::new(output::StringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_yul_decimal_literal(
        &mut self,
        source: &input::YulDecimalLiteral,
    ) -> output::DecimalLiteral {
        Arc::new(output::DecimalLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_yul_false_keyword(
        &mut self,
        source: &input::YulFalseKeyword,
    ) -> output::FalseKeyword {
        Arc::new(output::FalseKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }

    pub(super) fn build_yul_hex_literal(
        &mut self,
        source: &input::YulHexLiteral,
    ) -> output::HexLiteral {
        Arc::new(output::HexLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_yul_hex_string_literal(
        &mut self,
        source: &input::YulHexStringLiteral,
    ) -> output::HexStringLiteral {
        Arc::new(output::HexStringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_yul_identifier(
        &mut self,
        source: &input::YulIdentifier,
    ) -> output::Identifier {
        Arc::new(output::IdentifierStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_yul_string_literal(
        &mut self,
        source: &input::YulStringLiteral,
    ) -> output::StringLiteral {
        Arc::new(output::StringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text: self.unparse_range(source.range.clone()),
        })
    }

    pub(super) fn build_yul_true_keyword(
        &mut self,
        source: &input::YulTrueKeyword,
    ) -> output::TrueKeyword {
        Arc::new(output::TrueKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
        })
    }
}
