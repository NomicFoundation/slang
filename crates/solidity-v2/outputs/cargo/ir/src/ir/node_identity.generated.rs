// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::wildcard_imports)]

use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;

use super::nodes::*;

/// A trait for IR nodes that can report the `NodeId` that identifies them.
///
/// It returns `None` for nodes that can be legitimately
/// empty, ie. collections (eg. the positional arguments of `f()`),
/// and for nodes that may not be represented in the source code,
/// ie. external nodes.
pub trait NodeIdentity {
    /// Returns the `NodeId` of this node, or `None` if the node is empty
    /// or not represented in the source code.
    fn node_id(&self) -> Option<NodeId>;
}

impl<T: NodeIdentity> NodeIdentity for Arc<T> {
    fn node_id(&self) -> Option<NodeId> {
        self.as_ref().node_id()
    }
}

impl<T: NodeIdentity> NodeIdentity for Option<T> {
    fn node_id(&self) -> Option<NodeId> {
        self.as_ref().and_then(NodeIdentity::node_id)
    }
}

impl<T: NodeIdentity> NodeIdentity for Vec<T> {
    fn node_id(&self) -> Option<NodeId> {
        self.first()?.node_id()
    }
}

impl NodeIdentity for AbicoderPragmaStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AdditiveExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AddressTypeStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AndExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ArrayExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ArrayTypeNameStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AssemblyStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AssignmentExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BitwiseAndExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BitwiseOrExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BitwiseXorExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BlockStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BreakStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for CallOptionsExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for CatchClauseStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for CatchClauseErrorStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ConditionalExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ConstantDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ContinueStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ContractDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for DecimalNumberExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for DoWhileStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for EmitStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for EnumDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for EqualityExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ErrorDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for EventDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ExperimentalPragmaStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ExponentiationExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ExpressionStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ForStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for FunctionCallExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for FunctionDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for FunctionTypeStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for HexNumberExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for IfStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ImportDeconstructionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ImportDeconstructionSymbolStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for IndexAccessExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for InequalityExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for InheritanceTypeStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for InterfaceDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for LibraryDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MappingTypeStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MemberAccessExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ModifierInvocationStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MultiTypedDeclarationStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MultiTypedDeclarationElementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MultiplicativeExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for NamedArgumentStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for NewExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for OrExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ParameterStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PathImportStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PostfixExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PragmaDirectiveStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PrefixExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ReturnStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for RevertStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ShiftExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for SingleTypedDeclarationStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for SourceUnitStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for StateVariableDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for StructDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for StructMemberStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for TryStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for TupleExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for TupleValueStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for TypeExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for UncheckedBlockStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for UserDefinedValueTypeDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for UsingDeconstructionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for UsingDeconstructionSymbolStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for UsingDirectiveStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for VariableDeclarationStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for VariableDeclarationStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for VersionPragmaStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for VersionRangeStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for VersionTermStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for WhileStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulBlockStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulBreakStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulContinueStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulDefaultCaseStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulForStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulFunctionCallExpressionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulFunctionDefinitionStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulIfStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulLeaveStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulSwitchStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulValueCaseStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulVariableAssignmentStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulVariableDeclarationStatementStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for YulVariableDeclarationValueStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AbicoderVersion {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            AbicoderVersion::AbicoderV1Keyword(inner) => inner.node_id(),

            AbicoderVersion::AbicoderV2Keyword(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for AdditiveExpressionOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            AdditiveExpressionOperator::Minus(inner) => inner.node_id(),

            AdditiveExpressionOperator::Plus(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for ArgumentsDeclaration {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            ArgumentsDeclaration::PositionalArguments(inner) => inner.node_id(),

            ArgumentsDeclaration::NamedArguments(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for AssignmentExpressionOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            AssignmentExpressionOperator::AmpersandEqual(inner) => inner.node_id(),

            AssignmentExpressionOperator::AsteriskEqual(inner) => inner.node_id(),

            AssignmentExpressionOperator::BarEqual(inner) => inner.node_id(),

            AssignmentExpressionOperator::CaretEqual(inner) => inner.node_id(),

            AssignmentExpressionOperator::Equal(inner) => inner.node_id(),

            AssignmentExpressionOperator::GreaterThanGreaterThanEqual(inner) => inner.node_id(),

            AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(inner) => {
                inner.node_id()
            }

            AssignmentExpressionOperator::LessThanLessThanEqual(inner) => inner.node_id(),

            AssignmentExpressionOperator::MinusEqual(inner) => inner.node_id(),

            AssignmentExpressionOperator::PercentEqual(inner) => inner.node_id(),

            AssignmentExpressionOperator::PlusEqual(inner) => inner.node_id(),

            AssignmentExpressionOperator::SlashEqual(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for ContractMember {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            ContractMember::UsingDirective(inner) => inner.node_id(),

            ContractMember::FunctionDefinition(inner) => inner.node_id(),

            ContractMember::StructDefinition(inner) => inner.node_id(),

            ContractMember::EnumDefinition(inner) => inner.node_id(),

            ContractMember::EventDefinition(inner) => inner.node_id(),

            ContractMember::ErrorDefinition(inner) => inner.node_id(),

            ContractMember::UserDefinedValueTypeDefinition(inner) => inner.node_id(),

            ContractMember::StateVariableDefinition(inner) => inner.node_id(),

            ContractMember::ConstantDefinition(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for ElementaryType {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            ElementaryType::BoolKeyword(inner) => inner.node_id(),

            ElementaryType::StringKeyword(inner) => inner.node_id(),

            ElementaryType::AddressType(inner) => inner.node_id(),

            ElementaryType::BytesKeyword(inner) => inner.node_id(),

            ElementaryType::IntKeyword(inner) => inner.node_id(),

            ElementaryType::UintKeyword(inner) => inner.node_id(),

            ElementaryType::FixedKeyword(inner) => inner.node_id(),

            ElementaryType::UfixedKeyword(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for EqualityExpressionOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            EqualityExpressionOperator::BangEqual(inner) => inner.node_id(),

            EqualityExpressionOperator::EqualEqual(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for ExperimentalFeature {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            ExperimentalFeature::ABIEncoderV2Keyword(inner) => inner.node_id(),

            ExperimentalFeature::SMTCheckerKeyword(inner) => inner.node_id(),

            ExperimentalFeature::StringLiteral(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for Expression {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            Expression::AssignmentExpression(inner) => inner.node_id(),

            Expression::ConditionalExpression(inner) => inner.node_id(),

            Expression::OrExpression(inner) => inner.node_id(),

            Expression::AndExpression(inner) => inner.node_id(),

            Expression::EqualityExpression(inner) => inner.node_id(),

            Expression::InequalityExpression(inner) => inner.node_id(),

            Expression::BitwiseOrExpression(inner) => inner.node_id(),

            Expression::BitwiseXorExpression(inner) => inner.node_id(),

            Expression::BitwiseAndExpression(inner) => inner.node_id(),

            Expression::ShiftExpression(inner) => inner.node_id(),

            Expression::AdditiveExpression(inner) => inner.node_id(),

            Expression::MultiplicativeExpression(inner) => inner.node_id(),

            Expression::ExponentiationExpression(inner) => inner.node_id(),

            Expression::PostfixExpression(inner) => inner.node_id(),

            Expression::PrefixExpression(inner) => inner.node_id(),

            Expression::FunctionCallExpression(inner) => inner.node_id(),

            Expression::CallOptionsExpression(inner) => inner.node_id(),

            Expression::MemberAccessExpression(inner) => inner.node_id(),

            Expression::IndexAccessExpression(inner) => inner.node_id(),

            Expression::NewExpression(inner) => inner.node_id(),

            Expression::TupleExpression(inner) => inner.node_id(),

            Expression::TypeExpression(inner) => inner.node_id(),

            Expression::ArrayExpression(inner) => inner.node_id(),

            Expression::HexNumberExpression(inner) => inner.node_id(),

            Expression::DecimalNumberExpression(inner) => inner.node_id(),

            Expression::StringExpression(inner) => inner.node_id(),

            Expression::ElementaryType(inner) => inner.node_id(),

            Expression::PayableKeyword(inner) => inner.node_id(),

            Expression::ThisKeyword(inner) => inner.node_id(),

            Expression::SuperKeyword(inner) => inner.node_id(),

            Expression::TrueKeyword(inner) => inner.node_id(),

            Expression::FalseKeyword(inner) => inner.node_id(),

            Expression::Identifier(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for ForStatementCondition {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            ForStatementCondition::ExpressionStatement(inner) => inner.node_id(),

            ForStatementCondition::Semicolon(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for ForStatementInitialization {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            ForStatementInitialization::VariableDeclarationStatement(inner) => inner.node_id(),

            ForStatementInitialization::ExpressionStatement(inner) => inner.node_id(),

            ForStatementInitialization::Semicolon(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for FunctionKind {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            FunctionKind::Regular => None,

            FunctionKind::Constructor => None,

            FunctionKind::Fallback => None,

            FunctionKind::Receive => None,

            FunctionKind::Modifier => None,
        }
    }
}

impl NodeIdentity for FunctionMutability {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            FunctionMutability::Pure => None,

            FunctionMutability::View => None,

            FunctionMutability::NonPayable => None,

            FunctionMutability::Payable => None,
        }
    }
}

impl NodeIdentity for FunctionVisibility {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            FunctionVisibility::Public => None,

            FunctionVisibility::Private => None,

            FunctionVisibility::Internal => None,

            FunctionVisibility::External => None,
        }
    }
}

impl NodeIdentity for ImportClause {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            ImportClause::PathImport(inner) => inner.node_id(),

            ImportClause::ImportDeconstruction(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for InequalityExpressionOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            InequalityExpressionOperator::GreaterThan(inner) => inner.node_id(),

            InequalityExpressionOperator::GreaterThanEqual(inner) => inner.node_id(),

            InequalityExpressionOperator::LessThan(inner) => inner.node_id(),

            InequalityExpressionOperator::LessThanEqual(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for MultiplicativeExpressionOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            MultiplicativeExpressionOperator::Asterisk(inner) => inner.node_id(),

            MultiplicativeExpressionOperator::Percent(inner) => inner.node_id(),

            MultiplicativeExpressionOperator::Slash(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for NumberUnit {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            NumberUnit::WeiKeyword(inner) => inner.node_id(),

            NumberUnit::GweiKeyword(inner) => inner.node_id(),

            NumberUnit::EtherKeyword(inner) => inner.node_id(),

            NumberUnit::SecondsKeyword(inner) => inner.node_id(),

            NumberUnit::MinutesKeyword(inner) => inner.node_id(),

            NumberUnit::HoursKeyword(inner) => inner.node_id(),

            NumberUnit::DaysKeyword(inner) => inner.node_id(),

            NumberUnit::WeeksKeyword(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for PostfixExpressionOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            PostfixExpressionOperator::MinusMinus(inner) => inner.node_id(),

            PostfixExpressionOperator::PlusPlus(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for Pragma {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            Pragma::VersionPragma(inner) => inner.node_id(),

            Pragma::AbicoderPragma(inner) => inner.node_id(),

            Pragma::ExperimentalPragma(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for PrefixExpressionOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            PrefixExpressionOperator::Bang(inner) => inner.node_id(),

            PrefixExpressionOperator::DeleteKeyword(inner) => inner.node_id(),

            PrefixExpressionOperator::Minus(inner) => inner.node_id(),

            PrefixExpressionOperator::MinusMinus(inner) => inner.node_id(),

            PrefixExpressionOperator::PlusPlus(inner) => inner.node_id(),

            PrefixExpressionOperator::Tilde(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for ShiftExpressionOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            ShiftExpressionOperator::GreaterThanGreaterThan(inner) => inner.node_id(),

            ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(inner) => inner.node_id(),

            ShiftExpressionOperator::LessThanLessThan(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for SourceUnitMember {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            SourceUnitMember::PragmaDirective(inner) => inner.node_id(),

            SourceUnitMember::ImportClause(inner) => inner.node_id(),

            SourceUnitMember::ContractDefinition(inner) => inner.node_id(),

            SourceUnitMember::InterfaceDefinition(inner) => inner.node_id(),

            SourceUnitMember::LibraryDefinition(inner) => inner.node_id(),

            SourceUnitMember::StructDefinition(inner) => inner.node_id(),

            SourceUnitMember::EnumDefinition(inner) => inner.node_id(),

            SourceUnitMember::FunctionDefinition(inner) => inner.node_id(),

            SourceUnitMember::ErrorDefinition(inner) => inner.node_id(),

            SourceUnitMember::UserDefinedValueTypeDefinition(inner) => inner.node_id(),

            SourceUnitMember::UsingDirective(inner) => inner.node_id(),

            SourceUnitMember::EventDefinition(inner) => inner.node_id(),

            SourceUnitMember::ConstantDefinition(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for StateVariableMutability {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            StateVariableMutability::Mutable => None,

            StateVariableMutability::Constant => None,

            StateVariableMutability::Immutable => None,

            StateVariableMutability::Transient => None,
        }
    }
}

impl NodeIdentity for StateVariableVisibility {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            StateVariableVisibility::Public => None,

            StateVariableVisibility::Private => None,

            StateVariableVisibility::Internal => None,
        }
    }
}

impl NodeIdentity for Statement {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            Statement::IfStatement(inner) => inner.node_id(),

            Statement::ForStatement(inner) => inner.node_id(),

            Statement::WhileStatement(inner) => inner.node_id(),

            Statement::DoWhileStatement(inner) => inner.node_id(),

            Statement::ContinueStatement(inner) => inner.node_id(),

            Statement::BreakStatement(inner) => inner.node_id(),

            Statement::ReturnStatement(inner) => inner.node_id(),

            Statement::EmitStatement(inner) => inner.node_id(),

            Statement::TryStatement(inner) => inner.node_id(),

            Statement::RevertStatement(inner) => inner.node_id(),

            Statement::AssemblyStatement(inner) => inner.node_id(),

            Statement::Block(inner) => inner.node_id(),

            Statement::UncheckedBlock(inner) => inner.node_id(),

            Statement::VariableDeclarationStatement(inner) => inner.node_id(),

            Statement::ExpressionStatement(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for StorageLocation {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            StorageLocation::MemoryKeyword(inner) => inner.node_id(),

            StorageLocation::StorageKeyword(inner) => inner.node_id(),

            StorageLocation::CallDataKeyword(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for StringExpression {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            StringExpression::StringLiterals(inner) => inner.node_id(),

            StringExpression::HexStringLiterals(inner) => inner.node_id(),

            StringExpression::UnicodeStringLiterals(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for TypeName {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            TypeName::ArrayTypeName(inner) => inner.node_id(),

            TypeName::FunctionType(inner) => inner.node_id(),

            TypeName::MappingType(inner) => inner.node_id(),

            TypeName::ElementaryType(inner) => inner.node_id(),

            TypeName::IdentifierPath(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for UsingClause {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            UsingClause::IdentifierPath(inner) => inner.node_id(),

            UsingClause::UsingDeconstruction(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for UsingOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            UsingOperator::Ampersand(inner) => inner.node_id(),

            UsingOperator::Asterisk(inner) => inner.node_id(),

            UsingOperator::BangEqual(inner) => inner.node_id(),

            UsingOperator::Bar(inner) => inner.node_id(),

            UsingOperator::Caret(inner) => inner.node_id(),

            UsingOperator::EqualEqual(inner) => inner.node_id(),

            UsingOperator::GreaterThan(inner) => inner.node_id(),

            UsingOperator::GreaterThanEqual(inner) => inner.node_id(),

            UsingOperator::LessThan(inner) => inner.node_id(),

            UsingOperator::LessThanEqual(inner) => inner.node_id(),

            UsingOperator::Minus(inner) => inner.node_id(),

            UsingOperator::Percent(inner) => inner.node_id(),

            UsingOperator::Plus(inner) => inner.node_id(),

            UsingOperator::Slash(inner) => inner.node_id(),

            UsingOperator::Tilde(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for UsingTarget {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            UsingTarget::TypeName(inner) => inner.node_id(),

            UsingTarget::Asterisk(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for VariableDeclarationTarget {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            VariableDeclarationTarget::SingleTypedDeclaration(inner) => inner.node_id(),

            VariableDeclarationTarget::MultiTypedDeclaration(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for VersionExpression {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            VersionExpression::VersionRange(inner) => inner.node_id(),

            VersionExpression::VersionTerm(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for VersionLiteral {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            VersionLiteral::SimpleVersionLiteral(inner) => inner.node_id(),

            VersionLiteral::StringLiteral(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for VersionOperator {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            VersionOperator::PragmaCaret(inner) => inner.node_id(),

            VersionOperator::PragmaTilde(inner) => inner.node_id(),

            VersionOperator::PragmaEqual(inner) => inner.node_id(),

            VersionOperator::PragmaLessThan(inner) => inner.node_id(),

            VersionOperator::PragmaGreaterThan(inner) => inner.node_id(),

            VersionOperator::PragmaLessThanEqual(inner) => inner.node_id(),

            VersionOperator::PragmaGreaterThanEqual(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for YulExpression {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            YulExpression::YulFunctionCallExpression(inner) => inner.node_id(),

            YulExpression::YulLiteral(inner) => inner.node_id(),

            YulExpression::YulPath(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for YulLiteral {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            YulLiteral::TrueKeyword(inner) => inner.node_id(),

            YulLiteral::FalseKeyword(inner) => inner.node_id(),

            YulLiteral::DecimalLiteral(inner) => inner.node_id(),

            YulLiteral::HexLiteral(inner) => inner.node_id(),

            YulLiteral::HexStringLiteral(inner) => inner.node_id(),

            YulLiteral::StringLiteral(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for YulStatement {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            YulStatement::YulBlock(inner) => inner.node_id(),

            YulStatement::YulFunctionDefinition(inner) => inner.node_id(),

            YulStatement::YulIfStatement(inner) => inner.node_id(),

            YulStatement::YulForStatement(inner) => inner.node_id(),

            YulStatement::YulSwitchStatement(inner) => inner.node_id(),

            YulStatement::YulLeaveStatement(inner) => inner.node_id(),

            YulStatement::YulBreakStatement(inner) => inner.node_id(),

            YulStatement::YulContinueStatement(inner) => inner.node_id(),

            YulStatement::YulVariableAssignmentStatement(inner) => inner.node_id(),

            YulStatement::YulVariableDeclarationStatement(inner) => inner.node_id(),

            YulStatement::YulExpression(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for YulSwitchCase {
    fn node_id(&self) -> Option<NodeId> {
        match self {
            YulSwitchCase::YulDefaultCase(inner) => inner.node_id(),

            YulSwitchCase::YulValueCase(inner) => inner.node_id(),
        }
    }
}

impl NodeIdentity for ABIEncoderV2KeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AbicoderV1KeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AbicoderV2KeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AmpersandStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AmpersandEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AsteriskStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for AsteriskEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BangStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BangEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BarStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BarEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BoolKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for BytesKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for CallDataKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for CaretStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for CaretEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for DaysKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for DecimalLiteralStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for DeleteKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for EqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for EqualEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for EtherKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for FalseKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for FixedKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for GreaterThanStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for GreaterThanEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for GreaterThanGreaterThanStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for GreaterThanGreaterThanEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for GreaterThanGreaterThanGreaterThanStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for GreaterThanGreaterThanGreaterThanEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for GweiKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for HexLiteralStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for HexStringLiteralStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for HoursKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for IdentifierStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for IntKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for LessThanStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for LessThanEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for LessThanLessThanStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for LessThanLessThanEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MemoryKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MinusStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MinusEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MinusMinusStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for MinutesKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PayableKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PercentStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PercentEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PlusStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PlusEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PlusPlusStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PragmaCaretStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PragmaEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PragmaGreaterThanStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PragmaGreaterThanEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PragmaLessThanStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PragmaLessThanEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for PragmaTildeStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for SMTCheckerKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for SecondsKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for SemicolonStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for SlashStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for SlashEqualStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for StorageKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for StringKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for StringLiteralStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for SuperKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for ThisKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for TildeStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for TrueKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for UfixedKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for UintKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for UnicodeStringLiteralStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for VersionSpecifierStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for WeeksKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl NodeIdentity for WeiKeywordStruct {
    fn node_id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}
