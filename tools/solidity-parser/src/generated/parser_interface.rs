use chumsky::prelude::{BoxedParser, Simple};
pub type ErrorType = Simple<char>;
pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;
use super::tree_interface::*;

#[allow(dead_code)]
pub struct Parsers {
    pub decimal_integer: ParserType<DecimalInteger>,
    pub end_of_line: ParserType<EndOfLine>,
    pub fixed_bytes_type: ParserType<FixedBytesType>,
    pub fixed_type: ParserType<FixedType>,
    pub hex_byte_escape: ParserType<HexByteEscape>,
    pub hex_number: ParserType<HexNumber>,
    pub multiline_comment: ParserType<MultilineComment>,
    pub possibly_separated_pairs_of_hex_digits: ParserType<PossiblySeparatedPairsOfHexDigits>,
    pub pragma_directive: ParserType<PragmaDirective>,
    pub raw_identifier: ParserType<RawIdentifier>,
    pub signed_integer_type: ParserType<SignedIntegerType>,
    pub single_line_comment: ParserType<SingleLineComment>,
    pub unicode_escape: ParserType<UnicodeEscape>,
    pub whitespace: ParserType<Whitespace>,
    pub yul_decimal_number_literal: ParserType<YulDecimalNumberLiteral>,
    pub yul_hex_literal: ParserType<YulHexLiteral>,
    pub decimal_exponent: ParserType<DecimalExponent>,
    pub decimal_float: ParserType<DecimalFloat>,
    pub end_of_file_trivia: ParserType<EndOfFileTrivia>,
    pub escape_sequence: ParserType<EscapeSequence>,
    pub hex_string_literal: ParserType<HexStringLiteral>,
    pub leading_trivia: ParserType<LeadingTrivia>,
    pub trailing_trivia: ParserType<TrailingTrivia>,
    pub ufixed_type: ParserType<UfixedType>,
    pub unsigned_integer_type: ParserType<UnsignedIntegerType>,
    pub yul_identifier: ParserType<YulIdentifier>,
    pub break_statement: ParserType<BreakStatement>,
    pub continue_statement: ParserType<ContinueStatement>,
    pub decimal_number: ParserType<DecimalNumber>,
    pub double_quoted_ascii_string_literal: ParserType<DoubleQuotedAsciiStringLiteral>,
    pub double_quoted_unicode_string_literal: ParserType<DoubleQuotedUnicodeStringLiteral>,
    pub elementary_type: ParserType<ElementaryType>,
    pub keyword: ParserType<Keyword>,
    pub positional_argument_list: ParserType<PositionalArgumentList>,
    pub single_quoted_ascii_string_literal: ParserType<SingleQuotedAsciiStringLiteral>,
    pub single_quoted_unicode_string_literal: ParserType<SingleQuotedUnicodeStringLiteral>,
    pub unchecked_block: ParserType<UncheckedBlock>,
    pub yul_function_call: ParserType<YulFunctionCall>,
    pub yul_function_definition: ParserType<YulFunctionDefinition>,
    pub yul_path: ParserType<YulPath>,
    pub ascii_string_literal: ParserType<AsciiStringLiteral>,
    pub assembly_flags: ParserType<AssemblyFlags>,
    pub elementary_type_with_payable: ParserType<ElementaryTypeWithPayable>,
    pub elementary_type_without_payable: ParserType<ElementaryTypeWithoutPayable>,
    pub numeric_literal: ParserType<NumericLiteral>,
    pub reserved_word: ParserType<ReservedWord>,
    pub unicode_string_literal: ParserType<UnicodeStringLiteral>,
    pub identifier: ParserType<Identifier>,
    pub import_path: ParserType<ImportPath>,
    pub literal: ParserType<Literal>,
    pub yul_literal: ParserType<YulLiteral>,
    pub enum_definition: ParserType<EnumDefinition>,
    pub identifier_path: ParserType<IdentifierPath>,
    pub named_argument: ParserType<NamedArgument>,
    pub parameter_declaration: ParserType<ParameterDeclaration>,
    pub selected_import: ParserType<SelectedImport>,
    pub simple_import_directive: ParserType<SimpleImportDirective>,
    pub star_import_directive: ParserType<StarImportDirective>,
    pub user_defined_value_type_definition: ParserType<UserDefinedValueTypeDefinition>,
    pub yul_expression: ParserType<YulExpression>,
    pub mapping_type: ParserType<MappingType>,
    pub named_argument_list: ParserType<NamedArgumentList>,
    pub override_specifier: ParserType<OverrideSpecifier>,
    pub parameter_list: ParserType<ParameterList>,
    pub selecting_import_directive: ParserType<SelectingImportDirective>,
    pub yul_assignment: ParserType<YulAssignment>,
    pub yul_for_statement: ParserType<YulForStatement>,
    pub yul_if_statement: ParserType<YulIfStatement>,
    pub yul_switch_statement: ParserType<YulSwitchStatement>,
    pub yul_variable_declaration: ParserType<YulVariableDeclaration>,
    pub argument_list: ParserType<ArgumentList>,
    pub catch_clause: ParserType<CatchClause>,
    pub function_type: ParserType<FunctionType>,
    pub import_directive: ParserType<ImportDirective>,
    pub method_attribute: ParserType<MethodAttribute>,
    pub state_variable_attribute: ParserType<StateVariableAttribute>,
    pub yul_statement: ParserType<YulStatement>,
    pub inheritance_specifier: ParserType<InheritanceSpecifier>,
    pub modifier_invocation: ParserType<ModifierInvocation>,
    pub type_name: ParserType<TypeName>,
    pub yul_block: ParserType<YulBlock>,
    pub assembly_statement: ParserType<AssemblyStatement>,
    pub constructor_attribute: ParserType<ConstructorAttribute>,
    pub error_parameter: ParserType<ErrorParameter>,
    pub event_parameter: ParserType<EventParameter>,
    pub fallback_function_attribute: ParserType<FallbackFunctionAttribute>,
    pub function_attribute: ParserType<FunctionAttribute>,
    pub inheritance_specifier_list: ParserType<InheritanceSpecifierList>,
    pub primary_expression: ParserType<PrimaryExpression>,
    pub receive_function_attribute: ParserType<ReceiveFunctionAttribute>,
    pub struct_definition: ParserType<StructDefinition>,
    pub using_directive: ParserType<UsingDirective>,
    pub variable_declaration: ParserType<VariableDeclaration>,
    pub directive: ParserType<Directive>,
    pub error_definition: ParserType<ErrorDefinition>,
    pub event_definition: ParserType<EventDefinition>,
    pub index_access_expression: ParserType<IndexAccessExpression>,
    pub tuple_variable_declaration: ParserType<TupleVariableDeclaration>,
    pub member_access_expression: ParserType<MemberAccessExpression>,
    pub function_call_options_expression: ParserType<FunctionCallOptionsExpression>,
    pub function_call_expression: ParserType<FunctionCallExpression>,
    pub unary_prefix_expression: ParserType<UnaryPrefixExpression>,
    pub unary_suffix_expression: ParserType<UnarySuffixExpression>,
    pub exponentiation_expression: ParserType<ExponentiationExpression>,
    pub mul_div_mod_expression: ParserType<MulDivModExpression>,
    pub add_sub_expression: ParserType<AddSubExpression>,
    pub shift_expression: ParserType<ShiftExpression>,
    pub bit_and_expression: ParserType<BitAndExpression>,
    pub bit_x_or_expression: ParserType<BitXOrExpression>,
    pub bit_or_expression: ParserType<BitOrExpression>,
    pub order_comparison_expression: ParserType<OrderComparisonExpression>,
    pub equality_comparison_expression: ParserType<EqualityComparisonExpression>,
    pub and_expression: ParserType<AndExpression>,
    pub or_expression: ParserType<OrExpression>,
    pub conditional_expression: ParserType<ConditionalExpression>,
    pub assignment_expression: ParserType<AssignmentExpression>,
    pub expression: ParserType<Expression>,
    pub constant_definition: ParserType<ConstantDefinition>,
    pub do_while_statement: ParserType<DoWhileStatement>,
    pub emit_statement: ParserType<EmitStatement>,
    pub expression_statement: ParserType<ExpressionStatement>,
    pub if_statement: ParserType<IfStatement>,
    pub return_statement: ParserType<ReturnStatement>,
    pub revert_statement: ParserType<RevertStatement>,
    pub state_variable_declaration: ParserType<StateVariableDeclaration>,
    pub try_statement: ParserType<TryStatement>,
    pub variable_declaration_statement: ParserType<VariableDeclarationStatement>,
    pub while_statement: ParserType<WhileStatement>,
    pub simple_statement: ParserType<SimpleStatement>,
    pub for_statement: ParserType<ForStatement>,
    pub statement: ParserType<Statement>,
    pub block: ParserType<Block>,
    pub constructor_definition: ParserType<ConstructorDefinition>,
    pub fallback_function_definition: ParserType<FallbackFunctionDefinition>,
    pub function_definition: ParserType<FunctionDefinition>,
    pub modifier_definition: ParserType<ModifierDefinition>,
    pub receive_function_definition: ParserType<ReceiveFunctionDefinition>,
    pub contract_body_element: ParserType<ContractBodyElement>,
    pub contract_definition: ParserType<ContractDefinition>,
    pub interface_definition: ParserType<InterfaceDefinition>,
    pub library_definition: ParserType<LibraryDefinition>,
    pub definition: ParserType<Definition>,
    pub source_unit: ParserType<SourceUnit>,
}
