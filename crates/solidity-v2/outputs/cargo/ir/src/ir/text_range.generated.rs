// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::wildcard_imports)]

use std::ops::Range;
use std::sync::Arc;

use super::nodes::*;

/// A trait for IR nodes that can report their text range in the source.
///
/// It returns `None` for nodes that can be legitimately
/// empty, ie. collections (eg. the positional arguments of `f()`),
/// and for nodes that may not be represented in the source code,
/// ie. external nodes.
pub trait TextRange {
    /// Returns the text range of this node, or `None` if the node is empty.
    fn calculate_range(&self) -> Option<Range<usize>>;
}

impl<T: TextRange> TextRange for Arc<T> {
    fn calculate_range(&self) -> Option<Range<usize>> {
        self.as_ref().calculate_range()
    }
}

impl<T: TextRange> TextRange for Option<T> {
    fn calculate_range(&self) -> Option<Range<usize>> {
        self.as_ref().and_then(TextRange::calculate_range)
    }
}

impl<T: TextRange> TextRange for Vec<T> {
    fn calculate_range(&self) -> Option<Range<usize>> {
        let start = self.first()?.calculate_range()?.start;
        let end = self.last()?.calculate_range()?.end;
        Some(start..end)
    }
}

impl TextRange for AbicoderPragmaStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AdditiveExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AddressTypeStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AndExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ArrayExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ArrayTypeNameStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AssemblyStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AssignmentExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BitwiseAndExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BitwiseOrExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BitwiseXorExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BlockStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BreakStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for CallOptionsExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for CatchClauseStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for CatchClauseErrorStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ConditionalExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ConstantDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ContinueStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ContractDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for DecimalNumberExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for DoWhileStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for EmitStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for EnumDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for EqualityExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ErrorDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for EventDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ExperimentalPragmaStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ExponentiationExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ExpressionStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ForStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for FunctionCallExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for FunctionDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for FunctionTypeStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for HexNumberExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for IfStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ImportDeconstructionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ImportDeconstructionSymbolStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for IndexAccessExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for InequalityExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for InheritanceTypeStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for InterfaceDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for LibraryDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MappingTypeStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MemberAccessExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ModifierInvocationStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MultiTypedDeclarationStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MultiTypedDeclarationElementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MultiplicativeExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for NamedArgumentStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for NewExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for OrExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ParameterStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PathImportStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PostfixExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PragmaDirectiveStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PrefixExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ReturnStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for RevertStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ShiftExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for SingleTypedDeclarationStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for SourceUnitStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for StateVariableDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for StructDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for StructMemberStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for TryStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for TupleExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for TupleValueStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for TypeExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for UncheckedBlockStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for UserDefinedValueTypeDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for UsingDeconstructionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for UsingDeconstructionSymbolStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for UsingDirectiveStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for VariableDeclarationStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for VariableDeclarationStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for VersionPragmaStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for VersionRangeStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for VersionTermStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for WhileStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulBlockStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulBreakStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulContinueStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulDefaultCaseStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulForStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulFunctionCallExpressionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulFunctionDefinitionStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulIfStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulLeaveStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulSwitchStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulValueCaseStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulVariableAssignmentStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulVariableDeclarationStatementStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for YulVariableDeclarationValueStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AbicoderVersion {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            AbicoderVersion::AbicoderV1Keyword(inner) => inner.calculate_range(),

            AbicoderVersion::AbicoderV2Keyword(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for AdditiveExpressionOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            AdditiveExpressionOperator::Minus(inner) => inner.calculate_range(),

            AdditiveExpressionOperator::Plus(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for ArgumentsDeclaration {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            ArgumentsDeclaration::PositionalArguments(inner) => inner.calculate_range(),

            ArgumentsDeclaration::NamedArguments(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for AssignmentExpressionOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            AssignmentExpressionOperator::AmpersandEqual(inner) => inner.calculate_range(),

            AssignmentExpressionOperator::AsteriskEqual(inner) => inner.calculate_range(),

            AssignmentExpressionOperator::BarEqual(inner) => inner.calculate_range(),

            AssignmentExpressionOperator::CaretEqual(inner) => inner.calculate_range(),

            AssignmentExpressionOperator::Equal(inner) => inner.calculate_range(),

            AssignmentExpressionOperator::GreaterThanGreaterThanEqual(inner) => {
                inner.calculate_range()
            }

            AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(inner) => {
                inner.calculate_range()
            }

            AssignmentExpressionOperator::LessThanLessThanEqual(inner) => inner.calculate_range(),

            AssignmentExpressionOperator::MinusEqual(inner) => inner.calculate_range(),

            AssignmentExpressionOperator::PercentEqual(inner) => inner.calculate_range(),

            AssignmentExpressionOperator::PlusEqual(inner) => inner.calculate_range(),

            AssignmentExpressionOperator::SlashEqual(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for ContractMember {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            ContractMember::UsingDirective(inner) => inner.calculate_range(),

            ContractMember::FunctionDefinition(inner) => inner.calculate_range(),

            ContractMember::StructDefinition(inner) => inner.calculate_range(),

            ContractMember::EnumDefinition(inner) => inner.calculate_range(),

            ContractMember::EventDefinition(inner) => inner.calculate_range(),

            ContractMember::ErrorDefinition(inner) => inner.calculate_range(),

            ContractMember::UserDefinedValueTypeDefinition(inner) => inner.calculate_range(),

            ContractMember::StateVariableDefinition(inner) => inner.calculate_range(),

            ContractMember::ConstantDefinition(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for ElementaryType {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            ElementaryType::BoolKeyword(inner) => inner.calculate_range(),

            ElementaryType::StringKeyword(inner) => inner.calculate_range(),

            ElementaryType::AddressType(inner) => inner.calculate_range(),

            ElementaryType::BytesKeyword(inner) => inner.calculate_range(),

            ElementaryType::IntKeyword(inner) => inner.calculate_range(),

            ElementaryType::UintKeyword(inner) => inner.calculate_range(),

            ElementaryType::FixedKeyword(inner) => inner.calculate_range(),

            ElementaryType::UfixedKeyword(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for EqualityExpressionOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            EqualityExpressionOperator::BangEqual(inner) => inner.calculate_range(),

            EqualityExpressionOperator::EqualEqual(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for ExperimentalFeature {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            ExperimentalFeature::ABIEncoderV2Keyword(inner) => inner.calculate_range(),

            ExperimentalFeature::SMTCheckerKeyword(inner) => inner.calculate_range(),

            ExperimentalFeature::StringLiteral(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for Expression {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            Expression::AssignmentExpression(inner) => inner.calculate_range(),

            Expression::ConditionalExpression(inner) => inner.calculate_range(),

            Expression::OrExpression(inner) => inner.calculate_range(),

            Expression::AndExpression(inner) => inner.calculate_range(),

            Expression::EqualityExpression(inner) => inner.calculate_range(),

            Expression::InequalityExpression(inner) => inner.calculate_range(),

            Expression::BitwiseOrExpression(inner) => inner.calculate_range(),

            Expression::BitwiseXorExpression(inner) => inner.calculate_range(),

            Expression::BitwiseAndExpression(inner) => inner.calculate_range(),

            Expression::ShiftExpression(inner) => inner.calculate_range(),

            Expression::AdditiveExpression(inner) => inner.calculate_range(),

            Expression::MultiplicativeExpression(inner) => inner.calculate_range(),

            Expression::ExponentiationExpression(inner) => inner.calculate_range(),

            Expression::PostfixExpression(inner) => inner.calculate_range(),

            Expression::PrefixExpression(inner) => inner.calculate_range(),

            Expression::FunctionCallExpression(inner) => inner.calculate_range(),

            Expression::CallOptionsExpression(inner) => inner.calculate_range(),

            Expression::MemberAccessExpression(inner) => inner.calculate_range(),

            Expression::IndexAccessExpression(inner) => inner.calculate_range(),

            Expression::NewExpression(inner) => inner.calculate_range(),

            Expression::TupleExpression(inner) => inner.calculate_range(),

            Expression::TypeExpression(inner) => inner.calculate_range(),

            Expression::ArrayExpression(inner) => inner.calculate_range(),

            Expression::HexNumberExpression(inner) => inner.calculate_range(),

            Expression::DecimalNumberExpression(inner) => inner.calculate_range(),

            Expression::StringExpression(inner) => inner.calculate_range(),

            Expression::ElementaryType(inner) => inner.calculate_range(),

            Expression::PayableKeyword(inner) => inner.calculate_range(),

            Expression::ThisKeyword(inner) => inner.calculate_range(),

            Expression::SuperKeyword(inner) => inner.calculate_range(),

            Expression::TrueKeyword(inner) => inner.calculate_range(),

            Expression::FalseKeyword(inner) => inner.calculate_range(),

            Expression::Identifier(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for ForStatementCondition {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            ForStatementCondition::ExpressionStatement(inner) => inner.calculate_range(),

            ForStatementCondition::Semicolon(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for ForStatementInitialization {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            ForStatementInitialization::VariableDeclarationStatement(inner) => {
                inner.calculate_range()
            }

            ForStatementInitialization::ExpressionStatement(inner) => inner.calculate_range(),

            ForStatementInitialization::Semicolon(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for FunctionKind {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            FunctionKind::Regular => None,

            FunctionKind::Constructor => None,

            FunctionKind::Fallback => None,

            FunctionKind::Receive => None,

            FunctionKind::Modifier => None,
        }
    }
}

impl TextRange for FunctionMutability {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            FunctionMutability::Pure => None,

            FunctionMutability::View => None,

            FunctionMutability::NonPayable => None,

            FunctionMutability::Payable => None,
        }
    }
}

impl TextRange for FunctionVisibility {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            FunctionVisibility::Public => None,

            FunctionVisibility::Private => None,

            FunctionVisibility::Internal => None,

            FunctionVisibility::External => None,
        }
    }
}

impl TextRange for ImportClause {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            ImportClause::PathImport(inner) => inner.calculate_range(),

            ImportClause::ImportDeconstruction(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for InequalityExpressionOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            InequalityExpressionOperator::GreaterThan(inner) => inner.calculate_range(),

            InequalityExpressionOperator::GreaterThanEqual(inner) => inner.calculate_range(),

            InequalityExpressionOperator::LessThan(inner) => inner.calculate_range(),

            InequalityExpressionOperator::LessThanEqual(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for MultiplicativeExpressionOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            MultiplicativeExpressionOperator::Asterisk(inner) => inner.calculate_range(),

            MultiplicativeExpressionOperator::Percent(inner) => inner.calculate_range(),

            MultiplicativeExpressionOperator::Slash(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for NumberUnit {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            NumberUnit::WeiKeyword(inner) => inner.calculate_range(),

            NumberUnit::GweiKeyword(inner) => inner.calculate_range(),

            NumberUnit::EtherKeyword(inner) => inner.calculate_range(),

            NumberUnit::SecondsKeyword(inner) => inner.calculate_range(),

            NumberUnit::MinutesKeyword(inner) => inner.calculate_range(),

            NumberUnit::HoursKeyword(inner) => inner.calculate_range(),

            NumberUnit::DaysKeyword(inner) => inner.calculate_range(),

            NumberUnit::WeeksKeyword(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for PostfixExpressionOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            PostfixExpressionOperator::MinusMinus(inner) => inner.calculate_range(),

            PostfixExpressionOperator::PlusPlus(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for Pragma {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            Pragma::VersionPragma(inner) => inner.calculate_range(),

            Pragma::AbicoderPragma(inner) => inner.calculate_range(),

            Pragma::ExperimentalPragma(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for PrefixExpressionOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            PrefixExpressionOperator::Bang(inner) => inner.calculate_range(),

            PrefixExpressionOperator::DeleteKeyword(inner) => inner.calculate_range(),

            PrefixExpressionOperator::Minus(inner) => inner.calculate_range(),

            PrefixExpressionOperator::MinusMinus(inner) => inner.calculate_range(),

            PrefixExpressionOperator::PlusPlus(inner) => inner.calculate_range(),

            PrefixExpressionOperator::Tilde(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for ShiftExpressionOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            ShiftExpressionOperator::GreaterThanGreaterThan(inner) => inner.calculate_range(),

            ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(inner) => {
                inner.calculate_range()
            }

            ShiftExpressionOperator::LessThanLessThan(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for SourceUnitMember {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            SourceUnitMember::PragmaDirective(inner) => inner.calculate_range(),

            SourceUnitMember::ImportClause(inner) => inner.calculate_range(),

            SourceUnitMember::ContractDefinition(inner) => inner.calculate_range(),

            SourceUnitMember::InterfaceDefinition(inner) => inner.calculate_range(),

            SourceUnitMember::LibraryDefinition(inner) => inner.calculate_range(),

            SourceUnitMember::StructDefinition(inner) => inner.calculate_range(),

            SourceUnitMember::EnumDefinition(inner) => inner.calculate_range(),

            SourceUnitMember::FunctionDefinition(inner) => inner.calculate_range(),

            SourceUnitMember::ErrorDefinition(inner) => inner.calculate_range(),

            SourceUnitMember::UserDefinedValueTypeDefinition(inner) => inner.calculate_range(),

            SourceUnitMember::UsingDirective(inner) => inner.calculate_range(),

            SourceUnitMember::EventDefinition(inner) => inner.calculate_range(),

            SourceUnitMember::ConstantDefinition(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for StateVariableMutability {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            StateVariableMutability::Mutable => None,

            StateVariableMutability::Constant => None,

            StateVariableMutability::Immutable => None,

            StateVariableMutability::Transient => None,
        }
    }
}

impl TextRange for StateVariableVisibility {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            StateVariableVisibility::Public => None,

            StateVariableVisibility::Private => None,

            StateVariableVisibility::Internal => None,
        }
    }
}

impl TextRange for Statement {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            Statement::IfStatement(inner) => inner.calculate_range(),

            Statement::ForStatement(inner) => inner.calculate_range(),

            Statement::WhileStatement(inner) => inner.calculate_range(),

            Statement::DoWhileStatement(inner) => inner.calculate_range(),

            Statement::ContinueStatement(inner) => inner.calculate_range(),

            Statement::BreakStatement(inner) => inner.calculate_range(),

            Statement::ReturnStatement(inner) => inner.calculate_range(),

            Statement::EmitStatement(inner) => inner.calculate_range(),

            Statement::TryStatement(inner) => inner.calculate_range(),

            Statement::RevertStatement(inner) => inner.calculate_range(),

            Statement::AssemblyStatement(inner) => inner.calculate_range(),

            Statement::Block(inner) => inner.calculate_range(),

            Statement::UncheckedBlock(inner) => inner.calculate_range(),

            Statement::VariableDeclarationStatement(inner) => inner.calculate_range(),

            Statement::ExpressionStatement(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for StorageLocation {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            StorageLocation::MemoryKeyword(inner) => inner.calculate_range(),

            StorageLocation::StorageKeyword(inner) => inner.calculate_range(),

            StorageLocation::CallDataKeyword(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for StringExpression {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            StringExpression::StringLiterals(inner) => inner.calculate_range(),

            StringExpression::HexStringLiterals(inner) => inner.calculate_range(),

            StringExpression::UnicodeStringLiterals(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for TypeName {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            TypeName::ArrayTypeName(inner) => inner.calculate_range(),

            TypeName::FunctionType(inner) => inner.calculate_range(),

            TypeName::MappingType(inner) => inner.calculate_range(),

            TypeName::ElementaryType(inner) => inner.calculate_range(),

            TypeName::IdentifierPath(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for UsingClause {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            UsingClause::IdentifierPath(inner) => inner.calculate_range(),

            UsingClause::UsingDeconstruction(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for UsingOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            UsingOperator::Ampersand(inner) => inner.calculate_range(),

            UsingOperator::Asterisk(inner) => inner.calculate_range(),

            UsingOperator::BangEqual(inner) => inner.calculate_range(),

            UsingOperator::Bar(inner) => inner.calculate_range(),

            UsingOperator::Caret(inner) => inner.calculate_range(),

            UsingOperator::EqualEqual(inner) => inner.calculate_range(),

            UsingOperator::GreaterThan(inner) => inner.calculate_range(),

            UsingOperator::GreaterThanEqual(inner) => inner.calculate_range(),

            UsingOperator::LessThan(inner) => inner.calculate_range(),

            UsingOperator::LessThanEqual(inner) => inner.calculate_range(),

            UsingOperator::Minus(inner) => inner.calculate_range(),

            UsingOperator::Percent(inner) => inner.calculate_range(),

            UsingOperator::Plus(inner) => inner.calculate_range(),

            UsingOperator::Slash(inner) => inner.calculate_range(),

            UsingOperator::Tilde(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for UsingTarget {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            UsingTarget::TypeName(inner) => inner.calculate_range(),

            UsingTarget::Asterisk(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for VariableDeclarationTarget {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            VariableDeclarationTarget::SingleTypedDeclaration(inner) => inner.calculate_range(),

            VariableDeclarationTarget::MultiTypedDeclaration(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for VersionExpression {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            VersionExpression::VersionRange(inner) => inner.calculate_range(),

            VersionExpression::VersionTerm(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for VersionLiteral {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            VersionLiteral::SimpleVersionLiteral(inner) => inner.calculate_range(),

            VersionLiteral::StringLiteral(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for VersionOperator {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            VersionOperator::PragmaCaret(inner) => inner.calculate_range(),

            VersionOperator::PragmaTilde(inner) => inner.calculate_range(),

            VersionOperator::PragmaEqual(inner) => inner.calculate_range(),

            VersionOperator::PragmaLessThan(inner) => inner.calculate_range(),

            VersionOperator::PragmaGreaterThan(inner) => inner.calculate_range(),

            VersionOperator::PragmaLessThanEqual(inner) => inner.calculate_range(),

            VersionOperator::PragmaGreaterThanEqual(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for YulExpression {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            YulExpression::YulFunctionCallExpression(inner) => inner.calculate_range(),

            YulExpression::YulLiteral(inner) => inner.calculate_range(),

            YulExpression::YulPath(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for YulLiteral {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            YulLiteral::TrueKeyword(inner) => inner.calculate_range(),

            YulLiteral::FalseKeyword(inner) => inner.calculate_range(),

            YulLiteral::DecimalLiteral(inner) => inner.calculate_range(),

            YulLiteral::HexLiteral(inner) => inner.calculate_range(),

            YulLiteral::HexStringLiteral(inner) => inner.calculate_range(),

            YulLiteral::StringLiteral(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for YulStatement {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            YulStatement::YulBlock(inner) => inner.calculate_range(),

            YulStatement::YulFunctionDefinition(inner) => inner.calculate_range(),

            YulStatement::YulIfStatement(inner) => inner.calculate_range(),

            YulStatement::YulForStatement(inner) => inner.calculate_range(),

            YulStatement::YulSwitchStatement(inner) => inner.calculate_range(),

            YulStatement::YulLeaveStatement(inner) => inner.calculate_range(),

            YulStatement::YulBreakStatement(inner) => inner.calculate_range(),

            YulStatement::YulContinueStatement(inner) => inner.calculate_range(),

            YulStatement::YulVariableAssignmentStatement(inner) => inner.calculate_range(),

            YulStatement::YulVariableDeclarationStatement(inner) => inner.calculate_range(),

            YulStatement::YulExpression(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for YulSwitchCase {
    fn calculate_range(&self) -> Option<Range<usize>> {
        match self {
            YulSwitchCase::YulDefaultCase(inner) => inner.calculate_range(),

            YulSwitchCase::YulValueCase(inner) => inner.calculate_range(),
        }
    }
}

impl TextRange for ABIEncoderV2KeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AbicoderV1KeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AbicoderV2KeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AbstractKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AmpersandStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AmpersandEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AnonymousKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AsteriskStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for AsteriskEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BangStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BangEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BarStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BarEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BoolKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for BytesKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for CallDataKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for CaretStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for CaretEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for DaysKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for DecimalLiteralStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for DeleteKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for EqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for EqualEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for EtherKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for FalseKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for FixedKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for GlobalKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for GreaterThanStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for GreaterThanEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for GreaterThanGreaterThanStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for GreaterThanGreaterThanEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for GreaterThanGreaterThanGreaterThanStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for GreaterThanGreaterThanGreaterThanEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for GweiKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for HexLiteralStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for HexStringLiteralStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for HoursKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for IdentifierStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for IndexedKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for IntKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for LessThanStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for LessThanEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for LessThanLessThanStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for LessThanLessThanEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MemoryKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MinusStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MinusEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MinusMinusStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for MinutesKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PayableKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PercentStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PercentEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PlusStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PlusEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PlusPlusStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PragmaCaretStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PragmaEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PragmaGreaterThanStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PragmaGreaterThanEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PragmaLessThanStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PragmaLessThanEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for PragmaTildeStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for SMTCheckerKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for SecondsKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for SemicolonStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for SlashStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for SlashEqualStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for StorageKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for StringKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for StringLiteralStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for SuperKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for ThisKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for TildeStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for TrueKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for UfixedKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for UintKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for UnicodeStringLiteralStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for VersionSpecifierStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for VirtualKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for WeeksKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TextRange for WeiKeywordStruct {
    fn calculate_range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}
