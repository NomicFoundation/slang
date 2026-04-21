// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::wildcard_imports)]

use std::ops::Range;
use std::rc::Rc;

use super::text_end::TextEnd;
use super::text_start::TextStart;
use crate::structured_cst::nodes::*;

/// A trait for CST nodes that can report their text range in the source.
pub trait TextRange {
    /// Returns the text range of this node, or `None` if the node is empty.
    fn calculate_text_range(&self) -> Option<Range<usize>>;
}

impl<T: TextRange> TextRange for Option<T> {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.as_ref().and_then(TextRange::calculate_text_range)
    }
}

impl<T: TextRange> TextRange for Rc<T> {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        (**self).calculate_text_range()
    }
}

impl TextRange for AbicoderPragmaStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AdditiveExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AddressTypeStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AndExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ArrayExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ArrayTypeNameStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AssemblyStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AssignmentExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BitwiseAndExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BitwiseOrExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BitwiseXorExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BlockStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BreakStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CallOptionsExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CatchClauseStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CatchClauseErrorStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ConditionalExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ConstantDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ConstructorDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ContinueStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ContractDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for DecimalNumberExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for DoWhileStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ElseBranchStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EmitStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EnumDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EqualityExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ErrorDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ErrorParameterStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ErrorParametersDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EventDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EventParameterStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EventParametersDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ExperimentalPragmaStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ExponentiationExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ExpressionStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FallbackFunctionDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ForStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionCallExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionTypeStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for HexNumberExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for IfStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ImportAliasStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ImportDeconstructionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ImportDeconstructionSymbolStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ImportDirectiveStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for IndexAccessEndStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for IndexAccessExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InequalityExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InheritanceSpecifierStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InheritanceTypeStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InterfaceDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for LibraryDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MappingKeyStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MappingTypeStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MappingValueStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MemberAccessExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ModifierDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ModifierInvocationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MultiTypedDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MultiTypedDeclarationElementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MultiplicativeExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for NamedArgumentStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for NamedArgumentGroupStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for NamedArgumentsDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for NamedImportStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for NewExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for OrExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for OverridePathsDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for OverrideSpecifierStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ParameterStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ParametersDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PathImportStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PositionalArgumentsDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PostfixExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaDirectiveStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PrefixExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ReceiveFunctionDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ReturnStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ReturnsDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for RevertStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ShiftExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SingleTypedDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SourceUnitStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StateVariableDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StateVariableDefinitionValueStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StorageLayoutSpecifierStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StructDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StructMemberStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TryStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TupleExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TupleValueStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TypeExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UncheckedBlockStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UserDefinedValueTypeDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UsingAliasStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UsingDeconstructionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UsingDeconstructionSymbolStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UsingDirectiveStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VariableDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VariableDeclarationStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VariableDeclarationValueStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VersionPragmaStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VersionRangeStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VersionTermStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for WhileStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulBlockStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulBreakStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulContinueStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulDefaultCaseStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulFlagsDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulForStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulFunctionCallExpressionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulFunctionDefinitionStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulIfStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulLeaveStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulParametersDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulReturnsDeclarationStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulSwitchStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulValueCaseStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulVariableAssignmentStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulVariableDeclarationStatementStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulVariableDeclarationValueStruct {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AbicoderVersion {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ArgumentsDeclaration {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ConstructorAttribute {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ContractMember {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ContractSpecifier {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ElementaryType {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ExperimentalFeature {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Expression {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Expression_AdditiveExpression_Operator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Expression_AssignmentExpression_Operator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Expression_EqualityExpression_Operator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Expression_InequalityExpression_Operator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Expression_MultiplicativeExpression_Operator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Expression_PostfixExpression_Operator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Expression_PrefixExpression_Operator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Expression_ShiftExpression_Operator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FallbackFunctionAttribute {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ForStatementCondition {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ForStatementInitialization {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionAttribute {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionBody {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionName {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionTypeAttribute {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for IdentifierPathElement {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ImportClause {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MappingKeyType {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ModifierAttribute {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for NumberUnit {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Pragma {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ReceiveFunctionAttribute {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SourceUnitMember {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StateVariableAttribute {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Statement {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StorageLocation {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StringExpression {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TypeName {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UsingClause {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UsingOperator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UsingTarget {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VariableDeclarationTarget {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VersionExpression {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VersionLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VersionOperator {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulExpression {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulStatement {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulSwitchCase {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ArrayValues {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CallOptions {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CatchClauses {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ConstructorAttributes {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ContractMembers {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ContractSpecifiers {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EnumMembers {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ErrorParameters {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EventParameters {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FallbackFunctionAttributes {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionAttributes {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionTypeAttributes {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for HexStringLiterals {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for IdentifierPath {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ImportDeconstructionSymbols {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InheritanceTypes {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InterfaceMembers {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for LibraryMembers {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ModifierAttributes {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MultiTypedDeclarationElements {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for NamedArguments {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for OverridePaths {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Parameters {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PositionalArguments {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ReceiveFunctionAttributes {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SimpleVersionLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SourceUnitMembers {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StateVariableAttributes {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Statements {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StringLiterals {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StructMembers {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TupleValues {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UnicodeStringLiterals {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UsingDeconstructionSymbols {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VersionExpressionSet {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VersionExpressionSets {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulArguments {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulFlags {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulParameters {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulPath {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulPaths {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulStatements {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulSwitchCases {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulVariableNames {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ABIEncoderV2Keyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AbicoderKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AbicoderV1Keyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AbicoderV2Keyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AbstractKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AddressKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AfterKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AliasKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Ampersand {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AmpersandAmpersand {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AmpersandEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AnonymousKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ApplyKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AsKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AssemblyKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Asterisk {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AsteriskAsterisk {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AsteriskEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AtKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for AutoKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Bang {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BangEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Bar {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BarBar {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BarEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BoolKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BreakKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ByteKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for BytesKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CallDataKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Caret {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CaretEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CaseKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CatchKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CloseBrace {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CloseBracket {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CloseParen {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Colon {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Comma {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ConstantKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ConstructorKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ContinueKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ContractKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for CopyOfKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for DaysKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for DecimalLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for DefaultKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for DefineKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for DeleteKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for DoKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ElseKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EmitKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EnumKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Equal {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EqualEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EqualGreaterThan {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ErrorKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EtherKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for EventKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ExperimentalKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ExternalKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FallbackKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FalseKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FinalKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FixedKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ForKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FromKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for FunctionKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for GlobalKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for GreaterThan {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for GreaterThanEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for GreaterThanGreaterThan {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for GreaterThanGreaterThanEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for GreaterThanGreaterThanGreaterThan {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for GreaterThanGreaterThanGreaterThanEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for GweiKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for HexKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for HexLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for HexStringLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for HoursKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Identifier {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for IfKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ImmutableKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ImplementsKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ImportKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for IndexedKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InlineKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for IntKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InterfaceKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for InternalKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for IsKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for LayoutKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for LessThan {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for LessThanEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for LessThanLessThan {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for LessThanLessThanEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for LetKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for LibraryKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MacroKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MappingKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MatchKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MemoryKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Minus {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MinusEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MinusMinus {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MinutesKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ModifierKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for MutableKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for NewKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for NullKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for OfKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for OpenBrace {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for OpenBracket {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for OpenParen {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for OverrideKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PartialKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PayableKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Percent {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PercentEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Period {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Plus {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PlusEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PlusPlus {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaBarBar {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaCaret {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaGreaterThan {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaGreaterThanEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaLessThan {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaLessThanEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaMinus {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaPeriod {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaSemicolon {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaStringLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PragmaTilde {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PrivateKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PromiseKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PublicKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for PureKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for QuestionMark {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ReceiveKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ReferenceKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for RelocatableKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ReturnKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ReturnsKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for RevertKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SMTCheckerKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SealedKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SecondsKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Semicolon {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SizeOfKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Slash {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SlashEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SolidityKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StaticKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StorageKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StringKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StringLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for StructKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SuperKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SupportsKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for SwitchKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ThisKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ThrowKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for Tilde {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TransientKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TrueKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TryKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TypeDefKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TypeKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for TypeOfKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UfixedKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UintKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UncheckedKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UnicodeStringLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for UsingKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VarKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VersionSpecifier {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for ViewKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for VirtualKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for WeeksKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for WeiKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for WhileKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YearsKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulBreakKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulCaseKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulCloseBrace {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulCloseParen {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulColonEqual {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulComma {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulContinueKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulDecimalLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulDefaultKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulFalseKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulForKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulFunctionKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulHexKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulHexLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulHexStringLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulIdentifier {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulIfKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulLeaveKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulLetKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulMinusGreaterThan {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulOpenBrace {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulOpenParen {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulPeriod {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulStringLiteral {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulSuperKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulSwitchKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulThisKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}

impl TextRange for YulTrueKeyword {
    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.calculate_text_start()?..self.calculate_text_end()?)
    }
}
