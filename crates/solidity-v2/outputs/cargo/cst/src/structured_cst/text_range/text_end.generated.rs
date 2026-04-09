// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::wildcard_imports)]

use crate::structured_cst::nodes::*;

/// A trait for CST nodes that can report the end offset of their text range.
pub(super) trait TextEnd {
    /// Returns the end offset of this node's text, or `None` if the node is empty.
    fn text_end(&self) -> Option<usize>;
}

impl<T: TextEnd> TextEnd for Option<T> {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().and_then(TextEnd::text_end)
    }
}

impl TextEnd for AbicoderPragmaStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.version.text_end())
            .or_else(|| self.abicoder_keyword.text_end())
    }
}

impl TextEnd for AbicoderPragma {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for AdditiveExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.expression_additive_expression_operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for AdditiveExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for AddressTypeStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.payable_keyword.text_end())
            .or_else(|| self.address_keyword.text_end())
    }
}

impl TextEnd for AddressType {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for AndExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for AndExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ArrayExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_bracket.text_end())
            .or_else(|| self.items.text_end())
            .or_else(|| self.open_bracket.text_end())
    }
}

impl TextEnd for ArrayExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ArrayTypeNameStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_bracket.text_end())
            .or_else(|| self.index.text_end())
            .or_else(|| self.open_bracket.text_end())
            .or_else(|| self.operand.text_end())
    }
}

impl TextEnd for ArrayTypeName {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for AssemblyStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.flags.text_end())
            .or_else(|| self.label.text_end())
            .or_else(|| self.assembly_keyword.text_end())
    }
}

impl TextEnd for AssemblyStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for AssignmentExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.expression_assignment_expression_operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for AssignmentExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for BitwiseAndExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for BitwiseAndExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for BitwiseOrExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for BitwiseOrExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for BitwiseXorExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for BitwiseXorExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for BlockStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.statements.text_end())
            .or_else(|| self.open_brace.text_end())
    }
}

impl TextEnd for Block {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for BreakStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.break_keyword.text_end())
    }
}

impl TextEnd for BreakStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for CallOptionsExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.options.text_end())
            .or_else(|| self.open_brace.text_end())
            .or_else(|| self.operand.text_end())
    }
}

impl TextEnd for CallOptionsExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for CatchClauseStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.error.text_end())
            .or_else(|| self.catch_keyword.text_end())
    }
}

impl TextEnd for CatchClause {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for CatchClauseErrorStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.parameters.text_end())
            .or_else(|| self.name.text_end())
    }
}

impl TextEnd for CatchClauseError {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ConditionalExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.false_expression.text_end())
            .or_else(|| self.colon.text_end())
            .or_else(|| self.true_expression.text_end())
            .or_else(|| self.question_mark.text_end())
            .or_else(|| self.operand.text_end())
    }
}

impl TextEnd for ConditionalExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ConstantDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.value.text_end())
            .or_else(|| self.equal.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.constant_keyword.text_end())
            .or_else(|| self.type_name.text_end())
    }
}

impl TextEnd for ConstantDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ConstructorDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.attributes.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.constructor_keyword.text_end())
    }
}

impl TextEnd for ConstructorDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ContinueStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.continue_keyword.text_end())
    }
}

impl TextEnd for ContinueStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ContractDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.members.text_end())
            .or_else(|| self.open_brace.text_end())
            .or_else(|| self.specifiers.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.contract_keyword.text_end())
            .or_else(|| self.abstract_keyword.text_end())
    }
}

impl TextEnd for ContractDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for DecimalNumberExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.unit.text_end())
            .or_else(|| self.literal.text_end())
    }
}

impl TextEnd for DecimalNumberExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for DoWhileStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.close_paren.text_end())
            .or_else(|| self.condition.text_end())
            .or_else(|| self.open_paren.text_end())
            .or_else(|| self.while_keyword.text_end())
            .or_else(|| self.body.text_end())
            .or_else(|| self.do_keyword.text_end())
    }
}

impl TextEnd for DoWhileStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ElseBranchStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.else_keyword.text_end())
    }
}

impl TextEnd for ElseBranch {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for EmitStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.arguments.text_end())
            .or_else(|| self.event.text_end())
            .or_else(|| self.emit_keyword.text_end())
    }
}

impl TextEnd for EmitStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for EnumDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.members.text_end())
            .or_else(|| self.open_brace.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.enum_keyword.text_end())
    }
}

impl TextEnd for EnumDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for EqualityExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.expression_equality_expression_operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for EqualityExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ErrorDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.members.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.error_keyword.text_end())
    }
}

impl TextEnd for ErrorDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ErrorParameterStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.text_end())
            .or_else(|| self.type_name.text_end())
    }
}

impl TextEnd for ErrorParameter {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ErrorParametersDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for ErrorParametersDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for EventDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.anonymous_keyword.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.event_keyword.text_end())
    }
}

impl TextEnd for EventDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for EventParameterStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.text_end())
            .or_else(|| self.indexed_keyword.text_end())
            .or_else(|| self.type_name.text_end())
    }
}

impl TextEnd for EventParameter {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for EventParametersDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for EventParametersDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ExperimentalPragmaStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.feature.text_end())
            .or_else(|| self.experimental_keyword.text_end())
    }
}

impl TextEnd for ExperimentalPragma {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ExponentiationExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for ExponentiationExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ExpressionStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.expression.text_end())
    }
}

impl TextEnd for ExpressionStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for FallbackFunctionDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.returns.text_end())
            .or_else(|| self.attributes.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.fallback_keyword.text_end())
    }
}

impl TextEnd for FallbackFunctionDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ForStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.close_paren.text_end())
            .or_else(|| self.iterator.text_end())
            .or_else(|| self.condition.text_end())
            .or_else(|| self.initialization.text_end())
            .or_else(|| self.open_paren.text_end())
            .or_else(|| self.for_keyword.text_end())
    }
}

impl TextEnd for ForStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for FunctionCallExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.arguments.text_end())
            .or_else(|| self.operand.text_end())
    }
}

impl TextEnd for FunctionCallExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for FunctionDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.returns.text_end())
            .or_else(|| self.attributes.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.function_keyword.text_end())
    }
}

impl TextEnd for FunctionDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for FunctionTypeStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.returns.text_end())
            .or_else(|| self.attributes.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.function_keyword.text_end())
    }
}

impl TextEnd for FunctionType {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for HexNumberExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.literal.text_end())
    }
}

impl TextEnd for HexNumberExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for IfStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.else_branch.text_end())
            .or_else(|| self.body.text_end())
            .or_else(|| self.close_paren.text_end())
            .or_else(|| self.condition.text_end())
            .or_else(|| self.open_paren.text_end())
            .or_else(|| self.if_keyword.text_end())
    }
}

impl TextEnd for IfStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ImportAliasStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.identifier.text_end())
            .or_else(|| self.as_keyword.text_end())
    }
}

impl TextEnd for ImportAlias {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ImportDeconstructionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.path.text_end())
            .or_else(|| self.from_keyword.text_end())
            .or_else(|| self.close_brace.text_end())
            .or_else(|| self.symbols.text_end())
            .or_else(|| self.open_brace.text_end())
    }
}

impl TextEnd for ImportDeconstruction {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ImportDeconstructionSymbolStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.alias.text_end())
            .or_else(|| self.name.text_end())
    }
}

impl TextEnd for ImportDeconstructionSymbol {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ImportDirectiveStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.clause.text_end())
            .or_else(|| self.import_keyword.text_end())
    }
}

impl TextEnd for ImportDirective {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for IndexAccessEndStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.end.text_end())
            .or_else(|| self.colon.text_end())
    }
}

impl TextEnd for IndexAccessEnd {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for IndexAccessExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_bracket.text_end())
            .or_else(|| self.end.text_end())
            .or_else(|| self.start.text_end())
            .or_else(|| self.open_bracket.text_end())
            .or_else(|| self.operand.text_end())
    }
}

impl TextEnd for IndexAccessExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for InequalityExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.expression_inequality_expression_operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for InequalityExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for InheritanceSpecifierStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.types.text_end())
            .or_else(|| self.is_keyword.text_end())
    }
}

impl TextEnd for InheritanceSpecifier {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for InheritanceTypeStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.arguments.text_end())
            .or_else(|| self.type_name.text_end())
    }
}

impl TextEnd for InheritanceType {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for InterfaceDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.members.text_end())
            .or_else(|| self.open_brace.text_end())
            .or_else(|| self.inheritance.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.interface_keyword.text_end())
    }
}

impl TextEnd for InterfaceDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for LibraryDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.members.text_end())
            .or_else(|| self.open_brace.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.library_keyword.text_end())
    }
}

impl TextEnd for LibraryDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for MappingKeyStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.text_end())
            .or_else(|| self.key_type.text_end())
    }
}

impl TextEnd for MappingKey {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for MappingTypeStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.value_type.text_end())
            .or_else(|| self.equal_greater_than.text_end())
            .or_else(|| self.key_type.text_end())
            .or_else(|| self.open_paren.text_end())
            .or_else(|| self.mapping_keyword.text_end())
    }
}

impl TextEnd for MappingType {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for MappingValueStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.text_end())
            .or_else(|| self.type_name.text_end())
    }
}

impl TextEnd for MappingValue {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for MemberAccessExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.member.text_end())
            .or_else(|| self.period.text_end())
            .or_else(|| self.operand.text_end())
    }
}

impl TextEnd for MemberAccessExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ModifierDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.attributes.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.modifier_keyword.text_end())
    }
}

impl TextEnd for ModifierDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ModifierInvocationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.arguments.text_end())
            .or_else(|| self.name.text_end())
    }
}

impl TextEnd for ModifierInvocation {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for MultiTypedDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.text_end())
            .or_else(|| self.close_paren.text_end())
            .or_else(|| self.elements.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for MultiTypedDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for MultiTypedDeclarationElementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.member.text_end())
    }
}

impl TextEnd for MultiTypedDeclarationElement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for MultiplicativeExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| {
                self.expression_multiplicative_expression_operator
                    .text_end()
            })
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for MultiplicativeExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for NamedArgumentStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.text_end())
            .or_else(|| self.colon.text_end())
            .or_else(|| self.name.text_end())
    }
}

impl TextEnd for NamedArgument {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for NamedArgumentGroupStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.arguments.text_end())
            .or_else(|| self.open_brace.text_end())
    }
}

impl TextEnd for NamedArgumentGroup {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for NamedArgumentsDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.arguments.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for NamedArgumentsDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for NamedImportStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.path.text_end())
            .or_else(|| self.from_keyword.text_end())
            .or_else(|| self.alias.text_end())
            .or_else(|| self.asterisk.text_end())
    }
}

impl TextEnd for NamedImport {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for NewExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_end())
            .or_else(|| self.new_keyword.text_end())
    }
}

impl TextEnd for NewExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for OrExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for OrExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for OverridePathsDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.paths.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for OverridePathsDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for OverrideSpecifierStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.overridden.text_end())
            .or_else(|| self.override_keyword.text_end())
    }
}

impl TextEnd for OverrideSpecifier {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ParameterStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.text_end())
            .or_else(|| self.storage_location.text_end())
            .or_else(|| self.type_name.text_end())
    }
}

impl TextEnd for Parameter {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ParametersDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for ParametersDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for PathImportStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.alias.text_end())
            .or_else(|| self.path.text_end())
    }
}

impl TextEnd for PathImport {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for PositionalArgumentsDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.arguments.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for PositionalArgumentsDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for PostfixExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression_postfix_expression_operator.text_end())
            .or_else(|| self.operand.text_end())
    }
}

impl TextEnd for PostfixExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for PragmaDirectiveStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.pragma.text_end())
            .or_else(|| self.pragma_keyword.text_end())
    }
}

impl TextEnd for PragmaDirective {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for PrefixExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.operand.text_end())
            .or_else(|| self.expression_prefix_expression_operator.text_end())
    }
}

impl TextEnd for PrefixExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ReceiveFunctionDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.attributes.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.receive_keyword.text_end())
    }
}

impl TextEnd for ReceiveFunctionDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ReturnStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.expression.text_end())
            .or_else(|| self.return_keyword.text_end())
    }
}

impl TextEnd for ReturnStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ReturnsDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.variables.text_end())
            .or_else(|| self.returns_keyword.text_end())
    }
}

impl TextEnd for ReturnsDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for RevertStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.arguments.text_end())
            .or_else(|| self.error.text_end())
            .or_else(|| self.revert_keyword.text_end())
    }
}

impl TextEnd for RevertStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for ShiftExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.text_end())
            .or_else(|| self.expression_shift_expression_operator.text_end())
            .or_else(|| self.left_operand.text_end())
    }
}

impl TextEnd for ShiftExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for SingleTypedDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.text_end())
            .or_else(|| self.declaration.text_end())
    }
}

impl TextEnd for SingleTypedDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for SourceUnitStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.members.text_end())
    }
}

impl TextEnd for SourceUnit {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for StateVariableDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.value.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.attributes.text_end())
            .or_else(|| self.type_name.text_end())
    }
}

impl TextEnd for StateVariableDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for StateVariableDefinitionValueStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.text_end())
            .or_else(|| self.equal.text_end())
    }
}

impl TextEnd for StateVariableDefinitionValue {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for StorageLayoutSpecifierStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.text_end())
            .or_else(|| self.at_keyword.text_end())
            .or_else(|| self.layout_keyword.text_end())
    }
}

impl TextEnd for StorageLayoutSpecifier {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for StructDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.members.text_end())
            .or_else(|| self.open_brace.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.struct_keyword.text_end())
    }
}

impl TextEnd for StructDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for StructMemberStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.type_name.text_end())
    }
}

impl TextEnd for StructMember {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for TryStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.catch_clauses.text_end())
            .or_else(|| self.body.text_end())
            .or_else(|| self.returns.text_end())
            .or_else(|| self.expression.text_end())
            .or_else(|| self.try_keyword.text_end())
    }
}

impl TextEnd for TryStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for TupleExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.items.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for TupleExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for TupleValueStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.text_end())
    }
}

impl TextEnd for TupleValue {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for TypeExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.type_name.text_end())
            .or_else(|| self.open_paren.text_end())
            .or_else(|| self.type_keyword.text_end())
    }
}

impl TextEnd for TypeExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for UncheckedBlockStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.block.text_end())
            .or_else(|| self.unchecked_keyword.text_end())
    }
}

impl TextEnd for UncheckedBlock {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for UserDefinedValueTypeDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.value_type.text_end())
            .or_else(|| self.is_keyword.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.type_keyword.text_end())
    }
}

impl TextEnd for UserDefinedValueTypeDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for UsingAliasStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.operator.text_end())
            .or_else(|| self.as_keyword.text_end())
    }
}

impl TextEnd for UsingAlias {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for UsingDeconstructionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.symbols.text_end())
            .or_else(|| self.open_brace.text_end())
    }
}

impl TextEnd for UsingDeconstruction {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for UsingDeconstructionSymbolStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.alias.text_end())
            .or_else(|| self.name.text_end())
    }
}

impl TextEnd for UsingDeconstructionSymbol {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for UsingDirectiveStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.global_keyword.text_end())
            .or_else(|| self.target.text_end())
            .or_else(|| self.for_keyword.text_end())
            .or_else(|| self.clause.text_end())
            .or_else(|| self.using_keyword.text_end())
    }
}

impl TextEnd for UsingDirective {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for VariableDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.text_end())
            .or_else(|| self.storage_location.text_end())
            .or_else(|| self.type_name.text_end())
    }
}

impl TextEnd for VariableDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for VariableDeclarationStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.text_end())
            .or_else(|| self.target.text_end())
    }
}

impl TextEnd for VariableDeclarationStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for VariableDeclarationValueStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.text_end())
            .or_else(|| self.equal.text_end())
    }
}

impl TextEnd for VariableDeclarationValue {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for VersionPragmaStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.sets.text_end())
            .or_else(|| self.solidity_keyword.text_end())
    }
}

impl TextEnd for VersionPragma {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for VersionRangeStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.end.text_end())
            .or_else(|| self.minus.text_end())
            .or_else(|| self.start.text_end())
    }
}

impl TextEnd for VersionRange {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for VersionTermStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.literal.text_end())
            .or_else(|| self.operator.text_end())
    }
}

impl TextEnd for VersionTerm {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for WhileStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.close_paren.text_end())
            .or_else(|| self.condition.text_end())
            .or_else(|| self.open_paren.text_end())
            .or_else(|| self.while_keyword.text_end())
    }
}

impl TextEnd for WhileStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulBlockStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.text_end())
            .or_else(|| self.statements.text_end())
            .or_else(|| self.open_brace.text_end())
    }
}

impl TextEnd for YulBlock {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulBreakStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.break_keyword.text_end())
    }
}

impl TextEnd for YulBreakStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulContinueStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.continue_keyword.text_end())
    }
}

impl TextEnd for YulContinueStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulDefaultCaseStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.default_keyword.text_end())
    }
}

impl TextEnd for YulDefaultCase {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulFlagsDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.flags.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for YulFlagsDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulForStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.iterator.text_end())
            .or_else(|| self.condition.text_end())
            .or_else(|| self.initialization.text_end())
            .or_else(|| self.for_keyword.text_end())
    }
}

impl TextEnd for YulForStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulFunctionCallExpressionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.arguments.text_end())
            .or_else(|| self.open_paren.text_end())
            .or_else(|| self.operand.text_end())
    }
}

impl TextEnd for YulFunctionCallExpression {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulFunctionDefinitionStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.returns.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.name.text_end())
            .or_else(|| self.function_keyword.text_end())
    }
}

impl TextEnd for YulFunctionDefinition {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulIfStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.condition.text_end())
            .or_else(|| self.if_keyword.text_end())
    }
}

impl TextEnd for YulIfStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulLeaveStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.leave_keyword.text_end())
    }
}

impl TextEnd for YulLeaveStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulParametersDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.text_end())
            .or_else(|| self.parameters.text_end())
            .or_else(|| self.open_paren.text_end())
    }
}

impl TextEnd for YulParametersDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulReturnsDeclarationStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.variables.text_end())
            .or_else(|| self.minus_greater_than.text_end())
    }
}

impl TextEnd for YulReturnsDeclaration {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulSwitchStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.cases.text_end())
            .or_else(|| self.expression.text_end())
            .or_else(|| self.switch_keyword.text_end())
    }
}

impl TextEnd for YulSwitchStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulValueCaseStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.text_end())
            .or_else(|| self.value.text_end())
            .or_else(|| self.case_keyword.text_end())
    }
}

impl TextEnd for YulValueCase {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulVariableAssignmentStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.text_end())
            .or_else(|| self.assignment.text_end())
            .or_else(|| self.variables.text_end())
    }
}

impl TextEnd for YulVariableAssignmentStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulVariableDeclarationStatementStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.text_end())
            .or_else(|| self.variables.text_end())
            .or_else(|| self.let_keyword.text_end())
    }
}

impl TextEnd for YulVariableDeclarationStatement {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for YulVariableDeclarationValueStruct {
    fn text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.text_end())
            .or_else(|| self.assignment.text_end())
    }
}

impl TextEnd for YulVariableDeclarationValue {
    fn text_end(&self) -> Option<usize> {
        self.as_ref().text_end()
    }
}

impl TextEnd for AbicoderVersion {
    fn text_end(&self) -> Option<usize> {
        match self {
            AbicoderVersion::AbicoderV1Keyword(child) => child.text_end(),
            AbicoderVersion::AbicoderV2Keyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for ArgumentsDeclaration {
    fn text_end(&self) -> Option<usize> {
        match self {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(child) => child.text_end(),
            ArgumentsDeclaration::NamedArgumentsDeclaration(child) => child.text_end(),
        }
    }
}

impl TextEnd for ConstructorAttribute {
    fn text_end(&self) -> Option<usize> {
        match self {
            ConstructorAttribute::ModifierInvocation(child) => child.text_end(),
            ConstructorAttribute::InternalKeyword(child) => child.text_end(),
            ConstructorAttribute::PayableKeyword(child) => child.text_end(),
            ConstructorAttribute::PublicKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for ContractMember {
    fn text_end(&self) -> Option<usize> {
        match self {
            ContractMember::UsingDirective(child) => child.text_end(),
            ContractMember::FunctionDefinition(child) => child.text_end(),
            ContractMember::ConstructorDefinition(child) => child.text_end(),
            ContractMember::ReceiveFunctionDefinition(child) => child.text_end(),
            ContractMember::FallbackFunctionDefinition(child) => child.text_end(),
            ContractMember::ModifierDefinition(child) => child.text_end(),
            ContractMember::StructDefinition(child) => child.text_end(),
            ContractMember::EnumDefinition(child) => child.text_end(),
            ContractMember::EventDefinition(child) => child.text_end(),
            ContractMember::ErrorDefinition(child) => child.text_end(),
            ContractMember::UserDefinedValueTypeDefinition(child) => child.text_end(),
            ContractMember::StateVariableDefinition(child) => child.text_end(),
        }
    }
}

impl TextEnd for ContractSpecifier {
    fn text_end(&self) -> Option<usize> {
        match self {
            ContractSpecifier::InheritanceSpecifier(child) => child.text_end(),
            ContractSpecifier::StorageLayoutSpecifier(child) => child.text_end(),
        }
    }
}

impl TextEnd for ElementaryType {
    fn text_end(&self) -> Option<usize> {
        match self {
            ElementaryType::BoolKeyword(child) => child.text_end(),
            ElementaryType::StringKeyword(child) => child.text_end(),
            ElementaryType::AddressType(child) => child.text_end(),
            ElementaryType::BytesKeyword(child) => child.text_end(),
            ElementaryType::IntKeyword(child) => child.text_end(),
            ElementaryType::UintKeyword(child) => child.text_end(),
            ElementaryType::FixedKeyword(child) => child.text_end(),
            ElementaryType::UfixedKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for ExperimentalFeature {
    fn text_end(&self) -> Option<usize> {
        match self {
            ExperimentalFeature::ABIEncoderV2Keyword(child) => child.text_end(),
            ExperimentalFeature::SMTCheckerKeyword(child) => child.text_end(),
            ExperimentalFeature::PragmaStringLiteral(child) => child.text_end(),
        }
    }
}

impl TextEnd for Expression {
    fn text_end(&self) -> Option<usize> {
        match self {
            Expression::AssignmentExpression(child) => child.text_end(),
            Expression::ConditionalExpression(child) => child.text_end(),
            Expression::OrExpression(child) => child.text_end(),
            Expression::AndExpression(child) => child.text_end(),
            Expression::EqualityExpression(child) => child.text_end(),
            Expression::InequalityExpression(child) => child.text_end(),
            Expression::BitwiseOrExpression(child) => child.text_end(),
            Expression::BitwiseXorExpression(child) => child.text_end(),
            Expression::BitwiseAndExpression(child) => child.text_end(),
            Expression::ShiftExpression(child) => child.text_end(),
            Expression::AdditiveExpression(child) => child.text_end(),
            Expression::MultiplicativeExpression(child) => child.text_end(),
            Expression::ExponentiationExpression(child) => child.text_end(),
            Expression::PostfixExpression(child) => child.text_end(),
            Expression::PrefixExpression(child) => child.text_end(),
            Expression::FunctionCallExpression(child) => child.text_end(),
            Expression::CallOptionsExpression(child) => child.text_end(),
            Expression::MemberAccessExpression(child) => child.text_end(),
            Expression::IndexAccessExpression(child) => child.text_end(),
            Expression::NewExpression(child) => child.text_end(),
            Expression::TupleExpression(child) => child.text_end(),
            Expression::TypeExpression(child) => child.text_end(),
            Expression::ArrayExpression(child) => child.text_end(),
            Expression::HexNumberExpression(child) => child.text_end(),
            Expression::DecimalNumberExpression(child) => child.text_end(),
            Expression::StringExpression(child) => child.text_end(),
            Expression::ElementaryType(child) => child.text_end(),
            Expression::PayableKeyword(child) => child.text_end(),
            Expression::ThisKeyword(child) => child.text_end(),
            Expression::SuperKeyword(child) => child.text_end(),
            Expression::TrueKeyword(child) => child.text_end(),
            Expression::FalseKeyword(child) => child.text_end(),
            Expression::Identifier(child) => child.text_end(),
        }
    }
}

impl TextEnd for Expression_AdditiveExpression_Operator {
    fn text_end(&self) -> Option<usize> {
        match self {
            Expression_AdditiveExpression_Operator::Minus(child) => child.text_end(),
            Expression_AdditiveExpression_Operator::Plus(child) => child.text_end(),
        }
    }
}

impl TextEnd for Expression_AssignmentExpression_Operator {
    fn text_end(&self) -> Option<usize> {
        match self {
            Expression_AssignmentExpression_Operator::AmpersandEqual(child) => child.text_end(),
            Expression_AssignmentExpression_Operator::AsteriskEqual(child) => child.text_end(),
            Expression_AssignmentExpression_Operator::BarEqual(child) => child.text_end(),
            Expression_AssignmentExpression_Operator::CaretEqual(child) => child.text_end(),
            Expression_AssignmentExpression_Operator::Equal(child) => child.text_end(),
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(child) => {
                child.text_end()
            }
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(
                child,
            ) => child.text_end(),
            Expression_AssignmentExpression_Operator::LessThanLessThanEqual(child) => {
                child.text_end()
            }
            Expression_AssignmentExpression_Operator::MinusEqual(child) => child.text_end(),
            Expression_AssignmentExpression_Operator::PercentEqual(child) => child.text_end(),
            Expression_AssignmentExpression_Operator::PlusEqual(child) => child.text_end(),
            Expression_AssignmentExpression_Operator::SlashEqual(child) => child.text_end(),
        }
    }
}

impl TextEnd for Expression_EqualityExpression_Operator {
    fn text_end(&self) -> Option<usize> {
        match self {
            Expression_EqualityExpression_Operator::BangEqual(child) => child.text_end(),
            Expression_EqualityExpression_Operator::EqualEqual(child) => child.text_end(),
        }
    }
}

impl TextEnd for Expression_InequalityExpression_Operator {
    fn text_end(&self) -> Option<usize> {
        match self {
            Expression_InequalityExpression_Operator::GreaterThan(child) => child.text_end(),
            Expression_InequalityExpression_Operator::GreaterThanEqual(child) => child.text_end(),
            Expression_InequalityExpression_Operator::LessThan(child) => child.text_end(),
            Expression_InequalityExpression_Operator::LessThanEqual(child) => child.text_end(),
        }
    }
}

impl TextEnd for Expression_MultiplicativeExpression_Operator {
    fn text_end(&self) -> Option<usize> {
        match self {
            Expression_MultiplicativeExpression_Operator::Asterisk(child) => child.text_end(),
            Expression_MultiplicativeExpression_Operator::Percent(child) => child.text_end(),
            Expression_MultiplicativeExpression_Operator::Slash(child) => child.text_end(),
        }
    }
}

impl TextEnd for Expression_PostfixExpression_Operator {
    fn text_end(&self) -> Option<usize> {
        match self {
            Expression_PostfixExpression_Operator::MinusMinus(child) => child.text_end(),
            Expression_PostfixExpression_Operator::PlusPlus(child) => child.text_end(),
        }
    }
}

impl TextEnd for Expression_PrefixExpression_Operator {
    fn text_end(&self) -> Option<usize> {
        match self {
            Expression_PrefixExpression_Operator::Bang(child) => child.text_end(),
            Expression_PrefixExpression_Operator::DeleteKeyword(child) => child.text_end(),
            Expression_PrefixExpression_Operator::Minus(child) => child.text_end(),
            Expression_PrefixExpression_Operator::MinusMinus(child) => child.text_end(),
            Expression_PrefixExpression_Operator::PlusPlus(child) => child.text_end(),
            Expression_PrefixExpression_Operator::Tilde(child) => child.text_end(),
        }
    }
}

impl TextEnd for Expression_ShiftExpression_Operator {
    fn text_end(&self) -> Option<usize> {
        match self {
            Expression_ShiftExpression_Operator::GreaterThanGreaterThan(child) => child.text_end(),
            Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(child) => {
                child.text_end()
            }
            Expression_ShiftExpression_Operator::LessThanLessThan(child) => child.text_end(),
        }
    }
}

impl TextEnd for FallbackFunctionAttribute {
    fn text_end(&self) -> Option<usize> {
        match self {
            FallbackFunctionAttribute::ModifierInvocation(child) => child.text_end(),
            FallbackFunctionAttribute::OverrideSpecifier(child) => child.text_end(),
            FallbackFunctionAttribute::ExternalKeyword(child) => child.text_end(),
            FallbackFunctionAttribute::PayableKeyword(child) => child.text_end(),
            FallbackFunctionAttribute::PureKeyword(child) => child.text_end(),
            FallbackFunctionAttribute::ViewKeyword(child) => child.text_end(),
            FallbackFunctionAttribute::VirtualKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for ForStatementCondition {
    fn text_end(&self) -> Option<usize> {
        match self {
            ForStatementCondition::ExpressionStatement(child) => child.text_end(),
            ForStatementCondition::Semicolon(child) => child.text_end(),
        }
    }
}

impl TextEnd for ForStatementInitialization {
    fn text_end(&self) -> Option<usize> {
        match self {
            ForStatementInitialization::VariableDeclarationStatement(child) => child.text_end(),
            ForStatementInitialization::ExpressionStatement(child) => child.text_end(),
            ForStatementInitialization::Semicolon(child) => child.text_end(),
        }
    }
}

impl TextEnd for FunctionAttribute {
    fn text_end(&self) -> Option<usize> {
        match self {
            FunctionAttribute::ModifierInvocation(child) => child.text_end(),
            FunctionAttribute::OverrideSpecifier(child) => child.text_end(),
            FunctionAttribute::ExternalKeyword(child) => child.text_end(),
            FunctionAttribute::InternalKeyword(child) => child.text_end(),
            FunctionAttribute::PayableKeyword(child) => child.text_end(),
            FunctionAttribute::PrivateKeyword(child) => child.text_end(),
            FunctionAttribute::PublicKeyword(child) => child.text_end(),
            FunctionAttribute::PureKeyword(child) => child.text_end(),
            FunctionAttribute::ViewKeyword(child) => child.text_end(),
            FunctionAttribute::VirtualKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for FunctionBody {
    fn text_end(&self) -> Option<usize> {
        match self {
            FunctionBody::Block(child) => child.text_end(),
            FunctionBody::Semicolon(child) => child.text_end(),
        }
    }
}

impl TextEnd for FunctionName {
    fn text_end(&self) -> Option<usize> {
        match self {
            FunctionName::Identifier(child) => child.text_end(),
            FunctionName::FallbackKeyword(child) => child.text_end(),
            FunctionName::ReceiveKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for FunctionTypeAttribute {
    fn text_end(&self) -> Option<usize> {
        match self {
            FunctionTypeAttribute::InternalKeyword(child) => child.text_end(),
            FunctionTypeAttribute::ExternalKeyword(child) => child.text_end(),
            FunctionTypeAttribute::PrivateKeyword(child) => child.text_end(),
            FunctionTypeAttribute::PublicKeyword(child) => child.text_end(),
            FunctionTypeAttribute::PureKeyword(child) => child.text_end(),
            FunctionTypeAttribute::ViewKeyword(child) => child.text_end(),
            FunctionTypeAttribute::PayableKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for IdentifierPathElement {
    fn text_end(&self) -> Option<usize> {
        match self {
            IdentifierPathElement::Identifier(child) => child.text_end(),
            IdentifierPathElement::AddressKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for ImportClause {
    fn text_end(&self) -> Option<usize> {
        match self {
            ImportClause::PathImport(child) => child.text_end(),
            ImportClause::NamedImport(child) => child.text_end(),
            ImportClause::ImportDeconstruction(child) => child.text_end(),
        }
    }
}

impl TextEnd for MappingKeyType {
    fn text_end(&self) -> Option<usize> {
        match self {
            MappingKeyType::ElementaryType(child) => child.text_end(),
            MappingKeyType::IdentifierPath(child) => child.text_end(),
        }
    }
}

impl TextEnd for ModifierAttribute {
    fn text_end(&self) -> Option<usize> {
        match self {
            ModifierAttribute::OverrideSpecifier(child) => child.text_end(),
            ModifierAttribute::VirtualKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for NumberUnit {
    fn text_end(&self) -> Option<usize> {
        match self {
            NumberUnit::WeiKeyword(child) => child.text_end(),
            NumberUnit::GweiKeyword(child) => child.text_end(),
            NumberUnit::EtherKeyword(child) => child.text_end(),
            NumberUnit::SecondsKeyword(child) => child.text_end(),
            NumberUnit::MinutesKeyword(child) => child.text_end(),
            NumberUnit::HoursKeyword(child) => child.text_end(),
            NumberUnit::DaysKeyword(child) => child.text_end(),
            NumberUnit::WeeksKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for Pragma {
    fn text_end(&self) -> Option<usize> {
        match self {
            Pragma::VersionPragma(child) => child.text_end(),
            Pragma::AbicoderPragma(child) => child.text_end(),
            Pragma::ExperimentalPragma(child) => child.text_end(),
        }
    }
}

impl TextEnd for ReceiveFunctionAttribute {
    fn text_end(&self) -> Option<usize> {
        match self {
            ReceiveFunctionAttribute::ModifierInvocation(child) => child.text_end(),
            ReceiveFunctionAttribute::OverrideSpecifier(child) => child.text_end(),
            ReceiveFunctionAttribute::ExternalKeyword(child) => child.text_end(),
            ReceiveFunctionAttribute::PayableKeyword(child) => child.text_end(),
            ReceiveFunctionAttribute::VirtualKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for SourceUnitMember {
    fn text_end(&self) -> Option<usize> {
        match self {
            SourceUnitMember::PragmaDirective(child) => child.text_end(),
            SourceUnitMember::ImportDirective(child) => child.text_end(),
            SourceUnitMember::ContractDefinition(child) => child.text_end(),
            SourceUnitMember::InterfaceDefinition(child) => child.text_end(),
            SourceUnitMember::LibraryDefinition(child) => child.text_end(),
            SourceUnitMember::StructDefinition(child) => child.text_end(),
            SourceUnitMember::EnumDefinition(child) => child.text_end(),
            SourceUnitMember::FunctionDefinition(child) => child.text_end(),
            SourceUnitMember::ErrorDefinition(child) => child.text_end(),
            SourceUnitMember::UserDefinedValueTypeDefinition(child) => child.text_end(),
            SourceUnitMember::UsingDirective(child) => child.text_end(),
            SourceUnitMember::EventDefinition(child) => child.text_end(),
            SourceUnitMember::ConstantDefinition(child) => child.text_end(),
        }
    }
}

impl TextEnd for StateVariableAttribute {
    fn text_end(&self) -> Option<usize> {
        match self {
            StateVariableAttribute::OverrideSpecifier(child) => child.text_end(),
            StateVariableAttribute::ConstantKeyword(child) => child.text_end(),
            StateVariableAttribute::InternalKeyword(child) => child.text_end(),
            StateVariableAttribute::PrivateKeyword(child) => child.text_end(),
            StateVariableAttribute::PublicKeyword(child) => child.text_end(),
            StateVariableAttribute::ImmutableKeyword(child) => child.text_end(),
            StateVariableAttribute::TransientKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for Statement {
    fn text_end(&self) -> Option<usize> {
        match self {
            Statement::IfStatement(child) => child.text_end(),
            Statement::ForStatement(child) => child.text_end(),
            Statement::WhileStatement(child) => child.text_end(),
            Statement::DoWhileStatement(child) => child.text_end(),
            Statement::ContinueStatement(child) => child.text_end(),
            Statement::BreakStatement(child) => child.text_end(),
            Statement::ReturnStatement(child) => child.text_end(),
            Statement::EmitStatement(child) => child.text_end(),
            Statement::TryStatement(child) => child.text_end(),
            Statement::RevertStatement(child) => child.text_end(),
            Statement::AssemblyStatement(child) => child.text_end(),
            Statement::Block(child) => child.text_end(),
            Statement::UncheckedBlock(child) => child.text_end(),
            Statement::VariableDeclarationStatement(child) => child.text_end(),
            Statement::ExpressionStatement(child) => child.text_end(),
        }
    }
}

impl TextEnd for StorageLocation {
    fn text_end(&self) -> Option<usize> {
        match self {
            StorageLocation::MemoryKeyword(child) => child.text_end(),
            StorageLocation::StorageKeyword(child) => child.text_end(),
            StorageLocation::CallDataKeyword(child) => child.text_end(),
        }
    }
}

impl TextEnd for StringExpression {
    fn text_end(&self) -> Option<usize> {
        match self {
            StringExpression::StringLiterals(child) => child.text_end(),
            StringExpression::HexStringLiterals(child) => child.text_end(),
            StringExpression::UnicodeStringLiterals(child) => child.text_end(),
        }
    }
}

impl TextEnd for TypeName {
    fn text_end(&self) -> Option<usize> {
        match self {
            TypeName::ArrayTypeName(child) => child.text_end(),
            TypeName::FunctionType(child) => child.text_end(),
            TypeName::MappingType(child) => child.text_end(),
            TypeName::ElementaryType(child) => child.text_end(),
            TypeName::IdentifierPath(child) => child.text_end(),
        }
    }
}

impl TextEnd for UsingClause {
    fn text_end(&self) -> Option<usize> {
        match self {
            UsingClause::IdentifierPath(child) => child.text_end(),
            UsingClause::UsingDeconstruction(child) => child.text_end(),
        }
    }
}

impl TextEnd for UsingOperator {
    fn text_end(&self) -> Option<usize> {
        match self {
            UsingOperator::Ampersand(child) => child.text_end(),
            UsingOperator::Asterisk(child) => child.text_end(),
            UsingOperator::BangEqual(child) => child.text_end(),
            UsingOperator::Bar(child) => child.text_end(),
            UsingOperator::Caret(child) => child.text_end(),
            UsingOperator::EqualEqual(child) => child.text_end(),
            UsingOperator::GreaterThan(child) => child.text_end(),
            UsingOperator::GreaterThanEqual(child) => child.text_end(),
            UsingOperator::LessThan(child) => child.text_end(),
            UsingOperator::LessThanEqual(child) => child.text_end(),
            UsingOperator::Minus(child) => child.text_end(),
            UsingOperator::Percent(child) => child.text_end(),
            UsingOperator::Plus(child) => child.text_end(),
            UsingOperator::Slash(child) => child.text_end(),
            UsingOperator::Tilde(child) => child.text_end(),
        }
    }
}

impl TextEnd for UsingTarget {
    fn text_end(&self) -> Option<usize> {
        match self {
            UsingTarget::TypeName(child) => child.text_end(),
            UsingTarget::Asterisk(child) => child.text_end(),
        }
    }
}

impl TextEnd for VariableDeclarationTarget {
    fn text_end(&self) -> Option<usize> {
        match self {
            VariableDeclarationTarget::SingleTypedDeclaration(child) => child.text_end(),
            VariableDeclarationTarget::MultiTypedDeclaration(child) => child.text_end(),
        }
    }
}

impl TextEnd for VersionExpression {
    fn text_end(&self) -> Option<usize> {
        match self {
            VersionExpression::VersionRange(child) => child.text_end(),
            VersionExpression::VersionTerm(child) => child.text_end(),
        }
    }
}

impl TextEnd for VersionLiteral {
    fn text_end(&self) -> Option<usize> {
        match self {
            VersionLiteral::SimpleVersionLiteral(child) => child.text_end(),
            VersionLiteral::PragmaStringLiteral(child) => child.text_end(),
        }
    }
}

impl TextEnd for VersionOperator {
    fn text_end(&self) -> Option<usize> {
        match self {
            VersionOperator::PragmaCaret(child) => child.text_end(),
            VersionOperator::PragmaTilde(child) => child.text_end(),
            VersionOperator::PragmaEqual(child) => child.text_end(),
            VersionOperator::PragmaLessThan(child) => child.text_end(),
            VersionOperator::PragmaGreaterThan(child) => child.text_end(),
            VersionOperator::PragmaLessThanEqual(child) => child.text_end(),
            VersionOperator::PragmaGreaterThanEqual(child) => child.text_end(),
        }
    }
}

impl TextEnd for YulExpression {
    fn text_end(&self) -> Option<usize> {
        match self {
            YulExpression::YulFunctionCallExpression(child) => child.text_end(),
            YulExpression::YulLiteral(child) => child.text_end(),
            YulExpression::YulPath(child) => child.text_end(),
        }
    }
}

impl TextEnd for YulLiteral {
    fn text_end(&self) -> Option<usize> {
        match self {
            YulLiteral::YulTrueKeyword(child) => child.text_end(),
            YulLiteral::YulFalseKeyword(child) => child.text_end(),
            YulLiteral::YulDecimalLiteral(child) => child.text_end(),
            YulLiteral::YulHexLiteral(child) => child.text_end(),
            YulLiteral::YulHexStringLiteral(child) => child.text_end(),
            YulLiteral::YulStringLiteral(child) => child.text_end(),
        }
    }
}

impl TextEnd for YulStatement {
    fn text_end(&self) -> Option<usize> {
        match self {
            YulStatement::YulBlock(child) => child.text_end(),
            YulStatement::YulFunctionDefinition(child) => child.text_end(),
            YulStatement::YulIfStatement(child) => child.text_end(),
            YulStatement::YulForStatement(child) => child.text_end(),
            YulStatement::YulSwitchStatement(child) => child.text_end(),
            YulStatement::YulLeaveStatement(child) => child.text_end(),
            YulStatement::YulBreakStatement(child) => child.text_end(),
            YulStatement::YulContinueStatement(child) => child.text_end(),
            YulStatement::YulVariableAssignmentStatement(child) => child.text_end(),
            YulStatement::YulVariableDeclarationStatement(child) => child.text_end(),
            YulStatement::YulExpression(child) => child.text_end(),
        }
    }
}

impl TextEnd for YulSwitchCase {
    fn text_end(&self) -> Option<usize> {
        match self {
            YulSwitchCase::YulDefaultCase(child) => child.text_end(),
            YulSwitchCase::YulValueCase(child) => child.text_end(),
        }
    }
}

impl TextEnd for ArrayValues {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for CallOptions {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for CatchClauses {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for ConstructorAttributes {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for ContractMembers {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for ContractSpecifiers {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for EnumMembers {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for ErrorParameters {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for EventParameters {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for FallbackFunctionAttributes {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for FunctionAttributes {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for FunctionTypeAttributes {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for HexStringLiterals {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for IdentifierPath {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for ImportDeconstructionSymbols {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for InheritanceTypes {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for InterfaceMembers {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for LibraryMembers {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for ModifierAttributes {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for MultiTypedDeclarationElements {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for NamedArguments {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for OverridePaths {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for Parameters {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for PositionalArguments {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for ReceiveFunctionAttributes {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for SimpleVersionLiteral {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for SourceUnitMembers {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for StateVariableAttributes {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for Statements {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for StringLiterals {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for StructMembers {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for TupleValues {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for UnicodeStringLiterals {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for UsingDeconstructionSymbols {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for VersionExpressionSet {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for VersionExpressionSets {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for YulArguments {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for YulFlags {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for YulParameters {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for YulPath {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for YulPaths {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for YulStatements {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for YulSwitchCases {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for YulVariableNames {
    fn text_end(&self) -> Option<usize> {
        self.elements.iter().rev().find_map(TextEnd::text_end)
    }
}

impl TextEnd for ABIEncoderV2Keyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AbicoderKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AbicoderV1Keyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AbicoderV2Keyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AbstractKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AddressKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AfterKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AliasKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Ampersand {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AmpersandAmpersand {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AmpersandEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AnonymousKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ApplyKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AsKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AssemblyKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Asterisk {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AsteriskAsterisk {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AsteriskEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AtKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AutoKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Bang {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BangEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Bar {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BarBar {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BarEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BoolKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BreakKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ByteKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BytesKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CallDataKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Caret {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CaretEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CaseKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CatchKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CloseBrace {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CloseBracket {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CloseParen {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Colon {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Comma {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ConstantKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ConstructorKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ContinueKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ContractKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CopyOfKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DaysKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DecimalLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DefaultKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DefineKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DeleteKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DoKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ElseKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EmitKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EnumKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Equal {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EqualEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EqualGreaterThan {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ErrorKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EtherKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EventKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ExperimentalKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ExternalKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FallbackKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FalseKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FinalKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FixedKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ForKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FromKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FunctionKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GlobalKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThan {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanGreaterThan {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanGreaterThanEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanGreaterThanGreaterThan {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanGreaterThanGreaterThanEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GweiKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for HexKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for HexLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for HexStringLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for HoursKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Identifier {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for IfKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ImmutableKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ImplementsKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ImportKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for InKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for IndexedKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for InlineKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for IntKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for InterfaceKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for InternalKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for IsKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LayoutKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LessThan {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LessThanEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LessThanLessThan {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LessThanLessThanEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LetKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LibraryKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MacroKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MappingKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MatchKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MemoryKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Minus {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MinusEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MinusMinus {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MinutesKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ModifierKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MutableKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for NewKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for NullKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OfKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OpenBrace {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OpenBracket {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OpenParen {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OverrideKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PartialKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PayableKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Percent {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PercentEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Period {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Plus {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PlusEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PlusPlus {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaBarBar {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaCaret {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaGreaterThan {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaGreaterThanEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaLessThan {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaLessThanEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaMinus {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaPeriod {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaSemicolon {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaStringLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaTilde {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PrivateKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PromiseKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PublicKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PureKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for QuestionMark {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ReceiveKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ReferenceKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for RelocatableKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ReturnKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ReturnsKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for RevertKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SMTCheckerKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SealedKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SecondsKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Semicolon {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SizeOfKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Slash {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SlashEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SolidityKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StaticKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StorageKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StringKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StringLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StructKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SuperKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SupportsKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SwitchKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ThisKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ThrowKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Tilde {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TransientKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TrueKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TryKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TypeDefKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TypeKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TypeOfKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UfixedKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UintKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UncheckedKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UnicodeStringLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UsingKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for VarKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for VersionSpecifier {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ViewKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for VirtualKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for WeeksKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for WeiKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for WhileKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YearsKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulBreakKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulCaseKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulCloseBrace {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulCloseParen {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulColonEqual {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulComma {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulContinueKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulDecimalLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulDefaultKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulFalseKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulForKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulFunctionKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulHexKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulHexLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulHexStringLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulIdentifier {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulIfKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulLeaveKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulLetKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulMinusGreaterThan {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulOpenBrace {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulOpenParen {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulPeriod {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulStringLiteral {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulSuperKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulSwitchKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulThisKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulTrueKeyword {
    fn text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}
