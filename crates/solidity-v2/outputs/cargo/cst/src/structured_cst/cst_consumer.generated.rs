// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]

use std::ops::Range;
use std::rc::Rc;

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::parser::consumer::ParserConsumer;
use slang_solidity_v2_parser::parser::parser_helpers;
use slang_solidity_v2_parser::{Parser, ParserError};

use crate::structured_cst::nodes;

/// A [`ParserConsumer`] that produces the structured CST.
pub struct CstConsumer;

/// Convenience function to parse Solidity source and produce a structured CST.
pub fn parse(input: &str, version: LanguageVersion) -> Result<nodes::SourceUnit, ParserError> {
    Parser::parse_with_consumer(input, version, &CstConsumer)
}

impl ParserConsumer for CstConsumer {
    // ========================
    // === Associated Types ===
    // ========================

    // Terminals
    type ABIEncoderV2Keyword = nodes::ABIEncoderV2Keyword;
    type AbicoderKeyword = nodes::AbicoderKeyword;
    type AbicoderV1Keyword = nodes::AbicoderV1Keyword;
    type AbicoderV2Keyword = nodes::AbicoderV2Keyword;
    type AbstractKeyword = nodes::AbstractKeyword;
    type AddressKeyword = nodes::AddressKeyword;
    type AfterKeyword = nodes::AfterKeyword;
    type AliasKeyword = nodes::AliasKeyword;
    type Ampersand = nodes::Ampersand;
    type AmpersandAmpersand = nodes::AmpersandAmpersand;
    type AmpersandEqual = nodes::AmpersandEqual;
    type AnonymousKeyword = nodes::AnonymousKeyword;
    type ApplyKeyword = nodes::ApplyKeyword;
    type AsKeyword = nodes::AsKeyword;
    type AssemblyKeyword = nodes::AssemblyKeyword;
    type Asterisk = nodes::Asterisk;
    type AsteriskAsterisk = nodes::AsteriskAsterisk;
    type AsteriskEqual = nodes::AsteriskEqual;
    type AtKeyword = nodes::AtKeyword;
    type AutoKeyword = nodes::AutoKeyword;
    type Bang = nodes::Bang;
    type BangEqual = nodes::BangEqual;
    type Bar = nodes::Bar;
    type BarBar = nodes::BarBar;
    type BarEqual = nodes::BarEqual;
    type BoolKeyword = nodes::BoolKeyword;
    type BreakKeyword = nodes::BreakKeyword;
    type ByteKeyword = nodes::ByteKeyword;
    type BytesKeyword = nodes::BytesKeyword;
    type CallDataKeyword = nodes::CallDataKeyword;
    type Caret = nodes::Caret;
    type CaretEqual = nodes::CaretEqual;
    type CaseKeyword = nodes::CaseKeyword;
    type CatchKeyword = nodes::CatchKeyword;
    type CloseBrace = nodes::CloseBrace;
    type CloseBracket = nodes::CloseBracket;
    type CloseParen = nodes::CloseParen;
    type Colon = nodes::Colon;
    type Comma = nodes::Comma;
    type ConstantKeyword = nodes::ConstantKeyword;
    type ConstructorKeyword = nodes::ConstructorKeyword;
    type ContinueKeyword = nodes::ContinueKeyword;
    type ContractKeyword = nodes::ContractKeyword;
    type CopyOfKeyword = nodes::CopyOfKeyword;
    type DaysKeyword = nodes::DaysKeyword;
    type DecimalLiteral = nodes::DecimalLiteral;
    type DefaultKeyword = nodes::DefaultKeyword;
    type DefineKeyword = nodes::DefineKeyword;
    type DeleteKeyword = nodes::DeleteKeyword;
    type DoKeyword = nodes::DoKeyword;
    type ElseKeyword = nodes::ElseKeyword;
    type EmitKeyword = nodes::EmitKeyword;
    type EnumKeyword = nodes::EnumKeyword;
    type Equal = nodes::Equal;
    type EqualEqual = nodes::EqualEqual;
    type EqualGreaterThan = nodes::EqualGreaterThan;
    type ErrorKeyword = nodes::ErrorKeyword;
    type EtherKeyword = nodes::EtherKeyword;
    type EventKeyword = nodes::EventKeyword;
    type ExperimentalKeyword = nodes::ExperimentalKeyword;
    type ExternalKeyword = nodes::ExternalKeyword;
    type FallbackKeyword = nodes::FallbackKeyword;
    type FalseKeyword = nodes::FalseKeyword;
    type FinalKeyword = nodes::FinalKeyword;
    type FixedKeyword = nodes::FixedKeyword;
    type ForKeyword = nodes::ForKeyword;
    type FromKeyword = nodes::FromKeyword;
    type FunctionKeyword = nodes::FunctionKeyword;
    type GlobalKeyword = nodes::GlobalKeyword;
    type GreaterThan = nodes::GreaterThan;
    type GreaterThanEqual = nodes::GreaterThanEqual;
    type GreaterThanGreaterThan = nodes::GreaterThanGreaterThan;
    type GreaterThanGreaterThanEqual = nodes::GreaterThanGreaterThanEqual;
    type GreaterThanGreaterThanGreaterThan = nodes::GreaterThanGreaterThanGreaterThan;
    type GreaterThanGreaterThanGreaterThanEqual = nodes::GreaterThanGreaterThanGreaterThanEqual;
    type GweiKeyword = nodes::GweiKeyword;
    type HexKeyword = nodes::HexKeyword;
    type HexLiteral = nodes::HexLiteral;
    type HexStringLiteral = nodes::HexStringLiteral;
    type HoursKeyword = nodes::HoursKeyword;
    type Identifier = nodes::Identifier;
    type IfKeyword = nodes::IfKeyword;
    type ImmutableKeyword = nodes::ImmutableKeyword;
    type ImplementsKeyword = nodes::ImplementsKeyword;
    type ImportKeyword = nodes::ImportKeyword;
    type InKeyword = nodes::InKeyword;
    type IndexedKeyword = nodes::IndexedKeyword;
    type InlineKeyword = nodes::InlineKeyword;
    type IntKeyword = nodes::IntKeyword;
    type InterfaceKeyword = nodes::InterfaceKeyword;
    type InternalKeyword = nodes::InternalKeyword;
    type IsKeyword = nodes::IsKeyword;
    type LayoutKeyword = nodes::LayoutKeyword;
    type LessThan = nodes::LessThan;
    type LessThanEqual = nodes::LessThanEqual;
    type LessThanLessThan = nodes::LessThanLessThan;
    type LessThanLessThanEqual = nodes::LessThanLessThanEqual;
    type LetKeyword = nodes::LetKeyword;
    type LibraryKeyword = nodes::LibraryKeyword;
    type MacroKeyword = nodes::MacroKeyword;
    type MappingKeyword = nodes::MappingKeyword;
    type MatchKeyword = nodes::MatchKeyword;
    type MemoryKeyword = nodes::MemoryKeyword;
    type Minus = nodes::Minus;
    type MinusEqual = nodes::MinusEqual;
    type MinusMinus = nodes::MinusMinus;
    type MinutesKeyword = nodes::MinutesKeyword;
    type ModifierKeyword = nodes::ModifierKeyword;
    type MutableKeyword = nodes::MutableKeyword;
    type NewKeyword = nodes::NewKeyword;
    type NullKeyword = nodes::NullKeyword;
    type OfKeyword = nodes::OfKeyword;
    type OpenBrace = nodes::OpenBrace;
    type OpenBracket = nodes::OpenBracket;
    type OpenParen = nodes::OpenParen;
    type OverrideKeyword = nodes::OverrideKeyword;
    type PartialKeyword = nodes::PartialKeyword;
    type PayableKeyword = nodes::PayableKeyword;
    type Percent = nodes::Percent;
    type PercentEqual = nodes::PercentEqual;
    type Period = nodes::Period;
    type Plus = nodes::Plus;
    type PlusEqual = nodes::PlusEqual;
    type PlusPlus = nodes::PlusPlus;
    type PragmaBarBar = nodes::PragmaBarBar;
    type PragmaCaret = nodes::PragmaCaret;
    type PragmaEqual = nodes::PragmaEqual;
    type PragmaGreaterThan = nodes::PragmaGreaterThan;
    type PragmaGreaterThanEqual = nodes::PragmaGreaterThanEqual;
    type PragmaKeyword = nodes::PragmaKeyword;
    type PragmaLessThan = nodes::PragmaLessThan;
    type PragmaLessThanEqual = nodes::PragmaLessThanEqual;
    type PragmaMinus = nodes::PragmaMinus;
    type PragmaPeriod = nodes::PragmaPeriod;
    type PragmaSemicolon = nodes::PragmaSemicolon;
    type PragmaStringLiteral = nodes::PragmaStringLiteral;
    type PragmaTilde = nodes::PragmaTilde;
    type PrivateKeyword = nodes::PrivateKeyword;
    type PromiseKeyword = nodes::PromiseKeyword;
    type PublicKeyword = nodes::PublicKeyword;
    type PureKeyword = nodes::PureKeyword;
    type QuestionMark = nodes::QuestionMark;
    type ReceiveKeyword = nodes::ReceiveKeyword;
    type ReferenceKeyword = nodes::ReferenceKeyword;
    type RelocatableKeyword = nodes::RelocatableKeyword;
    type ReturnKeyword = nodes::ReturnKeyword;
    type ReturnsKeyword = nodes::ReturnsKeyword;
    type RevertKeyword = nodes::RevertKeyword;
    type SMTCheckerKeyword = nodes::SMTCheckerKeyword;
    type SealedKeyword = nodes::SealedKeyword;
    type SecondsKeyword = nodes::SecondsKeyword;
    type Semicolon = nodes::Semicolon;
    type SizeOfKeyword = nodes::SizeOfKeyword;
    type Slash = nodes::Slash;
    type SlashEqual = nodes::SlashEqual;
    type SolidityKeyword = nodes::SolidityKeyword;
    type StaticKeyword = nodes::StaticKeyword;
    type StorageKeyword = nodes::StorageKeyword;
    type StringKeyword = nodes::StringKeyword;
    type StringLiteral = nodes::StringLiteral;
    type StructKeyword = nodes::StructKeyword;
    type SuperKeyword = nodes::SuperKeyword;
    type SupportsKeyword = nodes::SupportsKeyword;
    type SwitchKeyword = nodes::SwitchKeyword;
    type ThisKeyword = nodes::ThisKeyword;
    type ThrowKeyword = nodes::ThrowKeyword;
    type Tilde = nodes::Tilde;
    type TransientKeyword = nodes::TransientKeyword;
    type TrueKeyword = nodes::TrueKeyword;
    type TryKeyword = nodes::TryKeyword;
    type TypeDefKeyword = nodes::TypeDefKeyword;
    type TypeKeyword = nodes::TypeKeyword;
    type TypeOfKeyword = nodes::TypeOfKeyword;
    type UfixedKeyword = nodes::UfixedKeyword;
    type UintKeyword = nodes::UintKeyword;
    type UncheckedKeyword = nodes::UncheckedKeyword;
    type UnicodeStringLiteral = nodes::UnicodeStringLiteral;
    type UsingKeyword = nodes::UsingKeyword;
    type VarKeyword = nodes::VarKeyword;
    type VersionSpecifier = nodes::VersionSpecifier;
    type ViewKeyword = nodes::ViewKeyword;
    type VirtualKeyword = nodes::VirtualKeyword;
    type WeeksKeyword = nodes::WeeksKeyword;
    type WeiKeyword = nodes::WeiKeyword;
    type WhileKeyword = nodes::WhileKeyword;
    type YearsKeyword = nodes::YearsKeyword;
    type YulBreakKeyword = nodes::YulBreakKeyword;
    type YulCaseKeyword = nodes::YulCaseKeyword;
    type YulCloseBrace = nodes::YulCloseBrace;
    type YulCloseParen = nodes::YulCloseParen;
    type YulColonEqual = nodes::YulColonEqual;
    type YulComma = nodes::YulComma;
    type YulContinueKeyword = nodes::YulContinueKeyword;
    type YulDecimalLiteral = nodes::YulDecimalLiteral;
    type YulDefaultKeyword = nodes::YulDefaultKeyword;
    type YulFalseKeyword = nodes::YulFalseKeyword;
    type YulForKeyword = nodes::YulForKeyword;
    type YulFunctionKeyword = nodes::YulFunctionKeyword;
    type YulHexKeyword = nodes::YulHexKeyword;
    type YulHexLiteral = nodes::YulHexLiteral;
    type YulHexStringLiteral = nodes::YulHexStringLiteral;
    type YulIdentifier = nodes::YulIdentifier;
    type YulIfKeyword = nodes::YulIfKeyword;
    type YulLeaveKeyword = nodes::YulLeaveKeyword;
    type YulLetKeyword = nodes::YulLetKeyword;
    type YulMinusGreaterThan = nodes::YulMinusGreaterThan;
    type YulOpenBrace = nodes::YulOpenBrace;
    type YulOpenParen = nodes::YulOpenParen;
    type YulPeriod = nodes::YulPeriod;
    type YulStringLiteral = nodes::YulStringLiteral;
    type YulSuperKeyword = nodes::YulSuperKeyword;
    type YulSwitchKeyword = nodes::YulSwitchKeyword;
    type YulThisKeyword = nodes::YulThisKeyword;
    type YulTrueKeyword = nodes::YulTrueKeyword;

    // Sequences
    type AbicoderPragma = nodes::AbicoderPragma;
    type AdditiveExpression = nodes::AdditiveExpression;
    type AddressType = nodes::AddressType;
    type AndExpression = nodes::AndExpression;
    type ArrayExpression = nodes::ArrayExpression;
    type ArrayTypeName = nodes::ArrayTypeName;
    type AssemblyStatement = nodes::AssemblyStatement;
    type AssignmentExpression = nodes::AssignmentExpression;
    type BitwiseAndExpression = nodes::BitwiseAndExpression;
    type BitwiseOrExpression = nodes::BitwiseOrExpression;
    type BitwiseXorExpression = nodes::BitwiseXorExpression;
    type Block = nodes::Block;
    type BreakStatement = nodes::BreakStatement;
    type CallOptionsExpression = nodes::CallOptionsExpression;
    type CatchClause = nodes::CatchClause;
    type CatchClauseError = nodes::CatchClauseError;
    type ConditionalExpression = nodes::ConditionalExpression;
    type ConstantDefinition = nodes::ConstantDefinition;
    type ConstructorDefinition = nodes::ConstructorDefinition;
    type ContinueStatement = nodes::ContinueStatement;
    type ContractDefinition = nodes::ContractDefinition;
    type DecimalNumberExpression = nodes::DecimalNumberExpression;
    type DoWhileStatement = nodes::DoWhileStatement;
    type ElseBranch = nodes::ElseBranch;
    type EmitStatement = nodes::EmitStatement;
    type EnumDefinition = nodes::EnumDefinition;
    type EqualityExpression = nodes::EqualityExpression;
    type ErrorDefinition = nodes::ErrorDefinition;
    type ErrorParameter = nodes::ErrorParameter;
    type ErrorParametersDeclaration = nodes::ErrorParametersDeclaration;
    type EventDefinition = nodes::EventDefinition;
    type EventParameter = nodes::EventParameter;
    type EventParametersDeclaration = nodes::EventParametersDeclaration;
    type ExperimentalPragma = nodes::ExperimentalPragma;
    type ExponentiationExpression = nodes::ExponentiationExpression;
    type ExpressionStatement = nodes::ExpressionStatement;
    type FallbackFunctionDefinition = nodes::FallbackFunctionDefinition;
    type ForStatement = nodes::ForStatement;
    type FunctionCallExpression = nodes::FunctionCallExpression;
    type FunctionDefinition = nodes::FunctionDefinition;
    type FunctionType = nodes::FunctionType;
    type HexNumberExpression = nodes::HexNumberExpression;
    type IfStatement = nodes::IfStatement;
    type ImportAlias = nodes::ImportAlias;
    type ImportDeconstruction = nodes::ImportDeconstruction;
    type ImportDeconstructionSymbol = nodes::ImportDeconstructionSymbol;
    type ImportDirective = nodes::ImportDirective;
    type IndexAccessEnd = nodes::IndexAccessEnd;
    type IndexAccessExpression = nodes::IndexAccessExpression;
    type InequalityExpression = nodes::InequalityExpression;
    type InheritanceSpecifier = nodes::InheritanceSpecifier;
    type InheritanceType = nodes::InheritanceType;
    type InterfaceDefinition = nodes::InterfaceDefinition;
    type LibraryDefinition = nodes::LibraryDefinition;
    type MappingKey = nodes::MappingKey;
    type MappingType = nodes::MappingType;
    type MappingValue = nodes::MappingValue;
    type MemberAccessExpression = nodes::MemberAccessExpression;
    type ModifierDefinition = nodes::ModifierDefinition;
    type ModifierInvocation = nodes::ModifierInvocation;
    type MultiTypedDeclaration = nodes::MultiTypedDeclaration;
    type MultiTypedDeclarationElement = nodes::MultiTypedDeclarationElement;
    type MultiplicativeExpression = nodes::MultiplicativeExpression;
    type NamedArgument = nodes::NamedArgument;
    type NamedArgumentGroup = nodes::NamedArgumentGroup;
    type NamedArgumentsDeclaration = nodes::NamedArgumentsDeclaration;
    type NamedImport = nodes::NamedImport;
    type NewExpression = nodes::NewExpression;
    type OrExpression = nodes::OrExpression;
    type OverridePathsDeclaration = nodes::OverridePathsDeclaration;
    type OverrideSpecifier = nodes::OverrideSpecifier;
    type Parameter = nodes::Parameter;
    type ParametersDeclaration = nodes::ParametersDeclaration;
    type PathImport = nodes::PathImport;
    type PositionalArgumentsDeclaration = nodes::PositionalArgumentsDeclaration;
    type PostfixExpression = nodes::PostfixExpression;
    type PragmaDirective = nodes::PragmaDirective;
    type PrefixExpression = nodes::PrefixExpression;
    type ReceiveFunctionDefinition = nodes::ReceiveFunctionDefinition;
    type ReturnStatement = nodes::ReturnStatement;
    type ReturnsDeclaration = nodes::ReturnsDeclaration;
    type RevertStatement = nodes::RevertStatement;
    type ShiftExpression = nodes::ShiftExpression;
    type SingleTypedDeclaration = nodes::SingleTypedDeclaration;
    type SourceUnit = nodes::SourceUnit;
    type StateVariableDefinition = nodes::StateVariableDefinition;
    type StateVariableDefinitionValue = nodes::StateVariableDefinitionValue;
    type StorageLayoutSpecifier = nodes::StorageLayoutSpecifier;
    type StructDefinition = nodes::StructDefinition;
    type StructMember = nodes::StructMember;
    type TryStatement = nodes::TryStatement;
    type TupleExpression = nodes::TupleExpression;
    type TupleValue = nodes::TupleValue;
    type TypeExpression = nodes::TypeExpression;
    type UncheckedBlock = nodes::UncheckedBlock;
    type UserDefinedValueTypeDefinition = nodes::UserDefinedValueTypeDefinition;
    type UsingAlias = nodes::UsingAlias;
    type UsingDeconstruction = nodes::UsingDeconstruction;
    type UsingDeconstructionSymbol = nodes::UsingDeconstructionSymbol;
    type UsingDirective = nodes::UsingDirective;
    type VariableDeclaration = nodes::VariableDeclaration;
    type VariableDeclarationStatement = nodes::VariableDeclarationStatement;
    type VariableDeclarationValue = nodes::VariableDeclarationValue;
    type VersionPragma = nodes::VersionPragma;
    type VersionRange = nodes::VersionRange;
    type VersionTerm = nodes::VersionTerm;
    type WhileStatement = nodes::WhileStatement;
    type YulBlock = nodes::YulBlock;
    type YulBreakStatement = nodes::YulBreakStatement;
    type YulContinueStatement = nodes::YulContinueStatement;
    type YulDefaultCase = nodes::YulDefaultCase;
    type YulFlagsDeclaration = nodes::YulFlagsDeclaration;
    type YulForStatement = nodes::YulForStatement;
    type YulFunctionCallExpression = nodes::YulFunctionCallExpression;
    type YulFunctionDefinition = nodes::YulFunctionDefinition;
    type YulIfStatement = nodes::YulIfStatement;
    type YulLeaveStatement = nodes::YulLeaveStatement;
    type YulParametersDeclaration = nodes::YulParametersDeclaration;
    type YulReturnsDeclaration = nodes::YulReturnsDeclaration;
    type YulSwitchStatement = nodes::YulSwitchStatement;
    type YulValueCase = nodes::YulValueCase;
    type YulVariableAssignmentStatement = nodes::YulVariableAssignmentStatement;
    type YulVariableDeclarationStatement = nodes::YulVariableDeclarationStatement;
    type YulVariableDeclarationValue = nodes::YulVariableDeclarationValue;

    // Choices
    type AbicoderVersion = nodes::AbicoderVersion;
    type ArgumentsDeclaration = nodes::ArgumentsDeclaration;
    type ConstructorAttribute = nodes::ConstructorAttribute;
    type ContractMember = nodes::ContractMember;
    type ContractSpecifier = nodes::ContractSpecifier;
    type ElementaryType = nodes::ElementaryType;
    type ExperimentalFeature = nodes::ExperimentalFeature;
    type Expression = nodes::Expression;
    type Expression_AdditiveExpression_Operator = nodes::Expression_AdditiveExpression_Operator;
    type Expression_AssignmentExpression_Operator = nodes::Expression_AssignmentExpression_Operator;
    type Expression_EqualityExpression_Operator = nodes::Expression_EqualityExpression_Operator;
    type Expression_InequalityExpression_Operator = nodes::Expression_InequalityExpression_Operator;
    type Expression_MultiplicativeExpression_Operator =
        nodes::Expression_MultiplicativeExpression_Operator;
    type Expression_PostfixExpression_Operator = nodes::Expression_PostfixExpression_Operator;
    type Expression_PrefixExpression_Operator = nodes::Expression_PrefixExpression_Operator;
    type Expression_ShiftExpression_Operator = nodes::Expression_ShiftExpression_Operator;
    type FallbackFunctionAttribute = nodes::FallbackFunctionAttribute;
    type ForStatementCondition = nodes::ForStatementCondition;
    type ForStatementInitialization = nodes::ForStatementInitialization;
    type FunctionAttribute = nodes::FunctionAttribute;
    type FunctionBody = nodes::FunctionBody;
    type FunctionName = nodes::FunctionName;
    type FunctionTypeAttribute = nodes::FunctionTypeAttribute;
    type IdentifierPathElement = nodes::IdentifierPathElement;
    type ImportClause = nodes::ImportClause;
    type MappingKeyType = nodes::MappingKeyType;
    type ModifierAttribute = nodes::ModifierAttribute;
    type NumberUnit = nodes::NumberUnit;
    type Pragma = nodes::Pragma;
    type ReceiveFunctionAttribute = nodes::ReceiveFunctionAttribute;
    type SourceUnitMember = nodes::SourceUnitMember;
    type StateVariableAttribute = nodes::StateVariableAttribute;
    type Statement = nodes::Statement;
    type StorageLocation = nodes::StorageLocation;
    type StringExpression = nodes::StringExpression;
    type TypeName = nodes::TypeName;
    type UsingClause = nodes::UsingClause;
    type UsingOperator = nodes::UsingOperator;
    type UsingTarget = nodes::UsingTarget;
    type VariableDeclarationTarget = nodes::VariableDeclarationTarget;
    type VersionExpression = nodes::VersionExpression;
    type VersionLiteral = nodes::VersionLiteral;
    type VersionOperator = nodes::VersionOperator;
    type YulExpression = nodes::YulExpression;
    type YulLiteral = nodes::YulLiteral;
    type YulStatement = nodes::YulStatement;
    type YulSwitchCase = nodes::YulSwitchCase;

    // Collections
    type ArrayValues = nodes::ArrayValues;
    type CallOptions = nodes::CallOptions;
    type CatchClauses = nodes::CatchClauses;
    type ConstructorAttributes = nodes::ConstructorAttributes;
    type ContractMembers = nodes::ContractMembers;
    type ContractSpecifiers = nodes::ContractSpecifiers;
    type EnumMembers = nodes::EnumMembers;
    type ErrorParameters = nodes::ErrorParameters;
    type EventParameters = nodes::EventParameters;
    type FallbackFunctionAttributes = nodes::FallbackFunctionAttributes;
    type FunctionAttributes = nodes::FunctionAttributes;
    type FunctionTypeAttributes = nodes::FunctionTypeAttributes;
    type HexStringLiterals = nodes::HexStringLiterals;
    type IdentifierPath = nodes::IdentifierPath;
    type ImportDeconstructionSymbols = nodes::ImportDeconstructionSymbols;
    type InheritanceTypes = nodes::InheritanceTypes;
    type InterfaceMembers = nodes::InterfaceMembers;
    type LibraryMembers = nodes::LibraryMembers;
    type ModifierAttributes = nodes::ModifierAttributes;
    type MultiTypedDeclarationElements = nodes::MultiTypedDeclarationElements;
    type NamedArguments = nodes::NamedArguments;
    type OverridePaths = nodes::OverridePaths;
    type Parameters = nodes::Parameters;
    type PositionalArguments = nodes::PositionalArguments;
    type ReceiveFunctionAttributes = nodes::ReceiveFunctionAttributes;
    type SimpleVersionLiteral = nodes::SimpleVersionLiteral;
    type SourceUnitMembers = nodes::SourceUnitMembers;
    type StateVariableAttributes = nodes::StateVariableAttributes;
    type Statements = nodes::Statements;
    type StringLiterals = nodes::StringLiterals;
    type StructMembers = nodes::StructMembers;
    type TupleValues = nodes::TupleValues;
    type UnicodeStringLiterals = nodes::UnicodeStringLiterals;
    type UsingDeconstructionSymbols = nodes::UsingDeconstructionSymbols;
    type VersionExpressionSet = nodes::VersionExpressionSet;
    type VersionExpressionSets = nodes::VersionExpressionSets;
    type YulArguments = nodes::YulArguments;
    type YulFlags = nodes::YulFlags;
    type YulParameters = nodes::YulParameters;
    type YulPath = nodes::YulPath;
    type YulPaths = nodes::YulPaths;
    type YulStatements = nodes::YulStatements;
    type YulSwitchCases = nodes::YulSwitchCases;
    type YulVariableNames = nodes::YulVariableNames;

    // =======================
    // === Factory Methods ===
    // =======================

    // Terminal factory methods
    fn make_abi_encoder_v2_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::ABIEncoderV2Keyword {
        nodes::new_abi_encoder_v2_keyword(range, source)
    }
    fn make_abicoder_keyword(&self, range: Range<usize>, source: &str) -> nodes::AbicoderKeyword {
        nodes::new_abicoder_keyword(range, source)
    }
    fn make_abicoder_v1_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::AbicoderV1Keyword {
        nodes::new_abicoder_v1_keyword(range, source)
    }
    fn make_abicoder_v2_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::AbicoderV2Keyword {
        nodes::new_abicoder_v2_keyword(range, source)
    }
    fn make_abstract_keyword(&self, range: Range<usize>, source: &str) -> nodes::AbstractKeyword {
        nodes::new_abstract_keyword(range, source)
    }
    fn make_address_keyword(&self, range: Range<usize>, source: &str) -> nodes::AddressKeyword {
        nodes::new_address_keyword(range, source)
    }
    fn make_after_keyword(&self, range: Range<usize>, source: &str) -> nodes::AfterKeyword {
        nodes::new_after_keyword(range, source)
    }
    fn make_alias_keyword(&self, range: Range<usize>, source: &str) -> nodes::AliasKeyword {
        nodes::new_alias_keyword(range, source)
    }
    fn make_ampersand(&self, range: Range<usize>, source: &str) -> nodes::Ampersand {
        nodes::new_ampersand(range, source)
    }
    fn make_ampersand_ampersand(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::AmpersandAmpersand {
        nodes::new_ampersand_ampersand(range, source)
    }
    fn make_ampersand_equal(&self, range: Range<usize>, source: &str) -> nodes::AmpersandEqual {
        nodes::new_ampersand_equal(range, source)
    }
    fn make_anonymous_keyword(&self, range: Range<usize>, source: &str) -> nodes::AnonymousKeyword {
        nodes::new_anonymous_keyword(range, source)
    }
    fn make_apply_keyword(&self, range: Range<usize>, source: &str) -> nodes::ApplyKeyword {
        nodes::new_apply_keyword(range, source)
    }
    fn make_as_keyword(&self, range: Range<usize>, source: &str) -> nodes::AsKeyword {
        nodes::new_as_keyword(range, source)
    }
    fn make_assembly_keyword(&self, range: Range<usize>, source: &str) -> nodes::AssemblyKeyword {
        nodes::new_assembly_keyword(range, source)
    }
    fn make_asterisk(&self, range: Range<usize>, source: &str) -> nodes::Asterisk {
        nodes::new_asterisk(range, source)
    }
    fn make_asterisk_asterisk(&self, range: Range<usize>, source: &str) -> nodes::AsteriskAsterisk {
        nodes::new_asterisk_asterisk(range, source)
    }
    fn make_asterisk_equal(&self, range: Range<usize>, source: &str) -> nodes::AsteriskEqual {
        nodes::new_asterisk_equal(range, source)
    }
    fn make_at_keyword(&self, range: Range<usize>, source: &str) -> nodes::AtKeyword {
        nodes::new_at_keyword(range, source)
    }
    fn make_auto_keyword(&self, range: Range<usize>, source: &str) -> nodes::AutoKeyword {
        nodes::new_auto_keyword(range, source)
    }
    fn make_bang(&self, range: Range<usize>, source: &str) -> nodes::Bang {
        nodes::new_bang(range, source)
    }
    fn make_bang_equal(&self, range: Range<usize>, source: &str) -> nodes::BangEqual {
        nodes::new_bang_equal(range, source)
    }
    fn make_bar(&self, range: Range<usize>, source: &str) -> nodes::Bar {
        nodes::new_bar(range, source)
    }
    fn make_bar_bar(&self, range: Range<usize>, source: &str) -> nodes::BarBar {
        nodes::new_bar_bar(range, source)
    }
    fn make_bar_equal(&self, range: Range<usize>, source: &str) -> nodes::BarEqual {
        nodes::new_bar_equal(range, source)
    }
    fn make_bool_keyword(&self, range: Range<usize>, source: &str) -> nodes::BoolKeyword {
        nodes::new_bool_keyword(range, source)
    }
    fn make_break_keyword(&self, range: Range<usize>, source: &str) -> nodes::BreakKeyword {
        nodes::new_break_keyword(range, source)
    }
    fn make_byte_keyword(&self, range: Range<usize>, source: &str) -> nodes::ByteKeyword {
        nodes::new_byte_keyword(range, source)
    }
    fn make_bytes_keyword(&self, range: Range<usize>, source: &str) -> nodes::BytesKeyword {
        nodes::new_bytes_keyword(range, source)
    }
    fn make_call_data_keyword(&self, range: Range<usize>, source: &str) -> nodes::CallDataKeyword {
        nodes::new_call_data_keyword(range, source)
    }
    fn make_caret(&self, range: Range<usize>, source: &str) -> nodes::Caret {
        nodes::new_caret(range, source)
    }
    fn make_caret_equal(&self, range: Range<usize>, source: &str) -> nodes::CaretEqual {
        nodes::new_caret_equal(range, source)
    }
    fn make_case_keyword(&self, range: Range<usize>, source: &str) -> nodes::CaseKeyword {
        nodes::new_case_keyword(range, source)
    }
    fn make_catch_keyword(&self, range: Range<usize>, source: &str) -> nodes::CatchKeyword {
        nodes::new_catch_keyword(range, source)
    }
    fn make_close_brace(&self, range: Range<usize>, source: &str) -> nodes::CloseBrace {
        nodes::new_close_brace(range, source)
    }
    fn make_close_bracket(&self, range: Range<usize>, source: &str) -> nodes::CloseBracket {
        nodes::new_close_bracket(range, source)
    }
    fn make_close_paren(&self, range: Range<usize>, source: &str) -> nodes::CloseParen {
        nodes::new_close_paren(range, source)
    }
    fn make_colon(&self, range: Range<usize>, source: &str) -> nodes::Colon {
        nodes::new_colon(range, source)
    }
    fn make_comma(&self, range: Range<usize>, source: &str) -> nodes::Comma {
        nodes::new_comma(range, source)
    }
    fn make_constant_keyword(&self, range: Range<usize>, source: &str) -> nodes::ConstantKeyword {
        nodes::new_constant_keyword(range, source)
    }
    fn make_constructor_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::ConstructorKeyword {
        nodes::new_constructor_keyword(range, source)
    }
    fn make_continue_keyword(&self, range: Range<usize>, source: &str) -> nodes::ContinueKeyword {
        nodes::new_continue_keyword(range, source)
    }
    fn make_contract_keyword(&self, range: Range<usize>, source: &str) -> nodes::ContractKeyword {
        nodes::new_contract_keyword(range, source)
    }
    fn make_copy_of_keyword(&self, range: Range<usize>, source: &str) -> nodes::CopyOfKeyword {
        nodes::new_copy_of_keyword(range, source)
    }
    fn make_days_keyword(&self, range: Range<usize>, source: &str) -> nodes::DaysKeyword {
        nodes::new_days_keyword(range, source)
    }
    fn make_decimal_literal(&self, range: Range<usize>, source: &str) -> nodes::DecimalLiteral {
        nodes::new_decimal_literal(range, source)
    }
    fn make_default_keyword(&self, range: Range<usize>, source: &str) -> nodes::DefaultKeyword {
        nodes::new_default_keyword(range, source)
    }
    fn make_define_keyword(&self, range: Range<usize>, source: &str) -> nodes::DefineKeyword {
        nodes::new_define_keyword(range, source)
    }
    fn make_delete_keyword(&self, range: Range<usize>, source: &str) -> nodes::DeleteKeyword {
        nodes::new_delete_keyword(range, source)
    }
    fn make_do_keyword(&self, range: Range<usize>, source: &str) -> nodes::DoKeyword {
        nodes::new_do_keyword(range, source)
    }
    fn make_else_keyword(&self, range: Range<usize>, source: &str) -> nodes::ElseKeyword {
        nodes::new_else_keyword(range, source)
    }
    fn make_emit_keyword(&self, range: Range<usize>, source: &str) -> nodes::EmitKeyword {
        nodes::new_emit_keyword(range, source)
    }
    fn make_enum_keyword(&self, range: Range<usize>, source: &str) -> nodes::EnumKeyword {
        nodes::new_enum_keyword(range, source)
    }
    fn make_equal(&self, range: Range<usize>, source: &str) -> nodes::Equal {
        nodes::new_equal(range, source)
    }
    fn make_equal_equal(&self, range: Range<usize>, source: &str) -> nodes::EqualEqual {
        nodes::new_equal_equal(range, source)
    }
    fn make_equal_greater_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::EqualGreaterThan {
        nodes::new_equal_greater_than(range, source)
    }
    fn make_error_keyword(&self, range: Range<usize>, source: &str) -> nodes::ErrorKeyword {
        nodes::new_error_keyword(range, source)
    }
    fn make_ether_keyword(&self, range: Range<usize>, source: &str) -> nodes::EtherKeyword {
        nodes::new_ether_keyword(range, source)
    }
    fn make_event_keyword(&self, range: Range<usize>, source: &str) -> nodes::EventKeyword {
        nodes::new_event_keyword(range, source)
    }
    fn make_experimental_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::ExperimentalKeyword {
        nodes::new_experimental_keyword(range, source)
    }
    fn make_external_keyword(&self, range: Range<usize>, source: &str) -> nodes::ExternalKeyword {
        nodes::new_external_keyword(range, source)
    }
    fn make_fallback_keyword(&self, range: Range<usize>, source: &str) -> nodes::FallbackKeyword {
        nodes::new_fallback_keyword(range, source)
    }
    fn make_false_keyword(&self, range: Range<usize>, source: &str) -> nodes::FalseKeyword {
        nodes::new_false_keyword(range, source)
    }
    fn make_final_keyword(&self, range: Range<usize>, source: &str) -> nodes::FinalKeyword {
        nodes::new_final_keyword(range, source)
    }
    fn make_fixed_keyword(&self, range: Range<usize>, source: &str) -> nodes::FixedKeyword {
        nodes::new_fixed_keyword(range, source)
    }
    fn make_for_keyword(&self, range: Range<usize>, source: &str) -> nodes::ForKeyword {
        nodes::new_for_keyword(range, source)
    }
    fn make_from_keyword(&self, range: Range<usize>, source: &str) -> nodes::FromKeyword {
        nodes::new_from_keyword(range, source)
    }
    fn make_function_keyword(&self, range: Range<usize>, source: &str) -> nodes::FunctionKeyword {
        nodes::new_function_keyword(range, source)
    }
    fn make_global_keyword(&self, range: Range<usize>, source: &str) -> nodes::GlobalKeyword {
        nodes::new_global_keyword(range, source)
    }
    fn make_greater_than(&self, range: Range<usize>, source: &str) -> nodes::GreaterThan {
        nodes::new_greater_than(range, source)
    }
    fn make_greater_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::GreaterThanEqual {
        nodes::new_greater_than_equal(range, source)
    }
    fn make_greater_than_greater_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::GreaterThanGreaterThan {
        nodes::new_greater_than_greater_than(range, source)
    }
    fn make_greater_than_greater_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::GreaterThanGreaterThanEqual {
        nodes::new_greater_than_greater_than_equal(range, source)
    }
    fn make_greater_than_greater_than_greater_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::GreaterThanGreaterThanGreaterThan {
        nodes::new_greater_than_greater_than_greater_than(range, source)
    }
    fn make_greater_than_greater_than_greater_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::GreaterThanGreaterThanGreaterThanEqual {
        nodes::new_greater_than_greater_than_greater_than_equal(range, source)
    }
    fn make_gwei_keyword(&self, range: Range<usize>, source: &str) -> nodes::GweiKeyword {
        nodes::new_gwei_keyword(range, source)
    }
    fn make_hex_keyword(&self, range: Range<usize>, source: &str) -> nodes::HexKeyword {
        nodes::new_hex_keyword(range, source)
    }
    fn make_hex_literal(&self, range: Range<usize>, source: &str) -> nodes::HexLiteral {
        nodes::new_hex_literal(range, source)
    }
    fn make_hex_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::HexStringLiteral {
        nodes::new_hex_string_literal(range, source)
    }
    fn make_hours_keyword(&self, range: Range<usize>, source: &str) -> nodes::HoursKeyword {
        nodes::new_hours_keyword(range, source)
    }
    fn make_identifier(&self, range: Range<usize>, source: &str) -> nodes::Identifier {
        nodes::new_identifier(range, source)
    }
    fn make_if_keyword(&self, range: Range<usize>, source: &str) -> nodes::IfKeyword {
        nodes::new_if_keyword(range, source)
    }
    fn make_immutable_keyword(&self, range: Range<usize>, source: &str) -> nodes::ImmutableKeyword {
        nodes::new_immutable_keyword(range, source)
    }
    fn make_implements_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::ImplementsKeyword {
        nodes::new_implements_keyword(range, source)
    }
    fn make_import_keyword(&self, range: Range<usize>, source: &str) -> nodes::ImportKeyword {
        nodes::new_import_keyword(range, source)
    }
    fn make_in_keyword(&self, range: Range<usize>, source: &str) -> nodes::InKeyword {
        nodes::new_in_keyword(range, source)
    }
    fn make_indexed_keyword(&self, range: Range<usize>, source: &str) -> nodes::IndexedKeyword {
        nodes::new_indexed_keyword(range, source)
    }
    fn make_inline_keyword(&self, range: Range<usize>, source: &str) -> nodes::InlineKeyword {
        nodes::new_inline_keyword(range, source)
    }
    fn make_int_keyword(&self, range: Range<usize>, source: &str) -> nodes::IntKeyword {
        nodes::new_int_keyword(range, source)
    }
    fn make_interface_keyword(&self, range: Range<usize>, source: &str) -> nodes::InterfaceKeyword {
        nodes::new_interface_keyword(range, source)
    }
    fn make_internal_keyword(&self, range: Range<usize>, source: &str) -> nodes::InternalKeyword {
        nodes::new_internal_keyword(range, source)
    }
    fn make_is_keyword(&self, range: Range<usize>, source: &str) -> nodes::IsKeyword {
        nodes::new_is_keyword(range, source)
    }
    fn make_layout_keyword(&self, range: Range<usize>, source: &str) -> nodes::LayoutKeyword {
        nodes::new_layout_keyword(range, source)
    }
    fn make_less_than(&self, range: Range<usize>, source: &str) -> nodes::LessThan {
        nodes::new_less_than(range, source)
    }
    fn make_less_than_equal(&self, range: Range<usize>, source: &str) -> nodes::LessThanEqual {
        nodes::new_less_than_equal(range, source)
    }
    fn make_less_than_less_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::LessThanLessThan {
        nodes::new_less_than_less_than(range, source)
    }
    fn make_less_than_less_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::LessThanLessThanEqual {
        nodes::new_less_than_less_than_equal(range, source)
    }
    fn make_let_keyword(&self, range: Range<usize>, source: &str) -> nodes::LetKeyword {
        nodes::new_let_keyword(range, source)
    }
    fn make_library_keyword(&self, range: Range<usize>, source: &str) -> nodes::LibraryKeyword {
        nodes::new_library_keyword(range, source)
    }
    fn make_macro_keyword(&self, range: Range<usize>, source: &str) -> nodes::MacroKeyword {
        nodes::new_macro_keyword(range, source)
    }
    fn make_mapping_keyword(&self, range: Range<usize>, source: &str) -> nodes::MappingKeyword {
        nodes::new_mapping_keyword(range, source)
    }
    fn make_match_keyword(&self, range: Range<usize>, source: &str) -> nodes::MatchKeyword {
        nodes::new_match_keyword(range, source)
    }
    fn make_memory_keyword(&self, range: Range<usize>, source: &str) -> nodes::MemoryKeyword {
        nodes::new_memory_keyword(range, source)
    }
    fn make_minus(&self, range: Range<usize>, source: &str) -> nodes::Minus {
        nodes::new_minus(range, source)
    }
    fn make_minus_equal(&self, range: Range<usize>, source: &str) -> nodes::MinusEqual {
        nodes::new_minus_equal(range, source)
    }
    fn make_minus_minus(&self, range: Range<usize>, source: &str) -> nodes::MinusMinus {
        nodes::new_minus_minus(range, source)
    }
    fn make_minutes_keyword(&self, range: Range<usize>, source: &str) -> nodes::MinutesKeyword {
        nodes::new_minutes_keyword(range, source)
    }
    fn make_modifier_keyword(&self, range: Range<usize>, source: &str) -> nodes::ModifierKeyword {
        nodes::new_modifier_keyword(range, source)
    }
    fn make_mutable_keyword(&self, range: Range<usize>, source: &str) -> nodes::MutableKeyword {
        nodes::new_mutable_keyword(range, source)
    }
    fn make_new_keyword(&self, range: Range<usize>, source: &str) -> nodes::NewKeyword {
        nodes::new_new_keyword(range, source)
    }
    fn make_null_keyword(&self, range: Range<usize>, source: &str) -> nodes::NullKeyword {
        nodes::new_null_keyword(range, source)
    }
    fn make_of_keyword(&self, range: Range<usize>, source: &str) -> nodes::OfKeyword {
        nodes::new_of_keyword(range, source)
    }
    fn make_open_brace(&self, range: Range<usize>, source: &str) -> nodes::OpenBrace {
        nodes::new_open_brace(range, source)
    }
    fn make_open_bracket(&self, range: Range<usize>, source: &str) -> nodes::OpenBracket {
        nodes::new_open_bracket(range, source)
    }
    fn make_open_paren(&self, range: Range<usize>, source: &str) -> nodes::OpenParen {
        nodes::new_open_paren(range, source)
    }
    fn make_override_keyword(&self, range: Range<usize>, source: &str) -> nodes::OverrideKeyword {
        nodes::new_override_keyword(range, source)
    }
    fn make_partial_keyword(&self, range: Range<usize>, source: &str) -> nodes::PartialKeyword {
        nodes::new_partial_keyword(range, source)
    }
    fn make_payable_keyword(&self, range: Range<usize>, source: &str) -> nodes::PayableKeyword {
        nodes::new_payable_keyword(range, source)
    }
    fn make_percent(&self, range: Range<usize>, source: &str) -> nodes::Percent {
        nodes::new_percent(range, source)
    }
    fn make_percent_equal(&self, range: Range<usize>, source: &str) -> nodes::PercentEqual {
        nodes::new_percent_equal(range, source)
    }
    fn make_period(&self, range: Range<usize>, source: &str) -> nodes::Period {
        nodes::new_period(range, source)
    }
    fn make_plus(&self, range: Range<usize>, source: &str) -> nodes::Plus {
        nodes::new_plus(range, source)
    }
    fn make_plus_equal(&self, range: Range<usize>, source: &str) -> nodes::PlusEqual {
        nodes::new_plus_equal(range, source)
    }
    fn make_plus_plus(&self, range: Range<usize>, source: &str) -> nodes::PlusPlus {
        nodes::new_plus_plus(range, source)
    }
    fn make_pragma_bar_bar(&self, range: Range<usize>, source: &str) -> nodes::PragmaBarBar {
        nodes::new_pragma_bar_bar(range, source)
    }
    fn make_pragma_caret(&self, range: Range<usize>, source: &str) -> nodes::PragmaCaret {
        nodes::new_pragma_caret(range, source)
    }
    fn make_pragma_equal(&self, range: Range<usize>, source: &str) -> nodes::PragmaEqual {
        nodes::new_pragma_equal(range, source)
    }
    fn make_pragma_greater_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::PragmaGreaterThan {
        nodes::new_pragma_greater_than(range, source)
    }
    fn make_pragma_greater_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::PragmaGreaterThanEqual {
        nodes::new_pragma_greater_than_equal(range, source)
    }
    fn make_pragma_keyword(&self, range: Range<usize>, source: &str) -> nodes::PragmaKeyword {
        nodes::new_pragma_keyword(range, source)
    }
    fn make_pragma_less_than(&self, range: Range<usize>, source: &str) -> nodes::PragmaLessThan {
        nodes::new_pragma_less_than(range, source)
    }
    fn make_pragma_less_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::PragmaLessThanEqual {
        nodes::new_pragma_less_than_equal(range, source)
    }
    fn make_pragma_minus(&self, range: Range<usize>, source: &str) -> nodes::PragmaMinus {
        nodes::new_pragma_minus(range, source)
    }
    fn make_pragma_period(&self, range: Range<usize>, source: &str) -> nodes::PragmaPeriod {
        nodes::new_pragma_period(range, source)
    }
    fn make_pragma_semicolon(&self, range: Range<usize>, source: &str) -> nodes::PragmaSemicolon {
        nodes::new_pragma_semicolon(range, source)
    }
    fn make_pragma_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::PragmaStringLiteral {
        nodes::new_pragma_string_literal(range, source)
    }
    fn make_pragma_tilde(&self, range: Range<usize>, source: &str) -> nodes::PragmaTilde {
        nodes::new_pragma_tilde(range, source)
    }
    fn make_private_keyword(&self, range: Range<usize>, source: &str) -> nodes::PrivateKeyword {
        nodes::new_private_keyword(range, source)
    }
    fn make_promise_keyword(&self, range: Range<usize>, source: &str) -> nodes::PromiseKeyword {
        nodes::new_promise_keyword(range, source)
    }
    fn make_public_keyword(&self, range: Range<usize>, source: &str) -> nodes::PublicKeyword {
        nodes::new_public_keyword(range, source)
    }
    fn make_pure_keyword(&self, range: Range<usize>, source: &str) -> nodes::PureKeyword {
        nodes::new_pure_keyword(range, source)
    }
    fn make_question_mark(&self, range: Range<usize>, source: &str) -> nodes::QuestionMark {
        nodes::new_question_mark(range, source)
    }
    fn make_receive_keyword(&self, range: Range<usize>, source: &str) -> nodes::ReceiveKeyword {
        nodes::new_receive_keyword(range, source)
    }
    fn make_reference_keyword(&self, range: Range<usize>, source: &str) -> nodes::ReferenceKeyword {
        nodes::new_reference_keyword(range, source)
    }
    fn make_relocatable_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::RelocatableKeyword {
        nodes::new_relocatable_keyword(range, source)
    }
    fn make_return_keyword(&self, range: Range<usize>, source: &str) -> nodes::ReturnKeyword {
        nodes::new_return_keyword(range, source)
    }
    fn make_returns_keyword(&self, range: Range<usize>, source: &str) -> nodes::ReturnsKeyword {
        nodes::new_returns_keyword(range, source)
    }
    fn make_revert_keyword(&self, range: Range<usize>, source: &str) -> nodes::RevertKeyword {
        nodes::new_revert_keyword(range, source)
    }
    fn make_smt_checker_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::SMTCheckerKeyword {
        nodes::new_smt_checker_keyword(range, source)
    }
    fn make_sealed_keyword(&self, range: Range<usize>, source: &str) -> nodes::SealedKeyword {
        nodes::new_sealed_keyword(range, source)
    }
    fn make_seconds_keyword(&self, range: Range<usize>, source: &str) -> nodes::SecondsKeyword {
        nodes::new_seconds_keyword(range, source)
    }
    fn make_semicolon(&self, range: Range<usize>, source: &str) -> nodes::Semicolon {
        nodes::new_semicolon(range, source)
    }
    fn make_size_of_keyword(&self, range: Range<usize>, source: &str) -> nodes::SizeOfKeyword {
        nodes::new_size_of_keyword(range, source)
    }
    fn make_slash(&self, range: Range<usize>, source: &str) -> nodes::Slash {
        nodes::new_slash(range, source)
    }
    fn make_slash_equal(&self, range: Range<usize>, source: &str) -> nodes::SlashEqual {
        nodes::new_slash_equal(range, source)
    }
    fn make_solidity_keyword(&self, range: Range<usize>, source: &str) -> nodes::SolidityKeyword {
        nodes::new_solidity_keyword(range, source)
    }
    fn make_static_keyword(&self, range: Range<usize>, source: &str) -> nodes::StaticKeyword {
        nodes::new_static_keyword(range, source)
    }
    fn make_storage_keyword(&self, range: Range<usize>, source: &str) -> nodes::StorageKeyword {
        nodes::new_storage_keyword(range, source)
    }
    fn make_string_keyword(&self, range: Range<usize>, source: &str) -> nodes::StringKeyword {
        nodes::new_string_keyword(range, source)
    }
    fn make_string_literal(&self, range: Range<usize>, source: &str) -> nodes::StringLiteral {
        nodes::new_string_literal(range, source)
    }
    fn make_struct_keyword(&self, range: Range<usize>, source: &str) -> nodes::StructKeyword {
        nodes::new_struct_keyword(range, source)
    }
    fn make_super_keyword(&self, range: Range<usize>, source: &str) -> nodes::SuperKeyword {
        nodes::new_super_keyword(range, source)
    }
    fn make_supports_keyword(&self, range: Range<usize>, source: &str) -> nodes::SupportsKeyword {
        nodes::new_supports_keyword(range, source)
    }
    fn make_switch_keyword(&self, range: Range<usize>, source: &str) -> nodes::SwitchKeyword {
        nodes::new_switch_keyword(range, source)
    }
    fn make_this_keyword(&self, range: Range<usize>, source: &str) -> nodes::ThisKeyword {
        nodes::new_this_keyword(range, source)
    }
    fn make_throw_keyword(&self, range: Range<usize>, source: &str) -> nodes::ThrowKeyword {
        nodes::new_throw_keyword(range, source)
    }
    fn make_tilde(&self, range: Range<usize>, source: &str) -> nodes::Tilde {
        nodes::new_tilde(range, source)
    }
    fn make_transient_keyword(&self, range: Range<usize>, source: &str) -> nodes::TransientKeyword {
        nodes::new_transient_keyword(range, source)
    }
    fn make_true_keyword(&self, range: Range<usize>, source: &str) -> nodes::TrueKeyword {
        nodes::new_true_keyword(range, source)
    }
    fn make_try_keyword(&self, range: Range<usize>, source: &str) -> nodes::TryKeyword {
        nodes::new_try_keyword(range, source)
    }
    fn make_type_def_keyword(&self, range: Range<usize>, source: &str) -> nodes::TypeDefKeyword {
        nodes::new_type_def_keyword(range, source)
    }
    fn make_type_keyword(&self, range: Range<usize>, source: &str) -> nodes::TypeKeyword {
        nodes::new_type_keyword(range, source)
    }
    fn make_type_of_keyword(&self, range: Range<usize>, source: &str) -> nodes::TypeOfKeyword {
        nodes::new_type_of_keyword(range, source)
    }
    fn make_ufixed_keyword(&self, range: Range<usize>, source: &str) -> nodes::UfixedKeyword {
        nodes::new_ufixed_keyword(range, source)
    }
    fn make_uint_keyword(&self, range: Range<usize>, source: &str) -> nodes::UintKeyword {
        nodes::new_uint_keyword(range, source)
    }
    fn make_unchecked_keyword(&self, range: Range<usize>, source: &str) -> nodes::UncheckedKeyword {
        nodes::new_unchecked_keyword(range, source)
    }
    fn make_unicode_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::UnicodeStringLiteral {
        nodes::new_unicode_string_literal(range, source)
    }
    fn make_using_keyword(&self, range: Range<usize>, source: &str) -> nodes::UsingKeyword {
        nodes::new_using_keyword(range, source)
    }
    fn make_var_keyword(&self, range: Range<usize>, source: &str) -> nodes::VarKeyword {
        nodes::new_var_keyword(range, source)
    }
    fn make_version_specifier(&self, range: Range<usize>, source: &str) -> nodes::VersionSpecifier {
        nodes::new_version_specifier(range, source)
    }
    fn make_view_keyword(&self, range: Range<usize>, source: &str) -> nodes::ViewKeyword {
        nodes::new_view_keyword(range, source)
    }
    fn make_virtual_keyword(&self, range: Range<usize>, source: &str) -> nodes::VirtualKeyword {
        nodes::new_virtual_keyword(range, source)
    }
    fn make_weeks_keyword(&self, range: Range<usize>, source: &str) -> nodes::WeeksKeyword {
        nodes::new_weeks_keyword(range, source)
    }
    fn make_wei_keyword(&self, range: Range<usize>, source: &str) -> nodes::WeiKeyword {
        nodes::new_wei_keyword(range, source)
    }
    fn make_while_keyword(&self, range: Range<usize>, source: &str) -> nodes::WhileKeyword {
        nodes::new_while_keyword(range, source)
    }
    fn make_years_keyword(&self, range: Range<usize>, source: &str) -> nodes::YearsKeyword {
        nodes::new_years_keyword(range, source)
    }
    fn make_yul_break_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulBreakKeyword {
        nodes::new_yul_break_keyword(range, source)
    }
    fn make_yul_case_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulCaseKeyword {
        nodes::new_yul_case_keyword(range, source)
    }
    fn make_yul_close_brace(&self, range: Range<usize>, source: &str) -> nodes::YulCloseBrace {
        nodes::new_yul_close_brace(range, source)
    }
    fn make_yul_close_paren(&self, range: Range<usize>, source: &str) -> nodes::YulCloseParen {
        nodes::new_yul_close_paren(range, source)
    }
    fn make_yul_colon_equal(&self, range: Range<usize>, source: &str) -> nodes::YulColonEqual {
        nodes::new_yul_colon_equal(range, source)
    }
    fn make_yul_comma(&self, range: Range<usize>, source: &str) -> nodes::YulComma {
        nodes::new_yul_comma(range, source)
    }
    fn make_yul_continue_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::YulContinueKeyword {
        nodes::new_yul_continue_keyword(range, source)
    }
    fn make_yul_decimal_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::YulDecimalLiteral {
        nodes::new_yul_decimal_literal(range, source)
    }
    fn make_yul_default_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::YulDefaultKeyword {
        nodes::new_yul_default_keyword(range, source)
    }
    fn make_yul_false_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulFalseKeyword {
        nodes::new_yul_false_keyword(range, source)
    }
    fn make_yul_for_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulForKeyword {
        nodes::new_yul_for_keyword(range, source)
    }
    fn make_yul_function_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::YulFunctionKeyword {
        nodes::new_yul_function_keyword(range, source)
    }
    fn make_yul_hex_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulHexKeyword {
        nodes::new_yul_hex_keyword(range, source)
    }
    fn make_yul_hex_literal(&self, range: Range<usize>, source: &str) -> nodes::YulHexLiteral {
        nodes::new_yul_hex_literal(range, source)
    }
    fn make_yul_hex_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::YulHexStringLiteral {
        nodes::new_yul_hex_string_literal(range, source)
    }
    fn make_yul_identifier(&self, range: Range<usize>, source: &str) -> nodes::YulIdentifier {
        nodes::new_yul_identifier(range, source)
    }
    fn make_yul_if_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulIfKeyword {
        nodes::new_yul_if_keyword(range, source)
    }
    fn make_yul_leave_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulLeaveKeyword {
        nodes::new_yul_leave_keyword(range, source)
    }
    fn make_yul_let_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulLetKeyword {
        nodes::new_yul_let_keyword(range, source)
    }
    fn make_yul_minus_greater_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::YulMinusGreaterThan {
        nodes::new_yul_minus_greater_than(range, source)
    }
    fn make_yul_open_brace(&self, range: Range<usize>, source: &str) -> nodes::YulOpenBrace {
        nodes::new_yul_open_brace(range, source)
    }
    fn make_yul_open_paren(&self, range: Range<usize>, source: &str) -> nodes::YulOpenParen {
        nodes::new_yul_open_paren(range, source)
    }
    fn make_yul_period(&self, range: Range<usize>, source: &str) -> nodes::YulPeriod {
        nodes::new_yul_period(range, source)
    }
    fn make_yul_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::YulStringLiteral {
        nodes::new_yul_string_literal(range, source)
    }
    fn make_yul_super_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulSuperKeyword {
        nodes::new_yul_super_keyword(range, source)
    }
    fn make_yul_switch_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> nodes::YulSwitchKeyword {
        nodes::new_yul_switch_keyword(range, source)
    }
    fn make_yul_this_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulThisKeyword {
        nodes::new_yul_this_keyword(range, source)
    }
    fn make_yul_true_keyword(&self, range: Range<usize>, source: &str) -> nodes::YulTrueKeyword {
        nodes::new_yul_true_keyword(range, source)
    }

    // Sequence factory methods
    fn make_abicoder_pragma(
        &self,
        abicoder_keyword: nodes::AbicoderKeyword,
        version: nodes::AbicoderVersion,
    ) -> nodes::AbicoderPragma {
        nodes::new_abicoder_pragma(abicoder_keyword, version)
    }
    fn make_additive_expression(
        &self,
        left_operand: nodes::Expression,
        expression_additive_expression_operator: nodes::Expression_AdditiveExpression_Operator,
        right_operand: nodes::Expression,
    ) -> nodes::AdditiveExpression {
        nodes::new_additive_expression(
            left_operand,
            expression_additive_expression_operator,
            right_operand,
        )
    }
    fn make_address_type(
        &self,
        address_keyword: nodes::AddressKeyword,
        payable_keyword: Option<nodes::PayableKeyword>,
    ) -> nodes::AddressType {
        nodes::new_address_type(address_keyword, payable_keyword)
    }
    fn make_and_expression(
        &self,
        left_operand: nodes::Expression,
        operator: nodes::AmpersandAmpersand,
        right_operand: nodes::Expression,
    ) -> nodes::AndExpression {
        nodes::new_and_expression(left_operand, operator, right_operand)
    }
    fn make_array_expression(
        &self,
        open_bracket: nodes::OpenBracket,
        items: nodes::ArrayValues,
        close_bracket: nodes::CloseBracket,
    ) -> nodes::ArrayExpression {
        nodes::new_array_expression(open_bracket, items, close_bracket)
    }
    fn make_array_type_name(
        &self,
        operand: nodes::TypeName,
        open_bracket: nodes::OpenBracket,
        index: Option<nodes::Expression>,
        close_bracket: nodes::CloseBracket,
    ) -> nodes::ArrayTypeName {
        nodes::new_array_type_name(operand, open_bracket, index, close_bracket)
    }
    fn make_assembly_statement(
        &self,
        assembly_keyword: nodes::AssemblyKeyword,
        label: Option<nodes::YulStringLiteral>,
        flags: Option<nodes::YulFlagsDeclaration>,
        body: nodes::YulBlock,
    ) -> nodes::AssemblyStatement {
        nodes::new_assembly_statement(assembly_keyword, label, flags, body)
    }
    fn make_assignment_expression(
        &self,
        left_operand: nodes::Expression,
        expression_assignment_expression_operator: nodes::Expression_AssignmentExpression_Operator,
        right_operand: nodes::Expression,
    ) -> nodes::AssignmentExpression {
        nodes::new_assignment_expression(
            left_operand,
            expression_assignment_expression_operator,
            right_operand,
        )
    }
    fn make_bitwise_and_expression(
        &self,
        left_operand: nodes::Expression,
        operator: nodes::Ampersand,
        right_operand: nodes::Expression,
    ) -> nodes::BitwiseAndExpression {
        nodes::new_bitwise_and_expression(left_operand, operator, right_operand)
    }
    fn make_bitwise_or_expression(
        &self,
        left_operand: nodes::Expression,
        operator: nodes::Bar,
        right_operand: nodes::Expression,
    ) -> nodes::BitwiseOrExpression {
        nodes::new_bitwise_or_expression(left_operand, operator, right_operand)
    }
    fn make_bitwise_xor_expression(
        &self,
        left_operand: nodes::Expression,
        operator: nodes::Caret,
        right_operand: nodes::Expression,
    ) -> nodes::BitwiseXorExpression {
        nodes::new_bitwise_xor_expression(left_operand, operator, right_operand)
    }
    fn make_block(
        &self,
        open_brace: nodes::OpenBrace,
        statements: nodes::Statements,
        close_brace: nodes::CloseBrace,
    ) -> nodes::Block {
        nodes::new_block(open_brace, statements, close_brace)
    }
    fn make_break_statement(
        &self,
        break_keyword: nodes::BreakKeyword,
        semicolon: nodes::Semicolon,
    ) -> nodes::BreakStatement {
        nodes::new_break_statement(break_keyword, semicolon)
    }
    fn make_call_options_expression(
        &self,
        operand: nodes::Expression,
        open_brace: nodes::OpenBrace,
        options: nodes::CallOptions,
        close_brace: nodes::CloseBrace,
    ) -> nodes::CallOptionsExpression {
        nodes::new_call_options_expression(operand, open_brace, options, close_brace)
    }
    fn make_catch_clause(
        &self,
        catch_keyword: nodes::CatchKeyword,
        error: Option<nodes::CatchClauseError>,
        body: nodes::Block,
    ) -> nodes::CatchClause {
        nodes::new_catch_clause(catch_keyword, error, body)
    }
    fn make_catch_clause_error(
        &self,
        name: Option<nodes::Identifier>,
        parameters: nodes::ParametersDeclaration,
    ) -> nodes::CatchClauseError {
        nodes::new_catch_clause_error(name, parameters)
    }
    fn make_conditional_expression(
        &self,
        operand: nodes::Expression,
        question_mark: nodes::QuestionMark,
        true_expression: nodes::Expression,
        colon: nodes::Colon,
        false_expression: nodes::Expression,
    ) -> nodes::ConditionalExpression {
        nodes::new_conditional_expression(
            operand,
            question_mark,
            true_expression,
            colon,
            false_expression,
        )
    }
    fn make_constant_definition(
        &self,
        type_name: nodes::TypeName,
        constant_keyword: nodes::ConstantKeyword,
        name: nodes::Identifier,
        equal: nodes::Equal,
        value: nodes::Expression,
        semicolon: nodes::Semicolon,
    ) -> nodes::ConstantDefinition {
        nodes::new_constant_definition(type_name, constant_keyword, name, equal, value, semicolon)
    }
    fn make_constructor_definition(
        &self,
        constructor_keyword: nodes::ConstructorKeyword,
        parameters: nodes::ParametersDeclaration,
        attributes: nodes::ConstructorAttributes,
        body: nodes::Block,
    ) -> nodes::ConstructorDefinition {
        nodes::new_constructor_definition(constructor_keyword, parameters, attributes, body)
    }
    fn make_continue_statement(
        &self,
        continue_keyword: nodes::ContinueKeyword,
        semicolon: nodes::Semicolon,
    ) -> nodes::ContinueStatement {
        nodes::new_continue_statement(continue_keyword, semicolon)
    }
    fn make_contract_definition(
        &self,
        abstract_keyword: Option<nodes::AbstractKeyword>,
        contract_keyword: nodes::ContractKeyword,
        name: nodes::Identifier,
        specifiers: nodes::ContractSpecifiers,
        open_brace: nodes::OpenBrace,
        members: nodes::ContractMembers,
        close_brace: nodes::CloseBrace,
    ) -> nodes::ContractDefinition {
        nodes::new_contract_definition(
            abstract_keyword,
            contract_keyword,
            name,
            specifiers,
            open_brace,
            members,
            close_brace,
        )
    }
    fn make_decimal_number_expression(
        &self,
        literal: nodes::DecimalLiteral,
        unit: Option<nodes::NumberUnit>,
    ) -> nodes::DecimalNumberExpression {
        nodes::new_decimal_number_expression(literal, unit)
    }
    fn make_do_while_statement(
        &self,
        do_keyword: nodes::DoKeyword,
        body: nodes::Statement,
        while_keyword: nodes::WhileKeyword,
        open_paren: nodes::OpenParen,
        condition: nodes::Expression,
        close_paren: nodes::CloseParen,
        semicolon: nodes::Semicolon,
    ) -> nodes::DoWhileStatement {
        nodes::new_do_while_statement(
            do_keyword,
            body,
            while_keyword,
            open_paren,
            condition,
            close_paren,
            semicolon,
        )
    }
    fn make_else_branch(
        &self,
        else_keyword: nodes::ElseKeyword,
        body: nodes::Statement,
    ) -> nodes::ElseBranch {
        nodes::new_else_branch(else_keyword, body)
    }
    fn make_emit_statement(
        &self,
        emit_keyword: nodes::EmitKeyword,
        event: nodes::IdentifierPath,
        arguments: nodes::ArgumentsDeclaration,
        semicolon: nodes::Semicolon,
    ) -> nodes::EmitStatement {
        nodes::new_emit_statement(emit_keyword, event, arguments, semicolon)
    }
    fn make_enum_definition(
        &self,
        enum_keyword: nodes::EnumKeyword,
        name: nodes::Identifier,
        open_brace: nodes::OpenBrace,
        members: nodes::EnumMembers,
        close_brace: nodes::CloseBrace,
    ) -> nodes::EnumDefinition {
        nodes::new_enum_definition(enum_keyword, name, open_brace, members, close_brace)
    }
    fn make_equality_expression(
        &self,
        left_operand: nodes::Expression,
        expression_equality_expression_operator: nodes::Expression_EqualityExpression_Operator,
        right_operand: nodes::Expression,
    ) -> nodes::EqualityExpression {
        nodes::new_equality_expression(
            left_operand,
            expression_equality_expression_operator,
            right_operand,
        )
    }
    fn make_error_definition(
        &self,
        error_keyword: nodes::ErrorKeyword,
        name: nodes::Identifier,
        members: nodes::ErrorParametersDeclaration,
        semicolon: nodes::Semicolon,
    ) -> nodes::ErrorDefinition {
        nodes::new_error_definition(error_keyword, name, members, semicolon)
    }
    fn make_error_parameter(
        &self,
        type_name: nodes::TypeName,
        name: Option<nodes::Identifier>,
    ) -> nodes::ErrorParameter {
        nodes::new_error_parameter(type_name, name)
    }
    fn make_error_parameters_declaration(
        &self,
        open_paren: nodes::OpenParen,
        parameters: nodes::ErrorParameters,
        close_paren: nodes::CloseParen,
    ) -> nodes::ErrorParametersDeclaration {
        nodes::new_error_parameters_declaration(open_paren, parameters, close_paren)
    }
    fn make_event_definition(
        &self,
        event_keyword: nodes::EventKeyword,
        name: nodes::Identifier,
        parameters: nodes::EventParametersDeclaration,
        anonymous_keyword: Option<nodes::AnonymousKeyword>,
        semicolon: nodes::Semicolon,
    ) -> nodes::EventDefinition {
        nodes::new_event_definition(
            event_keyword,
            name,
            parameters,
            anonymous_keyword,
            semicolon,
        )
    }
    fn make_event_parameter(
        &self,
        type_name: nodes::TypeName,
        indexed_keyword: Option<nodes::IndexedKeyword>,
        name: Option<nodes::Identifier>,
    ) -> nodes::EventParameter {
        nodes::new_event_parameter(type_name, indexed_keyword, name)
    }
    fn make_event_parameters_declaration(
        &self,
        open_paren: nodes::OpenParen,
        parameters: nodes::EventParameters,
        close_paren: nodes::CloseParen,
    ) -> nodes::EventParametersDeclaration {
        nodes::new_event_parameters_declaration(open_paren, parameters, close_paren)
    }
    fn make_experimental_pragma(
        &self,
        experimental_keyword: nodes::ExperimentalKeyword,
        feature: nodes::ExperimentalFeature,
    ) -> nodes::ExperimentalPragma {
        nodes::new_experimental_pragma(experimental_keyword, feature)
    }
    fn make_exponentiation_expression(
        &self,
        left_operand: nodes::Expression,
        operator: nodes::AsteriskAsterisk,
        right_operand: nodes::Expression,
    ) -> nodes::ExponentiationExpression {
        nodes::new_exponentiation_expression(left_operand, operator, right_operand)
    }
    fn make_expression_statement(
        &self,
        expression: nodes::Expression,
        semicolon: nodes::Semicolon,
    ) -> nodes::ExpressionStatement {
        nodes::new_expression_statement(expression, semicolon)
    }
    fn make_fallback_function_definition(
        &self,
        fallback_keyword: nodes::FallbackKeyword,
        parameters: nodes::ParametersDeclaration,
        attributes: nodes::FallbackFunctionAttributes,
        returns: Option<nodes::ReturnsDeclaration>,
        body: nodes::FunctionBody,
    ) -> nodes::FallbackFunctionDefinition {
        nodes::new_fallback_function_definition(
            fallback_keyword,
            parameters,
            attributes,
            returns,
            body,
        )
    }
    fn make_for_statement(
        &self,
        for_keyword: nodes::ForKeyword,
        open_paren: nodes::OpenParen,
        initialization: nodes::ForStatementInitialization,
        condition: nodes::ForStatementCondition,
        iterator: Option<nodes::Expression>,
        close_paren: nodes::CloseParen,
        body: nodes::Statement,
    ) -> nodes::ForStatement {
        nodes::new_for_statement(
            for_keyword,
            open_paren,
            initialization,
            condition,
            iterator,
            close_paren,
            body,
        )
    }
    fn make_function_call_expression(
        &self,
        operand: nodes::Expression,
        arguments: nodes::ArgumentsDeclaration,
    ) -> nodes::FunctionCallExpression {
        nodes::new_function_call_expression(operand, arguments)
    }
    fn make_function_definition(
        &self,
        function_keyword: nodes::FunctionKeyword,
        name: nodes::FunctionName,
        parameters: nodes::ParametersDeclaration,
        attributes: nodes::FunctionAttributes,
        returns: Option<nodes::ReturnsDeclaration>,
        body: nodes::FunctionBody,
    ) -> nodes::FunctionDefinition {
        nodes::new_function_definition(
            function_keyword,
            name,
            parameters,
            attributes,
            returns,
            body,
        )
    }
    fn make_function_type(
        &self,
        function_keyword: nodes::FunctionKeyword,
        parameters: nodes::ParametersDeclaration,
        attributes: nodes::FunctionTypeAttributes,
        returns: Option<nodes::ReturnsDeclaration>,
    ) -> nodes::FunctionType {
        nodes::new_function_type(function_keyword, parameters, attributes, returns)
    }
    fn make_hex_number_expression(&self, literal: nodes::HexLiteral) -> nodes::HexNumberExpression {
        nodes::new_hex_number_expression(literal)
    }
    fn make_if_statement(
        &self,
        if_keyword: nodes::IfKeyword,
        open_paren: nodes::OpenParen,
        condition: nodes::Expression,
        close_paren: nodes::CloseParen,
        body: nodes::Statement,
        else_branch: Option<nodes::ElseBranch>,
    ) -> nodes::IfStatement {
        nodes::new_if_statement(
            if_keyword,
            open_paren,
            condition,
            close_paren,
            body,
            else_branch,
        )
    }
    fn make_import_alias(
        &self,
        as_keyword: nodes::AsKeyword,
        identifier: nodes::Identifier,
    ) -> nodes::ImportAlias {
        nodes::new_import_alias(as_keyword, identifier)
    }
    fn make_import_deconstruction(
        &self,
        open_brace: nodes::OpenBrace,
        symbols: nodes::ImportDeconstructionSymbols,
        close_brace: nodes::CloseBrace,
        from_keyword: nodes::FromKeyword,
        path: nodes::StringLiteral,
    ) -> nodes::ImportDeconstruction {
        nodes::new_import_deconstruction(open_brace, symbols, close_brace, from_keyword, path)
    }
    fn make_import_deconstruction_symbol(
        &self,
        name: nodes::Identifier,
        alias: Option<nodes::ImportAlias>,
    ) -> nodes::ImportDeconstructionSymbol {
        nodes::new_import_deconstruction_symbol(name, alias)
    }
    fn make_import_directive(
        &self,
        import_keyword: nodes::ImportKeyword,
        clause: nodes::ImportClause,
        semicolon: nodes::Semicolon,
    ) -> nodes::ImportDirective {
        nodes::new_import_directive(import_keyword, clause, semicolon)
    }
    fn make_index_access_end(
        &self,
        colon: nodes::Colon,
        end: Option<nodes::Expression>,
    ) -> nodes::IndexAccessEnd {
        nodes::new_index_access_end(colon, end)
    }
    fn make_index_access_expression(
        &self,
        operand: nodes::Expression,
        open_bracket: nodes::OpenBracket,
        start: Option<nodes::Expression>,
        end: Option<nodes::IndexAccessEnd>,
        close_bracket: nodes::CloseBracket,
    ) -> nodes::IndexAccessExpression {
        nodes::new_index_access_expression(operand, open_bracket, start, end, close_bracket)
    }
    fn make_inequality_expression(
        &self,
        left_operand: nodes::Expression,
        expression_inequality_expression_operator: nodes::Expression_InequalityExpression_Operator,
        right_operand: nodes::Expression,
    ) -> nodes::InequalityExpression {
        nodes::new_inequality_expression(
            left_operand,
            expression_inequality_expression_operator,
            right_operand,
        )
    }
    fn make_inheritance_specifier(
        &self,
        is_keyword: nodes::IsKeyword,
        types: nodes::InheritanceTypes,
    ) -> nodes::InheritanceSpecifier {
        nodes::new_inheritance_specifier(is_keyword, types)
    }
    fn make_inheritance_type(
        &self,
        type_name: nodes::IdentifierPath,
        arguments: Option<nodes::ArgumentsDeclaration>,
    ) -> nodes::InheritanceType {
        nodes::new_inheritance_type(type_name, arguments)
    }
    fn make_interface_definition(
        &self,
        interface_keyword: nodes::InterfaceKeyword,
        name: nodes::Identifier,
        inheritance: Option<nodes::InheritanceSpecifier>,
        open_brace: nodes::OpenBrace,
        members: nodes::InterfaceMembers,
        close_brace: nodes::CloseBrace,
    ) -> nodes::InterfaceDefinition {
        nodes::new_interface_definition(
            interface_keyword,
            name,
            inheritance,
            open_brace,
            members,
            close_brace,
        )
    }
    fn make_library_definition(
        &self,
        library_keyword: nodes::LibraryKeyword,
        name: nodes::Identifier,
        open_brace: nodes::OpenBrace,
        members: nodes::LibraryMembers,
        close_brace: nodes::CloseBrace,
    ) -> nodes::LibraryDefinition {
        nodes::new_library_definition(library_keyword, name, open_brace, members, close_brace)
    }
    fn make_mapping_key(
        &self,
        key_type: nodes::MappingKeyType,
        name: Option<nodes::Identifier>,
    ) -> nodes::MappingKey {
        nodes::new_mapping_key(key_type, name)
    }
    fn make_mapping_type(
        &self,
        mapping_keyword: nodes::MappingKeyword,
        open_paren: nodes::OpenParen,
        key_type: nodes::MappingKey,
        equal_greater_than: nodes::EqualGreaterThan,
        value_type: nodes::MappingValue,
        close_paren: nodes::CloseParen,
    ) -> nodes::MappingType {
        nodes::new_mapping_type(
            mapping_keyword,
            open_paren,
            key_type,
            equal_greater_than,
            value_type,
            close_paren,
        )
    }
    fn make_mapping_value(
        &self,
        type_name: nodes::TypeName,
        name: Option<nodes::Identifier>,
    ) -> nodes::MappingValue {
        nodes::new_mapping_value(type_name, name)
    }
    fn make_member_access_expression(
        &self,
        operand: nodes::Expression,
        period: nodes::Period,
        member: nodes::IdentifierPathElement,
    ) -> nodes::MemberAccessExpression {
        nodes::new_member_access_expression(operand, period, member)
    }
    fn make_modifier_definition(
        &self,
        modifier_keyword: nodes::ModifierKeyword,
        name: nodes::Identifier,
        parameters: Option<nodes::ParametersDeclaration>,
        attributes: nodes::ModifierAttributes,
        body: nodes::FunctionBody,
    ) -> nodes::ModifierDefinition {
        nodes::new_modifier_definition(modifier_keyword, name, parameters, attributes, body)
    }
    fn make_modifier_invocation(
        &self,
        name: nodes::IdentifierPath,
        arguments: Option<nodes::ArgumentsDeclaration>,
    ) -> nodes::ModifierInvocation {
        nodes::new_modifier_invocation(name, arguments)
    }
    fn make_multi_typed_declaration(
        &self,
        open_paren: nodes::OpenParen,
        elements: nodes::MultiTypedDeclarationElements,
        close_paren: nodes::CloseParen,
        value: nodes::VariableDeclarationValue,
    ) -> nodes::MultiTypedDeclaration {
        nodes::new_multi_typed_declaration(open_paren, elements, close_paren, value)
    }
    fn make_multi_typed_declaration_element(
        &self,
        member: Option<nodes::VariableDeclaration>,
    ) -> nodes::MultiTypedDeclarationElement {
        nodes::new_multi_typed_declaration_element(member)
    }
    fn make_multiplicative_expression(
        &self,
        left_operand: nodes::Expression,
        expression_multiplicative_expression_operator: nodes::Expression_MultiplicativeExpression_Operator,
        right_operand: nodes::Expression,
    ) -> nodes::MultiplicativeExpression {
        nodes::new_multiplicative_expression(
            left_operand,
            expression_multiplicative_expression_operator,
            right_operand,
        )
    }
    fn make_named_argument(
        &self,
        name: nodes::Identifier,
        colon: nodes::Colon,
        value: nodes::Expression,
    ) -> nodes::NamedArgument {
        nodes::new_named_argument(name, colon, value)
    }
    fn make_named_argument_group(
        &self,
        open_brace: nodes::OpenBrace,
        arguments: nodes::NamedArguments,
        close_brace: nodes::CloseBrace,
    ) -> nodes::NamedArgumentGroup {
        nodes::new_named_argument_group(open_brace, arguments, close_brace)
    }
    fn make_named_arguments_declaration(
        &self,
        open_paren: nodes::OpenParen,
        arguments: nodes::NamedArgumentGroup,
        close_paren: nodes::CloseParen,
    ) -> nodes::NamedArgumentsDeclaration {
        nodes::new_named_arguments_declaration(open_paren, arguments, close_paren)
    }
    fn make_named_import(
        &self,
        asterisk: nodes::Asterisk,
        alias: nodes::ImportAlias,
        from_keyword: nodes::FromKeyword,
        path: nodes::StringLiteral,
    ) -> nodes::NamedImport {
        nodes::new_named_import(asterisk, alias, from_keyword, path)
    }
    fn make_new_expression(
        &self,
        new_keyword: nodes::NewKeyword,
        type_name: nodes::TypeName,
    ) -> nodes::NewExpression {
        nodes::new_new_expression(new_keyword, type_name)
    }
    fn make_or_expression(
        &self,
        left_operand: nodes::Expression,
        operator: nodes::BarBar,
        right_operand: nodes::Expression,
    ) -> nodes::OrExpression {
        nodes::new_or_expression(left_operand, operator, right_operand)
    }
    fn make_override_paths_declaration(
        &self,
        open_paren: nodes::OpenParen,
        paths: nodes::OverridePaths,
        close_paren: nodes::CloseParen,
    ) -> nodes::OverridePathsDeclaration {
        nodes::new_override_paths_declaration(open_paren, paths, close_paren)
    }
    fn make_override_specifier(
        &self,
        override_keyword: nodes::OverrideKeyword,
        overridden: Option<nodes::OverridePathsDeclaration>,
    ) -> nodes::OverrideSpecifier {
        nodes::new_override_specifier(override_keyword, overridden)
    }
    fn make_parameter(
        &self,
        type_name: nodes::TypeName,
        storage_location: Option<nodes::StorageLocation>,
        name: Option<nodes::Identifier>,
    ) -> nodes::Parameter {
        nodes::new_parameter(type_name, storage_location, name)
    }
    fn make_parameters_declaration(
        &self,
        open_paren: nodes::OpenParen,
        parameters: nodes::Parameters,
        close_paren: nodes::CloseParen,
    ) -> nodes::ParametersDeclaration {
        nodes::new_parameters_declaration(open_paren, parameters, close_paren)
    }
    fn make_path_import(
        &self,
        path: nodes::StringLiteral,
        alias: Option<nodes::ImportAlias>,
    ) -> nodes::PathImport {
        nodes::new_path_import(path, alias)
    }
    fn make_positional_arguments_declaration(
        &self,
        open_paren: nodes::OpenParen,
        arguments: nodes::PositionalArguments,
        close_paren: nodes::CloseParen,
    ) -> nodes::PositionalArgumentsDeclaration {
        nodes::new_positional_arguments_declaration(open_paren, arguments, close_paren)
    }
    fn make_postfix_expression(
        &self,
        operand: nodes::Expression,
        expression_postfix_expression_operator: nodes::Expression_PostfixExpression_Operator,
    ) -> nodes::PostfixExpression {
        nodes::new_postfix_expression(operand, expression_postfix_expression_operator)
    }
    fn make_pragma_directive(
        &self,
        pragma_keyword: nodes::PragmaKeyword,
        pragma: nodes::Pragma,
        semicolon: nodes::PragmaSemicolon,
    ) -> nodes::PragmaDirective {
        nodes::new_pragma_directive(pragma_keyword, pragma, semicolon)
    }
    fn make_prefix_expression(
        &self,
        expression_prefix_expression_operator: nodes::Expression_PrefixExpression_Operator,
        operand: nodes::Expression,
    ) -> nodes::PrefixExpression {
        nodes::new_prefix_expression(expression_prefix_expression_operator, operand)
    }
    fn make_receive_function_definition(
        &self,
        receive_keyword: nodes::ReceiveKeyword,
        parameters: nodes::ParametersDeclaration,
        attributes: nodes::ReceiveFunctionAttributes,
        body: nodes::FunctionBody,
    ) -> nodes::ReceiveFunctionDefinition {
        nodes::new_receive_function_definition(receive_keyword, parameters, attributes, body)
    }
    fn make_return_statement(
        &self,
        return_keyword: nodes::ReturnKeyword,
        expression: Option<nodes::Expression>,
        semicolon: nodes::Semicolon,
    ) -> nodes::ReturnStatement {
        nodes::new_return_statement(return_keyword, expression, semicolon)
    }
    fn make_returns_declaration(
        &self,
        returns_keyword: nodes::ReturnsKeyword,
        variables: nodes::ParametersDeclaration,
    ) -> nodes::ReturnsDeclaration {
        nodes::new_returns_declaration(returns_keyword, variables)
    }
    fn make_revert_statement(
        &self,
        revert_keyword: nodes::RevertKeyword,
        error: nodes::IdentifierPath,
        arguments: nodes::ArgumentsDeclaration,
        semicolon: nodes::Semicolon,
    ) -> nodes::RevertStatement {
        nodes::new_revert_statement(revert_keyword, error, arguments, semicolon)
    }
    fn make_shift_expression(
        &self,
        left_operand: nodes::Expression,
        expression_shift_expression_operator: nodes::Expression_ShiftExpression_Operator,
        right_operand: nodes::Expression,
    ) -> nodes::ShiftExpression {
        nodes::new_shift_expression(
            left_operand,
            expression_shift_expression_operator,
            right_operand,
        )
    }
    fn make_single_typed_declaration(
        &self,
        declaration: nodes::VariableDeclaration,
        value: Option<nodes::VariableDeclarationValue>,
    ) -> nodes::SingleTypedDeclaration {
        nodes::new_single_typed_declaration(declaration, value)
    }
    fn make_source_unit(&self, members: nodes::SourceUnitMembers) -> nodes::SourceUnit {
        nodes::new_source_unit(members)
    }
    fn make_state_variable_definition(
        &self,
        type_name: nodes::TypeName,
        attributes: nodes::StateVariableAttributes,
        name: nodes::Identifier,
        value: Option<nodes::StateVariableDefinitionValue>,
        semicolon: nodes::Semicolon,
    ) -> nodes::StateVariableDefinition {
        nodes::new_state_variable_definition(type_name, attributes, name, value, semicolon)
    }
    fn make_state_variable_definition_value(
        &self,
        equal: nodes::Equal,
        value: nodes::Expression,
    ) -> nodes::StateVariableDefinitionValue {
        nodes::new_state_variable_definition_value(equal, value)
    }
    fn make_storage_layout_specifier(
        &self,
        layout_keyword: nodes::LayoutKeyword,
        at_keyword: nodes::AtKeyword,
        expression: nodes::Expression,
    ) -> nodes::StorageLayoutSpecifier {
        nodes::new_storage_layout_specifier(layout_keyword, at_keyword, expression)
    }
    fn make_struct_definition(
        &self,
        struct_keyword: nodes::StructKeyword,
        name: nodes::Identifier,
        open_brace: nodes::OpenBrace,
        members: nodes::StructMembers,
        close_brace: nodes::CloseBrace,
    ) -> nodes::StructDefinition {
        nodes::new_struct_definition(struct_keyword, name, open_brace, members, close_brace)
    }
    fn make_struct_member(
        &self,
        type_name: nodes::TypeName,
        name: nodes::Identifier,
        semicolon: nodes::Semicolon,
    ) -> nodes::StructMember {
        nodes::new_struct_member(type_name, name, semicolon)
    }
    fn make_try_statement(
        &self,
        try_keyword: nodes::TryKeyword,
        expression: nodes::Expression,
        returns: Option<nodes::ReturnsDeclaration>,
        body: nodes::Block,
        catch_clauses: nodes::CatchClauses,
    ) -> nodes::TryStatement {
        nodes::new_try_statement(try_keyword, expression, returns, body, catch_clauses)
    }
    fn make_tuple_expression(
        &self,
        open_paren: nodes::OpenParen,
        items: nodes::TupleValues,
        close_paren: nodes::CloseParen,
    ) -> nodes::TupleExpression {
        nodes::new_tuple_expression(open_paren, items, close_paren)
    }
    fn make_tuple_value(&self, expression: Option<nodes::Expression>) -> nodes::TupleValue {
        nodes::new_tuple_value(expression)
    }
    fn make_type_expression(
        &self,
        type_keyword: nodes::TypeKeyword,
        open_paren: nodes::OpenParen,
        type_name: nodes::TypeName,
        close_paren: nodes::CloseParen,
    ) -> nodes::TypeExpression {
        nodes::new_type_expression(type_keyword, open_paren, type_name, close_paren)
    }
    fn make_unchecked_block(
        &self,
        unchecked_keyword: nodes::UncheckedKeyword,
        block: nodes::Block,
    ) -> nodes::UncheckedBlock {
        nodes::new_unchecked_block(unchecked_keyword, block)
    }
    fn make_user_defined_value_type_definition(
        &self,
        type_keyword: nodes::TypeKeyword,
        name: nodes::Identifier,
        is_keyword: nodes::IsKeyword,
        value_type: nodes::ElementaryType,
        semicolon: nodes::Semicolon,
    ) -> nodes::UserDefinedValueTypeDefinition {
        nodes::new_user_defined_value_type_definition(
            type_keyword,
            name,
            is_keyword,
            value_type,
            semicolon,
        )
    }
    fn make_using_alias(
        &self,
        as_keyword: nodes::AsKeyword,
        operator: nodes::UsingOperator,
    ) -> nodes::UsingAlias {
        nodes::new_using_alias(as_keyword, operator)
    }
    fn make_using_deconstruction(
        &self,
        open_brace: nodes::OpenBrace,
        symbols: nodes::UsingDeconstructionSymbols,
        close_brace: nodes::CloseBrace,
    ) -> nodes::UsingDeconstruction {
        nodes::new_using_deconstruction(open_brace, symbols, close_brace)
    }
    fn make_using_deconstruction_symbol(
        &self,
        name: nodes::IdentifierPath,
        alias: Option<nodes::UsingAlias>,
    ) -> nodes::UsingDeconstructionSymbol {
        nodes::new_using_deconstruction_symbol(name, alias)
    }
    fn make_using_directive(
        &self,
        using_keyword: nodes::UsingKeyword,
        clause: nodes::UsingClause,
        for_keyword: nodes::ForKeyword,
        target: nodes::UsingTarget,
        global_keyword: Option<nodes::GlobalKeyword>,
        semicolon: nodes::Semicolon,
    ) -> nodes::UsingDirective {
        nodes::new_using_directive(
            using_keyword,
            clause,
            for_keyword,
            target,
            global_keyword,
            semicolon,
        )
    }
    fn make_variable_declaration(
        &self,
        type_name: nodes::TypeName,
        storage_location: Option<nodes::StorageLocation>,
        name: nodes::Identifier,
    ) -> nodes::VariableDeclaration {
        nodes::new_variable_declaration(type_name, storage_location, name)
    }
    fn make_variable_declaration_statement(
        &self,
        target: nodes::VariableDeclarationTarget,
        semicolon: nodes::Semicolon,
    ) -> nodes::VariableDeclarationStatement {
        nodes::new_variable_declaration_statement(target, semicolon)
    }
    fn make_variable_declaration_value(
        &self,
        equal: nodes::Equal,
        expression: nodes::Expression,
    ) -> nodes::VariableDeclarationValue {
        nodes::new_variable_declaration_value(equal, expression)
    }
    fn make_version_pragma(
        &self,
        solidity_keyword: nodes::SolidityKeyword,
        sets: nodes::VersionExpressionSets,
    ) -> nodes::VersionPragma {
        nodes::new_version_pragma(solidity_keyword, sets)
    }
    fn make_version_range(
        &self,
        start: nodes::VersionLiteral,
        minus: nodes::PragmaMinus,
        end: nodes::VersionLiteral,
    ) -> nodes::VersionRange {
        nodes::new_version_range(start, minus, end)
    }
    fn make_version_term(
        &self,
        operator: Option<nodes::VersionOperator>,
        literal: nodes::VersionLiteral,
    ) -> nodes::VersionTerm {
        nodes::new_version_term(operator, literal)
    }
    fn make_while_statement(
        &self,
        while_keyword: nodes::WhileKeyword,
        open_paren: nodes::OpenParen,
        condition: nodes::Expression,
        close_paren: nodes::CloseParen,
        body: nodes::Statement,
    ) -> nodes::WhileStatement {
        nodes::new_while_statement(while_keyword, open_paren, condition, close_paren, body)
    }
    fn make_yul_block(
        &self,
        open_brace: nodes::YulOpenBrace,
        statements: nodes::YulStatements,
        close_brace: nodes::YulCloseBrace,
    ) -> nodes::YulBlock {
        nodes::new_yul_block(open_brace, statements, close_brace)
    }
    fn make_yul_break_statement(
        &self,
        break_keyword: nodes::YulBreakKeyword,
    ) -> nodes::YulBreakStatement {
        nodes::new_yul_break_statement(break_keyword)
    }
    fn make_yul_continue_statement(
        &self,
        continue_keyword: nodes::YulContinueKeyword,
    ) -> nodes::YulContinueStatement {
        nodes::new_yul_continue_statement(continue_keyword)
    }
    fn make_yul_default_case(
        &self,
        default_keyword: nodes::YulDefaultKeyword,
        body: nodes::YulBlock,
    ) -> nodes::YulDefaultCase {
        nodes::new_yul_default_case(default_keyword, body)
    }
    fn make_yul_flags_declaration(
        &self,
        open_paren: nodes::YulOpenParen,
        flags: nodes::YulFlags,
        close_paren: nodes::YulCloseParen,
    ) -> nodes::YulFlagsDeclaration {
        nodes::new_yul_flags_declaration(open_paren, flags, close_paren)
    }
    fn make_yul_for_statement(
        &self,
        for_keyword: nodes::YulForKeyword,
        initialization: nodes::YulBlock,
        condition: nodes::YulExpression,
        iterator: nodes::YulBlock,
        body: nodes::YulBlock,
    ) -> nodes::YulForStatement {
        nodes::new_yul_for_statement(for_keyword, initialization, condition, iterator, body)
    }
    fn make_yul_function_call_expression(
        &self,
        operand: nodes::YulExpression,
        open_paren: nodes::YulOpenParen,
        arguments: nodes::YulArguments,
        close_paren: nodes::YulCloseParen,
    ) -> nodes::YulFunctionCallExpression {
        nodes::new_yul_function_call_expression(operand, open_paren, arguments, close_paren)
    }
    fn make_yul_function_definition(
        &self,
        function_keyword: nodes::YulFunctionKeyword,
        name: nodes::YulIdentifier,
        parameters: nodes::YulParametersDeclaration,
        returns: Option<nodes::YulReturnsDeclaration>,
        body: nodes::YulBlock,
    ) -> nodes::YulFunctionDefinition {
        nodes::new_yul_function_definition(function_keyword, name, parameters, returns, body)
    }
    fn make_yul_if_statement(
        &self,
        if_keyword: nodes::YulIfKeyword,
        condition: nodes::YulExpression,
        body: nodes::YulBlock,
    ) -> nodes::YulIfStatement {
        nodes::new_yul_if_statement(if_keyword, condition, body)
    }
    fn make_yul_leave_statement(
        &self,
        leave_keyword: nodes::YulLeaveKeyword,
    ) -> nodes::YulLeaveStatement {
        nodes::new_yul_leave_statement(leave_keyword)
    }
    fn make_yul_parameters_declaration(
        &self,
        open_paren: nodes::YulOpenParen,
        parameters: nodes::YulParameters,
        close_paren: nodes::YulCloseParen,
    ) -> nodes::YulParametersDeclaration {
        nodes::new_yul_parameters_declaration(open_paren, parameters, close_paren)
    }
    fn make_yul_returns_declaration(
        &self,
        minus_greater_than: nodes::YulMinusGreaterThan,
        variables: nodes::YulVariableNames,
    ) -> nodes::YulReturnsDeclaration {
        nodes::new_yul_returns_declaration(minus_greater_than, variables)
    }
    fn make_yul_switch_statement(
        &self,
        switch_keyword: nodes::YulSwitchKeyword,
        expression: nodes::YulExpression,
        cases: nodes::YulSwitchCases,
    ) -> nodes::YulSwitchStatement {
        nodes::new_yul_switch_statement(switch_keyword, expression, cases)
    }
    fn make_yul_value_case(
        &self,
        case_keyword: nodes::YulCaseKeyword,
        value: nodes::YulLiteral,
        body: nodes::YulBlock,
    ) -> nodes::YulValueCase {
        nodes::new_yul_value_case(case_keyword, value, body)
    }
    fn make_yul_variable_assignment_statement(
        &self,
        variables: nodes::YulPaths,
        assignment: nodes::YulColonEqual,
        expression: nodes::YulExpression,
    ) -> nodes::YulVariableAssignmentStatement {
        nodes::new_yul_variable_assignment_statement(variables, assignment, expression)
    }
    fn make_yul_variable_declaration_statement(
        &self,
        let_keyword: nodes::YulLetKeyword,
        variables: nodes::YulVariableNames,
        value: Option<nodes::YulVariableDeclarationValue>,
    ) -> nodes::YulVariableDeclarationStatement {
        nodes::new_yul_variable_declaration_statement(let_keyword, variables, value)
    }
    fn make_yul_variable_declaration_value(
        &self,
        assignment: nodes::YulColonEqual,
        expression: nodes::YulExpression,
    ) -> nodes::YulVariableDeclarationValue {
        nodes::new_yul_variable_declaration_value(assignment, expression)
    }

    // Choice variant factory methods
    fn make_abicoder_version_abicoder_v1_keyword(
        &self,
        element: nodes::AbicoderV1Keyword,
    ) -> nodes::AbicoderVersion {
        nodes::new_abicoder_version_abicoder_v1_keyword(element)
    }
    fn make_abicoder_version_abicoder_v2_keyword(
        &self,
        element: nodes::AbicoderV2Keyword,
    ) -> nodes::AbicoderVersion {
        nodes::new_abicoder_version_abicoder_v2_keyword(element)
    }

    fn make_arguments_declaration_positional_arguments_declaration(
        &self,
        element: nodes::PositionalArgumentsDeclaration,
    ) -> nodes::ArgumentsDeclaration {
        nodes::new_arguments_declaration_positional_arguments_declaration(element)
    }
    fn make_arguments_declaration_named_arguments_declaration(
        &self,
        element: nodes::NamedArgumentsDeclaration,
    ) -> nodes::ArgumentsDeclaration {
        nodes::new_arguments_declaration_named_arguments_declaration(element)
    }

    fn make_constructor_attribute_modifier_invocation(
        &self,
        element: nodes::ModifierInvocation,
    ) -> nodes::ConstructorAttribute {
        nodes::new_constructor_attribute_modifier_invocation(element)
    }
    fn make_constructor_attribute_internal_keyword(
        &self,
        element: nodes::InternalKeyword,
    ) -> nodes::ConstructorAttribute {
        nodes::new_constructor_attribute_internal_keyword(element)
    }
    fn make_constructor_attribute_payable_keyword(
        &self,
        element: nodes::PayableKeyword,
    ) -> nodes::ConstructorAttribute {
        nodes::new_constructor_attribute_payable_keyword(element)
    }
    fn make_constructor_attribute_public_keyword(
        &self,
        element: nodes::PublicKeyword,
    ) -> nodes::ConstructorAttribute {
        nodes::new_constructor_attribute_public_keyword(element)
    }

    fn make_contract_member_using_directive(
        &self,
        element: nodes::UsingDirective,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_using_directive(element)
    }
    fn make_contract_member_function_definition(
        &self,
        element: nodes::FunctionDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_function_definition(element)
    }
    fn make_contract_member_constructor_definition(
        &self,
        element: nodes::ConstructorDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_constructor_definition(element)
    }
    fn make_contract_member_receive_function_definition(
        &self,
        element: nodes::ReceiveFunctionDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_receive_function_definition(element)
    }
    fn make_contract_member_fallback_function_definition(
        &self,
        element: nodes::FallbackFunctionDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_fallback_function_definition(element)
    }
    fn make_contract_member_modifier_definition(
        &self,
        element: nodes::ModifierDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_modifier_definition(element)
    }
    fn make_contract_member_struct_definition(
        &self,
        element: nodes::StructDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_struct_definition(element)
    }
    fn make_contract_member_enum_definition(
        &self,
        element: nodes::EnumDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_enum_definition(element)
    }
    fn make_contract_member_event_definition(
        &self,
        element: nodes::EventDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_event_definition(element)
    }
    fn make_contract_member_error_definition(
        &self,
        element: nodes::ErrorDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_error_definition(element)
    }
    fn make_contract_member_user_defined_value_type_definition(
        &self,
        element: nodes::UserDefinedValueTypeDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_user_defined_value_type_definition(element)
    }
    fn make_contract_member_state_variable_definition(
        &self,
        element: nodes::StateVariableDefinition,
    ) -> nodes::ContractMember {
        nodes::new_contract_member_state_variable_definition(element)
    }

    fn make_contract_specifier_inheritance_specifier(
        &self,
        element: nodes::InheritanceSpecifier,
    ) -> nodes::ContractSpecifier {
        nodes::new_contract_specifier_inheritance_specifier(element)
    }
    fn make_contract_specifier_storage_layout_specifier(
        &self,
        element: nodes::StorageLayoutSpecifier,
    ) -> nodes::ContractSpecifier {
        nodes::new_contract_specifier_storage_layout_specifier(element)
    }

    fn make_elementary_type_bool_keyword(
        &self,
        element: nodes::BoolKeyword,
    ) -> nodes::ElementaryType {
        nodes::new_elementary_type_bool_keyword(element)
    }
    fn make_elementary_type_string_keyword(
        &self,
        element: nodes::StringKeyword,
    ) -> nodes::ElementaryType {
        nodes::new_elementary_type_string_keyword(element)
    }
    fn make_elementary_type_address_type(
        &self,
        element: nodes::AddressType,
    ) -> nodes::ElementaryType {
        nodes::new_elementary_type_address_type(element)
    }
    fn make_elementary_type_bytes_keyword(
        &self,
        element: nodes::BytesKeyword,
    ) -> nodes::ElementaryType {
        nodes::new_elementary_type_bytes_keyword(element)
    }
    fn make_elementary_type_int_keyword(
        &self,
        element: nodes::IntKeyword,
    ) -> nodes::ElementaryType {
        nodes::new_elementary_type_int_keyword(element)
    }
    fn make_elementary_type_uint_keyword(
        &self,
        element: nodes::UintKeyword,
    ) -> nodes::ElementaryType {
        nodes::new_elementary_type_uint_keyword(element)
    }
    fn make_elementary_type_fixed_keyword(
        &self,
        element: nodes::FixedKeyword,
    ) -> nodes::ElementaryType {
        nodes::new_elementary_type_fixed_keyword(element)
    }
    fn make_elementary_type_ufixed_keyword(
        &self,
        element: nodes::UfixedKeyword,
    ) -> nodes::ElementaryType {
        nodes::new_elementary_type_ufixed_keyword(element)
    }

    fn make_experimental_feature_abi_encoder_v2_keyword(
        &self,
        element: nodes::ABIEncoderV2Keyword,
    ) -> nodes::ExperimentalFeature {
        nodes::new_experimental_feature_abi_encoder_v2_keyword(element)
    }
    fn make_experimental_feature_smt_checker_keyword(
        &self,
        element: nodes::SMTCheckerKeyword,
    ) -> nodes::ExperimentalFeature {
        nodes::new_experimental_feature_smt_checker_keyword(element)
    }
    fn make_experimental_feature_pragma_string_literal(
        &self,
        element: nodes::PragmaStringLiteral,
    ) -> nodes::ExperimentalFeature {
        nodes::new_experimental_feature_pragma_string_literal(element)
    }

    fn make_expression_assignment_expression(
        &self,
        element: nodes::AssignmentExpression,
    ) -> nodes::Expression {
        nodes::new_expression_assignment_expression(element)
    }
    fn make_expression_conditional_expression(
        &self,
        element: nodes::ConditionalExpression,
    ) -> nodes::Expression {
        nodes::new_expression_conditional_expression(element)
    }
    fn make_expression_or_expression(&self, element: nodes::OrExpression) -> nodes::Expression {
        nodes::new_expression_or_expression(element)
    }
    fn make_expression_and_expression(&self, element: nodes::AndExpression) -> nodes::Expression {
        nodes::new_expression_and_expression(element)
    }
    fn make_expression_equality_expression(
        &self,
        element: nodes::EqualityExpression,
    ) -> nodes::Expression {
        nodes::new_expression_equality_expression(element)
    }
    fn make_expression_inequality_expression(
        &self,
        element: nodes::InequalityExpression,
    ) -> nodes::Expression {
        nodes::new_expression_inequality_expression(element)
    }
    fn make_expression_bitwise_or_expression(
        &self,
        element: nodes::BitwiseOrExpression,
    ) -> nodes::Expression {
        nodes::new_expression_bitwise_or_expression(element)
    }
    fn make_expression_bitwise_xor_expression(
        &self,
        element: nodes::BitwiseXorExpression,
    ) -> nodes::Expression {
        nodes::new_expression_bitwise_xor_expression(element)
    }
    fn make_expression_bitwise_and_expression(
        &self,
        element: nodes::BitwiseAndExpression,
    ) -> nodes::Expression {
        nodes::new_expression_bitwise_and_expression(element)
    }
    fn make_expression_shift_expression(
        &self,
        element: nodes::ShiftExpression,
    ) -> nodes::Expression {
        nodes::new_expression_shift_expression(element)
    }
    fn make_expression_additive_expression(
        &self,
        element: nodes::AdditiveExpression,
    ) -> nodes::Expression {
        nodes::new_expression_additive_expression(element)
    }
    fn make_expression_multiplicative_expression(
        &self,
        element: nodes::MultiplicativeExpression,
    ) -> nodes::Expression {
        nodes::new_expression_multiplicative_expression(element)
    }
    fn make_expression_exponentiation_expression(
        &self,
        element: nodes::ExponentiationExpression,
    ) -> nodes::Expression {
        nodes::new_expression_exponentiation_expression(element)
    }
    fn make_expression_postfix_expression(
        &self,
        element: nodes::PostfixExpression,
    ) -> nodes::Expression {
        nodes::new_expression_postfix_expression(element)
    }
    fn make_expression_prefix_expression(
        &self,
        element: nodes::PrefixExpression,
    ) -> nodes::Expression {
        nodes::new_expression_prefix_expression(element)
    }
    fn make_expression_function_call_expression(
        &self,
        element: nodes::FunctionCallExpression,
    ) -> nodes::Expression {
        nodes::new_expression_function_call_expression(element)
    }
    fn make_expression_call_options_expression(
        &self,
        element: nodes::CallOptionsExpression,
    ) -> nodes::Expression {
        nodes::new_expression_call_options_expression(element)
    }
    fn make_expression_member_access_expression(
        &self,
        element: nodes::MemberAccessExpression,
    ) -> nodes::Expression {
        nodes::new_expression_member_access_expression(element)
    }
    fn make_expression_index_access_expression(
        &self,
        element: nodes::IndexAccessExpression,
    ) -> nodes::Expression {
        nodes::new_expression_index_access_expression(element)
    }
    fn make_expression_new_expression(&self, element: nodes::NewExpression) -> nodes::Expression {
        nodes::new_expression_new_expression(element)
    }
    fn make_expression_tuple_expression(
        &self,
        element: nodes::TupleExpression,
    ) -> nodes::Expression {
        nodes::new_expression_tuple_expression(element)
    }
    fn make_expression_type_expression(&self, element: nodes::TypeExpression) -> nodes::Expression {
        nodes::new_expression_type_expression(element)
    }
    fn make_expression_array_expression(
        &self,
        element: nodes::ArrayExpression,
    ) -> nodes::Expression {
        nodes::new_expression_array_expression(element)
    }
    fn make_expression_hex_number_expression(
        &self,
        element: nodes::HexNumberExpression,
    ) -> nodes::Expression {
        nodes::new_expression_hex_number_expression(element)
    }
    fn make_expression_decimal_number_expression(
        &self,
        element: nodes::DecimalNumberExpression,
    ) -> nodes::Expression {
        nodes::new_expression_decimal_number_expression(element)
    }
    fn make_expression_string_expression(
        &self,
        element: nodes::StringExpression,
    ) -> nodes::Expression {
        nodes::new_expression_string_expression(element)
    }
    fn make_expression_elementary_type(&self, element: nodes::ElementaryType) -> nodes::Expression {
        nodes::new_expression_elementary_type(element)
    }
    fn make_expression_payable_keyword(&self, element: nodes::PayableKeyword) -> nodes::Expression {
        nodes::new_expression_payable_keyword(element)
    }
    fn make_expression_this_keyword(&self, element: nodes::ThisKeyword) -> nodes::Expression {
        nodes::new_expression_this_keyword(element)
    }
    fn make_expression_super_keyword(&self, element: nodes::SuperKeyword) -> nodes::Expression {
        nodes::new_expression_super_keyword(element)
    }
    fn make_expression_true_keyword(&self, element: nodes::TrueKeyword) -> nodes::Expression {
        nodes::new_expression_true_keyword(element)
    }
    fn make_expression_false_keyword(&self, element: nodes::FalseKeyword) -> nodes::Expression {
        nodes::new_expression_false_keyword(element)
    }
    fn make_expression_identifier(&self, element: nodes::Identifier) -> nodes::Expression {
        nodes::new_expression_identifier(element)
    }

    fn make_expression_additive_expression_operator_minus(
        &self,
        element: nodes::Minus,
    ) -> nodes::Expression_AdditiveExpression_Operator {
        nodes::new_expression_additive_expression_operator_minus(element)
    }
    fn make_expression_additive_expression_operator_plus(
        &self,
        element: nodes::Plus,
    ) -> nodes::Expression_AdditiveExpression_Operator {
        nodes::new_expression_additive_expression_operator_plus(element)
    }

    fn make_expression_assignment_expression_operator_ampersand_equal(
        &self,
        element: nodes::AmpersandEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_ampersand_equal(element)
    }
    fn make_expression_assignment_expression_operator_asterisk_equal(
        &self,
        element: nodes::AsteriskEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_asterisk_equal(element)
    }
    fn make_expression_assignment_expression_operator_bar_equal(
        &self,
        element: nodes::BarEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_bar_equal(element)
    }
    fn make_expression_assignment_expression_operator_caret_equal(
        &self,
        element: nodes::CaretEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_caret_equal(element)
    }
    fn make_expression_assignment_expression_operator_equal(
        &self,
        element: nodes::Equal,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_equal(element)
    }
    fn make_expression_assignment_expression_operator_greater_than_greater_than_equal(
        &self,
        element: nodes::GreaterThanGreaterThanEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_greater_than_greater_than_equal(
            element,
        )
    }
    fn make_expression_assignment_expression_operator_greater_than_greater_than_greater_than_equal(
        &self,
        element: nodes::GreaterThanGreaterThanGreaterThanEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_greater_than_greater_than_greater_than_equal(element)
    }
    fn make_expression_assignment_expression_operator_less_than_less_than_equal(
        &self,
        element: nodes::LessThanLessThanEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_less_than_less_than_equal(element)
    }
    fn make_expression_assignment_expression_operator_minus_equal(
        &self,
        element: nodes::MinusEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_minus_equal(element)
    }
    fn make_expression_assignment_expression_operator_percent_equal(
        &self,
        element: nodes::PercentEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_percent_equal(element)
    }
    fn make_expression_assignment_expression_operator_plus_equal(
        &self,
        element: nodes::PlusEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_plus_equal(element)
    }
    fn make_expression_assignment_expression_operator_slash_equal(
        &self,
        element: nodes::SlashEqual,
    ) -> nodes::Expression_AssignmentExpression_Operator {
        nodes::new_expression_assignment_expression_operator_slash_equal(element)
    }

    fn make_expression_equality_expression_operator_bang_equal(
        &self,
        element: nodes::BangEqual,
    ) -> nodes::Expression_EqualityExpression_Operator {
        nodes::new_expression_equality_expression_operator_bang_equal(element)
    }
    fn make_expression_equality_expression_operator_equal_equal(
        &self,
        element: nodes::EqualEqual,
    ) -> nodes::Expression_EqualityExpression_Operator {
        nodes::new_expression_equality_expression_operator_equal_equal(element)
    }

    fn make_expression_inequality_expression_operator_greater_than(
        &self,
        element: nodes::GreaterThan,
    ) -> nodes::Expression_InequalityExpression_Operator {
        nodes::new_expression_inequality_expression_operator_greater_than(element)
    }
    fn make_expression_inequality_expression_operator_greater_than_equal(
        &self,
        element: nodes::GreaterThanEqual,
    ) -> nodes::Expression_InequalityExpression_Operator {
        nodes::new_expression_inequality_expression_operator_greater_than_equal(element)
    }
    fn make_expression_inequality_expression_operator_less_than(
        &self,
        element: nodes::LessThan,
    ) -> nodes::Expression_InequalityExpression_Operator {
        nodes::new_expression_inequality_expression_operator_less_than(element)
    }
    fn make_expression_inequality_expression_operator_less_than_equal(
        &self,
        element: nodes::LessThanEqual,
    ) -> nodes::Expression_InequalityExpression_Operator {
        nodes::new_expression_inequality_expression_operator_less_than_equal(element)
    }

    fn make_expression_multiplicative_expression_operator_asterisk(
        &self,
        element: nodes::Asterisk,
    ) -> nodes::Expression_MultiplicativeExpression_Operator {
        nodes::new_expression_multiplicative_expression_operator_asterisk(element)
    }
    fn make_expression_multiplicative_expression_operator_percent(
        &self,
        element: nodes::Percent,
    ) -> nodes::Expression_MultiplicativeExpression_Operator {
        nodes::new_expression_multiplicative_expression_operator_percent(element)
    }
    fn make_expression_multiplicative_expression_operator_slash(
        &self,
        element: nodes::Slash,
    ) -> nodes::Expression_MultiplicativeExpression_Operator {
        nodes::new_expression_multiplicative_expression_operator_slash(element)
    }

    fn make_expression_postfix_expression_operator_minus_minus(
        &self,
        element: nodes::MinusMinus,
    ) -> nodes::Expression_PostfixExpression_Operator {
        nodes::new_expression_postfix_expression_operator_minus_minus(element)
    }
    fn make_expression_postfix_expression_operator_plus_plus(
        &self,
        element: nodes::PlusPlus,
    ) -> nodes::Expression_PostfixExpression_Operator {
        nodes::new_expression_postfix_expression_operator_plus_plus(element)
    }

    fn make_expression_prefix_expression_operator_bang(
        &self,
        element: nodes::Bang,
    ) -> nodes::Expression_PrefixExpression_Operator {
        nodes::new_expression_prefix_expression_operator_bang(element)
    }
    fn make_expression_prefix_expression_operator_delete_keyword(
        &self,
        element: nodes::DeleteKeyword,
    ) -> nodes::Expression_PrefixExpression_Operator {
        nodes::new_expression_prefix_expression_operator_delete_keyword(element)
    }
    fn make_expression_prefix_expression_operator_minus(
        &self,
        element: nodes::Minus,
    ) -> nodes::Expression_PrefixExpression_Operator {
        nodes::new_expression_prefix_expression_operator_minus(element)
    }
    fn make_expression_prefix_expression_operator_minus_minus(
        &self,
        element: nodes::MinusMinus,
    ) -> nodes::Expression_PrefixExpression_Operator {
        nodes::new_expression_prefix_expression_operator_minus_minus(element)
    }
    fn make_expression_prefix_expression_operator_plus_plus(
        &self,
        element: nodes::PlusPlus,
    ) -> nodes::Expression_PrefixExpression_Operator {
        nodes::new_expression_prefix_expression_operator_plus_plus(element)
    }
    fn make_expression_prefix_expression_operator_tilde(
        &self,
        element: nodes::Tilde,
    ) -> nodes::Expression_PrefixExpression_Operator {
        nodes::new_expression_prefix_expression_operator_tilde(element)
    }

    fn make_expression_shift_expression_operator_greater_than_greater_than(
        &self,
        element: nodes::GreaterThanGreaterThan,
    ) -> nodes::Expression_ShiftExpression_Operator {
        nodes::new_expression_shift_expression_operator_greater_than_greater_than(element)
    }
    fn make_expression_shift_expression_operator_greater_than_greater_than_greater_than(
        &self,
        element: nodes::GreaterThanGreaterThanGreaterThan,
    ) -> nodes::Expression_ShiftExpression_Operator {
        nodes::new_expression_shift_expression_operator_greater_than_greater_than_greater_than(
            element,
        )
    }
    fn make_expression_shift_expression_operator_less_than_less_than(
        &self,
        element: nodes::LessThanLessThan,
    ) -> nodes::Expression_ShiftExpression_Operator {
        nodes::new_expression_shift_expression_operator_less_than_less_than(element)
    }

    fn make_fallback_function_attribute_modifier_invocation(
        &self,
        element: nodes::ModifierInvocation,
    ) -> nodes::FallbackFunctionAttribute {
        nodes::new_fallback_function_attribute_modifier_invocation(element)
    }
    fn make_fallback_function_attribute_override_specifier(
        &self,
        element: nodes::OverrideSpecifier,
    ) -> nodes::FallbackFunctionAttribute {
        nodes::new_fallback_function_attribute_override_specifier(element)
    }
    fn make_fallback_function_attribute_external_keyword(
        &self,
        element: nodes::ExternalKeyword,
    ) -> nodes::FallbackFunctionAttribute {
        nodes::new_fallback_function_attribute_external_keyword(element)
    }
    fn make_fallback_function_attribute_payable_keyword(
        &self,
        element: nodes::PayableKeyword,
    ) -> nodes::FallbackFunctionAttribute {
        nodes::new_fallback_function_attribute_payable_keyword(element)
    }
    fn make_fallback_function_attribute_pure_keyword(
        &self,
        element: nodes::PureKeyword,
    ) -> nodes::FallbackFunctionAttribute {
        nodes::new_fallback_function_attribute_pure_keyword(element)
    }
    fn make_fallback_function_attribute_view_keyword(
        &self,
        element: nodes::ViewKeyword,
    ) -> nodes::FallbackFunctionAttribute {
        nodes::new_fallback_function_attribute_view_keyword(element)
    }
    fn make_fallback_function_attribute_virtual_keyword(
        &self,
        element: nodes::VirtualKeyword,
    ) -> nodes::FallbackFunctionAttribute {
        nodes::new_fallback_function_attribute_virtual_keyword(element)
    }

    fn make_for_statement_condition_expression_statement(
        &self,
        element: nodes::ExpressionStatement,
    ) -> nodes::ForStatementCondition {
        nodes::new_for_statement_condition_expression_statement(element)
    }
    fn make_for_statement_condition_semicolon(
        &self,
        element: nodes::Semicolon,
    ) -> nodes::ForStatementCondition {
        nodes::new_for_statement_condition_semicolon(element)
    }

    fn make_for_statement_initialization_variable_declaration_statement(
        &self,
        element: nodes::VariableDeclarationStatement,
    ) -> nodes::ForStatementInitialization {
        nodes::new_for_statement_initialization_variable_declaration_statement(element)
    }
    fn make_for_statement_initialization_expression_statement(
        &self,
        element: nodes::ExpressionStatement,
    ) -> nodes::ForStatementInitialization {
        nodes::new_for_statement_initialization_expression_statement(element)
    }
    fn make_for_statement_initialization_semicolon(
        &self,
        element: nodes::Semicolon,
    ) -> nodes::ForStatementInitialization {
        nodes::new_for_statement_initialization_semicolon(element)
    }

    fn make_function_attribute_modifier_invocation(
        &self,
        element: nodes::ModifierInvocation,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_modifier_invocation(element)
    }
    fn make_function_attribute_override_specifier(
        &self,
        element: nodes::OverrideSpecifier,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_override_specifier(element)
    }
    fn make_function_attribute_external_keyword(
        &self,
        element: nodes::ExternalKeyword,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_external_keyword(element)
    }
    fn make_function_attribute_internal_keyword(
        &self,
        element: nodes::InternalKeyword,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_internal_keyword(element)
    }
    fn make_function_attribute_payable_keyword(
        &self,
        element: nodes::PayableKeyword,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_payable_keyword(element)
    }
    fn make_function_attribute_private_keyword(
        &self,
        element: nodes::PrivateKeyword,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_private_keyword(element)
    }
    fn make_function_attribute_public_keyword(
        &self,
        element: nodes::PublicKeyword,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_public_keyword(element)
    }
    fn make_function_attribute_pure_keyword(
        &self,
        element: nodes::PureKeyword,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_pure_keyword(element)
    }
    fn make_function_attribute_view_keyword(
        &self,
        element: nodes::ViewKeyword,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_view_keyword(element)
    }
    fn make_function_attribute_virtual_keyword(
        &self,
        element: nodes::VirtualKeyword,
    ) -> nodes::FunctionAttribute {
        nodes::new_function_attribute_virtual_keyword(element)
    }

    fn make_function_body_block(&self, element: nodes::Block) -> nodes::FunctionBody {
        nodes::new_function_body_block(element)
    }
    fn make_function_body_semicolon(&self, element: nodes::Semicolon) -> nodes::FunctionBody {
        nodes::new_function_body_semicolon(element)
    }

    fn make_function_name_identifier(&self, element: nodes::Identifier) -> nodes::FunctionName {
        nodes::new_function_name_identifier(element)
    }
    fn make_function_name_fallback_keyword(
        &self,
        element: nodes::FallbackKeyword,
    ) -> nodes::FunctionName {
        nodes::new_function_name_fallback_keyword(element)
    }
    fn make_function_name_receive_keyword(
        &self,
        element: nodes::ReceiveKeyword,
    ) -> nodes::FunctionName {
        nodes::new_function_name_receive_keyword(element)
    }

    fn make_function_type_attribute_internal_keyword(
        &self,
        element: nodes::InternalKeyword,
    ) -> nodes::FunctionTypeAttribute {
        nodes::new_function_type_attribute_internal_keyword(element)
    }
    fn make_function_type_attribute_external_keyword(
        &self,
        element: nodes::ExternalKeyword,
    ) -> nodes::FunctionTypeAttribute {
        nodes::new_function_type_attribute_external_keyword(element)
    }
    fn make_function_type_attribute_private_keyword(
        &self,
        element: nodes::PrivateKeyword,
    ) -> nodes::FunctionTypeAttribute {
        nodes::new_function_type_attribute_private_keyword(element)
    }
    fn make_function_type_attribute_public_keyword(
        &self,
        element: nodes::PublicKeyword,
    ) -> nodes::FunctionTypeAttribute {
        nodes::new_function_type_attribute_public_keyword(element)
    }
    fn make_function_type_attribute_pure_keyword(
        &self,
        element: nodes::PureKeyword,
    ) -> nodes::FunctionTypeAttribute {
        nodes::new_function_type_attribute_pure_keyword(element)
    }
    fn make_function_type_attribute_view_keyword(
        &self,
        element: nodes::ViewKeyword,
    ) -> nodes::FunctionTypeAttribute {
        nodes::new_function_type_attribute_view_keyword(element)
    }
    fn make_function_type_attribute_payable_keyword(
        &self,
        element: nodes::PayableKeyword,
    ) -> nodes::FunctionTypeAttribute {
        nodes::new_function_type_attribute_payable_keyword(element)
    }

    fn make_identifier_path_element_identifier(
        &self,
        element: nodes::Identifier,
    ) -> nodes::IdentifierPathElement {
        nodes::new_identifier_path_element_identifier(element)
    }
    fn make_identifier_path_element_address_keyword(
        &self,
        element: nodes::AddressKeyword,
    ) -> nodes::IdentifierPathElement {
        nodes::new_identifier_path_element_address_keyword(element)
    }

    fn make_import_clause_path_import(&self, element: nodes::PathImport) -> nodes::ImportClause {
        nodes::new_import_clause_path_import(element)
    }
    fn make_import_clause_named_import(&self, element: nodes::NamedImport) -> nodes::ImportClause {
        nodes::new_import_clause_named_import(element)
    }
    fn make_import_clause_import_deconstruction(
        &self,
        element: nodes::ImportDeconstruction,
    ) -> nodes::ImportClause {
        nodes::new_import_clause_import_deconstruction(element)
    }

    fn make_mapping_key_type_elementary_type(
        &self,
        element: nodes::ElementaryType,
    ) -> nodes::MappingKeyType {
        nodes::new_mapping_key_type_elementary_type(element)
    }
    fn make_mapping_key_type_identifier_path(
        &self,
        element: nodes::IdentifierPath,
    ) -> nodes::MappingKeyType {
        nodes::new_mapping_key_type_identifier_path(element)
    }

    fn make_modifier_attribute_override_specifier(
        &self,
        element: nodes::OverrideSpecifier,
    ) -> nodes::ModifierAttribute {
        nodes::new_modifier_attribute_override_specifier(element)
    }
    fn make_modifier_attribute_virtual_keyword(
        &self,
        element: nodes::VirtualKeyword,
    ) -> nodes::ModifierAttribute {
        nodes::new_modifier_attribute_virtual_keyword(element)
    }

    fn make_number_unit_wei_keyword(&self, element: nodes::WeiKeyword) -> nodes::NumberUnit {
        nodes::new_number_unit_wei_keyword(element)
    }
    fn make_number_unit_gwei_keyword(&self, element: nodes::GweiKeyword) -> nodes::NumberUnit {
        nodes::new_number_unit_gwei_keyword(element)
    }
    fn make_number_unit_ether_keyword(&self, element: nodes::EtherKeyword) -> nodes::NumberUnit {
        nodes::new_number_unit_ether_keyword(element)
    }
    fn make_number_unit_seconds_keyword(
        &self,
        element: nodes::SecondsKeyword,
    ) -> nodes::NumberUnit {
        nodes::new_number_unit_seconds_keyword(element)
    }
    fn make_number_unit_minutes_keyword(
        &self,
        element: nodes::MinutesKeyword,
    ) -> nodes::NumberUnit {
        nodes::new_number_unit_minutes_keyword(element)
    }
    fn make_number_unit_hours_keyword(&self, element: nodes::HoursKeyword) -> nodes::NumberUnit {
        nodes::new_number_unit_hours_keyword(element)
    }
    fn make_number_unit_days_keyword(&self, element: nodes::DaysKeyword) -> nodes::NumberUnit {
        nodes::new_number_unit_days_keyword(element)
    }
    fn make_number_unit_weeks_keyword(&self, element: nodes::WeeksKeyword) -> nodes::NumberUnit {
        nodes::new_number_unit_weeks_keyword(element)
    }

    fn make_pragma_version_pragma(&self, element: nodes::VersionPragma) -> nodes::Pragma {
        nodes::new_pragma_version_pragma(element)
    }
    fn make_pragma_abicoder_pragma(&self, element: nodes::AbicoderPragma) -> nodes::Pragma {
        nodes::new_pragma_abicoder_pragma(element)
    }
    fn make_pragma_experimental_pragma(&self, element: nodes::ExperimentalPragma) -> nodes::Pragma {
        nodes::new_pragma_experimental_pragma(element)
    }

    fn make_receive_function_attribute_modifier_invocation(
        &self,
        element: nodes::ModifierInvocation,
    ) -> nodes::ReceiveFunctionAttribute {
        nodes::new_receive_function_attribute_modifier_invocation(element)
    }
    fn make_receive_function_attribute_override_specifier(
        &self,
        element: nodes::OverrideSpecifier,
    ) -> nodes::ReceiveFunctionAttribute {
        nodes::new_receive_function_attribute_override_specifier(element)
    }
    fn make_receive_function_attribute_external_keyword(
        &self,
        element: nodes::ExternalKeyword,
    ) -> nodes::ReceiveFunctionAttribute {
        nodes::new_receive_function_attribute_external_keyword(element)
    }
    fn make_receive_function_attribute_payable_keyword(
        &self,
        element: nodes::PayableKeyword,
    ) -> nodes::ReceiveFunctionAttribute {
        nodes::new_receive_function_attribute_payable_keyword(element)
    }
    fn make_receive_function_attribute_virtual_keyword(
        &self,
        element: nodes::VirtualKeyword,
    ) -> nodes::ReceiveFunctionAttribute {
        nodes::new_receive_function_attribute_virtual_keyword(element)
    }

    fn make_source_unit_member_pragma_directive(
        &self,
        element: nodes::PragmaDirective,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_pragma_directive(element)
    }
    fn make_source_unit_member_import_directive(
        &self,
        element: nodes::ImportDirective,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_import_directive(element)
    }
    fn make_source_unit_member_contract_definition(
        &self,
        element: nodes::ContractDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_contract_definition(element)
    }
    fn make_source_unit_member_interface_definition(
        &self,
        element: nodes::InterfaceDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_interface_definition(element)
    }
    fn make_source_unit_member_library_definition(
        &self,
        element: nodes::LibraryDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_library_definition(element)
    }
    fn make_source_unit_member_struct_definition(
        &self,
        element: nodes::StructDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_struct_definition(element)
    }
    fn make_source_unit_member_enum_definition(
        &self,
        element: nodes::EnumDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_enum_definition(element)
    }
    fn make_source_unit_member_function_definition(
        &self,
        element: nodes::FunctionDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_function_definition(element)
    }
    fn make_source_unit_member_error_definition(
        &self,
        element: nodes::ErrorDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_error_definition(element)
    }
    fn make_source_unit_member_user_defined_value_type_definition(
        &self,
        element: nodes::UserDefinedValueTypeDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_user_defined_value_type_definition(element)
    }
    fn make_source_unit_member_using_directive(
        &self,
        element: nodes::UsingDirective,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_using_directive(element)
    }
    fn make_source_unit_member_event_definition(
        &self,
        element: nodes::EventDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_event_definition(element)
    }
    fn make_source_unit_member_constant_definition(
        &self,
        element: nodes::ConstantDefinition,
    ) -> nodes::SourceUnitMember {
        nodes::new_source_unit_member_constant_definition(element)
    }

    fn make_state_variable_attribute_override_specifier(
        &self,
        element: nodes::OverrideSpecifier,
    ) -> nodes::StateVariableAttribute {
        nodes::new_state_variable_attribute_override_specifier(element)
    }
    fn make_state_variable_attribute_constant_keyword(
        &self,
        element: nodes::ConstantKeyword,
    ) -> nodes::StateVariableAttribute {
        nodes::new_state_variable_attribute_constant_keyword(element)
    }
    fn make_state_variable_attribute_internal_keyword(
        &self,
        element: nodes::InternalKeyword,
    ) -> nodes::StateVariableAttribute {
        nodes::new_state_variable_attribute_internal_keyword(element)
    }
    fn make_state_variable_attribute_private_keyword(
        &self,
        element: nodes::PrivateKeyword,
    ) -> nodes::StateVariableAttribute {
        nodes::new_state_variable_attribute_private_keyword(element)
    }
    fn make_state_variable_attribute_public_keyword(
        &self,
        element: nodes::PublicKeyword,
    ) -> nodes::StateVariableAttribute {
        nodes::new_state_variable_attribute_public_keyword(element)
    }
    fn make_state_variable_attribute_immutable_keyword(
        &self,
        element: nodes::ImmutableKeyword,
    ) -> nodes::StateVariableAttribute {
        nodes::new_state_variable_attribute_immutable_keyword(element)
    }
    fn make_state_variable_attribute_transient_keyword(
        &self,
        element: nodes::TransientKeyword,
    ) -> nodes::StateVariableAttribute {
        nodes::new_state_variable_attribute_transient_keyword(element)
    }

    fn make_statement_if_statement(&self, element: nodes::IfStatement) -> nodes::Statement {
        nodes::new_statement_if_statement(element)
    }
    fn make_statement_for_statement(&self, element: nodes::ForStatement) -> nodes::Statement {
        nodes::new_statement_for_statement(element)
    }
    fn make_statement_while_statement(&self, element: nodes::WhileStatement) -> nodes::Statement {
        nodes::new_statement_while_statement(element)
    }
    fn make_statement_do_while_statement(
        &self,
        element: nodes::DoWhileStatement,
    ) -> nodes::Statement {
        nodes::new_statement_do_while_statement(element)
    }
    fn make_statement_continue_statement(
        &self,
        element: nodes::ContinueStatement,
    ) -> nodes::Statement {
        nodes::new_statement_continue_statement(element)
    }
    fn make_statement_break_statement(&self, element: nodes::BreakStatement) -> nodes::Statement {
        nodes::new_statement_break_statement(element)
    }
    fn make_statement_return_statement(&self, element: nodes::ReturnStatement) -> nodes::Statement {
        nodes::new_statement_return_statement(element)
    }
    fn make_statement_emit_statement(&self, element: nodes::EmitStatement) -> nodes::Statement {
        nodes::new_statement_emit_statement(element)
    }
    fn make_statement_try_statement(&self, element: nodes::TryStatement) -> nodes::Statement {
        nodes::new_statement_try_statement(element)
    }
    fn make_statement_revert_statement(&self, element: nodes::RevertStatement) -> nodes::Statement {
        nodes::new_statement_revert_statement(element)
    }
    fn make_statement_assembly_statement(
        &self,
        element: nodes::AssemblyStatement,
    ) -> nodes::Statement {
        nodes::new_statement_assembly_statement(element)
    }
    fn make_statement_block(&self, element: nodes::Block) -> nodes::Statement {
        nodes::new_statement_block(element)
    }
    fn make_statement_unchecked_block(&self, element: nodes::UncheckedBlock) -> nodes::Statement {
        nodes::new_statement_unchecked_block(element)
    }
    fn make_statement_variable_declaration_statement(
        &self,
        element: nodes::VariableDeclarationStatement,
    ) -> nodes::Statement {
        nodes::new_statement_variable_declaration_statement(element)
    }
    fn make_statement_expression_statement(
        &self,
        element: nodes::ExpressionStatement,
    ) -> nodes::Statement {
        nodes::new_statement_expression_statement(element)
    }

    fn make_storage_location_memory_keyword(
        &self,
        element: nodes::MemoryKeyword,
    ) -> nodes::StorageLocation {
        nodes::new_storage_location_memory_keyword(element)
    }
    fn make_storage_location_storage_keyword(
        &self,
        element: nodes::StorageKeyword,
    ) -> nodes::StorageLocation {
        nodes::new_storage_location_storage_keyword(element)
    }
    fn make_storage_location_call_data_keyword(
        &self,
        element: nodes::CallDataKeyword,
    ) -> nodes::StorageLocation {
        nodes::new_storage_location_call_data_keyword(element)
    }

    fn make_string_expression_string_literals(
        &self,
        element: nodes::StringLiterals,
    ) -> nodes::StringExpression {
        nodes::new_string_expression_string_literals(element)
    }
    fn make_string_expression_hex_string_literals(
        &self,
        element: nodes::HexStringLiterals,
    ) -> nodes::StringExpression {
        nodes::new_string_expression_hex_string_literals(element)
    }
    fn make_string_expression_unicode_string_literals(
        &self,
        element: nodes::UnicodeStringLiterals,
    ) -> nodes::StringExpression {
        nodes::new_string_expression_unicode_string_literals(element)
    }

    fn make_type_name_array_type_name(&self, element: nodes::ArrayTypeName) -> nodes::TypeName {
        nodes::new_type_name_array_type_name(element)
    }
    fn make_type_name_function_type(&self, element: nodes::FunctionType) -> nodes::TypeName {
        nodes::new_type_name_function_type(element)
    }
    fn make_type_name_mapping_type(&self, element: nodes::MappingType) -> nodes::TypeName {
        nodes::new_type_name_mapping_type(element)
    }
    fn make_type_name_elementary_type(&self, element: nodes::ElementaryType) -> nodes::TypeName {
        nodes::new_type_name_elementary_type(element)
    }
    fn make_type_name_identifier_path(&self, element: nodes::IdentifierPath) -> nodes::TypeName {
        nodes::new_type_name_identifier_path(element)
    }

    fn make_using_clause_identifier_path(
        &self,
        element: nodes::IdentifierPath,
    ) -> nodes::UsingClause {
        nodes::new_using_clause_identifier_path(element)
    }
    fn make_using_clause_using_deconstruction(
        &self,
        element: nodes::UsingDeconstruction,
    ) -> nodes::UsingClause {
        nodes::new_using_clause_using_deconstruction(element)
    }

    fn make_using_operator_ampersand(&self, element: nodes::Ampersand) -> nodes::UsingOperator {
        nodes::new_using_operator_ampersand(element)
    }
    fn make_using_operator_asterisk(&self, element: nodes::Asterisk) -> nodes::UsingOperator {
        nodes::new_using_operator_asterisk(element)
    }
    fn make_using_operator_bang_equal(&self, element: nodes::BangEqual) -> nodes::UsingOperator {
        nodes::new_using_operator_bang_equal(element)
    }
    fn make_using_operator_bar(&self, element: nodes::Bar) -> nodes::UsingOperator {
        nodes::new_using_operator_bar(element)
    }
    fn make_using_operator_caret(&self, element: nodes::Caret) -> nodes::UsingOperator {
        nodes::new_using_operator_caret(element)
    }
    fn make_using_operator_equal_equal(&self, element: nodes::EqualEqual) -> nodes::UsingOperator {
        nodes::new_using_operator_equal_equal(element)
    }
    fn make_using_operator_greater_than(
        &self,
        element: nodes::GreaterThan,
    ) -> nodes::UsingOperator {
        nodes::new_using_operator_greater_than(element)
    }
    fn make_using_operator_greater_than_equal(
        &self,
        element: nodes::GreaterThanEqual,
    ) -> nodes::UsingOperator {
        nodes::new_using_operator_greater_than_equal(element)
    }
    fn make_using_operator_less_than(&self, element: nodes::LessThan) -> nodes::UsingOperator {
        nodes::new_using_operator_less_than(element)
    }
    fn make_using_operator_less_than_equal(
        &self,
        element: nodes::LessThanEqual,
    ) -> nodes::UsingOperator {
        nodes::new_using_operator_less_than_equal(element)
    }
    fn make_using_operator_minus(&self, element: nodes::Minus) -> nodes::UsingOperator {
        nodes::new_using_operator_minus(element)
    }
    fn make_using_operator_percent(&self, element: nodes::Percent) -> nodes::UsingOperator {
        nodes::new_using_operator_percent(element)
    }
    fn make_using_operator_plus(&self, element: nodes::Plus) -> nodes::UsingOperator {
        nodes::new_using_operator_plus(element)
    }
    fn make_using_operator_slash(&self, element: nodes::Slash) -> nodes::UsingOperator {
        nodes::new_using_operator_slash(element)
    }
    fn make_using_operator_tilde(&self, element: nodes::Tilde) -> nodes::UsingOperator {
        nodes::new_using_operator_tilde(element)
    }

    fn make_using_target_type_name(&self, element: nodes::TypeName) -> nodes::UsingTarget {
        nodes::new_using_target_type_name(element)
    }
    fn make_using_target_asterisk(&self, element: nodes::Asterisk) -> nodes::UsingTarget {
        nodes::new_using_target_asterisk(element)
    }

    fn make_variable_declaration_target_single_typed_declaration(
        &self,
        element: nodes::SingleTypedDeclaration,
    ) -> nodes::VariableDeclarationTarget {
        nodes::new_variable_declaration_target_single_typed_declaration(element)
    }
    fn make_variable_declaration_target_multi_typed_declaration(
        &self,
        element: nodes::MultiTypedDeclaration,
    ) -> nodes::VariableDeclarationTarget {
        nodes::new_variable_declaration_target_multi_typed_declaration(element)
    }

    fn make_version_expression_version_range(
        &self,
        element: nodes::VersionRange,
    ) -> nodes::VersionExpression {
        nodes::new_version_expression_version_range(element)
    }
    fn make_version_expression_version_term(
        &self,
        element: nodes::VersionTerm,
    ) -> nodes::VersionExpression {
        nodes::new_version_expression_version_term(element)
    }

    fn make_version_literal_simple_version_literal(
        &self,
        element: nodes::SimpleVersionLiteral,
    ) -> nodes::VersionLiteral {
        nodes::new_version_literal_simple_version_literal(element)
    }
    fn make_version_literal_pragma_string_literal(
        &self,
        element: nodes::PragmaStringLiteral,
    ) -> nodes::VersionLiteral {
        nodes::new_version_literal_pragma_string_literal(element)
    }

    fn make_version_operator_pragma_caret(
        &self,
        element: nodes::PragmaCaret,
    ) -> nodes::VersionOperator {
        nodes::new_version_operator_pragma_caret(element)
    }
    fn make_version_operator_pragma_tilde(
        &self,
        element: nodes::PragmaTilde,
    ) -> nodes::VersionOperator {
        nodes::new_version_operator_pragma_tilde(element)
    }
    fn make_version_operator_pragma_equal(
        &self,
        element: nodes::PragmaEqual,
    ) -> nodes::VersionOperator {
        nodes::new_version_operator_pragma_equal(element)
    }
    fn make_version_operator_pragma_less_than(
        &self,
        element: nodes::PragmaLessThan,
    ) -> nodes::VersionOperator {
        nodes::new_version_operator_pragma_less_than(element)
    }
    fn make_version_operator_pragma_greater_than(
        &self,
        element: nodes::PragmaGreaterThan,
    ) -> nodes::VersionOperator {
        nodes::new_version_operator_pragma_greater_than(element)
    }
    fn make_version_operator_pragma_less_than_equal(
        &self,
        element: nodes::PragmaLessThanEqual,
    ) -> nodes::VersionOperator {
        nodes::new_version_operator_pragma_less_than_equal(element)
    }
    fn make_version_operator_pragma_greater_than_equal(
        &self,
        element: nodes::PragmaGreaterThanEqual,
    ) -> nodes::VersionOperator {
        nodes::new_version_operator_pragma_greater_than_equal(element)
    }

    fn make_yul_expression_yul_function_call_expression(
        &self,
        element: nodes::YulFunctionCallExpression,
    ) -> nodes::YulExpression {
        nodes::new_yul_expression_yul_function_call_expression(element)
    }
    fn make_yul_expression_yul_literal(&self, element: nodes::YulLiteral) -> nodes::YulExpression {
        nodes::new_yul_expression_yul_literal(element)
    }
    fn make_yul_expression_yul_path(&self, element: nodes::YulPath) -> nodes::YulExpression {
        nodes::new_yul_expression_yul_path(element)
    }

    fn make_yul_literal_yul_true_keyword(
        &self,
        element: nodes::YulTrueKeyword,
    ) -> nodes::YulLiteral {
        nodes::new_yul_literal_yul_true_keyword(element)
    }
    fn make_yul_literal_yul_false_keyword(
        &self,
        element: nodes::YulFalseKeyword,
    ) -> nodes::YulLiteral {
        nodes::new_yul_literal_yul_false_keyword(element)
    }
    fn make_yul_literal_yul_decimal_literal(
        &self,
        element: nodes::YulDecimalLiteral,
    ) -> nodes::YulLiteral {
        nodes::new_yul_literal_yul_decimal_literal(element)
    }
    fn make_yul_literal_yul_hex_literal(&self, element: nodes::YulHexLiteral) -> nodes::YulLiteral {
        nodes::new_yul_literal_yul_hex_literal(element)
    }
    fn make_yul_literal_yul_hex_string_literal(
        &self,
        element: nodes::YulHexStringLiteral,
    ) -> nodes::YulLiteral {
        nodes::new_yul_literal_yul_hex_string_literal(element)
    }
    fn make_yul_literal_yul_string_literal(
        &self,
        element: nodes::YulStringLiteral,
    ) -> nodes::YulLiteral {
        nodes::new_yul_literal_yul_string_literal(element)
    }

    fn make_yul_statement_yul_block(&self, element: nodes::YulBlock) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_block(element)
    }
    fn make_yul_statement_yul_function_definition(
        &self,
        element: nodes::YulFunctionDefinition,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_function_definition(element)
    }
    fn make_yul_statement_yul_if_statement(
        &self,
        element: nodes::YulIfStatement,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_if_statement(element)
    }
    fn make_yul_statement_yul_for_statement(
        &self,
        element: nodes::YulForStatement,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_for_statement(element)
    }
    fn make_yul_statement_yul_switch_statement(
        &self,
        element: nodes::YulSwitchStatement,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_switch_statement(element)
    }
    fn make_yul_statement_yul_leave_statement(
        &self,
        element: nodes::YulLeaveStatement,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_leave_statement(element)
    }
    fn make_yul_statement_yul_break_statement(
        &self,
        element: nodes::YulBreakStatement,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_break_statement(element)
    }
    fn make_yul_statement_yul_continue_statement(
        &self,
        element: nodes::YulContinueStatement,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_continue_statement(element)
    }
    fn make_yul_statement_yul_variable_assignment_statement(
        &self,
        element: nodes::YulVariableAssignmentStatement,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_variable_assignment_statement(element)
    }
    fn make_yul_statement_yul_variable_declaration_statement(
        &self,
        element: nodes::YulVariableDeclarationStatement,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_variable_declaration_statement(element)
    }
    fn make_yul_statement_yul_expression(
        &self,
        element: nodes::YulExpression,
    ) -> nodes::YulStatement {
        nodes::new_yul_statement_yul_expression(element)
    }

    fn make_yul_switch_case_yul_default_case(
        &self,
        element: nodes::YulDefaultCase,
    ) -> nodes::YulSwitchCase {
        nodes::new_yul_switch_case_yul_default_case(element)
    }
    fn make_yul_switch_case_yul_value_case(
        &self,
        element: nodes::YulValueCase,
    ) -> nodes::YulSwitchCase {
        nodes::new_yul_switch_case_yul_value_case(element)
    }

    // Collection factory methods
    fn make_array_values(&self, elements: Vec<nodes::Expression>) -> nodes::ArrayValues {
        nodes::new_array_values(elements)
    }
    fn make_call_options(&self, elements: Vec<nodes::NamedArgument>) -> nodes::CallOptions {
        nodes::new_call_options(elements)
    }
    fn make_catch_clauses(&self, elements: Vec<nodes::CatchClause>) -> nodes::CatchClauses {
        nodes::new_catch_clauses(elements)
    }
    fn make_constructor_attributes(
        &self,
        elements: Vec<nodes::ConstructorAttribute>,
    ) -> nodes::ConstructorAttributes {
        nodes::new_constructor_attributes(elements)
    }
    fn make_contract_members(
        &self,
        elements: Vec<nodes::ContractMember>,
    ) -> nodes::ContractMembers {
        nodes::new_contract_members(elements)
    }
    fn make_contract_specifiers(
        &self,
        elements: Vec<nodes::ContractSpecifier>,
    ) -> nodes::ContractSpecifiers {
        nodes::new_contract_specifiers(elements)
    }
    fn make_enum_members(&self, elements: Vec<nodes::Identifier>) -> nodes::EnumMembers {
        nodes::new_enum_members(elements)
    }
    fn make_error_parameters(
        &self,
        elements: Vec<nodes::ErrorParameter>,
    ) -> nodes::ErrorParameters {
        nodes::new_error_parameters(elements)
    }
    fn make_event_parameters(
        &self,
        elements: Vec<nodes::EventParameter>,
    ) -> nodes::EventParameters {
        nodes::new_event_parameters(elements)
    }
    fn make_fallback_function_attributes(
        &self,
        elements: Vec<nodes::FallbackFunctionAttribute>,
    ) -> nodes::FallbackFunctionAttributes {
        nodes::new_fallback_function_attributes(elements)
    }
    fn make_function_attributes(
        &self,
        elements: Vec<nodes::FunctionAttribute>,
    ) -> nodes::FunctionAttributes {
        nodes::new_function_attributes(elements)
    }
    fn make_function_type_attributes(
        &self,
        elements: Vec<nodes::FunctionTypeAttribute>,
    ) -> nodes::FunctionTypeAttributes {
        nodes::new_function_type_attributes(elements)
    }
    fn make_hex_string_literals(
        &self,
        elements: Vec<nodes::HexStringLiteral>,
    ) -> nodes::HexStringLiterals {
        nodes::new_hex_string_literals(elements)
    }
    fn make_identifier_path(
        &self,
        elements: Vec<nodes::IdentifierPathElement>,
    ) -> nodes::IdentifierPath {
        nodes::new_identifier_path(elements)
    }
    fn make_import_deconstruction_symbols(
        &self,
        elements: Vec<nodes::ImportDeconstructionSymbol>,
    ) -> nodes::ImportDeconstructionSymbols {
        nodes::new_import_deconstruction_symbols(elements)
    }
    fn make_inheritance_types(
        &self,
        elements: Vec<nodes::InheritanceType>,
    ) -> nodes::InheritanceTypes {
        nodes::new_inheritance_types(elements)
    }
    fn make_interface_members(
        &self,
        elements: Vec<nodes::ContractMember>,
    ) -> nodes::InterfaceMembers {
        nodes::new_interface_members(elements)
    }
    fn make_library_members(&self, elements: Vec<nodes::ContractMember>) -> nodes::LibraryMembers {
        nodes::new_library_members(elements)
    }
    fn make_modifier_attributes(
        &self,
        elements: Vec<nodes::ModifierAttribute>,
    ) -> nodes::ModifierAttributes {
        nodes::new_modifier_attributes(elements)
    }
    fn make_multi_typed_declaration_elements(
        &self,
        elements: Vec<nodes::MultiTypedDeclarationElement>,
    ) -> nodes::MultiTypedDeclarationElements {
        nodes::new_multi_typed_declaration_elements(elements)
    }
    fn make_named_arguments(&self, elements: Vec<nodes::NamedArgument>) -> nodes::NamedArguments {
        nodes::new_named_arguments(elements)
    }
    fn make_override_paths(&self, elements: Vec<nodes::IdentifierPath>) -> nodes::OverridePaths {
        nodes::new_override_paths(elements)
    }
    fn make_parameters(&self, elements: Vec<nodes::Parameter>) -> nodes::Parameters {
        nodes::new_parameters(elements)
    }
    fn make_positional_arguments(
        &self,
        elements: Vec<nodes::Expression>,
    ) -> nodes::PositionalArguments {
        nodes::new_positional_arguments(elements)
    }
    fn make_receive_function_attributes(
        &self,
        elements: Vec<nodes::ReceiveFunctionAttribute>,
    ) -> nodes::ReceiveFunctionAttributes {
        nodes::new_receive_function_attributes(elements)
    }
    fn make_simple_version_literal(
        &self,
        elements: Vec<nodes::VersionSpecifier>,
    ) -> nodes::SimpleVersionLiteral {
        nodes::new_simple_version_literal(elements)
    }
    fn make_source_unit_members(
        &self,
        elements: Vec<nodes::SourceUnitMember>,
    ) -> nodes::SourceUnitMembers {
        nodes::new_source_unit_members(elements)
    }
    fn make_state_variable_attributes(
        &self,
        elements: Vec<nodes::StateVariableAttribute>,
    ) -> nodes::StateVariableAttributes {
        nodes::new_state_variable_attributes(elements)
    }
    fn make_statements(&self, elements: Vec<nodes::Statement>) -> nodes::Statements {
        nodes::new_statements(elements)
    }
    fn make_string_literals(&self, elements: Vec<nodes::StringLiteral>) -> nodes::StringLiterals {
        nodes::new_string_literals(elements)
    }
    fn make_struct_members(&self, elements: Vec<nodes::StructMember>) -> nodes::StructMembers {
        nodes::new_struct_members(elements)
    }
    fn make_tuple_values(&self, elements: Vec<nodes::TupleValue>) -> nodes::TupleValues {
        nodes::new_tuple_values(elements)
    }
    fn make_unicode_string_literals(
        &self,
        elements: Vec<nodes::UnicodeStringLiteral>,
    ) -> nodes::UnicodeStringLiterals {
        nodes::new_unicode_string_literals(elements)
    }
    fn make_using_deconstruction_symbols(
        &self,
        elements: Vec<nodes::UsingDeconstructionSymbol>,
    ) -> nodes::UsingDeconstructionSymbols {
        nodes::new_using_deconstruction_symbols(elements)
    }
    fn make_version_expression_set(
        &self,
        elements: Vec<nodes::VersionExpression>,
    ) -> nodes::VersionExpressionSet {
        nodes::new_version_expression_set(elements)
    }
    fn make_version_expression_sets(
        &self,
        elements: Vec<nodes::VersionExpressionSet>,
    ) -> nodes::VersionExpressionSets {
        nodes::new_version_expression_sets(elements)
    }
    fn make_yul_arguments(&self, elements: Vec<nodes::YulExpression>) -> nodes::YulArguments {
        nodes::new_yul_arguments(elements)
    }
    fn make_yul_flags(&self, elements: Vec<nodes::YulStringLiteral>) -> nodes::YulFlags {
        nodes::new_yul_flags(elements)
    }
    fn make_yul_parameters(&self, elements: Vec<nodes::YulIdentifier>) -> nodes::YulParameters {
        nodes::new_yul_parameters(elements)
    }
    fn make_yul_path(&self, elements: Vec<nodes::YulIdentifier>) -> nodes::YulPath {
        nodes::new_yul_path(elements)
    }
    fn make_yul_paths(&self, elements: Vec<nodes::YulPath>) -> nodes::YulPaths {
        nodes::new_yul_paths(elements)
    }
    fn make_yul_statements(&self, elements: Vec<nodes::YulStatement>) -> nodes::YulStatements {
        nodes::new_yul_statements(elements)
    }
    fn make_yul_switch_cases(&self, elements: Vec<nodes::YulSwitchCase>) -> nodes::YulSwitchCases {
        nodes::new_yul_switch_cases(elements)
    }
    fn make_yul_variable_names(
        &self,
        elements: Vec<nodes::YulIdentifier>,
    ) -> nodes::YulVariableNames {
        nodes::new_yul_variable_names(elements)
    }

    // ==============================
    // === Special Helper Methods ===
    // ==============================

    fn make_type_name_from_index_access_path(
        &self,
        index_access_path: parser_helpers::IndexAccessPath<Self>,
    ) -> nodes::TypeName {
        let parser_helpers::IndexAccessPath { path, indices } = index_access_path;

        let mut type_name = match path {
            parser_helpers::Path::IdentifierPath(path) => {
                nodes::new_type_name_identifier_path(path)
            }
            parser_helpers::Path::ElementaryType(elem_type) => {
                nodes::new_type_name_elementary_type(elem_type)
            }
        };

        for index in indices {
            assert!(
                index.end.is_none(),
                "Slicing is not supported in type names yet"
            );
            let array_type = nodes::new_array_type_name(
                type_name,
                index.open_bracket,
                index.start,
                index.close_bracket,
            );
            type_name = nodes::new_type_name_array_type_name(array_type);
        }

        type_name
    }

    fn make_expression_from_index_access_path(
        &self,
        index_access_path: parser_helpers::IndexAccessPath<Self>,
    ) -> nodes::Expression {
        let parser_helpers::IndexAccessPath { path, indices } = index_access_path;

        let mut expression = match path {
            parser_helpers::Path::IdentifierPath(path) => {
                self.make_expression_from_identifier_path(path)
            }
            parser_helpers::Path::ElementaryType(elem_type) => {
                nodes::new_expression_elementary_type(elem_type)
            }
        };

        for index in indices {
            let array_expression = nodes::new_index_access_expression(
                expression,
                index.open_bracket,
                index.start,
                index.end,
                index.close_bracket,
            );
            expression = nodes::new_expression_index_access_expression(array_expression);
        }

        expression
    }

    fn make_expression_from_identifier_path(
        &self,
        identifier_path: nodes::IdentifierPath,
    ) -> nodes::Expression {
        identifier_path
            .elements
            .into_iter()
            .fold(None, |acc, id| {
                match acc {
                    None => Some(match id {
                        nodes::IdentifierPathElement::AddressKeyword(_) => {
                            // TODO(v2) we should validate that this is not the case and fail gracefully
                            // instead of returning a wrong identifier
                            nodes::new_expression_identifier(nodes::Identifier { range: 0..0 })
                        }
                        nodes::IdentifierPathElement::Identifier(id) => {
                            nodes::new_expression_identifier(id)
                        }
                    }),
                    Some(acc) => Some(nodes::new_expression_member_access_expression(
                        nodes::new_member_access_expression(
                            acc,
                            // TODO(v2) use real range
                            nodes::Period { range: 0..0 },
                            id,
                        ),
                    )),
                }
            })
            .expect("IdentifierPath should have at least one element!")
    }

    fn extract_extra_attributes(
        &self,
        fun_type: nodes::FunctionType,
    ) -> (nodes::FunctionType, Vec<nodes::StateVariableAttribute>) {
        let nodes::FunctionTypeStruct {
            function_keyword,
            parameters,
            attributes,
            returns,
        } = Rc::unwrap_or_clone(fun_type);
        let mut vec = attributes.elements;

        let mut seen_internal = false;
        let mut seen_private = false;
        let mut seen_public = false;
        let mut duplicate_found = false;

        let extracted = vec.extract_if(.., |attr| {
            if duplicate_found {
                true
            } else {
                let seen = match attr {
                    nodes::FunctionTypeAttribute::InternalKeyword(_) => &mut seen_internal,
                    nodes::FunctionTypeAttribute::PrivateKeyword(_) => &mut seen_private,
                    nodes::FunctionTypeAttribute::PublicKeyword(_) => &mut seen_public,
                    _ => return false,
                };

                if *seen {
                    duplicate_found = true;
                    true
                } else {
                    *seen = true;
                    false
                }
            }
        });

        let extra_attributes: Vec<nodes::StateVariableAttribute> = extracted
            .filter_map(|attr| match attr {
                nodes::FunctionTypeAttribute::InternalKeyword(terminal) => {
                    Some(nodes::StateVariableAttribute::InternalKeyword(terminal))
                }
                nodes::FunctionTypeAttribute::PrivateKeyword(terminal) => {
                    Some(nodes::StateVariableAttribute::PrivateKeyword(terminal))
                }
                nodes::FunctionTypeAttribute::PublicKeyword(terminal) => {
                    Some(nodes::StateVariableAttribute::PublicKeyword(terminal))
                }
                _ => None,
            })
            .collect();

        let new_fun_type = nodes::new_function_type(
            function_keyword,
            parameters,
            nodes::new_function_type_attributes(vec),
            returns,
        );

        (new_fun_type, extra_attributes)
    }
}
