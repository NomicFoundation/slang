// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::wildcard_imports)]

use crate::structured_cst::nodes::*;

/// A trait for CST nodes that can report the start offset of their text range.
pub(super) trait TextStart {
    /// Returns the start offset of this node's text, or `None` if the node is empty.
    fn text_start(&self) -> Option<usize>;
}

impl<T: TextStart> TextStart for Option<T> {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().and_then(TextStart::text_start)
    }
}

impl TextStart for AbicoderPragmaStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.abicoder_keyword.text_start())
            .or_else(|| self.version.text_start())
    }
}

impl TextStart for AbicoderPragma {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for AdditiveExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.expression_additive_expression_operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for AdditiveExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for AddressTypeStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.address_keyword.text_start())
            .or_else(|| self.payable_keyword.text_start())
    }
}

impl TextStart for AddressType {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for AndExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for AndExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ArrayExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_bracket.text_start())
            .or_else(|| self.items.text_start())
            .or_else(|| self.close_bracket.text_start())
    }
}

impl TextStart for ArrayExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ArrayTypeNameStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.text_start())
            .or_else(|| self.open_bracket.text_start())
            .or_else(|| self.index.text_start())
            .or_else(|| self.close_bracket.text_start())
    }
}

impl TextStart for ArrayTypeName {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for AssemblyStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.assembly_keyword.text_start())
            .or_else(|| self.label.text_start())
            .or_else(|| self.flags.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for AssemblyStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for AssignmentExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.expression_assignment_expression_operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for AssignmentExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for BitwiseAndExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for BitwiseAndExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for BitwiseOrExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for BitwiseOrExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for BitwiseXorExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for BitwiseXorExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for BlockStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.text_start())
            .or_else(|| self.statements.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for Block {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for BreakStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.break_keyword.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for BreakStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for CallOptionsExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.text_start())
            .or_else(|| self.open_brace.text_start())
            .or_else(|| self.options.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for CallOptionsExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for CatchClauseStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.catch_keyword.text_start())
            .or_else(|| self.error.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for CatchClause {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for CatchClauseErrorStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.text_start())
            .or_else(|| self.parameters.text_start())
    }
}

impl TextStart for CatchClauseError {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ConditionalExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.text_start())
            .or_else(|| self.question_mark.text_start())
            .or_else(|| self.true_expression.text_start())
            .or_else(|| self.colon.text_start())
            .or_else(|| self.false_expression.text_start())
    }
}

impl TextStart for ConditionalExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ConstantDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_start())
            .or_else(|| self.constant_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.equal.text_start())
            .or_else(|| self.value.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for ConstantDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ConstructorDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.constructor_keyword.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.attributes.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for ConstructorDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ContinueStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.continue_keyword.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for ContinueStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ContractDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.abstract_keyword.text_start())
            .or_else(|| self.contract_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.specifiers.text_start())
            .or_else(|| self.open_brace.text_start())
            .or_else(|| self.members.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for ContractDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for DecimalNumberExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.literal.text_start())
            .or_else(|| self.unit.text_start())
    }
}

impl TextStart for DecimalNumberExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for DoWhileStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.do_keyword.text_start())
            .or_else(|| self.body.text_start())
            .or_else(|| self.while_keyword.text_start())
            .or_else(|| self.open_paren.text_start())
            .or_else(|| self.condition.text_start())
            .or_else(|| self.close_paren.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for DoWhileStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ElseBranchStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.else_keyword.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for ElseBranch {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for EmitStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.emit_keyword.text_start())
            .or_else(|| self.event.text_start())
            .or_else(|| self.arguments.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for EmitStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for EnumDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.enum_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.open_brace.text_start())
            .or_else(|| self.members.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for EnumDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for EqualityExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.expression_equality_expression_operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for EqualityExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ErrorDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.error_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.members.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for ErrorDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ErrorParameterStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_start())
            .or_else(|| self.name.text_start())
    }
}

impl TextStart for ErrorParameter {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ErrorParametersDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for ErrorParametersDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for EventDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.event_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.anonymous_keyword.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for EventDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for EventParameterStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_start())
            .or_else(|| self.indexed_keyword.text_start())
            .or_else(|| self.name.text_start())
    }
}

impl TextStart for EventParameter {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for EventParametersDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for EventParametersDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ExperimentalPragmaStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.experimental_keyword.text_start())
            .or_else(|| self.feature.text_start())
    }
}

impl TextStart for ExperimentalPragma {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ExponentiationExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for ExponentiationExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ExpressionStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.expression.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for ExpressionStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for FallbackFunctionDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.fallback_keyword.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.attributes.text_start())
            .or_else(|| self.returns.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for FallbackFunctionDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ForStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.for_keyword.text_start())
            .or_else(|| self.open_paren.text_start())
            .or_else(|| self.initialization.text_start())
            .or_else(|| self.condition.text_start())
            .or_else(|| self.iterator.text_start())
            .or_else(|| self.close_paren.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for ForStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for FunctionCallExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.text_start())
            .or_else(|| self.arguments.text_start())
    }
}

impl TextStart for FunctionCallExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for FunctionDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.function_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.attributes.text_start())
            .or_else(|| self.returns.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for FunctionDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for FunctionTypeStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.function_keyword.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.attributes.text_start())
            .or_else(|| self.returns.text_start())
    }
}

impl TextStart for FunctionType {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for HexNumberExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.literal.text_start())
    }
}

impl TextStart for HexNumberExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for IfStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.if_keyword.text_start())
            .or_else(|| self.open_paren.text_start())
            .or_else(|| self.condition.text_start())
            .or_else(|| self.close_paren.text_start())
            .or_else(|| self.body.text_start())
            .or_else(|| self.else_branch.text_start())
    }
}

impl TextStart for IfStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ImportAliasStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.as_keyword.text_start())
            .or_else(|| self.identifier.text_start())
    }
}

impl TextStart for ImportAlias {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ImportDeconstructionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.text_start())
            .or_else(|| self.symbols.text_start())
            .or_else(|| self.close_brace.text_start())
            .or_else(|| self.from_keyword.text_start())
            .or_else(|| self.path.text_start())
    }
}

impl TextStart for ImportDeconstruction {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ImportDeconstructionSymbolStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.text_start())
            .or_else(|| self.alias.text_start())
    }
}

impl TextStart for ImportDeconstructionSymbol {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ImportDirectiveStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.import_keyword.text_start())
            .or_else(|| self.clause.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for ImportDirective {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for IndexAccessEndStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.colon.text_start())
            .or_else(|| self.end.text_start())
    }
}

impl TextStart for IndexAccessEnd {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for IndexAccessExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.text_start())
            .or_else(|| self.open_bracket.text_start())
            .or_else(|| self.start.text_start())
            .or_else(|| self.end.text_start())
            .or_else(|| self.close_bracket.text_start())
    }
}

impl TextStart for IndexAccessExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for InequalityExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.expression_inequality_expression_operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for InequalityExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for InheritanceSpecifierStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.is_keyword.text_start())
            .or_else(|| self.types.text_start())
    }
}

impl TextStart for InheritanceSpecifier {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for InheritanceTypeStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_start())
            .or_else(|| self.arguments.text_start())
    }
}

impl TextStart for InheritanceType {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for InterfaceDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.interface_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.inheritance.text_start())
            .or_else(|| self.open_brace.text_start())
            .or_else(|| self.members.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for InterfaceDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for LibraryDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.library_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.open_brace.text_start())
            .or_else(|| self.members.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for LibraryDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for MappingKeyStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.key_type.text_start())
            .or_else(|| self.name.text_start())
    }
}

impl TextStart for MappingKey {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for MappingTypeStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.mapping_keyword.text_start())
            .or_else(|| self.open_paren.text_start())
            .or_else(|| self.key_type.text_start())
            .or_else(|| self.equal_greater_than.text_start())
            .or_else(|| self.value_type.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for MappingType {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for MappingValueStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_start())
            .or_else(|| self.name.text_start())
    }
}

impl TextStart for MappingValue {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for MemberAccessExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.text_start())
            .or_else(|| self.period.text_start())
            .or_else(|| self.member.text_start())
    }
}

impl TextStart for MemberAccessExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ModifierDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.modifier_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.attributes.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for ModifierDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ModifierInvocationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.text_start())
            .or_else(|| self.arguments.text_start())
    }
}

impl TextStart for ModifierInvocation {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for MultiTypedDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.elements.text_start())
            .or_else(|| self.close_paren.text_start())
            .or_else(|| self.value.text_start())
    }
}

impl TextStart for MultiTypedDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for MultiTypedDeclarationElementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.member.text_start())
    }
}

impl TextStart for MultiTypedDeclarationElement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for MultiplicativeExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| {
                self.expression_multiplicative_expression_operator
                    .text_start()
            })
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for MultiplicativeExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for NamedArgumentStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.text_start())
            .or_else(|| self.colon.text_start())
            .or_else(|| self.value.text_start())
    }
}

impl TextStart for NamedArgument {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for NamedArgumentGroupStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.text_start())
            .or_else(|| self.arguments.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for NamedArgumentGroup {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for NamedArgumentsDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.arguments.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for NamedArgumentsDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for NamedImportStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.asterisk.text_start())
            .or_else(|| self.alias.text_start())
            .or_else(|| self.from_keyword.text_start())
            .or_else(|| self.path.text_start())
    }
}

impl TextStart for NamedImport {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for NewExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.new_keyword.text_start())
            .or_else(|| self.type_name.text_start())
    }
}

impl TextStart for NewExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for OrExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for OrExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for OverridePathsDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.paths.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for OverridePathsDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for OverrideSpecifierStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.override_keyword.text_start())
            .or_else(|| self.overridden.text_start())
    }
}

impl TextStart for OverrideSpecifier {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ParameterStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_start())
            .or_else(|| self.storage_location.text_start())
            .or_else(|| self.name.text_start())
    }
}

impl TextStart for Parameter {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ParametersDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for ParametersDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for PathImportStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.path.text_start())
            .or_else(|| self.alias.text_start())
    }
}

impl TextStart for PathImport {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for PositionalArgumentsDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.arguments.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for PositionalArgumentsDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for PostfixExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.text_start())
            .or_else(|| self.expression_postfix_expression_operator.text_start())
    }
}

impl TextStart for PostfixExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for PragmaDirectiveStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.pragma_keyword.text_start())
            .or_else(|| self.pragma.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for PragmaDirective {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for PrefixExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.expression_prefix_expression_operator.text_start())
            .or_else(|| self.operand.text_start())
    }
}

impl TextStart for PrefixExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ReceiveFunctionDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.receive_keyword.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.attributes.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for ReceiveFunctionDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ReturnStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.return_keyword.text_start())
            .or_else(|| self.expression.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for ReturnStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ReturnsDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.returns_keyword.text_start())
            .or_else(|| self.variables.text_start())
    }
}

impl TextStart for ReturnsDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for RevertStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.revert_keyword.text_start())
            .or_else(|| self.error.text_start())
            .or_else(|| self.arguments.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for RevertStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for ShiftExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.text_start())
            .or_else(|| self.expression_shift_expression_operator.text_start())
            .or_else(|| self.right_operand.text_start())
    }
}

impl TextStart for ShiftExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for SingleTypedDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.declaration.text_start())
            .or_else(|| self.value.text_start())
    }
}

impl TextStart for SingleTypedDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for SourceUnitStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.members.text_start())
    }
}

impl TextStart for SourceUnit {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for StateVariableDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_start())
            .or_else(|| self.attributes.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.value.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for StateVariableDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for StateVariableDefinitionValueStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.equal.text_start())
            .or_else(|| self.value.text_start())
    }
}

impl TextStart for StateVariableDefinitionValue {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for StorageLayoutSpecifierStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.layout_keyword.text_start())
            .or_else(|| self.at_keyword.text_start())
            .or_else(|| self.expression.text_start())
    }
}

impl TextStart for StorageLayoutSpecifier {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for StructDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.struct_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.open_brace.text_start())
            .or_else(|| self.members.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for StructDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for StructMemberStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for StructMember {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for TryStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.try_keyword.text_start())
            .or_else(|| self.expression.text_start())
            .or_else(|| self.returns.text_start())
            .or_else(|| self.body.text_start())
            .or_else(|| self.catch_clauses.text_start())
    }
}

impl TextStart for TryStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for TupleExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.items.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for TupleExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for TupleValueStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.expression.text_start())
    }
}

impl TextStart for TupleValue {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for TypeExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_keyword.text_start())
            .or_else(|| self.open_paren.text_start())
            .or_else(|| self.type_name.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for TypeExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for UncheckedBlockStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.unchecked_keyword.text_start())
            .or_else(|| self.block.text_start())
    }
}

impl TextStart for UncheckedBlock {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for UserDefinedValueTypeDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.is_keyword.text_start())
            .or_else(|| self.value_type.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for UserDefinedValueTypeDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for UsingAliasStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.as_keyword.text_start())
            .or_else(|| self.operator.text_start())
    }
}

impl TextStart for UsingAlias {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for UsingDeconstructionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.text_start())
            .or_else(|| self.symbols.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for UsingDeconstruction {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for UsingDeconstructionSymbolStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.text_start())
            .or_else(|| self.alias.text_start())
    }
}

impl TextStart for UsingDeconstructionSymbol {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for UsingDirectiveStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.using_keyword.text_start())
            .or_else(|| self.clause.text_start())
            .or_else(|| self.for_keyword.text_start())
            .or_else(|| self.target.text_start())
            .or_else(|| self.global_keyword.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for UsingDirective {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for VariableDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.text_start())
            .or_else(|| self.storage_location.text_start())
            .or_else(|| self.name.text_start())
    }
}

impl TextStart for VariableDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for VariableDeclarationStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.target.text_start())
            .or_else(|| self.semicolon.text_start())
    }
}

impl TextStart for VariableDeclarationStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for VariableDeclarationValueStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.equal.text_start())
            .or_else(|| self.expression.text_start())
    }
}

impl TextStart for VariableDeclarationValue {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for VersionPragmaStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.solidity_keyword.text_start())
            .or_else(|| self.sets.text_start())
    }
}

impl TextStart for VersionPragma {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for VersionRangeStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.start.text_start())
            .or_else(|| self.minus.text_start())
            .or_else(|| self.end.text_start())
    }
}

impl TextStart for VersionRange {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for VersionTermStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.operator.text_start())
            .or_else(|| self.literal.text_start())
    }
}

impl TextStart for VersionTerm {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for WhileStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.while_keyword.text_start())
            .or_else(|| self.open_paren.text_start())
            .or_else(|| self.condition.text_start())
            .or_else(|| self.close_paren.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for WhileStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulBlockStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.text_start())
            .or_else(|| self.statements.text_start())
            .or_else(|| self.close_brace.text_start())
    }
}

impl TextStart for YulBlock {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulBreakStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.break_keyword.text_start())
    }
}

impl TextStart for YulBreakStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulContinueStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.continue_keyword.text_start())
    }
}

impl TextStart for YulContinueStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulDefaultCaseStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.default_keyword.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for YulDefaultCase {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulFlagsDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.flags.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for YulFlagsDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulForStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.for_keyword.text_start())
            .or_else(|| self.initialization.text_start())
            .or_else(|| self.condition.text_start())
            .or_else(|| self.iterator.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for YulForStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulFunctionCallExpressionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.text_start())
            .or_else(|| self.open_paren.text_start())
            .or_else(|| self.arguments.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for YulFunctionCallExpression {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulFunctionDefinitionStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.function_keyword.text_start())
            .or_else(|| self.name.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.returns.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for YulFunctionDefinition {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulIfStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.if_keyword.text_start())
            .or_else(|| self.condition.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for YulIfStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulLeaveStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.leave_keyword.text_start())
    }
}

impl TextStart for YulLeaveStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulParametersDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.text_start())
            .or_else(|| self.parameters.text_start())
            .or_else(|| self.close_paren.text_start())
    }
}

impl TextStart for YulParametersDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulReturnsDeclarationStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.minus_greater_than.text_start())
            .or_else(|| self.variables.text_start())
    }
}

impl TextStart for YulReturnsDeclaration {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulSwitchStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.switch_keyword.text_start())
            .or_else(|| self.expression.text_start())
            .or_else(|| self.cases.text_start())
    }
}

impl TextStart for YulSwitchStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulValueCaseStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.case_keyword.text_start())
            .or_else(|| self.value.text_start())
            .or_else(|| self.body.text_start())
    }
}

impl TextStart for YulValueCase {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulVariableAssignmentStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.variables.text_start())
            .or_else(|| self.assignment.text_start())
            .or_else(|| self.expression.text_start())
    }
}

impl TextStart for YulVariableAssignmentStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulVariableDeclarationStatementStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.let_keyword.text_start())
            .or_else(|| self.variables.text_start())
            .or_else(|| self.value.text_start())
    }
}

impl TextStart for YulVariableDeclarationStatement {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for YulVariableDeclarationValueStruct {
    fn text_start(&self) -> Option<usize> {
        None.or_else(|| self.assignment.text_start())
            .or_else(|| self.expression.text_start())
    }
}

impl TextStart for YulVariableDeclarationValue {
    fn text_start(&self) -> Option<usize> {
        self.as_ref().text_start()
    }
}

impl TextStart for AbicoderVersion {
    fn text_start(&self) -> Option<usize> {
        match self {
            AbicoderVersion::AbicoderV1Keyword(child) => child.text_start(),
            AbicoderVersion::AbicoderV2Keyword(child) => child.text_start(),
        }
    }
}

impl TextStart for ArgumentsDeclaration {
    fn text_start(&self) -> Option<usize> {
        match self {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(child) => child.text_start(),
            ArgumentsDeclaration::NamedArgumentsDeclaration(child) => child.text_start(),
        }
    }
}

impl TextStart for ConstructorAttribute {
    fn text_start(&self) -> Option<usize> {
        match self {
            ConstructorAttribute::ModifierInvocation(child) => child.text_start(),
            ConstructorAttribute::InternalKeyword(child) => child.text_start(),
            ConstructorAttribute::PayableKeyword(child) => child.text_start(),
            ConstructorAttribute::PublicKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for ContractMember {
    fn text_start(&self) -> Option<usize> {
        match self {
            ContractMember::UsingDirective(child) => child.text_start(),
            ContractMember::FunctionDefinition(child) => child.text_start(),
            ContractMember::ConstructorDefinition(child) => child.text_start(),
            ContractMember::ReceiveFunctionDefinition(child) => child.text_start(),
            ContractMember::FallbackFunctionDefinition(child) => child.text_start(),
            ContractMember::ModifierDefinition(child) => child.text_start(),
            ContractMember::StructDefinition(child) => child.text_start(),
            ContractMember::EnumDefinition(child) => child.text_start(),
            ContractMember::EventDefinition(child) => child.text_start(),
            ContractMember::ErrorDefinition(child) => child.text_start(),
            ContractMember::UserDefinedValueTypeDefinition(child) => child.text_start(),
            ContractMember::StateVariableDefinition(child) => child.text_start(),
        }
    }
}

impl TextStart for ContractSpecifier {
    fn text_start(&self) -> Option<usize> {
        match self {
            ContractSpecifier::InheritanceSpecifier(child) => child.text_start(),
            ContractSpecifier::StorageLayoutSpecifier(child) => child.text_start(),
        }
    }
}

impl TextStart for ElementaryType {
    fn text_start(&self) -> Option<usize> {
        match self {
            ElementaryType::BoolKeyword(child) => child.text_start(),
            ElementaryType::StringKeyword(child) => child.text_start(),
            ElementaryType::AddressType(child) => child.text_start(),
            ElementaryType::BytesKeyword(child) => child.text_start(),
            ElementaryType::IntKeyword(child) => child.text_start(),
            ElementaryType::UintKeyword(child) => child.text_start(),
            ElementaryType::FixedKeyword(child) => child.text_start(),
            ElementaryType::UfixedKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for ExperimentalFeature {
    fn text_start(&self) -> Option<usize> {
        match self {
            ExperimentalFeature::ABIEncoderV2Keyword(child) => child.text_start(),
            ExperimentalFeature::SMTCheckerKeyword(child) => child.text_start(),
            ExperimentalFeature::PragmaStringLiteral(child) => child.text_start(),
        }
    }
}

impl TextStart for Expression {
    fn text_start(&self) -> Option<usize> {
        match self {
            Expression::AssignmentExpression(child) => child.text_start(),
            Expression::ConditionalExpression(child) => child.text_start(),
            Expression::OrExpression(child) => child.text_start(),
            Expression::AndExpression(child) => child.text_start(),
            Expression::EqualityExpression(child) => child.text_start(),
            Expression::InequalityExpression(child) => child.text_start(),
            Expression::BitwiseOrExpression(child) => child.text_start(),
            Expression::BitwiseXorExpression(child) => child.text_start(),
            Expression::BitwiseAndExpression(child) => child.text_start(),
            Expression::ShiftExpression(child) => child.text_start(),
            Expression::AdditiveExpression(child) => child.text_start(),
            Expression::MultiplicativeExpression(child) => child.text_start(),
            Expression::ExponentiationExpression(child) => child.text_start(),
            Expression::PostfixExpression(child) => child.text_start(),
            Expression::PrefixExpression(child) => child.text_start(),
            Expression::FunctionCallExpression(child) => child.text_start(),
            Expression::CallOptionsExpression(child) => child.text_start(),
            Expression::MemberAccessExpression(child) => child.text_start(),
            Expression::IndexAccessExpression(child) => child.text_start(),
            Expression::NewExpression(child) => child.text_start(),
            Expression::TupleExpression(child) => child.text_start(),
            Expression::TypeExpression(child) => child.text_start(),
            Expression::ArrayExpression(child) => child.text_start(),
            Expression::HexNumberExpression(child) => child.text_start(),
            Expression::DecimalNumberExpression(child) => child.text_start(),
            Expression::StringExpression(child) => child.text_start(),
            Expression::ElementaryType(child) => child.text_start(),
            Expression::PayableKeyword(child) => child.text_start(),
            Expression::ThisKeyword(child) => child.text_start(),
            Expression::SuperKeyword(child) => child.text_start(),
            Expression::TrueKeyword(child) => child.text_start(),
            Expression::FalseKeyword(child) => child.text_start(),
            Expression::Identifier(child) => child.text_start(),
        }
    }
}

impl TextStart for Expression_AdditiveExpression_Operator {
    fn text_start(&self) -> Option<usize> {
        match self {
            Expression_AdditiveExpression_Operator::Minus(child) => child.text_start(),
            Expression_AdditiveExpression_Operator::Plus(child) => child.text_start(),
        }
    }
}

impl TextStart for Expression_AssignmentExpression_Operator {
    fn text_start(&self) -> Option<usize> {
        match self {
            Expression_AssignmentExpression_Operator::AmpersandEqual(child) => child.text_start(),
            Expression_AssignmentExpression_Operator::AsteriskEqual(child) => child.text_start(),
            Expression_AssignmentExpression_Operator::BarEqual(child) => child.text_start(),
            Expression_AssignmentExpression_Operator::CaretEqual(child) => child.text_start(),
            Expression_AssignmentExpression_Operator::Equal(child) => child.text_start(),
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(child) => {
                child.text_start()
            }
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(
                child,
            ) => child.text_start(),
            Expression_AssignmentExpression_Operator::LessThanLessThanEqual(child) => {
                child.text_start()
            }
            Expression_AssignmentExpression_Operator::MinusEqual(child) => child.text_start(),
            Expression_AssignmentExpression_Operator::PercentEqual(child) => child.text_start(),
            Expression_AssignmentExpression_Operator::PlusEqual(child) => child.text_start(),
            Expression_AssignmentExpression_Operator::SlashEqual(child) => child.text_start(),
        }
    }
}

impl TextStart for Expression_EqualityExpression_Operator {
    fn text_start(&self) -> Option<usize> {
        match self {
            Expression_EqualityExpression_Operator::BangEqual(child) => child.text_start(),
            Expression_EqualityExpression_Operator::EqualEqual(child) => child.text_start(),
        }
    }
}

impl TextStart for Expression_InequalityExpression_Operator {
    fn text_start(&self) -> Option<usize> {
        match self {
            Expression_InequalityExpression_Operator::GreaterThan(child) => child.text_start(),
            Expression_InequalityExpression_Operator::GreaterThanEqual(child) => child.text_start(),
            Expression_InequalityExpression_Operator::LessThan(child) => child.text_start(),
            Expression_InequalityExpression_Operator::LessThanEqual(child) => child.text_start(),
        }
    }
}

impl TextStart for Expression_MultiplicativeExpression_Operator {
    fn text_start(&self) -> Option<usize> {
        match self {
            Expression_MultiplicativeExpression_Operator::Asterisk(child) => child.text_start(),
            Expression_MultiplicativeExpression_Operator::Percent(child) => child.text_start(),
            Expression_MultiplicativeExpression_Operator::Slash(child) => child.text_start(),
        }
    }
}

impl TextStart for Expression_PostfixExpression_Operator {
    fn text_start(&self) -> Option<usize> {
        match self {
            Expression_PostfixExpression_Operator::MinusMinus(child) => child.text_start(),
            Expression_PostfixExpression_Operator::PlusPlus(child) => child.text_start(),
        }
    }
}

impl TextStart for Expression_PrefixExpression_Operator {
    fn text_start(&self) -> Option<usize> {
        match self {
            Expression_PrefixExpression_Operator::Bang(child) => child.text_start(),
            Expression_PrefixExpression_Operator::DeleteKeyword(child) => child.text_start(),
            Expression_PrefixExpression_Operator::Minus(child) => child.text_start(),
            Expression_PrefixExpression_Operator::MinusMinus(child) => child.text_start(),
            Expression_PrefixExpression_Operator::PlusPlus(child) => child.text_start(),
            Expression_PrefixExpression_Operator::Tilde(child) => child.text_start(),
        }
    }
}

impl TextStart for Expression_ShiftExpression_Operator {
    fn text_start(&self) -> Option<usize> {
        match self {
            Expression_ShiftExpression_Operator::GreaterThanGreaterThan(child) => {
                child.text_start()
            }
            Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(child) => {
                child.text_start()
            }
            Expression_ShiftExpression_Operator::LessThanLessThan(child) => child.text_start(),
        }
    }
}

impl TextStart for FallbackFunctionAttribute {
    fn text_start(&self) -> Option<usize> {
        match self {
            FallbackFunctionAttribute::ModifierInvocation(child) => child.text_start(),
            FallbackFunctionAttribute::OverrideSpecifier(child) => child.text_start(),
            FallbackFunctionAttribute::ExternalKeyword(child) => child.text_start(),
            FallbackFunctionAttribute::PayableKeyword(child) => child.text_start(),
            FallbackFunctionAttribute::PureKeyword(child) => child.text_start(),
            FallbackFunctionAttribute::ViewKeyword(child) => child.text_start(),
            FallbackFunctionAttribute::VirtualKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for ForStatementCondition {
    fn text_start(&self) -> Option<usize> {
        match self {
            ForStatementCondition::ExpressionStatement(child) => child.text_start(),
            ForStatementCondition::Semicolon(child) => child.text_start(),
        }
    }
}

impl TextStart for ForStatementInitialization {
    fn text_start(&self) -> Option<usize> {
        match self {
            ForStatementInitialization::VariableDeclarationStatement(child) => child.text_start(),
            ForStatementInitialization::ExpressionStatement(child) => child.text_start(),
            ForStatementInitialization::Semicolon(child) => child.text_start(),
        }
    }
}

impl TextStart for FunctionAttribute {
    fn text_start(&self) -> Option<usize> {
        match self {
            FunctionAttribute::ModifierInvocation(child) => child.text_start(),
            FunctionAttribute::OverrideSpecifier(child) => child.text_start(),
            FunctionAttribute::ExternalKeyword(child) => child.text_start(),
            FunctionAttribute::InternalKeyword(child) => child.text_start(),
            FunctionAttribute::PayableKeyword(child) => child.text_start(),
            FunctionAttribute::PrivateKeyword(child) => child.text_start(),
            FunctionAttribute::PublicKeyword(child) => child.text_start(),
            FunctionAttribute::PureKeyword(child) => child.text_start(),
            FunctionAttribute::ViewKeyword(child) => child.text_start(),
            FunctionAttribute::VirtualKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for FunctionBody {
    fn text_start(&self) -> Option<usize> {
        match self {
            FunctionBody::Block(child) => child.text_start(),
            FunctionBody::Semicolon(child) => child.text_start(),
        }
    }
}

impl TextStart for FunctionName {
    fn text_start(&self) -> Option<usize> {
        match self {
            FunctionName::Identifier(child) => child.text_start(),
            FunctionName::FallbackKeyword(child) => child.text_start(),
            FunctionName::ReceiveKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for FunctionTypeAttribute {
    fn text_start(&self) -> Option<usize> {
        match self {
            FunctionTypeAttribute::InternalKeyword(child) => child.text_start(),
            FunctionTypeAttribute::ExternalKeyword(child) => child.text_start(),
            FunctionTypeAttribute::PrivateKeyword(child) => child.text_start(),
            FunctionTypeAttribute::PublicKeyword(child) => child.text_start(),
            FunctionTypeAttribute::PureKeyword(child) => child.text_start(),
            FunctionTypeAttribute::ViewKeyword(child) => child.text_start(),
            FunctionTypeAttribute::PayableKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for IdentifierPathElement {
    fn text_start(&self) -> Option<usize> {
        match self {
            IdentifierPathElement::Identifier(child) => child.text_start(),
            IdentifierPathElement::AddressKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for ImportClause {
    fn text_start(&self) -> Option<usize> {
        match self {
            ImportClause::PathImport(child) => child.text_start(),
            ImportClause::NamedImport(child) => child.text_start(),
            ImportClause::ImportDeconstruction(child) => child.text_start(),
        }
    }
}

impl TextStart for MappingKeyType {
    fn text_start(&self) -> Option<usize> {
        match self {
            MappingKeyType::ElementaryType(child) => child.text_start(),
            MappingKeyType::IdentifierPath(child) => child.text_start(),
        }
    }
}

impl TextStart for ModifierAttribute {
    fn text_start(&self) -> Option<usize> {
        match self {
            ModifierAttribute::OverrideSpecifier(child) => child.text_start(),
            ModifierAttribute::VirtualKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for NumberUnit {
    fn text_start(&self) -> Option<usize> {
        match self {
            NumberUnit::WeiKeyword(child) => child.text_start(),
            NumberUnit::GweiKeyword(child) => child.text_start(),
            NumberUnit::EtherKeyword(child) => child.text_start(),
            NumberUnit::SecondsKeyword(child) => child.text_start(),
            NumberUnit::MinutesKeyword(child) => child.text_start(),
            NumberUnit::HoursKeyword(child) => child.text_start(),
            NumberUnit::DaysKeyword(child) => child.text_start(),
            NumberUnit::WeeksKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for Pragma {
    fn text_start(&self) -> Option<usize> {
        match self {
            Pragma::VersionPragma(child) => child.text_start(),
            Pragma::AbicoderPragma(child) => child.text_start(),
            Pragma::ExperimentalPragma(child) => child.text_start(),
        }
    }
}

impl TextStart for ReceiveFunctionAttribute {
    fn text_start(&self) -> Option<usize> {
        match self {
            ReceiveFunctionAttribute::ModifierInvocation(child) => child.text_start(),
            ReceiveFunctionAttribute::OverrideSpecifier(child) => child.text_start(),
            ReceiveFunctionAttribute::ExternalKeyword(child) => child.text_start(),
            ReceiveFunctionAttribute::PayableKeyword(child) => child.text_start(),
            ReceiveFunctionAttribute::VirtualKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for SourceUnitMember {
    fn text_start(&self) -> Option<usize> {
        match self {
            SourceUnitMember::PragmaDirective(child) => child.text_start(),
            SourceUnitMember::ImportDirective(child) => child.text_start(),
            SourceUnitMember::ContractDefinition(child) => child.text_start(),
            SourceUnitMember::InterfaceDefinition(child) => child.text_start(),
            SourceUnitMember::LibraryDefinition(child) => child.text_start(),
            SourceUnitMember::StructDefinition(child) => child.text_start(),
            SourceUnitMember::EnumDefinition(child) => child.text_start(),
            SourceUnitMember::FunctionDefinition(child) => child.text_start(),
            SourceUnitMember::ErrorDefinition(child) => child.text_start(),
            SourceUnitMember::UserDefinedValueTypeDefinition(child) => child.text_start(),
            SourceUnitMember::UsingDirective(child) => child.text_start(),
            SourceUnitMember::EventDefinition(child) => child.text_start(),
            SourceUnitMember::ConstantDefinition(child) => child.text_start(),
        }
    }
}

impl TextStart for StateVariableAttribute {
    fn text_start(&self) -> Option<usize> {
        match self {
            StateVariableAttribute::OverrideSpecifier(child) => child.text_start(),
            StateVariableAttribute::ConstantKeyword(child) => child.text_start(),
            StateVariableAttribute::InternalKeyword(child) => child.text_start(),
            StateVariableAttribute::PrivateKeyword(child) => child.text_start(),
            StateVariableAttribute::PublicKeyword(child) => child.text_start(),
            StateVariableAttribute::ImmutableKeyword(child) => child.text_start(),
            StateVariableAttribute::TransientKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for Statement {
    fn text_start(&self) -> Option<usize> {
        match self {
            Statement::IfStatement(child) => child.text_start(),
            Statement::ForStatement(child) => child.text_start(),
            Statement::WhileStatement(child) => child.text_start(),
            Statement::DoWhileStatement(child) => child.text_start(),
            Statement::ContinueStatement(child) => child.text_start(),
            Statement::BreakStatement(child) => child.text_start(),
            Statement::ReturnStatement(child) => child.text_start(),
            Statement::EmitStatement(child) => child.text_start(),
            Statement::TryStatement(child) => child.text_start(),
            Statement::RevertStatement(child) => child.text_start(),
            Statement::AssemblyStatement(child) => child.text_start(),
            Statement::Block(child) => child.text_start(),
            Statement::UncheckedBlock(child) => child.text_start(),
            Statement::VariableDeclarationStatement(child) => child.text_start(),
            Statement::ExpressionStatement(child) => child.text_start(),
        }
    }
}

impl TextStart for StorageLocation {
    fn text_start(&self) -> Option<usize> {
        match self {
            StorageLocation::MemoryKeyword(child) => child.text_start(),
            StorageLocation::StorageKeyword(child) => child.text_start(),
            StorageLocation::CallDataKeyword(child) => child.text_start(),
        }
    }
}

impl TextStart for StringExpression {
    fn text_start(&self) -> Option<usize> {
        match self {
            StringExpression::StringLiterals(child) => child.text_start(),
            StringExpression::HexStringLiterals(child) => child.text_start(),
            StringExpression::UnicodeStringLiterals(child) => child.text_start(),
        }
    }
}

impl TextStart for TypeName {
    fn text_start(&self) -> Option<usize> {
        match self {
            TypeName::ArrayTypeName(child) => child.text_start(),
            TypeName::FunctionType(child) => child.text_start(),
            TypeName::MappingType(child) => child.text_start(),
            TypeName::ElementaryType(child) => child.text_start(),
            TypeName::IdentifierPath(child) => child.text_start(),
        }
    }
}

impl TextStart for UsingClause {
    fn text_start(&self) -> Option<usize> {
        match self {
            UsingClause::IdentifierPath(child) => child.text_start(),
            UsingClause::UsingDeconstruction(child) => child.text_start(),
        }
    }
}

impl TextStart for UsingOperator {
    fn text_start(&self) -> Option<usize> {
        match self {
            UsingOperator::Ampersand(child) => child.text_start(),
            UsingOperator::Asterisk(child) => child.text_start(),
            UsingOperator::BangEqual(child) => child.text_start(),
            UsingOperator::Bar(child) => child.text_start(),
            UsingOperator::Caret(child) => child.text_start(),
            UsingOperator::EqualEqual(child) => child.text_start(),
            UsingOperator::GreaterThan(child) => child.text_start(),
            UsingOperator::GreaterThanEqual(child) => child.text_start(),
            UsingOperator::LessThan(child) => child.text_start(),
            UsingOperator::LessThanEqual(child) => child.text_start(),
            UsingOperator::Minus(child) => child.text_start(),
            UsingOperator::Percent(child) => child.text_start(),
            UsingOperator::Plus(child) => child.text_start(),
            UsingOperator::Slash(child) => child.text_start(),
            UsingOperator::Tilde(child) => child.text_start(),
        }
    }
}

impl TextStart for UsingTarget {
    fn text_start(&self) -> Option<usize> {
        match self {
            UsingTarget::TypeName(child) => child.text_start(),
            UsingTarget::Asterisk(child) => child.text_start(),
        }
    }
}

impl TextStart for VariableDeclarationTarget {
    fn text_start(&self) -> Option<usize> {
        match self {
            VariableDeclarationTarget::SingleTypedDeclaration(child) => child.text_start(),
            VariableDeclarationTarget::MultiTypedDeclaration(child) => child.text_start(),
        }
    }
}

impl TextStart for VersionExpression {
    fn text_start(&self) -> Option<usize> {
        match self {
            VersionExpression::VersionRange(child) => child.text_start(),
            VersionExpression::VersionTerm(child) => child.text_start(),
        }
    }
}

impl TextStart for VersionLiteral {
    fn text_start(&self) -> Option<usize> {
        match self {
            VersionLiteral::SimpleVersionLiteral(child) => child.text_start(),
            VersionLiteral::PragmaStringLiteral(child) => child.text_start(),
        }
    }
}

impl TextStart for VersionOperator {
    fn text_start(&self) -> Option<usize> {
        match self {
            VersionOperator::PragmaCaret(child) => child.text_start(),
            VersionOperator::PragmaTilde(child) => child.text_start(),
            VersionOperator::PragmaEqual(child) => child.text_start(),
            VersionOperator::PragmaLessThan(child) => child.text_start(),
            VersionOperator::PragmaGreaterThan(child) => child.text_start(),
            VersionOperator::PragmaLessThanEqual(child) => child.text_start(),
            VersionOperator::PragmaGreaterThanEqual(child) => child.text_start(),
        }
    }
}

impl TextStart for YulExpression {
    fn text_start(&self) -> Option<usize> {
        match self {
            YulExpression::YulFunctionCallExpression(child) => child.text_start(),
            YulExpression::YulLiteral(child) => child.text_start(),
            YulExpression::YulPath(child) => child.text_start(),
        }
    }
}

impl TextStart for YulLiteral {
    fn text_start(&self) -> Option<usize> {
        match self {
            YulLiteral::YulTrueKeyword(child) => child.text_start(),
            YulLiteral::YulFalseKeyword(child) => child.text_start(),
            YulLiteral::YulDecimalLiteral(child) => child.text_start(),
            YulLiteral::YulHexLiteral(child) => child.text_start(),
            YulLiteral::YulHexStringLiteral(child) => child.text_start(),
            YulLiteral::YulStringLiteral(child) => child.text_start(),
        }
    }
}

impl TextStart for YulStatement {
    fn text_start(&self) -> Option<usize> {
        match self {
            YulStatement::YulBlock(child) => child.text_start(),
            YulStatement::YulFunctionDefinition(child) => child.text_start(),
            YulStatement::YulIfStatement(child) => child.text_start(),
            YulStatement::YulForStatement(child) => child.text_start(),
            YulStatement::YulSwitchStatement(child) => child.text_start(),
            YulStatement::YulLeaveStatement(child) => child.text_start(),
            YulStatement::YulBreakStatement(child) => child.text_start(),
            YulStatement::YulContinueStatement(child) => child.text_start(),
            YulStatement::YulVariableAssignmentStatement(child) => child.text_start(),
            YulStatement::YulVariableDeclarationStatement(child) => child.text_start(),
            YulStatement::YulExpression(child) => child.text_start(),
        }
    }
}

impl TextStart for YulSwitchCase {
    fn text_start(&self) -> Option<usize> {
        match self {
            YulSwitchCase::YulDefaultCase(child) => child.text_start(),
            YulSwitchCase::YulValueCase(child) => child.text_start(),
        }
    }
}

impl TextStart for ArrayValues {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for CallOptions {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for CatchClauses {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for ConstructorAttributes {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for ContractMembers {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for ContractSpecifiers {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for EnumMembers {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for ErrorParameters {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for EventParameters {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for FallbackFunctionAttributes {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for FunctionAttributes {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for FunctionTypeAttributes {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for HexStringLiterals {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for IdentifierPath {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for ImportDeconstructionSymbols {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for InheritanceTypes {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for InterfaceMembers {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for LibraryMembers {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for ModifierAttributes {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for MultiTypedDeclarationElements {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for NamedArguments {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for OverridePaths {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for Parameters {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for PositionalArguments {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for ReceiveFunctionAttributes {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for SimpleVersionLiteral {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for SourceUnitMembers {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for StateVariableAttributes {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for Statements {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for StringLiterals {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for StructMembers {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for TupleValues {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for UnicodeStringLiterals {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for UsingDeconstructionSymbols {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for VersionExpressionSet {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for VersionExpressionSets {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for YulArguments {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for YulFlags {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for YulParameters {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for YulPath {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for YulPaths {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for YulStatements {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for YulSwitchCases {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for YulVariableNames {
    fn text_start(&self) -> Option<usize> {
        self.elements.iter().find_map(TextStart::text_start)
    }
}

impl TextStart for ABIEncoderV2Keyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AbicoderKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AbicoderV1Keyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AbicoderV2Keyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AbstractKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AddressKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AfterKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AliasKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Ampersand {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AmpersandAmpersand {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AmpersandEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AnonymousKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ApplyKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AsKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AssemblyKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Asterisk {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AsteriskAsterisk {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AsteriskEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AtKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AutoKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Bang {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BangEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Bar {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BarBar {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BarEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BoolKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BreakKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ByteKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BytesKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CallDataKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Caret {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CaretEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CaseKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CatchKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CloseBrace {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CloseBracket {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CloseParen {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Colon {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Comma {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ConstantKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ConstructorKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ContinueKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ContractKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CopyOfKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DaysKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DecimalLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DefaultKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DefineKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DeleteKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DoKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ElseKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EmitKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EnumKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Equal {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EqualEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EqualGreaterThan {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ErrorKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EtherKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EventKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ExperimentalKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ExternalKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FallbackKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FalseKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FinalKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FixedKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ForKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FromKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FunctionKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GlobalKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThan {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanGreaterThan {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanGreaterThanEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanGreaterThanGreaterThan {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanGreaterThanGreaterThanEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GweiKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for HexKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for HexLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for HexStringLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for HoursKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Identifier {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for IfKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ImmutableKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ImplementsKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ImportKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for InKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for IndexedKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for InlineKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for IntKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for InterfaceKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for InternalKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for IsKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LayoutKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LessThan {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LessThanEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LessThanLessThan {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LessThanLessThanEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LetKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LibraryKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MacroKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MappingKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MatchKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MemoryKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Minus {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MinusEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MinusMinus {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MinutesKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ModifierKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MutableKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for NewKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for NullKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OfKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OpenBrace {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OpenBracket {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OpenParen {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OverrideKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PartialKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PayableKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Percent {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PercentEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Period {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Plus {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PlusEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PlusPlus {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaBarBar {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaCaret {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaGreaterThan {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaGreaterThanEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaLessThan {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaLessThanEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaMinus {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaPeriod {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaSemicolon {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaStringLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaTilde {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PrivateKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PromiseKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PublicKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PureKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for QuestionMark {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ReceiveKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ReferenceKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for RelocatableKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ReturnKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ReturnsKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for RevertKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SMTCheckerKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SealedKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SecondsKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Semicolon {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SizeOfKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Slash {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SlashEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SolidityKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StaticKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StorageKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StringKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StringLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StructKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SuperKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SupportsKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SwitchKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ThisKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ThrowKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Tilde {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TransientKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TrueKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TryKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TypeDefKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TypeKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TypeOfKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UfixedKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UintKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UncheckedKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UnicodeStringLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UsingKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for VarKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for VersionSpecifier {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ViewKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for VirtualKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for WeeksKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for WeiKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for WhileKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YearsKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulBreakKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulCaseKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulCloseBrace {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulCloseParen {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulColonEqual {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulComma {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulContinueKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulDecimalLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulDefaultKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulFalseKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulForKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulFunctionKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulHexKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulHexLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulHexStringLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulIdentifier {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulIfKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulLeaveKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulLetKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulMinusGreaterThan {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulOpenBrace {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulOpenParen {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulPeriod {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulStringLiteral {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulSuperKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulSwitchKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulThisKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulTrueKeyword {
    fn text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}
