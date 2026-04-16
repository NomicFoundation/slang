// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::ops::Range;
use std::rc::Rc;

use slang_solidity_v2_parser::parser::consumer::ParserConsumer;
use slang_solidity_v2_parser::parser::parser_helpers;

use super::{intermediate, IrConsumer};
use crate::ir::nodes as output;

impl<'source> ParserConsumer for IrConsumer<'source> {
    // ============================================
    // === Associated Types: Terminals ===
    // ============================================

    type ABIEncoderV2Keyword = Range<usize>;

    type AbicoderKeyword = Range<usize>;

    type AbicoderV1Keyword = Range<usize>;

    type AbicoderV2Keyword = Range<usize>;

    type AbstractKeyword = Range<usize>;

    type AddressKeyword = Range<usize>;

    type AfterKeyword = Range<usize>;

    type AliasKeyword = Range<usize>;

    type Ampersand = Range<usize>;

    type AmpersandAmpersand = Range<usize>;

    type AmpersandEqual = Range<usize>;

    type AnonymousKeyword = Range<usize>;

    type ApplyKeyword = Range<usize>;

    type AsKeyword = Range<usize>;

    type AssemblyKeyword = Range<usize>;

    type Asterisk = Range<usize>;

    type AsteriskAsterisk = Range<usize>;

    type AsteriskEqual = Range<usize>;

    type AtKeyword = Range<usize>;

    type AutoKeyword = Range<usize>;

    type Bang = Range<usize>;

    type BangEqual = Range<usize>;

    type Bar = Range<usize>;

    type BarBar = Range<usize>;

    type BarEqual = Range<usize>;

    type BoolKeyword = Range<usize>;

    type BreakKeyword = Range<usize>;

    type ByteKeyword = Range<usize>;

    type BytesKeyword = output::BytesKeyword;

    type CallDataKeyword = Range<usize>;

    type Caret = Range<usize>;

    type CaretEqual = Range<usize>;

    type CaseKeyword = Range<usize>;

    type CatchKeyword = Range<usize>;

    type CloseBrace = Range<usize>;

    type CloseBracket = Range<usize>;

    type CloseParen = Range<usize>;

    type Colon = Range<usize>;

    type Comma = Range<usize>;

    type ConstantKeyword = Range<usize>;

    type ConstructorKeyword = Range<usize>;

    type ContinueKeyword = Range<usize>;

    type ContractKeyword = Range<usize>;

    type CopyOfKeyword = Range<usize>;

    type DaysKeyword = Range<usize>;

    type DecimalLiteral = output::DecimalLiteral;

    type DefaultKeyword = Range<usize>;

    type DefineKeyword = Range<usize>;

    type DeleteKeyword = Range<usize>;

    type DoKeyword = Range<usize>;

    type ElseKeyword = Range<usize>;

    type EmitKeyword = Range<usize>;

    type EnumKeyword = Range<usize>;

    type Equal = Range<usize>;

    type EqualEqual = Range<usize>;

    type EqualGreaterThan = Range<usize>;

    type ErrorKeyword = Range<usize>;

    type EtherKeyword = Range<usize>;

    type EventKeyword = Range<usize>;

    type ExperimentalKeyword = Range<usize>;

    type ExternalKeyword = Range<usize>;

    type FallbackKeyword = Range<usize>;

    type FalseKeyword = Range<usize>;

    type FinalKeyword = Range<usize>;

    type FixedKeyword = output::FixedKeyword;

    type ForKeyword = Range<usize>;

    type FromKeyword = Range<usize>;

    type FunctionKeyword = Range<usize>;

    type GlobalKeyword = Range<usize>;

    type GreaterThan = Range<usize>;

    type GreaterThanEqual = Range<usize>;

    type GreaterThanGreaterThan = Range<usize>;

    type GreaterThanGreaterThanEqual = Range<usize>;

    type GreaterThanGreaterThanGreaterThan = Range<usize>;

    type GreaterThanGreaterThanGreaterThanEqual = Range<usize>;

    type GweiKeyword = Range<usize>;

    type HexKeyword = Range<usize>;

    type HexLiteral = output::HexLiteral;

    type HexStringLiteral = output::HexStringLiteral;

    type HoursKeyword = Range<usize>;

    type Identifier = output::Identifier;

    type IfKeyword = Range<usize>;

    type ImmutableKeyword = Range<usize>;

    type ImplementsKeyword = Range<usize>;

    type ImportKeyword = Range<usize>;

    type InKeyword = Range<usize>;

    type IndexedKeyword = Range<usize>;

    type InlineKeyword = Range<usize>;

    type IntKeyword = output::IntKeyword;

    type InterfaceKeyword = Range<usize>;

    type InternalKeyword = Range<usize>;

    type IsKeyword = Range<usize>;

    type LayoutKeyword = Range<usize>;

    type LessThan = Range<usize>;

    type LessThanEqual = Range<usize>;

    type LessThanLessThan = Range<usize>;

    type LessThanLessThanEqual = Range<usize>;

    type LetKeyword = Range<usize>;

    type LibraryKeyword = Range<usize>;

    type MacroKeyword = Range<usize>;

    type MappingKeyword = Range<usize>;

    type MatchKeyword = Range<usize>;

    type MemoryKeyword = Range<usize>;

    type Minus = Range<usize>;

    type MinusEqual = Range<usize>;

    type MinusMinus = Range<usize>;

    type MinutesKeyword = Range<usize>;

    type ModifierKeyword = Range<usize>;

    type MutableKeyword = Range<usize>;

    type NewKeyword = Range<usize>;

    type NullKeyword = Range<usize>;

    type OfKeyword = Range<usize>;

    type OpenBrace = Range<usize>;

    type OpenBracket = Range<usize>;

    type OpenParen = Range<usize>;

    type OverrideKeyword = Range<usize>;

    type PartialKeyword = Range<usize>;

    type PayableKeyword = Range<usize>;

    type Percent = Range<usize>;

    type PercentEqual = Range<usize>;

    type Period = Range<usize>;

    type Plus = Range<usize>;

    type PlusEqual = Range<usize>;

    type PlusPlus = Range<usize>;

    type PragmaBarBar = Range<usize>;

    type PragmaCaret = Range<usize>;

    type PragmaEqual = Range<usize>;

    type PragmaGreaterThan = Range<usize>;

    type PragmaGreaterThanEqual = Range<usize>;

    type PragmaKeyword = Range<usize>;

    type PragmaLessThan = Range<usize>;

    type PragmaLessThanEqual = Range<usize>;

    type PragmaMinus = Range<usize>;

    type PragmaPeriod = Range<usize>;

    type PragmaSemicolon = Range<usize>;

    type PragmaStringLiteral = output::StringLiteral;

    type PragmaTilde = Range<usize>;

    type PrivateKeyword = Range<usize>;

    type PromiseKeyword = Range<usize>;

    type PublicKeyword = Range<usize>;

    type PureKeyword = Range<usize>;

    type QuestionMark = Range<usize>;

    type ReceiveKeyword = Range<usize>;

    type ReferenceKeyword = Range<usize>;

    type RelocatableKeyword = Range<usize>;

    type ReturnKeyword = Range<usize>;

    type ReturnsKeyword = Range<usize>;

    type RevertKeyword = Range<usize>;

    type SMTCheckerKeyword = Range<usize>;

    type SealedKeyword = Range<usize>;

    type SecondsKeyword = Range<usize>;

    type Semicolon = Range<usize>;

    type SizeOfKeyword = Range<usize>;

    type Slash = Range<usize>;

    type SlashEqual = Range<usize>;

    type SolidityKeyword = Range<usize>;

    type StaticKeyword = Range<usize>;

    type StorageKeyword = Range<usize>;

    type StringKeyword = Range<usize>;

    type StringLiteral = output::StringLiteral;

    type StructKeyword = Range<usize>;

    type SuperKeyword = Range<usize>;

    type SupportsKeyword = Range<usize>;

    type SwitchKeyword = Range<usize>;

    type ThisKeyword = Range<usize>;

    type ThrowKeyword = Range<usize>;

    type Tilde = Range<usize>;

    type TransientKeyword = Range<usize>;

    type TrueKeyword = Range<usize>;

    type TryKeyword = Range<usize>;

    type TypeDefKeyword = Range<usize>;

    type TypeKeyword = Range<usize>;

    type TypeOfKeyword = Range<usize>;

    type UfixedKeyword = output::UfixedKeyword;

    type UintKeyword = output::UintKeyword;

    type UncheckedKeyword = Range<usize>;

    type UnicodeStringLiteral = output::UnicodeStringLiteral;

    type UsingKeyword = Range<usize>;

    type VarKeyword = Range<usize>;

    type VersionSpecifier = output::VersionSpecifier;

    type ViewKeyword = Range<usize>;

    type VirtualKeyword = Range<usize>;

    type WeeksKeyword = Range<usize>;

    type WeiKeyword = Range<usize>;

    type WhileKeyword = Range<usize>;

    type YearsKeyword = Range<usize>;

    type YulBreakKeyword = Range<usize>;

    type YulCaseKeyword = Range<usize>;

    type YulCloseBrace = Range<usize>;

    type YulCloseParen = Range<usize>;

    type YulColonEqual = Range<usize>;

    type YulComma = Range<usize>;

    type YulContinueKeyword = Range<usize>;

    type YulDecimalLiteral = output::DecimalLiteral;

    type YulDefaultKeyword = Range<usize>;

    type YulFalseKeyword = Range<usize>;

    type YulForKeyword = Range<usize>;

    type YulFunctionKeyword = Range<usize>;

    type YulHexKeyword = Range<usize>;

    type YulHexLiteral = output::HexLiteral;

    type YulHexStringLiteral = output::HexStringLiteral;

    type YulIdentifier = output::Identifier;

    type YulIfKeyword = Range<usize>;

    type YulLeaveKeyword = Range<usize>;

    type YulLetKeyword = Range<usize>;

    type YulMinusGreaterThan = Range<usize>;

    type YulOpenBrace = Range<usize>;

    type YulOpenParen = Range<usize>;

    type YulPeriod = Range<usize>;

    type YulStringLiteral = output::StringLiteral;

    type YulSuperKeyword = Range<usize>;

    type YulSwitchKeyword = Range<usize>;

    type YulThisKeyword = Range<usize>;

    type YulTrueKeyword = Range<usize>;

    // ============================================
    // === Associated Types: Sequences ===
    // ============================================

    type AbicoderPragma = output::AbicoderPragma;

    type AdditiveExpression = output::AdditiveExpression;

    type AddressType = output::AddressType;

    type AndExpression = output::AndExpression;

    type ArrayExpression = output::ArrayExpression;

    type ArrayTypeName = output::ArrayTypeName;

    type AssemblyStatement = output::AssemblyStatement;

    type AssignmentExpression = output::AssignmentExpression;

    type BitwiseAndExpression = output::BitwiseAndExpression;

    type BitwiseOrExpression = output::BitwiseOrExpression;

    type BitwiseXorExpression = output::BitwiseXorExpression;

    type Block = output::Block;

    type BreakStatement = output::BreakStatement;

    type CallOptionsExpression = output::CallOptionsExpression;

    type CatchClause = output::CatchClause;

    type CatchClauseError = output::CatchClauseError;

    type ConditionalExpression = output::ConditionalExpression;

    type ConstantDefinition = output::ConstantDefinition;

    type ConstructorDefinition = output::FunctionDefinition;

    type ContinueStatement = output::ContinueStatement;

    type ContractDefinition = output::ContractDefinition;

    type DecimalNumberExpression = output::DecimalNumberExpression;

    type DoWhileStatement = output::DoWhileStatement;

    type ElseBranch = output::Statement;

    type EmitStatement = output::EmitStatement;

    type EnumDefinition = output::EnumDefinition;

    type EqualityExpression = output::EqualityExpression;

    type ErrorDefinition = output::ErrorDefinition;

    type ErrorParameter = output::Parameter;

    type ErrorParametersDeclaration = output::Parameters;

    type EventDefinition = output::EventDefinition;

    type EventParameter = output::Parameter;

    type EventParametersDeclaration = output::Parameters;

    type ExperimentalPragma = output::ExperimentalPragma;

    type ExponentiationExpression = output::ExponentiationExpression;

    type ExpressionStatement = output::ExpressionStatement;

    type FallbackFunctionDefinition = output::FunctionDefinition;

    type ForStatement = output::ForStatement;

    type FunctionCallExpression = output::FunctionCallExpression;

    type FunctionDefinition = output::FunctionDefinition;

    type FunctionType = intermediate::IrFunctionType;

    type HexNumberExpression = output::HexNumberExpression;

    type IfStatement = output::IfStatement;

    type ImportAlias = output::Identifier;

    type ImportDeconstruction = output::ImportDeconstruction;

    type ImportDeconstructionSymbol = output::ImportDeconstructionSymbol;

    type ImportDirective = output::ImportClause;

    type IndexAccessEnd = Option<output::Expression>;

    type IndexAccessExpression = output::IndexAccessExpression;

    type InequalityExpression = output::InequalityExpression;

    type InheritanceSpecifier = output::InheritanceTypes;

    type InheritanceType = output::InheritanceType;

    type InterfaceDefinition = output::InterfaceDefinition;

    type LibraryDefinition = output::LibraryDefinition;

    type MappingKey = output::Parameter;

    type MappingType = output::MappingType;

    type MappingValue = output::Parameter;

    type MemberAccessExpression = output::MemberAccessExpression;

    type ModifierDefinition = output::FunctionDefinition;

    type ModifierInvocation = output::ModifierInvocation;

    type MultiTypedDeclaration = output::MultiTypedDeclaration;

    type MultiTypedDeclarationElement = output::MultiTypedDeclarationElement;

    type MultiplicativeExpression = output::MultiplicativeExpression;

    type NamedArgument = output::NamedArgument;

    type NamedArgumentGroup = output::NamedArguments;

    type NamedArgumentsDeclaration = output::NamedArguments;

    type NamedImport = output::PathImport;

    type NewExpression = output::NewExpression;

    type OrExpression = output::OrExpression;

    type OverridePathsDeclaration = output::OverridePaths;

    type OverrideSpecifier = output::OverridePaths;

    type Parameter = output::Parameter;

    type ParametersDeclaration = output::Parameters;

    type PathImport = output::PathImport;

    type PositionalArgumentsDeclaration = output::PositionalArguments;

    type PostfixExpression = output::PostfixExpression;

    type PragmaDirective = output::PragmaDirective;

    type PrefixExpression = output::PrefixExpression;

    type ReceiveFunctionDefinition = output::FunctionDefinition;

    type ReturnStatement = output::ReturnStatement;

    type ReturnsDeclaration = output::Parameters;

    type RevertStatement = output::RevertStatement;

    type ShiftExpression = output::ShiftExpression;

    type SingleTypedDeclaration = output::SingleTypedDeclaration;

    type SourceUnit = output::SourceUnit;

    type StateVariableDefinition = output::StateVariableDefinition;

    type StateVariableDefinitionValue = output::Expression;

    type StorageLayoutSpecifier = output::Expression;

    type StructDefinition = output::StructDefinition;

    type StructMember = output::StructMember;

    type TryStatement = output::TryStatement;

    type TupleExpression = output::TupleExpression;

    type TupleValue = output::TupleValue;

    type TypeExpression = output::TypeExpression;

    type UncheckedBlock = output::UncheckedBlock;

    type UserDefinedValueTypeDefinition = output::UserDefinedValueTypeDefinition;

    type UsingAlias = output::UsingOperator;

    type UsingDeconstruction = output::UsingDeconstruction;

    type UsingDeconstructionSymbol = output::UsingDeconstructionSymbol;

    type UsingDirective = output::UsingDirective;

    type VariableDeclaration = output::VariableDeclaration;

    type VariableDeclarationStatement = output::VariableDeclarationStatement;

    type VariableDeclarationValue = output::Expression;

    type VersionPragma = output::VersionPragma;

    type VersionRange = output::VersionRange;

    type VersionTerm = output::VersionTerm;

    type WhileStatement = output::WhileStatement;

    type YulBlock = output::YulBlock;

    type YulBreakStatement = output::YulBreakStatement;

    type YulContinueStatement = output::YulContinueStatement;

    type YulDefaultCase = output::YulDefaultCase;

    type YulFlagsDeclaration = output::YulFlags;

    type YulForStatement = output::YulForStatement;

    type YulFunctionCallExpression = output::YulFunctionCallExpression;

    type YulFunctionDefinition = output::YulFunctionDefinition;

    type YulIfStatement = output::YulIfStatement;

    type YulLeaveStatement = output::YulLeaveStatement;

    type YulParametersDeclaration = output::YulParameters;

    type YulReturnsDeclaration = output::YulVariableNames;

    type YulSwitchStatement = output::YulSwitchStatement;

    type YulValueCase = output::YulValueCase;

    type YulVariableAssignmentStatement = output::YulVariableAssignmentStatement;

    type YulVariableDeclarationStatement = output::YulVariableDeclarationStatement;

    type YulVariableDeclarationValue = output::YulVariableDeclarationValue;

    // ============================================
    // === Associated Types: Choices ===
    // ============================================

    type AbicoderVersion = output::AbicoderVersion;

    type ArgumentsDeclaration = output::ArgumentsDeclaration;

    type ConstructorAttribute = intermediate::IrConstructorAttribute;

    type ContractMember = output::ContractMember;

    type ContractSpecifier = intermediate::IrContractSpecifier;

    type ElementaryType = output::ElementaryType;

    type ExperimentalFeature = output::ExperimentalFeature;

    type Expression = output::Expression;

    type Expression_AdditiveExpression_Operator = output::Expression_AdditiveExpression_Operator;

    type Expression_AssignmentExpression_Operator =
        output::Expression_AssignmentExpression_Operator;

    type Expression_EqualityExpression_Operator = output::Expression_EqualityExpression_Operator;

    type Expression_InequalityExpression_Operator =
        output::Expression_InequalityExpression_Operator;

    type Expression_MultiplicativeExpression_Operator =
        output::Expression_MultiplicativeExpression_Operator;

    type Expression_PostfixExpression_Operator = output::Expression_PostfixExpression_Operator;

    type Expression_PrefixExpression_Operator = output::Expression_PrefixExpression_Operator;

    type Expression_ShiftExpression_Operator = output::Expression_ShiftExpression_Operator;

    type FallbackFunctionAttribute = intermediate::IrFallbackFunctionAttribute;

    type ForStatementCondition = output::ForStatementCondition;

    type ForStatementInitialization = output::ForStatementInitialization;

    type FunctionAttribute = intermediate::IrFunctionAttribute;

    type FunctionBody = Option<output::Block>;

    type FunctionName = (output::FunctionKind, Option<output::Identifier>);

    type FunctionTypeAttribute = intermediate::IrFunctionTypeAttribute;

    type IdentifierPathElement = output::Identifier;

    type ImportClause = output::ImportClause;

    type MappingKeyType = output::TypeName;

    type ModifierAttribute = intermediate::IrModifierAttribute;

    type NumberUnit = output::NumberUnit;

    type Pragma = output::Pragma;

    type ReceiveFunctionAttribute = intermediate::IrReceiveFunctionAttribute;

    type SourceUnitMember = output::SourceUnitMember;

    type StateVariableAttribute = intermediate::IrStateVariableAttribute;

    type Statement = output::Statement;

    type StorageLocation = output::StorageLocation;

    type StringExpression = output::StringExpression;

    type TypeName = output::TypeName;

    type UsingClause = output::UsingClause;

    type UsingOperator = output::UsingOperator;

    type UsingTarget = output::UsingTarget;

    type VariableDeclarationTarget = output::VariableDeclarationTarget;

    type VersionExpression = output::VersionExpression;

    type VersionLiteral = output::VersionLiteral;

    type VersionOperator = output::VersionOperator;

    type YulExpression = output::YulExpression;

    type YulLiteral = output::YulLiteral;

    type YulStatement = output::YulStatement;

    type YulSwitchCase = output::YulSwitchCase;

    // ============================================
    // === Associated Types: Collections ===
    // ============================================

    type ArrayValues = output::ArrayValues;

    type CallOptions = output::CallOptions;

    type CatchClauses = output::CatchClauses;

    type ConstructorAttributes = Vec<intermediate::IrConstructorAttribute>;

    type ContractMembers = output::ContractMembers;

    type ContractSpecifiers = Vec<intermediate::IrContractSpecifier>;

    type EnumMembers = output::EnumMembers;

    type ErrorParameters = output::Parameters;

    type EventParameters = output::Parameters;

    type FallbackFunctionAttributes = Vec<intermediate::IrFallbackFunctionAttribute>;

    type FunctionAttributes = Vec<intermediate::IrFunctionAttribute>;

    type FunctionTypeAttributes = Vec<intermediate::IrFunctionTypeAttribute>;

    type HexStringLiterals = output::HexStringLiterals;

    type IdentifierPath = output::IdentifierPath;

    type ImportDeconstructionSymbols = output::ImportDeconstructionSymbols;

    type InheritanceTypes = output::InheritanceTypes;

    type InterfaceMembers = output::InterfaceMembers;

    type LibraryMembers = output::LibraryMembers;

    type ModifierAttributes = Vec<intermediate::IrModifierAttribute>;

    type MultiTypedDeclarationElements = output::MultiTypedDeclarationElements;

    type NamedArguments = output::NamedArguments;

    type OverridePaths = output::OverridePaths;

    type Parameters = output::Parameters;

    type PositionalArguments = output::PositionalArguments;

    type ReceiveFunctionAttributes = Vec<intermediate::IrReceiveFunctionAttribute>;

    type SimpleVersionLiteral = output::SimpleVersionLiteral;

    type SourceUnitMembers = output::SourceUnitMembers;

    type StateVariableAttributes = Vec<intermediate::IrStateVariableAttribute>;

    type Statements = output::Statements;

    type StringLiterals = output::StringLiterals;

    type StructMembers = output::StructMembers;

    type TupleValues = output::TupleValues;

    type UnicodeStringLiterals = output::UnicodeStringLiterals;

    type UsingDeconstructionSymbols = output::UsingDeconstructionSymbols;

    type VersionExpressionSet = output::VersionExpressionSet;

    type VersionExpressionSets = output::VersionExpressionSets;

    type YulArguments = output::YulArguments;

    type YulFlags = output::YulFlags;

    type YulParameters = output::YulParameters;

    type YulPath = output::YulPath;

    type YulPaths = output::YulPaths;

    type YulStatements = output::YulStatements;

    type YulSwitchCases = output::YulSwitchCases;

    type YulVariableNames = output::YulVariableNames;

    // ============================================
    // === Terminal Factory Methods ===
    // ============================================

    fn make_abi_encoder_v2_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::ABIEncoderV2Keyword {
        range
    }

    fn make_abicoder_keyword(&self, range: Range<usize>, _source: &str) -> Self::AbicoderKeyword {
        range
    }

    fn make_abicoder_v1_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::AbicoderV1Keyword {
        range
    }

    fn make_abicoder_v2_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::AbicoderV2Keyword {
        range
    }

    fn make_abstract_keyword(&self, range: Range<usize>, _source: &str) -> Self::AbstractKeyword {
        range
    }

    fn make_address_keyword(&self, range: Range<usize>, _source: &str) -> Self::AddressKeyword {
        range
    }

    fn make_after_keyword(&self, range: Range<usize>, _source: &str) -> Self::AfterKeyword {
        range
    }

    fn make_alias_keyword(&self, range: Range<usize>, _source: &str) -> Self::AliasKeyword {
        range
    }

    fn make_ampersand(&self, range: Range<usize>, _source: &str) -> Self::Ampersand {
        range
    }

    fn make_ampersand_ampersand(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::AmpersandAmpersand {
        range
    }

    fn make_ampersand_equal(&self, range: Range<usize>, _source: &str) -> Self::AmpersandEqual {
        range
    }

    fn make_anonymous_keyword(&self, range: Range<usize>, _source: &str) -> Self::AnonymousKeyword {
        range
    }

    fn make_apply_keyword(&self, range: Range<usize>, _source: &str) -> Self::ApplyKeyword {
        range
    }

    fn make_as_keyword(&self, range: Range<usize>, _source: &str) -> Self::AsKeyword {
        range
    }

    fn make_assembly_keyword(&self, range: Range<usize>, _source: &str) -> Self::AssemblyKeyword {
        range
    }

    fn make_asterisk(&self, range: Range<usize>, _source: &str) -> Self::Asterisk {
        range
    }

    fn make_asterisk_asterisk(&self, range: Range<usize>, _source: &str) -> Self::AsteriskAsterisk {
        range
    }

    fn make_asterisk_equal(&self, range: Range<usize>, _source: &str) -> Self::AsteriskEqual {
        range
    }

    fn make_at_keyword(&self, range: Range<usize>, _source: &str) -> Self::AtKeyword {
        range
    }

    fn make_auto_keyword(&self, range: Range<usize>, _source: &str) -> Self::AutoKeyword {
        range
    }

    fn make_bang(&self, range: Range<usize>, _source: &str) -> Self::Bang {
        range
    }

    fn make_bang_equal(&self, range: Range<usize>, _source: &str) -> Self::BangEqual {
        range
    }

    fn make_bar(&self, range: Range<usize>, _source: &str) -> Self::Bar {
        range
    }

    fn make_bar_bar(&self, range: Range<usize>, _source: &str) -> Self::BarBar {
        range
    }

    fn make_bar_equal(&self, range: Range<usize>, _source: &str) -> Self::BarEqual {
        range
    }

    fn make_bool_keyword(&self, range: Range<usize>, _source: &str) -> Self::BoolKeyword {
        range
    }

    fn make_break_keyword(&self, range: Range<usize>, _source: &str) -> Self::BreakKeyword {
        range
    }

    fn make_byte_keyword(&self, range: Range<usize>, _source: &str) -> Self::ByteKeyword {
        range
    }

    fn make_bytes_keyword(&self, range: Range<usize>, source: &str) -> Self::BytesKeyword {
        Rc::new(output::BytesKeywordStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_call_data_keyword(&self, range: Range<usize>, _source: &str) -> Self::CallDataKeyword {
        range
    }

    fn make_caret(&self, range: Range<usize>, _source: &str) -> Self::Caret {
        range
    }

    fn make_caret_equal(&self, range: Range<usize>, _source: &str) -> Self::CaretEqual {
        range
    }

    fn make_case_keyword(&self, range: Range<usize>, _source: &str) -> Self::CaseKeyword {
        range
    }

    fn make_catch_keyword(&self, range: Range<usize>, _source: &str) -> Self::CatchKeyword {
        range
    }

    fn make_close_brace(&self, range: Range<usize>, _source: &str) -> Self::CloseBrace {
        range
    }

    fn make_close_bracket(&self, range: Range<usize>, _source: &str) -> Self::CloseBracket {
        range
    }

    fn make_close_paren(&self, range: Range<usize>, _source: &str) -> Self::CloseParen {
        range
    }

    fn make_colon(&self, range: Range<usize>, _source: &str) -> Self::Colon {
        range
    }

    fn make_comma(&self, range: Range<usize>, _source: &str) -> Self::Comma {
        range
    }

    fn make_constant_keyword(&self, range: Range<usize>, _source: &str) -> Self::ConstantKeyword {
        range
    }

    fn make_constructor_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::ConstructorKeyword {
        range
    }

    fn make_continue_keyword(&self, range: Range<usize>, _source: &str) -> Self::ContinueKeyword {
        range
    }

    fn make_contract_keyword(&self, range: Range<usize>, _source: &str) -> Self::ContractKeyword {
        range
    }

    fn make_copy_of_keyword(&self, range: Range<usize>, _source: &str) -> Self::CopyOfKeyword {
        range
    }

    fn make_days_keyword(&self, range: Range<usize>, _source: &str) -> Self::DaysKeyword {
        range
    }

    fn make_decimal_literal(&self, range: Range<usize>, source: &str) -> Self::DecimalLiteral {
        Rc::new(output::DecimalLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_default_keyword(&self, range: Range<usize>, _source: &str) -> Self::DefaultKeyword {
        range
    }

    fn make_define_keyword(&self, range: Range<usize>, _source: &str) -> Self::DefineKeyword {
        range
    }

    fn make_delete_keyword(&self, range: Range<usize>, _source: &str) -> Self::DeleteKeyword {
        range
    }

    fn make_do_keyword(&self, range: Range<usize>, _source: &str) -> Self::DoKeyword {
        range
    }

    fn make_else_keyword(&self, range: Range<usize>, _source: &str) -> Self::ElseKeyword {
        range
    }

    fn make_emit_keyword(&self, range: Range<usize>, _source: &str) -> Self::EmitKeyword {
        range
    }

    fn make_enum_keyword(&self, range: Range<usize>, _source: &str) -> Self::EnumKeyword {
        range
    }

    fn make_equal(&self, range: Range<usize>, _source: &str) -> Self::Equal {
        range
    }

    fn make_equal_equal(&self, range: Range<usize>, _source: &str) -> Self::EqualEqual {
        range
    }

    fn make_equal_greater_than(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::EqualGreaterThan {
        range
    }

    fn make_error_keyword(&self, range: Range<usize>, _source: &str) -> Self::ErrorKeyword {
        range
    }

    fn make_ether_keyword(&self, range: Range<usize>, _source: &str) -> Self::EtherKeyword {
        range
    }

    fn make_event_keyword(&self, range: Range<usize>, _source: &str) -> Self::EventKeyword {
        range
    }

    fn make_experimental_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::ExperimentalKeyword {
        range
    }

    fn make_external_keyword(&self, range: Range<usize>, _source: &str) -> Self::ExternalKeyword {
        range
    }

    fn make_fallback_keyword(&self, range: Range<usize>, _source: &str) -> Self::FallbackKeyword {
        range
    }

    fn make_false_keyword(&self, range: Range<usize>, _source: &str) -> Self::FalseKeyword {
        range
    }

    fn make_final_keyword(&self, range: Range<usize>, _source: &str) -> Self::FinalKeyword {
        range
    }

    fn make_fixed_keyword(&self, range: Range<usize>, source: &str) -> Self::FixedKeyword {
        Rc::new(output::FixedKeywordStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_for_keyword(&self, range: Range<usize>, _source: &str) -> Self::ForKeyword {
        range
    }

    fn make_from_keyword(&self, range: Range<usize>, _source: &str) -> Self::FromKeyword {
        range
    }

    fn make_function_keyword(&self, range: Range<usize>, _source: &str) -> Self::FunctionKeyword {
        range
    }

    fn make_global_keyword(&self, range: Range<usize>, _source: &str) -> Self::GlobalKeyword {
        range
    }

    fn make_greater_than(&self, range: Range<usize>, _source: &str) -> Self::GreaterThan {
        range
    }

    fn make_greater_than_equal(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::GreaterThanEqual {
        range
    }

    fn make_greater_than_greater_than(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::GreaterThanGreaterThan {
        range
    }

    fn make_greater_than_greater_than_equal(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::GreaterThanGreaterThanEqual {
        range
    }

    fn make_greater_than_greater_than_greater_than(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::GreaterThanGreaterThanGreaterThan {
        range
    }

    fn make_greater_than_greater_than_greater_than_equal(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::GreaterThanGreaterThanGreaterThanEqual {
        range
    }

    fn make_gwei_keyword(&self, range: Range<usize>, _source: &str) -> Self::GweiKeyword {
        range
    }

    fn make_hex_keyword(&self, range: Range<usize>, _source: &str) -> Self::HexKeyword {
        range
    }

    fn make_hex_literal(&self, range: Range<usize>, source: &str) -> Self::HexLiteral {
        Rc::new(output::HexLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_hex_string_literal(&self, range: Range<usize>, source: &str) -> Self::HexStringLiteral {
        Rc::new(output::HexStringLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_hours_keyword(&self, range: Range<usize>, _source: &str) -> Self::HoursKeyword {
        range
    }

    fn make_identifier(&self, range: Range<usize>, source: &str) -> Self::Identifier {
        Rc::new(output::IdentifierStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_if_keyword(&self, range: Range<usize>, _source: &str) -> Self::IfKeyword {
        range
    }

    fn make_immutable_keyword(&self, range: Range<usize>, _source: &str) -> Self::ImmutableKeyword {
        range
    }

    fn make_implements_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::ImplementsKeyword {
        range
    }

    fn make_import_keyword(&self, range: Range<usize>, _source: &str) -> Self::ImportKeyword {
        range
    }

    fn make_in_keyword(&self, range: Range<usize>, _source: &str) -> Self::InKeyword {
        range
    }

    fn make_indexed_keyword(&self, range: Range<usize>, _source: &str) -> Self::IndexedKeyword {
        range
    }

    fn make_inline_keyword(&self, range: Range<usize>, _source: &str) -> Self::InlineKeyword {
        range
    }

    fn make_int_keyword(&self, range: Range<usize>, source: &str) -> Self::IntKeyword {
        Rc::new(output::IntKeywordStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_interface_keyword(&self, range: Range<usize>, _source: &str) -> Self::InterfaceKeyword {
        range
    }

    fn make_internal_keyword(&self, range: Range<usize>, _source: &str) -> Self::InternalKeyword {
        range
    }

    fn make_is_keyword(&self, range: Range<usize>, _source: &str) -> Self::IsKeyword {
        range
    }

    fn make_layout_keyword(&self, range: Range<usize>, _source: &str) -> Self::LayoutKeyword {
        range
    }

    fn make_less_than(&self, range: Range<usize>, _source: &str) -> Self::LessThan {
        range
    }

    fn make_less_than_equal(&self, range: Range<usize>, _source: &str) -> Self::LessThanEqual {
        range
    }

    fn make_less_than_less_than(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::LessThanLessThan {
        range
    }

    fn make_less_than_less_than_equal(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::LessThanLessThanEqual {
        range
    }

    fn make_let_keyword(&self, range: Range<usize>, _source: &str) -> Self::LetKeyword {
        range
    }

    fn make_library_keyword(&self, range: Range<usize>, _source: &str) -> Self::LibraryKeyword {
        range
    }

    fn make_macro_keyword(&self, range: Range<usize>, _source: &str) -> Self::MacroKeyword {
        range
    }

    fn make_mapping_keyword(&self, range: Range<usize>, _source: &str) -> Self::MappingKeyword {
        range
    }

    fn make_match_keyword(&self, range: Range<usize>, _source: &str) -> Self::MatchKeyword {
        range
    }

    fn make_memory_keyword(&self, range: Range<usize>, _source: &str) -> Self::MemoryKeyword {
        range
    }

    fn make_minus(&self, range: Range<usize>, _source: &str) -> Self::Minus {
        range
    }

    fn make_minus_equal(&self, range: Range<usize>, _source: &str) -> Self::MinusEqual {
        range
    }

    fn make_minus_minus(&self, range: Range<usize>, _source: &str) -> Self::MinusMinus {
        range
    }

    fn make_minutes_keyword(&self, range: Range<usize>, _source: &str) -> Self::MinutesKeyword {
        range
    }

    fn make_modifier_keyword(&self, range: Range<usize>, _source: &str) -> Self::ModifierKeyword {
        range
    }

    fn make_mutable_keyword(&self, range: Range<usize>, _source: &str) -> Self::MutableKeyword {
        range
    }

    fn make_new_keyword(&self, range: Range<usize>, _source: &str) -> Self::NewKeyword {
        range
    }

    fn make_null_keyword(&self, range: Range<usize>, _source: &str) -> Self::NullKeyword {
        range
    }

    fn make_of_keyword(&self, range: Range<usize>, _source: &str) -> Self::OfKeyword {
        range
    }

    fn make_open_brace(&self, range: Range<usize>, _source: &str) -> Self::OpenBrace {
        range
    }

    fn make_open_bracket(&self, range: Range<usize>, _source: &str) -> Self::OpenBracket {
        range
    }

    fn make_open_paren(&self, range: Range<usize>, _source: &str) -> Self::OpenParen {
        range
    }

    fn make_override_keyword(&self, range: Range<usize>, _source: &str) -> Self::OverrideKeyword {
        range
    }

    fn make_partial_keyword(&self, range: Range<usize>, _source: &str) -> Self::PartialKeyword {
        range
    }

    fn make_payable_keyword(&self, range: Range<usize>, _source: &str) -> Self::PayableKeyword {
        range
    }

    fn make_percent(&self, range: Range<usize>, _source: &str) -> Self::Percent {
        range
    }

    fn make_percent_equal(&self, range: Range<usize>, _source: &str) -> Self::PercentEqual {
        range
    }

    fn make_period(&self, range: Range<usize>, _source: &str) -> Self::Period {
        range
    }

    fn make_plus(&self, range: Range<usize>, _source: &str) -> Self::Plus {
        range
    }

    fn make_plus_equal(&self, range: Range<usize>, _source: &str) -> Self::PlusEqual {
        range
    }

    fn make_plus_plus(&self, range: Range<usize>, _source: &str) -> Self::PlusPlus {
        range
    }

    fn make_pragma_bar_bar(&self, range: Range<usize>, _source: &str) -> Self::PragmaBarBar {
        range
    }

    fn make_pragma_caret(&self, range: Range<usize>, _source: &str) -> Self::PragmaCaret {
        range
    }

    fn make_pragma_equal(&self, range: Range<usize>, _source: &str) -> Self::PragmaEqual {
        range
    }

    fn make_pragma_greater_than(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::PragmaGreaterThan {
        range
    }

    fn make_pragma_greater_than_equal(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::PragmaGreaterThanEqual {
        range
    }

    fn make_pragma_keyword(&self, range: Range<usize>, _source: &str) -> Self::PragmaKeyword {
        range
    }

    fn make_pragma_less_than(&self, range: Range<usize>, _source: &str) -> Self::PragmaLessThan {
        range
    }

    fn make_pragma_less_than_equal(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::PragmaLessThanEqual {
        range
    }

    fn make_pragma_minus(&self, range: Range<usize>, _source: &str) -> Self::PragmaMinus {
        range
    }

    fn make_pragma_period(&self, range: Range<usize>, _source: &str) -> Self::PragmaPeriod {
        range
    }

    fn make_pragma_semicolon(&self, range: Range<usize>, _source: &str) -> Self::PragmaSemicolon {
        range
    }

    fn make_pragma_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::PragmaStringLiteral {
        Rc::new(output::StringLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_pragma_tilde(&self, range: Range<usize>, _source: &str) -> Self::PragmaTilde {
        range
    }

    fn make_private_keyword(&self, range: Range<usize>, _source: &str) -> Self::PrivateKeyword {
        range
    }

    fn make_promise_keyword(&self, range: Range<usize>, _source: &str) -> Self::PromiseKeyword {
        range
    }

    fn make_public_keyword(&self, range: Range<usize>, _source: &str) -> Self::PublicKeyword {
        range
    }

    fn make_pure_keyword(&self, range: Range<usize>, _source: &str) -> Self::PureKeyword {
        range
    }

    fn make_question_mark(&self, range: Range<usize>, _source: &str) -> Self::QuestionMark {
        range
    }

    fn make_receive_keyword(&self, range: Range<usize>, _source: &str) -> Self::ReceiveKeyword {
        range
    }

    fn make_reference_keyword(&self, range: Range<usize>, _source: &str) -> Self::ReferenceKeyword {
        range
    }

    fn make_relocatable_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::RelocatableKeyword {
        range
    }

    fn make_return_keyword(&self, range: Range<usize>, _source: &str) -> Self::ReturnKeyword {
        range
    }

    fn make_returns_keyword(&self, range: Range<usize>, _source: &str) -> Self::ReturnsKeyword {
        range
    }

    fn make_revert_keyword(&self, range: Range<usize>, _source: &str) -> Self::RevertKeyword {
        range
    }

    fn make_smt_checker_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::SMTCheckerKeyword {
        range
    }

    fn make_sealed_keyword(&self, range: Range<usize>, _source: &str) -> Self::SealedKeyword {
        range
    }

    fn make_seconds_keyword(&self, range: Range<usize>, _source: &str) -> Self::SecondsKeyword {
        range
    }

    fn make_semicolon(&self, range: Range<usize>, _source: &str) -> Self::Semicolon {
        range
    }

    fn make_size_of_keyword(&self, range: Range<usize>, _source: &str) -> Self::SizeOfKeyword {
        range
    }

    fn make_slash(&self, range: Range<usize>, _source: &str) -> Self::Slash {
        range
    }

    fn make_slash_equal(&self, range: Range<usize>, _source: &str) -> Self::SlashEqual {
        range
    }

    fn make_solidity_keyword(&self, range: Range<usize>, _source: &str) -> Self::SolidityKeyword {
        range
    }

    fn make_static_keyword(&self, range: Range<usize>, _source: &str) -> Self::StaticKeyword {
        range
    }

    fn make_storage_keyword(&self, range: Range<usize>, _source: &str) -> Self::StorageKeyword {
        range
    }

    fn make_string_keyword(&self, range: Range<usize>, _source: &str) -> Self::StringKeyword {
        range
    }

    fn make_string_literal(&self, range: Range<usize>, source: &str) -> Self::StringLiteral {
        Rc::new(output::StringLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_struct_keyword(&self, range: Range<usize>, _source: &str) -> Self::StructKeyword {
        range
    }

    fn make_super_keyword(&self, range: Range<usize>, _source: &str) -> Self::SuperKeyword {
        range
    }

    fn make_supports_keyword(&self, range: Range<usize>, _source: &str) -> Self::SupportsKeyword {
        range
    }

    fn make_switch_keyword(&self, range: Range<usize>, _source: &str) -> Self::SwitchKeyword {
        range
    }

    fn make_this_keyword(&self, range: Range<usize>, _source: &str) -> Self::ThisKeyword {
        range
    }

    fn make_throw_keyword(&self, range: Range<usize>, _source: &str) -> Self::ThrowKeyword {
        range
    }

    fn make_tilde(&self, range: Range<usize>, _source: &str) -> Self::Tilde {
        range
    }

    fn make_transient_keyword(&self, range: Range<usize>, _source: &str) -> Self::TransientKeyword {
        range
    }

    fn make_true_keyword(&self, range: Range<usize>, _source: &str) -> Self::TrueKeyword {
        range
    }

    fn make_try_keyword(&self, range: Range<usize>, _source: &str) -> Self::TryKeyword {
        range
    }

    fn make_type_def_keyword(&self, range: Range<usize>, _source: &str) -> Self::TypeDefKeyword {
        range
    }

    fn make_type_keyword(&self, range: Range<usize>, _source: &str) -> Self::TypeKeyword {
        range
    }

    fn make_type_of_keyword(&self, range: Range<usize>, _source: &str) -> Self::TypeOfKeyword {
        range
    }

    fn make_ufixed_keyword(&self, range: Range<usize>, source: &str) -> Self::UfixedKeyword {
        Rc::new(output::UfixedKeywordStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_uint_keyword(&self, range: Range<usize>, source: &str) -> Self::UintKeyword {
        Rc::new(output::UintKeywordStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_unchecked_keyword(&self, range: Range<usize>, _source: &str) -> Self::UncheckedKeyword {
        range
    }

    fn make_unicode_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::UnicodeStringLiteral {
        Rc::new(output::UnicodeStringLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_using_keyword(&self, range: Range<usize>, _source: &str) -> Self::UsingKeyword {
        range
    }

    fn make_var_keyword(&self, range: Range<usize>, _source: &str) -> Self::VarKeyword {
        range
    }

    fn make_version_specifier(&self, range: Range<usize>, source: &str) -> Self::VersionSpecifier {
        Rc::new(output::VersionSpecifierStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_view_keyword(&self, range: Range<usize>, _source: &str) -> Self::ViewKeyword {
        range
    }

    fn make_virtual_keyword(&self, range: Range<usize>, _source: &str) -> Self::VirtualKeyword {
        range
    }

    fn make_weeks_keyword(&self, range: Range<usize>, _source: &str) -> Self::WeeksKeyword {
        range
    }

    fn make_wei_keyword(&self, range: Range<usize>, _source: &str) -> Self::WeiKeyword {
        range
    }

    fn make_while_keyword(&self, range: Range<usize>, _source: &str) -> Self::WhileKeyword {
        range
    }

    fn make_years_keyword(&self, range: Range<usize>, _source: &str) -> Self::YearsKeyword {
        range
    }

    fn make_yul_break_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulBreakKeyword {
        range
    }

    fn make_yul_case_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulCaseKeyword {
        range
    }

    fn make_yul_close_brace(&self, range: Range<usize>, _source: &str) -> Self::YulCloseBrace {
        range
    }

    fn make_yul_close_paren(&self, range: Range<usize>, _source: &str) -> Self::YulCloseParen {
        range
    }

    fn make_yul_colon_equal(&self, range: Range<usize>, _source: &str) -> Self::YulColonEqual {
        range
    }

    fn make_yul_comma(&self, range: Range<usize>, _source: &str) -> Self::YulComma {
        range
    }

    fn make_yul_continue_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::YulContinueKeyword {
        range
    }

    fn make_yul_decimal_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::YulDecimalLiteral {
        Rc::new(output::DecimalLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_yul_default_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::YulDefaultKeyword {
        range
    }

    fn make_yul_false_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulFalseKeyword {
        range
    }

    fn make_yul_for_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulForKeyword {
        range
    }

    fn make_yul_function_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::YulFunctionKeyword {
        range
    }

    fn make_yul_hex_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulHexKeyword {
        range
    }

    fn make_yul_hex_literal(&self, range: Range<usize>, source: &str) -> Self::YulHexLiteral {
        Rc::new(output::HexLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_yul_hex_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::YulHexStringLiteral {
        Rc::new(output::HexStringLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_yul_identifier(&self, range: Range<usize>, source: &str) -> Self::YulIdentifier {
        Rc::new(output::IdentifierStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_yul_if_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulIfKeyword {
        range
    }

    fn make_yul_leave_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulLeaveKeyword {
        range
    }

    fn make_yul_let_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulLetKeyword {
        range
    }

    fn make_yul_minus_greater_than(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::YulMinusGreaterThan {
        range
    }

    fn make_yul_open_brace(&self, range: Range<usize>, _source: &str) -> Self::YulOpenBrace {
        range
    }

    fn make_yul_open_paren(&self, range: Range<usize>, _source: &str) -> Self::YulOpenParen {
        range
    }

    fn make_yul_period(&self, range: Range<usize>, _source: &str) -> Self::YulPeriod {
        range
    }

    fn make_yul_string_literal(&self, range: Range<usize>, source: &str) -> Self::YulStringLiteral {
        Rc::new(output::StringLiteralStruct {
            range: range.clone(),
            text: source[range].to_owned(),
        })
    }

    fn make_yul_super_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulSuperKeyword {
        range
    }

    fn make_yul_switch_keyword(
        &self,
        range: Range<usize>,
        _source: &str,
    ) -> Self::YulSwitchKeyword {
        range
    }

    fn make_yul_this_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulThisKeyword {
        range
    }

    fn make_yul_true_keyword(&self, range: Range<usize>, _source: &str) -> Self::YulTrueKeyword {
        range
    }

    // ============================================
    // === Collection Factory Methods ===
    // ============================================
    fn make_array_values(&self, elements: Vec<Self::Expression>) -> Self::ArrayValues {
        elements
    }
    fn make_call_options(&self, elements: Vec<Self::NamedArgument>) -> Self::CallOptions {
        elements
    }
    fn make_catch_clauses(&self, elements: Vec<Self::CatchClause>) -> Self::CatchClauses {
        elements
    }
    fn make_constructor_attributes(
        &self,
        elements: Vec<Self::ConstructorAttribute>,
    ) -> Self::ConstructorAttributes {
        elements
    }
    fn make_contract_members(&self, elements: Vec<Self::ContractMember>) -> Self::ContractMembers {
        elements
    }
    fn make_contract_specifiers(
        &self,
        elements: Vec<Self::ContractSpecifier>,
    ) -> Self::ContractSpecifiers {
        elements
    }
    fn make_enum_members(&self, elements: Vec<Self::Identifier>) -> Self::EnumMembers {
        elements
    }
    fn make_error_parameters(&self, elements: Vec<Self::ErrorParameter>) -> Self::ErrorParameters {
        elements
    }
    fn make_event_parameters(&self, elements: Vec<Self::EventParameter>) -> Self::EventParameters {
        elements
    }
    fn make_fallback_function_attributes(
        &self,
        elements: Vec<Self::FallbackFunctionAttribute>,
    ) -> Self::FallbackFunctionAttributes {
        elements
    }
    fn make_function_attributes(
        &self,
        elements: Vec<Self::FunctionAttribute>,
    ) -> Self::FunctionAttributes {
        elements
    }
    fn make_function_type_attributes(
        &self,
        elements: Vec<Self::FunctionTypeAttribute>,
    ) -> Self::FunctionTypeAttributes {
        elements
    }
    fn make_hex_string_literals(
        &self,
        elements: Vec<Self::HexStringLiteral>,
    ) -> Self::HexStringLiterals {
        elements
    }
    fn make_identifier_path(
        &self,
        elements: Vec<Self::IdentifierPathElement>,
    ) -> Self::IdentifierPath {
        elements
    }
    fn make_import_deconstruction_symbols(
        &self,
        elements: Vec<Self::ImportDeconstructionSymbol>,
    ) -> Self::ImportDeconstructionSymbols {
        elements
    }
    fn make_inheritance_types(
        &self,
        elements: Vec<Self::InheritanceType>,
    ) -> Self::InheritanceTypes {
        elements
    }
    fn make_interface_members(
        &self,
        elements: Vec<Self::ContractMember>,
    ) -> Self::InterfaceMembers {
        elements
    }
    fn make_library_members(&self, elements: Vec<Self::ContractMember>) -> Self::LibraryMembers {
        elements
    }
    fn make_modifier_attributes(
        &self,
        elements: Vec<Self::ModifierAttribute>,
    ) -> Self::ModifierAttributes {
        elements
    }
    fn make_multi_typed_declaration_elements(
        &self,
        elements: Vec<Self::MultiTypedDeclarationElement>,
    ) -> Self::MultiTypedDeclarationElements {
        elements
    }
    fn make_named_arguments(&self, elements: Vec<Self::NamedArgument>) -> Self::NamedArguments {
        elements
    }
    fn make_override_paths(&self, elements: Vec<Self::IdentifierPath>) -> Self::OverridePaths {
        elements
    }
    fn make_parameters(&self, elements: Vec<Self::Parameter>) -> Self::Parameters {
        elements
    }
    fn make_positional_arguments(
        &self,
        elements: Vec<Self::Expression>,
    ) -> Self::PositionalArguments {
        elements
    }
    fn make_receive_function_attributes(
        &self,
        elements: Vec<Self::ReceiveFunctionAttribute>,
    ) -> Self::ReceiveFunctionAttributes {
        elements
    }
    fn make_simple_version_literal(
        &self,
        elements: Vec<Self::VersionSpecifier>,
    ) -> Self::SimpleVersionLiteral {
        elements
    }
    fn make_source_unit_members(
        &self,
        elements: Vec<Self::SourceUnitMember>,
    ) -> Self::SourceUnitMembers {
        elements
    }
    fn make_state_variable_attributes(
        &self,
        elements: Vec<Self::StateVariableAttribute>,
    ) -> Self::StateVariableAttributes {
        elements
    }
    fn make_statements(&self, elements: Vec<Self::Statement>) -> Self::Statements {
        elements
    }
    fn make_string_literals(&self, elements: Vec<Self::StringLiteral>) -> Self::StringLiterals {
        elements
    }
    fn make_struct_members(&self, elements: Vec<Self::StructMember>) -> Self::StructMembers {
        elements
    }
    fn make_tuple_values(&self, elements: Vec<Self::TupleValue>) -> Self::TupleValues {
        elements
    }
    fn make_unicode_string_literals(
        &self,
        elements: Vec<Self::UnicodeStringLiteral>,
    ) -> Self::UnicodeStringLiterals {
        elements
    }
    fn make_using_deconstruction_symbols(
        &self,
        elements: Vec<Self::UsingDeconstructionSymbol>,
    ) -> Self::UsingDeconstructionSymbols {
        elements
    }
    fn make_version_expression_set(
        &self,
        elements: Vec<Self::VersionExpression>,
    ) -> Self::VersionExpressionSet {
        elements
    }
    fn make_version_expression_sets(
        &self,
        elements: Vec<Self::VersionExpressionSet>,
    ) -> Self::VersionExpressionSets {
        elements
    }
    fn make_yul_arguments(&self, elements: Vec<Self::YulExpression>) -> Self::YulArguments {
        elements
    }
    fn make_yul_flags(&self, elements: Vec<Self::YulStringLiteral>) -> Self::YulFlags {
        elements
    }
    fn make_yul_parameters(&self, elements: Vec<Self::YulIdentifier>) -> Self::YulParameters {
        elements
    }
    fn make_yul_path(&self, elements: Vec<Self::YulIdentifier>) -> Self::YulPath {
        elements
    }
    fn make_yul_paths(&self, elements: Vec<Self::YulPath>) -> Self::YulPaths {
        elements
    }
    fn make_yul_statements(&self, elements: Vec<Self::YulStatement>) -> Self::YulStatements {
        elements
    }
    fn make_yul_switch_cases(&self, elements: Vec<Self::YulSwitchCase>) -> Self::YulSwitchCases {
        elements
    }
    fn make_yul_variable_names(
        &self,
        elements: Vec<Self::YulIdentifier>,
    ) -> Self::YulVariableNames {
        elements
    }

    // ============================================
    // === Sequence Factory Methods ===
    // ============================================
    fn make_abicoder_pragma(
        &self,
        abicoder_keyword: Self::AbicoderKeyword,
        version: Self::AbicoderVersion,
    ) -> Self::AbicoderPragma {
        Rc::new(output::AbicoderPragmaStruct { version })
    }
    fn make_additive_expression(
        &self,
        left_operand: Self::Expression,
        expression_additive_expression_operator: Self::Expression_AdditiveExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::AdditiveExpression {
        Rc::new(output::AdditiveExpressionStruct {
            left_operand,
            expression_additive_expression_operator,
            right_operand,
        })
    }
    fn make_address_type(
        &self,
        address_keyword: Self::AddressKeyword,
        payable_keyword: Option<Self::PayableKeyword>,
    ) -> Self::AddressType {
        Rc::new(output::AddressTypeStruct {
            payable_keyword: payable_keyword.is_some(),
        })
    }
    fn make_and_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::AmpersandAmpersand,
        right_operand: Self::Expression,
    ) -> Self::AndExpression {
        Rc::new(output::AndExpressionStruct {
            left_operand,
            right_operand,
        })
    }
    fn make_array_expression(
        &self,
        open_bracket: Self::OpenBracket,
        items: Self::ArrayValues,
        close_bracket: Self::CloseBracket,
    ) -> Self::ArrayExpression {
        Rc::new(output::ArrayExpressionStruct { items })
    }
    fn make_array_type_name(
        &self,
        operand: Self::TypeName,
        open_bracket: Self::OpenBracket,
        index: Option<Self::Expression>,
        close_bracket: Self::CloseBracket,
    ) -> Self::ArrayTypeName {
        Rc::new(output::ArrayTypeNameStruct { operand, index })
    }
    fn make_assembly_statement(
        &self,
        assembly_keyword: Self::AssemblyKeyword,
        label: Option<Self::YulStringLiteral>,
        flags: Option<Self::YulFlagsDeclaration>,
        body: Self::YulBlock,
    ) -> Self::AssemblyStatement {
        Rc::new(output::AssemblyStatementStruct { label, flags, body })
    }
    fn make_assignment_expression(
        &self,
        left_operand: Self::Expression,
        expression_assignment_expression_operator: Self::Expression_AssignmentExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::AssignmentExpression {
        Rc::new(output::AssignmentExpressionStruct {
            left_operand,
            expression_assignment_expression_operator,
            right_operand,
        })
    }
    fn make_bitwise_and_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::Ampersand,
        right_operand: Self::Expression,
    ) -> Self::BitwiseAndExpression {
        Rc::new(output::BitwiseAndExpressionStruct {
            left_operand,
            right_operand,
        })
    }
    fn make_bitwise_or_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::Bar,
        right_operand: Self::Expression,
    ) -> Self::BitwiseOrExpression {
        Rc::new(output::BitwiseOrExpressionStruct {
            left_operand,
            right_operand,
        })
    }
    fn make_bitwise_xor_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::Caret,
        right_operand: Self::Expression,
    ) -> Self::BitwiseXorExpression {
        Rc::new(output::BitwiseXorExpressionStruct {
            left_operand,
            right_operand,
        })
    }
    fn make_block(
        &self,
        open_brace: Self::OpenBrace,
        statements: Self::Statements,
        close_brace: Self::CloseBrace,
    ) -> Self::Block {
        Rc::new(output::BlockStruct { statements })
    }
    fn make_break_statement(
        &self,
        break_keyword: Self::BreakKeyword,
        semicolon: Self::Semicolon,
    ) -> Self::BreakStatement {
        Rc::new(output::BreakStatementStruct {})
    }
    fn make_call_options_expression(
        &self,
        operand: Self::Expression,
        open_brace: Self::OpenBrace,
        options: Self::CallOptions,
        close_brace: Self::CloseBrace,
    ) -> Self::CallOptionsExpression {
        Rc::new(output::CallOptionsExpressionStruct { operand, options })
    }
    fn make_catch_clause(
        &self,
        catch_keyword: Self::CatchKeyword,
        error: Option<Self::CatchClauseError>,
        body: Self::Block,
    ) -> Self::CatchClause {
        Rc::new(output::CatchClauseStruct { error, body })
    }
    fn make_catch_clause_error(
        &self,
        name: Option<Self::Identifier>,
        parameters: Self::ParametersDeclaration,
    ) -> Self::CatchClauseError {
        Rc::new(output::CatchClauseErrorStruct { name, parameters })
    }
    fn make_conditional_expression(
        &self,
        operand: Self::Expression,
        question_mark: Self::QuestionMark,
        true_expression: Self::Expression,
        colon: Self::Colon,
        false_expression: Self::Expression,
    ) -> Self::ConditionalExpression {
        Rc::new(output::ConditionalExpressionStruct {
            operand,
            true_expression,
            false_expression,
        })
    }
    fn make_constant_definition(
        &self,
        type_name: Self::TypeName,
        constant_keyword: Self::ConstantKeyword,
        name: Self::Identifier,
        equal: Self::Equal,
        value: Self::Expression,
        semicolon: Self::Semicolon,
    ) -> Self::ConstantDefinition {
        self.build_constant_definition_impl(
            type_name,
            constant_keyword,
            name,
            equal,
            value,
            semicolon,
        )
    }
    fn make_constructor_definition(
        &self,
        constructor_keyword: Self::ConstructorKeyword,
        parameters: Self::ParametersDeclaration,
        attributes: Self::ConstructorAttributes,
        body: Self::Block,
    ) -> Self::ConstructorDefinition {
        self.build_constructor_definition_impl(constructor_keyword, parameters, attributes, body)
    }
    fn make_continue_statement(
        &self,
        continue_keyword: Self::ContinueKeyword,
        semicolon: Self::Semicolon,
    ) -> Self::ContinueStatement {
        Rc::new(output::ContinueStatementStruct {})
    }
    fn make_contract_definition(
        &self,
        abstract_keyword: Option<Self::AbstractKeyword>,
        contract_keyword: Self::ContractKeyword,
        name: Self::Identifier,
        specifiers: Self::ContractSpecifiers,
        open_brace: Self::OpenBrace,
        members: Self::ContractMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::ContractDefinition {
        self.build_contract_definition_impl(
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
        literal: Self::DecimalLiteral,
        unit: Option<Self::NumberUnit>,
    ) -> Self::DecimalNumberExpression {
        Rc::new(output::DecimalNumberExpressionStruct { literal, unit })
    }
    fn make_do_while_statement(
        &self,
        do_keyword: Self::DoKeyword,
        body: Self::Statement,
        while_keyword: Self::WhileKeyword,
        open_paren: Self::OpenParen,
        condition: Self::Expression,
        close_paren: Self::CloseParen,
        semicolon: Self::Semicolon,
    ) -> Self::DoWhileStatement {
        Rc::new(output::DoWhileStatementStruct { body, condition })
    }
    fn make_else_branch(
        &self,
        else_keyword: Self::ElseKeyword,
        body: Self::Statement,
    ) -> Self::ElseBranch {
        body
    }
    fn make_emit_statement(
        &self,
        emit_keyword: Self::EmitKeyword,
        event: Self::IdentifierPath,
        arguments: Self::ArgumentsDeclaration,
        semicolon: Self::Semicolon,
    ) -> Self::EmitStatement {
        Rc::new(output::EmitStatementStruct { event, arguments })
    }
    fn make_enum_definition(
        &self,
        enum_keyword: Self::EnumKeyword,
        name: Self::Identifier,
        open_brace: Self::OpenBrace,
        members: Self::EnumMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::EnumDefinition {
        Rc::new(output::EnumDefinitionStruct { name, members })
    }
    fn make_equality_expression(
        &self,
        left_operand: Self::Expression,
        expression_equality_expression_operator: Self::Expression_EqualityExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::EqualityExpression {
        Rc::new(output::EqualityExpressionStruct {
            left_operand,
            expression_equality_expression_operator,
            right_operand,
        })
    }
    fn make_error_definition(
        &self,
        error_keyword: Self::ErrorKeyword,
        name: Self::Identifier,
        members: Self::ErrorParametersDeclaration,
        semicolon: Self::Semicolon,
    ) -> Self::ErrorDefinition {
        self.build_error_definition_impl(error_keyword, name, members, semicolon)
    }
    fn make_error_parameter(
        &self,
        type_name: Self::TypeName,
        name: Option<Self::Identifier>,
    ) -> Self::ErrorParameter {
        self.build_error_parameter_impl(type_name, name)
    }
    fn make_error_parameters_declaration(
        &self,
        open_paren: Self::OpenParen,
        parameters: Self::ErrorParameters,
        close_paren: Self::CloseParen,
    ) -> Self::ErrorParametersDeclaration {
        self.build_error_parameters_declaration_impl(open_paren, parameters, close_paren)
    }
    fn make_event_definition(
        &self,
        event_keyword: Self::EventKeyword,
        name: Self::Identifier,
        parameters: Self::EventParametersDeclaration,
        anonymous_keyword: Option<Self::AnonymousKeyword>,
        semicolon: Self::Semicolon,
    ) -> Self::EventDefinition {
        self.build_event_definition_impl(
            event_keyword,
            name,
            parameters,
            anonymous_keyword,
            semicolon,
        )
    }
    fn make_event_parameter(
        &self,
        type_name: Self::TypeName,
        indexed_keyword: Option<Self::IndexedKeyword>,
        name: Option<Self::Identifier>,
    ) -> Self::EventParameter {
        self.build_event_parameter_impl(type_name, indexed_keyword, name)
    }
    fn make_event_parameters_declaration(
        &self,
        open_paren: Self::OpenParen,
        parameters: Self::EventParameters,
        close_paren: Self::CloseParen,
    ) -> Self::EventParametersDeclaration {
        self.build_event_parameters_declaration_impl(open_paren, parameters, close_paren)
    }
    fn make_experimental_pragma(
        &self,
        experimental_keyword: Self::ExperimentalKeyword,
        feature: Self::ExperimentalFeature,
    ) -> Self::ExperimentalPragma {
        Rc::new(output::ExperimentalPragmaStruct { feature })
    }
    fn make_exponentiation_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::AsteriskAsterisk,
        right_operand: Self::Expression,
    ) -> Self::ExponentiationExpression {
        Rc::new(output::ExponentiationExpressionStruct {
            left_operand,
            right_operand,
        })
    }
    fn make_expression_statement(
        &self,
        expression: Self::Expression,
        semicolon: Self::Semicolon,
    ) -> Self::ExpressionStatement {
        Rc::new(output::ExpressionStatementStruct { expression })
    }
    fn make_fallback_function_definition(
        &self,
        fallback_keyword: Self::FallbackKeyword,
        parameters: Self::ParametersDeclaration,
        attributes: Self::FallbackFunctionAttributes,
        returns: Option<Self::ReturnsDeclaration>,
        body: Self::FunctionBody,
    ) -> Self::FallbackFunctionDefinition {
        self.build_fallback_function_definition_impl(
            fallback_keyword,
            parameters,
            attributes,
            returns,
            body,
        )
    }
    fn make_for_statement(
        &self,
        for_keyword: Self::ForKeyword,
        open_paren: Self::OpenParen,
        initialization: Self::ForStatementInitialization,
        condition: Self::ForStatementCondition,
        iterator: Option<Self::Expression>,
        close_paren: Self::CloseParen,
        body: Self::Statement,
    ) -> Self::ForStatement {
        Rc::new(output::ForStatementStruct {
            initialization,
            condition,
            iterator,
            body,
        })
    }
    fn make_function_call_expression(
        &self,
        operand: Self::Expression,
        arguments: Self::ArgumentsDeclaration,
    ) -> Self::FunctionCallExpression {
        Rc::new(output::FunctionCallExpressionStruct { operand, arguments })
    }
    fn make_function_definition(
        &self,
        function_keyword: Self::FunctionKeyword,
        name: Self::FunctionName,
        parameters: Self::ParametersDeclaration,
        attributes: Self::FunctionAttributes,
        returns: Option<Self::ReturnsDeclaration>,
        body: Self::FunctionBody,
    ) -> Self::FunctionDefinition {
        self.build_function_definition_impl(
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
        function_keyword: Self::FunctionKeyword,
        parameters: Self::ParametersDeclaration,
        attributes: Self::FunctionTypeAttributes,
        returns: Option<Self::ReturnsDeclaration>,
    ) -> Self::FunctionType {
        self.build_function_type_impl(function_keyword, parameters, attributes, returns)
    }
    fn make_hex_number_expression(&self, literal: Self::HexLiteral) -> Self::HexNumberExpression {
        Rc::new(output::HexNumberExpressionStruct { literal })
    }
    fn make_if_statement(
        &self,
        if_keyword: Self::IfKeyword,
        open_paren: Self::OpenParen,
        condition: Self::Expression,
        close_paren: Self::CloseParen,
        body: Self::Statement,
        else_branch: Option<Self::ElseBranch>,
    ) -> Self::IfStatement {
        Rc::new(output::IfStatementStruct {
            condition,
            body,
            else_branch,
        })
    }
    fn make_import_alias(
        &self,
        as_keyword: Self::AsKeyword,
        identifier: Self::Identifier,
    ) -> Self::ImportAlias {
        identifier
    }
    fn make_import_deconstruction(
        &self,
        open_brace: Self::OpenBrace,
        symbols: Self::ImportDeconstructionSymbols,
        close_brace: Self::CloseBrace,
        from_keyword: Self::FromKeyword,
        path: Self::StringLiteral,
    ) -> Self::ImportDeconstruction {
        Rc::new(output::ImportDeconstructionStruct { symbols, path })
    }
    fn make_import_deconstruction_symbol(
        &self,
        name: Self::Identifier,
        alias: Option<Self::ImportAlias>,
    ) -> Self::ImportDeconstructionSymbol {
        Rc::new(output::ImportDeconstructionSymbolStruct { name, alias })
    }
    fn make_import_directive(
        &self,
        import_keyword: Self::ImportKeyword,
        clause: Self::ImportClause,
        semicolon: Self::Semicolon,
    ) -> Self::ImportDirective {
        clause
    }
    fn make_index_access_end(
        &self,
        colon: Self::Colon,
        end: Option<Self::Expression>,
    ) -> Self::IndexAccessEnd {
        self.build_index_access_end_impl(colon, end)
    }
    fn make_index_access_expression(
        &self,
        operand: Self::Expression,
        open_bracket: Self::OpenBracket,
        start: Option<Self::Expression>,
        end: Option<Self::IndexAccessEnd>,
        close_bracket: Self::CloseBracket,
    ) -> Self::IndexAccessExpression {
        self.build_index_access_expression_impl(operand, open_bracket, start, end, close_bracket)
    }
    fn make_inequality_expression(
        &self,
        left_operand: Self::Expression,
        expression_inequality_expression_operator: Self::Expression_InequalityExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::InequalityExpression {
        Rc::new(output::InequalityExpressionStruct {
            left_operand,
            expression_inequality_expression_operator,
            right_operand,
        })
    }
    fn make_inheritance_specifier(
        &self,
        is_keyword: Self::IsKeyword,
        types: Self::InheritanceTypes,
    ) -> Self::InheritanceSpecifier {
        types
    }
    fn make_inheritance_type(
        &self,
        type_name: Self::IdentifierPath,
        arguments: Option<Self::ArgumentsDeclaration>,
    ) -> Self::InheritanceType {
        Rc::new(output::InheritanceTypeStruct {
            type_name,
            arguments,
        })
    }
    fn make_interface_definition(
        &self,
        interface_keyword: Self::InterfaceKeyword,
        name: Self::Identifier,
        inheritance: Option<Self::InheritanceSpecifier>,
        open_brace: Self::OpenBrace,
        members: Self::InterfaceMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::InterfaceDefinition {
        Rc::new(output::InterfaceDefinitionStruct {
            name,
            inheritance,
            members,
        })
    }
    fn make_library_definition(
        &self,
        library_keyword: Self::LibraryKeyword,
        name: Self::Identifier,
        open_brace: Self::OpenBrace,
        members: Self::LibraryMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::LibraryDefinition {
        Rc::new(output::LibraryDefinitionStruct { name, members })
    }
    fn make_mapping_key(
        &self,
        key_type: Self::MappingKeyType,
        name: Option<Self::Identifier>,
    ) -> Self::MappingKey {
        self.build_mapping_key_impl(key_type, name)
    }
    fn make_mapping_type(
        &self,
        mapping_keyword: Self::MappingKeyword,
        open_paren: Self::OpenParen,
        key_type: Self::MappingKey,
        equal_greater_than: Self::EqualGreaterThan,
        value_type: Self::MappingValue,
        close_paren: Self::CloseParen,
    ) -> Self::MappingType {
        self.build_mapping_type_impl(
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
        type_name: Self::TypeName,
        name: Option<Self::Identifier>,
    ) -> Self::MappingValue {
        self.build_mapping_value_impl(type_name, name)
    }
    fn make_member_access_expression(
        &self,
        operand: Self::Expression,
        period: Self::Period,
        member: Self::IdentifierPathElement,
    ) -> Self::MemberAccessExpression {
        Rc::new(output::MemberAccessExpressionStruct { operand, member })
    }
    fn make_modifier_definition(
        &self,
        modifier_keyword: Self::ModifierKeyword,
        name: Self::Identifier,
        parameters: Option<Self::ParametersDeclaration>,
        attributes: Self::ModifierAttributes,
        body: Self::FunctionBody,
    ) -> Self::ModifierDefinition {
        self.build_modifier_definition_impl(modifier_keyword, name, parameters, attributes, body)
    }
    fn make_modifier_invocation(
        &self,
        name: Self::IdentifierPath,
        arguments: Option<Self::ArgumentsDeclaration>,
    ) -> Self::ModifierInvocation {
        Rc::new(output::ModifierInvocationStruct { name, arguments })
    }
    fn make_multi_typed_declaration(
        &self,
        open_paren: Self::OpenParen,
        elements: Self::MultiTypedDeclarationElements,
        close_paren: Self::CloseParen,
        value: Self::VariableDeclarationValue,
    ) -> Self::MultiTypedDeclaration {
        Rc::new(output::MultiTypedDeclarationStruct { elements, value })
    }
    fn make_multi_typed_declaration_element(
        &self,
        member: Option<Self::VariableDeclaration>,
    ) -> Self::MultiTypedDeclarationElement {
        Rc::new(output::MultiTypedDeclarationElementStruct { member })
    }
    fn make_multiplicative_expression(
        &self,
        left_operand: Self::Expression,
        expression_multiplicative_expression_operator: Self::Expression_MultiplicativeExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::MultiplicativeExpression {
        Rc::new(output::MultiplicativeExpressionStruct {
            left_operand,
            expression_multiplicative_expression_operator,
            right_operand,
        })
    }
    fn make_named_argument(
        &self,
        name: Self::Identifier,
        colon: Self::Colon,
        value: Self::Expression,
    ) -> Self::NamedArgument {
        Rc::new(output::NamedArgumentStruct { name, value })
    }
    fn make_named_argument_group(
        &self,
        open_brace: Self::OpenBrace,
        arguments: Self::NamedArguments,
        close_brace: Self::CloseBrace,
    ) -> Self::NamedArgumentGroup {
        arguments
    }
    fn make_named_arguments_declaration(
        &self,
        open_paren: Self::OpenParen,
        arguments: Self::NamedArgumentGroup,
        close_paren: Self::CloseParen,
    ) -> Self::NamedArgumentsDeclaration {
        self.build_named_arguments_declaration_impl(open_paren, arguments, close_paren)
    }
    fn make_named_import(
        &self,
        asterisk: Self::Asterisk,
        alias: Self::ImportAlias,
        from_keyword: Self::FromKeyword,
        path: Self::StringLiteral,
    ) -> Self::NamedImport {
        self.build_named_import_impl(asterisk, alias, from_keyword, path)
    }
    fn make_new_expression(
        &self,
        new_keyword: Self::NewKeyword,
        type_name: Self::TypeName,
    ) -> Self::NewExpression {
        Rc::new(output::NewExpressionStruct { type_name })
    }
    fn make_or_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::BarBar,
        right_operand: Self::Expression,
    ) -> Self::OrExpression {
        Rc::new(output::OrExpressionStruct {
            left_operand,
            right_operand,
        })
    }
    fn make_override_paths_declaration(
        &self,
        open_paren: Self::OpenParen,
        paths: Self::OverridePaths,
        close_paren: Self::CloseParen,
    ) -> Self::OverridePathsDeclaration {
        paths
    }
    fn make_override_specifier(
        &self,
        override_keyword: Self::OverrideKeyword,
        overridden: Option<Self::OverridePathsDeclaration>,
    ) -> Self::OverrideSpecifier {
        self.build_override_specifier_impl(override_keyword, overridden)
    }
    fn make_parameter(
        &self,
        type_name: Self::TypeName,
        storage_location: Option<Self::StorageLocation>,
        name: Option<Self::Identifier>,
    ) -> Self::Parameter {
        self.build_parameter_impl(type_name, storage_location, name)
    }
    fn make_parameters_declaration(
        &self,
        open_paren: Self::OpenParen,
        parameters: Self::Parameters,
        close_paren: Self::CloseParen,
    ) -> Self::ParametersDeclaration {
        parameters
    }
    fn make_path_import(
        &self,
        path: Self::StringLiteral,
        alias: Option<Self::ImportAlias>,
    ) -> Self::PathImport {
        Rc::new(output::PathImportStruct { path, alias })
    }
    fn make_positional_arguments_declaration(
        &self,
        open_paren: Self::OpenParen,
        arguments: Self::PositionalArguments,
        close_paren: Self::CloseParen,
    ) -> Self::PositionalArgumentsDeclaration {
        self.build_positional_arguments_declaration_impl(open_paren, arguments, close_paren)
    }
    fn make_postfix_expression(
        &self,
        operand: Self::Expression,
        expression_postfix_expression_operator: Self::Expression_PostfixExpression_Operator,
    ) -> Self::PostfixExpression {
        Rc::new(output::PostfixExpressionStruct {
            operand,
            expression_postfix_expression_operator,
        })
    }
    fn make_pragma_directive(
        &self,
        pragma_keyword: Self::PragmaKeyword,
        pragma: Self::Pragma,
        semicolon: Self::PragmaSemicolon,
    ) -> Self::PragmaDirective {
        Rc::new(output::PragmaDirectiveStruct { pragma })
    }
    fn make_prefix_expression(
        &self,
        expression_prefix_expression_operator: Self::Expression_PrefixExpression_Operator,
        operand: Self::Expression,
    ) -> Self::PrefixExpression {
        Rc::new(output::PrefixExpressionStruct {
            expression_prefix_expression_operator,
            operand,
        })
    }
    fn make_receive_function_definition(
        &self,
        receive_keyword: Self::ReceiveKeyword,
        parameters: Self::ParametersDeclaration,
        attributes: Self::ReceiveFunctionAttributes,
        body: Self::FunctionBody,
    ) -> Self::ReceiveFunctionDefinition {
        self.build_receive_function_definition_impl(receive_keyword, parameters, attributes, body)
    }
    fn make_return_statement(
        &self,
        return_keyword: Self::ReturnKeyword,
        expression: Option<Self::Expression>,
        semicolon: Self::Semicolon,
    ) -> Self::ReturnStatement {
        Rc::new(output::ReturnStatementStruct { expression })
    }
    fn make_returns_declaration(
        &self,
        returns_keyword: Self::ReturnsKeyword,
        variables: Self::ParametersDeclaration,
    ) -> Self::ReturnsDeclaration {
        variables
    }
    fn make_revert_statement(
        &self,
        revert_keyword: Self::RevertKeyword,
        error: Self::IdentifierPath,
        arguments: Self::ArgumentsDeclaration,
        semicolon: Self::Semicolon,
    ) -> Self::RevertStatement {
        Rc::new(output::RevertStatementStruct { error, arguments })
    }
    fn make_shift_expression(
        &self,
        left_operand: Self::Expression,
        expression_shift_expression_operator: Self::Expression_ShiftExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::ShiftExpression {
        Rc::new(output::ShiftExpressionStruct {
            left_operand,
            expression_shift_expression_operator,
            right_operand,
        })
    }
    fn make_single_typed_declaration(
        &self,
        declaration: Self::VariableDeclaration,
        value: Option<Self::VariableDeclarationValue>,
    ) -> Self::SingleTypedDeclaration {
        Rc::new(output::SingleTypedDeclarationStruct { declaration, value })
    }
    fn make_source_unit(&self, members: Self::SourceUnitMembers) -> Self::SourceUnit {
        Rc::new(output::SourceUnitStruct { members })
    }
    fn make_state_variable_definition(
        &self,
        type_name: Self::TypeName,
        attributes: Self::StateVariableAttributes,
        name: Self::Identifier,
        value: Option<Self::StateVariableDefinitionValue>,
        semicolon: Self::Semicolon,
    ) -> Self::StateVariableDefinition {
        self.build_state_variable_definition_impl(type_name, attributes, name, value, semicolon)
    }
    fn make_state_variable_definition_value(
        &self,
        equal: Self::Equal,
        value: Self::Expression,
    ) -> Self::StateVariableDefinitionValue {
        value
    }
    fn make_storage_layout_specifier(
        &self,
        layout_keyword: Self::LayoutKeyword,
        at_keyword: Self::AtKeyword,
        expression: Self::Expression,
    ) -> Self::StorageLayoutSpecifier {
        expression
    }
    fn make_struct_definition(
        &self,
        struct_keyword: Self::StructKeyword,
        name: Self::Identifier,
        open_brace: Self::OpenBrace,
        members: Self::StructMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::StructDefinition {
        Rc::new(output::StructDefinitionStruct { name, members })
    }
    fn make_struct_member(
        &self,
        type_name: Self::TypeName,
        name: Self::Identifier,
        semicolon: Self::Semicolon,
    ) -> Self::StructMember {
        Rc::new(output::StructMemberStruct { type_name, name })
    }
    fn make_try_statement(
        &self,
        try_keyword: Self::TryKeyword,
        expression: Self::Expression,
        returns: Option<Self::ReturnsDeclaration>,
        body: Self::Block,
        catch_clauses: Self::CatchClauses,
    ) -> Self::TryStatement {
        Rc::new(output::TryStatementStruct {
            expression,
            returns,
            body,
            catch_clauses,
        })
    }
    fn make_tuple_expression(
        &self,
        open_paren: Self::OpenParen,
        items: Self::TupleValues,
        close_paren: Self::CloseParen,
    ) -> Self::TupleExpression {
        Rc::new(output::TupleExpressionStruct { items })
    }
    fn make_tuple_value(&self, expression: Option<Self::Expression>) -> Self::TupleValue {
        Rc::new(output::TupleValueStruct { expression })
    }
    fn make_type_expression(
        &self,
        type_keyword: Self::TypeKeyword,
        open_paren: Self::OpenParen,
        type_name: Self::TypeName,
        close_paren: Self::CloseParen,
    ) -> Self::TypeExpression {
        Rc::new(output::TypeExpressionStruct { type_name })
    }
    fn make_unchecked_block(
        &self,
        unchecked_keyword: Self::UncheckedKeyword,
        block: Self::Block,
    ) -> Self::UncheckedBlock {
        Rc::new(output::UncheckedBlockStruct { block })
    }
    fn make_user_defined_value_type_definition(
        &self,
        type_keyword: Self::TypeKeyword,
        name: Self::Identifier,
        is_keyword: Self::IsKeyword,
        value_type: Self::ElementaryType,
        semicolon: Self::Semicolon,
    ) -> Self::UserDefinedValueTypeDefinition {
        Rc::new(output::UserDefinedValueTypeDefinitionStruct { name, value_type })
    }
    fn make_using_alias(
        &self,
        as_keyword: Self::AsKeyword,
        operator: Self::UsingOperator,
    ) -> Self::UsingAlias {
        operator
    }
    fn make_using_deconstruction(
        &self,
        open_brace: Self::OpenBrace,
        symbols: Self::UsingDeconstructionSymbols,
        close_brace: Self::CloseBrace,
    ) -> Self::UsingDeconstruction {
        Rc::new(output::UsingDeconstructionStruct { symbols })
    }
    fn make_using_deconstruction_symbol(
        &self,
        name: Self::IdentifierPath,
        alias: Option<Self::UsingAlias>,
    ) -> Self::UsingDeconstructionSymbol {
        Rc::new(output::UsingDeconstructionSymbolStruct { name, alias })
    }
    fn make_using_directive(
        &self,
        using_keyword: Self::UsingKeyword,
        clause: Self::UsingClause,
        for_keyword: Self::ForKeyword,
        target: Self::UsingTarget,
        global_keyword: Option<Self::GlobalKeyword>,
        semicolon: Self::Semicolon,
    ) -> Self::UsingDirective {
        Rc::new(output::UsingDirectiveStruct {
            clause,
            target,
            global_keyword: global_keyword.is_some(),
        })
    }
    fn make_variable_declaration(
        &self,
        type_name: Self::TypeName,
        storage_location: Option<Self::StorageLocation>,
        name: Self::Identifier,
    ) -> Self::VariableDeclaration {
        Rc::new(output::VariableDeclarationStruct {
            type_name,
            storage_location,
            name,
        })
    }
    fn make_variable_declaration_statement(
        &self,
        target: Self::VariableDeclarationTarget,
        semicolon: Self::Semicolon,
    ) -> Self::VariableDeclarationStatement {
        Rc::new(output::VariableDeclarationStatementStruct { target })
    }
    fn make_variable_declaration_value(
        &self,
        equal: Self::Equal,
        expression: Self::Expression,
    ) -> Self::VariableDeclarationValue {
        expression
    }
    fn make_version_pragma(
        &self,
        solidity_keyword: Self::SolidityKeyword,
        sets: Self::VersionExpressionSets,
    ) -> Self::VersionPragma {
        Rc::new(output::VersionPragmaStruct { sets })
    }
    fn make_version_range(
        &self,
        start: Self::VersionLiteral,
        minus: Self::PragmaMinus,
        end: Self::VersionLiteral,
    ) -> Self::VersionRange {
        Rc::new(output::VersionRangeStruct { start, end })
    }
    fn make_version_term(
        &self,
        operator: Option<Self::VersionOperator>,
        literal: Self::VersionLiteral,
    ) -> Self::VersionTerm {
        Rc::new(output::VersionTermStruct { operator, literal })
    }
    fn make_while_statement(
        &self,
        while_keyword: Self::WhileKeyword,
        open_paren: Self::OpenParen,
        condition: Self::Expression,
        close_paren: Self::CloseParen,
        body: Self::Statement,
    ) -> Self::WhileStatement {
        Rc::new(output::WhileStatementStruct { condition, body })
    }
    fn make_yul_block(
        &self,
        open_brace: Self::YulOpenBrace,
        statements: Self::YulStatements,
        close_brace: Self::YulCloseBrace,
    ) -> Self::YulBlock {
        Rc::new(output::YulBlockStruct { statements })
    }
    fn make_yul_break_statement(
        &self,
        break_keyword: Self::YulBreakKeyword,
    ) -> Self::YulBreakStatement {
        Rc::new(output::YulBreakStatementStruct {})
    }
    fn make_yul_continue_statement(
        &self,
        continue_keyword: Self::YulContinueKeyword,
    ) -> Self::YulContinueStatement {
        Rc::new(output::YulContinueStatementStruct {})
    }
    fn make_yul_default_case(
        &self,
        default_keyword: Self::YulDefaultKeyword,
        body: Self::YulBlock,
    ) -> Self::YulDefaultCase {
        Rc::new(output::YulDefaultCaseStruct { body })
    }
    fn make_yul_flags_declaration(
        &self,
        open_paren: Self::YulOpenParen,
        flags: Self::YulFlags,
        close_paren: Self::YulCloseParen,
    ) -> Self::YulFlagsDeclaration {
        flags
    }
    fn make_yul_for_statement(
        &self,
        for_keyword: Self::YulForKeyword,
        initialization: Self::YulBlock,
        condition: Self::YulExpression,
        iterator: Self::YulBlock,
        body: Self::YulBlock,
    ) -> Self::YulForStatement {
        Rc::new(output::YulForStatementStruct {
            initialization,
            condition,
            iterator,
            body,
        })
    }
    fn make_yul_function_call_expression(
        &self,
        operand: Self::YulExpression,
        open_paren: Self::YulOpenParen,
        arguments: Self::YulArguments,
        close_paren: Self::YulCloseParen,
    ) -> Self::YulFunctionCallExpression {
        Rc::new(output::YulFunctionCallExpressionStruct { operand, arguments })
    }
    fn make_yul_function_definition(
        &self,
        function_keyword: Self::YulFunctionKeyword,
        name: Self::YulIdentifier,
        parameters: Self::YulParametersDeclaration,
        returns: Option<Self::YulReturnsDeclaration>,
        body: Self::YulBlock,
    ) -> Self::YulFunctionDefinition {
        Rc::new(output::YulFunctionDefinitionStruct {
            name,
            parameters,
            returns,
            body,
        })
    }
    fn make_yul_if_statement(
        &self,
        if_keyword: Self::YulIfKeyword,
        condition: Self::YulExpression,
        body: Self::YulBlock,
    ) -> Self::YulIfStatement {
        Rc::new(output::YulIfStatementStruct { condition, body })
    }
    fn make_yul_leave_statement(
        &self,
        leave_keyword: Self::YulLeaveKeyword,
    ) -> Self::YulLeaveStatement {
        Rc::new(output::YulLeaveStatementStruct {})
    }
    fn make_yul_parameters_declaration(
        &self,
        open_paren: Self::YulOpenParen,
        parameters: Self::YulParameters,
        close_paren: Self::YulCloseParen,
    ) -> Self::YulParametersDeclaration {
        parameters
    }
    fn make_yul_returns_declaration(
        &self,
        minus_greater_than: Self::YulMinusGreaterThan,
        variables: Self::YulVariableNames,
    ) -> Self::YulReturnsDeclaration {
        variables
    }
    fn make_yul_switch_statement(
        &self,
        switch_keyword: Self::YulSwitchKeyword,
        expression: Self::YulExpression,
        cases: Self::YulSwitchCases,
    ) -> Self::YulSwitchStatement {
        Rc::new(output::YulSwitchStatementStruct { expression, cases })
    }
    fn make_yul_value_case(
        &self,
        case_keyword: Self::YulCaseKeyword,
        value: Self::YulLiteral,
        body: Self::YulBlock,
    ) -> Self::YulValueCase {
        Rc::new(output::YulValueCaseStruct { value, body })
    }
    fn make_yul_variable_assignment_statement(
        &self,
        variables: Self::YulPaths,
        assignment: Self::YulColonEqual,
        expression: Self::YulExpression,
    ) -> Self::YulVariableAssignmentStatement {
        Rc::new(output::YulVariableAssignmentStatementStruct {
            variables,
            expression,
        })
    }
    fn make_yul_variable_declaration_statement(
        &self,
        let_keyword: Self::YulLetKeyword,
        variables: Self::YulVariableNames,
        value: Option<Self::YulVariableDeclarationValue>,
    ) -> Self::YulVariableDeclarationStatement {
        Rc::new(output::YulVariableDeclarationStatementStruct { variables, value })
    }
    fn make_yul_variable_declaration_value(
        &self,
        assignment: Self::YulColonEqual,
        expression: Self::YulExpression,
    ) -> Self::YulVariableDeclarationValue {
        Rc::new(output::YulVariableDeclarationValueStruct { expression })
    }

    // ============================================
    // === Choice Variant Factory Methods ===
    // ============================================
    fn make_abicoder_version_abicoder_v1_keyword(
        &self,
        element: Self::AbicoderV1Keyword,
    ) -> Self::AbicoderVersion {
        output::AbicoderVersion::AbicoderV1Keyword
    }
    fn make_abicoder_version_abicoder_v2_keyword(
        &self,
        element: Self::AbicoderV2Keyword,
    ) -> Self::AbicoderVersion {
        output::AbicoderVersion::AbicoderV2Keyword
    }

    fn make_arguments_declaration_positional_arguments_declaration(
        &self,
        element: Self::PositionalArgumentsDeclaration,
    ) -> Self::ArgumentsDeclaration {
        self.build_arguments_declaration_positional_arguments_declaration_impl(element)
    }
    fn make_arguments_declaration_named_arguments_declaration(
        &self,
        element: Self::NamedArgumentsDeclaration,
    ) -> Self::ArgumentsDeclaration {
        self.build_arguments_declaration_named_arguments_declaration_impl(element)
    }

    fn make_constructor_attribute_modifier_invocation(
        &self,
        element: Self::ModifierInvocation,
    ) -> Self::ConstructorAttribute {
        self.build_constructor_attribute_modifier_invocation_impl(element)
    }
    fn make_constructor_attribute_internal_keyword(
        &self,
        element: Self::InternalKeyword,
    ) -> Self::ConstructorAttribute {
        self.build_constructor_attribute_internal_keyword_impl(element)
    }
    fn make_constructor_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::ConstructorAttribute {
        self.build_constructor_attribute_payable_keyword_impl(element)
    }
    fn make_constructor_attribute_public_keyword(
        &self,
        element: Self::PublicKeyword,
    ) -> Self::ConstructorAttribute {
        self.build_constructor_attribute_public_keyword_impl(element)
    }

    fn make_contract_member_using_directive(
        &self,
        element: Self::UsingDirective,
    ) -> Self::ContractMember {
        output::ContractMember::UsingDirective(element)
    }
    fn make_contract_member_function_definition(
        &self,
        element: Self::FunctionDefinition,
    ) -> Self::ContractMember {
        output::ContractMember::FunctionDefinition(element)
    }
    fn make_contract_member_constructor_definition(
        &self,
        element: Self::ConstructorDefinition,
    ) -> Self::ContractMember {
        self.build_contract_member_constructor_definition_impl(element)
    }
    fn make_contract_member_receive_function_definition(
        &self,
        element: Self::ReceiveFunctionDefinition,
    ) -> Self::ContractMember {
        self.build_contract_member_receive_function_definition_impl(element)
    }
    fn make_contract_member_fallback_function_definition(
        &self,
        element: Self::FallbackFunctionDefinition,
    ) -> Self::ContractMember {
        self.build_contract_member_fallback_function_definition_impl(element)
    }
    fn make_contract_member_modifier_definition(
        &self,
        element: Self::ModifierDefinition,
    ) -> Self::ContractMember {
        self.build_contract_member_modifier_definition_impl(element)
    }
    fn make_contract_member_struct_definition(
        &self,
        element: Self::StructDefinition,
    ) -> Self::ContractMember {
        output::ContractMember::StructDefinition(element)
    }
    fn make_contract_member_enum_definition(
        &self,
        element: Self::EnumDefinition,
    ) -> Self::ContractMember {
        output::ContractMember::EnumDefinition(element)
    }
    fn make_contract_member_event_definition(
        &self,
        element: Self::EventDefinition,
    ) -> Self::ContractMember {
        output::ContractMember::EventDefinition(element)
    }
    fn make_contract_member_error_definition(
        &self,
        element: Self::ErrorDefinition,
    ) -> Self::ContractMember {
        output::ContractMember::ErrorDefinition(element)
    }
    fn make_contract_member_user_defined_value_type_definition(
        &self,
        element: Self::UserDefinedValueTypeDefinition,
    ) -> Self::ContractMember {
        output::ContractMember::UserDefinedValueTypeDefinition(element)
    }
    fn make_contract_member_state_variable_definition(
        &self,
        element: Self::StateVariableDefinition,
    ) -> Self::ContractMember {
        self.build_contract_member_state_variable_definition_impl(element)
    }

    fn make_contract_specifier_inheritance_specifier(
        &self,
        element: Self::InheritanceSpecifier,
    ) -> Self::ContractSpecifier {
        self.build_contract_specifier_inheritance_specifier_impl(element)
    }
    fn make_contract_specifier_storage_layout_specifier(
        &self,
        element: Self::StorageLayoutSpecifier,
    ) -> Self::ContractSpecifier {
        self.build_contract_specifier_storage_layout_specifier_impl(element)
    }

    fn make_elementary_type_bool_keyword(
        &self,
        element: Self::BoolKeyword,
    ) -> Self::ElementaryType {
        output::ElementaryType::BoolKeyword
    }
    fn make_elementary_type_string_keyword(
        &self,
        element: Self::StringKeyword,
    ) -> Self::ElementaryType {
        output::ElementaryType::StringKeyword
    }
    fn make_elementary_type_address_type(
        &self,
        element: Self::AddressType,
    ) -> Self::ElementaryType {
        output::ElementaryType::AddressType(element)
    }
    fn make_elementary_type_bytes_keyword(
        &self,
        element: Self::BytesKeyword,
    ) -> Self::ElementaryType {
        output::ElementaryType::BytesKeyword(element)
    }
    fn make_elementary_type_int_keyword(&self, element: Self::IntKeyword) -> Self::ElementaryType {
        output::ElementaryType::IntKeyword(element)
    }
    fn make_elementary_type_uint_keyword(
        &self,
        element: Self::UintKeyword,
    ) -> Self::ElementaryType {
        output::ElementaryType::UintKeyword(element)
    }
    fn make_elementary_type_fixed_keyword(
        &self,
        element: Self::FixedKeyword,
    ) -> Self::ElementaryType {
        output::ElementaryType::FixedKeyword(element)
    }
    fn make_elementary_type_ufixed_keyword(
        &self,
        element: Self::UfixedKeyword,
    ) -> Self::ElementaryType {
        output::ElementaryType::UfixedKeyword(element)
    }

    fn make_experimental_feature_abi_encoder_v2_keyword(
        &self,
        element: Self::ABIEncoderV2Keyword,
    ) -> Self::ExperimentalFeature {
        output::ExperimentalFeature::ABIEncoderV2Keyword
    }
    fn make_experimental_feature_smt_checker_keyword(
        &self,
        element: Self::SMTCheckerKeyword,
    ) -> Self::ExperimentalFeature {
        output::ExperimentalFeature::SMTCheckerKeyword
    }
    fn make_experimental_feature_pragma_string_literal(
        &self,
        element: Self::PragmaStringLiteral,
    ) -> Self::ExperimentalFeature {
        output::ExperimentalFeature::StringLiteral(element)
    }

    fn make_expression_assignment_expression(
        &self,
        element: Self::AssignmentExpression,
    ) -> Self::Expression {
        output::Expression::AssignmentExpression(element)
    }
    fn make_expression_conditional_expression(
        &self,
        element: Self::ConditionalExpression,
    ) -> Self::Expression {
        output::Expression::ConditionalExpression(element)
    }
    fn make_expression_or_expression(&self, element: Self::OrExpression) -> Self::Expression {
        output::Expression::OrExpression(element)
    }
    fn make_expression_and_expression(&self, element: Self::AndExpression) -> Self::Expression {
        output::Expression::AndExpression(element)
    }
    fn make_expression_equality_expression(
        &self,
        element: Self::EqualityExpression,
    ) -> Self::Expression {
        output::Expression::EqualityExpression(element)
    }
    fn make_expression_inequality_expression(
        &self,
        element: Self::InequalityExpression,
    ) -> Self::Expression {
        output::Expression::InequalityExpression(element)
    }
    fn make_expression_bitwise_or_expression(
        &self,
        element: Self::BitwiseOrExpression,
    ) -> Self::Expression {
        output::Expression::BitwiseOrExpression(element)
    }
    fn make_expression_bitwise_xor_expression(
        &self,
        element: Self::BitwiseXorExpression,
    ) -> Self::Expression {
        output::Expression::BitwiseXorExpression(element)
    }
    fn make_expression_bitwise_and_expression(
        &self,
        element: Self::BitwiseAndExpression,
    ) -> Self::Expression {
        output::Expression::BitwiseAndExpression(element)
    }
    fn make_expression_shift_expression(&self, element: Self::ShiftExpression) -> Self::Expression {
        output::Expression::ShiftExpression(element)
    }
    fn make_expression_additive_expression(
        &self,
        element: Self::AdditiveExpression,
    ) -> Self::Expression {
        output::Expression::AdditiveExpression(element)
    }
    fn make_expression_multiplicative_expression(
        &self,
        element: Self::MultiplicativeExpression,
    ) -> Self::Expression {
        output::Expression::MultiplicativeExpression(element)
    }
    fn make_expression_exponentiation_expression(
        &self,
        element: Self::ExponentiationExpression,
    ) -> Self::Expression {
        output::Expression::ExponentiationExpression(element)
    }
    fn make_expression_postfix_expression(
        &self,
        element: Self::PostfixExpression,
    ) -> Self::Expression {
        output::Expression::PostfixExpression(element)
    }
    fn make_expression_prefix_expression(
        &self,
        element: Self::PrefixExpression,
    ) -> Self::Expression {
        output::Expression::PrefixExpression(element)
    }
    fn make_expression_function_call_expression(
        &self,
        element: Self::FunctionCallExpression,
    ) -> Self::Expression {
        output::Expression::FunctionCallExpression(element)
    }
    fn make_expression_call_options_expression(
        &self,
        element: Self::CallOptionsExpression,
    ) -> Self::Expression {
        output::Expression::CallOptionsExpression(element)
    }
    fn make_expression_member_access_expression(
        &self,
        element: Self::MemberAccessExpression,
    ) -> Self::Expression {
        output::Expression::MemberAccessExpression(element)
    }
    fn make_expression_index_access_expression(
        &self,
        element: Self::IndexAccessExpression,
    ) -> Self::Expression {
        output::Expression::IndexAccessExpression(element)
    }
    fn make_expression_new_expression(&self, element: Self::NewExpression) -> Self::Expression {
        output::Expression::NewExpression(element)
    }
    fn make_expression_tuple_expression(&self, element: Self::TupleExpression) -> Self::Expression {
        output::Expression::TupleExpression(element)
    }
    fn make_expression_type_expression(&self, element: Self::TypeExpression) -> Self::Expression {
        output::Expression::TypeExpression(element)
    }
    fn make_expression_array_expression(&self, element: Self::ArrayExpression) -> Self::Expression {
        output::Expression::ArrayExpression(element)
    }
    fn make_expression_hex_number_expression(
        &self,
        element: Self::HexNumberExpression,
    ) -> Self::Expression {
        output::Expression::HexNumberExpression(element)
    }
    fn make_expression_decimal_number_expression(
        &self,
        element: Self::DecimalNumberExpression,
    ) -> Self::Expression {
        output::Expression::DecimalNumberExpression(element)
    }
    fn make_expression_string_expression(
        &self,
        element: Self::StringExpression,
    ) -> Self::Expression {
        output::Expression::StringExpression(element)
    }
    fn make_expression_elementary_type(&self, element: Self::ElementaryType) -> Self::Expression {
        output::Expression::ElementaryType(element)
    }
    fn make_expression_payable_keyword(&self, element: Self::PayableKeyword) -> Self::Expression {
        output::Expression::PayableKeyword
    }
    fn make_expression_this_keyword(&self, element: Self::ThisKeyword) -> Self::Expression {
        output::Expression::ThisKeyword
    }
    fn make_expression_super_keyword(&self, element: Self::SuperKeyword) -> Self::Expression {
        output::Expression::SuperKeyword
    }
    fn make_expression_true_keyword(&self, element: Self::TrueKeyword) -> Self::Expression {
        output::Expression::TrueKeyword
    }
    fn make_expression_false_keyword(&self, element: Self::FalseKeyword) -> Self::Expression {
        output::Expression::FalseKeyword
    }
    fn make_expression_identifier(&self, element: Self::Identifier) -> Self::Expression {
        output::Expression::Identifier(element)
    }

    fn make_expression_additive_expression_operator_minus(
        &self,
        element: Self::Minus,
    ) -> Self::Expression_AdditiveExpression_Operator {
        output::Expression_AdditiveExpression_Operator::Minus
    }
    fn make_expression_additive_expression_operator_plus(
        &self,
        element: Self::Plus,
    ) -> Self::Expression_AdditiveExpression_Operator {
        output::Expression_AdditiveExpression_Operator::Plus
    }

    fn make_expression_assignment_expression_operator_ampersand_equal(
        &self,
        element: Self::AmpersandEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::AmpersandEqual
    }
    fn make_expression_assignment_expression_operator_asterisk_equal(
        &self,
        element: Self::AsteriskEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::AsteriskEqual
    }
    fn make_expression_assignment_expression_operator_bar_equal(
        &self,
        element: Self::BarEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::BarEqual
    }
    fn make_expression_assignment_expression_operator_caret_equal(
        &self,
        element: Self::CaretEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::CaretEqual
    }
    fn make_expression_assignment_expression_operator_equal(
        &self,
        element: Self::Equal,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::Equal
    }
    fn make_expression_assignment_expression_operator_greater_than_greater_than_equal(
        &self,
        element: Self::GreaterThanGreaterThanEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual
    }
    fn make_expression_assignment_expression_operator_greater_than_greater_than_greater_than_equal(
        &self,
        element: Self::GreaterThanGreaterThanGreaterThanEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual
    }
    fn make_expression_assignment_expression_operator_less_than_less_than_equal(
        &self,
        element: Self::LessThanLessThanEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::LessThanLessThanEqual
    }
    fn make_expression_assignment_expression_operator_minus_equal(
        &self,
        element: Self::MinusEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::MinusEqual
    }
    fn make_expression_assignment_expression_operator_percent_equal(
        &self,
        element: Self::PercentEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::PercentEqual
    }
    fn make_expression_assignment_expression_operator_plus_equal(
        &self,
        element: Self::PlusEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::PlusEqual
    }
    fn make_expression_assignment_expression_operator_slash_equal(
        &self,
        element: Self::SlashEqual,
    ) -> Self::Expression_AssignmentExpression_Operator {
        output::Expression_AssignmentExpression_Operator::SlashEqual
    }

    fn make_expression_equality_expression_operator_bang_equal(
        &self,
        element: Self::BangEqual,
    ) -> Self::Expression_EqualityExpression_Operator {
        output::Expression_EqualityExpression_Operator::BangEqual
    }
    fn make_expression_equality_expression_operator_equal_equal(
        &self,
        element: Self::EqualEqual,
    ) -> Self::Expression_EqualityExpression_Operator {
        output::Expression_EqualityExpression_Operator::EqualEqual
    }

    fn make_expression_inequality_expression_operator_greater_than(
        &self,
        element: Self::GreaterThan,
    ) -> Self::Expression_InequalityExpression_Operator {
        output::Expression_InequalityExpression_Operator::GreaterThan
    }
    fn make_expression_inequality_expression_operator_greater_than_equal(
        &self,
        element: Self::GreaterThanEqual,
    ) -> Self::Expression_InequalityExpression_Operator {
        output::Expression_InequalityExpression_Operator::GreaterThanEqual
    }
    fn make_expression_inequality_expression_operator_less_than(
        &self,
        element: Self::LessThan,
    ) -> Self::Expression_InequalityExpression_Operator {
        output::Expression_InequalityExpression_Operator::LessThan
    }
    fn make_expression_inequality_expression_operator_less_than_equal(
        &self,
        element: Self::LessThanEqual,
    ) -> Self::Expression_InequalityExpression_Operator {
        output::Expression_InequalityExpression_Operator::LessThanEqual
    }

    fn make_expression_multiplicative_expression_operator_asterisk(
        &self,
        element: Self::Asterisk,
    ) -> Self::Expression_MultiplicativeExpression_Operator {
        output::Expression_MultiplicativeExpression_Operator::Asterisk
    }
    fn make_expression_multiplicative_expression_operator_percent(
        &self,
        element: Self::Percent,
    ) -> Self::Expression_MultiplicativeExpression_Operator {
        output::Expression_MultiplicativeExpression_Operator::Percent
    }
    fn make_expression_multiplicative_expression_operator_slash(
        &self,
        element: Self::Slash,
    ) -> Self::Expression_MultiplicativeExpression_Operator {
        output::Expression_MultiplicativeExpression_Operator::Slash
    }

    fn make_expression_postfix_expression_operator_minus_minus(
        &self,
        element: Self::MinusMinus,
    ) -> Self::Expression_PostfixExpression_Operator {
        output::Expression_PostfixExpression_Operator::MinusMinus
    }
    fn make_expression_postfix_expression_operator_plus_plus(
        &self,
        element: Self::PlusPlus,
    ) -> Self::Expression_PostfixExpression_Operator {
        output::Expression_PostfixExpression_Operator::PlusPlus
    }

    fn make_expression_prefix_expression_operator_bang(
        &self,
        element: Self::Bang,
    ) -> Self::Expression_PrefixExpression_Operator {
        output::Expression_PrefixExpression_Operator::Bang
    }
    fn make_expression_prefix_expression_operator_delete_keyword(
        &self,
        element: Self::DeleteKeyword,
    ) -> Self::Expression_PrefixExpression_Operator {
        output::Expression_PrefixExpression_Operator::DeleteKeyword
    }
    fn make_expression_prefix_expression_operator_minus(
        &self,
        element: Self::Minus,
    ) -> Self::Expression_PrefixExpression_Operator {
        output::Expression_PrefixExpression_Operator::Minus
    }
    fn make_expression_prefix_expression_operator_minus_minus(
        &self,
        element: Self::MinusMinus,
    ) -> Self::Expression_PrefixExpression_Operator {
        output::Expression_PrefixExpression_Operator::MinusMinus
    }
    fn make_expression_prefix_expression_operator_plus_plus(
        &self,
        element: Self::PlusPlus,
    ) -> Self::Expression_PrefixExpression_Operator {
        output::Expression_PrefixExpression_Operator::PlusPlus
    }
    fn make_expression_prefix_expression_operator_tilde(
        &self,
        element: Self::Tilde,
    ) -> Self::Expression_PrefixExpression_Operator {
        output::Expression_PrefixExpression_Operator::Tilde
    }

    fn make_expression_shift_expression_operator_greater_than_greater_than(
        &self,
        element: Self::GreaterThanGreaterThan,
    ) -> Self::Expression_ShiftExpression_Operator {
        output::Expression_ShiftExpression_Operator::GreaterThanGreaterThan
    }
    fn make_expression_shift_expression_operator_greater_than_greater_than_greater_than(
        &self,
        element: Self::GreaterThanGreaterThanGreaterThan,
    ) -> Self::Expression_ShiftExpression_Operator {
        output::Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan
    }
    fn make_expression_shift_expression_operator_less_than_less_than(
        &self,
        element: Self::LessThanLessThan,
    ) -> Self::Expression_ShiftExpression_Operator {
        output::Expression_ShiftExpression_Operator::LessThanLessThan
    }

    fn make_fallback_function_attribute_modifier_invocation(
        &self,
        element: Self::ModifierInvocation,
    ) -> Self::FallbackFunctionAttribute {
        self.build_fallback_function_attribute_modifier_invocation_impl(element)
    }
    fn make_fallback_function_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::FallbackFunctionAttribute {
        self.build_fallback_function_attribute_override_specifier_impl(element)
    }
    fn make_fallback_function_attribute_external_keyword(
        &self,
        element: Self::ExternalKeyword,
    ) -> Self::FallbackFunctionAttribute {
        self.build_fallback_function_attribute_external_keyword_impl(element)
    }
    fn make_fallback_function_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::FallbackFunctionAttribute {
        self.build_fallback_function_attribute_payable_keyword_impl(element)
    }
    fn make_fallback_function_attribute_pure_keyword(
        &self,
        element: Self::PureKeyword,
    ) -> Self::FallbackFunctionAttribute {
        self.build_fallback_function_attribute_pure_keyword_impl(element)
    }
    fn make_fallback_function_attribute_view_keyword(
        &self,
        element: Self::ViewKeyword,
    ) -> Self::FallbackFunctionAttribute {
        self.build_fallback_function_attribute_view_keyword_impl(element)
    }
    fn make_fallback_function_attribute_virtual_keyword(
        &self,
        element: Self::VirtualKeyword,
    ) -> Self::FallbackFunctionAttribute {
        self.build_fallback_function_attribute_virtual_keyword_impl(element)
    }

    fn make_for_statement_condition_expression_statement(
        &self,
        element: Self::ExpressionStatement,
    ) -> Self::ForStatementCondition {
        output::ForStatementCondition::ExpressionStatement(element)
    }
    fn make_for_statement_condition_semicolon(
        &self,
        element: Self::Semicolon,
    ) -> Self::ForStatementCondition {
        output::ForStatementCondition::Semicolon
    }

    fn make_for_statement_initialization_variable_declaration_statement(
        &self,
        element: Self::VariableDeclarationStatement,
    ) -> Self::ForStatementInitialization {
        output::ForStatementInitialization::VariableDeclarationStatement(element)
    }
    fn make_for_statement_initialization_expression_statement(
        &self,
        element: Self::ExpressionStatement,
    ) -> Self::ForStatementInitialization {
        output::ForStatementInitialization::ExpressionStatement(element)
    }
    fn make_for_statement_initialization_semicolon(
        &self,
        element: Self::Semicolon,
    ) -> Self::ForStatementInitialization {
        output::ForStatementInitialization::Semicolon
    }

    fn make_function_attribute_modifier_invocation(
        &self,
        element: Self::ModifierInvocation,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_modifier_invocation_impl(element)
    }
    fn make_function_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_override_specifier_impl(element)
    }
    fn make_function_attribute_external_keyword(
        &self,
        element: Self::ExternalKeyword,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_external_keyword_impl(element)
    }
    fn make_function_attribute_internal_keyword(
        &self,
        element: Self::InternalKeyword,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_internal_keyword_impl(element)
    }
    fn make_function_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_payable_keyword_impl(element)
    }
    fn make_function_attribute_private_keyword(
        &self,
        element: Self::PrivateKeyword,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_private_keyword_impl(element)
    }
    fn make_function_attribute_public_keyword(
        &self,
        element: Self::PublicKeyword,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_public_keyword_impl(element)
    }
    fn make_function_attribute_pure_keyword(
        &self,
        element: Self::PureKeyword,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_pure_keyword_impl(element)
    }
    fn make_function_attribute_view_keyword(
        &self,
        element: Self::ViewKeyword,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_view_keyword_impl(element)
    }
    fn make_function_attribute_virtual_keyword(
        &self,
        element: Self::VirtualKeyword,
    ) -> Self::FunctionAttribute {
        self.build_function_attribute_virtual_keyword_impl(element)
    }

    fn make_function_body_block(&self, element: Self::Block) -> Self::FunctionBody {
        self.build_function_body_block_impl(element)
    }
    fn make_function_body_semicolon(&self, element: Self::Semicolon) -> Self::FunctionBody {
        self.build_function_body_semicolon_impl(element)
    }

    fn make_function_name_identifier(&self, element: Self::Identifier) -> Self::FunctionName {
        self.build_function_name_identifier_impl(element)
    }
    fn make_function_name_fallback_keyword(
        &self,
        element: Self::FallbackKeyword,
    ) -> Self::FunctionName {
        self.build_function_name_fallback_keyword_impl(element)
    }
    fn make_function_name_receive_keyword(
        &self,
        element: Self::ReceiveKeyword,
    ) -> Self::FunctionName {
        self.build_function_name_receive_keyword_impl(element)
    }

    fn make_function_type_attribute_internal_keyword(
        &self,
        element: Self::InternalKeyword,
    ) -> Self::FunctionTypeAttribute {
        self.build_function_type_attribute_internal_keyword_impl(element)
    }
    fn make_function_type_attribute_external_keyword(
        &self,
        element: Self::ExternalKeyword,
    ) -> Self::FunctionTypeAttribute {
        self.build_function_type_attribute_external_keyword_impl(element)
    }
    fn make_function_type_attribute_private_keyword(
        &self,
        element: Self::PrivateKeyword,
    ) -> Self::FunctionTypeAttribute {
        self.build_function_type_attribute_private_keyword_impl(element)
    }
    fn make_function_type_attribute_public_keyword(
        &self,
        element: Self::PublicKeyword,
    ) -> Self::FunctionTypeAttribute {
        self.build_function_type_attribute_public_keyword_impl(element)
    }
    fn make_function_type_attribute_pure_keyword(
        &self,
        element: Self::PureKeyword,
    ) -> Self::FunctionTypeAttribute {
        self.build_function_type_attribute_pure_keyword_impl(element)
    }
    fn make_function_type_attribute_view_keyword(
        &self,
        element: Self::ViewKeyword,
    ) -> Self::FunctionTypeAttribute {
        self.build_function_type_attribute_view_keyword_impl(element)
    }
    fn make_function_type_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::FunctionTypeAttribute {
        self.build_function_type_attribute_payable_keyword_impl(element)
    }

    fn make_identifier_path_element_identifier(
        &self,
        element: Self::Identifier,
    ) -> Self::IdentifierPathElement {
        element
    }
    fn make_identifier_path_element_address_keyword(
        &self,
        element: Self::AddressKeyword,
    ) -> Self::IdentifierPathElement {
        self.build_identifier_path_element_address_keyword_impl(element)
    }

    fn make_import_clause_path_import(&self, element: Self::PathImport) -> Self::ImportClause {
        output::ImportClause::PathImport(element)
    }
    fn make_import_clause_named_import(&self, element: Self::NamedImport) -> Self::ImportClause {
        self.build_import_clause_named_import_impl(element)
    }
    fn make_import_clause_import_deconstruction(
        &self,
        element: Self::ImportDeconstruction,
    ) -> Self::ImportClause {
        output::ImportClause::ImportDeconstruction(element)
    }

    fn make_mapping_key_type_elementary_type(
        &self,
        element: Self::ElementaryType,
    ) -> Self::MappingKeyType {
        self.build_mapping_key_type_elementary_type_impl(element)
    }
    fn make_mapping_key_type_identifier_path(
        &self,
        element: Self::IdentifierPath,
    ) -> Self::MappingKeyType {
        self.build_mapping_key_type_identifier_path_impl(element)
    }

    fn make_modifier_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::ModifierAttribute {
        self.build_modifier_attribute_override_specifier_impl(element)
    }
    fn make_modifier_attribute_virtual_keyword(
        &self,
        element: Self::VirtualKeyword,
    ) -> Self::ModifierAttribute {
        self.build_modifier_attribute_virtual_keyword_impl(element)
    }

    fn make_number_unit_wei_keyword(&self, element: Self::WeiKeyword) -> Self::NumberUnit {
        output::NumberUnit::WeiKeyword
    }
    fn make_number_unit_gwei_keyword(&self, element: Self::GweiKeyword) -> Self::NumberUnit {
        output::NumberUnit::GweiKeyword
    }
    fn make_number_unit_ether_keyword(&self, element: Self::EtherKeyword) -> Self::NumberUnit {
        output::NumberUnit::EtherKeyword
    }
    fn make_number_unit_seconds_keyword(&self, element: Self::SecondsKeyword) -> Self::NumberUnit {
        output::NumberUnit::SecondsKeyword
    }
    fn make_number_unit_minutes_keyword(&self, element: Self::MinutesKeyword) -> Self::NumberUnit {
        output::NumberUnit::MinutesKeyword
    }
    fn make_number_unit_hours_keyword(&self, element: Self::HoursKeyword) -> Self::NumberUnit {
        output::NumberUnit::HoursKeyword
    }
    fn make_number_unit_days_keyword(&self, element: Self::DaysKeyword) -> Self::NumberUnit {
        output::NumberUnit::DaysKeyword
    }
    fn make_number_unit_weeks_keyword(&self, element: Self::WeeksKeyword) -> Self::NumberUnit {
        output::NumberUnit::WeeksKeyword
    }

    fn make_pragma_version_pragma(&self, element: Self::VersionPragma) -> Self::Pragma {
        output::Pragma::VersionPragma(element)
    }
    fn make_pragma_abicoder_pragma(&self, element: Self::AbicoderPragma) -> Self::Pragma {
        output::Pragma::AbicoderPragma(element)
    }
    fn make_pragma_experimental_pragma(&self, element: Self::ExperimentalPragma) -> Self::Pragma {
        output::Pragma::ExperimentalPragma(element)
    }

    fn make_receive_function_attribute_modifier_invocation(
        &self,
        element: Self::ModifierInvocation,
    ) -> Self::ReceiveFunctionAttribute {
        self.build_receive_function_attribute_modifier_invocation_impl(element)
    }
    fn make_receive_function_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::ReceiveFunctionAttribute {
        self.build_receive_function_attribute_override_specifier_impl(element)
    }
    fn make_receive_function_attribute_external_keyword(
        &self,
        element: Self::ExternalKeyword,
    ) -> Self::ReceiveFunctionAttribute {
        self.build_receive_function_attribute_external_keyword_impl(element)
    }
    fn make_receive_function_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::ReceiveFunctionAttribute {
        self.build_receive_function_attribute_payable_keyword_impl(element)
    }
    fn make_receive_function_attribute_virtual_keyword(
        &self,
        element: Self::VirtualKeyword,
    ) -> Self::ReceiveFunctionAttribute {
        self.build_receive_function_attribute_virtual_keyword_impl(element)
    }

    fn make_source_unit_member_pragma_directive(
        &self,
        element: Self::PragmaDirective,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::PragmaDirective(element)
    }
    fn make_source_unit_member_import_directive(
        &self,
        element: Self::ImportDirective,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::ImportClause(element)
    }
    fn make_source_unit_member_contract_definition(
        &self,
        element: Self::ContractDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::ContractDefinition(element)
    }
    fn make_source_unit_member_interface_definition(
        &self,
        element: Self::InterfaceDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::InterfaceDefinition(element)
    }
    fn make_source_unit_member_library_definition(
        &self,
        element: Self::LibraryDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::LibraryDefinition(element)
    }
    fn make_source_unit_member_struct_definition(
        &self,
        element: Self::StructDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::StructDefinition(element)
    }
    fn make_source_unit_member_enum_definition(
        &self,
        element: Self::EnumDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::EnumDefinition(element)
    }
    fn make_source_unit_member_function_definition(
        &self,
        element: Self::FunctionDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::FunctionDefinition(element)
    }
    fn make_source_unit_member_error_definition(
        &self,
        element: Self::ErrorDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::ErrorDefinition(element)
    }
    fn make_source_unit_member_user_defined_value_type_definition(
        &self,
        element: Self::UserDefinedValueTypeDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::UserDefinedValueTypeDefinition(element)
    }
    fn make_source_unit_member_using_directive(
        &self,
        element: Self::UsingDirective,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::UsingDirective(element)
    }
    fn make_source_unit_member_event_definition(
        &self,
        element: Self::EventDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::EventDefinition(element)
    }
    fn make_source_unit_member_constant_definition(
        &self,
        element: Self::ConstantDefinition,
    ) -> Self::SourceUnitMember {
        output::SourceUnitMember::ConstantDefinition(element)
    }

    fn make_state_variable_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::StateVariableAttribute {
        self.build_state_variable_attribute_override_specifier_impl(element)
    }
    fn make_state_variable_attribute_constant_keyword(
        &self,
        element: Self::ConstantKeyword,
    ) -> Self::StateVariableAttribute {
        self.build_state_variable_attribute_constant_keyword_impl(element)
    }
    fn make_state_variable_attribute_internal_keyword(
        &self,
        element: Self::InternalKeyword,
    ) -> Self::StateVariableAttribute {
        self.build_state_variable_attribute_internal_keyword_impl(element)
    }
    fn make_state_variable_attribute_private_keyword(
        &self,
        element: Self::PrivateKeyword,
    ) -> Self::StateVariableAttribute {
        self.build_state_variable_attribute_private_keyword_impl(element)
    }
    fn make_state_variable_attribute_public_keyword(
        &self,
        element: Self::PublicKeyword,
    ) -> Self::StateVariableAttribute {
        self.build_state_variable_attribute_public_keyword_impl(element)
    }
    fn make_state_variable_attribute_immutable_keyword(
        &self,
        element: Self::ImmutableKeyword,
    ) -> Self::StateVariableAttribute {
        self.build_state_variable_attribute_immutable_keyword_impl(element)
    }
    fn make_state_variable_attribute_transient_keyword(
        &self,
        element: Self::TransientKeyword,
    ) -> Self::StateVariableAttribute {
        self.build_state_variable_attribute_transient_keyword_impl(element)
    }

    fn make_statement_if_statement(&self, element: Self::IfStatement) -> Self::Statement {
        output::Statement::IfStatement(element)
    }
    fn make_statement_for_statement(&self, element: Self::ForStatement) -> Self::Statement {
        output::Statement::ForStatement(element)
    }
    fn make_statement_while_statement(&self, element: Self::WhileStatement) -> Self::Statement {
        output::Statement::WhileStatement(element)
    }
    fn make_statement_do_while_statement(
        &self,
        element: Self::DoWhileStatement,
    ) -> Self::Statement {
        output::Statement::DoWhileStatement(element)
    }
    fn make_statement_continue_statement(
        &self,
        element: Self::ContinueStatement,
    ) -> Self::Statement {
        output::Statement::ContinueStatement(element)
    }
    fn make_statement_break_statement(&self, element: Self::BreakStatement) -> Self::Statement {
        output::Statement::BreakStatement(element)
    }
    fn make_statement_return_statement(&self, element: Self::ReturnStatement) -> Self::Statement {
        output::Statement::ReturnStatement(element)
    }
    fn make_statement_emit_statement(&self, element: Self::EmitStatement) -> Self::Statement {
        output::Statement::EmitStatement(element)
    }
    fn make_statement_try_statement(&self, element: Self::TryStatement) -> Self::Statement {
        output::Statement::TryStatement(element)
    }
    fn make_statement_revert_statement(&self, element: Self::RevertStatement) -> Self::Statement {
        output::Statement::RevertStatement(element)
    }
    fn make_statement_assembly_statement(
        &self,
        element: Self::AssemblyStatement,
    ) -> Self::Statement {
        output::Statement::AssemblyStatement(element)
    }
    fn make_statement_block(&self, element: Self::Block) -> Self::Statement {
        output::Statement::Block(element)
    }
    fn make_statement_unchecked_block(&self, element: Self::UncheckedBlock) -> Self::Statement {
        output::Statement::UncheckedBlock(element)
    }
    fn make_statement_variable_declaration_statement(
        &self,
        element: Self::VariableDeclarationStatement,
    ) -> Self::Statement {
        output::Statement::VariableDeclarationStatement(element)
    }
    fn make_statement_expression_statement(
        &self,
        element: Self::ExpressionStatement,
    ) -> Self::Statement {
        output::Statement::ExpressionStatement(element)
    }

    fn make_storage_location_memory_keyword(
        &self,
        element: Self::MemoryKeyword,
    ) -> Self::StorageLocation {
        output::StorageLocation::MemoryKeyword
    }
    fn make_storage_location_storage_keyword(
        &self,
        element: Self::StorageKeyword,
    ) -> Self::StorageLocation {
        output::StorageLocation::StorageKeyword
    }
    fn make_storage_location_call_data_keyword(
        &self,
        element: Self::CallDataKeyword,
    ) -> Self::StorageLocation {
        output::StorageLocation::CallDataKeyword
    }

    fn make_string_expression_string_literals(
        &self,
        element: Self::StringLiterals,
    ) -> Self::StringExpression {
        output::StringExpression::StringLiterals(element)
    }
    fn make_string_expression_hex_string_literals(
        &self,
        element: Self::HexStringLiterals,
    ) -> Self::StringExpression {
        output::StringExpression::HexStringLiterals(element)
    }
    fn make_string_expression_unicode_string_literals(
        &self,
        element: Self::UnicodeStringLiterals,
    ) -> Self::StringExpression {
        output::StringExpression::UnicodeStringLiterals(element)
    }

    fn make_type_name_array_type_name(&self, element: Self::ArrayTypeName) -> Self::TypeName {
        output::TypeName::ArrayTypeName(element)
    }
    fn make_type_name_function_type(&self, element: Self::FunctionType) -> Self::TypeName {
        output::TypeName::FunctionType(element.into_output())
    }
    fn make_type_name_mapping_type(&self, element: Self::MappingType) -> Self::TypeName {
        output::TypeName::MappingType(element)
    }
    fn make_type_name_elementary_type(&self, element: Self::ElementaryType) -> Self::TypeName {
        output::TypeName::ElementaryType(element)
    }
    fn make_type_name_identifier_path(&self, element: Self::IdentifierPath) -> Self::TypeName {
        output::TypeName::IdentifierPath(element)
    }

    fn make_using_clause_identifier_path(
        &self,
        element: Self::IdentifierPath,
    ) -> Self::UsingClause {
        output::UsingClause::IdentifierPath(element)
    }
    fn make_using_clause_using_deconstruction(
        &self,
        element: Self::UsingDeconstruction,
    ) -> Self::UsingClause {
        output::UsingClause::UsingDeconstruction(element)
    }

    fn make_using_operator_ampersand(&self, element: Self::Ampersand) -> Self::UsingOperator {
        output::UsingOperator::Ampersand
    }
    fn make_using_operator_asterisk(&self, element: Self::Asterisk) -> Self::UsingOperator {
        output::UsingOperator::Asterisk
    }
    fn make_using_operator_bang_equal(&self, element: Self::BangEqual) -> Self::UsingOperator {
        output::UsingOperator::BangEqual
    }
    fn make_using_operator_bar(&self, element: Self::Bar) -> Self::UsingOperator {
        output::UsingOperator::Bar
    }
    fn make_using_operator_caret(&self, element: Self::Caret) -> Self::UsingOperator {
        output::UsingOperator::Caret
    }
    fn make_using_operator_equal_equal(&self, element: Self::EqualEqual) -> Self::UsingOperator {
        output::UsingOperator::EqualEqual
    }
    fn make_using_operator_greater_than(&self, element: Self::GreaterThan) -> Self::UsingOperator {
        output::UsingOperator::GreaterThan
    }
    fn make_using_operator_greater_than_equal(
        &self,
        element: Self::GreaterThanEqual,
    ) -> Self::UsingOperator {
        output::UsingOperator::GreaterThanEqual
    }
    fn make_using_operator_less_than(&self, element: Self::LessThan) -> Self::UsingOperator {
        output::UsingOperator::LessThan
    }
    fn make_using_operator_less_than_equal(
        &self,
        element: Self::LessThanEqual,
    ) -> Self::UsingOperator {
        output::UsingOperator::LessThanEqual
    }
    fn make_using_operator_minus(&self, element: Self::Minus) -> Self::UsingOperator {
        output::UsingOperator::Minus
    }
    fn make_using_operator_percent(&self, element: Self::Percent) -> Self::UsingOperator {
        output::UsingOperator::Percent
    }
    fn make_using_operator_plus(&self, element: Self::Plus) -> Self::UsingOperator {
        output::UsingOperator::Plus
    }
    fn make_using_operator_slash(&self, element: Self::Slash) -> Self::UsingOperator {
        output::UsingOperator::Slash
    }
    fn make_using_operator_tilde(&self, element: Self::Tilde) -> Self::UsingOperator {
        output::UsingOperator::Tilde
    }

    fn make_using_target_type_name(&self, element: Self::TypeName) -> Self::UsingTarget {
        output::UsingTarget::TypeName(element)
    }
    fn make_using_target_asterisk(&self, element: Self::Asterisk) -> Self::UsingTarget {
        output::UsingTarget::Asterisk
    }

    fn make_variable_declaration_target_single_typed_declaration(
        &self,
        element: Self::SingleTypedDeclaration,
    ) -> Self::VariableDeclarationTarget {
        output::VariableDeclarationTarget::SingleTypedDeclaration(element)
    }
    fn make_variable_declaration_target_multi_typed_declaration(
        &self,
        element: Self::MultiTypedDeclaration,
    ) -> Self::VariableDeclarationTarget {
        output::VariableDeclarationTarget::MultiTypedDeclaration(element)
    }

    fn make_version_expression_version_range(
        &self,
        element: Self::VersionRange,
    ) -> Self::VersionExpression {
        output::VersionExpression::VersionRange(element)
    }
    fn make_version_expression_version_term(
        &self,
        element: Self::VersionTerm,
    ) -> Self::VersionExpression {
        output::VersionExpression::VersionTerm(element)
    }

    fn make_version_literal_simple_version_literal(
        &self,
        element: Self::SimpleVersionLiteral,
    ) -> Self::VersionLiteral {
        output::VersionLiteral::SimpleVersionLiteral(element)
    }
    fn make_version_literal_pragma_string_literal(
        &self,
        element: Self::PragmaStringLiteral,
    ) -> Self::VersionLiteral {
        output::VersionLiteral::StringLiteral(element)
    }

    fn make_version_operator_pragma_caret(
        &self,
        element: Self::PragmaCaret,
    ) -> Self::VersionOperator {
        output::VersionOperator::PragmaCaret
    }
    fn make_version_operator_pragma_tilde(
        &self,
        element: Self::PragmaTilde,
    ) -> Self::VersionOperator {
        output::VersionOperator::PragmaTilde
    }
    fn make_version_operator_pragma_equal(
        &self,
        element: Self::PragmaEqual,
    ) -> Self::VersionOperator {
        output::VersionOperator::PragmaEqual
    }
    fn make_version_operator_pragma_less_than(
        &self,
        element: Self::PragmaLessThan,
    ) -> Self::VersionOperator {
        output::VersionOperator::PragmaLessThan
    }
    fn make_version_operator_pragma_greater_than(
        &self,
        element: Self::PragmaGreaterThan,
    ) -> Self::VersionOperator {
        output::VersionOperator::PragmaGreaterThan
    }
    fn make_version_operator_pragma_less_than_equal(
        &self,
        element: Self::PragmaLessThanEqual,
    ) -> Self::VersionOperator {
        output::VersionOperator::PragmaLessThanEqual
    }
    fn make_version_operator_pragma_greater_than_equal(
        &self,
        element: Self::PragmaGreaterThanEqual,
    ) -> Self::VersionOperator {
        output::VersionOperator::PragmaGreaterThanEqual
    }

    fn make_yul_expression_yul_function_call_expression(
        &self,
        element: Self::YulFunctionCallExpression,
    ) -> Self::YulExpression {
        output::YulExpression::YulFunctionCallExpression(element)
    }
    fn make_yul_expression_yul_literal(&self, element: Self::YulLiteral) -> Self::YulExpression {
        output::YulExpression::YulLiteral(element)
    }
    fn make_yul_expression_yul_path(&self, element: Self::YulPath) -> Self::YulExpression {
        output::YulExpression::YulPath(element)
    }

    fn make_yul_literal_yul_true_keyword(&self, element: Self::YulTrueKeyword) -> Self::YulLiteral {
        output::YulLiteral::TrueKeyword
    }
    fn make_yul_literal_yul_false_keyword(
        &self,
        element: Self::YulFalseKeyword,
    ) -> Self::YulLiteral {
        output::YulLiteral::FalseKeyword
    }
    fn make_yul_literal_yul_decimal_literal(
        &self,
        element: Self::YulDecimalLiteral,
    ) -> Self::YulLiteral {
        output::YulLiteral::DecimalLiteral(element)
    }
    fn make_yul_literal_yul_hex_literal(&self, element: Self::YulHexLiteral) -> Self::YulLiteral {
        output::YulLiteral::HexLiteral(element)
    }
    fn make_yul_literal_yul_hex_string_literal(
        &self,
        element: Self::YulHexStringLiteral,
    ) -> Self::YulLiteral {
        output::YulLiteral::HexStringLiteral(element)
    }
    fn make_yul_literal_yul_string_literal(
        &self,
        element: Self::YulStringLiteral,
    ) -> Self::YulLiteral {
        output::YulLiteral::StringLiteral(element)
    }

    fn make_yul_statement_yul_block(&self, element: Self::YulBlock) -> Self::YulStatement {
        output::YulStatement::YulBlock(element)
    }
    fn make_yul_statement_yul_function_definition(
        &self,
        element: Self::YulFunctionDefinition,
    ) -> Self::YulStatement {
        output::YulStatement::YulFunctionDefinition(element)
    }
    fn make_yul_statement_yul_if_statement(
        &self,
        element: Self::YulIfStatement,
    ) -> Self::YulStatement {
        output::YulStatement::YulIfStatement(element)
    }
    fn make_yul_statement_yul_for_statement(
        &self,
        element: Self::YulForStatement,
    ) -> Self::YulStatement {
        output::YulStatement::YulForStatement(element)
    }
    fn make_yul_statement_yul_switch_statement(
        &self,
        element: Self::YulSwitchStatement,
    ) -> Self::YulStatement {
        output::YulStatement::YulSwitchStatement(element)
    }
    fn make_yul_statement_yul_leave_statement(
        &self,
        element: Self::YulLeaveStatement,
    ) -> Self::YulStatement {
        output::YulStatement::YulLeaveStatement(element)
    }
    fn make_yul_statement_yul_break_statement(
        &self,
        element: Self::YulBreakStatement,
    ) -> Self::YulStatement {
        output::YulStatement::YulBreakStatement(element)
    }
    fn make_yul_statement_yul_continue_statement(
        &self,
        element: Self::YulContinueStatement,
    ) -> Self::YulStatement {
        output::YulStatement::YulContinueStatement(element)
    }
    fn make_yul_statement_yul_variable_assignment_statement(
        &self,
        element: Self::YulVariableAssignmentStatement,
    ) -> Self::YulStatement {
        output::YulStatement::YulVariableAssignmentStatement(element)
    }
    fn make_yul_statement_yul_variable_declaration_statement(
        &self,
        element: Self::YulVariableDeclarationStatement,
    ) -> Self::YulStatement {
        output::YulStatement::YulVariableDeclarationStatement(element)
    }
    fn make_yul_statement_yul_expression(
        &self,
        element: Self::YulExpression,
    ) -> Self::YulStatement {
        output::YulStatement::YulExpression(element)
    }

    fn make_yul_switch_case_yul_default_case(
        &self,
        element: Self::YulDefaultCase,
    ) -> Self::YulSwitchCase {
        output::YulSwitchCase::YulDefaultCase(element)
    }
    fn make_yul_switch_case_yul_value_case(
        &self,
        element: Self::YulValueCase,
    ) -> Self::YulSwitchCase {
        output::YulSwitchCase::YulValueCase(element)
    }

    // ============================================
    // === Special Helper Methods ===
    // ============================================

    fn make_type_name_from_index_access_path(
        &self,
        index_access_path: parser_helpers::IndexAccessPath<Self>,
    ) -> Self::TypeName
    where
        Self: Sized,
    {
        self.build_type_name_from_index_access_path_impl(index_access_path)
    }

    fn make_expression_from_index_access_path(
        &self,
        index_access_path: parser_helpers::IndexAccessPath<Self>,
    ) -> Self::Expression
    where
        Self: Sized,
    {
        self.build_expression_from_index_access_path_impl(index_access_path)
    }

    fn make_expression_from_identifier_path(
        &self,
        identifier_path: Self::IdentifierPath,
    ) -> Self::Expression {
        self.build_expression_from_identifier_path_impl(identifier_path)
    }

    fn extract_extra_attributes(
        &self,
        fun_type: Self::FunctionType,
    ) -> (Self::FunctionType, Vec<Self::StateVariableAttribute>) {
        self.extract_extra_attributes_impl(fun_type)
    }
}
