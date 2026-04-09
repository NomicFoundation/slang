// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::wildcard_imports)]

use std::ops::Range;

use super::text_end::TextEnd;
use super::text_start::TextStart;
use crate::structured_cst::nodes::*;

/// A trait for CST nodes that can report their text range in the source.
/// Note: this value is calculated lazily, and not stored on the node itself.
pub trait TextRange {
    /// Returns the text range of this node, or `None` if the node is empty.
    /// Note: this value is calculated lazily, and not stored on the node itself.
    fn text_range(&self) -> Option<Range<usize>>;
}

impl<T: TextRange> TextRange for Option<T> {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().and_then(TextRange::text_range)
    }
}

impl TextRange for AbicoderPragmaStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AbicoderPragma {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for AdditiveExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AdditiveExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for AddressTypeStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AddressType {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for AndExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AndExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ArrayExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ArrayExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ArrayTypeNameStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ArrayTypeName {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for AssemblyStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AssemblyStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for AssignmentExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AssignmentExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for BitwiseAndExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BitwiseAndExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for BitwiseOrExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BitwiseOrExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for BitwiseXorExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BitwiseXorExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for BlockStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Block {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for BreakStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BreakStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for CallOptionsExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CallOptionsExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for CatchClauseStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CatchClause {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for CatchClauseErrorStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CatchClauseError {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ConditionalExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ConditionalExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ConstantDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ConstantDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ConstructorDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ConstructorDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ContinueStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ContinueStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ContractDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ContractDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for DecimalNumberExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for DecimalNumberExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for DoWhileStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for DoWhileStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ElseBranchStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ElseBranch {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for EmitStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EmitStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for EnumDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EnumDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for EqualityExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EqualityExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ErrorDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ErrorDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ErrorParameterStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ErrorParameter {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ErrorParametersDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ErrorParametersDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for EventDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EventDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for EventParameterStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EventParameter {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for EventParametersDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EventParametersDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ExperimentalPragmaStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ExperimentalPragma {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ExponentiationExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ExponentiationExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ExpressionStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ExpressionStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for FallbackFunctionDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FallbackFunctionDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ForStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ForStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for FunctionCallExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionCallExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for FunctionDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for FunctionTypeStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionType {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for HexNumberExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for HexNumberExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for IfStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for IfStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ImportAliasStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ImportAlias {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ImportDeconstructionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ImportDeconstruction {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ImportDeconstructionSymbolStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ImportDeconstructionSymbol {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ImportDirectiveStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ImportDirective {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for IndexAccessEndStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for IndexAccessEnd {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for IndexAccessExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for IndexAccessExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for InequalityExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InequalityExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for InheritanceSpecifierStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InheritanceSpecifier {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for InheritanceTypeStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InheritanceType {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for InterfaceDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InterfaceDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for LibraryDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for LibraryDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for MappingKeyStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MappingKey {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for MappingTypeStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MappingType {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for MappingValueStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MappingValue {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for MemberAccessExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MemberAccessExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ModifierDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ModifierDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ModifierInvocationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ModifierInvocation {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for MultiTypedDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MultiTypedDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for MultiTypedDeclarationElementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MultiTypedDeclarationElement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for MultiplicativeExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MultiplicativeExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for NamedArgumentStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for NamedArgument {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for NamedArgumentGroupStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for NamedArgumentGroup {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for NamedArgumentsDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for NamedArgumentsDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for NamedImportStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for NamedImport {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for NewExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for NewExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for OrExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for OrExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for OverridePathsDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for OverridePathsDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for OverrideSpecifierStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for OverrideSpecifier {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ParameterStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Parameter {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ParametersDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ParametersDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for PathImportStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PathImport {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for PositionalArgumentsDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PositionalArgumentsDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for PostfixExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PostfixExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for PragmaDirectiveStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaDirective {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for PrefixExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PrefixExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ReceiveFunctionDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ReceiveFunctionDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ReturnStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ReturnStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ReturnsDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ReturnsDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for RevertStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for RevertStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for ShiftExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ShiftExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for SingleTypedDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SingleTypedDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for SourceUnitStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SourceUnit {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for StateVariableDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StateVariableDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for StateVariableDefinitionValueStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StateVariableDefinitionValue {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for StorageLayoutSpecifierStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StorageLayoutSpecifier {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for StructDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StructDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for StructMemberStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StructMember {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for TryStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TryStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for TupleExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TupleExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for TupleValueStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TupleValue {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for TypeExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TypeExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for UncheckedBlockStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UncheckedBlock {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for UserDefinedValueTypeDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UserDefinedValueTypeDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for UsingAliasStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UsingAlias {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for UsingDeconstructionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UsingDeconstruction {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for UsingDeconstructionSymbolStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UsingDeconstructionSymbol {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for UsingDirectiveStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UsingDirective {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for VariableDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VariableDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for VariableDeclarationStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VariableDeclarationStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for VariableDeclarationValueStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VariableDeclarationValue {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for VersionPragmaStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VersionPragma {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for VersionRangeStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VersionRange {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for VersionTermStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VersionTerm {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for WhileStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for WhileStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulBlockStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulBlock {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulBreakStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulBreakStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulContinueStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulContinueStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulDefaultCaseStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulDefaultCase {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulFlagsDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulFlagsDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulForStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulForStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulFunctionCallExpressionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulFunctionCallExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulFunctionDefinitionStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulFunctionDefinition {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulIfStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulIfStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulLeaveStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulLeaveStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulParametersDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulParametersDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulReturnsDeclarationStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulReturnsDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulSwitchStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulSwitchStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulValueCaseStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulValueCase {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulVariableAssignmentStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulVariableAssignmentStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulVariableDeclarationStatementStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulVariableDeclarationStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for YulVariableDeclarationValueStruct {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulVariableDeclarationValue {
    fn text_range(&self) -> Option<Range<usize>> {
        self.as_ref().text_range()
    }
}

impl TextRange for AbicoderVersion {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ArgumentsDeclaration {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ConstructorAttribute {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ContractMember {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ContractSpecifier {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ElementaryType {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ExperimentalFeature {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Expression {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Expression_AdditiveExpression_Operator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Expression_AssignmentExpression_Operator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Expression_EqualityExpression_Operator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Expression_InequalityExpression_Operator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Expression_MultiplicativeExpression_Operator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Expression_PostfixExpression_Operator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Expression_PrefixExpression_Operator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Expression_ShiftExpression_Operator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FallbackFunctionAttribute {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ForStatementCondition {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ForStatementInitialization {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionAttribute {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionBody {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionName {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionTypeAttribute {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for IdentifierPathElement {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ImportClause {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MappingKeyType {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ModifierAttribute {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for NumberUnit {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Pragma {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ReceiveFunctionAttribute {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SourceUnitMember {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StateVariableAttribute {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Statement {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StorageLocation {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StringExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TypeName {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UsingClause {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UsingOperator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UsingTarget {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VariableDeclarationTarget {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VersionExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VersionLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VersionOperator {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulExpression {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulStatement {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulSwitchCase {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ArrayValues {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CallOptions {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CatchClauses {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ConstructorAttributes {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ContractMembers {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ContractSpecifiers {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EnumMembers {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ErrorParameters {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EventParameters {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FallbackFunctionAttributes {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionAttributes {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionTypeAttributes {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for HexStringLiterals {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for IdentifierPath {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ImportDeconstructionSymbols {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InheritanceTypes {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InterfaceMembers {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for LibraryMembers {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ModifierAttributes {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MultiTypedDeclarationElements {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for NamedArguments {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for OverridePaths {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Parameters {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PositionalArguments {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ReceiveFunctionAttributes {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SimpleVersionLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SourceUnitMembers {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StateVariableAttributes {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Statements {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StringLiterals {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StructMembers {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TupleValues {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UnicodeStringLiterals {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UsingDeconstructionSymbols {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VersionExpressionSet {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VersionExpressionSets {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulArguments {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulFlags {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulParameters {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulPath {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulPaths {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulStatements {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulSwitchCases {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulVariableNames {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ABIEncoderV2Keyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AbicoderKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AbicoderV1Keyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AbicoderV2Keyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AbstractKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AddressKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AfterKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AliasKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Ampersand {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AmpersandAmpersand {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AmpersandEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AnonymousKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ApplyKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AsKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AssemblyKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Asterisk {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AsteriskAsterisk {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AsteriskEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AtKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for AutoKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Bang {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BangEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Bar {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BarBar {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BarEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BoolKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BreakKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ByteKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for BytesKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CallDataKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Caret {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CaretEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CaseKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CatchKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CloseBrace {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CloseBracket {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CloseParen {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Colon {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Comma {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ConstantKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ConstructorKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ContinueKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ContractKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for CopyOfKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for DaysKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for DecimalLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for DefaultKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for DefineKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for DeleteKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for DoKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ElseKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EmitKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EnumKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Equal {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EqualEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EqualGreaterThan {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ErrorKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EtherKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for EventKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ExperimentalKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ExternalKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FallbackKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FalseKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FinalKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FixedKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ForKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FromKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for FunctionKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for GlobalKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for GreaterThan {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for GreaterThanEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for GreaterThanGreaterThan {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for GreaterThanGreaterThanEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for GreaterThanGreaterThanGreaterThan {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for GreaterThanGreaterThanGreaterThanEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for GweiKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for HexKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for HexLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for HexStringLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for HoursKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Identifier {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for IfKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ImmutableKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ImplementsKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ImportKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for IndexedKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InlineKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for IntKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InterfaceKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for InternalKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for IsKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for LayoutKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for LessThan {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for LessThanEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for LessThanLessThan {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for LessThanLessThanEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for LetKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for LibraryKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MacroKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MappingKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MatchKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MemoryKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Minus {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MinusEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MinusMinus {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MinutesKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ModifierKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for MutableKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for NewKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for NullKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for OfKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for OpenBrace {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for OpenBracket {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for OpenParen {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for OverrideKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PartialKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PayableKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Percent {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PercentEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Period {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Plus {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PlusEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PlusPlus {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaBarBar {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaCaret {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaGreaterThan {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaGreaterThanEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaLessThan {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaLessThanEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaMinus {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaPeriod {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaSemicolon {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaStringLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PragmaTilde {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PrivateKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PromiseKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PublicKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for PureKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for QuestionMark {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ReceiveKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ReferenceKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for RelocatableKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ReturnKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ReturnsKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for RevertKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SMTCheckerKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SealedKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SecondsKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Semicolon {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SizeOfKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Slash {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SlashEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SolidityKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StaticKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StorageKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StringKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StringLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for StructKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SuperKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SupportsKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for SwitchKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ThisKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ThrowKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for Tilde {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TransientKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TrueKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TryKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TypeDefKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TypeKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for TypeOfKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UfixedKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UintKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UncheckedKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UnicodeStringLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for UsingKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VarKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VersionSpecifier {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for ViewKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for VirtualKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for WeeksKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for WeiKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for WhileKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YearsKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulBreakKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulCaseKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulCloseBrace {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulCloseParen {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulColonEqual {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulComma {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulContinueKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulDecimalLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulDefaultKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulFalseKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulForKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulFunctionKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulHexKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulHexLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulHexStringLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulIdentifier {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulIfKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulLeaveKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulLetKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulMinusGreaterThan {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulOpenBrace {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulOpenParen {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulPeriod {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulStringLiteral {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulSuperKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulSwitchKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulThisKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}

impl TextRange for YulTrueKeyword {
    fn text_range(&self) -> Option<Range<usize>> {
        Some(self.text_start()?..self.text_end()?)
    }
}
