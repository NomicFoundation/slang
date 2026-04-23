// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// Builder from StructuredCstModel
#![allow(clippy::too_many_lines)]
#![allow(unused_variables)]

use std::rc::Rc;

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
        let range = source.text_range().unwrap_or_default();
        let version = self.build_abicoder_version(&source.version);

        Rc::new(output::AbicoderPragmaStruct { id, range, version })
    }

    pub(super) fn build_additive_expression(
        &mut self,
        source: &input::AdditiveExpression,
    ) -> output::AdditiveExpression {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let expression_additive_expression_operator = self
            .build_expression_additive_expression_operator(
                &source.expression_additive_expression_operator,
            );
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::AdditiveExpressionStruct {
            id,
            range,
            left_operand,
            expression_additive_expression_operator,
            right_operand,
        })
    }

    pub(super) fn build_address_type(
        &mut self,
        source: &input::AddressType,
    ) -> output::AddressType {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let payable_keyword = source.payable_keyword.is_some();

        Rc::new(output::AddressTypeStruct {
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
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::AndExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let items = self.build_array_values(&source.items);

        Rc::new(output::ArrayExpressionStruct { id, range, items })
    }

    pub(super) fn build_array_type_name(
        &mut self,
        source: &input::ArrayTypeName,
    ) -> output::ArrayTypeName {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let operand = self.build_type_name(&source.operand);
        let index = source
            .index
            .as_ref()
            .map(|value| self.build_expression(value));

        Rc::new(output::ArrayTypeNameStruct {
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
        let range = source.text_range().unwrap_or_default();
        let label = source
            .label
            .as_ref()
            .map(|value| self.build_yul_string_literal(value));
        let flags = source
            .flags
            .as_ref()
            .map(|value| self.build_yul_flags_declaration(value));
        let body = self.build_yul_block(&source.body);

        Rc::new(output::AssemblyStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let expression_assignment_expression_operator = self
            .build_expression_assignment_expression_operator(
                &source.expression_assignment_expression_operator,
            );
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::AssignmentExpressionStruct {
            id,
            range,
            left_operand,
            expression_assignment_expression_operator,
            right_operand,
        })
    }

    pub(super) fn build_bitwise_and_expression(
        &mut self,
        source: &input::BitwiseAndExpression,
    ) -> output::BitwiseAndExpression {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::BitwiseAndExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::BitwiseOrExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::BitwiseXorExpressionStruct {
            id,
            range,
            left_operand,
            right_operand,
        })
    }

    pub(super) fn build_block(&mut self, source: &input::Block) -> output::Block {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let statements = self.build_statements(&source.statements);

        Rc::new(output::BlockStruct {
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
        let range = source.text_range().unwrap_or_default();

        Rc::new(output::BreakStatementStruct { id, range })
    }

    pub(super) fn build_call_options_expression(
        &mut self,
        source: &input::CallOptionsExpression,
    ) -> output::CallOptionsExpression {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let options = self.build_call_options(&source.options);

        Rc::new(output::CallOptionsExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let error = source
            .error
            .as_ref()
            .map(|value| self.build_catch_clause_error(value));
        let body = self.build_block(&source.body);

        Rc::new(output::CatchClauseStruct {
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
        let range = source.text_range().unwrap_or_default();
        let name = source
            .name
            .as_ref()
            .map(|value| self.build_identifier(value));
        let parameters = self.build_parameters_declaration(&source.parameters);

        Rc::new(output::CatchClauseErrorStruct {
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
        let range = source.text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let true_expression = self.build_expression(&source.true_expression);
        let false_expression = self.build_expression(&source.false_expression);

        Rc::new(output::ConditionalExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();

        Rc::new(output::ContinueStatementStruct { id, range })
    }

    pub(super) fn build_decimal_number_expression(
        &mut self,
        source: &input::DecimalNumberExpression,
    ) -> output::DecimalNumberExpression {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let literal = self.build_decimal_literal(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.build_number_unit(value));

        Rc::new(output::DecimalNumberExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let body = self.build_statement(&source.body);
        let condition = self.build_expression(&source.condition);

        Rc::new(output::DoWhileStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let event = self.build_identifier_path(&source.event);
        let arguments = self.build_arguments_declaration(&source.arguments);

        Rc::new(output::EmitStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let members = self.build_enum_members(&source.members);

        Rc::new(output::EnumDefinitionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let expression_equality_expression_operator = self
            .build_expression_equality_expression_operator(
                &source.expression_equality_expression_operator,
            );
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::EqualityExpressionStruct {
            id,
            range,
            left_operand,
            expression_equality_expression_operator,
            right_operand,
        })
    }

    pub(super) fn build_experimental_pragma(
        &mut self,
        source: &input::ExperimentalPragma,
    ) -> output::ExperimentalPragma {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let feature = self.build_experimental_feature(&source.feature);

        Rc::new(output::ExperimentalPragmaStruct { id, range, feature })
    }

    pub(super) fn build_exponentiation_expression(
        &mut self,
        source: &input::ExponentiationExpression,
    ) -> output::ExponentiationExpression {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::ExponentiationExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let expression = self.build_expression(&source.expression);

        Rc::new(output::ExpressionStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let initialization = self.build_for_statement_initialization(&source.initialization);
        let condition = self.build_for_statement_condition(&source.condition);
        let iterator = source
            .iterator
            .as_ref()
            .map(|value| self.build_expression(value));
        let body = self.build_statement(&source.body);

        Rc::new(output::ForStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let arguments = self.build_arguments_declaration(&source.arguments);

        Rc::new(output::FunctionCallExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let literal = self.build_hex_literal(&source.literal);

        Rc::new(output::HexNumberExpressionStruct { id, range, literal })
    }

    pub(super) fn build_if_statement(
        &mut self,
        source: &input::IfStatement,
    ) -> output::IfStatement {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let condition = self.build_expression(&source.condition);
        let body = self.build_statement(&source.body);
        let else_branch = source
            .else_branch
            .as_ref()
            .map(|value| self.build_else_branch(value));

        Rc::new(output::IfStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let symbols = self.build_import_deconstruction_symbols(&source.symbols);
        let path = self.build_string_literal(&source.path);

        Rc::new(output::ImportDeconstructionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.build_import_alias(value));

        Rc::new(output::ImportDeconstructionSymbolStruct {
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
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let expression_inequality_expression_operator = self
            .build_expression_inequality_expression_operator(
                &source.expression_inequality_expression_operator,
            );
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::InequalityExpressionStruct {
            id,
            range,
            left_operand,
            expression_inequality_expression_operator,
            right_operand,
        })
    }

    pub(super) fn build_inheritance_type(
        &mut self,
        source: &input::InheritanceType,
    ) -> output::InheritanceType {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let type_name = self.build_identifier_path(&source.type_name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.build_arguments_declaration(value));

        Rc::new(output::InheritanceTypeStruct {
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
        let range = source.text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let inheritance = source
            .inheritance
            .as_ref()
            .map(|value| self.build_inheritance_specifier(value));
        let members = self.build_interface_members(&source.members);

        Rc::new(output::InterfaceDefinitionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let members = self.build_library_members(&source.members);

        Rc::new(output::LibraryDefinitionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let member = self.build_identifier_path_element(&source.member);

        Rc::new(output::MemberAccessExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let name = self.build_identifier_path(&source.name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.build_arguments_declaration(value));

        Rc::new(output::ModifierInvocationStruct {
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
        let range = source.text_range().unwrap_or_default();
        let elements = self.build_multi_typed_declaration_elements(&source.elements);
        let value = self.build_variable_declaration_value(&source.value);

        Rc::new(output::MultiTypedDeclarationStruct {
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
        let range = source.text_range().unwrap_or_default();
        let member = source
            .member
            .as_ref()
            .map(|value| self.build_variable_declaration(value));

        Rc::new(output::MultiTypedDeclarationElementStruct { id, range, member })
    }

    pub(super) fn build_multiplicative_expression(
        &mut self,
        source: &input::MultiplicativeExpression,
    ) -> output::MultiplicativeExpression {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let expression_multiplicative_expression_operator = self
            .build_expression_multiplicative_expression_operator(
                &source.expression_multiplicative_expression_operator,
            );
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::MultiplicativeExpressionStruct {
            id,
            range,
            left_operand,
            expression_multiplicative_expression_operator,
            right_operand,
        })
    }

    pub(super) fn build_named_argument(
        &mut self,
        source: &input::NamedArgument,
    ) -> output::NamedArgument {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let value = self.build_expression(&source.value);

        Rc::new(output::NamedArgumentStruct {
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
        let range = source.text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);

        Rc::new(output::NewExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::OrExpressionStruct {
            id,
            range,
            left_operand,
            right_operand,
        })
    }

    pub(super) fn build_path_import(&mut self, source: &input::PathImport) -> output::PathImport {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let path = self.build_string_literal(&source.path);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.build_import_alias(value));

        Rc::new(output::PathImportStruct {
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
        let range = source.text_range().unwrap_or_default();
        let operand = self.build_expression(&source.operand);
        let expression_postfix_expression_operator = self
            .build_expression_postfix_expression_operator(
                &source.expression_postfix_expression_operator,
            );

        Rc::new(output::PostfixExpressionStruct {
            id,
            range,
            operand,
            expression_postfix_expression_operator,
        })
    }

    pub(super) fn build_pragma_directive(
        &mut self,
        source: &input::PragmaDirective,
    ) -> output::PragmaDirective {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let pragma = self.build_pragma(&source.pragma);

        Rc::new(output::PragmaDirectiveStruct { id, range, pragma })
    }

    pub(super) fn build_prefix_expression(
        &mut self,
        source: &input::PrefixExpression,
    ) -> output::PrefixExpression {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let expression_prefix_expression_operator = self
            .build_expression_prefix_expression_operator(
                &source.expression_prefix_expression_operator,
            );
        let operand = self.build_expression(&source.operand);

        Rc::new(output::PrefixExpressionStruct {
            id,
            range,
            expression_prefix_expression_operator,
            operand,
        })
    }

    pub(super) fn build_return_statement(
        &mut self,
        source: &input::ReturnStatement,
    ) -> output::ReturnStatement {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.build_expression(value));

        Rc::new(output::ReturnStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let error = self.build_identifier_path(&source.error);
        let arguments = self.build_arguments_declaration(&source.arguments);

        Rc::new(output::RevertStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let left_operand = self.build_expression(&source.left_operand);
        let expression_shift_expression_operator = self.build_expression_shift_expression_operator(
            &source.expression_shift_expression_operator,
        );
        let right_operand = self.build_expression(&source.right_operand);

        Rc::new(output::ShiftExpressionStruct {
            id,
            range,
            left_operand,
            expression_shift_expression_operator,
            right_operand,
        })
    }

    pub(super) fn build_single_typed_declaration(
        &mut self,
        source: &input::SingleTypedDeclaration,
    ) -> output::SingleTypedDeclaration {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let declaration = self.build_variable_declaration(&source.declaration);
        let value = source
            .value
            .as_ref()
            .map(|value| self.build_variable_declaration_value(value));

        Rc::new(output::SingleTypedDeclarationStruct {
            id,
            range,
            declaration,
            value,
        })
    }

    pub(super) fn build_source_unit(&mut self, source: &input::SourceUnit) -> output::SourceUnit {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let members = self.build_source_unit_members(&source.members);

        Rc::new(output::SourceUnitStruct { id, range, members })
    }

    pub(super) fn build_struct_definition(
        &mut self,
        source: &input::StructDefinition,
    ) -> output::StructDefinition {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let members = self.build_struct_members(&source.members);

        Rc::new(output::StructDefinitionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let name = self.build_identifier(&source.name);

        Rc::new(output::StructMemberStruct {
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
        let range = source.text_range().unwrap_or_default();
        let expression = self.build_expression(&source.expression);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.build_returns_declaration(value));
        let body = self.build_block(&source.body);
        let catch_clauses = self.build_catch_clauses(&source.catch_clauses);

        Rc::new(output::TryStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let items = self.build_tuple_values(&source.items);

        Rc::new(output::TupleExpressionStruct { id, range, items })
    }

    pub(super) fn build_tuple_value(&mut self, source: &input::TupleValue) -> output::TupleValue {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.build_expression(value));

        Rc::new(output::TupleValueStruct {
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
        let range = source.text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);

        Rc::new(output::TypeExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let block = self.build_block(&source.block);

        Rc::new(output::UncheckedBlockStruct { id, range, block })
    }

    pub(super) fn build_user_defined_value_type_definition(
        &mut self,
        source: &input::UserDefinedValueTypeDefinition,
    ) -> output::UserDefinedValueTypeDefinition {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let name = self.build_identifier(&source.name);
        let value_type = self.build_elementary_type(&source.value_type);

        Rc::new(output::UserDefinedValueTypeDefinitionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let symbols = self.build_using_deconstruction_symbols(&source.symbols);

        Rc::new(output::UsingDeconstructionStruct { id, range, symbols })
    }

    pub(super) fn build_using_deconstruction_symbol(
        &mut self,
        source: &input::UsingDeconstructionSymbol,
    ) -> output::UsingDeconstructionSymbol {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let name = self.build_identifier_path(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.build_using_alias(value));

        Rc::new(output::UsingDeconstructionSymbolStruct {
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
        let range = source.text_range().unwrap_or_default();
        let clause = self.build_using_clause(&source.clause);
        let target = self.build_using_target(&source.target);
        let global_keyword = source.global_keyword.is_some();

        Rc::new(output::UsingDirectiveStruct {
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
        let range = source.text_range().unwrap_or_default();
        let type_name = self.build_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.build_storage_location(value));
        let name = self.build_identifier(&source.name);

        Rc::new(output::VariableDeclarationStruct {
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
        let range = source.text_range().unwrap_or_default();
        let target = self.build_variable_declaration_target(&source.target);

        Rc::new(output::VariableDeclarationStatementStruct { id, range, target })
    }

    pub(super) fn build_version_pragma(
        &mut self,
        source: &input::VersionPragma,
    ) -> output::VersionPragma {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let sets = self.build_version_expression_sets(&source.sets);

        Rc::new(output::VersionPragmaStruct { id, range, sets })
    }

    pub(super) fn build_version_range(
        &mut self,
        source: &input::VersionRange,
    ) -> output::VersionRange {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let start = self.build_version_literal(&source.start);
        let end = self.build_version_literal(&source.end);

        Rc::new(output::VersionRangeStruct {
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
        let range = source.text_range().unwrap_or_default();
        let operator = source
            .operator
            .as_ref()
            .map(|value| self.build_version_operator(value));
        let literal = self.build_version_literal(&source.literal);

        Rc::new(output::VersionTermStruct {
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
        let range = source.text_range().unwrap_or_default();
        let condition = self.build_expression(&source.condition);
        let body = self.build_statement(&source.body);

        Rc::new(output::WhileStatementStruct {
            id,
            range,
            condition,
            body,
        })
    }

    pub(super) fn build_yul_block(&mut self, source: &input::YulBlock) -> output::YulBlock {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let statements = self.build_yul_statements(&source.statements);

        Rc::new(output::YulBlockStruct {
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
        let range = source.text_range().unwrap_or_default();

        Rc::new(output::YulBreakStatementStruct { id, range })
    }

    pub(super) fn build_yul_continue_statement(
        &mut self,
        source: &input::YulContinueStatement,
    ) -> output::YulContinueStatement {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();

        Rc::new(output::YulContinueStatementStruct { id, range })
    }

    pub(super) fn build_yul_default_case(
        &mut self,
        source: &input::YulDefaultCase,
    ) -> output::YulDefaultCase {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let body = self.build_yul_block(&source.body);

        Rc::new(output::YulDefaultCaseStruct { id, range, body })
    }

    pub(super) fn build_yul_for_statement(
        &mut self,
        source: &input::YulForStatement,
    ) -> output::YulForStatement {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let initialization = self.build_yul_block(&source.initialization);
        let condition = self.build_yul_expression(&source.condition);
        let iterator = self.build_yul_block(&source.iterator);
        let body = self.build_yul_block(&source.body);

        Rc::new(output::YulForStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let operand = self.build_yul_expression(&source.operand);
        let arguments = self.build_yul_arguments(&source.arguments);

        Rc::new(output::YulFunctionCallExpressionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let name = self.build_yul_identifier(&source.name);
        let parameters = self.build_yul_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.build_yul_returns_declaration(value));
        let body = self.build_yul_block(&source.body);

        Rc::new(output::YulFunctionDefinitionStruct {
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
        let range = source.text_range().unwrap_or_default();
        let condition = self.build_yul_expression(&source.condition);
        let body = self.build_yul_block(&source.body);

        Rc::new(output::YulIfStatementStruct {
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
        let range = source.text_range().unwrap_or_default();

        Rc::new(output::YulLeaveStatementStruct { id, range })
    }

    pub(super) fn build_yul_switch_statement(
        &mut self,
        source: &input::YulSwitchStatement,
    ) -> output::YulSwitchStatement {
        let id = self.next_id();
        let range = source.text_range().unwrap_or_default();
        let expression = self.build_yul_expression(&source.expression);
        let cases = self.build_yul_switch_cases(&source.cases);

        Rc::new(output::YulSwitchStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let value = self.build_yul_literal(&source.value);
        let body = self.build_yul_block(&source.body);

        Rc::new(output::YulValueCaseStruct {
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
        let range = source.text_range().unwrap_or_default();
        let variables = self.build_yul_paths(&source.variables);
        let expression = self.build_yul_expression(&source.expression);

        Rc::new(output::YulVariableAssignmentStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let variables = self.build_yul_variable_names(&source.variables);
        let value = source
            .value
            .as_ref()
            .map(|value| self.build_yul_variable_declaration_value(value));

        Rc::new(output::YulVariableDeclarationStatementStruct {
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
        let range = source.text_range().unwrap_or_default();
        let expression = self.build_yul_expression(&source.expression);

        Rc::new(output::YulVariableDeclarationValueStruct {
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
            input::AbicoderVersion::AbicoderV1Keyword(_) => {
                output::AbicoderVersion::AbicoderV1Keyword
            }
            input::AbicoderVersion::AbicoderV2Keyword(_) => {
                output::AbicoderVersion::AbicoderV2Keyword
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
            input::ElementaryType::BoolKeyword(_) => output::ElementaryType::BoolKeyword,
            input::ElementaryType::StringKeyword(_) => output::ElementaryType::StringKeyword,
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
    pub(super) fn build_experimental_feature(
        &mut self,
        source: &input::ExperimentalFeature,
    ) -> output::ExperimentalFeature {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::ExperimentalFeature::ABIEncoderV2Keyword(_) => {
                output::ExperimentalFeature::ABIEncoderV2Keyword
            }
            input::ExperimentalFeature::SMTCheckerKeyword(_) => {
                output::ExperimentalFeature::SMTCheckerKeyword
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
            input::Expression::PayableKeyword(_) => output::Expression::PayableKeyword,
            input::Expression::ThisKeyword(_) => output::Expression::ThisKeyword,
            input::Expression::SuperKeyword(_) => output::Expression::SuperKeyword,
            input::Expression::TrueKeyword(_) => output::Expression::TrueKeyword,
            input::Expression::FalseKeyword(_) => output::Expression::FalseKeyword,
            input::Expression::Identifier(ref identifier) => {
                output::Expression::Identifier(self.build_identifier(identifier))
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_additive_expression_operator(
        &mut self,
        source: &input::Expression_AdditiveExpression_Operator,
    ) -> output::Expression_AdditiveExpression_Operator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_AdditiveExpression_Operator::Minus(_) => {
                output::Expression_AdditiveExpression_Operator::Minus
            }
            input::Expression_AdditiveExpression_Operator::Plus(_) => {
                output::Expression_AdditiveExpression_Operator::Plus
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_assignment_expression_operator(
        &mut self,
        source: &input::Expression_AssignmentExpression_Operator,
    ) -> output::Expression_AssignmentExpression_Operator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
          input::Expression_AssignmentExpression_Operator::AmpersandEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::AmpersandEqual
                }
              input::Expression_AssignmentExpression_Operator::AsteriskEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::AsteriskEqual
                }
              input::Expression_AssignmentExpression_Operator::BarEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::BarEqual
                }
              input::Expression_AssignmentExpression_Operator::CaretEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::CaretEqual
                }
              input::Expression_AssignmentExpression_Operator::Equal(_) => {
                  output::Expression_AssignmentExpression_Operator::Equal
                }
              input::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual
                }
              input::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual
                }
              input::Expression_AssignmentExpression_Operator::LessThanLessThanEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::LessThanLessThanEqual
                }
              input::Expression_AssignmentExpression_Operator::MinusEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::MinusEqual
                }
              input::Expression_AssignmentExpression_Operator::PercentEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::PercentEqual
                }
              input::Expression_AssignmentExpression_Operator::PlusEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::PlusEqual
                }
              input::Expression_AssignmentExpression_Operator::SlashEqual(_) => {
                  output::Expression_AssignmentExpression_Operator::SlashEqual
                }
              }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_equality_expression_operator(
        &mut self,
        source: &input::Expression_EqualityExpression_Operator,
    ) -> output::Expression_EqualityExpression_Operator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_EqualityExpression_Operator::BangEqual(_) => {
                output::Expression_EqualityExpression_Operator::BangEqual
            }
            input::Expression_EqualityExpression_Operator::EqualEqual(_) => {
                output::Expression_EqualityExpression_Operator::EqualEqual
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_inequality_expression_operator(
        &mut self,
        source: &input::Expression_InequalityExpression_Operator,
    ) -> output::Expression_InequalityExpression_Operator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_InequalityExpression_Operator::GreaterThan(_) => {
                output::Expression_InequalityExpression_Operator::GreaterThan
            }
            input::Expression_InequalityExpression_Operator::GreaterThanEqual(_) => {
                output::Expression_InequalityExpression_Operator::GreaterThanEqual
            }
            input::Expression_InequalityExpression_Operator::LessThan(_) => {
                output::Expression_InequalityExpression_Operator::LessThan
            }
            input::Expression_InequalityExpression_Operator::LessThanEqual(_) => {
                output::Expression_InequalityExpression_Operator::LessThanEqual
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_multiplicative_expression_operator(
        &mut self,
        source: &input::Expression_MultiplicativeExpression_Operator,
    ) -> output::Expression_MultiplicativeExpression_Operator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_MultiplicativeExpression_Operator::Asterisk(_) => {
                output::Expression_MultiplicativeExpression_Operator::Asterisk
            }
            input::Expression_MultiplicativeExpression_Operator::Percent(_) => {
                output::Expression_MultiplicativeExpression_Operator::Percent
            }
            input::Expression_MultiplicativeExpression_Operator::Slash(_) => {
                output::Expression_MultiplicativeExpression_Operator::Slash
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_postfix_expression_operator(
        &mut self,
        source: &input::Expression_PostfixExpression_Operator,
    ) -> output::Expression_PostfixExpression_Operator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_PostfixExpression_Operator::MinusMinus(_) => {
                output::Expression_PostfixExpression_Operator::MinusMinus
            }
            input::Expression_PostfixExpression_Operator::PlusPlus(_) => {
                output::Expression_PostfixExpression_Operator::PlusPlus
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_prefix_expression_operator(
        &mut self,
        source: &input::Expression_PrefixExpression_Operator,
    ) -> output::Expression_PrefixExpression_Operator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_PrefixExpression_Operator::Bang(_) => {
                output::Expression_PrefixExpression_Operator::Bang
            }
            input::Expression_PrefixExpression_Operator::DeleteKeyword(_) => {
                output::Expression_PrefixExpression_Operator::DeleteKeyword
            }
            input::Expression_PrefixExpression_Operator::Minus(_) => {
                output::Expression_PrefixExpression_Operator::Minus
            }
            input::Expression_PrefixExpression_Operator::MinusMinus(_) => {
                output::Expression_PrefixExpression_Operator::MinusMinus
            }
            input::Expression_PrefixExpression_Operator::PlusPlus(_) => {
                output::Expression_PrefixExpression_Operator::PlusPlus
            }
            input::Expression_PrefixExpression_Operator::Tilde(_) => {
                output::Expression_PrefixExpression_Operator::Tilde
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub(super) fn build_expression_shift_expression_operator(
        &mut self,
        source: &input::Expression_ShiftExpression_Operator,
    ) -> output::Expression_ShiftExpression_Operator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::Expression_ShiftExpression_Operator::GreaterThanGreaterThan(_) => {
                output::Expression_ShiftExpression_Operator::GreaterThanGreaterThan
            }
            input::Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(_) => {
                output::Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan
            }
            input::Expression_ShiftExpression_Operator::LessThanLessThan(_) => {
                output::Expression_ShiftExpression_Operator::LessThanLessThan
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
            input::ForStatementCondition::Semicolon(_) => output::ForStatementCondition::Semicolon,
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
            input::ForStatementInitialization::Semicolon(_) => {
                output::ForStatementInitialization::Semicolon
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
    pub(super) fn build_number_unit(&mut self, source: &input::NumberUnit) -> output::NumberUnit {
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[allow(clippy::match_single_binding)]
        match source {
            input::NumberUnit::WeiKeyword(_) => output::NumberUnit::WeiKeyword,
            input::NumberUnit::GweiKeyword(_) => output::NumberUnit::GweiKeyword,
            input::NumberUnit::EtherKeyword(_) => output::NumberUnit::EtherKeyword,
            input::NumberUnit::SecondsKeyword(_) => output::NumberUnit::SecondsKeyword,
            input::NumberUnit::MinutesKeyword(_) => output::NumberUnit::MinutesKeyword,
            input::NumberUnit::HoursKeyword(_) => output::NumberUnit::HoursKeyword,
            input::NumberUnit::DaysKeyword(_) => output::NumberUnit::DaysKeyword,
            input::NumberUnit::WeeksKeyword(_) => output::NumberUnit::WeeksKeyword,
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
            input::StorageLocation::MemoryKeyword(_) => output::StorageLocation::MemoryKeyword,
            input::StorageLocation::StorageKeyword(_) => output::StorageLocation::StorageKeyword,
            input::StorageLocation::CallDataKeyword(_) => output::StorageLocation::CallDataKeyword,
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
            input::UsingOperator::Ampersand(_) => output::UsingOperator::Ampersand,
            input::UsingOperator::Asterisk(_) => output::UsingOperator::Asterisk,
            input::UsingOperator::BangEqual(_) => output::UsingOperator::BangEqual,
            input::UsingOperator::Bar(_) => output::UsingOperator::Bar,
            input::UsingOperator::Caret(_) => output::UsingOperator::Caret,
            input::UsingOperator::EqualEqual(_) => output::UsingOperator::EqualEqual,
            input::UsingOperator::GreaterThan(_) => output::UsingOperator::GreaterThan,
            input::UsingOperator::GreaterThanEqual(_) => output::UsingOperator::GreaterThanEqual,
            input::UsingOperator::LessThan(_) => output::UsingOperator::LessThan,
            input::UsingOperator::LessThanEqual(_) => output::UsingOperator::LessThanEqual,
            input::UsingOperator::Minus(_) => output::UsingOperator::Minus,
            input::UsingOperator::Percent(_) => output::UsingOperator::Percent,
            input::UsingOperator::Plus(_) => output::UsingOperator::Plus,
            input::UsingOperator::Slash(_) => output::UsingOperator::Slash,
            input::UsingOperator::Tilde(_) => output::UsingOperator::Tilde,
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
            input::UsingTarget::Asterisk(_) => output::UsingTarget::Asterisk,
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
            input::VersionOperator::PragmaCaret(_) => output::VersionOperator::PragmaCaret,
            input::VersionOperator::PragmaTilde(_) => output::VersionOperator::PragmaTilde,
            input::VersionOperator::PragmaEqual(_) => output::VersionOperator::PragmaEqual,
            input::VersionOperator::PragmaLessThan(_) => output::VersionOperator::PragmaLessThan,
            input::VersionOperator::PragmaGreaterThan(_) => {
                output::VersionOperator::PragmaGreaterThan
            }
            input::VersionOperator::PragmaLessThanEqual(_) => {
                output::VersionOperator::PragmaLessThanEqual
            }
            input::VersionOperator::PragmaGreaterThanEqual(_) => {
                output::VersionOperator::PragmaGreaterThanEqual
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
            input::YulLiteral::YulTrueKeyword(_) => output::YulLiteral::TrueKeyword,
            input::YulLiteral::YulFalseKeyword(_) => output::YulLiteral::FalseKeyword,
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
                Rc::new(output::IdentifierStruct {
                    id: self.next_id(),
                    range: terminal.range.clone(),
                    text,
                })
            }
            input::IdentifierPathElement::AddressKeyword(terminal) => {
                let text = self.unparse_range(terminal.range.clone());
                Rc::new(output::IdentifierStruct {
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

    pub(super) fn build_bytes_keyword(
        &mut self,
        source: &input::BytesKeyword,
    ) -> output::BytesKeyword {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::BytesKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_decimal_literal(
        &mut self,
        source: &input::DecimalLiteral,
    ) -> output::DecimalLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::DecimalLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_fixed_keyword(
        &mut self,
        source: &input::FixedKeyword,
    ) -> output::FixedKeyword {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::FixedKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_hex_literal(&mut self, source: &input::HexLiteral) -> output::HexLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::HexLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_hex_string_literal(
        &mut self,
        source: &input::HexStringLiteral,
    ) -> output::HexStringLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::HexStringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_identifier(&mut self, source: &input::Identifier) -> output::Identifier {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::IdentifierStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_int_keyword(&mut self, source: &input::IntKeyword) -> output::IntKeyword {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::IntKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_string_literal(
        &mut self,
        source: &input::StringLiteral,
    ) -> output::StringLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::StringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_ufixed_keyword(
        &mut self,
        source: &input::UfixedKeyword,
    ) -> output::UfixedKeyword {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::UfixedKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_uint_keyword(
        &mut self,
        source: &input::UintKeyword,
    ) -> output::UintKeyword {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::UintKeywordStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_unicode_string_literal(
        &mut self,
        source: &input::UnicodeStringLiteral,
    ) -> output::UnicodeStringLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::UnicodeStringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_version_specifier(
        &mut self,
        source: &input::VersionSpecifier,
    ) -> output::VersionSpecifier {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::VersionSpecifierStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    //
    // Normalized Terminals
    //

    pub(super) fn build_pragma_string_literal(
        &mut self,
        source: &input::PragmaStringLiteral,
    ) -> output::StringLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::StringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_yul_decimal_literal(
        &mut self,
        source: &input::YulDecimalLiteral,
    ) -> output::DecimalLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::DecimalLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_yul_hex_literal(
        &mut self,
        source: &input::YulHexLiteral,
    ) -> output::HexLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::HexLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_yul_hex_string_literal(
        &mut self,
        source: &input::YulHexStringLiteral,
    ) -> output::HexStringLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::HexStringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_yul_identifier(
        &mut self,
        source: &input::YulIdentifier,
    ) -> output::Identifier {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::IdentifierStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }

    pub(super) fn build_yul_string_literal(
        &mut self,
        source: &input::YulStringLiteral,
    ) -> output::StringLiteral {
        let text = self.unparse_range(source.range.clone());
        Rc::new(output::StringLiteralStruct {
            id: self.next_id(),
            range: source.range.clone(),
            text,
        })
    }
}
