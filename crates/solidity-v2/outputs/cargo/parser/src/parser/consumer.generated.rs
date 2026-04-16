// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]

use std::ops::Range;

use crate::parser::parser_helpers;

/// A trait that abstracts over the construction of parser output nodes.
///
/// The LALRPOP grammar is generic over this trait: instead of calling concrete
/// CST constructors, it calls methods on the consumer. This allows alternative
/// consumers (e.g., direct IR builders) to use the same grammar without
/// materializing a full CST.
pub trait ParserConsumer {
    // ========================
    // === Associated Types ===
    // ========================

    // Terminals
    type ABIEncoderV2Keyword;
    type AbicoderKeyword;
    type AbicoderV1Keyword;
    type AbicoderV2Keyword;
    type AbstractKeyword;
    type AddressKeyword;
    type AfterKeyword;
    type AliasKeyword;
    type Ampersand;
    type AmpersandAmpersand;
    type AmpersandEqual;
    type AnonymousKeyword;
    type ApplyKeyword;
    type AsKeyword;
    type AssemblyKeyword;
    type Asterisk;
    type AsteriskAsterisk;
    type AsteriskEqual;
    type AtKeyword;
    type AutoKeyword;
    type Bang;
    type BangEqual;
    type Bar;
    type BarBar;
    type BarEqual;
    type BoolKeyword;
    type BreakKeyword;
    type ByteKeyword;
    type BytesKeyword;
    type CallDataKeyword;
    type Caret;
    type CaretEqual;
    type CaseKeyword;
    type CatchKeyword;
    type CloseBrace;
    type CloseBracket;
    type CloseParen;
    type Colon;
    type Comma;
    type ConstantKeyword;
    type ConstructorKeyword;
    type ContinueKeyword;
    type ContractKeyword;
    type CopyOfKeyword;
    type DaysKeyword;
    type DecimalLiteral;
    type DefaultKeyword;
    type DefineKeyword;
    type DeleteKeyword;
    type DoKeyword;
    type ElseKeyword;
    type EmitKeyword;
    type EnumKeyword;
    type Equal;
    type EqualEqual;
    type EqualGreaterThan;
    type ErrorKeyword;
    type EtherKeyword;
    type EventKeyword;
    type ExperimentalKeyword;
    type ExternalKeyword;
    type FallbackKeyword;
    type FalseKeyword;
    type FinalKeyword;
    type FixedKeyword;
    type ForKeyword;
    type FromKeyword;
    type FunctionKeyword;
    type GlobalKeyword;
    type GreaterThan;
    type GreaterThanEqual;
    type GreaterThanGreaterThan;
    type GreaterThanGreaterThanEqual;
    type GreaterThanGreaterThanGreaterThan;
    type GreaterThanGreaterThanGreaterThanEqual;
    type GweiKeyword;
    type HexKeyword;
    type HexLiteral;
    type HexStringLiteral;
    type HoursKeyword;
    type Identifier;
    type IfKeyword;
    type ImmutableKeyword;
    type ImplementsKeyword;
    type ImportKeyword;
    type InKeyword;
    type IndexedKeyword;
    type InlineKeyword;
    type IntKeyword;
    type InterfaceKeyword;
    type InternalKeyword;
    type IsKeyword;
    type LayoutKeyword;
    type LessThan;
    type LessThanEqual;
    type LessThanLessThan;
    type LessThanLessThanEqual;
    type LetKeyword;
    type LibraryKeyword;
    type MacroKeyword;
    type MappingKeyword;
    type MatchKeyword;
    type MemoryKeyword;
    type Minus;
    type MinusEqual;
    type MinusMinus;
    type MinutesKeyword;
    type ModifierKeyword;
    type MutableKeyword;
    type NewKeyword;
    type NullKeyword;
    type OfKeyword;
    type OpenBrace;
    type OpenBracket;
    type OpenParen;
    type OverrideKeyword;
    type PartialKeyword;
    type PayableKeyword;
    type Percent;
    type PercentEqual;
    type Period;
    type Plus;
    type PlusEqual;
    type PlusPlus;
    type PragmaBarBar;
    type PragmaCaret;
    type PragmaEqual;
    type PragmaGreaterThan;
    type PragmaGreaterThanEqual;
    type PragmaKeyword;
    type PragmaLessThan;
    type PragmaLessThanEqual;
    type PragmaMinus;
    type PragmaPeriod;
    type PragmaSemicolon;
    type PragmaStringLiteral;
    type PragmaTilde;
    type PrivateKeyword;
    type PromiseKeyword;
    type PublicKeyword;
    type PureKeyword;
    type QuestionMark;
    type ReceiveKeyword;
    type ReferenceKeyword;
    type RelocatableKeyword;
    type ReturnKeyword;
    type ReturnsKeyword;
    type RevertKeyword;
    type SMTCheckerKeyword;
    type SealedKeyword;
    type SecondsKeyword;
    type Semicolon;
    type SizeOfKeyword;
    type Slash;
    type SlashEqual;
    type SolidityKeyword;
    type StaticKeyword;
    type StorageKeyword;
    type StringKeyword;
    type StringLiteral;
    type StructKeyword;
    type SuperKeyword;
    type SupportsKeyword;
    type SwitchKeyword;
    type ThisKeyword;
    type ThrowKeyword;
    type Tilde;
    type TransientKeyword;
    type TrueKeyword;
    type TryKeyword;
    type TypeDefKeyword;
    type TypeKeyword;
    type TypeOfKeyword;
    type UfixedKeyword;
    type UintKeyword;
    type UncheckedKeyword;
    type UnicodeStringLiteral;
    type UsingKeyword;
    type VarKeyword;
    type VersionSpecifier;
    type ViewKeyword;
    type VirtualKeyword;
    type WeeksKeyword;
    type WeiKeyword;
    type WhileKeyword;
    type YearsKeyword;
    type YulBreakKeyword;
    type YulCaseKeyword;
    type YulCloseBrace;
    type YulCloseParen;
    type YulColonEqual;
    type YulComma;
    type YulContinueKeyword;
    type YulDecimalLiteral;
    type YulDefaultKeyword;
    type YulFalseKeyword;
    type YulForKeyword;
    type YulFunctionKeyword;
    type YulHexKeyword;
    type YulHexLiteral;
    type YulHexStringLiteral;
    type YulIdentifier;
    type YulIfKeyword;
    type YulLeaveKeyword;
    type YulLetKeyword;
    type YulMinusGreaterThan;
    type YulOpenBrace;
    type YulOpenParen;
    type YulPeriod;
    type YulStringLiteral;
    type YulSuperKeyword;
    type YulSwitchKeyword;
    type YulThisKeyword;
    type YulTrueKeyword;

    // Sequences
    type AbicoderPragma;
    type AdditiveExpression;
    type AddressType;
    type AndExpression;
    type ArrayExpression;
    type ArrayTypeName;
    type AssemblyStatement;
    type AssignmentExpression;
    type BitwiseAndExpression;
    type BitwiseOrExpression;
    type BitwiseXorExpression;
    type Block;
    type BreakStatement;
    type CallOptionsExpression;
    type CatchClause;
    type CatchClauseError;
    type ConditionalExpression;
    type ConstantDefinition;
    type ConstructorDefinition;
    type ContinueStatement;
    type ContractDefinition;
    type DecimalNumberExpression;
    type DoWhileStatement;
    type ElseBranch;
    type EmitStatement;
    type EnumDefinition;
    type EqualityExpression;
    type ErrorDefinition;
    type ErrorParameter;
    type ErrorParametersDeclaration;
    type EventDefinition;
    type EventParameter;
    type EventParametersDeclaration;
    type ExperimentalPragma;
    type ExponentiationExpression;
    type ExpressionStatement;
    type FallbackFunctionDefinition;
    type ForStatement;
    type FunctionCallExpression;
    type FunctionDefinition;
    type FunctionType;
    type HexNumberExpression;
    type IfStatement;
    type ImportAlias;
    type ImportDeconstruction;
    type ImportDeconstructionSymbol;
    type ImportDirective;
    type IndexAccessEnd;
    type IndexAccessExpression;
    type InequalityExpression;
    type InheritanceSpecifier;
    type InheritanceType;
    type InterfaceDefinition;
    type LibraryDefinition;
    type MappingKey;
    type MappingType;
    type MappingValue;
    type MemberAccessExpression;
    type ModifierDefinition;
    type ModifierInvocation;
    type MultiTypedDeclaration;
    type MultiTypedDeclarationElement;
    type MultiplicativeExpression;
    type NamedArgument;
    type NamedArgumentGroup;
    type NamedArgumentsDeclaration;
    type NamedImport;
    type NewExpression;
    type OrExpression;
    type OverridePathsDeclaration;
    type OverrideSpecifier;
    type Parameter;
    type ParametersDeclaration;
    type PathImport;
    type PositionalArgumentsDeclaration;
    type PostfixExpression;
    type PragmaDirective;
    type PrefixExpression;
    type ReceiveFunctionDefinition;
    type ReturnStatement;
    type ReturnsDeclaration;
    type RevertStatement;
    type ShiftExpression;
    type SingleTypedDeclaration;
    type SourceUnit;
    type StateVariableDefinition;
    type StateVariableDefinitionValue;
    type StorageLayoutSpecifier;
    type StructDefinition;
    type StructMember;
    type TryStatement;
    type TupleExpression;
    type TupleValue;
    type TypeExpression;
    type UncheckedBlock;
    type UserDefinedValueTypeDefinition;
    type UsingAlias;
    type UsingDeconstruction;
    type UsingDeconstructionSymbol;
    type UsingDirective;
    type VariableDeclaration;
    type VariableDeclarationStatement;
    type VariableDeclarationValue;
    type VersionPragma;
    type VersionRange;
    type VersionTerm;
    type WhileStatement;
    type YulBlock;
    type YulBreakStatement;
    type YulContinueStatement;
    type YulDefaultCase;
    type YulFlagsDeclaration;
    type YulForStatement;
    type YulFunctionCallExpression;
    type YulFunctionDefinition;
    type YulIfStatement;
    type YulLeaveStatement;
    type YulParametersDeclaration;
    type YulReturnsDeclaration;
    type YulSwitchStatement;
    type YulValueCase;
    type YulVariableAssignmentStatement;
    type YulVariableDeclarationStatement;
    type YulVariableDeclarationValue;

    // Choices
    type AbicoderVersion;
    type ArgumentsDeclaration;
    type ConstructorAttribute;
    type ContractMember;
    type ContractSpecifier;
    type ElementaryType;
    type ExperimentalFeature;
    type Expression;
    type Expression_AdditiveExpression_Operator;
    type Expression_AssignmentExpression_Operator;
    type Expression_EqualityExpression_Operator;
    type Expression_InequalityExpression_Operator;
    type Expression_MultiplicativeExpression_Operator;
    type Expression_PostfixExpression_Operator;
    type Expression_PrefixExpression_Operator;
    type Expression_ShiftExpression_Operator;
    type FallbackFunctionAttribute;
    type ForStatementCondition;
    type ForStatementInitialization;
    type FunctionAttribute;
    type FunctionBody;
    type FunctionName;
    type FunctionTypeAttribute;
    type IdentifierPathElement;
    type ImportClause;
    type MappingKeyType;
    type ModifierAttribute;
    type NumberUnit;
    type Pragma;
    type ReceiveFunctionAttribute;
    type SourceUnitMember;
    type StateVariableAttribute;
    type Statement;
    type StorageLocation;
    type StringExpression;
    type TypeName;
    type UsingClause;
    type UsingOperator;
    type UsingTarget;
    type VariableDeclarationTarget;
    type VersionExpression;
    type VersionLiteral;
    type VersionOperator;
    type YulExpression;
    type YulLiteral;
    type YulStatement;
    type YulSwitchCase;

    // Collections
    type ArrayValues;
    type CallOptions;
    type CatchClauses;
    type ConstructorAttributes;
    type ContractMembers;
    type ContractSpecifiers;
    type EnumMembers;
    type ErrorParameters;
    type EventParameters;
    type FallbackFunctionAttributes;
    type FunctionAttributes;
    type FunctionTypeAttributes;
    type HexStringLiterals;
    type IdentifierPath;
    type ImportDeconstructionSymbols;
    type InheritanceTypes;
    type InterfaceMembers;
    type LibraryMembers;
    type ModifierAttributes;
    type MultiTypedDeclarationElements;
    type NamedArguments;
    type OverridePaths;
    type Parameters;
    type PositionalArguments;
    type ReceiveFunctionAttributes;
    type SimpleVersionLiteral;
    type SourceUnitMembers;
    type StateVariableAttributes;
    type Statements;
    type StringLiterals;
    type StructMembers;
    type TupleValues;
    type UnicodeStringLiterals;
    type UsingDeconstructionSymbols;
    type VersionExpressionSet;
    type VersionExpressionSets;
    type YulArguments;
    type YulFlags;
    type YulParameters;
    type YulPath;
    type YulPaths;
    type YulStatements;
    type YulSwitchCases;
    type YulVariableNames;

    // =======================
    // === Factory Methods ===
    // =======================

    // Terminal factory methods
    fn make_abi_encoder_v2_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::ABIEncoderV2Keyword;
    fn make_abicoder_keyword(&self, range: Range<usize>, source: &str) -> Self::AbicoderKeyword;
    fn make_abicoder_v1_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::AbicoderV1Keyword;
    fn make_abicoder_v2_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::AbicoderV2Keyword;
    fn make_abstract_keyword(&self, range: Range<usize>, source: &str) -> Self::AbstractKeyword;
    fn make_address_keyword(&self, range: Range<usize>, source: &str) -> Self::AddressKeyword;
    fn make_after_keyword(&self, range: Range<usize>, source: &str) -> Self::AfterKeyword;
    fn make_alias_keyword(&self, range: Range<usize>, source: &str) -> Self::AliasKeyword;
    fn make_ampersand(&self, range: Range<usize>, source: &str) -> Self::Ampersand;
    fn make_ampersand_ampersand(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::AmpersandAmpersand;
    fn make_ampersand_equal(&self, range: Range<usize>, source: &str) -> Self::AmpersandEqual;
    fn make_anonymous_keyword(&self, range: Range<usize>, source: &str) -> Self::AnonymousKeyword;
    fn make_apply_keyword(&self, range: Range<usize>, source: &str) -> Self::ApplyKeyword;
    fn make_as_keyword(&self, range: Range<usize>, source: &str) -> Self::AsKeyword;
    fn make_assembly_keyword(&self, range: Range<usize>, source: &str) -> Self::AssemblyKeyword;
    fn make_asterisk(&self, range: Range<usize>, source: &str) -> Self::Asterisk;
    fn make_asterisk_asterisk(&self, range: Range<usize>, source: &str) -> Self::AsteriskAsterisk;
    fn make_asterisk_equal(&self, range: Range<usize>, source: &str) -> Self::AsteriskEqual;
    fn make_at_keyword(&self, range: Range<usize>, source: &str) -> Self::AtKeyword;
    fn make_auto_keyword(&self, range: Range<usize>, source: &str) -> Self::AutoKeyword;
    fn make_bang(&self, range: Range<usize>, source: &str) -> Self::Bang;
    fn make_bang_equal(&self, range: Range<usize>, source: &str) -> Self::BangEqual;
    fn make_bar(&self, range: Range<usize>, source: &str) -> Self::Bar;
    fn make_bar_bar(&self, range: Range<usize>, source: &str) -> Self::BarBar;
    fn make_bar_equal(&self, range: Range<usize>, source: &str) -> Self::BarEqual;
    fn make_bool_keyword(&self, range: Range<usize>, source: &str) -> Self::BoolKeyword;
    fn make_break_keyword(&self, range: Range<usize>, source: &str) -> Self::BreakKeyword;
    fn make_byte_keyword(&self, range: Range<usize>, source: &str) -> Self::ByteKeyword;
    fn make_bytes_keyword(&self, range: Range<usize>, source: &str) -> Self::BytesKeyword;
    fn make_call_data_keyword(&self, range: Range<usize>, source: &str) -> Self::CallDataKeyword;
    fn make_caret(&self, range: Range<usize>, source: &str) -> Self::Caret;
    fn make_caret_equal(&self, range: Range<usize>, source: &str) -> Self::CaretEqual;
    fn make_case_keyword(&self, range: Range<usize>, source: &str) -> Self::CaseKeyword;
    fn make_catch_keyword(&self, range: Range<usize>, source: &str) -> Self::CatchKeyword;
    fn make_close_brace(&self, range: Range<usize>, source: &str) -> Self::CloseBrace;
    fn make_close_bracket(&self, range: Range<usize>, source: &str) -> Self::CloseBracket;
    fn make_close_paren(&self, range: Range<usize>, source: &str) -> Self::CloseParen;
    fn make_colon(&self, range: Range<usize>, source: &str) -> Self::Colon;
    fn make_comma(&self, range: Range<usize>, source: &str) -> Self::Comma;
    fn make_constant_keyword(&self, range: Range<usize>, source: &str) -> Self::ConstantKeyword;
    fn make_constructor_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::ConstructorKeyword;
    fn make_continue_keyword(&self, range: Range<usize>, source: &str) -> Self::ContinueKeyword;
    fn make_contract_keyword(&self, range: Range<usize>, source: &str) -> Self::ContractKeyword;
    fn make_copy_of_keyword(&self, range: Range<usize>, source: &str) -> Self::CopyOfKeyword;
    fn make_days_keyword(&self, range: Range<usize>, source: &str) -> Self::DaysKeyword;
    fn make_decimal_literal(&self, range: Range<usize>, source: &str) -> Self::DecimalLiteral;
    fn make_default_keyword(&self, range: Range<usize>, source: &str) -> Self::DefaultKeyword;
    fn make_define_keyword(&self, range: Range<usize>, source: &str) -> Self::DefineKeyword;
    fn make_delete_keyword(&self, range: Range<usize>, source: &str) -> Self::DeleteKeyword;
    fn make_do_keyword(&self, range: Range<usize>, source: &str) -> Self::DoKeyword;
    fn make_else_keyword(&self, range: Range<usize>, source: &str) -> Self::ElseKeyword;
    fn make_emit_keyword(&self, range: Range<usize>, source: &str) -> Self::EmitKeyword;
    fn make_enum_keyword(&self, range: Range<usize>, source: &str) -> Self::EnumKeyword;
    fn make_equal(&self, range: Range<usize>, source: &str) -> Self::Equal;
    fn make_equal_equal(&self, range: Range<usize>, source: &str) -> Self::EqualEqual;
    fn make_equal_greater_than(&self, range: Range<usize>, source: &str) -> Self::EqualGreaterThan;
    fn make_error_keyword(&self, range: Range<usize>, source: &str) -> Self::ErrorKeyword;
    fn make_ether_keyword(&self, range: Range<usize>, source: &str) -> Self::EtherKeyword;
    fn make_event_keyword(&self, range: Range<usize>, source: &str) -> Self::EventKeyword;
    fn make_experimental_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::ExperimentalKeyword;
    fn make_external_keyword(&self, range: Range<usize>, source: &str) -> Self::ExternalKeyword;
    fn make_fallback_keyword(&self, range: Range<usize>, source: &str) -> Self::FallbackKeyword;
    fn make_false_keyword(&self, range: Range<usize>, source: &str) -> Self::FalseKeyword;
    fn make_final_keyword(&self, range: Range<usize>, source: &str) -> Self::FinalKeyword;
    fn make_fixed_keyword(&self, range: Range<usize>, source: &str) -> Self::FixedKeyword;
    fn make_for_keyword(&self, range: Range<usize>, source: &str) -> Self::ForKeyword;
    fn make_from_keyword(&self, range: Range<usize>, source: &str) -> Self::FromKeyword;
    fn make_function_keyword(&self, range: Range<usize>, source: &str) -> Self::FunctionKeyword;
    fn make_global_keyword(&self, range: Range<usize>, source: &str) -> Self::GlobalKeyword;
    fn make_greater_than(&self, range: Range<usize>, source: &str) -> Self::GreaterThan;
    fn make_greater_than_equal(&self, range: Range<usize>, source: &str) -> Self::GreaterThanEqual;
    fn make_greater_than_greater_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::GreaterThanGreaterThan;
    fn make_greater_than_greater_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::GreaterThanGreaterThanEqual;
    fn make_greater_than_greater_than_greater_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::GreaterThanGreaterThanGreaterThan;
    fn make_greater_than_greater_than_greater_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::GreaterThanGreaterThanGreaterThanEqual;
    fn make_gwei_keyword(&self, range: Range<usize>, source: &str) -> Self::GweiKeyword;
    fn make_hex_keyword(&self, range: Range<usize>, source: &str) -> Self::HexKeyword;
    fn make_hex_literal(&self, range: Range<usize>, source: &str) -> Self::HexLiteral;
    fn make_hex_string_literal(&self, range: Range<usize>, source: &str) -> Self::HexStringLiteral;
    fn make_hours_keyword(&self, range: Range<usize>, source: &str) -> Self::HoursKeyword;
    fn make_identifier(&self, range: Range<usize>, source: &str) -> Self::Identifier;
    fn make_if_keyword(&self, range: Range<usize>, source: &str) -> Self::IfKeyword;
    fn make_immutable_keyword(&self, range: Range<usize>, source: &str) -> Self::ImmutableKeyword;
    fn make_implements_keyword(&self, range: Range<usize>, source: &str)
        -> Self::ImplementsKeyword;
    fn make_import_keyword(&self, range: Range<usize>, source: &str) -> Self::ImportKeyword;
    fn make_in_keyword(&self, range: Range<usize>, source: &str) -> Self::InKeyword;
    fn make_indexed_keyword(&self, range: Range<usize>, source: &str) -> Self::IndexedKeyword;
    fn make_inline_keyword(&self, range: Range<usize>, source: &str) -> Self::InlineKeyword;
    fn make_int_keyword(&self, range: Range<usize>, source: &str) -> Self::IntKeyword;
    fn make_interface_keyword(&self, range: Range<usize>, source: &str) -> Self::InterfaceKeyword;
    fn make_internal_keyword(&self, range: Range<usize>, source: &str) -> Self::InternalKeyword;
    fn make_is_keyword(&self, range: Range<usize>, source: &str) -> Self::IsKeyword;
    fn make_layout_keyword(&self, range: Range<usize>, source: &str) -> Self::LayoutKeyword;
    fn make_less_than(&self, range: Range<usize>, source: &str) -> Self::LessThan;
    fn make_less_than_equal(&self, range: Range<usize>, source: &str) -> Self::LessThanEqual;
    fn make_less_than_less_than(&self, range: Range<usize>, source: &str)
        -> Self::LessThanLessThan;
    fn make_less_than_less_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::LessThanLessThanEqual;
    fn make_let_keyword(&self, range: Range<usize>, source: &str) -> Self::LetKeyword;
    fn make_library_keyword(&self, range: Range<usize>, source: &str) -> Self::LibraryKeyword;
    fn make_macro_keyword(&self, range: Range<usize>, source: &str) -> Self::MacroKeyword;
    fn make_mapping_keyword(&self, range: Range<usize>, source: &str) -> Self::MappingKeyword;
    fn make_match_keyword(&self, range: Range<usize>, source: &str) -> Self::MatchKeyword;
    fn make_memory_keyword(&self, range: Range<usize>, source: &str) -> Self::MemoryKeyword;
    fn make_minus(&self, range: Range<usize>, source: &str) -> Self::Minus;
    fn make_minus_equal(&self, range: Range<usize>, source: &str) -> Self::MinusEqual;
    fn make_minus_minus(&self, range: Range<usize>, source: &str) -> Self::MinusMinus;
    fn make_minutes_keyword(&self, range: Range<usize>, source: &str) -> Self::MinutesKeyword;
    fn make_modifier_keyword(&self, range: Range<usize>, source: &str) -> Self::ModifierKeyword;
    fn make_mutable_keyword(&self, range: Range<usize>, source: &str) -> Self::MutableKeyword;
    fn make_new_keyword(&self, range: Range<usize>, source: &str) -> Self::NewKeyword;
    fn make_null_keyword(&self, range: Range<usize>, source: &str) -> Self::NullKeyword;
    fn make_of_keyword(&self, range: Range<usize>, source: &str) -> Self::OfKeyword;
    fn make_open_brace(&self, range: Range<usize>, source: &str) -> Self::OpenBrace;
    fn make_open_bracket(&self, range: Range<usize>, source: &str) -> Self::OpenBracket;
    fn make_open_paren(&self, range: Range<usize>, source: &str) -> Self::OpenParen;
    fn make_override_keyword(&self, range: Range<usize>, source: &str) -> Self::OverrideKeyword;
    fn make_partial_keyword(&self, range: Range<usize>, source: &str) -> Self::PartialKeyword;
    fn make_payable_keyword(&self, range: Range<usize>, source: &str) -> Self::PayableKeyword;
    fn make_percent(&self, range: Range<usize>, source: &str) -> Self::Percent;
    fn make_percent_equal(&self, range: Range<usize>, source: &str) -> Self::PercentEqual;
    fn make_period(&self, range: Range<usize>, source: &str) -> Self::Period;
    fn make_plus(&self, range: Range<usize>, source: &str) -> Self::Plus;
    fn make_plus_equal(&self, range: Range<usize>, source: &str) -> Self::PlusEqual;
    fn make_plus_plus(&self, range: Range<usize>, source: &str) -> Self::PlusPlus;
    fn make_pragma_bar_bar(&self, range: Range<usize>, source: &str) -> Self::PragmaBarBar;
    fn make_pragma_caret(&self, range: Range<usize>, source: &str) -> Self::PragmaCaret;
    fn make_pragma_equal(&self, range: Range<usize>, source: &str) -> Self::PragmaEqual;
    fn make_pragma_greater_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::PragmaGreaterThan;
    fn make_pragma_greater_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::PragmaGreaterThanEqual;
    fn make_pragma_keyword(&self, range: Range<usize>, source: &str) -> Self::PragmaKeyword;
    fn make_pragma_less_than(&self, range: Range<usize>, source: &str) -> Self::PragmaLessThan;
    fn make_pragma_less_than_equal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::PragmaLessThanEqual;
    fn make_pragma_minus(&self, range: Range<usize>, source: &str) -> Self::PragmaMinus;
    fn make_pragma_period(&self, range: Range<usize>, source: &str) -> Self::PragmaPeriod;
    fn make_pragma_semicolon(&self, range: Range<usize>, source: &str) -> Self::PragmaSemicolon;
    fn make_pragma_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::PragmaStringLiteral;
    fn make_pragma_tilde(&self, range: Range<usize>, source: &str) -> Self::PragmaTilde;
    fn make_private_keyword(&self, range: Range<usize>, source: &str) -> Self::PrivateKeyword;
    fn make_promise_keyword(&self, range: Range<usize>, source: &str) -> Self::PromiseKeyword;
    fn make_public_keyword(&self, range: Range<usize>, source: &str) -> Self::PublicKeyword;
    fn make_pure_keyword(&self, range: Range<usize>, source: &str) -> Self::PureKeyword;
    fn make_question_mark(&self, range: Range<usize>, source: &str) -> Self::QuestionMark;
    fn make_receive_keyword(&self, range: Range<usize>, source: &str) -> Self::ReceiveKeyword;
    fn make_reference_keyword(&self, range: Range<usize>, source: &str) -> Self::ReferenceKeyword;
    fn make_relocatable_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::RelocatableKeyword;
    fn make_return_keyword(&self, range: Range<usize>, source: &str) -> Self::ReturnKeyword;
    fn make_returns_keyword(&self, range: Range<usize>, source: &str) -> Self::ReturnsKeyword;
    fn make_revert_keyword(&self, range: Range<usize>, source: &str) -> Self::RevertKeyword;
    fn make_smt_checker_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::SMTCheckerKeyword;
    fn make_sealed_keyword(&self, range: Range<usize>, source: &str) -> Self::SealedKeyword;
    fn make_seconds_keyword(&self, range: Range<usize>, source: &str) -> Self::SecondsKeyword;
    fn make_semicolon(&self, range: Range<usize>, source: &str) -> Self::Semicolon;
    fn make_size_of_keyword(&self, range: Range<usize>, source: &str) -> Self::SizeOfKeyword;
    fn make_slash(&self, range: Range<usize>, source: &str) -> Self::Slash;
    fn make_slash_equal(&self, range: Range<usize>, source: &str) -> Self::SlashEqual;
    fn make_solidity_keyword(&self, range: Range<usize>, source: &str) -> Self::SolidityKeyword;
    fn make_static_keyword(&self, range: Range<usize>, source: &str) -> Self::StaticKeyword;
    fn make_storage_keyword(&self, range: Range<usize>, source: &str) -> Self::StorageKeyword;
    fn make_string_keyword(&self, range: Range<usize>, source: &str) -> Self::StringKeyword;
    fn make_string_literal(&self, range: Range<usize>, source: &str) -> Self::StringLiteral;
    fn make_struct_keyword(&self, range: Range<usize>, source: &str) -> Self::StructKeyword;
    fn make_super_keyword(&self, range: Range<usize>, source: &str) -> Self::SuperKeyword;
    fn make_supports_keyword(&self, range: Range<usize>, source: &str) -> Self::SupportsKeyword;
    fn make_switch_keyword(&self, range: Range<usize>, source: &str) -> Self::SwitchKeyword;
    fn make_this_keyword(&self, range: Range<usize>, source: &str) -> Self::ThisKeyword;
    fn make_throw_keyword(&self, range: Range<usize>, source: &str) -> Self::ThrowKeyword;
    fn make_tilde(&self, range: Range<usize>, source: &str) -> Self::Tilde;
    fn make_transient_keyword(&self, range: Range<usize>, source: &str) -> Self::TransientKeyword;
    fn make_true_keyword(&self, range: Range<usize>, source: &str) -> Self::TrueKeyword;
    fn make_try_keyword(&self, range: Range<usize>, source: &str) -> Self::TryKeyword;
    fn make_type_def_keyword(&self, range: Range<usize>, source: &str) -> Self::TypeDefKeyword;
    fn make_type_keyword(&self, range: Range<usize>, source: &str) -> Self::TypeKeyword;
    fn make_type_of_keyword(&self, range: Range<usize>, source: &str) -> Self::TypeOfKeyword;
    fn make_ufixed_keyword(&self, range: Range<usize>, source: &str) -> Self::UfixedKeyword;
    fn make_uint_keyword(&self, range: Range<usize>, source: &str) -> Self::UintKeyword;
    fn make_unchecked_keyword(&self, range: Range<usize>, source: &str) -> Self::UncheckedKeyword;
    fn make_unicode_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::UnicodeStringLiteral;
    fn make_using_keyword(&self, range: Range<usize>, source: &str) -> Self::UsingKeyword;
    fn make_var_keyword(&self, range: Range<usize>, source: &str) -> Self::VarKeyword;
    fn make_version_specifier(&self, range: Range<usize>, source: &str) -> Self::VersionSpecifier;
    fn make_view_keyword(&self, range: Range<usize>, source: &str) -> Self::ViewKeyword;
    fn make_virtual_keyword(&self, range: Range<usize>, source: &str) -> Self::VirtualKeyword;
    fn make_weeks_keyword(&self, range: Range<usize>, source: &str) -> Self::WeeksKeyword;
    fn make_wei_keyword(&self, range: Range<usize>, source: &str) -> Self::WeiKeyword;
    fn make_while_keyword(&self, range: Range<usize>, source: &str) -> Self::WhileKeyword;
    fn make_years_keyword(&self, range: Range<usize>, source: &str) -> Self::YearsKeyword;
    fn make_yul_break_keyword(&self, range: Range<usize>, source: &str) -> Self::YulBreakKeyword;
    fn make_yul_case_keyword(&self, range: Range<usize>, source: &str) -> Self::YulCaseKeyword;
    fn make_yul_close_brace(&self, range: Range<usize>, source: &str) -> Self::YulCloseBrace;
    fn make_yul_close_paren(&self, range: Range<usize>, source: &str) -> Self::YulCloseParen;
    fn make_yul_colon_equal(&self, range: Range<usize>, source: &str) -> Self::YulColonEqual;
    fn make_yul_comma(&self, range: Range<usize>, source: &str) -> Self::YulComma;
    fn make_yul_continue_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::YulContinueKeyword;
    fn make_yul_decimal_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::YulDecimalLiteral;
    fn make_yul_default_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::YulDefaultKeyword;
    fn make_yul_false_keyword(&self, range: Range<usize>, source: &str) -> Self::YulFalseKeyword;
    fn make_yul_for_keyword(&self, range: Range<usize>, source: &str) -> Self::YulForKeyword;
    fn make_yul_function_keyword(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::YulFunctionKeyword;
    fn make_yul_hex_keyword(&self, range: Range<usize>, source: &str) -> Self::YulHexKeyword;
    fn make_yul_hex_literal(&self, range: Range<usize>, source: &str) -> Self::YulHexLiteral;
    fn make_yul_hex_string_literal(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::YulHexStringLiteral;
    fn make_yul_identifier(&self, range: Range<usize>, source: &str) -> Self::YulIdentifier;
    fn make_yul_if_keyword(&self, range: Range<usize>, source: &str) -> Self::YulIfKeyword;
    fn make_yul_leave_keyword(&self, range: Range<usize>, source: &str) -> Self::YulLeaveKeyword;
    fn make_yul_let_keyword(&self, range: Range<usize>, source: &str) -> Self::YulLetKeyword;
    fn make_yul_minus_greater_than(
        &self,
        range: Range<usize>,
        source: &str,
    ) -> Self::YulMinusGreaterThan;
    fn make_yul_open_brace(&self, range: Range<usize>, source: &str) -> Self::YulOpenBrace;
    fn make_yul_open_paren(&self, range: Range<usize>, source: &str) -> Self::YulOpenParen;
    fn make_yul_period(&self, range: Range<usize>, source: &str) -> Self::YulPeriod;
    fn make_yul_string_literal(&self, range: Range<usize>, source: &str) -> Self::YulStringLiteral;
    fn make_yul_super_keyword(&self, range: Range<usize>, source: &str) -> Self::YulSuperKeyword;
    fn make_yul_switch_keyword(&self, range: Range<usize>, source: &str) -> Self::YulSwitchKeyword;
    fn make_yul_this_keyword(&self, range: Range<usize>, source: &str) -> Self::YulThisKeyword;
    fn make_yul_true_keyword(&self, range: Range<usize>, source: &str) -> Self::YulTrueKeyword;

    // Sequence factory methods
    fn make_abicoder_pragma(
        &self,
        abicoder_keyword: Self::AbicoderKeyword,
        version: Self::AbicoderVersion,
    ) -> Self::AbicoderPragma;
    fn make_additive_expression(
        &self,
        left_operand: Self::Expression,
        expression_additive_expression_operator: Self::Expression_AdditiveExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::AdditiveExpression;
    fn make_address_type(
        &self,
        address_keyword: Self::AddressKeyword,
        payable_keyword: Option<Self::PayableKeyword>,
    ) -> Self::AddressType;
    fn make_and_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::AmpersandAmpersand,
        right_operand: Self::Expression,
    ) -> Self::AndExpression;
    fn make_array_expression(
        &self,
        open_bracket: Self::OpenBracket,
        items: Self::ArrayValues,
        close_bracket: Self::CloseBracket,
    ) -> Self::ArrayExpression;
    fn make_array_type_name(
        &self,
        operand: Self::TypeName,
        open_bracket: Self::OpenBracket,
        index: Option<Self::Expression>,
        close_bracket: Self::CloseBracket,
    ) -> Self::ArrayTypeName;
    fn make_assembly_statement(
        &self,
        assembly_keyword: Self::AssemblyKeyword,
        label: Option<Self::YulStringLiteral>,
        flags: Option<Self::YulFlagsDeclaration>,
        body: Self::YulBlock,
    ) -> Self::AssemblyStatement;
    fn make_assignment_expression(
        &self,
        left_operand: Self::Expression,
        expression_assignment_expression_operator: Self::Expression_AssignmentExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::AssignmentExpression;
    fn make_bitwise_and_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::Ampersand,
        right_operand: Self::Expression,
    ) -> Self::BitwiseAndExpression;
    fn make_bitwise_or_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::Bar,
        right_operand: Self::Expression,
    ) -> Self::BitwiseOrExpression;
    fn make_bitwise_xor_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::Caret,
        right_operand: Self::Expression,
    ) -> Self::BitwiseXorExpression;
    fn make_block(
        &self,
        open_brace: Self::OpenBrace,
        statements: Self::Statements,
        close_brace: Self::CloseBrace,
    ) -> Self::Block;
    fn make_break_statement(
        &self,
        break_keyword: Self::BreakKeyword,
        semicolon: Self::Semicolon,
    ) -> Self::BreakStatement;
    fn make_call_options_expression(
        &self,
        operand: Self::Expression,
        open_brace: Self::OpenBrace,
        options: Self::CallOptions,
        close_brace: Self::CloseBrace,
    ) -> Self::CallOptionsExpression;
    fn make_catch_clause(
        &self,
        catch_keyword: Self::CatchKeyword,
        error: Option<Self::CatchClauseError>,
        body: Self::Block,
    ) -> Self::CatchClause;
    fn make_catch_clause_error(
        &self,
        name: Option<Self::Identifier>,
        parameters: Self::ParametersDeclaration,
    ) -> Self::CatchClauseError;
    fn make_conditional_expression(
        &self,
        operand: Self::Expression,
        question_mark: Self::QuestionMark,
        true_expression: Self::Expression,
        colon: Self::Colon,
        false_expression: Self::Expression,
    ) -> Self::ConditionalExpression;
    fn make_constant_definition(
        &self,
        type_name: Self::TypeName,
        constant_keyword: Self::ConstantKeyword,
        name: Self::Identifier,
        equal: Self::Equal,
        value: Self::Expression,
        semicolon: Self::Semicolon,
    ) -> Self::ConstantDefinition;
    fn make_constructor_definition(
        &self,
        constructor_keyword: Self::ConstructorKeyword,
        parameters: Self::ParametersDeclaration,
        attributes: Self::ConstructorAttributes,
        body: Self::Block,
    ) -> Self::ConstructorDefinition;
    fn make_continue_statement(
        &self,
        continue_keyword: Self::ContinueKeyword,
        semicolon: Self::Semicolon,
    ) -> Self::ContinueStatement;
    fn make_contract_definition(
        &self,
        abstract_keyword: Option<Self::AbstractKeyword>,
        contract_keyword: Self::ContractKeyword,
        name: Self::Identifier,
        specifiers: Self::ContractSpecifiers,
        open_brace: Self::OpenBrace,
        members: Self::ContractMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::ContractDefinition;
    fn make_decimal_number_expression(
        &self,
        literal: Self::DecimalLiteral,
        unit: Option<Self::NumberUnit>,
    ) -> Self::DecimalNumberExpression;
    fn make_do_while_statement(
        &self,
        do_keyword: Self::DoKeyword,
        body: Self::Statement,
        while_keyword: Self::WhileKeyword,
        open_paren: Self::OpenParen,
        condition: Self::Expression,
        close_paren: Self::CloseParen,
        semicolon: Self::Semicolon,
    ) -> Self::DoWhileStatement;
    fn make_else_branch(
        &self,
        else_keyword: Self::ElseKeyword,
        body: Self::Statement,
    ) -> Self::ElseBranch;
    fn make_emit_statement(
        &self,
        emit_keyword: Self::EmitKeyword,
        event: Self::IdentifierPath,
        arguments: Self::ArgumentsDeclaration,
        semicolon: Self::Semicolon,
    ) -> Self::EmitStatement;
    fn make_enum_definition(
        &self,
        enum_keyword: Self::EnumKeyword,
        name: Self::Identifier,
        open_brace: Self::OpenBrace,
        members: Self::EnumMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::EnumDefinition;
    fn make_equality_expression(
        &self,
        left_operand: Self::Expression,
        expression_equality_expression_operator: Self::Expression_EqualityExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::EqualityExpression;
    fn make_error_definition(
        &self,
        error_keyword: Self::ErrorKeyword,
        name: Self::Identifier,
        members: Self::ErrorParametersDeclaration,
        semicolon: Self::Semicolon,
    ) -> Self::ErrorDefinition;
    fn make_error_parameter(
        &self,
        type_name: Self::TypeName,
        name: Option<Self::Identifier>,
    ) -> Self::ErrorParameter;
    fn make_error_parameters_declaration(
        &self,
        open_paren: Self::OpenParen,
        parameters: Self::ErrorParameters,
        close_paren: Self::CloseParen,
    ) -> Self::ErrorParametersDeclaration;
    fn make_event_definition(
        &self,
        event_keyword: Self::EventKeyword,
        name: Self::Identifier,
        parameters: Self::EventParametersDeclaration,
        anonymous_keyword: Option<Self::AnonymousKeyword>,
        semicolon: Self::Semicolon,
    ) -> Self::EventDefinition;
    fn make_event_parameter(
        &self,
        type_name: Self::TypeName,
        indexed_keyword: Option<Self::IndexedKeyword>,
        name: Option<Self::Identifier>,
    ) -> Self::EventParameter;
    fn make_event_parameters_declaration(
        &self,
        open_paren: Self::OpenParen,
        parameters: Self::EventParameters,
        close_paren: Self::CloseParen,
    ) -> Self::EventParametersDeclaration;
    fn make_experimental_pragma(
        &self,
        experimental_keyword: Self::ExperimentalKeyword,
        feature: Self::ExperimentalFeature,
    ) -> Self::ExperimentalPragma;
    fn make_exponentiation_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::AsteriskAsterisk,
        right_operand: Self::Expression,
    ) -> Self::ExponentiationExpression;
    fn make_expression_statement(
        &self,
        expression: Self::Expression,
        semicolon: Self::Semicolon,
    ) -> Self::ExpressionStatement;
    fn make_fallback_function_definition(
        &self,
        fallback_keyword: Self::FallbackKeyword,
        parameters: Self::ParametersDeclaration,
        attributes: Self::FallbackFunctionAttributes,
        returns: Option<Self::ReturnsDeclaration>,
        body: Self::FunctionBody,
    ) -> Self::FallbackFunctionDefinition;
    fn make_for_statement(
        &self,
        for_keyword: Self::ForKeyword,
        open_paren: Self::OpenParen,
        initialization: Self::ForStatementInitialization,
        condition: Self::ForStatementCondition,
        iterator: Option<Self::Expression>,
        close_paren: Self::CloseParen,
        body: Self::Statement,
    ) -> Self::ForStatement;
    fn make_function_call_expression(
        &self,
        operand: Self::Expression,
        arguments: Self::ArgumentsDeclaration,
    ) -> Self::FunctionCallExpression;
    fn make_function_definition(
        &self,
        function_keyword: Self::FunctionKeyword,
        name: Self::FunctionName,
        parameters: Self::ParametersDeclaration,
        attributes: Self::FunctionAttributes,
        returns: Option<Self::ReturnsDeclaration>,
        body: Self::FunctionBody,
    ) -> Self::FunctionDefinition;
    fn make_function_type(
        &self,
        function_keyword: Self::FunctionKeyword,
        parameters: Self::ParametersDeclaration,
        attributes: Self::FunctionTypeAttributes,
        returns: Option<Self::ReturnsDeclaration>,
    ) -> Self::FunctionType;
    fn make_hex_number_expression(&self, literal: Self::HexLiteral) -> Self::HexNumberExpression;
    fn make_if_statement(
        &self,
        if_keyword: Self::IfKeyword,
        open_paren: Self::OpenParen,
        condition: Self::Expression,
        close_paren: Self::CloseParen,
        body: Self::Statement,
        else_branch: Option<Self::ElseBranch>,
    ) -> Self::IfStatement;
    fn make_import_alias(
        &self,
        as_keyword: Self::AsKeyword,
        identifier: Self::Identifier,
    ) -> Self::ImportAlias;
    fn make_import_deconstruction(
        &self,
        open_brace: Self::OpenBrace,
        symbols: Self::ImportDeconstructionSymbols,
        close_brace: Self::CloseBrace,
        from_keyword: Self::FromKeyword,
        path: Self::StringLiteral,
    ) -> Self::ImportDeconstruction;
    fn make_import_deconstruction_symbol(
        &self,
        name: Self::Identifier,
        alias: Option<Self::ImportAlias>,
    ) -> Self::ImportDeconstructionSymbol;
    fn make_import_directive(
        &self,
        import_keyword: Self::ImportKeyword,
        clause: Self::ImportClause,
        semicolon: Self::Semicolon,
    ) -> Self::ImportDirective;
    fn make_index_access_end(
        &self,
        colon: Self::Colon,
        end: Option<Self::Expression>,
    ) -> Self::IndexAccessEnd;
    fn make_index_access_expression(
        &self,
        operand: Self::Expression,
        open_bracket: Self::OpenBracket,
        start: Option<Self::Expression>,
        end: Option<Self::IndexAccessEnd>,
        close_bracket: Self::CloseBracket,
    ) -> Self::IndexAccessExpression;
    fn make_inequality_expression(
        &self,
        left_operand: Self::Expression,
        expression_inequality_expression_operator: Self::Expression_InequalityExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::InequalityExpression;
    fn make_inheritance_specifier(
        &self,
        is_keyword: Self::IsKeyword,
        types: Self::InheritanceTypes,
    ) -> Self::InheritanceSpecifier;
    fn make_inheritance_type(
        &self,
        type_name: Self::IdentifierPath,
        arguments: Option<Self::ArgumentsDeclaration>,
    ) -> Self::InheritanceType;
    fn make_interface_definition(
        &self,
        interface_keyword: Self::InterfaceKeyword,
        name: Self::Identifier,
        inheritance: Option<Self::InheritanceSpecifier>,
        open_brace: Self::OpenBrace,
        members: Self::InterfaceMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::InterfaceDefinition;
    fn make_library_definition(
        &self,
        library_keyword: Self::LibraryKeyword,
        name: Self::Identifier,
        open_brace: Self::OpenBrace,
        members: Self::LibraryMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::LibraryDefinition;
    fn make_mapping_key(
        &self,
        key_type: Self::MappingKeyType,
        name: Option<Self::Identifier>,
    ) -> Self::MappingKey;
    fn make_mapping_type(
        &self,
        mapping_keyword: Self::MappingKeyword,
        open_paren: Self::OpenParen,
        key_type: Self::MappingKey,
        equal_greater_than: Self::EqualGreaterThan,
        value_type: Self::MappingValue,
        close_paren: Self::CloseParen,
    ) -> Self::MappingType;
    fn make_mapping_value(
        &self,
        type_name: Self::TypeName,
        name: Option<Self::Identifier>,
    ) -> Self::MappingValue;
    fn make_member_access_expression(
        &self,
        operand: Self::Expression,
        period: Self::Period,
        member: Self::IdentifierPathElement,
    ) -> Self::MemberAccessExpression;
    fn make_modifier_definition(
        &self,
        modifier_keyword: Self::ModifierKeyword,
        name: Self::Identifier,
        parameters: Option<Self::ParametersDeclaration>,
        attributes: Self::ModifierAttributes,
        body: Self::FunctionBody,
    ) -> Self::ModifierDefinition;
    fn make_modifier_invocation(
        &self,
        name: Self::IdentifierPath,
        arguments: Option<Self::ArgumentsDeclaration>,
    ) -> Self::ModifierInvocation;
    fn make_multi_typed_declaration(
        &self,
        open_paren: Self::OpenParen,
        elements: Self::MultiTypedDeclarationElements,
        close_paren: Self::CloseParen,
        value: Self::VariableDeclarationValue,
    ) -> Self::MultiTypedDeclaration;
    fn make_multi_typed_declaration_element(
        &self,
        member: Option<Self::VariableDeclaration>,
    ) -> Self::MultiTypedDeclarationElement;
    fn make_multiplicative_expression(
        &self,
        left_operand: Self::Expression,
        expression_multiplicative_expression_operator: Self::Expression_MultiplicativeExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::MultiplicativeExpression;
    fn make_named_argument(
        &self,
        name: Self::Identifier,
        colon: Self::Colon,
        value: Self::Expression,
    ) -> Self::NamedArgument;
    fn make_named_argument_group(
        &self,
        open_brace: Self::OpenBrace,
        arguments: Self::NamedArguments,
        close_brace: Self::CloseBrace,
    ) -> Self::NamedArgumentGroup;
    fn make_named_arguments_declaration(
        &self,
        open_paren: Self::OpenParen,
        arguments: Self::NamedArgumentGroup,
        close_paren: Self::CloseParen,
    ) -> Self::NamedArgumentsDeclaration;
    fn make_named_import(
        &self,
        asterisk: Self::Asterisk,
        alias: Self::ImportAlias,
        from_keyword: Self::FromKeyword,
        path: Self::StringLiteral,
    ) -> Self::NamedImport;
    fn make_new_expression(
        &self,
        new_keyword: Self::NewKeyword,
        type_name: Self::TypeName,
    ) -> Self::NewExpression;
    fn make_or_expression(
        &self,
        left_operand: Self::Expression,
        operator: Self::BarBar,
        right_operand: Self::Expression,
    ) -> Self::OrExpression;
    fn make_override_paths_declaration(
        &self,
        open_paren: Self::OpenParen,
        paths: Self::OverridePaths,
        close_paren: Self::CloseParen,
    ) -> Self::OverridePathsDeclaration;
    fn make_override_specifier(
        &self,
        override_keyword: Self::OverrideKeyword,
        overridden: Option<Self::OverridePathsDeclaration>,
    ) -> Self::OverrideSpecifier;
    fn make_parameter(
        &self,
        type_name: Self::TypeName,
        storage_location: Option<Self::StorageLocation>,
        name: Option<Self::Identifier>,
    ) -> Self::Parameter;
    fn make_parameters_declaration(
        &self,
        open_paren: Self::OpenParen,
        parameters: Self::Parameters,
        close_paren: Self::CloseParen,
    ) -> Self::ParametersDeclaration;
    fn make_path_import(
        &self,
        path: Self::StringLiteral,
        alias: Option<Self::ImportAlias>,
    ) -> Self::PathImport;
    fn make_positional_arguments_declaration(
        &self,
        open_paren: Self::OpenParen,
        arguments: Self::PositionalArguments,
        close_paren: Self::CloseParen,
    ) -> Self::PositionalArgumentsDeclaration;
    fn make_postfix_expression(
        &self,
        operand: Self::Expression,
        expression_postfix_expression_operator: Self::Expression_PostfixExpression_Operator,
    ) -> Self::PostfixExpression;
    fn make_pragma_directive(
        &self,
        pragma_keyword: Self::PragmaKeyword,
        pragma: Self::Pragma,
        semicolon: Self::PragmaSemicolon,
    ) -> Self::PragmaDirective;
    fn make_prefix_expression(
        &self,
        expression_prefix_expression_operator: Self::Expression_PrefixExpression_Operator,
        operand: Self::Expression,
    ) -> Self::PrefixExpression;
    fn make_receive_function_definition(
        &self,
        receive_keyword: Self::ReceiveKeyword,
        parameters: Self::ParametersDeclaration,
        attributes: Self::ReceiveFunctionAttributes,
        body: Self::FunctionBody,
    ) -> Self::ReceiveFunctionDefinition;
    fn make_return_statement(
        &self,
        return_keyword: Self::ReturnKeyword,
        expression: Option<Self::Expression>,
        semicolon: Self::Semicolon,
    ) -> Self::ReturnStatement;
    fn make_returns_declaration(
        &self,
        returns_keyword: Self::ReturnsKeyword,
        variables: Self::ParametersDeclaration,
    ) -> Self::ReturnsDeclaration;
    fn make_revert_statement(
        &self,
        revert_keyword: Self::RevertKeyword,
        error: Self::IdentifierPath,
        arguments: Self::ArgumentsDeclaration,
        semicolon: Self::Semicolon,
    ) -> Self::RevertStatement;
    fn make_shift_expression(
        &self,
        left_operand: Self::Expression,
        expression_shift_expression_operator: Self::Expression_ShiftExpression_Operator,
        right_operand: Self::Expression,
    ) -> Self::ShiftExpression;
    fn make_single_typed_declaration(
        &self,
        declaration: Self::VariableDeclaration,
        value: Option<Self::VariableDeclarationValue>,
    ) -> Self::SingleTypedDeclaration;
    fn make_source_unit(&self, members: Self::SourceUnitMembers) -> Self::SourceUnit;
    fn make_state_variable_definition(
        &self,
        type_name: Self::TypeName,
        attributes: Self::StateVariableAttributes,
        name: Self::Identifier,
        value: Option<Self::StateVariableDefinitionValue>,
        semicolon: Self::Semicolon,
    ) -> Self::StateVariableDefinition;
    fn make_state_variable_definition_value(
        &self,
        equal: Self::Equal,
        value: Self::Expression,
    ) -> Self::StateVariableDefinitionValue;
    fn make_storage_layout_specifier(
        &self,
        layout_keyword: Self::LayoutKeyword,
        at_keyword: Self::AtKeyword,
        expression: Self::Expression,
    ) -> Self::StorageLayoutSpecifier;
    fn make_struct_definition(
        &self,
        struct_keyword: Self::StructKeyword,
        name: Self::Identifier,
        open_brace: Self::OpenBrace,
        members: Self::StructMembers,
        close_brace: Self::CloseBrace,
    ) -> Self::StructDefinition;
    fn make_struct_member(
        &self,
        type_name: Self::TypeName,
        name: Self::Identifier,
        semicolon: Self::Semicolon,
    ) -> Self::StructMember;
    fn make_try_statement(
        &self,
        try_keyword: Self::TryKeyword,
        expression: Self::Expression,
        returns: Option<Self::ReturnsDeclaration>,
        body: Self::Block,
        catch_clauses: Self::CatchClauses,
    ) -> Self::TryStatement;
    fn make_tuple_expression(
        &self,
        open_paren: Self::OpenParen,
        items: Self::TupleValues,
        close_paren: Self::CloseParen,
    ) -> Self::TupleExpression;
    fn make_tuple_value(&self, expression: Option<Self::Expression>) -> Self::TupleValue;
    fn make_type_expression(
        &self,
        type_keyword: Self::TypeKeyword,
        open_paren: Self::OpenParen,
        type_name: Self::TypeName,
        close_paren: Self::CloseParen,
    ) -> Self::TypeExpression;
    fn make_unchecked_block(
        &self,
        unchecked_keyword: Self::UncheckedKeyword,
        block: Self::Block,
    ) -> Self::UncheckedBlock;
    fn make_user_defined_value_type_definition(
        &self,
        type_keyword: Self::TypeKeyword,
        name: Self::Identifier,
        is_keyword: Self::IsKeyword,
        value_type: Self::ElementaryType,
        semicolon: Self::Semicolon,
    ) -> Self::UserDefinedValueTypeDefinition;
    fn make_using_alias(
        &self,
        as_keyword: Self::AsKeyword,
        operator: Self::UsingOperator,
    ) -> Self::UsingAlias;
    fn make_using_deconstruction(
        &self,
        open_brace: Self::OpenBrace,
        symbols: Self::UsingDeconstructionSymbols,
        close_brace: Self::CloseBrace,
    ) -> Self::UsingDeconstruction;
    fn make_using_deconstruction_symbol(
        &self,
        name: Self::IdentifierPath,
        alias: Option<Self::UsingAlias>,
    ) -> Self::UsingDeconstructionSymbol;
    fn make_using_directive(
        &self,
        using_keyword: Self::UsingKeyword,
        clause: Self::UsingClause,
        for_keyword: Self::ForKeyword,
        target: Self::UsingTarget,
        global_keyword: Option<Self::GlobalKeyword>,
        semicolon: Self::Semicolon,
    ) -> Self::UsingDirective;
    fn make_variable_declaration(
        &self,
        type_name: Self::TypeName,
        storage_location: Option<Self::StorageLocation>,
        name: Self::Identifier,
    ) -> Self::VariableDeclaration;
    fn make_variable_declaration_statement(
        &self,
        target: Self::VariableDeclarationTarget,
        semicolon: Self::Semicolon,
    ) -> Self::VariableDeclarationStatement;
    fn make_variable_declaration_value(
        &self,
        equal: Self::Equal,
        expression: Self::Expression,
    ) -> Self::VariableDeclarationValue;
    fn make_version_pragma(
        &self,
        solidity_keyword: Self::SolidityKeyword,
        sets: Self::VersionExpressionSets,
    ) -> Self::VersionPragma;
    fn make_version_range(
        &self,
        start: Self::VersionLiteral,
        minus: Self::PragmaMinus,
        end: Self::VersionLiteral,
    ) -> Self::VersionRange;
    fn make_version_term(
        &self,
        operator: Option<Self::VersionOperator>,
        literal: Self::VersionLiteral,
    ) -> Self::VersionTerm;
    fn make_while_statement(
        &self,
        while_keyword: Self::WhileKeyword,
        open_paren: Self::OpenParen,
        condition: Self::Expression,
        close_paren: Self::CloseParen,
        body: Self::Statement,
    ) -> Self::WhileStatement;
    fn make_yul_block(
        &self,
        open_brace: Self::YulOpenBrace,
        statements: Self::YulStatements,
        close_brace: Self::YulCloseBrace,
    ) -> Self::YulBlock;
    fn make_yul_break_statement(
        &self,
        break_keyword: Self::YulBreakKeyword,
    ) -> Self::YulBreakStatement;
    fn make_yul_continue_statement(
        &self,
        continue_keyword: Self::YulContinueKeyword,
    ) -> Self::YulContinueStatement;
    fn make_yul_default_case(
        &self,
        default_keyword: Self::YulDefaultKeyword,
        body: Self::YulBlock,
    ) -> Self::YulDefaultCase;
    fn make_yul_flags_declaration(
        &self,
        open_paren: Self::YulOpenParen,
        flags: Self::YulFlags,
        close_paren: Self::YulCloseParen,
    ) -> Self::YulFlagsDeclaration;
    fn make_yul_for_statement(
        &self,
        for_keyword: Self::YulForKeyword,
        initialization: Self::YulBlock,
        condition: Self::YulExpression,
        iterator: Self::YulBlock,
        body: Self::YulBlock,
    ) -> Self::YulForStatement;
    fn make_yul_function_call_expression(
        &self,
        operand: Self::YulExpression,
        open_paren: Self::YulOpenParen,
        arguments: Self::YulArguments,
        close_paren: Self::YulCloseParen,
    ) -> Self::YulFunctionCallExpression;
    fn make_yul_function_definition(
        &self,
        function_keyword: Self::YulFunctionKeyword,
        name: Self::YulIdentifier,
        parameters: Self::YulParametersDeclaration,
        returns: Option<Self::YulReturnsDeclaration>,
        body: Self::YulBlock,
    ) -> Self::YulFunctionDefinition;
    fn make_yul_if_statement(
        &self,
        if_keyword: Self::YulIfKeyword,
        condition: Self::YulExpression,
        body: Self::YulBlock,
    ) -> Self::YulIfStatement;
    fn make_yul_leave_statement(
        &self,
        leave_keyword: Self::YulLeaveKeyword,
    ) -> Self::YulLeaveStatement;
    fn make_yul_parameters_declaration(
        &self,
        open_paren: Self::YulOpenParen,
        parameters: Self::YulParameters,
        close_paren: Self::YulCloseParen,
    ) -> Self::YulParametersDeclaration;
    fn make_yul_returns_declaration(
        &self,
        minus_greater_than: Self::YulMinusGreaterThan,
        variables: Self::YulVariableNames,
    ) -> Self::YulReturnsDeclaration;
    fn make_yul_switch_statement(
        &self,
        switch_keyword: Self::YulSwitchKeyword,
        expression: Self::YulExpression,
        cases: Self::YulSwitchCases,
    ) -> Self::YulSwitchStatement;
    fn make_yul_value_case(
        &self,
        case_keyword: Self::YulCaseKeyword,
        value: Self::YulLiteral,
        body: Self::YulBlock,
    ) -> Self::YulValueCase;
    fn make_yul_variable_assignment_statement(
        &self,
        variables: Self::YulPaths,
        assignment: Self::YulColonEqual,
        expression: Self::YulExpression,
    ) -> Self::YulVariableAssignmentStatement;
    fn make_yul_variable_declaration_statement(
        &self,
        let_keyword: Self::YulLetKeyword,
        variables: Self::YulVariableNames,
        value: Option<Self::YulVariableDeclarationValue>,
    ) -> Self::YulVariableDeclarationStatement;
    fn make_yul_variable_declaration_value(
        &self,
        assignment: Self::YulColonEqual,
        expression: Self::YulExpression,
    ) -> Self::YulVariableDeclarationValue;

    // Choice variant factory methods
    fn make_abicoder_version_abicoder_v1_keyword(
        &self,
        element: Self::AbicoderV1Keyword,
    ) -> Self::AbicoderVersion;
    fn make_abicoder_version_abicoder_v2_keyword(
        &self,
        element: Self::AbicoderV2Keyword,
    ) -> Self::AbicoderVersion;

    fn make_arguments_declaration_positional_arguments_declaration(
        &self,
        element: Self::PositionalArgumentsDeclaration,
    ) -> Self::ArgumentsDeclaration;
    fn make_arguments_declaration_named_arguments_declaration(
        &self,
        element: Self::NamedArgumentsDeclaration,
    ) -> Self::ArgumentsDeclaration;

    fn make_constructor_attribute_modifier_invocation(
        &self,
        element: Self::ModifierInvocation,
    ) -> Self::ConstructorAttribute;
    fn make_constructor_attribute_internal_keyword(
        &self,
        element: Self::InternalKeyword,
    ) -> Self::ConstructorAttribute;
    fn make_constructor_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::ConstructorAttribute;
    fn make_constructor_attribute_public_keyword(
        &self,
        element: Self::PublicKeyword,
    ) -> Self::ConstructorAttribute;

    fn make_contract_member_using_directive(
        &self,
        element: Self::UsingDirective,
    ) -> Self::ContractMember;
    fn make_contract_member_function_definition(
        &self,
        element: Self::FunctionDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_constructor_definition(
        &self,
        element: Self::ConstructorDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_receive_function_definition(
        &self,
        element: Self::ReceiveFunctionDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_fallback_function_definition(
        &self,
        element: Self::FallbackFunctionDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_modifier_definition(
        &self,
        element: Self::ModifierDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_struct_definition(
        &self,
        element: Self::StructDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_enum_definition(
        &self,
        element: Self::EnumDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_event_definition(
        &self,
        element: Self::EventDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_error_definition(
        &self,
        element: Self::ErrorDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_user_defined_value_type_definition(
        &self,
        element: Self::UserDefinedValueTypeDefinition,
    ) -> Self::ContractMember;
    fn make_contract_member_state_variable_definition(
        &self,
        element: Self::StateVariableDefinition,
    ) -> Self::ContractMember;

    fn make_contract_specifier_inheritance_specifier(
        &self,
        element: Self::InheritanceSpecifier,
    ) -> Self::ContractSpecifier;
    fn make_contract_specifier_storage_layout_specifier(
        &self,
        element: Self::StorageLayoutSpecifier,
    ) -> Self::ContractSpecifier;

    fn make_elementary_type_bool_keyword(&self, element: Self::BoolKeyword)
        -> Self::ElementaryType;
    fn make_elementary_type_string_keyword(
        &self,
        element: Self::StringKeyword,
    ) -> Self::ElementaryType;
    fn make_elementary_type_address_type(&self, element: Self::AddressType)
        -> Self::ElementaryType;
    fn make_elementary_type_bytes_keyword(
        &self,
        element: Self::BytesKeyword,
    ) -> Self::ElementaryType;
    fn make_elementary_type_int_keyword(&self, element: Self::IntKeyword) -> Self::ElementaryType;
    fn make_elementary_type_uint_keyword(&self, element: Self::UintKeyword)
        -> Self::ElementaryType;
    fn make_elementary_type_fixed_keyword(
        &self,
        element: Self::FixedKeyword,
    ) -> Self::ElementaryType;
    fn make_elementary_type_ufixed_keyword(
        &self,
        element: Self::UfixedKeyword,
    ) -> Self::ElementaryType;

    fn make_experimental_feature_abi_encoder_v2_keyword(
        &self,
        element: Self::ABIEncoderV2Keyword,
    ) -> Self::ExperimentalFeature;
    fn make_experimental_feature_smt_checker_keyword(
        &self,
        element: Self::SMTCheckerKeyword,
    ) -> Self::ExperimentalFeature;
    fn make_experimental_feature_pragma_string_literal(
        &self,
        element: Self::PragmaStringLiteral,
    ) -> Self::ExperimentalFeature;

    fn make_expression_assignment_expression(
        &self,
        element: Self::AssignmentExpression,
    ) -> Self::Expression;
    fn make_expression_conditional_expression(
        &self,
        element: Self::ConditionalExpression,
    ) -> Self::Expression;
    fn make_expression_or_expression(&self, element: Self::OrExpression) -> Self::Expression;
    fn make_expression_and_expression(&self, element: Self::AndExpression) -> Self::Expression;
    fn make_expression_equality_expression(
        &self,
        element: Self::EqualityExpression,
    ) -> Self::Expression;
    fn make_expression_inequality_expression(
        &self,
        element: Self::InequalityExpression,
    ) -> Self::Expression;
    fn make_expression_bitwise_or_expression(
        &self,
        element: Self::BitwiseOrExpression,
    ) -> Self::Expression;
    fn make_expression_bitwise_xor_expression(
        &self,
        element: Self::BitwiseXorExpression,
    ) -> Self::Expression;
    fn make_expression_bitwise_and_expression(
        &self,
        element: Self::BitwiseAndExpression,
    ) -> Self::Expression;
    fn make_expression_shift_expression(&self, element: Self::ShiftExpression) -> Self::Expression;
    fn make_expression_additive_expression(
        &self,
        element: Self::AdditiveExpression,
    ) -> Self::Expression;
    fn make_expression_multiplicative_expression(
        &self,
        element: Self::MultiplicativeExpression,
    ) -> Self::Expression;
    fn make_expression_exponentiation_expression(
        &self,
        element: Self::ExponentiationExpression,
    ) -> Self::Expression;
    fn make_expression_postfix_expression(
        &self,
        element: Self::PostfixExpression,
    ) -> Self::Expression;
    fn make_expression_prefix_expression(
        &self,
        element: Self::PrefixExpression,
    ) -> Self::Expression;
    fn make_expression_function_call_expression(
        &self,
        element: Self::FunctionCallExpression,
    ) -> Self::Expression;
    fn make_expression_call_options_expression(
        &self,
        element: Self::CallOptionsExpression,
    ) -> Self::Expression;
    fn make_expression_member_access_expression(
        &self,
        element: Self::MemberAccessExpression,
    ) -> Self::Expression;
    fn make_expression_index_access_expression(
        &self,
        element: Self::IndexAccessExpression,
    ) -> Self::Expression;
    fn make_expression_new_expression(&self, element: Self::NewExpression) -> Self::Expression;
    fn make_expression_tuple_expression(&self, element: Self::TupleExpression) -> Self::Expression;
    fn make_expression_type_expression(&self, element: Self::TypeExpression) -> Self::Expression;
    fn make_expression_array_expression(&self, element: Self::ArrayExpression) -> Self::Expression;
    fn make_expression_hex_number_expression(
        &self,
        element: Self::HexNumberExpression,
    ) -> Self::Expression;
    fn make_expression_decimal_number_expression(
        &self,
        element: Self::DecimalNumberExpression,
    ) -> Self::Expression;
    fn make_expression_string_expression(
        &self,
        element: Self::StringExpression,
    ) -> Self::Expression;
    fn make_expression_elementary_type(&self, element: Self::ElementaryType) -> Self::Expression;
    fn make_expression_payable_keyword(&self, element: Self::PayableKeyword) -> Self::Expression;
    fn make_expression_this_keyword(&self, element: Self::ThisKeyword) -> Self::Expression;
    fn make_expression_super_keyword(&self, element: Self::SuperKeyword) -> Self::Expression;
    fn make_expression_true_keyword(&self, element: Self::TrueKeyword) -> Self::Expression;
    fn make_expression_false_keyword(&self, element: Self::FalseKeyword) -> Self::Expression;
    fn make_expression_identifier(&self, element: Self::Identifier) -> Self::Expression;

    fn make_expression_additive_expression_operator_minus(
        &self,
        element: Self::Minus,
    ) -> Self::Expression_AdditiveExpression_Operator;
    fn make_expression_additive_expression_operator_plus(
        &self,
        element: Self::Plus,
    ) -> Self::Expression_AdditiveExpression_Operator;

    fn make_expression_assignment_expression_operator_ampersand_equal(
        &self,
        element: Self::AmpersandEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_asterisk_equal(
        &self,
        element: Self::AsteriskEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_bar_equal(
        &self,
        element: Self::BarEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_caret_equal(
        &self,
        element: Self::CaretEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_equal(
        &self,
        element: Self::Equal,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_greater_than_greater_than_equal(
        &self,
        element: Self::GreaterThanGreaterThanEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_greater_than_greater_than_greater_than_equal(
        &self,
        element: Self::GreaterThanGreaterThanGreaterThanEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_less_than_less_than_equal(
        &self,
        element: Self::LessThanLessThanEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_minus_equal(
        &self,
        element: Self::MinusEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_percent_equal(
        &self,
        element: Self::PercentEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_plus_equal(
        &self,
        element: Self::PlusEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;
    fn make_expression_assignment_expression_operator_slash_equal(
        &self,
        element: Self::SlashEqual,
    ) -> Self::Expression_AssignmentExpression_Operator;

    fn make_expression_equality_expression_operator_bang_equal(
        &self,
        element: Self::BangEqual,
    ) -> Self::Expression_EqualityExpression_Operator;
    fn make_expression_equality_expression_operator_equal_equal(
        &self,
        element: Self::EqualEqual,
    ) -> Self::Expression_EqualityExpression_Operator;

    fn make_expression_inequality_expression_operator_greater_than(
        &self,
        element: Self::GreaterThan,
    ) -> Self::Expression_InequalityExpression_Operator;
    fn make_expression_inequality_expression_operator_greater_than_equal(
        &self,
        element: Self::GreaterThanEqual,
    ) -> Self::Expression_InequalityExpression_Operator;
    fn make_expression_inequality_expression_operator_less_than(
        &self,
        element: Self::LessThan,
    ) -> Self::Expression_InequalityExpression_Operator;
    fn make_expression_inequality_expression_operator_less_than_equal(
        &self,
        element: Self::LessThanEqual,
    ) -> Self::Expression_InequalityExpression_Operator;

    fn make_expression_multiplicative_expression_operator_asterisk(
        &self,
        element: Self::Asterisk,
    ) -> Self::Expression_MultiplicativeExpression_Operator;
    fn make_expression_multiplicative_expression_operator_percent(
        &self,
        element: Self::Percent,
    ) -> Self::Expression_MultiplicativeExpression_Operator;
    fn make_expression_multiplicative_expression_operator_slash(
        &self,
        element: Self::Slash,
    ) -> Self::Expression_MultiplicativeExpression_Operator;

    fn make_expression_postfix_expression_operator_minus_minus(
        &self,
        element: Self::MinusMinus,
    ) -> Self::Expression_PostfixExpression_Operator;
    fn make_expression_postfix_expression_operator_plus_plus(
        &self,
        element: Self::PlusPlus,
    ) -> Self::Expression_PostfixExpression_Operator;

    fn make_expression_prefix_expression_operator_bang(
        &self,
        element: Self::Bang,
    ) -> Self::Expression_PrefixExpression_Operator;
    fn make_expression_prefix_expression_operator_delete_keyword(
        &self,
        element: Self::DeleteKeyword,
    ) -> Self::Expression_PrefixExpression_Operator;
    fn make_expression_prefix_expression_operator_minus(
        &self,
        element: Self::Minus,
    ) -> Self::Expression_PrefixExpression_Operator;
    fn make_expression_prefix_expression_operator_minus_minus(
        &self,
        element: Self::MinusMinus,
    ) -> Self::Expression_PrefixExpression_Operator;
    fn make_expression_prefix_expression_operator_plus_plus(
        &self,
        element: Self::PlusPlus,
    ) -> Self::Expression_PrefixExpression_Operator;
    fn make_expression_prefix_expression_operator_tilde(
        &self,
        element: Self::Tilde,
    ) -> Self::Expression_PrefixExpression_Operator;

    fn make_expression_shift_expression_operator_greater_than_greater_than(
        &self,
        element: Self::GreaterThanGreaterThan,
    ) -> Self::Expression_ShiftExpression_Operator;
    fn make_expression_shift_expression_operator_greater_than_greater_than_greater_than(
        &self,
        element: Self::GreaterThanGreaterThanGreaterThan,
    ) -> Self::Expression_ShiftExpression_Operator;
    fn make_expression_shift_expression_operator_less_than_less_than(
        &self,
        element: Self::LessThanLessThan,
    ) -> Self::Expression_ShiftExpression_Operator;

    fn make_fallback_function_attribute_modifier_invocation(
        &self,
        element: Self::ModifierInvocation,
    ) -> Self::FallbackFunctionAttribute;
    fn make_fallback_function_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::FallbackFunctionAttribute;
    fn make_fallback_function_attribute_external_keyword(
        &self,
        element: Self::ExternalKeyword,
    ) -> Self::FallbackFunctionAttribute;
    fn make_fallback_function_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::FallbackFunctionAttribute;
    fn make_fallback_function_attribute_pure_keyword(
        &self,
        element: Self::PureKeyword,
    ) -> Self::FallbackFunctionAttribute;
    fn make_fallback_function_attribute_view_keyword(
        &self,
        element: Self::ViewKeyword,
    ) -> Self::FallbackFunctionAttribute;
    fn make_fallback_function_attribute_virtual_keyword(
        &self,
        element: Self::VirtualKeyword,
    ) -> Self::FallbackFunctionAttribute;

    fn make_for_statement_condition_expression_statement(
        &self,
        element: Self::ExpressionStatement,
    ) -> Self::ForStatementCondition;
    fn make_for_statement_condition_semicolon(
        &self,
        element: Self::Semicolon,
    ) -> Self::ForStatementCondition;

    fn make_for_statement_initialization_variable_declaration_statement(
        &self,
        element: Self::VariableDeclarationStatement,
    ) -> Self::ForStatementInitialization;
    fn make_for_statement_initialization_expression_statement(
        &self,
        element: Self::ExpressionStatement,
    ) -> Self::ForStatementInitialization;
    fn make_for_statement_initialization_semicolon(
        &self,
        element: Self::Semicolon,
    ) -> Self::ForStatementInitialization;

    fn make_function_attribute_modifier_invocation(
        &self,
        element: Self::ModifierInvocation,
    ) -> Self::FunctionAttribute;
    fn make_function_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::FunctionAttribute;
    fn make_function_attribute_external_keyword(
        &self,
        element: Self::ExternalKeyword,
    ) -> Self::FunctionAttribute;
    fn make_function_attribute_internal_keyword(
        &self,
        element: Self::InternalKeyword,
    ) -> Self::FunctionAttribute;
    fn make_function_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::FunctionAttribute;
    fn make_function_attribute_private_keyword(
        &self,
        element: Self::PrivateKeyword,
    ) -> Self::FunctionAttribute;
    fn make_function_attribute_public_keyword(
        &self,
        element: Self::PublicKeyword,
    ) -> Self::FunctionAttribute;
    fn make_function_attribute_pure_keyword(
        &self,
        element: Self::PureKeyword,
    ) -> Self::FunctionAttribute;
    fn make_function_attribute_view_keyword(
        &self,
        element: Self::ViewKeyword,
    ) -> Self::FunctionAttribute;
    fn make_function_attribute_virtual_keyword(
        &self,
        element: Self::VirtualKeyword,
    ) -> Self::FunctionAttribute;

    fn make_function_body_block(&self, element: Self::Block) -> Self::FunctionBody;
    fn make_function_body_semicolon(&self, element: Self::Semicolon) -> Self::FunctionBody;

    fn make_function_name_identifier(&self, element: Self::Identifier) -> Self::FunctionName;
    fn make_function_name_fallback_keyword(
        &self,
        element: Self::FallbackKeyword,
    ) -> Self::FunctionName;
    fn make_function_name_receive_keyword(
        &self,
        element: Self::ReceiveKeyword,
    ) -> Self::FunctionName;

    fn make_function_type_attribute_internal_keyword(
        &self,
        element: Self::InternalKeyword,
    ) -> Self::FunctionTypeAttribute;
    fn make_function_type_attribute_external_keyword(
        &self,
        element: Self::ExternalKeyword,
    ) -> Self::FunctionTypeAttribute;
    fn make_function_type_attribute_private_keyword(
        &self,
        element: Self::PrivateKeyword,
    ) -> Self::FunctionTypeAttribute;
    fn make_function_type_attribute_public_keyword(
        &self,
        element: Self::PublicKeyword,
    ) -> Self::FunctionTypeAttribute;
    fn make_function_type_attribute_pure_keyword(
        &self,
        element: Self::PureKeyword,
    ) -> Self::FunctionTypeAttribute;
    fn make_function_type_attribute_view_keyword(
        &self,
        element: Self::ViewKeyword,
    ) -> Self::FunctionTypeAttribute;
    fn make_function_type_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::FunctionTypeAttribute;

    fn make_identifier_path_element_identifier(
        &self,
        element: Self::Identifier,
    ) -> Self::IdentifierPathElement;
    fn make_identifier_path_element_address_keyword(
        &self,
        element: Self::AddressKeyword,
    ) -> Self::IdentifierPathElement;

    fn make_import_clause_path_import(&self, element: Self::PathImport) -> Self::ImportClause;
    fn make_import_clause_named_import(&self, element: Self::NamedImport) -> Self::ImportClause;
    fn make_import_clause_import_deconstruction(
        &self,
        element: Self::ImportDeconstruction,
    ) -> Self::ImportClause;

    fn make_mapping_key_type_elementary_type(
        &self,
        element: Self::ElementaryType,
    ) -> Self::MappingKeyType;
    fn make_mapping_key_type_identifier_path(
        &self,
        element: Self::IdentifierPath,
    ) -> Self::MappingKeyType;

    fn make_modifier_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::ModifierAttribute;
    fn make_modifier_attribute_virtual_keyword(
        &self,
        element: Self::VirtualKeyword,
    ) -> Self::ModifierAttribute;

    fn make_number_unit_wei_keyword(&self, element: Self::WeiKeyword) -> Self::NumberUnit;
    fn make_number_unit_gwei_keyword(&self, element: Self::GweiKeyword) -> Self::NumberUnit;
    fn make_number_unit_ether_keyword(&self, element: Self::EtherKeyword) -> Self::NumberUnit;
    fn make_number_unit_seconds_keyword(&self, element: Self::SecondsKeyword) -> Self::NumberUnit;
    fn make_number_unit_minutes_keyword(&self, element: Self::MinutesKeyword) -> Self::NumberUnit;
    fn make_number_unit_hours_keyword(&self, element: Self::HoursKeyword) -> Self::NumberUnit;
    fn make_number_unit_days_keyword(&self, element: Self::DaysKeyword) -> Self::NumberUnit;
    fn make_number_unit_weeks_keyword(&self, element: Self::WeeksKeyword) -> Self::NumberUnit;

    fn make_pragma_version_pragma(&self, element: Self::VersionPragma) -> Self::Pragma;
    fn make_pragma_abicoder_pragma(&self, element: Self::AbicoderPragma) -> Self::Pragma;
    fn make_pragma_experimental_pragma(&self, element: Self::ExperimentalPragma) -> Self::Pragma;

    fn make_receive_function_attribute_modifier_invocation(
        &self,
        element: Self::ModifierInvocation,
    ) -> Self::ReceiveFunctionAttribute;
    fn make_receive_function_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::ReceiveFunctionAttribute;
    fn make_receive_function_attribute_external_keyword(
        &self,
        element: Self::ExternalKeyword,
    ) -> Self::ReceiveFunctionAttribute;
    fn make_receive_function_attribute_payable_keyword(
        &self,
        element: Self::PayableKeyword,
    ) -> Self::ReceiveFunctionAttribute;
    fn make_receive_function_attribute_virtual_keyword(
        &self,
        element: Self::VirtualKeyword,
    ) -> Self::ReceiveFunctionAttribute;

    fn make_source_unit_member_pragma_directive(
        &self,
        element: Self::PragmaDirective,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_import_directive(
        &self,
        element: Self::ImportDirective,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_contract_definition(
        &self,
        element: Self::ContractDefinition,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_interface_definition(
        &self,
        element: Self::InterfaceDefinition,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_library_definition(
        &self,
        element: Self::LibraryDefinition,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_struct_definition(
        &self,
        element: Self::StructDefinition,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_enum_definition(
        &self,
        element: Self::EnumDefinition,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_function_definition(
        &self,
        element: Self::FunctionDefinition,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_error_definition(
        &self,
        element: Self::ErrorDefinition,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_user_defined_value_type_definition(
        &self,
        element: Self::UserDefinedValueTypeDefinition,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_using_directive(
        &self,
        element: Self::UsingDirective,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_event_definition(
        &self,
        element: Self::EventDefinition,
    ) -> Self::SourceUnitMember;
    fn make_source_unit_member_constant_definition(
        &self,
        element: Self::ConstantDefinition,
    ) -> Self::SourceUnitMember;

    fn make_state_variable_attribute_override_specifier(
        &self,
        element: Self::OverrideSpecifier,
    ) -> Self::StateVariableAttribute;
    fn make_state_variable_attribute_constant_keyword(
        &self,
        element: Self::ConstantKeyword,
    ) -> Self::StateVariableAttribute;
    fn make_state_variable_attribute_internal_keyword(
        &self,
        element: Self::InternalKeyword,
    ) -> Self::StateVariableAttribute;
    fn make_state_variable_attribute_private_keyword(
        &self,
        element: Self::PrivateKeyword,
    ) -> Self::StateVariableAttribute;
    fn make_state_variable_attribute_public_keyword(
        &self,
        element: Self::PublicKeyword,
    ) -> Self::StateVariableAttribute;
    fn make_state_variable_attribute_immutable_keyword(
        &self,
        element: Self::ImmutableKeyword,
    ) -> Self::StateVariableAttribute;
    fn make_state_variable_attribute_transient_keyword(
        &self,
        element: Self::TransientKeyword,
    ) -> Self::StateVariableAttribute;

    fn make_statement_if_statement(&self, element: Self::IfStatement) -> Self::Statement;
    fn make_statement_for_statement(&self, element: Self::ForStatement) -> Self::Statement;
    fn make_statement_while_statement(&self, element: Self::WhileStatement) -> Self::Statement;
    fn make_statement_do_while_statement(&self, element: Self::DoWhileStatement)
        -> Self::Statement;
    fn make_statement_continue_statement(
        &self,
        element: Self::ContinueStatement,
    ) -> Self::Statement;
    fn make_statement_break_statement(&self, element: Self::BreakStatement) -> Self::Statement;
    fn make_statement_return_statement(&self, element: Self::ReturnStatement) -> Self::Statement;
    fn make_statement_emit_statement(&self, element: Self::EmitStatement) -> Self::Statement;
    fn make_statement_try_statement(&self, element: Self::TryStatement) -> Self::Statement;
    fn make_statement_revert_statement(&self, element: Self::RevertStatement) -> Self::Statement;
    fn make_statement_assembly_statement(
        &self,
        element: Self::AssemblyStatement,
    ) -> Self::Statement;
    fn make_statement_block(&self, element: Self::Block) -> Self::Statement;
    fn make_statement_unchecked_block(&self, element: Self::UncheckedBlock) -> Self::Statement;
    fn make_statement_variable_declaration_statement(
        &self,
        element: Self::VariableDeclarationStatement,
    ) -> Self::Statement;
    fn make_statement_expression_statement(
        &self,
        element: Self::ExpressionStatement,
    ) -> Self::Statement;

    fn make_storage_location_memory_keyword(
        &self,
        element: Self::MemoryKeyword,
    ) -> Self::StorageLocation;
    fn make_storage_location_storage_keyword(
        &self,
        element: Self::StorageKeyword,
    ) -> Self::StorageLocation;
    fn make_storage_location_call_data_keyword(
        &self,
        element: Self::CallDataKeyword,
    ) -> Self::StorageLocation;

    fn make_string_expression_string_literals(
        &self,
        element: Self::StringLiterals,
    ) -> Self::StringExpression;
    fn make_string_expression_hex_string_literals(
        &self,
        element: Self::HexStringLiterals,
    ) -> Self::StringExpression;
    fn make_string_expression_unicode_string_literals(
        &self,
        element: Self::UnicodeStringLiterals,
    ) -> Self::StringExpression;

    fn make_type_name_array_type_name(&self, element: Self::ArrayTypeName) -> Self::TypeName;
    fn make_type_name_function_type(&self, element: Self::FunctionType) -> Self::TypeName;
    fn make_type_name_mapping_type(&self, element: Self::MappingType) -> Self::TypeName;
    fn make_type_name_elementary_type(&self, element: Self::ElementaryType) -> Self::TypeName;
    fn make_type_name_identifier_path(&self, element: Self::IdentifierPath) -> Self::TypeName;

    fn make_using_clause_identifier_path(&self, element: Self::IdentifierPath)
        -> Self::UsingClause;
    fn make_using_clause_using_deconstruction(
        &self,
        element: Self::UsingDeconstruction,
    ) -> Self::UsingClause;

    fn make_using_operator_ampersand(&self, element: Self::Ampersand) -> Self::UsingOperator;
    fn make_using_operator_asterisk(&self, element: Self::Asterisk) -> Self::UsingOperator;
    fn make_using_operator_bang_equal(&self, element: Self::BangEqual) -> Self::UsingOperator;
    fn make_using_operator_bar(&self, element: Self::Bar) -> Self::UsingOperator;
    fn make_using_operator_caret(&self, element: Self::Caret) -> Self::UsingOperator;
    fn make_using_operator_equal_equal(&self, element: Self::EqualEqual) -> Self::UsingOperator;
    fn make_using_operator_greater_than(&self, element: Self::GreaterThan) -> Self::UsingOperator;
    fn make_using_operator_greater_than_equal(
        &self,
        element: Self::GreaterThanEqual,
    ) -> Self::UsingOperator;
    fn make_using_operator_less_than(&self, element: Self::LessThan) -> Self::UsingOperator;
    fn make_using_operator_less_than_equal(
        &self,
        element: Self::LessThanEqual,
    ) -> Self::UsingOperator;
    fn make_using_operator_minus(&self, element: Self::Minus) -> Self::UsingOperator;
    fn make_using_operator_percent(&self, element: Self::Percent) -> Self::UsingOperator;
    fn make_using_operator_plus(&self, element: Self::Plus) -> Self::UsingOperator;
    fn make_using_operator_slash(&self, element: Self::Slash) -> Self::UsingOperator;
    fn make_using_operator_tilde(&self, element: Self::Tilde) -> Self::UsingOperator;

    fn make_using_target_type_name(&self, element: Self::TypeName) -> Self::UsingTarget;
    fn make_using_target_asterisk(&self, element: Self::Asterisk) -> Self::UsingTarget;

    fn make_variable_declaration_target_single_typed_declaration(
        &self,
        element: Self::SingleTypedDeclaration,
    ) -> Self::VariableDeclarationTarget;
    fn make_variable_declaration_target_multi_typed_declaration(
        &self,
        element: Self::MultiTypedDeclaration,
    ) -> Self::VariableDeclarationTarget;

    fn make_version_expression_version_range(
        &self,
        element: Self::VersionRange,
    ) -> Self::VersionExpression;
    fn make_version_expression_version_term(
        &self,
        element: Self::VersionTerm,
    ) -> Self::VersionExpression;

    fn make_version_literal_simple_version_literal(
        &self,
        element: Self::SimpleVersionLiteral,
    ) -> Self::VersionLiteral;
    fn make_version_literal_pragma_string_literal(
        &self,
        element: Self::PragmaStringLiteral,
    ) -> Self::VersionLiteral;

    fn make_version_operator_pragma_caret(
        &self,
        element: Self::PragmaCaret,
    ) -> Self::VersionOperator;
    fn make_version_operator_pragma_tilde(
        &self,
        element: Self::PragmaTilde,
    ) -> Self::VersionOperator;
    fn make_version_operator_pragma_equal(
        &self,
        element: Self::PragmaEqual,
    ) -> Self::VersionOperator;
    fn make_version_operator_pragma_less_than(
        &self,
        element: Self::PragmaLessThan,
    ) -> Self::VersionOperator;
    fn make_version_operator_pragma_greater_than(
        &self,
        element: Self::PragmaGreaterThan,
    ) -> Self::VersionOperator;
    fn make_version_operator_pragma_less_than_equal(
        &self,
        element: Self::PragmaLessThanEqual,
    ) -> Self::VersionOperator;
    fn make_version_operator_pragma_greater_than_equal(
        &self,
        element: Self::PragmaGreaterThanEqual,
    ) -> Self::VersionOperator;

    fn make_yul_expression_yul_function_call_expression(
        &self,
        element: Self::YulFunctionCallExpression,
    ) -> Self::YulExpression;
    fn make_yul_expression_yul_literal(&self, element: Self::YulLiteral) -> Self::YulExpression;
    fn make_yul_expression_yul_path(&self, element: Self::YulPath) -> Self::YulExpression;

    fn make_yul_literal_yul_true_keyword(&self, element: Self::YulTrueKeyword) -> Self::YulLiteral;
    fn make_yul_literal_yul_false_keyword(
        &self,
        element: Self::YulFalseKeyword,
    ) -> Self::YulLiteral;
    fn make_yul_literal_yul_decimal_literal(
        &self,
        element: Self::YulDecimalLiteral,
    ) -> Self::YulLiteral;
    fn make_yul_literal_yul_hex_literal(&self, element: Self::YulHexLiteral) -> Self::YulLiteral;
    fn make_yul_literal_yul_hex_string_literal(
        &self,
        element: Self::YulHexStringLiteral,
    ) -> Self::YulLiteral;
    fn make_yul_literal_yul_string_literal(
        &self,
        element: Self::YulStringLiteral,
    ) -> Self::YulLiteral;

    fn make_yul_statement_yul_block(&self, element: Self::YulBlock) -> Self::YulStatement;
    fn make_yul_statement_yul_function_definition(
        &self,
        element: Self::YulFunctionDefinition,
    ) -> Self::YulStatement;
    fn make_yul_statement_yul_if_statement(
        &self,
        element: Self::YulIfStatement,
    ) -> Self::YulStatement;
    fn make_yul_statement_yul_for_statement(
        &self,
        element: Self::YulForStatement,
    ) -> Self::YulStatement;
    fn make_yul_statement_yul_switch_statement(
        &self,
        element: Self::YulSwitchStatement,
    ) -> Self::YulStatement;
    fn make_yul_statement_yul_leave_statement(
        &self,
        element: Self::YulLeaveStatement,
    ) -> Self::YulStatement;
    fn make_yul_statement_yul_break_statement(
        &self,
        element: Self::YulBreakStatement,
    ) -> Self::YulStatement;
    fn make_yul_statement_yul_continue_statement(
        &self,
        element: Self::YulContinueStatement,
    ) -> Self::YulStatement;
    fn make_yul_statement_yul_variable_assignment_statement(
        &self,
        element: Self::YulVariableAssignmentStatement,
    ) -> Self::YulStatement;
    fn make_yul_statement_yul_variable_declaration_statement(
        &self,
        element: Self::YulVariableDeclarationStatement,
    ) -> Self::YulStatement;
    fn make_yul_statement_yul_expression(&self, element: Self::YulExpression)
        -> Self::YulStatement;

    fn make_yul_switch_case_yul_default_case(
        &self,
        element: Self::YulDefaultCase,
    ) -> Self::YulSwitchCase;
    fn make_yul_switch_case_yul_value_case(
        &self,
        element: Self::YulValueCase,
    ) -> Self::YulSwitchCase;

    // Collection factory methods
    fn make_array_values(&self, elements: Vec<Self::Expression>) -> Self::ArrayValues;
    fn make_call_options(&self, elements: Vec<Self::NamedArgument>) -> Self::CallOptions;
    fn make_catch_clauses(&self, elements: Vec<Self::CatchClause>) -> Self::CatchClauses;
    fn make_constructor_attributes(
        &self,
        elements: Vec<Self::ConstructorAttribute>,
    ) -> Self::ConstructorAttributes;
    fn make_contract_members(&self, elements: Vec<Self::ContractMember>) -> Self::ContractMembers;
    fn make_contract_specifiers(
        &self,
        elements: Vec<Self::ContractSpecifier>,
    ) -> Self::ContractSpecifiers;
    fn make_enum_members(&self, elements: Vec<Self::Identifier>) -> Self::EnumMembers;
    fn make_error_parameters(&self, elements: Vec<Self::ErrorParameter>) -> Self::ErrorParameters;
    fn make_event_parameters(&self, elements: Vec<Self::EventParameter>) -> Self::EventParameters;
    fn make_fallback_function_attributes(
        &self,
        elements: Vec<Self::FallbackFunctionAttribute>,
    ) -> Self::FallbackFunctionAttributes;
    fn make_function_attributes(
        &self,
        elements: Vec<Self::FunctionAttribute>,
    ) -> Self::FunctionAttributes;
    fn make_function_type_attributes(
        &self,
        elements: Vec<Self::FunctionTypeAttribute>,
    ) -> Self::FunctionTypeAttributes;
    fn make_hex_string_literals(
        &self,
        elements: Vec<Self::HexStringLiteral>,
    ) -> Self::HexStringLiterals;
    fn make_identifier_path(
        &self,
        elements: Vec<Self::IdentifierPathElement>,
    ) -> Self::IdentifierPath;
    fn make_import_deconstruction_symbols(
        &self,
        elements: Vec<Self::ImportDeconstructionSymbol>,
    ) -> Self::ImportDeconstructionSymbols;
    fn make_inheritance_types(
        &self,
        elements: Vec<Self::InheritanceType>,
    ) -> Self::InheritanceTypes;
    fn make_interface_members(&self, elements: Vec<Self::ContractMember>)
        -> Self::InterfaceMembers;
    fn make_library_members(&self, elements: Vec<Self::ContractMember>) -> Self::LibraryMembers;
    fn make_modifier_attributes(
        &self,
        elements: Vec<Self::ModifierAttribute>,
    ) -> Self::ModifierAttributes;
    fn make_multi_typed_declaration_elements(
        &self,
        elements: Vec<Self::MultiTypedDeclarationElement>,
    ) -> Self::MultiTypedDeclarationElements;
    fn make_named_arguments(&self, elements: Vec<Self::NamedArgument>) -> Self::NamedArguments;
    fn make_override_paths(&self, elements: Vec<Self::IdentifierPath>) -> Self::OverridePaths;
    fn make_parameters(&self, elements: Vec<Self::Parameter>) -> Self::Parameters;
    fn make_positional_arguments(
        &self,
        elements: Vec<Self::Expression>,
    ) -> Self::PositionalArguments;
    fn make_receive_function_attributes(
        &self,
        elements: Vec<Self::ReceiveFunctionAttribute>,
    ) -> Self::ReceiveFunctionAttributes;
    fn make_simple_version_literal(
        &self,
        elements: Vec<Self::VersionSpecifier>,
    ) -> Self::SimpleVersionLiteral;
    fn make_source_unit_members(
        &self,
        elements: Vec<Self::SourceUnitMember>,
    ) -> Self::SourceUnitMembers;
    fn make_state_variable_attributes(
        &self,
        elements: Vec<Self::StateVariableAttribute>,
    ) -> Self::StateVariableAttributes;
    fn make_statements(&self, elements: Vec<Self::Statement>) -> Self::Statements;
    fn make_string_literals(&self, elements: Vec<Self::StringLiteral>) -> Self::StringLiterals;
    fn make_struct_members(&self, elements: Vec<Self::StructMember>) -> Self::StructMembers;
    fn make_tuple_values(&self, elements: Vec<Self::TupleValue>) -> Self::TupleValues;
    fn make_unicode_string_literals(
        &self,
        elements: Vec<Self::UnicodeStringLiteral>,
    ) -> Self::UnicodeStringLiterals;
    fn make_using_deconstruction_symbols(
        &self,
        elements: Vec<Self::UsingDeconstructionSymbol>,
    ) -> Self::UsingDeconstructionSymbols;
    fn make_version_expression_set(
        &self,
        elements: Vec<Self::VersionExpression>,
    ) -> Self::VersionExpressionSet;
    fn make_version_expression_sets(
        &self,
        elements: Vec<Self::VersionExpressionSet>,
    ) -> Self::VersionExpressionSets;
    fn make_yul_arguments(&self, elements: Vec<Self::YulExpression>) -> Self::YulArguments;
    fn make_yul_flags(&self, elements: Vec<Self::YulStringLiteral>) -> Self::YulFlags;
    fn make_yul_parameters(&self, elements: Vec<Self::YulIdentifier>) -> Self::YulParameters;
    fn make_yul_path(&self, elements: Vec<Self::YulIdentifier>) -> Self::YulPath;
    fn make_yul_paths(&self, elements: Vec<Self::YulPath>) -> Self::YulPaths;
    fn make_yul_statements(&self, elements: Vec<Self::YulStatement>) -> Self::YulStatements;
    fn make_yul_switch_cases(&self, elements: Vec<Self::YulSwitchCase>) -> Self::YulSwitchCases;
    fn make_yul_variable_names(&self, elements: Vec<Self::YulIdentifier>)
        -> Self::YulVariableNames;

    // ==============================
    // === Special Helper Methods ===
    // ==============================

    /// Consume an `IndexAccessPath` and produce a `TypeName`.
    ///
    /// Requires `Self: Sized` because `IndexAccessPath<Self>` stores associated
    /// types by value, not behind a reference.
    fn make_type_name_from_index_access_path(
        &self,
        index_access_path: parser_helpers::IndexAccessPath<Self>,
    ) -> Self::TypeName
    where
        Self: Sized;

    /// Consume an `IndexAccessPath` and produce an `Expression`.
    ///
    /// Requires `Self: Sized` because `IndexAccessPath<Self>` stores associated
    /// types by value, not behind a reference.
    fn make_expression_from_index_access_path(
        &self,
        index_access_path: parser_helpers::IndexAccessPath<Self>,
    ) -> Self::Expression
    where
        Self: Sized;

    /// Consume an `IdentifierPath` and produce an `Expression` by folding it
    /// into a chain of member access expressions.
    fn make_expression_from_identifier_path(
        &self,
        identifier_path: Self::IdentifierPath,
    ) -> Self::Expression;

    /// Extract trailing state variable attributes from a function type.
    ///
    /// This is a parser disambiguation concern: in `function (uint a) internal internal foo;`,
    /// the first `internal` belongs to the function type and the second to the state variable.
    /// The LR(1) grammar cannot distinguish these, so the function type captures all compatible
    /// attributes and this method extracts the trailing ones for the state variable definition.
    fn extract_extra_attributes(
        &self,
        fun_type: Self::FunctionType,
    ) -> (Self::FunctionType, Vec<Self::StateVariableAttribute>);
}
