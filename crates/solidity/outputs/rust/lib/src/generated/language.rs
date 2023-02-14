// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

pub use std::ops::Range;
pub use std::rc::Rc;

#[allow(deprecated, unused_imports)]
use semver::Version;

pub use super::cst;
pub use super::kinds::*;

#[derive(PartialEq)]
pub struct ParseError {
    pub position: usize,
    pub expected: String,
}

impl ParseError {
    pub fn new<T: Into<String>>(position: usize, expected: T) -> Self {
        Self {
            position,
            expected: expected.into(),
        }
    }

    pub fn merge_with(&mut self, other: Self) {
        // LEAVE THIS: it is super useful for debugging

        // if self.position < other.position {
        //     self.expected = format!(
        //         "O={}\nNOT {}@[{}]",
        //         other.expected, self.position, self.expected
        //     );
        //     self.position = other.position;
        // } else if self.position == other.position {
        //     self.expected = format!("{}, or {}", other.expected, self.expected);
        // } else {
        //     self.expected = format!(
        //         "S={}\nNOT {}@[{}]",
        //         self.expected, other.position, other.expected
        //     );
        // }

        if self.position < other.position {
            *self = other;
        } else if self.position == other.position {
            self.expected = format!("{}, or {}", other.expected, self.expected);
        }
    }

    pub fn maybe_merge_with(mut self, other: Option<Self>) -> Self {
        if let Some(other) = other {
            self.merge_with(other)
        }
        self
    }
}

pub enum ParseResult {
    Pass {
        node: Rc<cst::Node>,
        error: Option<ParseError>,
    },
    Fail {
        error: ParseError,
    },
}

pub struct Stream<'s> {
    source: &'s str,
    position: usize,
    undo_position: usize,
    has_undo: bool,
}

impl<'s> Stream<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            position: 0,
            undo_position: 0,
            has_undo: false,
        }
    }

    pub fn source(&self) -> &'s str {
        self.source
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    pub fn next(&mut self) -> Option<char> {
        self.has_undo = true;
        self.undo_position = self.position;
        let mut chars = self.source[self.position..].chars();
        if let Some(c) = chars.next() {
            self.position += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    pub fn undo(&mut self) {
        if !self.has_undo {
            panic!("No undo position");
        }
        self.position = self.undo_position;
        self.has_undo = false;
    }
}

use ariadne::{Color, Config, Label, Report, ReportKind, Source};

pub(crate) fn render_error_report(
    error: &ParseError,
    source_id: &str,
    source: &str,
    with_color: bool,
) -> String {
    let kind = ReportKind::Error;
    let color = if with_color { Color::Red } else { Color::Unset };
    // LEAVE THIS: it is super useful for debugging
    // let message = format!("{}: Expected {}", error.position, error.expected);
    let message = format!("Expected {}", error.expected);
    let source_start = error.position;
    let source_end = error.position;
    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:{source_start}:{source_end}]");
    }
    let label = "Error occurred here.".to_string();
    let mut builder = Report::build(kind, source_id, source_start)
        .with_config(Config::default().with_color(with_color))
        .with_message(message);
    builder.add_label(
        Label::new((source_id, source_start..source_end))
            .with_color(color)
            .with_message(label),
    );
    let mut result = vec![];
    builder
        .finish()
        .write((source_id, Source::from(&source)), &mut result)
        .expect("Failed to write report");
    return String::from_utf8(result)
        .expect("Failed to convert report to utf8")
        .trim()
        .to_string();
}

pub use super::parser_output::ParserOutput;

pub struct Language {
    version: Version,
    pub(crate) version_is_equal_to_or_greater_than_0_6_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_18: bool,
}

impl Language {
    pub fn new(version: Version) -> Self {
        Self {
            version_is_equal_to_or_greater_than_0_6_0: Version::parse("0.6.0").unwrap() <= version,
            version_is_equal_to_or_greater_than_0_8_18: Version::parse("0.8.18").unwrap()
                <= version,
            version,
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn parse(&self, kind: ProductionKind, input: &str) -> ParserOutput {
        fn call_scanner<F>(
            language: &Language,
            input: &str,
            scanner: F,
            kind: TokenKind,
            error_message: &str,
        ) -> ParserOutput
        where
            F: Fn(&Language, &mut Stream) -> bool,
        {
            let mut stream = Stream::new(input);
            if scanner(language, &mut stream) && stream.peek().is_none() {
                ParserOutput {
                    parse_tree: Some(cst::Node::token(
                        kind,
                        Range {
                            start: 0,
                            end: stream.position(),
                        },
                        None,
                        None,
                    )),
                    errors: vec![],
                }
            } else {
                ParserOutput {
                    parse_tree: None,
                    errors: vec![ParseError::new(stream.position(), error_message)],
                }
            }
        }
        fn call_parser<F>(language: &Language, input: &str, parser: F) -> ParserOutput
        where
            F: Fn(&Language, &mut Stream) -> ParseResult,
        {
            let mut stream = Stream::new(input);
            match parser(language, &mut stream) {
                ParseResult::Pass { node, .. } if stream.peek().is_none() => ParserOutput {
                    parse_tree: Some(node),
                    errors: vec![],
                },
                ParseResult::Pass { .. } => ParserOutput {
                    parse_tree: None,
                    errors: vec![ParseError::new(stream.position(), "end of input")],
                },
                ParseResult::Fail { error } => ParserOutput {
                    parse_tree: None,
                    errors: vec![error],
                },
            }
        }
        match kind {
            ProductionKind::AsciiEscape => call_scanner(
                self,
                input,
                Language::scan_ascii_escape,
                TokenKind::AsciiEscape,
                "AsciiEscape",
            ),
            ProductionKind::AsciiStringLiteral => call_scanner(
                self,
                input,
                Language::scan_ascii_string_literal,
                TokenKind::AsciiStringLiteral,
                "AsciiStringLiteral",
            ),
            ProductionKind::BooleanLiteral => call_scanner(
                self,
                input,
                Language::scan_boolean_literal,
                TokenKind::BooleanLiteral,
                "BooleanLiteral",
            ),
            ProductionKind::DecimalExponent => call_scanner(
                self,
                input,
                Language::scan_decimal_exponent,
                TokenKind::DecimalExponent,
                "DecimalExponent",
            ),
            ProductionKind::DecimalLiteral => call_scanner(
                self,
                input,
                Language::scan_decimal_literal,
                TokenKind::DecimalLiteral,
                "DecimalLiteral",
            ),
            ProductionKind::DecimalNumber => call_scanner(
                self,
                input,
                Language::scan_decimal_number,
                TokenKind::DecimalNumber,
                "DecimalNumber",
            ),
            ProductionKind::DoubleQuotedAsciiStringLiteral => call_scanner(
                self,
                input,
                Language::scan_double_quoted_ascii_string_literal,
                TokenKind::DoubleQuotedAsciiStringLiteral,
                "DoubleQuotedAsciiStringLiteral",
            ),
            ProductionKind::DoubleQuotedUnicodeStringLiteral => call_scanner(
                self,
                input,
                Language::scan_double_quoted_unicode_string_literal,
                TokenKind::DoubleQuotedUnicodeStringLiteral,
                "DoubleQuotedUnicodeStringLiteral",
            ),
            ProductionKind::EndOfLine => call_scanner(
                self,
                input,
                Language::scan_end_of_line,
                TokenKind::EndOfLine,
                "EndOfLine",
            ),
            ProductionKind::EscapeSequence => call_scanner(
                self,
                input,
                Language::scan_escape_sequence,
                TokenKind::EscapeSequence,
                "EscapeSequence",
            ),
            ProductionKind::FixedBytesType => call_scanner(
                self,
                input,
                Language::scan_fixed_bytes_type,
                TokenKind::FixedBytesType,
                "FixedBytesType",
            ),
            ProductionKind::HexByteEscape => call_scanner(
                self,
                input,
                Language::scan_hex_byte_escape,
                TokenKind::HexByteEscape,
                "HexByteEscape",
            ),
            ProductionKind::HexCharacter => call_scanner(
                self,
                input,
                Language::scan_hex_character,
                TokenKind::HexCharacter,
                "HexCharacter",
            ),
            ProductionKind::HexLiteral => call_scanner(
                self,
                input,
                Language::scan_hex_literal,
                TokenKind::HexLiteral,
                "HexLiteral",
            ),
            ProductionKind::HexStringLiteral => call_scanner(
                self,
                input,
                Language::scan_hex_string_literal,
                TokenKind::HexStringLiteral,
                "HexStringLiteral",
            ),
            ProductionKind::Identifier => call_scanner(
                self,
                input,
                Language::scan_identifier,
                TokenKind::Identifier,
                "Identifier",
            ),
            ProductionKind::IdentifierPart => call_scanner(
                self,
                input,
                Language::scan_identifier_part,
                TokenKind::IdentifierPart,
                "IdentifierPart",
            ),
            ProductionKind::IdentifierStart => call_scanner(
                self,
                input,
                Language::scan_identifier_start,
                TokenKind::IdentifierStart,
                "IdentifierStart",
            ),
            ProductionKind::Keyword => call_scanner(
                self,
                input,
                Language::scan_keyword,
                TokenKind::Keyword,
                "Keyword",
            ),
            ProductionKind::MultilineComment => call_scanner(
                self,
                input,
                Language::scan_multiline_comment,
                TokenKind::MultilineComment,
                "MultilineComment",
            ),
            ProductionKind::NumberUnit => call_scanner(
                self,
                input,
                Language::scan_number_unit,
                TokenKind::NumberUnit,
                "NumberUnit",
            ),
            ProductionKind::PossiblySeparatedPairsOfHexDigits => call_scanner(
                self,
                input,
                Language::scan_possibly_separated_pairs_of_hex_digits,
                TokenKind::PossiblySeparatedPairsOfHexDigits,
                "PossiblySeparatedPairsOfHexDigits",
            ),
            ProductionKind::RawIdentifier => call_scanner(
                self,
                input,
                Language::scan_raw_identifier,
                TokenKind::RawIdentifier,
                "RawIdentifier",
            ),
            ProductionKind::ReservedKeyword => call_scanner(
                self,
                input,
                Language::scan_reserved_keyword,
                TokenKind::ReservedKeyword,
                "ReservedKeyword",
            ),
            ProductionKind::SignedFixedType => call_scanner(
                self,
                input,
                Language::scan_signed_fixed_type,
                TokenKind::SignedFixedType,
                "SignedFixedType",
            ),
            ProductionKind::SignedIntegerType => call_scanner(
                self,
                input,
                Language::scan_signed_integer_type,
                TokenKind::SignedIntegerType,
                "SignedIntegerType",
            ),
            ProductionKind::SingleLineComment => call_scanner(
                self,
                input,
                Language::scan_single_line_comment,
                TokenKind::SingleLineComment,
                "SingleLineComment",
            ),
            ProductionKind::SingleQuotedAsciiStringLiteral => call_scanner(
                self,
                input,
                Language::scan_single_quoted_ascii_string_literal,
                TokenKind::SingleQuotedAsciiStringLiteral,
                "SingleQuotedAsciiStringLiteral",
            ),
            ProductionKind::SingleQuotedUnicodeStringLiteral => call_scanner(
                self,
                input,
                Language::scan_single_quoted_unicode_string_literal,
                TokenKind::SingleQuotedUnicodeStringLiteral,
                "SingleQuotedUnicodeStringLiteral",
            ),
            ProductionKind::UnicodeEscape => call_scanner(
                self,
                input,
                Language::scan_unicode_escape,
                TokenKind::UnicodeEscape,
                "UnicodeEscape",
            ),
            ProductionKind::UnicodeStringLiteral => call_scanner(
                self,
                input,
                Language::scan_unicode_string_literal,
                TokenKind::UnicodeStringLiteral,
                "UnicodeStringLiteral",
            ),
            ProductionKind::UnsignedFixedType => call_scanner(
                self,
                input,
                Language::scan_unsigned_fixed_type,
                TokenKind::UnsignedFixedType,
                "UnsignedFixedType",
            ),
            ProductionKind::UnsignedIntegerType => call_scanner(
                self,
                input,
                Language::scan_unsigned_integer_type,
                TokenKind::UnsignedIntegerType,
                "UnsignedIntegerType",
            ),
            ProductionKind::VersionPragmaOperator => call_scanner(
                self,
                input,
                Language::scan_version_pragma_operator,
                TokenKind::VersionPragmaOperator,
                "VersionPragmaOperator",
            ),
            ProductionKind::VersionPragmaValue => call_scanner(
                self,
                input,
                Language::scan_version_pragma_value,
                TokenKind::VersionPragmaValue,
                "VersionPragmaValue",
            ),
            ProductionKind::Whitespace => call_scanner(
                self,
                input,
                Language::scan_whitespace,
                TokenKind::Whitespace,
                "Whitespace",
            ),
            ProductionKind::YulDecimalLiteral => call_scanner(
                self,
                input,
                Language::scan_yul_decimal_literal,
                TokenKind::YulDecimalLiteral,
                "YulDecimalLiteral",
            ),
            ProductionKind::YulHexLiteral => call_scanner(
                self,
                input,
                Language::scan_yul_hex_literal,
                TokenKind::YulHexLiteral,
                "YulHexLiteral",
            ),
            ProductionKind::YulIdentifier => call_scanner(
                self,
                input,
                Language::scan_yul_identifier,
                TokenKind::YulIdentifier,
                "YulIdentifier",
            ),
            ProductionKind::YulKeyword => call_scanner(
                self,
                input,
                Language::scan_yul_keyword,
                TokenKind::YulKeyword,
                "YulKeyword",
            ),
            ProductionKind::ABICoderPragma => {
                call_parser(self, input, Language::parse_abi_coder_pragma)
            }
            ProductionKind::AddSubExpression => {
                call_parser(self, input, Language::parse_add_sub_expression)
            }
            ProductionKind::AddressType => call_parser(self, input, Language::parse_address_type),
            ProductionKind::AndExpression => {
                call_parser(self, input, Language::parse_and_expression)
            }
            ProductionKind::ArgumentList => call_parser(self, input, Language::parse_argument_list),
            ProductionKind::ArrayLiteral => call_parser(self, input, Language::parse_array_literal),
            ProductionKind::AssemblyFlags => {
                call_parser(self, input, Language::parse_assembly_flags)
            }
            ProductionKind::AssemblyStatement => {
                call_parser(self, input, Language::parse_assembly_statement)
            }
            ProductionKind::AssignmentExpression => {
                call_parser(self, input, Language::parse_assignment_expression)
            }
            ProductionKind::BitAndExpression => {
                call_parser(self, input, Language::parse_bit_and_expression)
            }
            ProductionKind::BitOrExpression => {
                call_parser(self, input, Language::parse_bit_or_expression)
            }
            ProductionKind::BitXOrExpression => {
                call_parser(self, input, Language::parse_bit_x_or_expression)
            }
            ProductionKind::Block => call_parser(self, input, Language::parse_block),
            ProductionKind::BreakStatement => {
                call_parser(self, input, Language::parse_break_statement)
            }
            ProductionKind::CatchClause => call_parser(self, input, Language::parse_catch_clause),
            ProductionKind::ConditionalExpression => {
                call_parser(self, input, Language::parse_conditional_expression)
            }
            ProductionKind::ConstantDefinition => {
                call_parser(self, input, Language::parse_constant_definition)
            }
            ProductionKind::ConstructorAttribute => {
                call_parser(self, input, Language::parse_constructor_attribute)
            }
            ProductionKind::ConstructorDefinition => {
                call_parser(self, input, Language::parse_constructor_definition)
            }
            ProductionKind::ContinueStatement => {
                call_parser(self, input, Language::parse_continue_statement)
            }
            ProductionKind::ContractBodyElement => {
                call_parser(self, input, Language::parse_contract_body_element)
            }
            ProductionKind::ContractDefinition => {
                call_parser(self, input, Language::parse_contract_definition)
            }
            ProductionKind::DataLocation => call_parser(self, input, Language::parse_data_location),
            ProductionKind::Definition => call_parser(self, input, Language::parse_definition),
            ProductionKind::DeleteStatement => {
                call_parser(self, input, Language::parse_delete_statement)
            }
            ProductionKind::Directive => call_parser(self, input, Language::parse_directive),
            ProductionKind::DoWhileStatement => {
                call_parser(self, input, Language::parse_do_while_statement)
            }
            ProductionKind::ElementaryType => {
                call_parser(self, input, Language::parse_elementary_type)
            }
            ProductionKind::EmitStatement => {
                call_parser(self, input, Language::parse_emit_statement)
            }
            ProductionKind::EndOfFileTrivia => {
                call_parser(self, input, Language::parse_end_of_file_trivia)
            }
            ProductionKind::EnumDefinition => {
                call_parser(self, input, Language::parse_enum_definition)
            }
            ProductionKind::EqualityComparisonExpression => {
                call_parser(self, input, Language::parse_equality_comparison_expression)
            }
            ProductionKind::ErrorDefinition => {
                call_parser(self, input, Language::parse_error_definition)
            }
            ProductionKind::ErrorParameter => {
                call_parser(self, input, Language::parse_error_parameter)
            }
            ProductionKind::EventDefinition => {
                call_parser(self, input, Language::parse_event_definition)
            }
            ProductionKind::EventParameter => {
                call_parser(self, input, Language::parse_event_parameter)
            }
            ProductionKind::ExperimentalPragma => {
                call_parser(self, input, Language::parse_experimental_pragma)
            }
            ProductionKind::ExponentiationExpression => {
                call_parser(self, input, Language::parse_exponentiation_expression)
            }
            ProductionKind::Expression => call_parser(self, input, Language::parse_expression),
            ProductionKind::ExpressionStatement => {
                call_parser(self, input, Language::parse_expression_statement)
            }
            ProductionKind::FallbackFunctionAttribute => {
                call_parser(self, input, Language::parse_fallback_function_attribute)
            }
            ProductionKind::FallbackFunctionDefinition => {
                call_parser(self, input, Language::parse_fallback_function_definition)
            }
            ProductionKind::ForStatement => call_parser(self, input, Language::parse_for_statement),
            ProductionKind::FunctionAttribute => {
                call_parser(self, input, Language::parse_function_attribute)
            }
            ProductionKind::FunctionCallExpression => {
                call_parser(self, input, Language::parse_function_call_expression)
            }
            ProductionKind::FunctionDefinition => {
                call_parser(self, input, Language::parse_function_definition)
            }
            ProductionKind::FunctionType => call_parser(self, input, Language::parse_function_type),
            ProductionKind::IdentifierPath => {
                call_parser(self, input, Language::parse_identifier_path)
            }
            ProductionKind::IfStatement => call_parser(self, input, Language::parse_if_statement),
            ProductionKind::ImportDirective => {
                call_parser(self, input, Language::parse_import_directive)
            }
            ProductionKind::ImportPath => call_parser(self, input, Language::parse_import_path),
            ProductionKind::IndexAccessExpression => {
                call_parser(self, input, Language::parse_index_access_expression)
            }
            ProductionKind::InheritanceSpecifier => {
                call_parser(self, input, Language::parse_inheritance_specifier)
            }
            ProductionKind::InheritanceSpecifierList => {
                call_parser(self, input, Language::parse_inheritance_specifier_list)
            }
            ProductionKind::InterfaceDefinition => {
                call_parser(self, input, Language::parse_interface_definition)
            }
            ProductionKind::LeadingTrivia => {
                call_parser(self, input, Language::parse_leading_trivia)
            }
            ProductionKind::LibraryDefinition => {
                call_parser(self, input, Language::parse_library_definition)
            }
            ProductionKind::MappingKeyType => {
                call_parser(self, input, Language::parse_mapping_key_type)
            }
            ProductionKind::MappingType => call_parser(self, input, Language::parse_mapping_type),
            ProductionKind::MappingValueType => {
                call_parser(self, input, Language::parse_mapping_value_type)
            }
            ProductionKind::MemberAccessExpression => {
                call_parser(self, input, Language::parse_member_access_expression)
            }
            ProductionKind::ModifierAttribute => {
                call_parser(self, input, Language::parse_modifier_attribute)
            }
            ProductionKind::ModifierDefinition => {
                call_parser(self, input, Language::parse_modifier_definition)
            }
            ProductionKind::ModifierInvocation => {
                call_parser(self, input, Language::parse_modifier_invocation)
            }
            ProductionKind::MulDivModExpression => {
                call_parser(self, input, Language::parse_mul_div_mod_expression)
            }
            ProductionKind::NamedArgument => {
                call_parser(self, input, Language::parse_named_argument)
            }
            ProductionKind::NamedArgumentList => {
                call_parser(self, input, Language::parse_named_argument_list)
            }
            ProductionKind::NewExpression => {
                call_parser(self, input, Language::parse_new_expression)
            }
            ProductionKind::NumericLiteral => {
                call_parser(self, input, Language::parse_numeric_literal)
            }
            ProductionKind::OrExpression => call_parser(self, input, Language::parse_or_expression),
            ProductionKind::OrderComparisonExpression => {
                call_parser(self, input, Language::parse_order_comparison_expression)
            }
            ProductionKind::OverrideSpecifier => {
                call_parser(self, input, Language::parse_override_specifier)
            }
            ProductionKind::ParameterDeclaration => {
                call_parser(self, input, Language::parse_parameter_declaration)
            }
            ProductionKind::ParameterList => {
                call_parser(self, input, Language::parse_parameter_list)
            }
            ProductionKind::PayableType => call_parser(self, input, Language::parse_payable_type),
            ProductionKind::PositionalArgumentList => {
                call_parser(self, input, Language::parse_positional_argument_list)
            }
            ProductionKind::PragmaDirective => {
                call_parser(self, input, Language::parse_pragma_directive)
            }
            ProductionKind::PrimaryExpression => {
                call_parser(self, input, Language::parse_primary_expression)
            }
            ProductionKind::ReceiveFunctionAttribute => {
                call_parser(self, input, Language::parse_receive_function_attribute)
            }
            ProductionKind::ReceiveFunctionDefinition => {
                call_parser(self, input, Language::parse_receive_function_definition)
            }
            ProductionKind::ReturnStatement => {
                call_parser(self, input, Language::parse_return_statement)
            }
            ProductionKind::RevertStatement => {
                call_parser(self, input, Language::parse_revert_statement)
            }
            ProductionKind::SelectedImport => {
                call_parser(self, input, Language::parse_selected_import)
            }
            ProductionKind::SelectingImportDirective => {
                call_parser(self, input, Language::parse_selecting_import_directive)
            }
            ProductionKind::ShiftExpression => {
                call_parser(self, input, Language::parse_shift_expression)
            }
            ProductionKind::SimpleImportDirective => {
                call_parser(self, input, Language::parse_simple_import_directive)
            }
            ProductionKind::SimpleStatement => {
                call_parser(self, input, Language::parse_simple_statement)
            }
            ProductionKind::SourceUnit => call_parser(self, input, Language::parse_source_unit),
            ProductionKind::StarImportDirective => {
                call_parser(self, input, Language::parse_star_import_directive)
            }
            ProductionKind::StateVariableAttribute => {
                call_parser(self, input, Language::parse_state_variable_attribute)
            }
            ProductionKind::StateVariableDeclaration => {
                call_parser(self, input, Language::parse_state_variable_declaration)
            }
            ProductionKind::Statement => call_parser(self, input, Language::parse_statement),
            ProductionKind::StringExpression => {
                call_parser(self, input, Language::parse_string_expression)
            }
            ProductionKind::StructDefinition => {
                call_parser(self, input, Language::parse_struct_definition)
            }
            ProductionKind::StructMember => call_parser(self, input, Language::parse_struct_member),
            ProductionKind::TrailingTrivia => {
                call_parser(self, input, Language::parse_trailing_trivia)
            }
            ProductionKind::TryStatement => call_parser(self, input, Language::parse_try_statement),
            ProductionKind::TupleDeconstructionStatement => {
                call_parser(self, input, Language::parse_tuple_deconstruction_statement)
            }
            ProductionKind::TupleExpression => {
                call_parser(self, input, Language::parse_tuple_expression)
            }
            ProductionKind::TypeExpression => {
                call_parser(self, input, Language::parse_type_expression)
            }
            ProductionKind::TypeName => call_parser(self, input, Language::parse_type_name),
            ProductionKind::UnaryPrefixExpression => {
                call_parser(self, input, Language::parse_unary_prefix_expression)
            }
            ProductionKind::UnarySuffixExpression => {
                call_parser(self, input, Language::parse_unary_suffix_expression)
            }
            ProductionKind::UncheckedBlock => {
                call_parser(self, input, Language::parse_unchecked_block)
            }
            ProductionKind::UserDefinedValueTypeDefinition => call_parser(
                self,
                input,
                Language::parse_user_defined_value_type_definition,
            ),
            ProductionKind::UsingDirective => {
                call_parser(self, input, Language::parse_using_directive)
            }
            ProductionKind::VariableDeclarationStatement => {
                call_parser(self, input, Language::parse_variable_declaration_statement)
            }
            ProductionKind::VersionPragma => {
                call_parser(self, input, Language::parse_version_pragma)
            }
            ProductionKind::VersionPragmaSpecifier => {
                call_parser(self, input, Language::parse_version_pragma_specifier)
            }
            ProductionKind::WhileStatement => {
                call_parser(self, input, Language::parse_while_statement)
            }
            ProductionKind::YulAssignmentStatement => {
                call_parser(self, input, Language::parse_yul_assignment_statement)
            }
            ProductionKind::YulBlock => call_parser(self, input, Language::parse_yul_block),
            ProductionKind::YulBreakStatement => {
                call_parser(self, input, Language::parse_yul_break_statement)
            }
            ProductionKind::YulContinueStatement => {
                call_parser(self, input, Language::parse_yul_continue_statement)
            }
            ProductionKind::YulExpression => {
                call_parser(self, input, Language::parse_yul_expression)
            }
            ProductionKind::YulForStatement => {
                call_parser(self, input, Language::parse_yul_for_statement)
            }
            ProductionKind::YulFunctionCall => {
                call_parser(self, input, Language::parse_yul_function_call)
            }
            ProductionKind::YulFunctionDefinition => {
                call_parser(self, input, Language::parse_yul_function_definition)
            }
            ProductionKind::YulIdentifierPath => {
                call_parser(self, input, Language::parse_yul_identifier_path)
            }
            ProductionKind::YulIfStatement => {
                call_parser(self, input, Language::parse_yul_if_statement)
            }
            ProductionKind::YulLeaveStatement => {
                call_parser(self, input, Language::parse_yul_leave_statement)
            }
            ProductionKind::YulLiteral => call_parser(self, input, Language::parse_yul_literal),
            ProductionKind::YulStatement => call_parser(self, input, Language::parse_yul_statement),
            ProductionKind::YulSwitchStatement => {
                call_parser(self, input, Language::parse_yul_switch_statement)
            }
            ProductionKind::YulVariableDeclaration => {
                call_parser(self, input, Language::parse_yul_variable_declaration)
            }
        }
    }
}
