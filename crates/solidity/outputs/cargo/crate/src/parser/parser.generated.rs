// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// This file is generated; we can't reasonably satisfy some of these lints.
#![allow(
    clippy::if_not_else,
    clippy::too_many_lines,
    clippy::unused_self,
    clippy::struct_excessive_bools,
    clippy::similar_names,
    unused_imports
)]

use semver::Version;

use crate::cst;
use crate::cst::{
    EdgeLabel, IsLexicalContext, LexicalContext, LexicalContextType, NonterminalKind, TerminalKind,
};
use crate::parser::lexer::{KeywordScan, Lexer, ScannedTerminal};
use crate::parser::parser_support::{
    ChoiceHelper, OneOrMoreHelper, OptionalHelper, ParserContext, ParserFunction, ParserResult,
    PrecedenceHelper, SeparatedHelper, SequenceHelper, ZeroOrMoreHelper,
};
use crate::parser::scanner_macros::{
    scan_char_range, scan_chars, scan_choice, scan_keyword_choice, scan_none_of,
    scan_not_followed_by, scan_one_or_more, scan_optional, scan_sequence, scan_zero_or_more,
};
use crate::parser::ParseOutput;
use crate::utils::LanguageFacts;

/// Parses `Solidity` source code. `Parser` must be initialized with a specific
/// language version that's supported by Slang. See [`LanguageFacts`] to determine what language
/// versions are available.
///
/// ```
/// use slang_solidity::parser::Parser;
/// use slang_solidity::utils::LanguageFacts;
///
/// // Initialize parser
/// let parser = Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
/// // Get source code to parse
/// let source = "contract AContract { }";
///
/// // Parse the entire file, then get a cursor to start navigating the parsed CST.
/// // Any parse errors will be reflected by error nodes in the tree. Errors can also
/// // be listed with `output.errors()`.
/// let output = parser.parse_file_contents(&source);
/// let mut cursor = output.create_tree_cursor();
/// ```
#[derive(Debug)]
pub struct Parser {
    #[allow(dead_code)]
    pub(crate) version_is_at_least_0_4_11: bool,
    pub(crate) version_is_at_least_0_4_14: bool,
    pub(crate) version_is_at_least_0_4_16: bool,
    pub(crate) version_is_at_least_0_4_21: bool,
    pub(crate) version_is_at_least_0_4_22: bool,
    pub(crate) version_is_at_least_0_4_25: bool,
    pub(crate) version_is_at_least_0_5_0: bool,
    pub(crate) version_is_at_least_0_5_3: bool,
    pub(crate) version_is_at_least_0_5_5: bool,
    pub(crate) version_is_at_least_0_5_8: bool,
    pub(crate) version_is_at_least_0_5_10: bool,
    pub(crate) version_is_at_least_0_5_14: bool,
    pub(crate) version_is_at_least_0_6_0: bool,
    pub(crate) version_is_at_least_0_6_2: bool,
    pub(crate) version_is_at_least_0_6_5: bool,
    pub(crate) version_is_at_least_0_6_7: bool,
    pub(crate) version_is_at_least_0_6_8: bool,
    pub(crate) version_is_at_least_0_6_11: bool,
    pub(crate) version_is_at_least_0_7_0: bool,
    pub(crate) version_is_at_least_0_7_1: bool,
    pub(crate) version_is_at_least_0_7_4: bool,
    pub(crate) version_is_at_least_0_7_5: bool,
    pub(crate) version_is_at_least_0_8_0: bool,
    pub(crate) version_is_at_least_0_8_4: bool,
    pub(crate) version_is_at_least_0_8_8: bool,
    pub(crate) version_is_at_least_0_8_13: bool,
    pub(crate) version_is_at_least_0_8_18: bool,
    pub(crate) version_is_at_least_0_8_19: bool,
    pub(crate) version_is_at_least_0_8_22: bool,
    pub(crate) version_is_at_least_0_8_27: bool,
    pub(crate) version_is_at_least_0_8_29: bool,
    language_version: Version,
}

/// Errors that may occur when initializing a [`Parser`].
#[derive(thiserror::Error, Debug)]
pub enum ParserInitializationError {
    /// Tried to initialize a [`Parser`] with a version that is not supported for `Solidity`.
    /// See [`LanguageFacts::ALL_VERSIONS`] for a complete list of supported versions.
    #[error("Unsupported language version '{0}'.")]
    UnsupportedLanguageVersion(Version),
}

impl Parser {
    /// Create a new `Solidity` parser that supports the specified language version.
    pub fn create(
        language_version: Version,
    ) -> std::result::Result<Self, ParserInitializationError> {
        if LanguageFacts::ALL_VERSIONS
            .binary_search(&language_version)
            .is_ok()
        {
            Ok(Self {
                version_is_at_least_0_4_11: Version::new(0, 4, 11) <= language_version,
                version_is_at_least_0_4_14: Version::new(0, 4, 14) <= language_version,
                version_is_at_least_0_4_16: Version::new(0, 4, 16) <= language_version,
                version_is_at_least_0_4_21: Version::new(0, 4, 21) <= language_version,
                version_is_at_least_0_4_22: Version::new(0, 4, 22) <= language_version,
                version_is_at_least_0_4_25: Version::new(0, 4, 25) <= language_version,
                version_is_at_least_0_5_0: Version::new(0, 5, 0) <= language_version,
                version_is_at_least_0_5_3: Version::new(0, 5, 3) <= language_version,
                version_is_at_least_0_5_5: Version::new(0, 5, 5) <= language_version,
                version_is_at_least_0_5_8: Version::new(0, 5, 8) <= language_version,
                version_is_at_least_0_5_10: Version::new(0, 5, 10) <= language_version,
                version_is_at_least_0_5_14: Version::new(0, 5, 14) <= language_version,
                version_is_at_least_0_6_0: Version::new(0, 6, 0) <= language_version,
                version_is_at_least_0_6_2: Version::new(0, 6, 2) <= language_version,
                version_is_at_least_0_6_5: Version::new(0, 6, 5) <= language_version,
                version_is_at_least_0_6_7: Version::new(0, 6, 7) <= language_version,
                version_is_at_least_0_6_8: Version::new(0, 6, 8) <= language_version,
                version_is_at_least_0_6_11: Version::new(0, 6, 11) <= language_version,
                version_is_at_least_0_7_0: Version::new(0, 7, 0) <= language_version,
                version_is_at_least_0_7_1: Version::new(0, 7, 1) <= language_version,
                version_is_at_least_0_7_4: Version::new(0, 7, 4) <= language_version,
                version_is_at_least_0_7_5: Version::new(0, 7, 5) <= language_version,
                version_is_at_least_0_8_0: Version::new(0, 8, 0) <= language_version,
                version_is_at_least_0_8_4: Version::new(0, 8, 4) <= language_version,
                version_is_at_least_0_8_8: Version::new(0, 8, 8) <= language_version,
                version_is_at_least_0_8_13: Version::new(0, 8, 13) <= language_version,
                version_is_at_least_0_8_18: Version::new(0, 8, 18) <= language_version,
                version_is_at_least_0_8_19: Version::new(0, 8, 19) <= language_version,
                version_is_at_least_0_8_22: Version::new(0, 8, 22) <= language_version,
                version_is_at_least_0_8_27: Version::new(0, 8, 27) <= language_version,
                version_is_at_least_0_8_29: Version::new(0, 8, 29) <= language_version,
                language_version,
            })
        } else {
            Err(ParserInitializationError::UnsupportedLanguageVersion(
                language_version,
            ))
        }
    }

    /// Returns the `Solidity` version that this parser supports.
    pub fn language_version(&self) -> &Version {
        &self.language_version
    }

    /// Parse the contents of an entire `Solidity` source file.
    pub fn parse_file_contents(&self, input: &str) -> ParseOutput {
        self.parse_nonterminal(NonterminalKind::SourceUnit, input)
    }

    /// Parse the given `Solidity` source code as a specific [`NonterminalKind`].
    pub fn parse_nonterminal(&self, kind: NonterminalKind, input: &str) -> ParseOutput {
        match kind {
            NonterminalKind::AbicoderPragma => Self::abicoder_pragma.parse(self, input, kind),
            NonterminalKind::AbicoderVersion => Self::abicoder_version.parse(self, input, kind),
            NonterminalKind::AdditiveExpression => {
                Self::additive_expression.parse(self, input, kind)
            }
            NonterminalKind::AddressType => Self::address_type.parse(self, input, kind),
            NonterminalKind::AndExpression => Self::and_expression.parse(self, input, kind),
            NonterminalKind::ArgumentsDeclaration => {
                Self::arguments_declaration.parse(self, input, kind)
            }
            NonterminalKind::ArrayExpression => Self::array_expression.parse(self, input, kind),
            NonterminalKind::ArrayTypeName => Self::array_type_name.parse(self, input, kind),
            NonterminalKind::ArrayValues => Self::array_values.parse(self, input, kind),
            NonterminalKind::AssemblyFlags => Self::assembly_flags.parse(self, input, kind),
            NonterminalKind::AssemblyFlagsDeclaration => {
                Self::assembly_flags_declaration.parse(self, input, kind)
            }
            NonterminalKind::AssemblyStatement => Self::assembly_statement.parse(self, input, kind),
            NonterminalKind::AssignmentExpression => {
                Self::assignment_expression.parse(self, input, kind)
            }
            NonterminalKind::BitwiseAndExpression => {
                Self::bitwise_and_expression.parse(self, input, kind)
            }
            NonterminalKind::BitwiseOrExpression => {
                Self::bitwise_or_expression.parse(self, input, kind)
            }
            NonterminalKind::BitwiseXorExpression => {
                Self::bitwise_xor_expression.parse(self, input, kind)
            }
            NonterminalKind::Block => Self::block.parse(self, input, kind),
            NonterminalKind::BreakStatement => Self::break_statement.parse(self, input, kind),
            NonterminalKind::CallOptions => Self::call_options.parse(self, input, kind),
            NonterminalKind::CallOptionsExpression => {
                Self::call_options_expression.parse(self, input, kind)
            }
            NonterminalKind::CatchClause => Self::catch_clause.parse(self, input, kind),
            NonterminalKind::CatchClauseError => Self::catch_clause_error.parse(self, input, kind),
            NonterminalKind::CatchClauses => Self::catch_clauses.parse(self, input, kind),
            NonterminalKind::ConditionalExpression => {
                Self::conditional_expression.parse(self, input, kind)
            }
            NonterminalKind::ConstantDefinition => {
                Self::constant_definition.parse(self, input, kind)
            }
            NonterminalKind::ConstructorAttribute => {
                Self::constructor_attribute.parse(self, input, kind)
            }
            NonterminalKind::ConstructorAttributes => {
                Self::constructor_attributes.parse(self, input, kind)
            }
            NonterminalKind::ConstructorDefinition => {
                Self::constructor_definition.parse(self, input, kind)
            }
            NonterminalKind::ContinueStatement => Self::continue_statement.parse(self, input, kind),
            NonterminalKind::ContractDefinition => {
                Self::contract_definition.parse(self, input, kind)
            }
            NonterminalKind::ContractMember => Self::contract_member.parse(self, input, kind),
            NonterminalKind::ContractMembers => Self::contract_members.parse(self, input, kind),
            NonterminalKind::ContractSpecifier => Self::contract_specifier.parse(self, input, kind),
            NonterminalKind::ContractSpecifiers => {
                Self::contract_specifiers.parse(self, input, kind)
            }
            NonterminalKind::DecimalNumberExpression => {
                Self::decimal_number_expression.parse(self, input, kind)
            }
            NonterminalKind::DoWhileStatement => Self::do_while_statement.parse(self, input, kind),
            NonterminalKind::ElementaryType => Self::elementary_type.parse(self, input, kind),
            NonterminalKind::ElseBranch => Self::else_branch.parse(self, input, kind),
            NonterminalKind::EmitStatement => Self::emit_statement.parse(self, input, kind),
            NonterminalKind::EnumDefinition => Self::enum_definition.parse(self, input, kind),
            NonterminalKind::EnumMembers => Self::enum_members.parse(self, input, kind),
            NonterminalKind::EqualityExpression => {
                Self::equality_expression.parse(self, input, kind)
            }
            NonterminalKind::ErrorDefinition => Self::error_definition.parse(self, input, kind),
            NonterminalKind::ErrorParameter => Self::error_parameter.parse(self, input, kind),
            NonterminalKind::ErrorParameters => Self::error_parameters.parse(self, input, kind),
            NonterminalKind::ErrorParametersDeclaration => {
                Self::error_parameters_declaration.parse(self, input, kind)
            }
            NonterminalKind::EventDefinition => Self::event_definition.parse(self, input, kind),
            NonterminalKind::EventParameter => Self::event_parameter.parse(self, input, kind),
            NonterminalKind::EventParameters => Self::event_parameters.parse(self, input, kind),
            NonterminalKind::EventParametersDeclaration => {
                Self::event_parameters_declaration.parse(self, input, kind)
            }
            NonterminalKind::ExperimentalFeature => {
                Self::experimental_feature.parse(self, input, kind)
            }
            NonterminalKind::ExperimentalPragma => {
                Self::experimental_pragma.parse(self, input, kind)
            }
            NonterminalKind::ExponentiationExpression => {
                Self::exponentiation_expression.parse(self, input, kind)
            }
            NonterminalKind::Expression => Self::expression.parse(self, input, kind),
            NonterminalKind::ExpressionStatement => {
                Self::expression_statement.parse(self, input, kind)
            }
            NonterminalKind::FallbackFunctionAttribute => {
                Self::fallback_function_attribute.parse(self, input, kind)
            }
            NonterminalKind::FallbackFunctionAttributes => {
                Self::fallback_function_attributes.parse(self, input, kind)
            }
            NonterminalKind::FallbackFunctionDefinition => {
                Self::fallback_function_definition.parse(self, input, kind)
            }
            NonterminalKind::ForStatement => Self::for_statement.parse(self, input, kind),
            NonterminalKind::ForStatementCondition => {
                Self::for_statement_condition.parse(self, input, kind)
            }
            NonterminalKind::ForStatementInitialization => {
                Self::for_statement_initialization.parse(self, input, kind)
            }
            NonterminalKind::FunctionAttribute => Self::function_attribute.parse(self, input, kind),
            NonterminalKind::FunctionAttributes => {
                Self::function_attributes.parse(self, input, kind)
            }
            NonterminalKind::FunctionBody => Self::function_body.parse(self, input, kind),
            NonterminalKind::FunctionCallExpression => {
                Self::function_call_expression.parse(self, input, kind)
            }
            NonterminalKind::FunctionDefinition => {
                Self::function_definition.parse(self, input, kind)
            }
            NonterminalKind::FunctionName => Self::function_name.parse(self, input, kind),
            NonterminalKind::FunctionType => Self::function_type.parse(self, input, kind),
            NonterminalKind::FunctionTypeAttribute => {
                Self::function_type_attribute.parse(self, input, kind)
            }
            NonterminalKind::FunctionTypeAttributes => {
                Self::function_type_attributes.parse(self, input, kind)
            }
            NonterminalKind::HexNumberExpression => {
                Self::hex_number_expression.parse(self, input, kind)
            }
            NonterminalKind::HexStringLiteral => Self::hex_string_literal.parse(self, input, kind),
            NonterminalKind::HexStringLiterals => {
                Self::hex_string_literals.parse(self, input, kind)
            }
            NonterminalKind::IdentifierPath => Self::identifier_path.parse(self, input, kind),
            NonterminalKind::IfStatement => Self::if_statement.parse(self, input, kind),
            NonterminalKind::ImportAlias => Self::import_alias.parse(self, input, kind),
            NonterminalKind::ImportClause => Self::import_clause.parse(self, input, kind),
            NonterminalKind::ImportDeconstruction => {
                Self::import_deconstruction.parse(self, input, kind)
            }
            NonterminalKind::ImportDeconstructionSymbol => {
                Self::import_deconstruction_symbol.parse(self, input, kind)
            }
            NonterminalKind::ImportDeconstructionSymbols => {
                Self::import_deconstruction_symbols.parse(self, input, kind)
            }
            NonterminalKind::ImportDirective => Self::import_directive.parse(self, input, kind),
            NonterminalKind::IndexAccessEnd => Self::index_access_end.parse(self, input, kind),
            NonterminalKind::IndexAccessExpression => {
                Self::index_access_expression.parse(self, input, kind)
            }
            NonterminalKind::InequalityExpression => {
                Self::inequality_expression.parse(self, input, kind)
            }
            NonterminalKind::InheritanceSpecifier => {
                Self::inheritance_specifier.parse(self, input, kind)
            }
            NonterminalKind::InheritanceType => Self::inheritance_type.parse(self, input, kind),
            NonterminalKind::InheritanceTypes => Self::inheritance_types.parse(self, input, kind),
            NonterminalKind::InterfaceDefinition => {
                Self::interface_definition.parse(self, input, kind)
            }
            NonterminalKind::InterfaceMembers => Self::interface_members.parse(self, input, kind),
            NonterminalKind::LibraryDefinition => Self::library_definition.parse(self, input, kind),
            NonterminalKind::LibraryMembers => Self::library_members.parse(self, input, kind),
            NonterminalKind::MappingKey => Self::mapping_key.parse(self, input, kind),
            NonterminalKind::MappingKeyType => Self::mapping_key_type.parse(self, input, kind),
            NonterminalKind::MappingType => Self::mapping_type.parse(self, input, kind),
            NonterminalKind::MappingValue => Self::mapping_value.parse(self, input, kind),
            NonterminalKind::MemberAccessExpression => {
                Self::member_access_expression.parse(self, input, kind)
            }
            NonterminalKind::ModifierAttribute => Self::modifier_attribute.parse(self, input, kind),
            NonterminalKind::ModifierAttributes => {
                Self::modifier_attributes.parse(self, input, kind)
            }
            NonterminalKind::ModifierDefinition => {
                Self::modifier_definition.parse(self, input, kind)
            }
            NonterminalKind::ModifierInvocation => {
                Self::modifier_invocation.parse(self, input, kind)
            }
            NonterminalKind::MultiplicativeExpression => {
                Self::multiplicative_expression.parse(self, input, kind)
            }
            NonterminalKind::NamedArgument => Self::named_argument.parse(self, input, kind),
            NonterminalKind::NamedArgumentGroup => {
                Self::named_argument_group.parse(self, input, kind)
            }
            NonterminalKind::NamedArguments => Self::named_arguments.parse(self, input, kind),
            NonterminalKind::NamedArgumentsDeclaration => {
                Self::named_arguments_declaration.parse(self, input, kind)
            }
            NonterminalKind::NamedImport => Self::named_import.parse(self, input, kind),
            NonterminalKind::NewExpression => Self::new_expression.parse(self, input, kind),
            NonterminalKind::NumberUnit => Self::number_unit.parse(self, input, kind),
            NonterminalKind::OrExpression => Self::or_expression.parse(self, input, kind),
            NonterminalKind::OverridePaths => Self::override_paths.parse(self, input, kind),
            NonterminalKind::OverridePathsDeclaration => {
                Self::override_paths_declaration.parse(self, input, kind)
            }
            NonterminalKind::OverrideSpecifier => Self::override_specifier.parse(self, input, kind),
            NonterminalKind::Parameter => Self::parameter.parse(self, input, kind),
            NonterminalKind::Parameters => Self::parameters.parse(self, input, kind),
            NonterminalKind::ParametersDeclaration => {
                Self::parameters_declaration.parse(self, input, kind)
            }
            NonterminalKind::PathImport => Self::path_import.parse(self, input, kind),
            NonterminalKind::PositionalArguments => {
                Self::positional_arguments.parse(self, input, kind)
            }
            NonterminalKind::PositionalArgumentsDeclaration => {
                Self::positional_arguments_declaration.parse(self, input, kind)
            }
            NonterminalKind::PostfixExpression => Self::postfix_expression.parse(self, input, kind),
            NonterminalKind::Pragma => Self::pragma.parse(self, input, kind),
            NonterminalKind::PragmaDirective => Self::pragma_directive.parse(self, input, kind),
            NonterminalKind::PrefixExpression => Self::prefix_expression.parse(self, input, kind),
            NonterminalKind::ReceiveFunctionAttribute => {
                Self::receive_function_attribute.parse(self, input, kind)
            }
            NonterminalKind::ReceiveFunctionAttributes => {
                Self::receive_function_attributes.parse(self, input, kind)
            }
            NonterminalKind::ReceiveFunctionDefinition => {
                Self::receive_function_definition.parse(self, input, kind)
            }
            NonterminalKind::ReturnStatement => Self::return_statement.parse(self, input, kind),
            NonterminalKind::ReturnsDeclaration => {
                Self::returns_declaration.parse(self, input, kind)
            }
            NonterminalKind::RevertStatement => Self::revert_statement.parse(self, input, kind),
            NonterminalKind::ShiftExpression => Self::shift_expression.parse(self, input, kind),
            NonterminalKind::SimpleVersionLiteral => {
                Self::simple_version_literal.parse(self, input, kind)
            }
            NonterminalKind::SourceUnit => Self::source_unit.parse(self, input, kind),
            NonterminalKind::SourceUnitMember => Self::source_unit_member.parse(self, input, kind),
            NonterminalKind::SourceUnitMembers => {
                Self::source_unit_members.parse(self, input, kind)
            }
            NonterminalKind::StateVariableAttribute => {
                Self::state_variable_attribute.parse(self, input, kind)
            }
            NonterminalKind::StateVariableAttributes => {
                Self::state_variable_attributes.parse(self, input, kind)
            }
            NonterminalKind::StateVariableDefinition => {
                Self::state_variable_definition.parse(self, input, kind)
            }
            NonterminalKind::StateVariableDefinitionValue => {
                Self::state_variable_definition_value.parse(self, input, kind)
            }
            NonterminalKind::Statement => Self::statement.parse(self, input, kind),
            NonterminalKind::Statements => Self::statements.parse(self, input, kind),
            NonterminalKind::StorageLayoutSpecifier => {
                Self::storage_layout_specifier.parse(self, input, kind)
            }
            NonterminalKind::StorageLocation => Self::storage_location.parse(self, input, kind),
            NonterminalKind::StringExpression => Self::string_expression.parse(self, input, kind),
            NonterminalKind::StringLiteral => Self::string_literal.parse(self, input, kind),
            NonterminalKind::StringLiterals => Self::string_literals.parse(self, input, kind),
            NonterminalKind::StructDefinition => Self::struct_definition.parse(self, input, kind),
            NonterminalKind::StructMember => Self::struct_member.parse(self, input, kind),
            NonterminalKind::StructMembers => Self::struct_members.parse(self, input, kind),
            NonterminalKind::ThrowStatement => Self::throw_statement.parse(self, input, kind),
            NonterminalKind::TryStatement => Self::try_statement.parse(self, input, kind),
            NonterminalKind::TupleDeconstructionElement => {
                Self::tuple_deconstruction_element.parse(self, input, kind)
            }
            NonterminalKind::TupleDeconstructionElements => {
                Self::tuple_deconstruction_elements.parse(self, input, kind)
            }
            NonterminalKind::TupleDeconstructionStatement => {
                Self::tuple_deconstruction_statement.parse(self, input, kind)
            }
            NonterminalKind::TupleExpression => Self::tuple_expression.parse(self, input, kind),
            NonterminalKind::TupleMember => Self::tuple_member.parse(self, input, kind),
            NonterminalKind::TupleValue => Self::tuple_value.parse(self, input, kind),
            NonterminalKind::TupleValues => Self::tuple_values.parse(self, input, kind),
            NonterminalKind::TypeExpression => Self::type_expression.parse(self, input, kind),
            NonterminalKind::TypeName => Self::type_name.parse(self, input, kind),
            NonterminalKind::TypedTupleMember => Self::typed_tuple_member.parse(self, input, kind),
            NonterminalKind::UncheckedBlock => Self::unchecked_block.parse(self, input, kind),
            NonterminalKind::UnicodeStringLiteral => {
                Self::unicode_string_literal.parse(self, input, kind)
            }
            NonterminalKind::UnicodeStringLiterals => {
                Self::unicode_string_literals.parse(self, input, kind)
            }
            NonterminalKind::UnnamedFunctionAttribute => {
                Self::unnamed_function_attribute.parse(self, input, kind)
            }
            NonterminalKind::UnnamedFunctionAttributes => {
                Self::unnamed_function_attributes.parse(self, input, kind)
            }
            NonterminalKind::UnnamedFunctionDefinition => {
                Self::unnamed_function_definition.parse(self, input, kind)
            }
            NonterminalKind::UntypedTupleMember => {
                Self::untyped_tuple_member.parse(self, input, kind)
            }
            NonterminalKind::UserDefinedValueTypeDefinition => {
                Self::user_defined_value_type_definition.parse(self, input, kind)
            }
            NonterminalKind::UsingAlias => Self::using_alias.parse(self, input, kind),
            NonterminalKind::UsingClause => Self::using_clause.parse(self, input, kind),
            NonterminalKind::UsingDeconstruction => {
                Self::using_deconstruction.parse(self, input, kind)
            }
            NonterminalKind::UsingDeconstructionSymbol => {
                Self::using_deconstruction_symbol.parse(self, input, kind)
            }
            NonterminalKind::UsingDeconstructionSymbols => {
                Self::using_deconstruction_symbols.parse(self, input, kind)
            }
            NonterminalKind::UsingDirective => Self::using_directive.parse(self, input, kind),
            NonterminalKind::UsingOperator => Self::using_operator.parse(self, input, kind),
            NonterminalKind::UsingTarget => Self::using_target.parse(self, input, kind),
            NonterminalKind::VariableDeclarationStatement => {
                Self::variable_declaration_statement.parse(self, input, kind)
            }
            NonterminalKind::VariableDeclarationType => {
                Self::variable_declaration_type.parse(self, input, kind)
            }
            NonterminalKind::VariableDeclarationValue => {
                Self::variable_declaration_value.parse(self, input, kind)
            }
            NonterminalKind::VersionExpression => Self::version_expression.parse(self, input, kind),
            NonterminalKind::VersionExpressionSet => {
                Self::version_expression_set.parse(self, input, kind)
            }
            NonterminalKind::VersionExpressionSets => {
                Self::version_expression_sets.parse(self, input, kind)
            }
            NonterminalKind::VersionLiteral => Self::version_literal.parse(self, input, kind),
            NonterminalKind::VersionOperator => Self::version_operator.parse(self, input, kind),
            NonterminalKind::VersionPragma => Self::version_pragma.parse(self, input, kind),
            NonterminalKind::VersionRange => Self::version_range.parse(self, input, kind),
            NonterminalKind::VersionTerm => Self::version_term.parse(self, input, kind),
            NonterminalKind::WhileStatement => Self::while_statement.parse(self, input, kind),
            NonterminalKind::YulArguments => Self::yul_arguments.parse(self, input, kind),
            NonterminalKind::YulAssignmentOperator => {
                Self::yul_assignment_operator.parse(self, input, kind)
            }
            NonterminalKind::YulBlock => Self::yul_block.parse(self, input, kind),
            NonterminalKind::YulBreakStatement => {
                Self::yul_break_statement.parse(self, input, kind)
            }
            NonterminalKind::YulColonAndEqual => Self::yul_colon_and_equal.parse(self, input, kind),
            NonterminalKind::YulContinueStatement => {
                Self::yul_continue_statement.parse(self, input, kind)
            }
            NonterminalKind::YulDefaultCase => Self::yul_default_case.parse(self, input, kind),
            NonterminalKind::YulEqualAndColon => Self::yul_equal_and_colon.parse(self, input, kind),
            NonterminalKind::YulExpression => Self::yul_expression.parse(self, input, kind),
            NonterminalKind::YulForStatement => Self::yul_for_statement.parse(self, input, kind),
            NonterminalKind::YulFunctionCallExpression => {
                Self::yul_function_call_expression.parse(self, input, kind)
            }
            NonterminalKind::YulFunctionDefinition => {
                Self::yul_function_definition.parse(self, input, kind)
            }
            NonterminalKind::YulIfStatement => Self::yul_if_statement.parse(self, input, kind),
            NonterminalKind::YulLabel => Self::yul_label.parse(self, input, kind),
            NonterminalKind::YulLeaveStatement => {
                Self::yul_leave_statement.parse(self, input, kind)
            }
            NonterminalKind::YulLiteral => Self::yul_literal.parse(self, input, kind),
            NonterminalKind::YulParameters => Self::yul_parameters.parse(self, input, kind),
            NonterminalKind::YulParametersDeclaration => {
                Self::yul_parameters_declaration.parse(self, input, kind)
            }
            NonterminalKind::YulPath => Self::yul_path.parse(self, input, kind),
            NonterminalKind::YulPaths => Self::yul_paths.parse(self, input, kind),
            NonterminalKind::YulReturnsDeclaration => {
                Self::yul_returns_declaration.parse(self, input, kind)
            }
            NonterminalKind::YulStackAssignmentOperator => {
                Self::yul_stack_assignment_operator.parse(self, input, kind)
            }
            NonterminalKind::YulStackAssignmentStatement => {
                Self::yul_stack_assignment_statement.parse(self, input, kind)
            }
            NonterminalKind::YulStatement => Self::yul_statement.parse(self, input, kind),
            NonterminalKind::YulStatements => Self::yul_statements.parse(self, input, kind),
            NonterminalKind::YulSwitchCase => Self::yul_switch_case.parse(self, input, kind),
            NonterminalKind::YulSwitchCases => Self::yul_switch_cases.parse(self, input, kind),
            NonterminalKind::YulSwitchStatement => {
                Self::yul_switch_statement.parse(self, input, kind)
            }
            NonterminalKind::YulValueCase => Self::yul_value_case.parse(self, input, kind),
            NonterminalKind::YulVariableAssignmentStatement => {
                Self::yul_variable_assignment_statement.parse(self, input, kind)
            }
            NonterminalKind::YulVariableDeclarationStatement => {
                Self::yul_variable_declaration_statement.parse(self, input, kind)
            }
            NonterminalKind::YulVariableDeclarationValue => {
                Self::yul_variable_declaration_value.parse(self, input, kind)
            }
            NonterminalKind::YulVariableNames => Self::yul_variable_names.parse(self, input, kind),
        }
    }

    /********************************************
     *         Parser Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn abicoder_pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_7_5 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::AbicoderKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                        input,
                        TerminalKind::AbicoderKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Version, self.abicoder_version(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::AbicoderPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn abicoder_version(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_7_5 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TerminalKind::AbicoderV1Keyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TerminalKind::AbicoderV2Keyword,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::AbicoderVersion)
    }

    #[allow(unused_assignments, unused_parens)]
    fn additive_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::AdditiveExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn address_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::AddressKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::AddressKeyword,
                ),
            )?;
            if self.version_is_at_least_0_5_0 {
                seq.elem_labeled(
                    EdgeLabel::PayableKeyword,
                    OptionalHelper::transform(
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::PayableKeyword,
                        ),
                    ),
                )?;
            }
            seq.finish()
        })
        .with_kind(NonterminalKind::AddressType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn and_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::AndExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn arguments_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.positional_arguments_declaration(input);
            choice.consider(input, result)?;
            let result = self.named_arguments_declaration(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::ArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseBracket);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenBracket,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::OpenBracket,
                ),
            )?;
            match self.array_values(input).with_label(EdgeLabel::Items) {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::CloseBracket,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseBracket,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::CloseBracket,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ArrayExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_type_name(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.type_name(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::TypeName => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::ArrayTypeName => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_values(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.expression(input).with_label(EdgeLabel::Item),
            TerminalKind::Comma,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::ArrayValues)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_flags(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_13 {
            SeparatedHelper::run::<_, LexicalContextType::Default>(
                input,
                self,
                |input| self.string_literal(input).with_label(EdgeLabel::Item),
                TerminalKind::Comma,
                EdgeLabel::Separator,
            )
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::AssemblyFlags)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_flags_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_13 {
            SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenParen,
                    ),
                )?;
                match self.assembly_flags(input).with_label(EdgeLabel::Flags) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseParen,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseParen,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::AssemblyFlagsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::AssemblyKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::AssemblyKeyword,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Label,
                OptionalHelper::transform(self.string_literal(input)),
            )?;
            if self.version_is_at_least_0_8_13 {
                seq.elem_labeled(
                    EdgeLabel::Flags,
                    OptionalHelper::transform(self.assembly_flags_declaration(input)),
                )?;
            }
            seq.elem_labeled(EdgeLabel::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::AssemblyStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assignment_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::AssignmentExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn bitwise_and_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::BitwiseAndExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn bitwise_or_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::BitwiseOrExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn bitwise_xor_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::BitwiseXorExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn block(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenBrace,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::OpenBrace,
                ),
            )?;
            match self.statements(input).with_label(EdgeLabel::Statements) {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::CloseBrace,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseBrace,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::CloseBrace,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::Block)
    }

    #[allow(unused_assignments, unused_parens)]
    fn break_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match self
                .parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::BreakKeyword,
                )
                .with_label(EdgeLabel::BreakKeyword)
            {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::BreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn call_options(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_2 {
            SeparatedHelper::run::<_, LexicalContextType::Default>(
                input,
                self,
                |input| self.named_argument(input).with_label(EdgeLabel::Item),
                TerminalKind::Comma,
                EdgeLabel::Separator,
            )
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::CallOptions)
    }

    #[allow(unused_assignments, unused_parens)]
    fn call_options_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::CallOptionsExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::CatchKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CatchKeyword,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Error,
                    OptionalHelper::transform(self.catch_clause_error(input)),
                )?;
                seq.elem_labeled(EdgeLabel::Body, self.block(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::CatchClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause_error(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::Name,
                    OptionalHelper::transform(
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Parameters, self.parameters_declaration(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::CatchClauseError)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clauses(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                self.catch_clause(input).with_label(EdgeLabel::Item)
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::CatchClauses)
    }

    #[allow(unused_assignments, unused_parens)]
    fn conditional_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::ConditionalExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn constant_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_7_4 {
            SequenceHelper::run(|mut seq| {
                match SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(EdgeLabel::TypeName, self.type_name(input))?;
                    seq.elem_labeled(
                        EdgeLabel::ConstantKeyword,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::ConstantKeyword,
                        ),
                    )?;
                    seq.elem_labeled(
                        EdgeLabel::Name,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    )?;
                    seq.elem_labeled(
                        EdgeLabel::Equal,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Equal,
                        ),
                    )?;
                    seq.elem_labeled(EdgeLabel::Value, self.expression(input))?;
                    seq.finish()
                }) {
                    result if result.matches_at_least_n_terminals(1u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::Semicolon,
                                ),
                        )?;
                    }
                    result => {
                        seq.elem(result)?;
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::Semicolon,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ConstantDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.modifier_invocation(input);
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::InternalKeyword,
                );
                choice.consider(input, result)?;
                if self.version_is_at_least_0_6_0 && !self.version_is_at_least_0_6_7 {
                    let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OverrideKeyword,
                    );
                    choice.consider(input, result)?;
                }
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::PayableKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::PublicKeyword,
                );
                choice.consider(input, result)?;
                if self.version_is_at_least_0_6_0 && !self.version_is_at_least_0_6_7 {
                    let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::VirtualKeyword,
                    );
                    choice.consider(input, result)?;
                }
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ConstructorAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            ZeroOrMoreHelper::run(input, |input| {
                self.constructor_attribute(input)
                    .with_label(EdgeLabel::Item)
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ConstructorAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::ConstructorKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::ConstructorKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Parameters, self.parameters_declaration(input))?;
                seq.elem_labeled(EdgeLabel::Attributes, self.constructor_attributes(input))?;
                seq.elem_labeled(EdgeLabel::Body, self.block(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ConstructorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn continue_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match self
                .parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ContinueKeyword,
                )
                .with_label(EdgeLabel::ContinueKeyword)
            {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            if self.version_is_at_least_0_6_0 {
                seq.elem_labeled(
                    EdgeLabel::AbstractKeyword,
                    OptionalHelper::transform(
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::AbstractKeyword,
                        ),
                    ),
                )?;
            }
            seq.elem_labeled(
                EdgeLabel::ContractKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ContractKeyword,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Specifiers, self.contract_specifiers(input))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenBrace,
                    ),
                )?;
                match self.contract_members(input).with_label(EdgeLabel::Members) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseBrace,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ContractDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.using_directive(input);
            choice.consider(input, result)?;
            let result = self.function_definition(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_4_22 {
                let result = self.constructor_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.receive_function_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.fallback_function_definition(input);
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_6_0 {
                let result = self.unnamed_function_definition(input);
                choice.consider(input, result)?;
            }
            let result = self.modifier_definition(input);
            choice.consider(input, result)?;
            let result = self.struct_definition(input);
            choice.consider(input, result)?;
            let result = self.enum_definition(input);
            choice.consider(input, result)?;
            let result = self.event_definition(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_8_4 {
                let result = self.error_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_8 {
                let result = self.user_defined_value_type_definition(input);
                choice.consider(input, result)?;
            }
            let result = self.state_variable_definition(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::ContractMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.contract_member(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::ContractMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_specifier(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.inheritance_specifier(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_8_29 {
                let result = self.storage_layout_specifier(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::ContractSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_specifiers(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.contract_specifier(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::ContractSpecifiers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_number_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::Literal,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::DecimalLiteral,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Unit,
                OptionalHelper::transform(self.number_unit(input)),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::DecimalNumberExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn do_while_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper :: run (| mut seq | { match SequenceHelper :: run (| mut seq | { seq . elem_labeled (EdgeLabel :: DoKeyword , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: DoKeyword)) ? ; seq . elem_labeled (EdgeLabel :: Body , self . statement (input)) ? ; seq . elem_labeled (EdgeLabel :: WhileKeyword , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: WhileKeyword)) ? ; seq . elem (SequenceHelper :: run (| mut seq | { let mut delim_guard = input . open_delim (TerminalKind :: CloseParen) ; let input = delim_guard . ctx () ; seq . elem_labeled (EdgeLabel :: OpenParen , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: OpenParen)) ? ; match self . expression (input) . with_label (EdgeLabel :: Condition) { result if result . matches_at_least_n_terminals (0u8) => { seq . elem (result . recover_until_with_nested_delims :: < _ , LexicalContextType :: Default > (input , self , TerminalKind :: CloseParen ,)) ? ; } , _ => { return std :: ops :: ControlFlow :: Break (ParserResult :: no_match (vec ! [])) ; } , } seq . elem_labeled (EdgeLabel :: CloseParen , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: CloseParen)) ? ; seq . finish () })) ? ; seq . finish () }) { result if result . matches_at_least_n_terminals (1u8) => { seq . elem (result . recover_until_with_nested_delims :: < _ , LexicalContextType :: Default > (input , self , TerminalKind :: Semicolon ,)) ? ; } , result => { seq . elem (result) ? ; } , } seq . elem_labeled (EdgeLabel :: Semicolon , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: Semicolon)) ? ; seq . finish () }) . with_kind (NonterminalKind :: DoWhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn elementary_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::BoolKeyword,
            );
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_8_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ByteKeyword,
                );
                choice.consider(input, result)?;
            }
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::StringKeyword,
            );
            choice.consider(input, result)?;
            let result = self.address_type(input);
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::BytesKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::IntKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::UintKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::FixedKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::UfixedKeyword,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::ElementaryType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn else_branch(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::ElseKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ElseKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Body, self.statement(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ElseBranch)
    }

    #[allow(unused_assignments, unused_parens)]
    fn emit_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_21 {
            SequenceHelper::run(|mut seq| {
                match SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::EmitKeyword,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::EmitKeyword,
                        ),
                    )?;
                    seq.elem_labeled(EdgeLabel::Event, self.identifier_path(input))?;
                    seq.elem_labeled(EdgeLabel::Arguments, self.arguments_declaration(input))?;
                    seq.finish()
                }) {
                    result if result.matches_at_least_n_terminals(1u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::Semicolon,
                                ),
                        )?;
                    }
                    result => {
                        seq.elem(result)?;
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::Semicolon,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::EmitStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn enum_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::EnumKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::EnumKeyword,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenBrace,
                    ),
                )?;
                match self.enum_members(input).with_label(EdgeLabel::Members) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseBrace,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::EnumDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn enum_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| {
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                )
                .with_label(EdgeLabel::Item)
            },
            TerminalKind::Comma,
            EdgeLabel::Separator,
        ))
        .with_kind(NonterminalKind::EnumMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn equality_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::EqualityExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                match SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::ErrorKeyword,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::ErrorKeyword,
                        ),
                    )?;
                    seq.elem_labeled(
                        EdgeLabel::Name,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    )?;
                    seq.elem_labeled(EdgeLabel::Members, self.error_parameters_declaration(input))?;
                    seq.finish()
                }) {
                    result if result.matches_at_least_n_terminals(1u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::Semicolon,
                                ),
                        )?;
                    }
                    result => {
                        seq.elem(result)?;
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::Semicolon,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ErrorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameter(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(EdgeLabel::TypeName, self.type_name(input))?;
                seq.elem_labeled(
                    EdgeLabel::Name,
                    OptionalHelper::transform(
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ErrorParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameters(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            OptionalHelper::transform(SeparatedHelper::run::<_, LexicalContextType::Default>(
                input,
                self,
                |input| self.error_parameter(input).with_label(EdgeLabel::Item),
                TerminalKind::Comma,
                EdgeLabel::Separator,
            ))
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ErrorParameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenParen,
                    ),
                )?;
                match self
                    .error_parameters(input)
                    .with_label(EdgeLabel::Parameters)
                {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseParen,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseParen,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ErrorParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::EventKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::EventKeyword,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Name,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Identifier,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Parameters,
                    self.event_parameters_declaration(input),
                )?;
                seq.elem_labeled(
                    EdgeLabel::AnonymousKeyword,
                    OptionalHelper::transform(
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::AnonymousKeyword,
                        ),
                    ),
                )?;
                seq.finish()
            }) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::EventDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameter(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::TypeName, self.type_name(input))?;
            seq.elem_labeled(
                EdgeLabel::IndexedKeyword,
                OptionalHelper::transform(
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::IndexedKeyword,
                    ),
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                OptionalHelper::transform(
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Identifier,
                    ),
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::EventParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameters(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.event_parameter(input).with_label(EdgeLabel::Item),
            TerminalKind::Comma,
            EdgeLabel::Separator,
        ))
        .with_kind(NonterminalKind::EventParameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::OpenParen,
                ),
            )?;
            match self
                .event_parameters(input)
                .with_label(EdgeLabel::Parameters)
            {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::CloseParen,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::EventParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn experimental_feature(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_16 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TerminalKind::ABIEncoderV2Keyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TerminalKind::SMTCheckerKeyword,
                );
                choice.consider(input, result)?;
                let result = self.string_literal(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ExperimentalFeature)
    }

    #[allow(unused_assignments, unused_parens)]
    fn experimental_pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_16 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::ExperimentalKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                        input,
                        TerminalKind::ExperimentalKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Feature, self.experimental_feature(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ExperimentalPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn exponentiation_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::ExponentiationExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let parse_left_assignment_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::AssignmentExpression,
                1u8,
                1u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Equal,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::BarEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::PlusEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::MinusEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::CaretEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::SlashEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::PercentEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::AsteriskEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::AmpersandEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::LessThanLessThanEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::GreaterThanGreaterThanEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::GreaterThanGreaterThanGreaterThanEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_postfix_conditional_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonterminalKind::ConditionalExpression,
                3u8,
                SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::QuestionMark,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::QuestionMark,
                        ),
                    )?;
                    seq.elem_labeled(EdgeLabel::TrueExpression, self.expression(input))?;
                    seq.elem_labeled(
                        EdgeLabel::Colon,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Colon,
                        ),
                    )?;
                    seq.elem_labeled(EdgeLabel::FalseExpression, self.expression(input))?;
                    seq.finish()
                }),
            )
        };
        let parse_left_or_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::OrExpression,
                5u8,
                5u8 + 1,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::BarBar,
                )
                .with_label(EdgeLabel::Operator),
            )
        };
        let parse_left_and_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::AndExpression,
                7u8,
                7u8 + 1,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::AmpersandAmpersand,
                )
                .with_label(EdgeLabel::Operator),
            )
        };
        let parse_left_equality_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::EqualityExpression,
                9u8,
                9u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::EqualEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::BangEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_inequality_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::InequalityExpression,
                11u8,
                11u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::LessThan,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::GreaterThan,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::LessThanEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::GreaterThanEqual,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_bitwise_or_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::BitwiseOrExpression,
                13u8,
                13u8 + 1,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Bar,
                )
                .with_label(EdgeLabel::Operator),
            )
        };
        let parse_left_bitwise_xor_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::BitwiseXorExpression,
                15u8,
                15u8 + 1,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Caret,
                )
                .with_label(EdgeLabel::Operator),
            )
        };
        let parse_left_bitwise_and_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::BitwiseAndExpression,
                17u8,
                17u8 + 1,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Ampersand,
                )
                .with_label(EdgeLabel::Operator),
            )
        };
        let parse_left_shift_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::ShiftExpression,
                19u8,
                19u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::LessThanLessThan,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::GreaterThanGreaterThan,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::GreaterThanGreaterThanGreaterThan,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_additive_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::AdditiveExpression,
                21u8,
                21u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Plus,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Minus,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_multiplicative_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::MultiplicativeExpression,
                23u8,
                23u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Asterisk,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Slash,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Percent,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_exponentiation_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::ExponentiationExpression,
                25u8,
                25u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    if !self.version_is_at_least_0_8_0 {
                        let result = self
                            .parse_terminal_with_trivia::<LexicalContextType::Default>(
                                input,
                                TerminalKind::AsteriskAsterisk,
                            )
                            .with_label(EdgeLabel::Operator);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                }),
            )
        };
        let parse_right_exponentiation_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::ExponentiationExpression,
                27u8 + 1,
                27u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    if self.version_is_at_least_0_8_0 {
                        let result = self
                            .parse_terminal_with_trivia::<LexicalContextType::Default>(
                                input,
                                TerminalKind::AsteriskAsterisk,
                            )
                            .with_label(EdgeLabel::Operator);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                }),
            )
        };
        let parse_postfix_postfix_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonterminalKind::PostfixExpression,
                29u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::PlusPlus,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::MinusMinus,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_prefix_prefix_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_prefix_operator(
                NonterminalKind::PrefixExpression,
                31u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::PlusPlus,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::MinusMinus,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Tilde,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Bang,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Minus,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    if !self.version_is_at_least_0_5_0 {
                        let result = self
                            .parse_terminal_with_trivia::<LexicalContextType::Default>(
                                input,
                                TerminalKind::Plus,
                            )
                            .with_label(EdgeLabel::Operator);
                        choice.consider(input, result)?;
                    }
                    let result = self
                        .parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::DeleteKeyword,
                        )
                        .with_label(EdgeLabel::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_postfix_function_call_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonterminalKind::FunctionCallExpression,
                33u8,
                self.arguments_declaration(input)
                    .with_label(EdgeLabel::Arguments),
            )
        };
        let parse_postfix_call_options_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonterminalKind::CallOptionsExpression,
                35u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    if self.version_is_at_least_0_6_2 {
                        let result = SequenceHelper::run(|mut seq| {
                            let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
                            let input = delim_guard.ctx();
                            seq.elem_labeled(
                                EdgeLabel::OpenBrace,
                                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                                    input,
                                    TerminalKind::OpenBrace,
                                ),
                            )?;
                            match self.call_options(input).with_label(EdgeLabel::Options) {
                                result if result.matches_at_least_n_terminals(2u8) => {
                                    seq . elem (result . recover_until_with_nested_delims :: < _ , LexicalContextType :: Default > (input , self , TerminalKind :: CloseBrace ,)) ? ;
                                }
                                _ => {
                                    return std::ops::ControlFlow::Break(ParserResult::no_match(
                                        vec![],
                                    ));
                                }
                            }
                            seq.elem_labeled(
                                EdgeLabel::CloseBrace,
                                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                                    input,
                                    TerminalKind::CloseBrace,
                                ),
                            )?;
                            seq.finish()
                        });
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                }),
            )
        };
        let parse_postfix_member_access_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonterminalKind::MemberAccessExpression,
                37u8,
                SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::Period,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Period,
                        ),
                    )?;
                    seq.elem_labeled(
                        EdgeLabel::Member,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    )?;
                    seq.finish()
                }),
            )
        };
        let parse_postfix_index_access_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonterminalKind::IndexAccessExpression,
                39u8,
                SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TerminalKind::CloseBracket);
                    let input = delim_guard.ctx();
                    seq.elem_labeled(
                        EdgeLabel::OpenBracket,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::OpenBracket,
                        ),
                    )?;
                    match SequenceHelper::run(|mut seq| {
                        seq.elem_labeled(
                            EdgeLabel::Start,
                            OptionalHelper::transform(self.expression(input)),
                        )?;
                        seq.elem_labeled(
                            EdgeLabel::End,
                            OptionalHelper::transform(self.index_access_end(input)),
                        )?;
                        seq.finish()
                    }) {
                        result if result.matches_at_least_n_terminals(0u8) => {
                            seq . elem (result . recover_until_with_nested_delims :: < _ , LexicalContextType :: Default > (input , self , TerminalKind :: CloseBracket ,)) ? ;
                        }
                        _ => {
                            return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                        }
                    }
                    seq.elem_labeled(
                        EdgeLabel::CloseBracket,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::CloseBracket,
                        ),
                    )?;
                    seq.finish()
                }),
            )
        };
        let prefix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_prefix_prefix_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let primary_expression_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.new_expression(input);
                choice.consider(input, result)?;
                let result = self.tuple_expression(input);
                choice.consider(input, result)?;
                if self.version_is_at_least_0_5_3 {
                    let result = self.type_expression(input);
                    choice.consider(input, result)?;
                }
                let result = self.array_expression(input);
                choice.consider(input, result)?;
                let result = self.hex_number_expression(input);
                choice.consider(input, result)?;
                let result = self.decimal_number_expression(input);
                choice.consider(input, result)?;
                let result = self.string_expression(input);
                choice.consider(input, result)?;
                let result = self.elementary_type(input);
                choice.consider(input, result)?;
                if self.version_is_at_least_0_6_0 {
                    let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::PayableKeyword,
                    );
                    choice.consider(input, result)?;
                }
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ThisKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::SuperKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::TrueKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::FalseKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        };
        let postfix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_postfix_conditional_expression(input);
                choice.consider(input, result)?;
                let result = parse_postfix_postfix_expression(input);
                choice.consider(input, result)?;
                let result = parse_postfix_function_call_expression(input);
                choice.consider(input, result)?;
                let result = parse_postfix_call_options_expression(input);
                choice.consider(input, result)?;
                let result = parse_postfix_member_access_expression(input);
                choice.consider(input, result)?;
                let result = parse_postfix_index_access_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let binary_operand_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(ZeroOrMoreHelper::run(input, prefix_operator_parser))?;
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, postfix_operator_parser))?;
                seq.finish()
            })
        };
        let binary_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_left_assignment_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_or_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_and_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_equality_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_inequality_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_bitwise_or_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_bitwise_xor_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_bitwise_and_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_shift_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_additive_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_multiplicative_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_exponentiation_expression(input);
                choice.consider(input, result)?;
                let result = parse_right_exponentiation_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(binary_operand_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, |input| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(binary_operator_parser(input))?;
                        seq.elem(binary_operand_parser(input))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            NonterminalKind::Expression,
            linear_expression_parser(input),
        )
        .with_kind(NonterminalKind::Expression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match self.expression(input).with_label(EdgeLabel::Expression) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ExpressionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.modifier_invocation(input);
                choice.consider(input, result)?;
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ExternalKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::PayableKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::PureKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ViewKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::VirtualKeyword,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::FallbackFunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            ZeroOrMoreHelper::run(input, |input| {
                self.fallback_function_attribute(input)
                    .with_label(EdgeLabel::Item)
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::FallbackFunctionAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::FallbackKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::FallbackKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Parameters, self.parameters_declaration(input))?;
                seq.elem_labeled(
                    EdgeLabel::Attributes,
                    self.fallback_function_attributes(input),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Returns,
                    OptionalHelper::transform(self.returns_declaration(input)),
                )?;
                seq.elem_labeled(EdgeLabel::Body, self.function_body(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::FallbackFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn for_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::ForKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ForKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenParen,
                    ),
                )?;
                match SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::Initialization,
                        self.for_statement_initialization(input),
                    )?;
                    seq.elem_labeled(EdgeLabel::Condition, self.for_statement_condition(input))?;
                    seq.elem_labeled(
                        EdgeLabel::Iterator,
                        OptionalHelper::transform(self.expression(input)),
                    )?;
                    seq.finish()
                }) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseParen,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_labeled(EdgeLabel::Body, self.statement(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn for_statement_condition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.expression_statement(input);
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::Semicolon,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::ForStatementCondition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn for_statement_initialization(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.tuple_deconstruction_statement(input);
            choice.consider(input, result)?;
            let result = self.variable_declaration_statement(input);
            choice.consider(input, result)?;
            let result = self.expression_statement(input);
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::Semicolon,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::ForStatementInitialization)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.modifier_invocation(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_0 {
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_5_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ConstantKeyword,
                );
                choice.consider(input, result)?;
            }
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::ExternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::InternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::PayableKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::PrivateKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::PublicKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_4_16 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::PureKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_16 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ViewKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::VirtualKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::FunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.function_attribute(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::FunctionAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_body(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.block(input);
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::Semicolon,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::FunctionBody)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_call_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::FunctionCallExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::FunctionKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::FunctionKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Name, self.function_name(input))?;
            seq.elem_labeled(EdgeLabel::Parameters, self.parameters_declaration(input))?;
            seq.elem_labeled(EdgeLabel::Attributes, self.function_attributes(input))?;
            seq.elem_labeled(
                EdgeLabel::Returns,
                OptionalHelper::transform(self.returns_declaration(input)),
            )?;
            seq.elem_labeled(EdgeLabel::Body, self.function_body(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::FunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_name(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::Identifier,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::FallbackKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::ReceiveKeyword,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::FunctionName)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::FunctionKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::FunctionKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Parameters, self.parameters_declaration(input))?;
            seq.elem_labeled(EdgeLabel::Attributes, self.function_type_attributes(input))?;
            seq.elem_labeled(
                EdgeLabel::Returns,
                OptionalHelper::transform(self.returns_declaration(input)),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::FunctionType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::InternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::ExternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::PrivateKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::PublicKeyword,
            );
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ConstantKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_16 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::PureKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_16 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ViewKeyword,
                );
                choice.consider(input, result)?;
            }
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::PayableKeyword,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::FunctionTypeAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.function_type_attribute(input)
                .with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::FunctionTypeAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_number_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::Literal,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::HexLiteral,
                ),
            )?;
            if !self.version_is_at_least_0_5_0 {
                seq.elem_labeled(
                    EdgeLabel::Unit,
                    OptionalHelper::transform(self.number_unit(input)),
                )?;
            }
            seq.finish()
        })
        .with_kind(NonterminalKind::HexNumberExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::SingleQuotedHexStringLiteral,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::DoubleQuotedHexStringLiteral,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::HexStringLiteral)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literals(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_5_14 {
            OneOrMoreHelper::run(input, |input| {
                self.hex_string_literal(input).with_label(EdgeLabel::Item)
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::HexStringLiterals)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_path(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| {
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                )
                .with_label(EdgeLabel::Item)
            },
            TerminalKind::Period,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::IdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn if_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::IfKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::IfKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenParen,
                    ),
                )?;
                match self.expression(input).with_label(EdgeLabel::Condition) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseParen,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_labeled(EdgeLabel::Body, self.statement(input))?;
            seq.elem_labeled(
                EdgeLabel::ElseBranch,
                OptionalHelper::transform(self.else_branch(input)),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::IfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_alias(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::AsKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::AsKeyword,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Identifier,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ImportAlias)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_clause(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.path_import(input);
            choice.consider(input, result)?;
            let result = self.named_import(input);
            choice.consider(input, result)?;
            let result = self.import_deconstruction(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::ImportClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_deconstruction(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenBrace,
                    ),
                )?;
                match self
                    .import_deconstruction_symbols(input)
                    .with_label(EdgeLabel::Symbols)
                {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseBrace,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_labeled(
                EdgeLabel::FromKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::FromKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Path, self.string_literal(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ImportDeconstruction)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_deconstruction_symbol(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Alias,
                OptionalHelper::transform(self.import_alias(input)),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ImportDeconstructionSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_deconstruction_symbols(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| {
                self.import_deconstruction_symbol(input)
                    .with_label(EdgeLabel::Item)
            },
            TerminalKind::Comma,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::ImportDeconstructionSymbols)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_directive(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::ImportKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::ImportKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Clause, self.import_clause(input))?;
                seq.finish()
            }) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ImportDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn index_access_end(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::Colon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Colon,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::End,
                OptionalHelper::transform(self.expression(input)),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::IndexAccessEnd)
    }

    #[allow(unused_assignments, unused_parens)]
    fn index_access_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::IndexAccessExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn inequality_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::InequalityExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_specifier(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::IsKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::IsKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Types, self.inheritance_types(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::InheritanceSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::TypeName, self.identifier_path(input))?;
            seq.elem_labeled(
                EdgeLabel::Arguments,
                OptionalHelper::transform(self.arguments_declaration(input)),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::InheritanceType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_types(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.inheritance_type(input).with_label(EdgeLabel::Item),
            TerminalKind::Comma,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::InheritanceTypes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::InterfaceKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::InterfaceKeyword,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Inheritance,
                OptionalHelper::transform(self.inheritance_specifier(input)),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenBrace,
                    ),
                )?;
                match self.interface_members(input).with_label(EdgeLabel::Members) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseBrace,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::InterfaceDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.contract_member(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::InterfaceMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::LibraryKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::LibraryKeyword,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenBrace,
                    ),
                )?;
                match self.library_members(input).with_label(EdgeLabel::Members) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseBrace,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::LibraryDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.contract_member(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::LibraryMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_key(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::KeyType, self.mapping_key_type(input))?;
            if self.version_is_at_least_0_8_18 {
                seq.elem_labeled(
                    EdgeLabel::Name,
                    OptionalHelper::transform(
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    ),
                )?;
            }
            seq.finish()
        })
        .with_kind(NonterminalKind::MappingKey)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_key_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.elementary_type(input);
            choice.consider(input, result)?;
            let result = self.identifier_path(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::MappingKeyType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::MappingKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::MappingKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenParen,
                    ),
                )?;
                match SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(EdgeLabel::KeyType, self.mapping_key(input))?;
                    seq.elem_labeled(
                        EdgeLabel::EqualGreaterThan,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::EqualGreaterThan,
                        ),
                    )?;
                    seq.elem_labeled(EdgeLabel::ValueType, self.mapping_value(input))?;
                    seq.finish()
                }) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseParen,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::MappingType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::TypeName, self.type_name(input))?;
            if self.version_is_at_least_0_8_18 {
                seq.elem_labeled(
                    EdgeLabel::Name,
                    OptionalHelper::transform(
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    ),
                )?;
            }
            seq.finish()
        })
        .with_kind(NonterminalKind::MappingValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn member_access_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::MemberAccessExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            if self.version_is_at_least_0_6_0 {
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::VirtualKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::ModifierAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.modifier_attribute(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::ModifierAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::ModifierKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ModifierKeyword,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Parameters,
                OptionalHelper::transform(self.parameters_declaration(input)),
            )?;
            seq.elem_labeled(EdgeLabel::Attributes, self.modifier_attributes(input))?;
            seq.elem_labeled(EdgeLabel::Body, self.function_body(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ModifierDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_invocation(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::Name, self.identifier_path(input))?;
            seq.elem_labeled(
                EdgeLabel::Arguments,
                OptionalHelper::transform(self.arguments_declaration(input)),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ModifierInvocation)
    }

    #[allow(unused_assignments, unused_parens)]
    fn multiplicative_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::MultiplicativeExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_argument(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Colon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Colon,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Value, self.expression(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::NamedArgument)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_argument_group(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenBrace,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::OpenBrace,
                ),
            )?;
            match self.named_arguments(input).with_label(EdgeLabel::Arguments) {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::CloseBrace,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseBrace,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::CloseBrace,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::NamedArgumentGroup)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.named_argument(input).with_label(EdgeLabel::Item),
            TerminalKind::Comma,
            EdgeLabel::Separator,
        ))
        .with_kind(NonterminalKind::NamedArguments)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::OpenParen,
                ),
            )?;
            match OptionalHelper::transform(self.named_argument_group(input))
                .with_label(EdgeLabel::Arguments)
            {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::CloseParen,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::NamedArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_import(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::Asterisk,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Asterisk,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Alias, self.import_alias(input))?;
            seq.elem_labeled(
                EdgeLabel::FromKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::FromKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Path, self.string_literal(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::NamedImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn new_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::NewKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::NewKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::TypeName, self.type_name(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::NewExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn number_unit(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::WeiKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_11 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::GweiKeyword,
                );
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_7_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::SzaboKeyword,
                );
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_7_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::FinneyKeyword,
                );
                choice.consider(input, result)?;
            }
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::EtherKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::SecondsKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::MinutesKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::HoursKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::DaysKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::WeeksKeyword,
            );
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::YearsKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::NumberUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn or_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::OrExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_paths(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SeparatedHelper::run::<_, LexicalContextType::Default>(
                input,
                self,
                |input| self.identifier_path(input).with_label(EdgeLabel::Item),
                TerminalKind::Comma,
                EdgeLabel::Separator,
            )
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::OverridePaths)
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_paths_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenParen,
                    ),
                )?;
                match self.override_paths(input).with_label(EdgeLabel::Paths) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseParen,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseParen,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::OverridePathsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_specifier(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::OverrideKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OverrideKeyword,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Overridden,
                    OptionalHelper::transform(self.override_paths_declaration(input)),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::OverrideSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameter(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::TypeName, self.type_name(input))?;
            seq.elem_labeled(
                EdgeLabel::StorageLocation,
                OptionalHelper::transform(self.storage_location(input)),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                OptionalHelper::transform(
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Identifier,
                    ),
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::Parameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.parameter(input).with_label(EdgeLabel::Item),
            TerminalKind::Comma,
            EdgeLabel::Separator,
        ))
        .with_kind(NonterminalKind::Parameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::OpenParen,
                ),
            )?;
            match self.parameters(input).with_label(EdgeLabel::Parameters) {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::CloseParen,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn path_import(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::Path, self.string_literal(input))?;
            seq.elem_labeled(
                EdgeLabel::Alias,
                OptionalHelper::transform(self.import_alias(input)),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::PathImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn positional_arguments(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.expression(input).with_label(EdgeLabel::Item),
            TerminalKind::Comma,
            EdgeLabel::Separator,
        ))
        .with_kind(NonterminalKind::PositionalArguments)
    }

    #[allow(unused_assignments, unused_parens)]
    fn positional_arguments_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::OpenParen,
                ),
            )?;
            match self
                .positional_arguments(input)
                .with_label(EdgeLabel::Arguments)
            {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::CloseParen,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::PositionalArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn postfix_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::PostfixExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.version_pragma(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_7_5 {
                let result = self.abicoder_pragma(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_16 {
                let result = self.experimental_pragma(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::Pragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn pragma_directive(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::PragmaKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                        input,
                        TerminalKind::PragmaKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Pragma, self.pragma(input))?;
                seq.finish()
            }) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Pragma>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::PragmaDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn prefix_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::PrefixExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.modifier_invocation(input);
                choice.consider(input, result)?;
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ExternalKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::PayableKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::VirtualKeyword,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ReceiveFunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            ZeroOrMoreHelper::run(input, |input| {
                self.receive_function_attribute(input)
                    .with_label(EdgeLabel::Item)
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ReceiveFunctionAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::ReceiveKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::ReceiveKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Parameters, self.parameters_declaration(input))?;
                seq.elem_labeled(
                    EdgeLabel::Attributes,
                    self.receive_function_attributes(input),
                )?;
                seq.elem_labeled(EdgeLabel::Body, self.function_body(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ReceiveFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn return_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::ReturnKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::ReturnKeyword,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Expression,
                    OptionalHelper::transform(self.expression(input)),
                )?;
                seq.finish()
            }) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ReturnStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn returns_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::ReturnsKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ReturnsKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Variables, self.parameters_declaration(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::ReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn revert_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                match SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::RevertKeyword,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::RevertKeyword,
                        ),
                    )?;
                    seq.elem_labeled(EdgeLabel::Error, self.identifier_path(input))?;
                    seq.elem_labeled(EdgeLabel::Arguments, self.arguments_declaration(input))?;
                    seq.finish()
                }) {
                    result if result.matches_at_least_n_terminals(1u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::Semicolon,
                                ),
                        )?;
                    }
                    result => {
                        seq.elem(result)?;
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::Semicolon,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::RevertStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn shift_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::ShiftExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn simple_version_literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Pragma>(
            input,
            self,
            |input| {
                self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TerminalKind::VersionSpecifier,
                )
                .with_label(EdgeLabel::Item)
            },
            TerminalKind::Period,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::SimpleVersionLiteral)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit(&self, input: &mut ParserContext<'_>) -> ParserResult {
        self.source_unit_members(input)
            .with_label(EdgeLabel::Members)
            .with_kind(NonterminalKind::SourceUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.pragma_directive(input);
            choice.consider(input, result)?;
            let result = self.import_directive(input);
            choice.consider(input, result)?;
            let result = self.contract_definition(input);
            choice.consider(input, result)?;
            let result = self.interface_definition(input);
            choice.consider(input, result)?;
            let result = self.library_definition(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_0 {
                let result = self.struct_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.enum_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_7_1 {
                let result = self.function_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_4 {
                let result = self.error_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_8 {
                let result = self.user_defined_value_type_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_13 {
                let result = self.using_directive(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_22 {
                let result = self.event_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_7_4 {
                let result = self.constant_definition(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::SourceUnitMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.source_unit_member(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::SourceUnitMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            if self.version_is_at_least_0_6_0 {
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
            }
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::ConstantKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::InternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::PrivateKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::PublicKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_5 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ImmutableKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_27 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::TransientKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::StateVariableAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.state_variable_attribute(input)
                .with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::StateVariableAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match SequenceHelper::run(|mut seq| {
                seq.elem_labeled(EdgeLabel::TypeName, self.type_name(input))?;
                seq.elem_labeled(EdgeLabel::Attributes, self.state_variable_attributes(input))?;
                seq.elem_labeled(
                    EdgeLabel::Name,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Identifier,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Value,
                    OptionalHelper::transform(self.state_variable_definition_value(input)),
                )?;
                seq.finish()
            }) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::StateVariableDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_definition_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::Equal,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Equal,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Value, self.expression(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::StateVariableDefinitionValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.if_statement(input);
            choice.consider(input, result)?;
            let result = self.for_statement(input);
            choice.consider(input, result)?;
            let result = self.while_statement(input);
            choice.consider(input, result)?;
            let result = self.do_while_statement(input);
            choice.consider(input, result)?;
            let result = self.continue_statement(input);
            choice.consider(input, result)?;
            let result = self.break_statement(input);
            choice.consider(input, result)?;
            let result = self.return_statement(input);
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.throw_statement(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_21 {
                let result = self.emit_statement(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.try_statement(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_4 {
                let result = self.revert_statement(input);
                choice.consider(input, result)?;
            }
            let result = self.assembly_statement(input);
            choice.consider(input, result)?;
            let result = self.block(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_8_0 {
                let result = self.unchecked_block(input);
                choice.consider(input, result)?;
            }
            let result = self.tuple_deconstruction_statement(input);
            choice.consider(input, result)?;
            let result = self.variable_declaration_statement(input);
            choice.consider(input, result)?;
            let result = self.expression_statement(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::Statement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statements(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.statement(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::Statements)
    }

    #[allow(unused_assignments, unused_parens)]
    fn storage_layout_specifier(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_29 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::LayoutKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::LayoutKeyword,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::AtKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::AtKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Expression, self.expression(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::StorageLayoutSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn storage_location(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::MemoryKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::StorageKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_5_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::CallDataKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::StorageLocation)
    }

    #[allow(unused_assignments, unused_parens)]
    fn string_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            if !self.version_is_at_least_0_5_14 {
                let result = self.string_literal(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_5_14 {
                let result = self.string_literals(input);
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_5_14 {
                let result = self.hex_string_literal(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_5_14 {
                let result = self.hex_string_literals(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_7_0 {
                let result = self.unicode_string_literals(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::StringExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn string_literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::SingleQuotedStringLiteral,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::DoubleQuotedStringLiteral,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::StringLiteral)
    }

    #[allow(unused_assignments, unused_parens)]
    fn string_literals(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_5_14 {
            OneOrMoreHelper::run(input, |input| {
                self.string_literal(input).with_label(EdgeLabel::Item)
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::StringLiterals)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::StructKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::StructKeyword,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenBrace,
                    ),
                )?;
                match self.struct_members(input).with_label(EdgeLabel::Members) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseBrace,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::StructDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match SequenceHelper::run(|mut seq| {
                seq.elem_labeled(EdgeLabel::TypeName, self.type_name(input))?;
                seq.elem_labeled(
                    EdgeLabel::Name,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Identifier,
                    ),
                )?;
                seq.finish()
            }) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::StructMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.struct_member(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::StructMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn throw_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            SequenceHelper::run(|mut seq| {
                match self
                    .parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::ThrowKeyword,
                    )
                    .with_label(EdgeLabel::ThrowKeyword)
                {
                    result if result.matches_at_least_n_terminals(1u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::Semicolon,
                                ),
                        )?;
                    }
                    result => {
                        seq.elem(result)?;
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::Semicolon,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::ThrowStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn try_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::TryKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::TryKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Expression, self.expression(input))?;
                seq.elem_labeled(
                    EdgeLabel::Returns,
                    OptionalHelper::transform(self.returns_declaration(input)),
                )?;
                seq.elem_labeled(EdgeLabel::Body, self.block(input))?;
                seq.elem_labeled(EdgeLabel::CatchClauses, self.catch_clauses(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::TryStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_element(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(self.tuple_member(input))
            .with_label(EdgeLabel::Member)
            .with_kind(NonterminalKind::TupleDeconstructionElement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_elements(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| {
                self.tuple_deconstruction_element(input)
                    .with_label(EdgeLabel::Item)
            },
            TerminalKind::Comma,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::TupleDeconstructionElements)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper :: run (| mut seq | { match SequenceHelper :: run (| mut seq | { if ! self . version_is_at_least_0_5_0 { seq . elem_labeled (EdgeLabel :: VarKeyword , OptionalHelper :: transform (self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: VarKeyword))) ? ; } seq . elem (SequenceHelper :: run (| mut seq | { let mut delim_guard = input . open_delim (TerminalKind :: CloseParen) ; let input = delim_guard . ctx () ; seq . elem_labeled (EdgeLabel :: OpenParen , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: OpenParen)) ? ; match self . tuple_deconstruction_elements (input) . with_label (EdgeLabel :: Elements) { result if result . matches_at_least_n_terminals (0u8) => { seq . elem (result . recover_until_with_nested_delims :: < _ , LexicalContextType :: Default > (input , self , TerminalKind :: CloseParen ,)) ? ; } , _ => { return std :: ops :: ControlFlow :: Break (ParserResult :: no_match (vec ! [])) ; } , } seq . elem_labeled (EdgeLabel :: CloseParen , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: CloseParen)) ? ; seq . finish () })) ? ; seq . elem_labeled (EdgeLabel :: Equal , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: Equal)) ? ; seq . elem_labeled (EdgeLabel :: Expression , self . expression (input)) ? ; seq . finish () }) { result if result . matches_at_least_n_terminals (1u8) => { seq . elem (result . recover_until_with_nested_delims :: < _ , LexicalContextType :: Default > (input , self , TerminalKind :: Semicolon ,)) ? ; } , result => { seq . elem (result) ? ; } , } seq . elem_labeled (EdgeLabel :: Semicolon , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: Semicolon)) ? ; seq . finish () }) . with_kind (NonterminalKind :: TupleDeconstructionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::OpenParen,
                ),
            )?;
            match self.tuple_values(input).with_label(EdgeLabel::Items) {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::CloseParen,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::TupleExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.typed_tuple_member(input);
            choice.consider(input, result)?;
            let result = self.untyped_tuple_member(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::TupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(self.expression(input))
            .with_label(EdgeLabel::Expression)
            .with_kind(NonterminalKind::TupleValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_values(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.tuple_value(input).with_label(EdgeLabel::Item),
            TerminalKind::Comma,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::TupleValues)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self . version_is_at_least_0_5_3 { SequenceHelper :: run (| mut seq | { seq . elem_labeled (EdgeLabel :: TypeKeyword , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: TypeKeyword)) ? ; seq . elem (SequenceHelper :: run (| mut seq | { let mut delim_guard = input . open_delim (TerminalKind :: CloseParen) ; let input = delim_guard . ctx () ; seq . elem_labeled (EdgeLabel :: OpenParen , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: OpenParen)) ? ; match self . type_name (input) . with_label (EdgeLabel :: TypeName) { result if result . matches_at_least_n_terminals (0u8) => { seq . elem (result . recover_until_with_nested_delims :: < _ , LexicalContextType :: Default > (input , self , TerminalKind :: CloseParen ,)) ? ; } , _ => { return std :: ops :: ControlFlow :: Break (ParserResult :: no_match (vec ! [])) ; } , } seq . elem_labeled (EdgeLabel :: CloseParen , self . parse_terminal_with_trivia :: < LexicalContextType :: Default > (input , TerminalKind :: CloseParen)) ? ; seq . finish () })) ? ; seq . finish () }) } else { ParserResult :: no_match (vec ! []) } . with_kind (NonterminalKind :: TypeExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_name(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let parse_postfix_array_type_name = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonterminalKind::ArrayTypeName,
                1u8,
                SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TerminalKind::CloseBracket);
                    let input = delim_guard.ctx();
                    seq.elem_labeled(
                        EdgeLabel::OpenBracket,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::OpenBracket,
                        ),
                    )?;
                    match OptionalHelper::transform(self.expression(input))
                        .with_label(EdgeLabel::Index)
                    {
                        result if result.matches_at_least_n_terminals(0u8) => {
                            seq . elem (result . recover_until_with_nested_delims :: < _ , LexicalContextType :: Default > (input , self , TerminalKind :: CloseBracket ,)) ? ;
                        }
                        _ => {
                            return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                        }
                    }
                    seq.elem_labeled(
                        EdgeLabel::CloseBracket,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::CloseBracket,
                        ),
                    )?;
                    seq.finish()
                }),
            )
        };
        let primary_expression_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.function_type(input);
                choice.consider(input, result)?;
                let result = self.mapping_type(input);
                choice.consider(input, result)?;
                let result = self.elementary_type(input);
                choice.consider(input, result)?;
                let result = self.identifier_path(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        };
        let postfix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_postfix_array_type_name(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, postfix_operator_parser))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            NonterminalKind::TypeName,
            linear_expression_parser(input),
        )
        .with_kind(NonterminalKind::TypeName)
    }

    #[allow(unused_assignments, unused_parens)]
    fn typed_tuple_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::TypeName, self.type_name(input))?;
            seq.elem_labeled(
                EdgeLabel::StorageLocation,
                OptionalHelper::transform(self.storage_location(input)),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::TypedTupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unchecked_block(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::UncheckedKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::UncheckedKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Block, self.block(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UncheckedBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_string_literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_7_0 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::SingleQuotedUnicodeStringLiteral,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::DoubleQuotedUnicodeStringLiteral,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UnicodeStringLiteral)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_string_literals(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_7_0 {
            OneOrMoreHelper::run(input, |input| {
                self.unicode_string_literal(input)
                    .with_label(EdgeLabel::Item)
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UnicodeStringLiterals)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.modifier_invocation(input);
                choice.consider(input, result)?;
                if !self.version_is_at_least_0_5_0 {
                    let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::ConstantKeyword,
                    );
                    choice.consider(input, result)?;
                }
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::ExternalKeyword,
                );
                choice.consider(input, result)?;
                if !self.version_is_at_least_0_5_0 {
                    let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::InternalKeyword,
                    );
                    choice.consider(input, result)?;
                }
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::PayableKeyword,
                );
                choice.consider(input, result)?;
                if !self.version_is_at_least_0_5_0 {
                    let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::PrivateKeyword,
                    );
                    choice.consider(input, result)?;
                }
                if !self.version_is_at_least_0_5_0 {
                    let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::PublicKeyword,
                    );
                    choice.consider(input, result)?;
                }
                if self.version_is_at_least_0_4_16 && !self.version_is_at_least_0_6_0 {
                    let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::PureKeyword,
                    );
                    choice.consider(input, result)?;
                }
                if self.version_is_at_least_0_4_16 && !self.version_is_at_least_0_6_0 {
                    let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::ViewKeyword,
                    );
                    choice.consider(input, result)?;
                }
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UnnamedFunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            ZeroOrMoreHelper::run(input, |input| {
                self.unnamed_function_attribute(input)
                    .with_label(EdgeLabel::Item)
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UnnamedFunctionAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::FunctionKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::FunctionKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Parameters, self.parameters_declaration(input))?;
                seq.elem_labeled(
                    EdgeLabel::Attributes,
                    self.unnamed_function_attributes(input),
                )?;
                seq.elem_labeled(EdgeLabel::Body, self.function_body(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UnnamedFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn untyped_tuple_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::StorageLocation,
                OptionalHelper::transform(self.storage_location(input)),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::UntypedTupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn user_defined_value_type_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_8 {
            SequenceHelper::run(|mut seq| {
                match SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::TypeKeyword,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::TypeKeyword,
                        ),
                    )?;
                    seq.elem_labeled(
                        EdgeLabel::Name,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    )?;
                    seq.elem_labeled(
                        EdgeLabel::IsKeyword,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::IsKeyword,
                        ),
                    )?;
                    seq.elem_labeled(EdgeLabel::ValueType, self.elementary_type(input))?;
                    seq.finish()
                }) {
                    result if result.matches_at_least_n_terminals(1u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::Semicolon,
                                ),
                        )?;
                    }
                    result => {
                        seq.elem(result)?;
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::Semicolon,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UserDefinedValueTypeDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_alias(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_19 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::AsKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::AsKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Operator, self.using_operator(input))?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UsingAlias)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_clause(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.identifier_path(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_8_13 {
                let result = self.using_deconstruction(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::UsingClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_deconstruction(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_13 {
            SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenBrace,
                    ),
                )?;
                match self
                    .using_deconstruction_symbols(input)
                    .with_label(EdgeLabel::Symbols)
                {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseBrace,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseBrace,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UsingDeconstruction)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_deconstruction_symbol(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_13 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(EdgeLabel::Name, self.identifier_path(input))?;
                if self.version_is_at_least_0_8_19 {
                    seq.elem_labeled(
                        EdgeLabel::Alias,
                        OptionalHelper::transform(self.using_alias(input)),
                    )?;
                }
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UsingDeconstructionSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_deconstruction_symbols(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_13 {
            SeparatedHelper::run::<_, LexicalContextType::Default>(
                input,
                self,
                |input| {
                    self.using_deconstruction_symbol(input)
                        .with_label(EdgeLabel::Item)
                },
                TerminalKind::Comma,
                EdgeLabel::Separator,
            )
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UsingDeconstructionSymbols)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::UsingKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::UsingKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Clause, self.using_clause(input))?;
                seq.elem_labeled(
                    EdgeLabel::ForKeyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::ForKeyword,
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Target, self.using_target(input))?;
                if self.version_is_at_least_0_8_13 {
                    seq.elem_labeled(
                        EdgeLabel::GlobalKeyword,
                        OptionalHelper::transform(
                            self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                                input,
                                TerminalKind::GlobalKeyword,
                            ),
                        ),
                    )?;
                }
                seq.finish()
            }) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::UsingDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_operator(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_19 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Ampersand,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Asterisk,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::BangEqual,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Bar,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Caret,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::EqualEqual,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::GreaterThan,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::GreaterThanEqual,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::LessThan,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::LessThanEqual,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Minus,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Percent,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Plus,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Slash,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Tilde,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::UsingOperator)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_target(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.type_name(input);
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::Asterisk,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::UsingTarget)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::VariableType,
                    self.variable_declaration_type(input),
                )?;
                seq.elem_labeled(
                    EdgeLabel::StorageLocation,
                    OptionalHelper::transform(self.storage_location(input)),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Name,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Identifier,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Value,
                    OptionalHelper::transform(self.variable_declaration_value(input)),
                )?;
                seq.finish()
            }) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::VariableDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.type_name(input);
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::VarKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::VariableDeclarationType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::Equal,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Equal,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Expression, self.expression(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::VariableDeclarationValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.version_range(input);
            choice.consider(input, result)?;
            let result = self.version_term(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::VersionExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_expression_set(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.version_expression(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::VersionExpressionSet)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_expression_sets(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Pragma>(
            input,
            self,
            |input| {
                self.version_expression_set(input)
                    .with_label(EdgeLabel::Item)
            },
            TerminalKind::BarBar,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::VersionExpressionSets)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.simple_version_literal(input);
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                input,
                TerminalKind::SingleQuotedVersionLiteral,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                input,
                TerminalKind::DoubleQuotedVersionLiteral,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::VersionLiteral)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_operator(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                input,
                TerminalKind::Caret,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                input,
                TerminalKind::Tilde,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                input,
                TerminalKind::Equal,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                input,
                TerminalKind::LessThan,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                input,
                TerminalKind::GreaterThan,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                input,
                TerminalKind::LessThanEqual,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                input,
                TerminalKind::GreaterThanEqual,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::VersionOperator)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::SolidityKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TerminalKind::SolidityKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Sets, self.version_expression_sets(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::VersionPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_range(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::Start, self.version_literal(input))?;
            seq.elem_labeled(
                EdgeLabel::Minus,
                self.parse_terminal_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TerminalKind::Minus,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::End, self.version_literal(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::VersionRange)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_term(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::Operator,
                OptionalHelper::transform(self.version_operator(input)),
            )?;
            seq.elem_labeled(EdgeLabel::Literal, self.version_literal(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::VersionTerm)
    }

    #[allow(unused_assignments, unused_parens)]
    fn while_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::WhileKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::WhileKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_labeled(
                    EdgeLabel::OpenParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::OpenParen,
                    ),
                )?;
                match self.expression(input).with_label(EdgeLabel::Condition) {
                    result if result.matches_at_least_n_terminals(0u8) => {
                        seq.elem(
                            result
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TerminalKind::CloseParen,
                                ),
                        )?;
                    }
                    _ => {
                        return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                    }
                }
                seq.elem_labeled(
                    EdgeLabel::CloseParen,
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_labeled(EdgeLabel::Body, self.statement(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::WhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_arguments(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| self.yul_expression(input).with_label(EdgeLabel::Item),
            TerminalKind::Comma,
            EdgeLabel::Separator,
        ))
        .with_kind(NonterminalKind::YulArguments)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_assignment_operator(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                input,
                TerminalKind::ColonEqual,
            );
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_5 {
                let result = self.yul_colon_and_equal(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::YulAssignmentOperator)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_block(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenBrace,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::OpenBrace,
                ),
            )?;
            match self.yul_statements(input).with_label(EdgeLabel::Statements) {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Yul>(
                            input,
                            self,
                            TerminalKind::CloseBrace,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseBrace,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::CloseBrace,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_break_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
            input,
            TerminalKind::YulBreakKeyword,
        )
        .with_label(EdgeLabel::BreakKeyword)
        .with_kind(NonterminalKind::YulBreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_colon_and_equal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_5_5 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::Colon,
                    self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                        input,
                        TerminalKind::Colon,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Equal,
                    self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                        input,
                        TerminalKind::Equal,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::YulColonAndEqual)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_continue_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
            input,
            TerminalKind::YulContinueKeyword,
        )
        .with_label(EdgeLabel::ContinueKeyword)
        .with_kind(NonterminalKind::YulContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_default_case(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::DefaultKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulDefaultKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulDefaultCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_equal_and_colon(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::Equal,
                    self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                        input,
                        TerminalKind::Equal,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Colon,
                    self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                        input,
                        TerminalKind::Colon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::YulEqualAndColon)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let parse_postfix_yul_function_call_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonterminalKind::YulFunctionCallExpression,
                1u8,
                SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
                    let input = delim_guard.ctx();
                    seq.elem_labeled(
                        EdgeLabel::OpenParen,
                        self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                            input,
                            TerminalKind::OpenParen,
                        ),
                    )?;
                    match self.yul_arguments(input).with_label(EdgeLabel::Arguments) {
                        result if result.matches_at_least_n_terminals(0u8) => {
                            seq.elem(
                                result
                                    .recover_until_with_nested_delims::<_, LexicalContextType::Yul>(
                                        input,
                                        self,
                                        TerminalKind::CloseParen,
                                    ),
                            )?;
                        }
                        _ => {
                            return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                        }
                    }
                    seq.elem_labeled(
                        EdgeLabel::CloseParen,
                        self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                            input,
                            TerminalKind::CloseParen,
                        ),
                    )?;
                    seq.finish()
                }),
            )
        };
        let primary_expression_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.yul_literal(input);
                choice.consider(input, result)?;
                let result = self.yul_path(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        };
        let postfix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_postfix_yul_function_call_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, postfix_operator_parser))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            NonterminalKind::YulExpression,
            linear_expression_parser(input),
        )
        .with_kind(NonterminalKind::YulExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_for_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::ForKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulForKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Initialization, self.yul_block(input))?;
            seq.elem_labeled(EdgeLabel::Condition, self.yul_expression(input))?;
            seq.elem_labeled(EdgeLabel::Iterator, self.yul_block(input))?;
            seq.elem_labeled(EdgeLabel::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_function_call_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.yul_expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::YulExpression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::YulFunctionCallExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::FunctionKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulFunctionKeyword,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Name,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulIdentifier,
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Parameters,
                self.yul_parameters_declaration(input),
            )?;
            seq.elem_labeled(
                EdgeLabel::Returns,
                OptionalHelper::transform(self.yul_returns_declaration(input)),
            )?;
            seq.elem_labeled(EdgeLabel::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_if_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::IfKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulIfKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Condition, self.yul_expression(input))?;
            seq.elem_labeled(EdgeLabel::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulIfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_label(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::Label,
                    self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                        input,
                        TerminalKind::YulIdentifier,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Colon,
                    self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                        input,
                        TerminalKind::Colon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::YulLabel)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_leave_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                input,
                TerminalKind::YulLeaveKeyword,
            )
            .with_label(EdgeLabel::LeaveKeyword)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::YulLeaveStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            if self.version_is_at_least_0_6_2 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulTrueKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_2 {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulFalseKeyword,
                );
                choice.consider(input, result)?;
            }
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                input,
                TerminalKind::YulDecimalLiteral,
            );
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                input,
                TerminalKind::YulHexLiteral,
            );
            choice.consider(input, result)?;
            let result = self.hex_string_literal(input);
            choice.consider(input, result)?;
            let result = self.string_literal(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::YulLiteral)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_parameters(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| {
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulIdentifier,
                )
                .with_label(EdgeLabel::Item)
            },
            TerminalKind::Comma,
            EdgeLabel::Separator,
        ))
        .with_kind(NonterminalKind::YulParameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::OpenParen,
                ),
            )?;
            match self.yul_parameters(input).with_label(EdgeLabel::Parameters) {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Yul>(
                            input,
                            self,
                            TerminalKind::CloseParen,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseParen,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_path(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| {
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulIdentifier,
                )
                .with_label(EdgeLabel::Item)
            },
            TerminalKind::Period,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::YulPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_paths(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| self.yul_path(input).with_label(EdgeLabel::Item),
            TerminalKind::Comma,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::YulPaths)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_returns_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::MinusGreaterThan,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::MinusGreaterThan,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Variables, self.yul_variable_names(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_stack_assignment_operator(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::EqualColon,
                );
                choice.consider(input, result)?;
                let result = self.yul_equal_and_colon(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::YulStackAssignmentOperator)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_stack_assignment_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::Assignment,
                    self.yul_stack_assignment_operator(input),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Variable,
                    self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                        input,
                        TerminalKind::YulIdentifier,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::YulStackAssignmentStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.yul_block(input);
            choice.consider(input, result)?;
            let result = self.yul_function_definition(input);
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.yul_stack_assignment_statement(input);
                choice.consider(input, result)?;
            }
            let result = self.yul_if_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_for_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_switch_statement(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_0 {
                let result = self.yul_leave_statement(input);
                choice.consider(input, result)?;
            }
            let result = self.yul_break_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_continue_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_variable_assignment_statement(input);
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.yul_label(input);
                choice.consider(input, result)?;
            }
            let result = self.yul_variable_declaration_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_expression(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::YulStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statements(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ZeroOrMoreHelper::run(input, |input| {
            self.yul_statement(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::YulStatements)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_case(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.yul_default_case(input);
            choice.consider(input, result)?;
            let result = self.yul_value_case(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::YulSwitchCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_cases(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.yul_switch_case(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::YulSwitchCases)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::SwitchKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulSwitchKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Expression, self.yul_expression(input))?;
            seq.elem_labeled(EdgeLabel::Cases, self.yul_switch_cases(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulSwitchStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_value_case(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::CaseKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulCaseKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Value, self.yul_literal(input))?;
            seq.elem_labeled(EdgeLabel::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulValueCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_variable_assignment_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::Variables, self.yul_paths(input))?;
            seq.elem_labeled(EdgeLabel::Assignment, self.yul_assignment_operator(input))?;
            seq.elem_labeled(EdgeLabel::Expression, self.yul_expression(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulVariableAssignmentStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_variable_declaration_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(
                EdgeLabel::LetKeyword,
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulLetKeyword,
                ),
            )?;
            seq.elem_labeled(EdgeLabel::Variables, self.yul_variable_names(input))?;
            seq.elem_labeled(
                EdgeLabel::Value,
                OptionalHelper::transform(self.yul_variable_declaration_value(input)),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulVariableDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_variable_declaration_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_labeled(EdgeLabel::Assignment, self.yul_assignment_operator(input))?;
            seq.elem_labeled(EdgeLabel::Expression, self.yul_expression(input))?;
            seq.finish()
        })
        .with_kind(NonterminalKind::YulVariableDeclarationValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_variable_names(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| {
                self.parse_terminal_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TerminalKind::YulIdentifier,
                )
                .with_label(EdgeLabel::Item)
            },
            TerminalKind::Comma,
            EdgeLabel::Separator,
        )
        .with_kind(NonterminalKind::YulVariableNames)
    }

    #[allow(unused_assignments, unused_parens)]
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(input, TerminalKind::Whitespace)
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(input, TerminalKind::EndOfLine)
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(
                        input,
                        TerminalKind::SingleLineComment,
                    )
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(
                        input,
                        TerminalKind::MultiLineComment,
                    )
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(
                        input,
                        TerminalKind::SingleLineNatSpecComment,
                    )
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(
                        input,
                        TerminalKind::MultiLineNatSpecComment,
                    )
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        })
    }

    #[allow(unused_assignments, unused_parens)]
    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(OptionalHelper::transform(
                self.parse_terminal::<LexicalContextType::Default>(input, TerminalKind::Whitespace)
                    .with_label(EdgeLabel::TrailingTrivia),
            ))?;
            seq.elem(OptionalHelper::transform(
                self.parse_terminal::<LexicalContextType::Default>(
                    input,
                    TerminalKind::SingleLineComment,
                )
                .with_label(EdgeLabel::TrailingTrivia),
            ))?;
            seq.elem(
                self.parse_terminal::<LexicalContextType::Default>(input, TerminalKind::EndOfLine)
                    .with_label(EdgeLabel::TrailingTrivia),
            )?;
            seq.finish()
        })
    }

    /********************************************
     *         Scanner Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn ascii_escape(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, 't'),
            scan_chars!(input, 'r'),
            scan_chars!(input, 'n'),
            scan_chars!(input, '\\'),
            scan_chars!(input, '\''),
            scan_chars!(input, '"'),
            scan_chars!(input, '\r', '\n'),
            scan_chars!(input, '\r'),
            scan_chars!(input, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_digits(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_one_or_more!(input, scan_char_range!(input, '0'..='9')),
            scan_zero_or_more!(
                input,
                scan_sequence!(
                    scan_chars!(input, '_'),
                    scan_one_or_more!(input, scan_char_range!(input, '0'..='9'))
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_exponent(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_choice!(input, scan_chars!(input, 'e'), scan_chars!(input, 'E')),
            scan_optional!(input, scan_chars!(input, '-')),
            self.decimal_digits(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_not_followed_by!(
                input,
                scan_sequence!(
                    scan_chars!(input, '.'),
                    self.decimal_digits(input),
                    scan_optional!(input, self.decimal_exponent(input))
                ),
                self.identifier_start(input)
            ),
            scan_not_followed_by!(
                input,
                scan_sequence!(
                    scan_not_followed_by!(
                        input,
                        self.decimal_digits(input),
                        scan_chars!(input, '.')
                    ),
                    scan_optional!(input, self.decimal_exponent(input))
                ),
                self.identifier_start(input)
            ),
            if !self.version_is_at_least_0_5_0 {
                scan_not_followed_by!(
                    input,
                    scan_sequence!(
                        scan_not_followed_by!(
                            input,
                            scan_sequence!(self.decimal_digits(input), scan_chars!(input, '.')),
                            self.decimal_digits(input)
                        ),
                        scan_optional!(input, self.decimal_exponent(input))
                    ),
                    self.identifier_start(input)
                )
            } else {
                false
            },
            if !self.version_is_at_least_0_5_0 {
                scan_not_followed_by!(
                    input,
                    scan_sequence!(
                        self.decimal_digits(input),
                        scan_chars!(input, '.'),
                        self.decimal_digits(input),
                        scan_optional!(input, self.decimal_exponent(input))
                    ),
                    self.identifier_start(input)
                )
            } else {
                false
            },
            if self.version_is_at_least_0_5_0 {
                scan_not_followed_by!(
                    input,
                    scan_sequence!(
                        self.decimal_digits(input),
                        scan_optional!(
                            input,
                            scan_sequence!(scan_chars!(input, '.'), self.decimal_digits(input))
                        ),
                        scan_optional!(input, self.decimal_exponent(input))
                    ),
                    self.identifier_start(input)
                )
            } else {
                false
            }
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_hex_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'h', 'e', 'x', '"'),
            scan_optional!(input, self.hex_string_contents(input)),
            scan_chars!(input, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            if !self.version_is_at_least_0_4_25 {
                scan_sequence!(
                    scan_chars!(input, '"'),
                    scan_zero_or_more!(
                        input,
                        scan_choice!(
                            input,
                            self.escape_sequence_arbitrary(input),
                            scan_none_of!(input, '"', '\\', '\r', '\n')
                        )
                    ),
                    scan_chars!(input, '"')
                )
            } else {
                false
            },
            if self.version_is_at_least_0_4_25 && !self.version_is_at_least_0_7_0 {
                scan_sequence!(
                    scan_chars!(input, '"'),
                    scan_zero_or_more!(
                        input,
                        scan_choice!(
                            input,
                            self.escape_sequence(input),
                            scan_none_of!(input, '"', '\\', '\r', '\n')
                        )
                    ),
                    scan_chars!(input, '"')
                )
            } else {
                false
            },
            scan_sequence!(
                scan_chars!(input, '"'),
                scan_zero_or_more!(
                    input,
                    scan_choice!(
                        input,
                        self.escape_sequence(input),
                        scan_char_range!(input, ' '..='!'),
                        scan_char_range!(input, '#'..='['),
                        scan_char_range!(input, ']'..='~')
                    )
                ),
                scan_chars!(input, '"')
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_unicode_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(input, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '"'),
                scan_zero_or_more!(
                    input,
                    scan_choice!(
                        input,
                        self.escape_sequence(input),
                        scan_none_of!(input, '"', '\\', '\r', '\n')
                    )
                ),
                scan_chars!(input, '"')
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_version_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '"'),
            self.version_specifier_fragment(input),
            scan_zero_or_more!(
                input,
                scan_sequence!(
                    scan_chars!(input, '.'),
                    self.version_specifier_fragment(input)
                )
            ),
            scan_chars!(input, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn end_of_line(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, '\n'),
            scan_sequence!(
                scan_chars!(input, '\r'),
                scan_optional!(input, scan_chars!(input, '\n'))
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn escape_sequence(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '\\'),
            scan_choice!(
                input,
                self.ascii_escape(input),
                self.hex_byte_escape(input),
                self.unicode_escape(input)
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn escape_sequence_arbitrary(&self, input: &mut ParserContext<'_>) -> bool {
        if !self.version_is_at_least_0_4_25 {
            scan_sequence!(
                scan_chars!(input, '\\'),
                scan_choice!(
                    input,
                    scan_none_of!(input, 'x', 'u'),
                    self.hex_byte_escape(input),
                    self.unicode_escape(input)
                )
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_byte_escape(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'x'),
            self.hex_character(input),
            self.hex_character(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_character(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_char_range!(input, '0'..='9'),
            scan_char_range!(input, 'a'..='f'),
            scan_char_range!(input, 'A'..='F')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_not_followed_by!(
                input,
                scan_sequence!(
                    scan_chars!(input, '0', 'x'),
                    scan_one_or_more!(input, self.hex_character(input)),
                    scan_zero_or_more!(
                        input,
                        scan_sequence!(
                            scan_chars!(input, '_'),
                            scan_one_or_more!(input, self.hex_character(input))
                        )
                    )
                ),
                self.identifier_start(input)
            ),
            if !self.version_is_at_least_0_5_0 {
                scan_not_followed_by!(
                    input,
                    scan_sequence!(
                        scan_chars!(input, '0', 'X'),
                        scan_one_or_more!(input, self.hex_character(input)),
                        scan_zero_or_more!(
                            input,
                            scan_sequence!(
                                scan_chars!(input, '_'),
                                scan_one_or_more!(input, self.hex_character(input))
                            )
                        )
                    ),
                    self.identifier_start(input)
                )
            } else {
                false
            }
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_contents(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            self.hex_character(input),
            self.hex_character(input),
            scan_zero_or_more!(
                input,
                scan_sequence!(
                    scan_optional!(input, scan_chars!(input, '_')),
                    self.hex_character(input),
                    self.hex_character(input)
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            self.identifier_start(input),
            scan_zero_or_more!(input, self.identifier_part(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_part(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            self.identifier_start(input),
            scan_char_range!(input, '0'..='9')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_start(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, '_'),
            scan_chars!(input, '$'),
            scan_char_range!(input, 'a'..='z'),
            scan_char_range!(input, 'A'..='Z')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn multi_line_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_not_followed_by!(
                input,
                scan_chars!(input, '/', '*'),
                scan_sequence!(scan_chars!(input, '*'), scan_none_of!(input, '/'))
            ),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    scan_none_of!(input, '*'),
                    scan_not_followed_by!(input, scan_chars!(input, '*'), scan_chars!(input, '/'))
                )
            ),
            scan_chars!(input, '*', '/')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn multi_line_nat_spec_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_not_followed_by!(
                input,
                scan_chars!(input, '/', '*', '*'),
                scan_chars!(input, '/')
            ),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    scan_none_of!(input, '*'),
                    scan_not_followed_by!(input, scan_chars!(input, '*'), scan_chars!(input, '/'))
                )
            ),
            scan_chars!(input, '*', '/')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_line_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_not_followed_by!(input, scan_chars!(input, '/', '/'), scan_chars!(input, '/')),
            scan_zero_or_more!(input, scan_none_of!(input, '\r', '\n'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_line_nat_spec_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '/', '/', '/'),
            scan_zero_or_more!(input, scan_none_of!(input, '\r', '\n'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_hex_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'h', 'e', 'x', '\''),
            scan_optional!(input, self.hex_string_contents(input)),
            scan_chars!(input, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            if !self.version_is_at_least_0_4_25 {
                scan_sequence!(
                    scan_chars!(input, '\''),
                    scan_zero_or_more!(
                        input,
                        scan_choice!(
                            input,
                            self.escape_sequence_arbitrary(input),
                            scan_none_of!(input, '\'', '\\', '\r', '\n')
                        )
                    ),
                    scan_chars!(input, '\'')
                )
            } else {
                false
            },
            if self.version_is_at_least_0_4_25 && !self.version_is_at_least_0_7_0 {
                scan_sequence!(
                    scan_chars!(input, '\''),
                    scan_zero_or_more!(
                        input,
                        scan_choice!(
                            input,
                            self.escape_sequence(input),
                            scan_none_of!(input, '\'', '\\', '\r', '\n')
                        )
                    ),
                    scan_chars!(input, '\'')
                )
            } else {
                false
            },
            scan_sequence!(
                scan_chars!(input, '\''),
                scan_zero_or_more!(
                    input,
                    scan_choice!(
                        input,
                        self.escape_sequence(input),
                        scan_char_range!(input, ' '..='&'),
                        scan_char_range!(input, '('..='['),
                        scan_char_range!(input, ']'..='~')
                    )
                ),
                scan_chars!(input, '\'')
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_unicode_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(input, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '\''),
                scan_zero_or_more!(
                    input,
                    scan_choice!(
                        input,
                        self.escape_sequence(input),
                        scan_none_of!(input, '\'', '\\', '\r', '\n')
                    )
                ),
                scan_chars!(input, '\'')
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_version_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '\''),
            self.version_specifier_fragment(input),
            scan_zero_or_more!(
                input,
                scan_sequence!(
                    scan_chars!(input, '.'),
                    self.version_specifier_fragment(input)
                )
            ),
            scan_chars!(input, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn slash(&self, input: &mut ParserContext<'_>) -> bool {
        scan_not_followed_by!(
            input,
            scan_chars!(input, '/'),
            scan_choice!(
                input,
                scan_chars!(input, '='),
                scan_chars!(input, '/'),
                scan_chars!(input, '*')
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_escape(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'u'),
            self.hex_character(input),
            self.hex_character(input),
            self.hex_character(input),
            self.hex_character(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_specifier(&self, input: &mut ParserContext<'_>) -> bool {
        self.version_specifier_fragment(input)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_specifier_fragment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_one_or_more!(
            input,
            scan_choice!(
                input,
                scan_chars!(input, 'x'),
                scan_chars!(input, 'X'),
                scan_chars!(input, '*'),
                scan_char_range!(input, '0'..='9')
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn whitespace(&self, input: &mut ParserContext<'_>) -> bool {
        scan_one_or_more!(
            input,
            scan_choice!(input, scan_chars!(input, ' '), scan_chars!(input, '\t'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_decimal_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_not_followed_by!(
            input,
            scan_choice!(
                input,
                scan_chars!(input, '0'),
                scan_sequence!(
                    scan_char_range!(input, '1'..='9'),
                    scan_zero_or_more!(input, scan_char_range!(input, '0'..='9'))
                )
            ),
            self.identifier_start(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_hex_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_not_followed_by!(
            input,
            scan_sequence!(
                scan_chars!(input, '0', 'x'),
                scan_one_or_more!(input, self.hex_character(input))
            ),
            self.identifier_start(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            if self.version_is_at_least_0_5_8 && !self.version_is_at_least_0_7_0 {
                scan_sequence!(
                    self.identifier_start(input),
                    scan_zero_or_more!(
                        input,
                        scan_choice!(input, scan_chars!(input, '.'), self.identifier_part(input))
                    )
                )
            } else {
                false
            },
            scan_sequence!(
                self.identifier_start(input),
                scan_zero_or_more!(input, self.identifier_part(input))
            )
        )
    }

    #[inline]
    fn bytes_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if scan_sequence!(
                scan_chars!(input, 'b', 'y', 't', 'e', 's'),
                scan_optional!(
                    input,
                    scan_choice!(
                        input,
                        scan_chars!(input, '9'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7'),
                        scan_chars!(input, '6'),
                        scan_chars!(input, '5'),
                        scan_chars!(input, '4'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '3', '1'),
                        scan_chars!(input, '3', '0'),
                        scan_chars!(input, '3'),
                        scan_chars!(input, '2', '9'),
                        scan_chars!(input, '2', '8'),
                        scan_chars!(input, '2', '7'),
                        scan_chars!(input, '2', '6'),
                        scan_chars!(input, '2', '5'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3'),
                        scan_chars!(input, '2', '2'),
                        scan_chars!(input, '2', '1'),
                        scan_chars!(input, '2', '0'),
                        scan_chars!(input, '2'),
                        scan_chars!(input, '1', '9'),
                        scan_chars!(input, '1', '8'),
                        scan_chars!(input, '1', '7'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5'),
                        scan_chars!(input, '1', '4'),
                        scan_chars!(input, '1', '3'),
                        scan_chars!(input, '1', '2'),
                        scan_chars!(input, '1', '1'),
                        scan_chars!(input, '1', '0'),
                        scan_chars!(input, '1')
                    )
                )
            ) {
                KeywordScan::Reserved(TerminalKind::BytesKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn fixed_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if scan_chars!(input, 'f', 'i', 'x', 'e', 'd') {
                KeywordScan::Reserved(TerminalKind::FixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9', '6'),
                    scan_chars!(input, '8', '8'),
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '1', '7', '6'),
                    scan_chars!(input, '1', '6', '8'),
                    scan_chars!(input, '1', '6', '0'),
                    scan_chars!(input, '1', '6'),
                    scan_chars!(input, '1', '5', '2'),
                    scan_chars!(input, '1', '4', '4'),
                    scan_chars!(input, '1', '3', '6'),
                    scan_chars!(input, '1', '2', '8'),
                    scan_chars!(input, '1', '2', '0'),
                    scan_chars!(input, '1', '1', '2'),
                    scan_chars!(input, '1', '0', '4')
                ),
                scan_chars!(input, 'x'),
                scan_choice!(
                    input,
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '1', '6')
                )
            ) {
                KeywordScan::Reserved(TerminalKind::FixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '2', '4', '8', 'x', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '1', '6'),
                    scan_chars!(input, '2', '3', '2', 'x', '8'),
                    scan_chars!(input, '2', '3', '2', 'x', '2', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '1', '6'),
                    scan_chars!(input, '2', '2', '4', 'x', '8'),
                    scan_chars!(input, '2', '2', '4', 'x', '3', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '2', '4'),
                    scan_chars!(input, '2', '2', '4', 'x', '1', '6'),
                    scan_chars!(input, '2', '1', '6', 'x', '8'),
                    scan_chars!(input, '2', '1', '6', 'x', '4', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '3', '2'),
                    scan_chars!(input, '2', '1', '6', 'x', '2', '4'),
                    scan_chars!(input, '2', '1', '6', 'x', '1', '6'),
                    scan_chars!(input, '2', '0', '8', 'x', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '4', '0'),
                    scan_chars!(input, '2', '0', '8', 'x', '3', '2'),
                    scan_chars!(input, '2', '0', '8', 'x', '2', '4'),
                    scan_chars!(input, '2', '0', '8', 'x', '1', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '8'),
                    scan_chars!(input, '2', '0', '0', 'x', '5', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '0', 'x', '4', '0'),
                    scan_chars!(input, '2', '0', '0', 'x', '3', '2'),
                    scan_chars!(input, '2', '0', '0', 'x', '2', '4'),
                    scan_chars!(input, '2', '0', '0', 'x', '1', '6'),
                    scan_chars!(input, '1', '9', '2', 'x', '8'),
                    scan_chars!(input, '1', '9', '2', 'x', '6', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '5', '6'),
                    scan_chars!(input, '1', '9', '2', 'x', '4', '8'),
                    scan_chars!(input, '1', '9', '2', 'x', '4', '0'),
                    scan_chars!(input, '1', '9', '2', 'x', '3', '2'),
                    scan_chars!(input, '1', '9', '2', 'x', '2', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '1', '6'),
                    scan_chars!(input, '1', '8', '4', 'x', '8'),
                    scan_chars!(input, '1', '8', '4', 'x', '7', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '6', '4'),
                    scan_chars!(input, '1', '8', '4', 'x', '5', '6'),
                    scan_chars!(input, '1', '8', '4', 'x', '4', '8'),
                    scan_chars!(input, '1', '8', '4', 'x', '4', '0'),
                    scan_chars!(input, '1', '8', '4', 'x', '3', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '2', '4'),
                    scan_chars!(input, '1', '8', '4', 'x', '1', '6')
                )
            ) {
                KeywordScan::Reserved(TerminalKind::FixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '2', '5', '6', 'x', '8', '0'),
                    scan_chars!(input, '2', '5', '6', 'x', '8'),
                    scan_chars!(input, '2', '5', '6', 'x', '7', '2'),
                    scan_chars!(input, '2', '5', '6', 'x', '6', '4'),
                    scan_chars!(input, '2', '5', '6', 'x', '5', '6'),
                    scan_chars!(input, '2', '5', '6', 'x', '4', '8'),
                    scan_chars!(input, '2', '5', '6', 'x', '4', '0'),
                    scan_chars!(input, '2', '5', '6', 'x', '3', '2'),
                    scan_chars!(input, '2', '5', '6', 'x', '2', '4'),
                    scan_chars!(input, '2', '5', '6', 'x', '1', '6'),
                    scan_chars!(input, '2', '4', '8', 'x', '8', '0'),
                    scan_chars!(input, '2', '4', '8', 'x', '7', '2'),
                    scan_chars!(input, '2', '4', '8', 'x', '6', '4'),
                    scan_chars!(input, '2', '4', '8', 'x', '5', '6'),
                    scan_chars!(input, '2', '4', '8', 'x', '4', '8'),
                    scan_chars!(input, '2', '4', '8', 'x', '4', '0'),
                    scan_chars!(input, '2', '4', '8', 'x', '3', '2'),
                    scan_chars!(input, '2', '4', '8', 'x', '2', '4'),
                    scan_chars!(input, '2', '4', '8', 'x', '1', '6'),
                    scan_chars!(input, '2', '4', '0', 'x', '8', '0'),
                    scan_chars!(input, '2', '4', '0', 'x', '7', '2'),
                    scan_chars!(input, '2', '4', '0', 'x', '6', '4'),
                    scan_chars!(input, '2', '4', '0', 'x', '5', '6'),
                    scan_chars!(input, '2', '4', '0', 'x', '4', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '4', '0'),
                    scan_chars!(input, '2', '4', '0', 'x', '3', '2'),
                    scan_chars!(input, '2', '4', '0', 'x', '2', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '8', '0'),
                    scan_chars!(input, '2', '3', '2', 'x', '7', '2'),
                    scan_chars!(input, '2', '3', '2', 'x', '6', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '5', '6'),
                    scan_chars!(input, '2', '3', '2', 'x', '4', '8'),
                    scan_chars!(input, '2', '3', '2', 'x', '4', '0'),
                    scan_chars!(input, '2', '3', '2', 'x', '3', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '8', '0'),
                    scan_chars!(input, '2', '2', '4', 'x', '7', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '6', '4'),
                    scan_chars!(input, '2', '2', '4', 'x', '5', '6'),
                    scan_chars!(input, '2', '2', '4', 'x', '4', '8'),
                    scan_chars!(input, '2', '2', '4', 'x', '4', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '8', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '7', '2'),
                    scan_chars!(input, '2', '1', '6', 'x', '6', '4'),
                    scan_chars!(input, '2', '1', '6', 'x', '5', '6'),
                    scan_chars!(input, '2', '1', '6', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '8', '0'),
                    scan_chars!(input, '2', '0', '8', 'x', '7', '2'),
                    scan_chars!(input, '2', '0', '8', 'x', '6', '4'),
                    scan_chars!(input, '2', '0', '8', 'x', '5', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '8', '0'),
                    scan_chars!(input, '2', '0', '0', 'x', '7', '2'),
                    scan_chars!(input, '2', '0', '0', 'x', '6', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '8', '0'),
                    scan_chars!(input, '1', '9', '2', 'x', '7', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '8', '0')
                )
            ) {
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved(TerminalKind::FixedKeyword)
                } else {
                    KeywordScan::Present(TerminalKind::FixedKeyword)
                }
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9', '6'),
                    scan_chars!(input, '8', '8'),
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '5', '6'),
                    scan_chars!(input, '2', '4', '8'),
                    scan_chars!(input, '2', '4', '0'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '2', '3', '2'),
                    scan_chars!(input, '2', '2', '4'),
                    scan_chars!(input, '2', '1', '6'),
                    scan_chars!(input, '2', '0', '8'),
                    scan_chars!(input, '2', '0', '0'),
                    scan_chars!(input, '1', '9', '2'),
                    scan_chars!(input, '1', '8', '4'),
                    scan_chars!(input, '1', '7', '6'),
                    scan_chars!(input, '1', '6', '8'),
                    scan_chars!(input, '1', '6', '0'),
                    scan_chars!(input, '1', '6'),
                    scan_chars!(input, '1', '5', '2'),
                    scan_chars!(input, '1', '4', '4'),
                    scan_chars!(input, '1', '3', '6'),
                    scan_chars!(input, '1', '2', '8'),
                    scan_chars!(input, '1', '2', '0'),
                    scan_chars!(input, '1', '1', '2'),
                    scan_chars!(input, '1', '0', '4')
                ),
                scan_chars!(input, 'x'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9'),
                    scan_chars!(input, '7', '9'),
                    scan_chars!(input, '7', '8'),
                    scan_chars!(input, '7', '7'),
                    scan_chars!(input, '7', '6'),
                    scan_chars!(input, '7', '5'),
                    scan_chars!(input, '7', '4'),
                    scan_chars!(input, '7', '3'),
                    scan_chars!(input, '7', '1'),
                    scan_chars!(input, '7', '0'),
                    scan_chars!(input, '7'),
                    scan_chars!(input, '6', '9'),
                    scan_chars!(input, '6', '8'),
                    scan_chars!(input, '6', '7'),
                    scan_chars!(input, '6', '6'),
                    scan_chars!(input, '6', '5'),
                    scan_chars!(input, '6', '3'),
                    scan_chars!(input, '6', '2'),
                    scan_chars!(input, '6', '1'),
                    scan_chars!(input, '6', '0'),
                    scan_chars!(input, '6'),
                    scan_chars!(input, '5', '9'),
                    scan_chars!(input, '5', '8'),
                    scan_chars!(input, '5', '7'),
                    scan_chars!(input, '5', '5'),
                    scan_chars!(input, '5', '4'),
                    scan_chars!(input, '5', '3'),
                    scan_chars!(input, '5', '2'),
                    scan_chars!(input, '5', '1'),
                    scan_chars!(input, '5', '0'),
                    scan_chars!(input, '5'),
                    scan_chars!(input, '4', '9'),
                    scan_chars!(input, '4', '7'),
                    scan_chars!(input, '4', '6'),
                    scan_chars!(input, '4', '5'),
                    scan_chars!(input, '4', '4'),
                    scan_chars!(input, '4', '3'),
                    scan_chars!(input, '4', '2'),
                    scan_chars!(input, '4', '1'),
                    scan_chars!(input, '4'),
                    scan_chars!(input, '3', '9'),
                    scan_chars!(input, '3', '8'),
                    scan_chars!(input, '3', '7'),
                    scan_chars!(input, '3', '6'),
                    scan_chars!(input, '3', '5'),
                    scan_chars!(input, '3', '4'),
                    scan_chars!(input, '3', '3'),
                    scan_chars!(input, '3', '1'),
                    scan_chars!(input, '3', '0'),
                    scan_chars!(input, '3'),
                    scan_chars!(input, '2', '9'),
                    scan_chars!(input, '2', '8'),
                    scan_chars!(input, '2', '7'),
                    scan_chars!(input, '2', '6'),
                    scan_chars!(input, '2', '5'),
                    scan_chars!(input, '2', '3'),
                    scan_chars!(input, '2', '2'),
                    scan_chars!(input, '2', '1'),
                    scan_chars!(input, '2', '0'),
                    scan_chars!(input, '2'),
                    scan_chars!(input, '1', '9'),
                    scan_chars!(input, '1', '8'),
                    scan_chars!(input, '1', '7'),
                    scan_chars!(input, '1', '5'),
                    scan_chars!(input, '1', '4'),
                    scan_chars!(input, '1', '3'),
                    scan_chars!(input, '1', '2'),
                    scan_chars!(input, '1', '1'),
                    scan_chars!(input, '1', '0'),
                    scan_chars!(input, '1'),
                    scan_chars!(input, '0')
                )
            ) {
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved(TerminalKind::FixedKeyword)
                } else {
                    KeywordScan::Present(TerminalKind::FixedKeyword)
                }
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn int_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if scan_sequence!(
                scan_chars!(input, 'i', 'n', 't'),
                scan_optional!(
                    input,
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '5', '6'),
                        scan_chars!(input, '2', '4', '8'),
                        scan_chars!(input, '2', '4', '0'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3', '2'),
                        scan_chars!(input, '2', '2', '4'),
                        scan_chars!(input, '2', '1', '6'),
                        scan_chars!(input, '2', '0', '8'),
                        scan_chars!(input, '2', '0', '0'),
                        scan_chars!(input, '1', '9', '2'),
                        scan_chars!(input, '1', '8', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    )
                )
            ) {
                KeywordScan::Reserved(TerminalKind::IntKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn ufixed_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd') {
                KeywordScan::Reserved(TerminalKind::UfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9', '6'),
                    scan_chars!(input, '8', '8'),
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '1', '7', '6'),
                    scan_chars!(input, '1', '6', '8'),
                    scan_chars!(input, '1', '6', '0'),
                    scan_chars!(input, '1', '6'),
                    scan_chars!(input, '1', '5', '2'),
                    scan_chars!(input, '1', '4', '4'),
                    scan_chars!(input, '1', '3', '6'),
                    scan_chars!(input, '1', '2', '8'),
                    scan_chars!(input, '1', '2', '0'),
                    scan_chars!(input, '1', '1', '2'),
                    scan_chars!(input, '1', '0', '4')
                ),
                scan_chars!(input, 'x'),
                scan_choice!(
                    input,
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '1', '6')
                )
            ) {
                KeywordScan::Reserved(TerminalKind::UfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '2', '4', '8', 'x', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '1', '6'),
                    scan_chars!(input, '2', '3', '2', 'x', '8'),
                    scan_chars!(input, '2', '3', '2', 'x', '2', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '1', '6'),
                    scan_chars!(input, '2', '2', '4', 'x', '8'),
                    scan_chars!(input, '2', '2', '4', 'x', '3', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '2', '4'),
                    scan_chars!(input, '2', '2', '4', 'x', '1', '6'),
                    scan_chars!(input, '2', '1', '6', 'x', '8'),
                    scan_chars!(input, '2', '1', '6', 'x', '4', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '3', '2'),
                    scan_chars!(input, '2', '1', '6', 'x', '2', '4'),
                    scan_chars!(input, '2', '1', '6', 'x', '1', '6'),
                    scan_chars!(input, '2', '0', '8', 'x', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '4', '0'),
                    scan_chars!(input, '2', '0', '8', 'x', '3', '2'),
                    scan_chars!(input, '2', '0', '8', 'x', '2', '4'),
                    scan_chars!(input, '2', '0', '8', 'x', '1', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '8'),
                    scan_chars!(input, '2', '0', '0', 'x', '5', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '0', 'x', '4', '0'),
                    scan_chars!(input, '2', '0', '0', 'x', '3', '2'),
                    scan_chars!(input, '2', '0', '0', 'x', '2', '4'),
                    scan_chars!(input, '2', '0', '0', 'x', '1', '6'),
                    scan_chars!(input, '1', '9', '2', 'x', '8'),
                    scan_chars!(input, '1', '9', '2', 'x', '6', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '5', '6'),
                    scan_chars!(input, '1', '9', '2', 'x', '4', '8'),
                    scan_chars!(input, '1', '9', '2', 'x', '4', '0'),
                    scan_chars!(input, '1', '9', '2', 'x', '3', '2'),
                    scan_chars!(input, '1', '9', '2', 'x', '2', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '1', '6'),
                    scan_chars!(input, '1', '8', '4', 'x', '8'),
                    scan_chars!(input, '1', '8', '4', 'x', '7', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '6', '4'),
                    scan_chars!(input, '1', '8', '4', 'x', '5', '6'),
                    scan_chars!(input, '1', '8', '4', 'x', '4', '8'),
                    scan_chars!(input, '1', '8', '4', 'x', '4', '0'),
                    scan_chars!(input, '1', '8', '4', 'x', '3', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '2', '4'),
                    scan_chars!(input, '1', '8', '4', 'x', '1', '6')
                )
            ) {
                KeywordScan::Reserved(TerminalKind::UfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '2', '5', '6', 'x', '8', '0'),
                    scan_chars!(input, '2', '5', '6', 'x', '8'),
                    scan_chars!(input, '2', '5', '6', 'x', '7', '2'),
                    scan_chars!(input, '2', '5', '6', 'x', '6', '4'),
                    scan_chars!(input, '2', '5', '6', 'x', '5', '6'),
                    scan_chars!(input, '2', '5', '6', 'x', '4', '8'),
                    scan_chars!(input, '2', '5', '6', 'x', '4', '0'),
                    scan_chars!(input, '2', '5', '6', 'x', '3', '2'),
                    scan_chars!(input, '2', '5', '6', 'x', '2', '4'),
                    scan_chars!(input, '2', '5', '6', 'x', '1', '6'),
                    scan_chars!(input, '2', '4', '8', 'x', '8', '0'),
                    scan_chars!(input, '2', '4', '8', 'x', '7', '2'),
                    scan_chars!(input, '2', '4', '8', 'x', '6', '4'),
                    scan_chars!(input, '2', '4', '8', 'x', '5', '6'),
                    scan_chars!(input, '2', '4', '8', 'x', '4', '8'),
                    scan_chars!(input, '2', '4', '8', 'x', '4', '0'),
                    scan_chars!(input, '2', '4', '8', 'x', '3', '2'),
                    scan_chars!(input, '2', '4', '8', 'x', '2', '4'),
                    scan_chars!(input, '2', '4', '8', 'x', '1', '6'),
                    scan_chars!(input, '2', '4', '0', 'x', '8', '0'),
                    scan_chars!(input, '2', '4', '0', 'x', '7', '2'),
                    scan_chars!(input, '2', '4', '0', 'x', '6', '4'),
                    scan_chars!(input, '2', '4', '0', 'x', '5', '6'),
                    scan_chars!(input, '2', '4', '0', 'x', '4', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '4', '0'),
                    scan_chars!(input, '2', '4', '0', 'x', '3', '2'),
                    scan_chars!(input, '2', '4', '0', 'x', '2', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '8', '0'),
                    scan_chars!(input, '2', '3', '2', 'x', '7', '2'),
                    scan_chars!(input, '2', '3', '2', 'x', '6', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '5', '6'),
                    scan_chars!(input, '2', '3', '2', 'x', '4', '8'),
                    scan_chars!(input, '2', '3', '2', 'x', '4', '0'),
                    scan_chars!(input, '2', '3', '2', 'x', '3', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '8', '0'),
                    scan_chars!(input, '2', '2', '4', 'x', '7', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '6', '4'),
                    scan_chars!(input, '2', '2', '4', 'x', '5', '6'),
                    scan_chars!(input, '2', '2', '4', 'x', '4', '8'),
                    scan_chars!(input, '2', '2', '4', 'x', '4', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '8', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '7', '2'),
                    scan_chars!(input, '2', '1', '6', 'x', '6', '4'),
                    scan_chars!(input, '2', '1', '6', 'x', '5', '6'),
                    scan_chars!(input, '2', '1', '6', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '8', '0'),
                    scan_chars!(input, '2', '0', '8', 'x', '7', '2'),
                    scan_chars!(input, '2', '0', '8', 'x', '6', '4'),
                    scan_chars!(input, '2', '0', '8', 'x', '5', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '8', '0'),
                    scan_chars!(input, '2', '0', '0', 'x', '7', '2'),
                    scan_chars!(input, '2', '0', '0', 'x', '6', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '8', '0'),
                    scan_chars!(input, '1', '9', '2', 'x', '7', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '8', '0')
                )
            ) {
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved(TerminalKind::UfixedKeyword)
                } else {
                    KeywordScan::Present(TerminalKind::UfixedKeyword)
                }
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9', '6'),
                    scan_chars!(input, '8', '8'),
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '5', '6'),
                    scan_chars!(input, '2', '4', '8'),
                    scan_chars!(input, '2', '4', '0'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '2', '3', '2'),
                    scan_chars!(input, '2', '2', '4'),
                    scan_chars!(input, '2', '1', '6'),
                    scan_chars!(input, '2', '0', '8'),
                    scan_chars!(input, '2', '0', '0'),
                    scan_chars!(input, '1', '9', '2'),
                    scan_chars!(input, '1', '8', '4'),
                    scan_chars!(input, '1', '7', '6'),
                    scan_chars!(input, '1', '6', '8'),
                    scan_chars!(input, '1', '6', '0'),
                    scan_chars!(input, '1', '6'),
                    scan_chars!(input, '1', '5', '2'),
                    scan_chars!(input, '1', '4', '4'),
                    scan_chars!(input, '1', '3', '6'),
                    scan_chars!(input, '1', '2', '8'),
                    scan_chars!(input, '1', '2', '0'),
                    scan_chars!(input, '1', '1', '2'),
                    scan_chars!(input, '1', '0', '4')
                ),
                scan_chars!(input, 'x'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9'),
                    scan_chars!(input, '7', '9'),
                    scan_chars!(input, '7', '8'),
                    scan_chars!(input, '7', '7'),
                    scan_chars!(input, '7', '6'),
                    scan_chars!(input, '7', '5'),
                    scan_chars!(input, '7', '4'),
                    scan_chars!(input, '7', '3'),
                    scan_chars!(input, '7', '1'),
                    scan_chars!(input, '7', '0'),
                    scan_chars!(input, '7'),
                    scan_chars!(input, '6', '9'),
                    scan_chars!(input, '6', '8'),
                    scan_chars!(input, '6', '7'),
                    scan_chars!(input, '6', '6'),
                    scan_chars!(input, '6', '5'),
                    scan_chars!(input, '6', '3'),
                    scan_chars!(input, '6', '2'),
                    scan_chars!(input, '6', '1'),
                    scan_chars!(input, '6', '0'),
                    scan_chars!(input, '6'),
                    scan_chars!(input, '5', '9'),
                    scan_chars!(input, '5', '8'),
                    scan_chars!(input, '5', '7'),
                    scan_chars!(input, '5', '5'),
                    scan_chars!(input, '5', '4'),
                    scan_chars!(input, '5', '3'),
                    scan_chars!(input, '5', '2'),
                    scan_chars!(input, '5', '1'),
                    scan_chars!(input, '5', '0'),
                    scan_chars!(input, '5'),
                    scan_chars!(input, '4', '9'),
                    scan_chars!(input, '4', '7'),
                    scan_chars!(input, '4', '6'),
                    scan_chars!(input, '4', '5'),
                    scan_chars!(input, '4', '4'),
                    scan_chars!(input, '4', '3'),
                    scan_chars!(input, '4', '2'),
                    scan_chars!(input, '4', '1'),
                    scan_chars!(input, '4'),
                    scan_chars!(input, '3', '9'),
                    scan_chars!(input, '3', '8'),
                    scan_chars!(input, '3', '7'),
                    scan_chars!(input, '3', '6'),
                    scan_chars!(input, '3', '5'),
                    scan_chars!(input, '3', '4'),
                    scan_chars!(input, '3', '3'),
                    scan_chars!(input, '3', '1'),
                    scan_chars!(input, '3', '0'),
                    scan_chars!(input, '3'),
                    scan_chars!(input, '2', '9'),
                    scan_chars!(input, '2', '8'),
                    scan_chars!(input, '2', '7'),
                    scan_chars!(input, '2', '6'),
                    scan_chars!(input, '2', '5'),
                    scan_chars!(input, '2', '3'),
                    scan_chars!(input, '2', '2'),
                    scan_chars!(input, '2', '1'),
                    scan_chars!(input, '2', '0'),
                    scan_chars!(input, '2'),
                    scan_chars!(input, '1', '9'),
                    scan_chars!(input, '1', '8'),
                    scan_chars!(input, '1', '7'),
                    scan_chars!(input, '1', '5'),
                    scan_chars!(input, '1', '4'),
                    scan_chars!(input, '1', '3'),
                    scan_chars!(input, '1', '2'),
                    scan_chars!(input, '1', '1'),
                    scan_chars!(input, '1', '0'),
                    scan_chars!(input, '1'),
                    scan_chars!(input, '0')
                )
            ) {
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved(TerminalKind::UfixedKeyword)
                } else {
                    KeywordScan::Present(TerminalKind::UfixedKeyword)
                }
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn uint_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if scan_sequence!(
                scan_chars!(input, 'u', 'i', 'n', 't'),
                scan_optional!(
                    input,
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '5', '6'),
                        scan_chars!(input, '2', '4', '8'),
                        scan_chars!(input, '2', '4', '0'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3', '2'),
                        scan_chars!(input, '2', '2', '4'),
                        scan_chars!(input, '2', '1', '6'),
                        scan_chars!(input, '2', '0', '8'),
                        scan_chars!(input, '2', '0', '0'),
                        scan_chars!(input, '1', '9', '2'),
                        scan_chars!(input, '1', '8', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    )
                )
            ) {
                KeywordScan::Reserved(TerminalKind::UintKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_bytes_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'b', 'y', 't', 'e', 's'),
                    scan_optional!(
                        input,
                        scan_choice!(
                            input,
                            scan_chars!(input, '9'),
                            scan_chars!(input, '8'),
                            scan_chars!(input, '7'),
                            scan_chars!(input, '6'),
                            scan_chars!(input, '5'),
                            scan_chars!(input, '4'),
                            scan_chars!(input, '3', '2'),
                            scan_chars!(input, '3', '1'),
                            scan_chars!(input, '3', '0'),
                            scan_chars!(input, '3'),
                            scan_chars!(input, '2', '9'),
                            scan_chars!(input, '2', '8'),
                            scan_chars!(input, '2', '7'),
                            scan_chars!(input, '2', '6'),
                            scan_chars!(input, '2', '5'),
                            scan_chars!(input, '2', '4'),
                            scan_chars!(input, '2', '3'),
                            scan_chars!(input, '2', '2'),
                            scan_chars!(input, '2', '1'),
                            scan_chars!(input, '2', '0'),
                            scan_chars!(input, '2'),
                            scan_chars!(input, '1', '9'),
                            scan_chars!(input, '1', '8'),
                            scan_chars!(input, '1', '7'),
                            scan_chars!(input, '1', '6'),
                            scan_chars!(input, '1', '5'),
                            scan_chars!(input, '1', '4'),
                            scan_chars!(input, '1', '3'),
                            scan_chars!(input, '1', '2'),
                            scan_chars!(input, '1', '1'),
                            scan_chars!(input, '1', '0'),
                            scan_chars!(input, '1')
                        )
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulBytesKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_fixed_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if !self.version_is_at_least_0_7_1 && scan_chars!(input, 'f', 'i', 'x', 'e', 'd') {
                KeywordScan::Reserved(TerminalKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    ),
                    scan_chars!(input, 'x'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '1', '6')
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '2', '4', '8', 'x', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '1', '6'),
                        scan_chars!(input, '2', '3', '2', 'x', '8'),
                        scan_chars!(input, '2', '3', '2', 'x', '2', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '1', '6'),
                        scan_chars!(input, '2', '2', '4', 'x', '8'),
                        scan_chars!(input, '2', '2', '4', 'x', '3', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '2', '4'),
                        scan_chars!(input, '2', '2', '4', 'x', '1', '6'),
                        scan_chars!(input, '2', '1', '6', 'x', '8'),
                        scan_chars!(input, '2', '1', '6', 'x', '4', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '3', '2'),
                        scan_chars!(input, '2', '1', '6', 'x', '2', '4'),
                        scan_chars!(input, '2', '1', '6', 'x', '1', '6'),
                        scan_chars!(input, '2', '0', '8', 'x', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '4', '0'),
                        scan_chars!(input, '2', '0', '8', 'x', '3', '2'),
                        scan_chars!(input, '2', '0', '8', 'x', '2', '4'),
                        scan_chars!(input, '2', '0', '8', 'x', '1', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '8'),
                        scan_chars!(input, '2', '0', '0', 'x', '5', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '0', 'x', '4', '0'),
                        scan_chars!(input, '2', '0', '0', 'x', '3', '2'),
                        scan_chars!(input, '2', '0', '0', 'x', '2', '4'),
                        scan_chars!(input, '2', '0', '0', 'x', '1', '6'),
                        scan_chars!(input, '1', '9', '2', 'x', '8'),
                        scan_chars!(input, '1', '9', '2', 'x', '6', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '5', '6'),
                        scan_chars!(input, '1', '9', '2', 'x', '4', '8'),
                        scan_chars!(input, '1', '9', '2', 'x', '4', '0'),
                        scan_chars!(input, '1', '9', '2', 'x', '3', '2'),
                        scan_chars!(input, '1', '9', '2', 'x', '2', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '1', '6'),
                        scan_chars!(input, '1', '8', '4', 'x', '8'),
                        scan_chars!(input, '1', '8', '4', 'x', '7', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '6', '4'),
                        scan_chars!(input, '1', '8', '4', 'x', '5', '6'),
                        scan_chars!(input, '1', '8', '4', 'x', '4', '8'),
                        scan_chars!(input, '1', '8', '4', 'x', '4', '0'),
                        scan_chars!(input, '1', '8', '4', 'x', '3', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '2', '4'),
                        scan_chars!(input, '1', '8', '4', 'x', '1', '6')
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if self.version_is_at_least_0_4_14
                && !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '2', '5', '6', 'x', '8', '0'),
                        scan_chars!(input, '2', '5', '6', 'x', '8'),
                        scan_chars!(input, '2', '5', '6', 'x', '7', '2'),
                        scan_chars!(input, '2', '5', '6', 'x', '6', '4'),
                        scan_chars!(input, '2', '5', '6', 'x', '5', '6'),
                        scan_chars!(input, '2', '5', '6', 'x', '4', '8'),
                        scan_chars!(input, '2', '5', '6', 'x', '4', '0'),
                        scan_chars!(input, '2', '5', '6', 'x', '3', '2'),
                        scan_chars!(input, '2', '5', '6', 'x', '2', '4'),
                        scan_chars!(input, '2', '5', '6', 'x', '1', '6'),
                        scan_chars!(input, '2', '4', '8', 'x', '8', '0'),
                        scan_chars!(input, '2', '4', '8', 'x', '7', '2'),
                        scan_chars!(input, '2', '4', '8', 'x', '6', '4'),
                        scan_chars!(input, '2', '4', '8', 'x', '5', '6'),
                        scan_chars!(input, '2', '4', '8', 'x', '4', '8'),
                        scan_chars!(input, '2', '4', '8', 'x', '4', '0'),
                        scan_chars!(input, '2', '4', '8', 'x', '3', '2'),
                        scan_chars!(input, '2', '4', '8', 'x', '2', '4'),
                        scan_chars!(input, '2', '4', '8', 'x', '1', '6'),
                        scan_chars!(input, '2', '4', '0', 'x', '8', '0'),
                        scan_chars!(input, '2', '4', '0', 'x', '7', '2'),
                        scan_chars!(input, '2', '4', '0', 'x', '6', '4'),
                        scan_chars!(input, '2', '4', '0', 'x', '5', '6'),
                        scan_chars!(input, '2', '4', '0', 'x', '4', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '4', '0'),
                        scan_chars!(input, '2', '4', '0', 'x', '3', '2'),
                        scan_chars!(input, '2', '4', '0', 'x', '2', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '8', '0'),
                        scan_chars!(input, '2', '3', '2', 'x', '7', '2'),
                        scan_chars!(input, '2', '3', '2', 'x', '6', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '5', '6'),
                        scan_chars!(input, '2', '3', '2', 'x', '4', '8'),
                        scan_chars!(input, '2', '3', '2', 'x', '4', '0'),
                        scan_chars!(input, '2', '3', '2', 'x', '3', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '8', '0'),
                        scan_chars!(input, '2', '2', '4', 'x', '7', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '6', '4'),
                        scan_chars!(input, '2', '2', '4', 'x', '5', '6'),
                        scan_chars!(input, '2', '2', '4', 'x', '4', '8'),
                        scan_chars!(input, '2', '2', '4', 'x', '4', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '8', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '7', '2'),
                        scan_chars!(input, '2', '1', '6', 'x', '6', '4'),
                        scan_chars!(input, '2', '1', '6', 'x', '5', '6'),
                        scan_chars!(input, '2', '1', '6', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '8', '0'),
                        scan_chars!(input, '2', '0', '8', 'x', '7', '2'),
                        scan_chars!(input, '2', '0', '8', 'x', '6', '4'),
                        scan_chars!(input, '2', '0', '8', 'x', '5', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '8', '0'),
                        scan_chars!(input, '2', '0', '0', 'x', '7', '2'),
                        scan_chars!(input, '2', '0', '0', 'x', '6', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '8', '0'),
                        scan_chars!(input, '1', '9', '2', 'x', '7', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '8', '0')
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if self.version_is_at_least_0_4_14
                && !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '5', '6'),
                        scan_chars!(input, '2', '4', '8'),
                        scan_chars!(input, '2', '4', '0'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3', '2'),
                        scan_chars!(input, '2', '2', '4'),
                        scan_chars!(input, '2', '1', '6'),
                        scan_chars!(input, '2', '0', '8'),
                        scan_chars!(input, '2', '0', '0'),
                        scan_chars!(input, '1', '9', '2'),
                        scan_chars!(input, '1', '8', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    ),
                    scan_chars!(input, 'x'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9'),
                        scan_chars!(input, '7', '9'),
                        scan_chars!(input, '7', '8'),
                        scan_chars!(input, '7', '7'),
                        scan_chars!(input, '7', '6'),
                        scan_chars!(input, '7', '5'),
                        scan_chars!(input, '7', '4'),
                        scan_chars!(input, '7', '3'),
                        scan_chars!(input, '7', '1'),
                        scan_chars!(input, '7', '0'),
                        scan_chars!(input, '7'),
                        scan_chars!(input, '6', '9'),
                        scan_chars!(input, '6', '8'),
                        scan_chars!(input, '6', '7'),
                        scan_chars!(input, '6', '6'),
                        scan_chars!(input, '6', '5'),
                        scan_chars!(input, '6', '3'),
                        scan_chars!(input, '6', '2'),
                        scan_chars!(input, '6', '1'),
                        scan_chars!(input, '6', '0'),
                        scan_chars!(input, '6'),
                        scan_chars!(input, '5', '9'),
                        scan_chars!(input, '5', '8'),
                        scan_chars!(input, '5', '7'),
                        scan_chars!(input, '5', '5'),
                        scan_chars!(input, '5', '4'),
                        scan_chars!(input, '5', '3'),
                        scan_chars!(input, '5', '2'),
                        scan_chars!(input, '5', '1'),
                        scan_chars!(input, '5', '0'),
                        scan_chars!(input, '5'),
                        scan_chars!(input, '4', '9'),
                        scan_chars!(input, '4', '7'),
                        scan_chars!(input, '4', '6'),
                        scan_chars!(input, '4', '5'),
                        scan_chars!(input, '4', '4'),
                        scan_chars!(input, '4', '3'),
                        scan_chars!(input, '4', '2'),
                        scan_chars!(input, '4', '1'),
                        scan_chars!(input, '4'),
                        scan_chars!(input, '3', '9'),
                        scan_chars!(input, '3', '8'),
                        scan_chars!(input, '3', '7'),
                        scan_chars!(input, '3', '6'),
                        scan_chars!(input, '3', '5'),
                        scan_chars!(input, '3', '4'),
                        scan_chars!(input, '3', '3'),
                        scan_chars!(input, '3', '1'),
                        scan_chars!(input, '3', '0'),
                        scan_chars!(input, '3'),
                        scan_chars!(input, '2', '9'),
                        scan_chars!(input, '2', '8'),
                        scan_chars!(input, '2', '7'),
                        scan_chars!(input, '2', '6'),
                        scan_chars!(input, '2', '5'),
                        scan_chars!(input, '2', '3'),
                        scan_chars!(input, '2', '2'),
                        scan_chars!(input, '2', '1'),
                        scan_chars!(input, '2', '0'),
                        scan_chars!(input, '2'),
                        scan_chars!(input, '1', '9'),
                        scan_chars!(input, '1', '8'),
                        scan_chars!(input, '1', '7'),
                        scan_chars!(input, '1', '5'),
                        scan_chars!(input, '1', '4'),
                        scan_chars!(input, '1', '3'),
                        scan_chars!(input, '1', '2'),
                        scan_chars!(input, '1', '1'),
                        scan_chars!(input, '1', '0'),
                        scan_chars!(input, '1'),
                        scan_chars!(input, '0')
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_int_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'i', 'n', 't'),
                    scan_optional!(
                        input,
                        scan_choice!(
                            input,
                            scan_chars!(input, '9', '6'),
                            scan_chars!(input, '8', '8'),
                            scan_chars!(input, '8', '0'),
                            scan_chars!(input, '8'),
                            scan_chars!(input, '7', '2'),
                            scan_chars!(input, '6', '4'),
                            scan_chars!(input, '5', '6'),
                            scan_chars!(input, '4', '8'),
                            scan_chars!(input, '4', '0'),
                            scan_chars!(input, '3', '2'),
                            scan_chars!(input, '2', '5', '6'),
                            scan_chars!(input, '2', '4', '8'),
                            scan_chars!(input, '2', '4', '0'),
                            scan_chars!(input, '2', '4'),
                            scan_chars!(input, '2', '3', '2'),
                            scan_chars!(input, '2', '2', '4'),
                            scan_chars!(input, '2', '1', '6'),
                            scan_chars!(input, '2', '0', '8'),
                            scan_chars!(input, '2', '0', '0'),
                            scan_chars!(input, '1', '9', '2'),
                            scan_chars!(input, '1', '8', '4'),
                            scan_chars!(input, '1', '7', '6'),
                            scan_chars!(input, '1', '6', '8'),
                            scan_chars!(input, '1', '6', '0'),
                            scan_chars!(input, '1', '6'),
                            scan_chars!(input, '1', '5', '2'),
                            scan_chars!(input, '1', '4', '4'),
                            scan_chars!(input, '1', '3', '6'),
                            scan_chars!(input, '1', '2', '8'),
                            scan_chars!(input, '1', '2', '0'),
                            scan_chars!(input, '1', '1', '2'),
                            scan_chars!(input, '1', '0', '4')
                        )
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulIntKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_ufixed_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if !self.version_is_at_least_0_7_1 && scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd') {
                KeywordScan::Reserved(TerminalKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    ),
                    scan_chars!(input, 'x'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '1', '6')
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '2', '4', '8', 'x', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '1', '6'),
                        scan_chars!(input, '2', '3', '2', 'x', '8'),
                        scan_chars!(input, '2', '3', '2', 'x', '2', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '1', '6'),
                        scan_chars!(input, '2', '2', '4', 'x', '8'),
                        scan_chars!(input, '2', '2', '4', 'x', '3', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '2', '4'),
                        scan_chars!(input, '2', '2', '4', 'x', '1', '6'),
                        scan_chars!(input, '2', '1', '6', 'x', '8'),
                        scan_chars!(input, '2', '1', '6', 'x', '4', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '3', '2'),
                        scan_chars!(input, '2', '1', '6', 'x', '2', '4'),
                        scan_chars!(input, '2', '1', '6', 'x', '1', '6'),
                        scan_chars!(input, '2', '0', '8', 'x', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '4', '0'),
                        scan_chars!(input, '2', '0', '8', 'x', '3', '2'),
                        scan_chars!(input, '2', '0', '8', 'x', '2', '4'),
                        scan_chars!(input, '2', '0', '8', 'x', '1', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '8'),
                        scan_chars!(input, '2', '0', '0', 'x', '5', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '0', 'x', '4', '0'),
                        scan_chars!(input, '2', '0', '0', 'x', '3', '2'),
                        scan_chars!(input, '2', '0', '0', 'x', '2', '4'),
                        scan_chars!(input, '2', '0', '0', 'x', '1', '6'),
                        scan_chars!(input, '1', '9', '2', 'x', '8'),
                        scan_chars!(input, '1', '9', '2', 'x', '6', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '5', '6'),
                        scan_chars!(input, '1', '9', '2', 'x', '4', '8'),
                        scan_chars!(input, '1', '9', '2', 'x', '4', '0'),
                        scan_chars!(input, '1', '9', '2', 'x', '3', '2'),
                        scan_chars!(input, '1', '9', '2', 'x', '2', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '1', '6'),
                        scan_chars!(input, '1', '8', '4', 'x', '8'),
                        scan_chars!(input, '1', '8', '4', 'x', '7', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '6', '4'),
                        scan_chars!(input, '1', '8', '4', 'x', '5', '6'),
                        scan_chars!(input, '1', '8', '4', 'x', '4', '8'),
                        scan_chars!(input, '1', '8', '4', 'x', '4', '0'),
                        scan_chars!(input, '1', '8', '4', 'x', '3', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '2', '4'),
                        scan_chars!(input, '1', '8', '4', 'x', '1', '6')
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if self.version_is_at_least_0_4_14
                && !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '2', '5', '6', 'x', '8', '0'),
                        scan_chars!(input, '2', '5', '6', 'x', '8'),
                        scan_chars!(input, '2', '5', '6', 'x', '7', '2'),
                        scan_chars!(input, '2', '5', '6', 'x', '6', '4'),
                        scan_chars!(input, '2', '5', '6', 'x', '5', '6'),
                        scan_chars!(input, '2', '5', '6', 'x', '4', '8'),
                        scan_chars!(input, '2', '5', '6', 'x', '4', '0'),
                        scan_chars!(input, '2', '5', '6', 'x', '3', '2'),
                        scan_chars!(input, '2', '5', '6', 'x', '2', '4'),
                        scan_chars!(input, '2', '5', '6', 'x', '1', '6'),
                        scan_chars!(input, '2', '4', '8', 'x', '8', '0'),
                        scan_chars!(input, '2', '4', '8', 'x', '7', '2'),
                        scan_chars!(input, '2', '4', '8', 'x', '6', '4'),
                        scan_chars!(input, '2', '4', '8', 'x', '5', '6'),
                        scan_chars!(input, '2', '4', '8', 'x', '4', '8'),
                        scan_chars!(input, '2', '4', '8', 'x', '4', '0'),
                        scan_chars!(input, '2', '4', '8', 'x', '3', '2'),
                        scan_chars!(input, '2', '4', '8', 'x', '2', '4'),
                        scan_chars!(input, '2', '4', '8', 'x', '1', '6'),
                        scan_chars!(input, '2', '4', '0', 'x', '8', '0'),
                        scan_chars!(input, '2', '4', '0', 'x', '7', '2'),
                        scan_chars!(input, '2', '4', '0', 'x', '6', '4'),
                        scan_chars!(input, '2', '4', '0', 'x', '5', '6'),
                        scan_chars!(input, '2', '4', '0', 'x', '4', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '4', '0'),
                        scan_chars!(input, '2', '4', '0', 'x', '3', '2'),
                        scan_chars!(input, '2', '4', '0', 'x', '2', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '8', '0'),
                        scan_chars!(input, '2', '3', '2', 'x', '7', '2'),
                        scan_chars!(input, '2', '3', '2', 'x', '6', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '5', '6'),
                        scan_chars!(input, '2', '3', '2', 'x', '4', '8'),
                        scan_chars!(input, '2', '3', '2', 'x', '4', '0'),
                        scan_chars!(input, '2', '3', '2', 'x', '3', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '8', '0'),
                        scan_chars!(input, '2', '2', '4', 'x', '7', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '6', '4'),
                        scan_chars!(input, '2', '2', '4', 'x', '5', '6'),
                        scan_chars!(input, '2', '2', '4', 'x', '4', '8'),
                        scan_chars!(input, '2', '2', '4', 'x', '4', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '8', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '7', '2'),
                        scan_chars!(input, '2', '1', '6', 'x', '6', '4'),
                        scan_chars!(input, '2', '1', '6', 'x', '5', '6'),
                        scan_chars!(input, '2', '1', '6', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '8', '0'),
                        scan_chars!(input, '2', '0', '8', 'x', '7', '2'),
                        scan_chars!(input, '2', '0', '8', 'x', '6', '4'),
                        scan_chars!(input, '2', '0', '8', 'x', '5', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '8', '0'),
                        scan_chars!(input, '2', '0', '0', 'x', '7', '2'),
                        scan_chars!(input, '2', '0', '0', 'x', '6', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '8', '0'),
                        scan_chars!(input, '1', '9', '2', 'x', '7', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '8', '0')
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if self.version_is_at_least_0_4_14
                && !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '5', '6'),
                        scan_chars!(input, '2', '4', '8'),
                        scan_chars!(input, '2', '4', '0'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3', '2'),
                        scan_chars!(input, '2', '2', '4'),
                        scan_chars!(input, '2', '1', '6'),
                        scan_chars!(input, '2', '0', '8'),
                        scan_chars!(input, '2', '0', '0'),
                        scan_chars!(input, '1', '9', '2'),
                        scan_chars!(input, '1', '8', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    ),
                    scan_chars!(input, 'x'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9'),
                        scan_chars!(input, '7', '9'),
                        scan_chars!(input, '7', '8'),
                        scan_chars!(input, '7', '7'),
                        scan_chars!(input, '7', '6'),
                        scan_chars!(input, '7', '5'),
                        scan_chars!(input, '7', '4'),
                        scan_chars!(input, '7', '3'),
                        scan_chars!(input, '7', '1'),
                        scan_chars!(input, '7', '0'),
                        scan_chars!(input, '7'),
                        scan_chars!(input, '6', '9'),
                        scan_chars!(input, '6', '8'),
                        scan_chars!(input, '6', '7'),
                        scan_chars!(input, '6', '6'),
                        scan_chars!(input, '6', '5'),
                        scan_chars!(input, '6', '3'),
                        scan_chars!(input, '6', '2'),
                        scan_chars!(input, '6', '1'),
                        scan_chars!(input, '6', '0'),
                        scan_chars!(input, '6'),
                        scan_chars!(input, '5', '9'),
                        scan_chars!(input, '5', '8'),
                        scan_chars!(input, '5', '7'),
                        scan_chars!(input, '5', '5'),
                        scan_chars!(input, '5', '4'),
                        scan_chars!(input, '5', '3'),
                        scan_chars!(input, '5', '2'),
                        scan_chars!(input, '5', '1'),
                        scan_chars!(input, '5', '0'),
                        scan_chars!(input, '5'),
                        scan_chars!(input, '4', '9'),
                        scan_chars!(input, '4', '7'),
                        scan_chars!(input, '4', '6'),
                        scan_chars!(input, '4', '5'),
                        scan_chars!(input, '4', '4'),
                        scan_chars!(input, '4', '3'),
                        scan_chars!(input, '4', '2'),
                        scan_chars!(input, '4', '1'),
                        scan_chars!(input, '4'),
                        scan_chars!(input, '3', '9'),
                        scan_chars!(input, '3', '8'),
                        scan_chars!(input, '3', '7'),
                        scan_chars!(input, '3', '6'),
                        scan_chars!(input, '3', '5'),
                        scan_chars!(input, '3', '4'),
                        scan_chars!(input, '3', '3'),
                        scan_chars!(input, '3', '1'),
                        scan_chars!(input, '3', '0'),
                        scan_chars!(input, '3'),
                        scan_chars!(input, '2', '9'),
                        scan_chars!(input, '2', '8'),
                        scan_chars!(input, '2', '7'),
                        scan_chars!(input, '2', '6'),
                        scan_chars!(input, '2', '5'),
                        scan_chars!(input, '2', '3'),
                        scan_chars!(input, '2', '2'),
                        scan_chars!(input, '2', '1'),
                        scan_chars!(input, '2', '0'),
                        scan_chars!(input, '2'),
                        scan_chars!(input, '1', '9'),
                        scan_chars!(input, '1', '8'),
                        scan_chars!(input, '1', '7'),
                        scan_chars!(input, '1', '5'),
                        scan_chars!(input, '1', '4'),
                        scan_chars!(input, '1', '3'),
                        scan_chars!(input, '1', '2'),
                        scan_chars!(input, '1', '1'),
                        scan_chars!(input, '1', '0'),
                        scan_chars!(input, '1'),
                        scan_chars!(input, '0')
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_uint_keyword(&self, input: &mut ParserContext<'_>, ident_len: usize) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident_len,
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'i', 'n', 't'),
                    scan_optional!(
                        input,
                        scan_choice!(
                            input,
                            scan_chars!(input, '9', '6'),
                            scan_chars!(input, '8', '8'),
                            scan_chars!(input, '8', '0'),
                            scan_chars!(input, '8'),
                            scan_chars!(input, '7', '2'),
                            scan_chars!(input, '6', '4'),
                            scan_chars!(input, '5', '6'),
                            scan_chars!(input, '4', '8'),
                            scan_chars!(input, '4', '0'),
                            scan_chars!(input, '3', '2'),
                            scan_chars!(input, '2', '5', '6'),
                            scan_chars!(input, '2', '4', '8'),
                            scan_chars!(input, '2', '4', '0'),
                            scan_chars!(input, '2', '4'),
                            scan_chars!(input, '2', '3', '2'),
                            scan_chars!(input, '2', '2', '4'),
                            scan_chars!(input, '2', '1', '6'),
                            scan_chars!(input, '2', '0', '8'),
                            scan_chars!(input, '2', '0', '0'),
                            scan_chars!(input, '1', '9', '2'),
                            scan_chars!(input, '1', '8', '4'),
                            scan_chars!(input, '1', '7', '6'),
                            scan_chars!(input, '1', '6', '8'),
                            scan_chars!(input, '1', '6', '0'),
                            scan_chars!(input, '1', '6'),
                            scan_chars!(input, '1', '5', '2'),
                            scan_chars!(input, '1', '4', '4'),
                            scan_chars!(input, '1', '3', '6'),
                            scan_chars!(input, '1', '2', '8'),
                            scan_chars!(input, '1', '2', '0'),
                            scan_chars!(input, '1', '1', '2'),
                            scan_chars!(input, '1', '0', '4')
                        )
                    )
                )
            {
                KeywordScan::Reserved(TerminalKind::YulUintKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }
}

impl Lexer for Parser {
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        input.cached_leading_trivia_or(|input| Parser::leading_trivia(self, input))
    }

    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        Parser::trailing_trivia(self, input)
    }

    fn delimiters<LexCtx: IsLexicalContext>() -> &'static [(TerminalKind, TerminalKind)] {
        match LexCtx::value() {
            LexicalContext::Default => &[
                (TerminalKind::OpenBrace, TerminalKind::CloseBrace),
                (TerminalKind::OpenBracket, TerminalKind::CloseBracket),
                (TerminalKind::OpenParen, TerminalKind::CloseParen),
            ],
            LexicalContext::Pragma => &[],
            LexicalContext::Yul => &[
                (TerminalKind::OpenBrace, TerminalKind::CloseBrace),
                (TerminalKind::OpenParen, TerminalKind::CloseParen),
            ],
        }
    }

    fn next_terminal<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
    ) -> Option<ScannedTerminal> {
        let save = input.position();
        let mut furthest_position = input.position();
        let mut longest_terminal = None;

        macro_rules! longest_match {
            ($( { $kind:ident = $function:ident } )*) => {
                $(
                    if self.$function(input) && input.position() > furthest_position {
                        furthest_position = input.position();

                        longest_terminal = Some(TerminalKind::$kind);
                    }
                    input.set_position(save);
                )*
            };
        }

        match LexCtx::value() {
            LexicalContext::Default => {
                if let Some(kind) = match input.next() {
                    Some('!') => match input.next() {
                        Some('=') => Some(TerminalKind::BangEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Bang)
                        }
                        None => Some(TerminalKind::Bang),
                    },
                    Some('%') => match input.next() {
                        Some('=') => Some(TerminalKind::PercentEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Percent)
                        }
                        None => Some(TerminalKind::Percent),
                    },
                    Some('&') => match input.next() {
                        Some('&') => Some(TerminalKind::AmpersandAmpersand),
                        Some('=') => Some(TerminalKind::AmpersandEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Ampersand)
                        }
                        None => Some(TerminalKind::Ampersand),
                    },
                    Some('(') => Some(TerminalKind::OpenParen),
                    Some(')') => Some(TerminalKind::CloseParen),
                    Some('*') => match input.next() {
                        Some('*') => Some(TerminalKind::AsteriskAsterisk),
                        Some('=') => Some(TerminalKind::AsteriskEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Asterisk)
                        }
                        None => Some(TerminalKind::Asterisk),
                    },
                    Some('+') => match input.next() {
                        Some('+') => Some(TerminalKind::PlusPlus),
                        Some('=') => Some(TerminalKind::PlusEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Plus)
                        }
                        None => Some(TerminalKind::Plus),
                    },
                    Some(',') => Some(TerminalKind::Comma),
                    Some('-') => match input.next() {
                        Some('-') => Some(TerminalKind::MinusMinus),
                        Some('=') => Some(TerminalKind::MinusEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Minus)
                        }
                        None => Some(TerminalKind::Minus),
                    },
                    Some('.') => Some(TerminalKind::Period),
                    Some('/') => {
                        if scan_chars!(input, '=') {
                            Some(TerminalKind::SlashEqual)
                        } else {
                            None
                        }
                    }
                    Some(':') => Some(TerminalKind::Colon),
                    Some(';') => Some(TerminalKind::Semicolon),
                    Some('<') => match input.next() {
                        Some('<') => match input.next() {
                            Some('=') => Some(TerminalKind::LessThanLessThanEqual),
                            Some(_) => {
                                input.undo();
                                Some(TerminalKind::LessThanLessThan)
                            }
                            None => Some(TerminalKind::LessThanLessThan),
                        },
                        Some('=') => Some(TerminalKind::LessThanEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::LessThan)
                        }
                        None => Some(TerminalKind::LessThan),
                    },
                    Some('=') => match input.next() {
                        Some('=') => Some(TerminalKind::EqualEqual),
                        Some('>') => Some(TerminalKind::EqualGreaterThan),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Equal)
                        }
                        None => Some(TerminalKind::Equal),
                    },
                    Some('>') => match input.next() {
                        Some('=') => Some(TerminalKind::GreaterThanEqual),
                        Some('>') => match input.next() {
                            Some('=') => Some(TerminalKind::GreaterThanGreaterThanEqual),
                            Some('>') => match input.next() {
                                Some('=') => {
                                    Some(TerminalKind::GreaterThanGreaterThanGreaterThanEqual)
                                }
                                Some(_) => {
                                    input.undo();
                                    Some(TerminalKind::GreaterThanGreaterThanGreaterThan)
                                }
                                None => Some(TerminalKind::GreaterThanGreaterThanGreaterThan),
                            },
                            Some(_) => {
                                input.undo();
                                Some(TerminalKind::GreaterThanGreaterThan)
                            }
                            None => Some(TerminalKind::GreaterThanGreaterThan),
                        },
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::GreaterThan)
                        }
                        None => Some(TerminalKind::GreaterThan),
                    },
                    Some('?') => Some(TerminalKind::QuestionMark),
                    Some('[') => Some(TerminalKind::OpenBracket),
                    Some(']') => Some(TerminalKind::CloseBracket),
                    Some('^') => match input.next() {
                        Some('=') => Some(TerminalKind::CaretEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Caret)
                        }
                        None => Some(TerminalKind::Caret),
                    },
                    Some('{') => Some(TerminalKind::OpenBrace),
                    Some('|') => match input.next() {
                        Some('=') => Some(TerminalKind::BarEqual),
                        Some('|') => Some(TerminalKind::BarBar),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Bar)
                        }
                        None => Some(TerminalKind::Bar),
                    },
                    Some('}') => Some(TerminalKind::CloseBrace),
                    Some('~') => Some(TerminalKind::Tilde),
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = input.position();
                    longest_terminal = Some(kind);
                }
                input.set_position(save);

                longest_match! {
                    { DecimalLiteral = decimal_literal }
                    { DoubleQuotedHexStringLiteral = double_quoted_hex_string_literal }
                    { DoubleQuotedStringLiteral = double_quoted_string_literal }
                    { DoubleQuotedUnicodeStringLiteral = double_quoted_unicode_string_literal }
                    { EndOfLine = end_of_line }
                    { HexLiteral = hex_literal }
                    { MultiLineComment = multi_line_comment }
                    { MultiLineNatSpecComment = multi_line_nat_spec_comment }
                    { SingleLineComment = single_line_comment }
                    { SingleLineNatSpecComment = single_line_nat_spec_comment }
                    { SingleQuotedHexStringLiteral = single_quoted_hex_string_literal }
                    { SingleQuotedStringLiteral = single_quoted_string_literal }
                    { SingleQuotedUnicodeStringLiteral = single_quoted_unicode_string_literal }
                    { Slash = slash }
                    { Whitespace = whitespace }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {
                    { Identifier = identifier }
                }

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) =
                    longest_terminal.filter(|tok| [TerminalKind::Identifier].contains(tok))
                {
                    let kw_scan = match input.next() {
                        Some('A') => {
                            if scan_chars!(
                                input, 'B', 'I', 'E', 'n', 'c', 'o', 'd', 'e', 'r', 'V', '2'
                            ) {
                                if self.version_is_at_least_0_4_16 {
                                    KeywordScan::Present(TerminalKind::ABIEncoderV2Keyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('S') => {
                            if scan_chars!(input, 'M', 'T', 'C', 'h', 'e', 'c', 'k', 'e', 'r') {
                                if self.version_is_at_least_0_4_16 {
                                    KeywordScan::Present(TerminalKind::SMTCheckerKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('a') => match input.next() {
                            Some('b') => match input.next() {
                                Some('i') => {
                                    if scan_chars!(input, 'c', 'o', 'd', 'e', 'r') {
                                        if self.version_is_at_least_0_7_5 {
                                            KeywordScan::Present(TerminalKind::AbicoderKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('s') => {
                                    if scan_chars!(input, 't', 'r', 'a', 'c', 't') {
                                        KeywordScan::Reserved(TerminalKind::AbstractKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('d') => {
                                if scan_chars!(input, 'd', 'r', 'e', 's', 's') {
                                    KeywordScan::Present(TerminalKind::AddressKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('f') => {
                                if scan_chars!(input, 't', 'e', 'r') {
                                    KeywordScan::Reserved(TerminalKind::AfterKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('l') => {
                                if scan_chars!(input, 'i', 'a', 's') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TerminalKind::AliasKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'o', 'n', 'y', 'm', 'o', 'u', 's') {
                                    KeywordScan::Reserved(TerminalKind::AnonymousKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('p') => {
                                if scan_chars!(input, 'p', 'l', 'y') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TerminalKind::ApplyKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => match input.next() {
                                Some('s') => {
                                    if scan_chars!(input, 'e', 'm', 'b', 'l', 'y') {
                                        KeywordScan::Reserved(TerminalKind::AssemblyKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Reserved(TerminalKind::AsKeyword)
                                }
                                None => KeywordScan::Reserved(TerminalKind::AsKeyword),
                            },
                            Some('t') => {
                                if self.version_is_at_least_0_8_29 {
                                    KeywordScan::Present(TerminalKind::AtKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 't', 'o') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TerminalKind::AutoKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('b') => match input.next() {
                            Some('o') => {
                                if scan_chars!(input, 'o', 'l') {
                                    KeywordScan::Reserved(TerminalKind::BoolKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'e', 'a', 'k') {
                                    KeywordScan::Reserved(TerminalKind::BreakKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('y') => {
                                if scan_chars!(input, 't', 'e') {
                                    KeywordScan::Reserved(TerminalKind::ByteKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('c') => match input.next() {
                            Some('a') => match input.next() {
                                Some('l') => {
                                    if scan_chars!(input, 'l', 'd', 'a', 't', 'a') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TerminalKind::CallDataKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('s') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TerminalKind::CaseKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        KeywordScan::Reserved(TerminalKind::CatchKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('o') => match input.next() {
                                Some('n') => match input.next() {
                                    Some('s') => {
                                        if scan_chars!(input, 't') {
                                            match input.next() {
                                                Some('a') => {
                                                    if scan_chars!(input, 'n', 't') {
                                                        KeywordScan::Reserved(
                                                            TerminalKind::ConstantKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                }
                                                Some('r') => {
                                                    if scan_chars!(input, 'u', 'c', 't', 'o', 'r') {
                                                        if self.version_is_at_least_0_5_0 {
                                                            KeywordScan::Reserved(
                                                                TerminalKind::ConstructorKeyword,
                                                            )
                                                        } else if self.version_is_at_least_0_4_22 {
                                                            KeywordScan::Present(
                                                                TerminalKind::ConstructorKeyword,
                                                            )
                                                        } else {
                                                            KeywordScan::Absent
                                                        }
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                }
                                                Some(_) => {
                                                    input.undo();
                                                    KeywordScan::Absent
                                                }
                                                None => KeywordScan::Absent,
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('t') => match input.next() {
                                        Some('i') => {
                                            if scan_chars!(input, 'n', 'u', 'e') {
                                                KeywordScan::Reserved(TerminalKind::ContinueKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('r') => {
                                            if scan_chars!(input, 'a', 'c', 't') {
                                                KeywordScan::Reserved(TerminalKind::ContractKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    },
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some('p') => {
                                    if scan_chars!(input, 'y', 'o', 'f') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TerminalKind::CopyOfKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('d') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'y', 's') {
                                    KeywordScan::Reserved(TerminalKind::DaysKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('e') => match input.next() {
                                Some('f') => match input.next() {
                                    Some('a') => {
                                        if scan_chars!(input, 'u', 'l', 't') {
                                            KeywordScan::Reserved(TerminalKind::DefaultKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'e') {
                                            if self.version_is_at_least_0_5_0 {
                                                KeywordScan::Reserved(TerminalKind::DefineKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some('l') => {
                                    if scan_chars!(input, 'e', 't', 'e') {
                                        KeywordScan::Reserved(TerminalKind::DeleteKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('o') => KeywordScan::Reserved(TerminalKind::DoKeyword),
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('e') => match input.next() {
                            Some('l') => {
                                if scan_chars!(input, 's', 'e') {
                                    KeywordScan::Reserved(TerminalKind::ElseKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('m') => {
                                if scan_chars!(input, 'i', 't') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TerminalKind::EmitKeyword)
                                    } else if self.version_is_at_least_0_4_21 {
                                        KeywordScan::Present(TerminalKind::EmitKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'u', 'm') {
                                    KeywordScan::Reserved(TerminalKind::EnumKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'r', 'o', 'r') {
                                    if self.version_is_at_least_0_8_4 {
                                        KeywordScan::Present(TerminalKind::ErrorKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('t') => {
                                if scan_chars!(input, 'h', 'e', 'r') {
                                    KeywordScan::Reserved(TerminalKind::EtherKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('v') => {
                                if scan_chars!(input, 'e', 'n', 't') {
                                    KeywordScan::Reserved(TerminalKind::EventKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('x') => match input.next() {
                                Some('p') => {
                                    if scan_chars!(
                                        input, 'e', 'r', 'i', 'm', 'e', 'n', 't', 'a', 'l'
                                    ) {
                                        if self.version_is_at_least_0_4_16 {
                                            KeywordScan::Present(TerminalKind::ExperimentalKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'e', 'r', 'n', 'a', 'l') {
                                        KeywordScan::Reserved(TerminalKind::ExternalKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('f') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'l') {
                                    match input.next() {
                                        Some('l') => {
                                            if scan_chars!(input, 'b', 'a', 'c', 'k') {
                                                if self.version_is_at_least_0_6_0 {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::FallbackKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Present(
                                                        TerminalKind::FallbackKeyword,
                                                    )
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('s') => {
                                            if scan_chars!(input, 'e') {
                                                KeywordScan::Reserved(TerminalKind::FalseKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n') {
                                    match input.next() {
                                        Some('a') => {
                                            if scan_chars!(input, 'l') {
                                                KeywordScan::Reserved(TerminalKind::FinalKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('n') => {
                                            if scan_chars!(input, 'e', 'y') {
                                                if !self.version_is_at_least_0_7_0 {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::FinneyKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'r') {
                                    KeywordScan::Reserved(TerminalKind::ForKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'o', 'm') {
                                    KeywordScan::Present(TerminalKind::FromKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'n', 'c', 't', 'i', 'o', 'n') {
                                    KeywordScan::Reserved(TerminalKind::FunctionKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('g') => match input.next() {
                            Some('l') => {
                                if scan_chars!(input, 'o', 'b', 'a', 'l') {
                                    if self.version_is_at_least_0_8_13 {
                                        KeywordScan::Present(TerminalKind::GlobalKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('w') => {
                                if scan_chars!(input, 'e', 'i') {
                                    if self.version_is_at_least_0_7_0 {
                                        KeywordScan::Reserved(TerminalKind::GweiKeyword)
                                    } else if self.version_is_at_least_0_6_11 {
                                        KeywordScan::Present(TerminalKind::GweiKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('h') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'x') {
                                    KeywordScan::Reserved(TerminalKind::HexKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'u', 'r', 's') {
                                    KeywordScan::Reserved(TerminalKind::HoursKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('i') => match input.next() {
                            Some('f') => KeywordScan::Reserved(TerminalKind::IfKeyword),
                            Some('m') => match input.next() {
                                Some('m') => {
                                    if scan_chars!(input, 'u', 't', 'a', 'b', 'l', 'e') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TerminalKind::ImmutableKeyword)
                                        } else if self.version_is_at_least_0_6_5 {
                                            KeywordScan::Present(TerminalKind::ImmutableKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('p') => match input.next() {
                                    Some('l') => {
                                        if scan_chars!(input, 'e', 'm', 'e', 'n', 't', 's') {
                                            if self.version_is_at_least_0_5_0 {
                                                KeywordScan::Reserved(
                                                    TerminalKind::ImplementsKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('o') => {
                                        if scan_chars!(input, 'r', 't') {
                                            KeywordScan::Reserved(TerminalKind::ImportKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('n') => match input.next() {
                                Some('d') => {
                                    if scan_chars!(input, 'e', 'x', 'e', 'd') {
                                        KeywordScan::Reserved(TerminalKind::IndexedKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('l') => {
                                    if scan_chars!(input, 'i', 'n', 'e') {
                                        KeywordScan::Reserved(TerminalKind::InlineKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'e', 'r') {
                                        match input.next() {
                                            Some('f') => {
                                                if scan_chars!(input, 'a', 'c', 'e') {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::InterfaceKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('n') => {
                                                if scan_chars!(input, 'a', 'l') {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::InternalKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Absent
                                            }
                                            None => KeywordScan::Absent,
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Reserved(TerminalKind::InKeyword)
                                }
                                None => KeywordScan::Reserved(TerminalKind::InKeyword),
                            },
                            Some('s') => KeywordScan::Reserved(TerminalKind::IsKeyword),
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('l') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'y', 'o', 'u', 't') {
                                    if self.version_is_at_least_0_8_29 {
                                        KeywordScan::Present(TerminalKind::LayoutKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('e') => {
                                if scan_chars!(input, 't') {
                                    KeywordScan::Reserved(TerminalKind::LetKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'b', 'r', 'a', 'r', 'y') {
                                    KeywordScan::Reserved(TerminalKind::LibraryKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('m') => match input.next() {
                            Some('a') => match input.next() {
                                Some('c') => {
                                    if scan_chars!(input, 'r', 'o') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TerminalKind::MacroKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('p') => {
                                    if scan_chars!(input, 'p', 'i', 'n', 'g') {
                                        KeywordScan::Reserved(TerminalKind::MappingKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        KeywordScan::Reserved(TerminalKind::MatchKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('e') => {
                                if scan_chars!(input, 'm', 'o', 'r', 'y') {
                                    KeywordScan::Reserved(TerminalKind::MemoryKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n', 'u', 't', 'e', 's') {
                                    KeywordScan::Reserved(TerminalKind::MinutesKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'd', 'i', 'f', 'i', 'e', 'r') {
                                    KeywordScan::Reserved(TerminalKind::ModifierKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 't', 'a', 'b', 'l', 'e') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TerminalKind::MutableKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('n') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'w') {
                                    KeywordScan::Reserved(TerminalKind::NewKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'l', 'l') {
                                    KeywordScan::Reserved(TerminalKind::NullKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('o') => match input.next() {
                            Some('f') => KeywordScan::Reserved(TerminalKind::OfKeyword),
                            Some('v') => {
                                if scan_chars!(input, 'e', 'r', 'r', 'i', 'd', 'e') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TerminalKind::OverrideKeyword)
                                    } else if self.version_is_at_least_0_6_0 {
                                        KeywordScan::Present(TerminalKind::OverrideKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('p') => match input.next() {
                            Some('a') => match input.next() {
                                Some('r') => {
                                    if scan_chars!(input, 't', 'i', 'a', 'l') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TerminalKind::PartialKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('y') => {
                                    if scan_chars!(input, 'a', 'b', 'l', 'e') {
                                        KeywordScan::Reserved(TerminalKind::PayableKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('r') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'g', 'm', 'a') {
                                        KeywordScan::Reserved(TerminalKind::PragmaKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('i') => {
                                    if scan_chars!(input, 'v', 'a', 't', 'e') {
                                        KeywordScan::Reserved(TerminalKind::PrivateKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'm', 'i', 's', 'e') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TerminalKind::PromiseKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('u') => match input.next() {
                                Some('b') => {
                                    if scan_chars!(input, 'l', 'i', 'c') {
                                        KeywordScan::Reserved(TerminalKind::PublicKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TerminalKind::PureKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('r') => {
                            if scan_chars!(input, 'e') {
                                match input.next() {
                                    Some('c') => {
                                        if scan_chars!(input, 'e', 'i', 'v', 'e') {
                                            if self.version_is_at_least_0_6_0 {
                                                KeywordScan::Reserved(TerminalKind::ReceiveKeyword)
                                            } else {
                                                KeywordScan::Present(TerminalKind::ReceiveKeyword)
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('f') => {
                                        if scan_chars!(input, 'e', 'r', 'e', 'n', 'c', 'e') {
                                            if self.version_is_at_least_0_5_0 {
                                                KeywordScan::Reserved(
                                                    TerminalKind::ReferenceKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('l') => {
                                        if scan_chars!(
                                            input, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'
                                        ) {
                                            KeywordScan::Reserved(TerminalKind::RelocatableKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('t') => {
                                        if scan_chars!(input, 'u', 'r', 'n') {
                                            match input.next() {
                                                Some('s') => KeywordScan::Reserved(
                                                    TerminalKind::ReturnsKeyword,
                                                ),
                                                Some(_) => {
                                                    input.undo();
                                                    KeywordScan::Reserved(
                                                        TerminalKind::ReturnKeyword,
                                                    )
                                                }
                                                None => KeywordScan::Reserved(
                                                    TerminalKind::ReturnKeyword,
                                                ),
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('v') => {
                                        if scan_chars!(input, 'e', 'r', 't') {
                                            if self.version_is_at_least_0_8_4 {
                                                KeywordScan::Present(TerminalKind::RevertKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('s') => match input.next() {
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'l', 'e', 'd') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TerminalKind::SealedKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('c') => {
                                    if scan_chars!(input, 'o', 'n', 'd', 's') {
                                        KeywordScan::Reserved(TerminalKind::SecondsKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('i') => {
                                if scan_chars!(input, 'z', 'e', 'o', 'f') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TerminalKind::SizeOfKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'l', 'i', 'd', 'i', 't', 'y') {
                                    KeywordScan::Present(TerminalKind::SolidityKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('t') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 't', 'i', 'c') {
                                        KeywordScan::Reserved(TerminalKind::StaticKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'r', 'a', 'g', 'e') {
                                        KeywordScan::Reserved(TerminalKind::StorageKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => match input.next() {
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'g') {
                                            KeywordScan::Reserved(TerminalKind::StringKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('u') => {
                                        if scan_chars!(input, 'c', 't') {
                                            KeywordScan::Reserved(TerminalKind::StructKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('u') => {
                                if scan_chars!(input, 'p') {
                                    match input.next() {
                                        Some('e') => {
                                            if scan_chars!(input, 'r') {
                                                if self.version_is_at_least_0_8_0 {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::SuperKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Present(TerminalKind::SuperKeyword)
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('p') => {
                                            if scan_chars!(input, 'o', 'r', 't', 's') {
                                                if self.version_is_at_least_0_5_0 {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::SupportsKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('w') => {
                                if scan_chars!(input, 'i', 't', 'c', 'h') {
                                    KeywordScan::Reserved(TerminalKind::SwitchKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('z') => {
                                if scan_chars!(input, 'a', 'b', 'o') {
                                    if !self.version_is_at_least_0_7_0 {
                                        KeywordScan::Reserved(TerminalKind::SzaboKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('t') => match input.next() {
                            Some('h') => match input.next() {
                                Some('i') => {
                                    if scan_chars!(input, 's') {
                                        if self.version_is_at_least_0_8_0 {
                                            KeywordScan::Reserved(TerminalKind::ThisKeyword)
                                        } else {
                                            KeywordScan::Present(TerminalKind::ThisKeyword)
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 'o', 'w') {
                                        KeywordScan::Reserved(TerminalKind::ThrowKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('r') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'n', 's', 'i', 'e', 'n', 't') {
                                        if self.version_is_at_least_0_8_27 {
                                            KeywordScan::Present(TerminalKind::TransientKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('u') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TerminalKind::TrueKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('y') => KeywordScan::Reserved(TerminalKind::TryKeyword),
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('y') => {
                                if scan_chars!(input, 'p', 'e') {
                                    match input.next() {
                                        Some('d') => {
                                            if scan_chars!(input, 'e', 'f') {
                                                if self.version_is_at_least_0_5_0 {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::TypeDefKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('o') => {
                                            if scan_chars!(input, 'f') {
                                                KeywordScan::Reserved(TerminalKind::TypeOfKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Reserved(TerminalKind::TypeKeyword)
                                        }
                                        None => KeywordScan::Reserved(TerminalKind::TypeKeyword),
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('u') => match input.next() {
                            Some('n') => {
                                if scan_chars!(input, 'c', 'h', 'e', 'c', 'k', 'e', 'd') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TerminalKind::UncheckedKeyword)
                                    } else if self.version_is_at_least_0_8_0 {
                                        KeywordScan::Present(TerminalKind::UncheckedKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => {
                                if scan_chars!(input, 'i', 'n', 'g') {
                                    KeywordScan::Reserved(TerminalKind::UsingKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('v') => match input.next() {
                            Some('1') => {
                                if self.version_is_at_least_0_7_5 {
                                    KeywordScan::Present(TerminalKind::AbicoderV1Keyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('2') => {
                                if self.version_is_at_least_0_7_5 {
                                    KeywordScan::Present(TerminalKind::AbicoderV2Keyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('a') => {
                                if scan_chars!(input, 'r') {
                                    KeywordScan::Reserved(TerminalKind::VarKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'w') {
                                        KeywordScan::Reserved(TerminalKind::ViewKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 't', 'u', 'a', 'l') {
                                        if self.version_is_at_least_0_6_0 {
                                            KeywordScan::Reserved(TerminalKind::VirtualKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('w') => match input.next() {
                            Some('e') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'k', 's') {
                                        KeywordScan::Reserved(TerminalKind::WeeksKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('i') => KeywordScan::Reserved(TerminalKind::WeiKeyword),
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('h') => {
                                if scan_chars!(input, 'i', 'l', 'e') {
                                    KeywordScan::Reserved(TerminalKind::WhileKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('y') => {
                            if scan_chars!(input, 'e', 'a', 'r', 's') {
                                KeywordScan::Reserved(TerminalKind::YearsKeyword)
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some(_) => {
                            input.undo();
                            KeywordScan::Absent
                        }
                        None => KeywordScan::Absent,
                    };
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        _ if input.position() < furthest_position => KeywordScan::Absent,
                        value => value,
                    };

                    // Perf: only scan for a compound keyword if we didn't already find one
                    let mut kw_scan = kw_scan;
                    if kw_scan == KeywordScan::Absent {
                        input.set_position(save);

                        let ident_len = furthest_position - save;

                        for keyword_compound_scanner in [
                            Self::bytes_keyword,
                            Self::fixed_keyword,
                            Self::int_keyword,
                            Self::ufixed_keyword,
                            Self::uint_keyword,
                        ] {
                            match keyword_compound_scanner(self, input, ident_len) {
                                _ if input.position() < furthest_position => { /* Strict prefix */ }
                                KeywordScan::Absent => {}
                                value => kw_scan = value,
                            }
                            input.set_position(save);
                        }
                    }

                    input.set_position(furthest_position);
                    return Some(ScannedTerminal::IdentifierOrKeyword {
                        identifier,
                        kw: kw_scan,
                    });
                }
            }
            LexicalContext::Pragma => {
                if let Some(kind) = match input.next() {
                    Some('-') => Some(TerminalKind::Minus),
                    Some('.') => Some(TerminalKind::Period),
                    Some(';') => Some(TerminalKind::Semicolon),
                    Some('<') => match input.next() {
                        Some('=') => Some(TerminalKind::LessThanEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::LessThan)
                        }
                        None => Some(TerminalKind::LessThan),
                    },
                    Some('=') => Some(TerminalKind::Equal),
                    Some('>') => match input.next() {
                        Some('=') => Some(TerminalKind::GreaterThanEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::GreaterThan)
                        }
                        None => Some(TerminalKind::GreaterThan),
                    },
                    Some('^') => Some(TerminalKind::Caret),
                    Some('|') => {
                        if scan_chars!(input, '|') {
                            Some(TerminalKind::BarBar)
                        } else {
                            None
                        }
                    }
                    Some('~') => Some(TerminalKind::Tilde),
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = input.position();
                    longest_terminal = Some(kind);
                }
                input.set_position(save);

                longest_match! {
                    { DoubleQuotedVersionLiteral = double_quoted_version_literal }
                    { SingleQuotedVersionLiteral = single_quoted_version_literal }
                    { VersionSpecifier = version_specifier }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {
                    { Identifier = identifier }
                }

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) =
                    longest_terminal.filter(|tok| [TerminalKind::Identifier].contains(tok))
                {
                    let kw_scan = match input.next() {
                        Some('A') => {
                            if scan_chars!(
                                input, 'B', 'I', 'E', 'n', 'c', 'o', 'd', 'e', 'r', 'V', '2'
                            ) {
                                if self.version_is_at_least_0_4_16 {
                                    KeywordScan::Present(TerminalKind::ABIEncoderV2Keyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('S') => {
                            if scan_chars!(input, 'M', 'T', 'C', 'h', 'e', 'c', 'k', 'e', 'r') {
                                if self.version_is_at_least_0_4_16 {
                                    KeywordScan::Present(TerminalKind::SMTCheckerKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('a') => {
                            if scan_chars!(input, 'b', 'i', 'c', 'o', 'd', 'e', 'r') {
                                if self.version_is_at_least_0_7_5 {
                                    KeywordScan::Present(TerminalKind::AbicoderKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('e') => {
                            if scan_chars!(
                                input, 'x', 'p', 'e', 'r', 'i', 'm', 'e', 'n', 't', 'a', 'l'
                            ) {
                                if self.version_is_at_least_0_4_16 {
                                    KeywordScan::Present(TerminalKind::ExperimentalKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('p') => {
                            if scan_chars!(input, 'r', 'a', 'g', 'm', 'a') {
                                KeywordScan::Reserved(TerminalKind::PragmaKeyword)
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('s') => {
                            if scan_chars!(input, 'o', 'l', 'i', 'd', 'i', 't', 'y') {
                                KeywordScan::Present(TerminalKind::SolidityKeyword)
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('v') => match input.next() {
                            Some('1') => {
                                if self.version_is_at_least_0_7_5 {
                                    KeywordScan::Present(TerminalKind::AbicoderV1Keyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('2') => {
                                if self.version_is_at_least_0_7_5 {
                                    KeywordScan::Present(TerminalKind::AbicoderV2Keyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some(_) => {
                            input.undo();
                            KeywordScan::Absent
                        }
                        None => KeywordScan::Absent,
                    };
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        _ if input.position() < furthest_position => KeywordScan::Absent,
                        value => value,
                    };

                    input.set_position(furthest_position);
                    return Some(ScannedTerminal::IdentifierOrKeyword {
                        identifier,
                        kw: kw_scan,
                    });
                }
            }
            LexicalContext::Yul => {
                if let Some(kind) = match input.next() {
                    Some('(') => Some(TerminalKind::OpenParen),
                    Some(')') => Some(TerminalKind::CloseParen),
                    Some(',') => Some(TerminalKind::Comma),
                    Some('-') => {
                        if scan_chars!(input, '>') {
                            Some(TerminalKind::MinusGreaterThan)
                        } else {
                            None
                        }
                    }
                    Some('.') => Some(TerminalKind::Period),
                    Some(':') => match input.next() {
                        Some('=') => Some(TerminalKind::ColonEqual),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Colon)
                        }
                        None => Some(TerminalKind::Colon),
                    },
                    Some('=') => match input.next() {
                        Some(':') => Some(TerminalKind::EqualColon),
                        Some(_) => {
                            input.undo();
                            Some(TerminalKind::Equal)
                        }
                        None => Some(TerminalKind::Equal),
                    },
                    Some('{') => Some(TerminalKind::OpenBrace),
                    Some('}') => Some(TerminalKind::CloseBrace),
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = input.position();
                    longest_terminal = Some(kind);
                }
                input.set_position(save);

                longest_match! {
                    { YulDecimalLiteral = yul_decimal_literal }
                    { YulHexLiteral = yul_hex_literal }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {
                    { YulIdentifier = yul_identifier }
                }

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) =
                    longest_terminal.filter(|tok| [TerminalKind::YulIdentifier].contains(tok))
                {
                    let kw_scan = match input.next() {
                        Some('a') => match input.next() {
                            Some('b') => {
                                if scan_chars!(input, 's', 't', 'r', 'a', 'c', 't') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulAbstractKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('f') => {
                                if scan_chars!(input, 't', 'e', 'r') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulAfterKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('l') => {
                                if scan_chars!(input, 'i', 'a', 's') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TerminalKind::YulAliasKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'o', 'n', 'y', 'm', 'o', 'u', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulAnonymousKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('p') => {
                                if scan_chars!(input, 'p', 'l', 'y') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TerminalKind::YulApplyKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => match input.next() {
                                Some('s') => {
                                    if scan_chars!(input, 'e', 'm', 'b', 'l', 'y') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulAssemblyKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulAsKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                None => {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulAsKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                            },
                            Some('u') => {
                                if scan_chars!(input, 't', 'o') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TerminalKind::YulAutoKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('b') => match input.next() {
                            Some('o') => {
                                if scan_chars!(input, 'o', 'l') {
                                    if !self.version_is_at_least_0_5_10 {
                                        KeywordScan::Reserved(TerminalKind::YulBoolKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'e', 'a', 'k') {
                                    KeywordScan::Reserved(TerminalKind::YulBreakKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('c') => match input.next() {
                            Some('a') => match input.next() {
                                Some('l') => {
                                    if scan_chars!(input, 'l', 'd', 'a', 't', 'a') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TerminalKind::YulCallDataKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('s') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TerminalKind::YulCaseKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulCatchKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('o') => match input.next() {
                                Some('n') => {
                                    match input.next() {
                                        Some('s') => {
                                            if scan_chars!(input, 't') {
                                                match input.next() {
                                                    Some('a') => {
                                                        if scan_chars!(input, 'n', 't') {
                                                            if !self.version_is_at_least_0_7_1 {
                                                                KeywordScan :: Reserved (TerminalKind :: YulConstantKeyword)
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        } else {
                                                            KeywordScan::Absent
                                                        }
                                                    }
                                                    Some('r') => {
                                                        if scan_chars!(
                                                            input, 'u', 'c', 't', 'o', 'r'
                                                        ) {
                                                            if self.version_is_at_least_0_5_0
                                                                && !self.version_is_at_least_0_7_1
                                                            {
                                                                KeywordScan :: Reserved (TerminalKind :: YulConstructorKeyword)
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        } else {
                                                            KeywordScan::Absent
                                                        }
                                                    }
                                                    Some(_) => {
                                                        input.undo();
                                                        KeywordScan::Absent
                                                    }
                                                    None => KeywordScan::Absent,
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('t') => match input.next() {
                                            Some('i') => {
                                                if scan_chars!(input, 'n', 'u', 'e') {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::YulContinueKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('r') => {
                                                if scan_chars!(input, 'a', 'c', 't') {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        KeywordScan::Reserved(
                                                            TerminalKind::YulContractKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Absent
                                            }
                                            None => KeywordScan::Absent,
                                        },
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                }
                                Some('p') => {
                                    if scan_chars!(input, 'y', 'o', 'f') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TerminalKind::YulCopyOfKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('d') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'y', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulDaysKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('e') => match input.next() {
                                Some('f') => match input.next() {
                                    Some('a') => {
                                        if scan_chars!(input, 'u', 'l', 't') {
                                            KeywordScan::Reserved(TerminalKind::YulDefaultKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'e') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                KeywordScan::Reserved(
                                                    TerminalKind::YulDefineKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some('l') => {
                                    if scan_chars!(input, 'e', 't', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulDeleteKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('o') => {
                                if !self.version_is_at_least_0_7_1 {
                                    KeywordScan::Reserved(TerminalKind::YulDoKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('e') => match input.next() {
                            Some('l') => {
                                if scan_chars!(input, 's', 'e') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulElseKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('m') => {
                                if scan_chars!(input, 'i', 't') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TerminalKind::YulEmitKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'u', 'm') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulEnumKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('t') => {
                                if scan_chars!(input, 'h', 'e', 'r') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulEtherKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('v') => {
                                if scan_chars!(input, 'e', 'n', 't') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulEventKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('x') => {
                                if scan_chars!(input, 't', 'e', 'r', 'n', 'a', 'l') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulExternalKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('f') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'l') {
                                    match input.next() {
                                        Some('l') => {
                                            if scan_chars!(input, 'b', 'a', 'c', 'k') {
                                                if self.version_is_at_least_0_6_0
                                                    && !self.version_is_at_least_0_7_1
                                                {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::YulFallbackKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('s') => {
                                            if scan_chars!(input, 'e') {
                                                KeywordScan::Reserved(TerminalKind::YulFalseKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n') {
                                    match input.next() {
                                        Some('a') => {
                                            if scan_chars!(input, 'l') {
                                                if !self.version_is_at_least_0_7_1 {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::YulFinalKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('n') => {
                                            if scan_chars!(input, 'e', 'y') {
                                                if !self.version_is_at_least_0_7_0 {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::YulFinneyKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'r') {
                                    KeywordScan::Reserved(TerminalKind::YulForKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'n', 'c', 't', 'i', 'o', 'n') {
                                    KeywordScan::Reserved(TerminalKind::YulFunctionKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('g') => {
                            if scan_chars!(input, 'w', 'e', 'i') {
                                if self.version_is_at_least_0_7_0 && !self.version_is_at_least_0_7_1
                                {
                                    KeywordScan::Reserved(TerminalKind::YulGweiKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('h') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'x') {
                                    KeywordScan::Reserved(TerminalKind::YulHexKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'u', 'r', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulHoursKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('i') => match input.next() {
                            Some('f') => KeywordScan::Reserved(TerminalKind::YulIfKeyword),
                            Some('m') => match input.next() {
                                Some('m') => {
                                    if scan_chars!(input, 'u', 't', 'a', 'b', 'l', 'e') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TerminalKind::YulImmutableKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('p') => match input.next() {
                                    Some('l') => {
                                        if scan_chars!(input, 'e', 'm', 'e', 'n', 't', 's') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                KeywordScan::Reserved(
                                                    TerminalKind::YulImplementsKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('o') => {
                                        if scan_chars!(input, 'r', 't') {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(
                                                    TerminalKind::YulImportKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('n') => match input.next() {
                                Some('d') => {
                                    if scan_chars!(input, 'e', 'x', 'e', 'd') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulIndexedKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('l') => {
                                    if scan_chars!(input, 'i', 'n', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulInlineKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'e', 'r') {
                                        match input.next() {
                                            Some('f') => {
                                                if scan_chars!(input, 'a', 'c', 'e') {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        KeywordScan::Reserved(
                                                            TerminalKind::YulInterfaceKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('n') => {
                                                if scan_chars!(input, 'a', 'l') {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        KeywordScan::Reserved(
                                                            TerminalKind::YulInternalKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Absent
                                            }
                                            None => KeywordScan::Absent,
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    if !self.version_is_at_least_0_6_8 {
                                        KeywordScan::Reserved(TerminalKind::YulInKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                None => {
                                    if !self.version_is_at_least_0_6_8 {
                                        KeywordScan::Reserved(TerminalKind::YulInKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                            },
                            Some('s') => {
                                if !self.version_is_at_least_0_7_1 {
                                    KeywordScan::Reserved(TerminalKind::YulIsKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('l') => match input.next() {
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'v', 'e') {
                                        if self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulLeaveKeyword)
                                        } else if self.version_is_at_least_0_6_0 {
                                            KeywordScan::Present(TerminalKind::YulLeaveKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => KeywordScan::Reserved(TerminalKind::YulLetKeyword),
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('i') => {
                                if scan_chars!(input, 'b', 'r', 'a', 'r', 'y') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulLibraryKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('m') => match input.next() {
                            Some('a') => match input.next() {
                                Some('c') => {
                                    if scan_chars!(input, 'r', 'o') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TerminalKind::YulMacroKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('p') => {
                                    if scan_chars!(input, 'p', 'i', 'n', 'g') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulMappingKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulMatchKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('e') => {
                                if scan_chars!(input, 'm', 'o', 'r', 'y') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulMemoryKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n', 'u', 't', 'e', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulMinutesKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'd', 'i', 'f', 'i', 'e', 'r') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulModifierKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 't', 'a', 'b', 'l', 'e') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TerminalKind::YulMutableKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('n') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'w') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulNewKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'l', 'l') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulNullKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('o') => match input.next() {
                            Some('f') => {
                                if !self.version_is_at_least_0_7_1 {
                                    KeywordScan::Reserved(TerminalKind::YulOfKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('v') => {
                                if scan_chars!(input, 'e', 'r', 'r', 'i', 'd', 'e') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TerminalKind::YulOverrideKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('p') => match input.next() {
                            Some('a') => match input.next() {
                                Some('r') => {
                                    if scan_chars!(input, 't', 'i', 'a', 'l') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TerminalKind::YulPartialKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('y') => {
                                    if scan_chars!(input, 'a', 'b', 'l', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulPayableKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('r') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'g', 'm', 'a') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulPragmaKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('i') => {
                                    if scan_chars!(input, 'v', 'a', 't', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulPrivateKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'm', 'i', 's', 'e') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TerminalKind::YulPromiseKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('u') => match input.next() {
                                Some('b') => {
                                    if scan_chars!(input, 'l', 'i', 'c') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulPublicKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulPureKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('r') => {
                            if scan_chars!(input, 'e') {
                                match input.next() {
                                    Some('c') => {
                                        if scan_chars!(input, 'e', 'i', 'v', 'e') {
                                            if self.version_is_at_least_0_6_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                KeywordScan::Reserved(
                                                    TerminalKind::YulReceiveKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('f') => {
                                        if scan_chars!(input, 'e', 'r', 'e', 'n', 'c', 'e') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                KeywordScan::Reserved(
                                                    TerminalKind::YulReferenceKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('l') => {
                                        if scan_chars!(
                                            input, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'
                                        ) {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(
                                                    TerminalKind::YulRelocatableKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('t') => {
                                        if scan_chars!(input, 'u', 'r', 'n', 's') {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(
                                                    TerminalKind::YulReturnsKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('s') => match input.next() {
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'l', 'e', 'd') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TerminalKind::YulSealedKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('c') => {
                                    if scan_chars!(input, 'o', 'n', 'd', 's') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulSecondsKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('i') => {
                                if scan_chars!(input, 'z', 'e', 'o', 'f') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TerminalKind::YulSizeOfKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('t') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 't', 'i', 'c') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulStaticKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'r', 'a', 'g', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulStorageKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => match input.next() {
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'g') {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(
                                                    TerminalKind::YulStringKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('u') => {
                                        if scan_chars!(input, 'c', 't') {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(
                                                    TerminalKind::YulStructKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('u') => {
                                if scan_chars!(input, 'p') {
                                    match input.next() {
                                        Some('e') => {
                                            if scan_chars!(input, 'r') {
                                                if self.version_is_at_least_0_8_0 {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::YulSuperKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('p') => {
                                            if scan_chars!(input, 'o', 'r', 't', 's') {
                                                if self.version_is_at_least_0_5_0
                                                    && !self.version_is_at_least_0_7_1
                                                {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::YulSupportsKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('w') => {
                                if scan_chars!(input, 'i', 't', 'c', 'h') {
                                    KeywordScan::Reserved(TerminalKind::YulSwitchKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('z') => {
                                if scan_chars!(input, 'a', 'b', 'o') {
                                    if !self.version_is_at_least_0_7_0 {
                                        KeywordScan::Reserved(TerminalKind::YulSzaboKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('t') => match input.next() {
                            Some('h') => match input.next() {
                                Some('i') => {
                                    if scan_chars!(input, 's') {
                                        if self.version_is_at_least_0_8_0 {
                                            KeywordScan::Reserved(TerminalKind::YulThisKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 'o', 'w') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulThrowKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('r') => match input.next() {
                                Some('u') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TerminalKind::YulTrueKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('y') => {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulTryKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('y') => {
                                if scan_chars!(input, 'p', 'e') {
                                    match input.next() {
                                        Some('d') => {
                                            if scan_chars!(input, 'e', 'f') {
                                                if self.version_is_at_least_0_5_0
                                                    && !self.version_is_at_least_0_7_1
                                                {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::YulTypeDefKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('o') => {
                                            if scan_chars!(input, 'f') {
                                                if !self.version_is_at_least_0_7_1 {
                                                    KeywordScan::Reserved(
                                                        TerminalKind::YulTypeOfKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(TerminalKind::YulTypeKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        None => {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(TerminalKind::YulTypeKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('u') => match input.next() {
                            Some('n') => {
                                if scan_chars!(input, 'c', 'h', 'e', 'c', 'k', 'e', 'd') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TerminalKind::YulUncheckedKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => {
                                if scan_chars!(input, 'i', 'n', 'g') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulUsingKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('v') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'r') {
                                    if !self.version_is_at_least_0_6_5 {
                                        KeywordScan::Reserved(TerminalKind::YulVarKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'w') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulViewKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 't', 'u', 'a', 'l') {
                                        if self.version_is_at_least_0_6_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TerminalKind::YulVirtualKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('w') => match input.next() {
                            Some('e') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'k', 's') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TerminalKind::YulWeeksKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('i') => {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulWeiKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('h') => {
                                if scan_chars!(input, 'i', 'l', 'e') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TerminalKind::YulWhileKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('y') => {
                            if scan_chars!(input, 'e', 'a', 'r', 's') {
                                if !self.version_is_at_least_0_7_1 {
                                    KeywordScan::Reserved(TerminalKind::YulYearsKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some(_) => {
                            input.undo();
                            KeywordScan::Absent
                        }
                        None => KeywordScan::Absent,
                    };
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        _ if input.position() < furthest_position => KeywordScan::Absent,
                        value => value,
                    };

                    // Perf: only scan for a compound keyword if we didn't already find one
                    let mut kw_scan = kw_scan;
                    if kw_scan == KeywordScan::Absent {
                        input.set_position(save);

                        let ident_len = furthest_position - save;

                        for keyword_compound_scanner in [
                            Self::yul_bytes_keyword,
                            Self::yul_fixed_keyword,
                            Self::yul_int_keyword,
                            Self::yul_ufixed_keyword,
                            Self::yul_uint_keyword,
                        ] {
                            match keyword_compound_scanner(self, input, ident_len) {
                                _ if input.position() < furthest_position => { /* Strict prefix */ }
                                KeywordScan::Absent => {}
                                value => kw_scan = value,
                            }
                            input.set_position(save);
                        }
                    }

                    input.set_position(furthest_position);
                    return Some(ScannedTerminal::IdentifierOrKeyword {
                        identifier,
                        kw: kw_scan,
                    });
                }
            }
        }

        match longest_terminal {
            Some(terminal) => {
                input.set_position(furthest_position);
                Some(ScannedTerminal::Single(terminal))
            }
            // Skip a character if possible and if we didn't recognize a terminal
            None if input.peek().is_some() => {
                let _ = input.next();
                Some(ScannedTerminal::Single(TerminalKind::UNRECOGNIZED))
            }
            None => None,
        }
    }
}
