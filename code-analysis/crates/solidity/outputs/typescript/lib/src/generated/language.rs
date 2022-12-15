// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{
    cst,
    cst::RcNodeExtensions as CSTRcNodeExtensions,
    parse::{BoxedParserType, ErrorType, Parsers, SpanType},
};
use ariadne::{Color, Config, Fmt, Label, Report, ReportBuilder, ReportKind, Source};
use chumsky::{error::SimpleReason, Parser, Span};
use napi::bindgen_prelude::*;
use semver::Version;
use std::rc::Rc;
#[napi]
pub struct Language {
    parsers: Parsers,
    version: Version,
}
#[napi]
impl Language {
    #[napi(constructor)]
    pub fn new(version: String) -> Self {
        let version = Version::parse(&version).unwrap();
        Self {
            parsers: Parsers::new(&version),
            version,
        }
    }
    #[napi]
    pub fn version(&self) -> String {
        self.version.to_string()
    }
}
#[napi]
pub struct ParserOutput {
    parse_tree: Option<Rc<cst::Node>>,
    errors: Vec<ErrorType>,
}
#[napi]
impl ParserOutput {
    fn new(source: String, parser: &BoxedParserType) -> Self {
        let (parse_tree, errors) = parser.parse_recovery(source.as_str());
        Self { parse_tree, errors }
    }
    #[napi(ts_return_type = "CSTRuleNode | CSTTokenNode | null")]
    pub fn parse_tree(&self, env: Env) -> Option<napi::JsObject> {
        self.parse_tree.clone().map(|n| n.to_js(&env))
    }
    #[napi]
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
    #[napi]
    pub fn errors_as_strings(&self, source: String, with_colour: bool) -> Vec<String> {
        let mut results = vec![];
        for error in &self.errors {
            let report = render_error_report(&error, with_colour);
            let mut result = vec![];
            report
                .write(Source::from(source.as_str()), &mut result)
                .expect("Failed to write report");
            let result = String::from_utf8(result).expect("Failed to convert report to utf8");
            results.push(result);
        }
        results
    }
    #[napi]
    pub fn is_valid(&self) -> bool {
        self.parse_tree.is_some() && self.errors.is_empty()
    }
}
fn render_error_report(error: &ErrorType, with_color: bool) -> Report<SpanType> {
    let mut builder: ReportBuilder<SpanType> = Report::build(
        ReportKind::Error,
        error.span().context(),
        error.span().start(),
    )
    .with_config(Config::default().with_color(with_color));
    let error_color = if with_color { Color::Red } else { Color::Unset };
    builder = match error.reason() {
        SimpleReason::Custom(message) => builder.with_message(message),
        SimpleReason::Unclosed { span, delimiter } => {
            builder.add_label(
                Label::<SpanType>::new(span.to_owned())
                    .with_color(error_color)
                    .with_message("Unclosed delimiter"),
            );
            builder.with_message(format!(
                "Expected delimiter '{}' to be closed",
                delimiter.fg(error_color)
            ))
        }
        SimpleReason::Unexpected => {
            let found = if let Some(found) = error.found() {
                format!("'{}'", found.fg(error_color))
            } else {
                "end of input".to_string()
            };
            builder.add_label(
                Label::<SpanType>::new(error.span())
                    .with_color(error_color)
                    .with_message(format!("Found {found}.")),
            );
            let mut expected: Vec<&Option<char>> = error.expected().collect();
            expected.sort();
            let expected = if expected.len() == 0 {
                "something else".to_string()
            } else {
                expected
                    .iter()
                    .map(|expected| match expected {
                        Some(expected) => format!("'{}'", expected),
                        None => "end of input".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" or ")
            };
            builder.with_message(format!("Expected {expected}."))
        }
    };
    return builder.finish();
}

#[napi]
impl Language {
    // ABICoderPragma = 'abicoder' «Identifier» ;
    #[napi]
    pub fn parse_abi_coder_pragma(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.abi_coder_pragma)
    }

    // AddSubExpression = Expression ( '+' | '-' ) Expression ;
    #[napi]
    pub fn parse_add_sub_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.add_sub_expression)
    }

    // AddressType = 'address' [ 'payable' ] ;
    #[napi]
    pub fn parse_address_type(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.address_type)
    }

    // AndExpression = Expression '&&' Expression ;
    #[napi]
    pub fn parse_and_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.and_expression)
    }

    // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
    #[napi]
    pub fn parse_argument_list(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.argument_list)
    }

    // ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
    #[napi]
    pub fn parse_array_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.array_literal)
    }

    // «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
    #[napi]
    pub fn parse_ascii_escape(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.ascii_escape)
    }

    // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
    #[napi]
    pub fn parse_ascii_string_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.ascii_string_literal)
    }

    // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
    #[napi]
    pub fn parse_assembly_flags(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.assembly_flags)
    }

    // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
    #[napi]
    pub fn parse_assembly_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.assembly_statement)
    }

    // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
    #[napi]
    pub fn parse_assignment_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.assignment_expression)
    }

    // BitAndExpression = Expression '&' Expression ;
    #[napi]
    pub fn parse_bit_and_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.bit_and_expression)
    }

    // BitOrExpression = Expression '|' Expression ;
    #[napi]
    pub fn parse_bit_or_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.bit_or_expression)
    }

    // BitXOrExpression = Expression '^' Expression ;
    #[napi]
    pub fn parse_bit_x_or_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.bit_x_or_expression)
    }

    // Block = '{' { Statement | UncheckedBlock } '}' ;
    #[napi]
    pub fn parse_block(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.block)
    }

    // «BooleanLiteral» = 'true' | 'false' ;
    #[napi]
    pub fn parse_boolean_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.boolean_literal)
    }

    // BreakStatement = 'break' ';' ;
    #[napi]
    pub fn parse_break_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.break_statement)
    }

    // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
    #[napi]
    pub fn parse_catch_clause(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.catch_clause)
    }

    // ConditionalExpression = Expression '?' Expression ':' Expression ;
    #[napi]
    pub fn parse_conditional_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.conditional_expression)
    }

    // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
    #[napi]
    pub fn parse_constant_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.constant_definition)
    }

    // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
    #[napi]
    pub fn parse_constructor_attribute(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.constructor_attribute)
    }

    // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
    #[napi]
    pub fn parse_constructor_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.constructor_definition)
    }

    // ContinueStatement = 'continue' ';' ;
    #[napi]
    pub fn parse_continue_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.continue_statement)
    }

    // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
    #[napi]
    pub fn parse_contract_body_element(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.contract_body_element)
    }

    // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    #[napi]
    pub fn parse_contract_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.contract_definition)
    }

    // DataLocation = 'memory' | 'storage' | 'calldata' ;
    #[napi]
    pub fn parse_data_location(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.data_location)
    }

    // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
    #[napi]
    pub fn parse_decimal_exponent(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.decimal_exponent)
    }

    // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
    #[napi]
    pub fn parse_decimal_float(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.decimal_float)
    }

    // «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
    #[napi]
    pub fn parse_decimal_integer(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.decimal_integer)
    }

    // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
    #[napi]
    pub fn parse_decimal_number(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.decimal_number)
    }

    // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
    #[napi]
    pub fn parse_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.definition)
    }

    // DeleteStatement = 'delete' «Identifier» ';' ;
    #[napi]
    pub fn parse_delete_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.delete_statement)
    }

    // Directive = PragmaDirective | ImportDirective | UsingDirective ;
    #[napi]
    pub fn parse_directive(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.directive)
    }

    // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
    #[napi]
    pub fn parse_do_while_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.do_while_statement)
    }

    // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
    #[napi]
    pub fn parse_double_quoted_ascii_string_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.double_quoted_ascii_string_literal)
    }

    // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
    #[napi]
    pub fn parse_double_quoted_unicode_string_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.double_quoted_unicode_string_literal)
    }

    // ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
    #[napi]
    pub fn parse_elementary_type(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.elementary_type)
    }

    // EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
    #[napi]
    pub fn parse_emit_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.emit_statement)
    }

    // EndOfFileTrivia = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
    #[napi]
    pub fn parse_end_of_file_trivia(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.end_of_file_trivia)
    }

    // «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
    #[napi]
    pub fn parse_end_of_line(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.end_of_line)
    }

    // EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
    #[napi]
    pub fn parse_enum_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.enum_definition)
    }

    // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
    #[napi]
    pub fn parse_equality_comparison_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.equality_comparison_expression)
    }

    // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
    #[napi]
    pub fn parse_error_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.error_definition)
    }

    // ErrorParameter = TypeName [ «Identifier» ] ;
    #[napi]
    pub fn parse_error_parameter(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.error_parameter)
    }

    // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
    #[napi]
    pub fn parse_escape_sequence(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.escape_sequence)
    }

    // EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
    #[napi]
    pub fn parse_event_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.event_definition)
    }

    // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
    #[napi]
    pub fn parse_event_parameter(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.event_parameter)
    }

    // ExperimentalPragma = 'experimental' «Identifier» ;
    #[napi]
    pub fn parse_experimental_pragma(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.experimental_pragma)
    }

    // (* 0.4.11 *) ExponentiationExpression = Expression '**' Expression ;
    // (* 0.6.0 *) ExponentiationExpression = Expression '**' Expression ;
    #[napi]
    pub fn parse_exponentiation_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.exponentiation_expression)
    }

    // Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    #[napi]
    pub fn parse_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.expression)
    }

    // ExpressionStatement = Expression ';' ;
    #[napi]
    pub fn parse_expression_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.expression_statement)
    }

    // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
    #[napi]
    pub fn parse_fallback_function_attribute(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.fallback_function_attribute)
    }

    // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    #[napi]
    pub fn parse_fallback_function_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.fallback_function_definition)
    }

    // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
    #[napi]
    pub fn parse_fixed_bytes_type(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.fixed_bytes_type)
    }

    // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
    #[napi]
    pub fn parse_for_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.for_statement)
    }

    // FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
    #[napi]
    pub fn parse_function_attribute(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.function_attribute)
    }

    // FunctionCallExpression = Expression [ '{' NamedArgument  { ',' NamedArgument } '}' ] ArgumentList ;
    #[napi]
    pub fn parse_function_call_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.function_call_expression)
    }

    // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    #[napi]
    pub fn parse_function_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.function_definition)
    }

    // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
    #[napi]
    pub fn parse_function_type(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.function_type)
    }

    // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
    #[napi]
    pub fn parse_hex_byte_escape(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.hex_byte_escape)
    }

    // «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
    #[napi]
    pub fn parse_hex_character(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.hex_character)
    }

    // «HexNumber» = '0x' «HexCharacter» { [ '_' ] «HexCharacter» } ;
    #[napi]
    pub fn parse_hex_number(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.hex_number)
    }

    // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
    #[napi]
    pub fn parse_hex_string_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.hex_string_literal)
    }

    // «Identifier» = «RawIdentifier» - «Keyword» ;
    #[napi]
    pub fn parse_identifier(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.identifier)
    }

    // «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
    #[napi]
    pub fn parse_identifier_part(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.identifier_part)
    }

    // IdentifierPath = «Identifier»  { '.' «Identifier» } ;
    #[napi]
    pub fn parse_identifier_path(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.identifier_path)
    }

    // «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
    #[napi]
    pub fn parse_identifier_start(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.identifier_start)
    }

    // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
    #[napi]
    pub fn parse_if_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.if_statement)
    }

    // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
    #[napi]
    pub fn parse_import_directive(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.import_directive)
    }

    // ImportPath = «AsciiStringLiteral» ;
    #[napi]
    pub fn parse_import_path(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.import_path)
    }

    // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
    #[napi]
    pub fn parse_index_access_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.index_access_expression)
    }

    // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
    #[napi]
    pub fn parse_inheritance_specifier(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.inheritance_specifier)
    }

    // InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
    #[napi]
    pub fn parse_inheritance_specifier_list(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.inheritance_specifier_list)
    }

    // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    #[napi]
    pub fn parse_interface_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.interface_definition)
    }

    // «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
    #[napi]
    pub fn parse_keyword(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.keyword)
    }

    // LeadingTrivia = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
    #[napi]
    pub fn parse_leading_trivia(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.leading_trivia)
    }

    // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
    #[napi]
    pub fn parse_library_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.library_definition)
    }

    // MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
    #[napi]
    pub fn parse_mapping_type(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.mapping_type)
    }

    // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
    #[napi]
    pub fn parse_member_access_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.member_access_expression)
    }

    // ModifierAttribute = OverrideSpecifier | 'virtual' ;
    #[napi]
    pub fn parse_modifier_attribute(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.modifier_attribute)
    }

    // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
    #[napi]
    pub fn parse_modifier_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.modifier_definition)
    }

    // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
    #[napi]
    pub fn parse_modifier_invocation(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.modifier_invocation)
    }

    // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
    #[napi]
    pub fn parse_mul_div_mod_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.mul_div_mod_expression)
    }

    // «MultilineComment» = '/*' { ¬'*' | '*' ¬'/' } '*/' ;
    #[napi]
    pub fn parse_multiline_comment(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.multiline_comment)
    }

    // NamedArgument = «Identifier» ':' Expression ;
    #[napi]
    pub fn parse_named_argument(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.named_argument)
    }

    // NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
    #[napi]
    pub fn parse_named_argument_list(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.named_argument_list)
    }

    // NewExpression = 'new' IdentifierPath ArgumentList ;
    #[napi]
    pub fn parse_new_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.new_expression)
    }

    // «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
    #[napi]
    pub fn parse_number_unit(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.number_unit)
    }

    // «NumericLiteral» = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
    #[napi]
    pub fn parse_numeric_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.numeric_literal)
    }

    // OrExpression = Expression '||' Expression ;
    #[napi]
    pub fn parse_or_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.or_expression)
    }

    // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
    #[napi]
    pub fn parse_order_comparison_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.order_comparison_expression)
    }

    // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
    #[napi]
    pub fn parse_override_specifier(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.override_specifier)
    }

    // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
    #[napi]
    pub fn parse_parameter_declaration(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.parameter_declaration)
    }

    // ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
    #[napi]
    pub fn parse_parameter_list(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.parameter_list)
    }

    // ParenthesisExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
    #[napi]
    pub fn parse_parenthesis_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.parenthesis_expression)
    }

    // PayableExpression = 'payable' ArgumentList ;
    #[napi]
    pub fn parse_payable_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.payable_expression)
    }

    // PositionalArgumentList = Expression  { ',' Expression } ;
    #[napi]
    pub fn parse_positional_argument_list(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.positional_argument_list)
    }

    // «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
    #[napi]
    pub fn parse_possibly_separated_pairs_of_hex_digits(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.possibly_separated_pairs_of_hex_digits)
    }

    // PragmaDirective = 'pragma' ( VersionPragma | ABICoderPragma | ExperimentalPragma ) ';' ;
    #[napi]
    pub fn parse_pragma_directive(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.pragma_directive)
    }

    // PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | «NumericLiteral» | «BooleanLiteral» | «Identifier» ;
    #[napi]
    pub fn parse_primary_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.primary_expression)
    }

    // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
    #[napi]
    pub fn parse_raw_identifier(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.raw_identifier)
    }

    // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
    #[napi]
    pub fn parse_receive_function_attribute(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.receive_function_attribute)
    }

    // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
    #[napi]
    pub fn parse_receive_function_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.receive_function_definition)
    }

    // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
    #[napi]
    pub fn parse_reserved_keyword(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.reserved_keyword)
    }

    // ReturnStatement = 'return' [ Expression ] ';' ;
    #[napi]
    pub fn parse_return_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.return_statement)
    }

    // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
    #[napi]
    pub fn parse_revert_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.revert_statement)
    }

    // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
    #[napi]
    pub fn parse_selected_import(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.selected_import)
    }

    // SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
    #[napi]
    pub fn parse_selecting_import_directive(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.selecting_import_directive)
    }

    // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
    #[napi]
    pub fn parse_shift_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.shift_expression)
    }

    // «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
    #[napi]
    pub fn parse_signed_fixed_type(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.signed_fixed_type)
    }

    // «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
    #[napi]
    pub fn parse_signed_integer_type(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.signed_integer_type)
    }

    // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
    #[napi]
    pub fn parse_simple_import_directive(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.simple_import_directive)
    }

    // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
    #[napi]
    pub fn parse_simple_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.simple_statement)
    }

    // «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
    #[napi]
    pub fn parse_single_line_comment(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.single_line_comment)
    }

    // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
    #[napi]
    pub fn parse_single_quoted_ascii_string_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.single_quoted_ascii_string_literal)
    }

    // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
    #[napi]
    pub fn parse_single_quoted_unicode_string_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.single_quoted_unicode_string_literal)
    }

    // SourceUnit = LeadingTrivia { Directive | Definition } EndOfFileTrivia ;
    #[napi]
    pub fn parse_source_unit(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.source_unit)
    }

    // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
    #[napi]
    pub fn parse_star_import_directive(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.star_import_directive)
    }

    // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
    #[napi]
    pub fn parse_state_variable_attribute(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.state_variable_attribute)
    }

    // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
    #[napi]
    pub fn parse_state_variable_declaration(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.state_variable_declaration)
    }

    // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
    #[napi]
    pub fn parse_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.statement)
    }

    // StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
    #[napi]
    pub fn parse_struct_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.struct_definition)
    }

    // StructMember = TypeName «Identifier» ';' ;
    #[napi]
    pub fn parse_struct_member(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.struct_member)
    }

    // TrailingTrivia = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
    #[napi]
    pub fn parse_trailing_trivia(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.trailing_trivia)
    }

    // TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
    #[napi]
    pub fn parse_try_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.try_statement)
    }

    // TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
    #[napi]
    pub fn parse_tuple_deconstruction_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.tuple_deconstruction_statement)
    }

    // TypeExpression = 'type' '(' TypeName ')' ;
    #[napi]
    pub fn parse_type_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.type_expression)
    }

    // TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
    #[napi]
    pub fn parse_type_name(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.type_name)
    }

    // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
    #[napi]
    pub fn parse_unary_prefix_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.unary_prefix_expression)
    }

    // UnarySuffixExpression = Expression ( '++' | '--' ) ;
    #[napi]
    pub fn parse_unary_suffix_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.unary_suffix_expression)
    }

    // UncheckedBlock = 'unchecked' Block ;
    #[napi]
    pub fn parse_unchecked_block(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.unchecked_block)
    }

    // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
    #[napi]
    pub fn parse_unicode_escape(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.unicode_escape)
    }

    // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
    #[napi]
    pub fn parse_unicode_string_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.unicode_string_literal)
    }

    // «UnsignedFixedType» = 'u' «SignedFixedType» ;
    #[napi]
    pub fn parse_unsigned_fixed_type(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.unsigned_fixed_type)
    }

    // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
    #[napi]
    pub fn parse_unsigned_integer_type(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.unsigned_integer_type)
    }

    // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
    #[napi]
    pub fn parse_user_defined_value_type_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.user_defined_value_type_definition)
    }

    // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
    #[napi]
    pub fn parse_using_directive(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.using_directive)
    }

    // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
    #[napi]
    pub fn parse_variable_declaration_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.variable_declaration_statement)
    }

    // VersionPragma = 'solidity' 1…*{ VersionPragmaSpecifier } ;
    #[napi]
    pub fn parse_version_pragma(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.version_pragma)
    }

    // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
    #[napi]
    pub fn parse_version_pragma_operator(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.version_pragma_operator)
    }

    // VersionPragmaSpecifier = [ «VersionPragmaOperator» ] «VersionPragmaValue»  { '.' «VersionPragmaValue» } ;
    #[napi]
    pub fn parse_version_pragma_specifier(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.version_pragma_specifier)
    }

    // «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' } ;
    #[napi]
    pub fn parse_version_pragma_value(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.version_pragma_value)
    }

    // WhileStatement = 'while' '(' Expression ')' Statement ;
    #[napi]
    pub fn parse_while_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.while_statement)
    }

    // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
    #[napi]
    pub fn parse_whitespace(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.whitespace)
    }

    // YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
    #[napi]
    pub fn parse_yul_assignment_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_assignment_statement)
    }

    // YulBlock = '{' { YulStatement } '}' ;
    #[napi]
    pub fn parse_yul_block(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_block)
    }

    // YulBreakStatement = 'break' ;
    #[napi]
    pub fn parse_yul_break_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_break_statement)
    }

    // YulContinueStatement = 'continue' ;
    #[napi]
    pub fn parse_yul_continue_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_continue_statement)
    }

    // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
    #[napi]
    pub fn parse_yul_decimal_number_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_decimal_number_literal)
    }

    // YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
    #[napi]
    pub fn parse_yul_expression(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_expression)
    }

    // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
    #[napi]
    pub fn parse_yul_for_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_for_statement)
    }

    // YulFunctionCall = «YulIdentifier» '(' [ YulExpression  { ',' YulExpression } ] ')' ;
    #[napi]
    pub fn parse_yul_function_call(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_function_call)
    }

    // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
    #[napi]
    pub fn parse_yul_function_definition(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_function_definition)
    }

    // «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
    #[napi]
    pub fn parse_yul_hex_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_hex_literal)
    }

    // «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
    #[napi]
    pub fn parse_yul_identifier(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_identifier)
    }

    // YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
    #[napi]
    pub fn parse_yul_identifier_path(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_identifier_path)
    }

    // YulIfStatement = 'if' YulExpression YulBlock ;
    #[napi]
    pub fn parse_yul_if_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_if_statement)
    }

    // «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
    #[napi]
    pub fn parse_yul_keyword(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_keyword)
    }

    // YulLeaveStatement = 'leave' ;
    #[napi]
    pub fn parse_yul_leave_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_leave_statement)
    }

    // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
    #[napi]
    pub fn parse_yul_literal(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_literal)
    }

    // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
    #[napi]
    pub fn parse_yul_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_statement)
    }

    // YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
    #[napi]
    pub fn parse_yul_switch_statement(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_switch_statement)
    }

    // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
    #[napi]
    pub fn parse_yul_variable_declaration(&self, source: String) -> ParserOutput {
        ParserOutput::new(source, &self.parsers.yul_variable_declaration)
    }

    #[napi]
    pub fn parse(&self, parser_name: String, source: String) -> Option<ParserOutput> {
        match parser_name.as_str() {
            "ABICoderPragma" => Some(self.parse_abi_coder_pragma(source)),
            "AddSubExpression" => Some(self.parse_add_sub_expression(source)),
            "AddressType" => Some(self.parse_address_type(source)),
            "AndExpression" => Some(self.parse_and_expression(source)),
            "ArgumentList" => Some(self.parse_argument_list(source)),
            "ArrayLiteral" => Some(self.parse_array_literal(source)),
            "AsciiEscape" => Some(self.parse_ascii_escape(source)),
            "AsciiStringLiteral" => Some(self.parse_ascii_string_literal(source)),
            "AssemblyFlags" => Some(self.parse_assembly_flags(source)),
            "AssemblyStatement" => Some(self.parse_assembly_statement(source)),
            "AssignmentExpression" => Some(self.parse_assignment_expression(source)),
            "BitAndExpression" => Some(self.parse_bit_and_expression(source)),
            "BitOrExpression" => Some(self.parse_bit_or_expression(source)),
            "BitXOrExpression" => Some(self.parse_bit_x_or_expression(source)),
            "Block" => Some(self.parse_block(source)),
            "BooleanLiteral" => Some(self.parse_boolean_literal(source)),
            "BreakStatement" => Some(self.parse_break_statement(source)),
            "CatchClause" => Some(self.parse_catch_clause(source)),
            "ConditionalExpression" => Some(self.parse_conditional_expression(source)),
            "ConstantDefinition" => Some(self.parse_constant_definition(source)),
            "ConstructorAttribute" => Some(self.parse_constructor_attribute(source)),
            "ConstructorDefinition" => Some(self.parse_constructor_definition(source)),
            "ContinueStatement" => Some(self.parse_continue_statement(source)),
            "ContractBodyElement" => Some(self.parse_contract_body_element(source)),
            "ContractDefinition" => Some(self.parse_contract_definition(source)),
            "DataLocation" => Some(self.parse_data_location(source)),
            "DecimalExponent" => Some(self.parse_decimal_exponent(source)),
            "DecimalFloat" => Some(self.parse_decimal_float(source)),
            "DecimalInteger" => Some(self.parse_decimal_integer(source)),
            "DecimalNumber" => Some(self.parse_decimal_number(source)),
            "Definition" => Some(self.parse_definition(source)),
            "DeleteStatement" => Some(self.parse_delete_statement(source)),
            "Directive" => Some(self.parse_directive(source)),
            "DoWhileStatement" => Some(self.parse_do_while_statement(source)),
            "DoubleQuotedAsciiStringLiteral" => {
                Some(self.parse_double_quoted_ascii_string_literal(source))
            }
            "DoubleQuotedUnicodeStringLiteral" => {
                Some(self.parse_double_quoted_unicode_string_literal(source))
            }
            "ElementaryType" => Some(self.parse_elementary_type(source)),
            "EmitStatement" => Some(self.parse_emit_statement(source)),
            "EndOfFileTrivia" => Some(self.parse_end_of_file_trivia(source)),
            "EndOfLine" => Some(self.parse_end_of_line(source)),
            "EnumDefinition" => Some(self.parse_enum_definition(source)),
            "EqualityComparisonExpression" => {
                Some(self.parse_equality_comparison_expression(source))
            }
            "ErrorDefinition" => Some(self.parse_error_definition(source)),
            "ErrorParameter" => Some(self.parse_error_parameter(source)),
            "EscapeSequence" => Some(self.parse_escape_sequence(source)),
            "EventDefinition" => Some(self.parse_event_definition(source)),
            "EventParameter" => Some(self.parse_event_parameter(source)),
            "ExperimentalPragma" => Some(self.parse_experimental_pragma(source)),
            "ExponentiationExpression" => Some(self.parse_exponentiation_expression(source)),
            "Expression" => Some(self.parse_expression(source)),
            "ExpressionStatement" => Some(self.parse_expression_statement(source)),
            "FallbackFunctionAttribute" => Some(self.parse_fallback_function_attribute(source)),
            "FallbackFunctionDefinition" => Some(self.parse_fallback_function_definition(source)),
            "FixedBytesType" => Some(self.parse_fixed_bytes_type(source)),
            "ForStatement" => Some(self.parse_for_statement(source)),
            "FunctionAttribute" => Some(self.parse_function_attribute(source)),
            "FunctionCallExpression" => Some(self.parse_function_call_expression(source)),
            "FunctionDefinition" => Some(self.parse_function_definition(source)),
            "FunctionType" => Some(self.parse_function_type(source)),
            "HexByteEscape" => Some(self.parse_hex_byte_escape(source)),
            "HexCharacter" => Some(self.parse_hex_character(source)),
            "HexNumber" => Some(self.parse_hex_number(source)),
            "HexStringLiteral" => Some(self.parse_hex_string_literal(source)),
            "Identifier" => Some(self.parse_identifier(source)),
            "IdentifierPart" => Some(self.parse_identifier_part(source)),
            "IdentifierPath" => Some(self.parse_identifier_path(source)),
            "IdentifierStart" => Some(self.parse_identifier_start(source)),
            "IfStatement" => Some(self.parse_if_statement(source)),
            "ImportDirective" => Some(self.parse_import_directive(source)),
            "ImportPath" => Some(self.parse_import_path(source)),
            "IndexAccessExpression" => Some(self.parse_index_access_expression(source)),
            "InheritanceSpecifier" => Some(self.parse_inheritance_specifier(source)),
            "InheritanceSpecifierList" => Some(self.parse_inheritance_specifier_list(source)),
            "InterfaceDefinition" => Some(self.parse_interface_definition(source)),
            "Keyword" => Some(self.parse_keyword(source)),
            "LeadingTrivia" => Some(self.parse_leading_trivia(source)),
            "LibraryDefinition" => Some(self.parse_library_definition(source)),
            "MappingType" => Some(self.parse_mapping_type(source)),
            "MemberAccessExpression" => Some(self.parse_member_access_expression(source)),
            "ModifierAttribute" => Some(self.parse_modifier_attribute(source)),
            "ModifierDefinition" => Some(self.parse_modifier_definition(source)),
            "ModifierInvocation" => Some(self.parse_modifier_invocation(source)),
            "MulDivModExpression" => Some(self.parse_mul_div_mod_expression(source)),
            "MultilineComment" => Some(self.parse_multiline_comment(source)),
            "NamedArgument" => Some(self.parse_named_argument(source)),
            "NamedArgumentList" => Some(self.parse_named_argument_list(source)),
            "NewExpression" => Some(self.parse_new_expression(source)),
            "NumberUnit" => Some(self.parse_number_unit(source)),
            "NumericLiteral" => Some(self.parse_numeric_literal(source)),
            "OrExpression" => Some(self.parse_or_expression(source)),
            "OrderComparisonExpression" => Some(self.parse_order_comparison_expression(source)),
            "OverrideSpecifier" => Some(self.parse_override_specifier(source)),
            "ParameterDeclaration" => Some(self.parse_parameter_declaration(source)),
            "ParameterList" => Some(self.parse_parameter_list(source)),
            "ParenthesisExpression" => Some(self.parse_parenthesis_expression(source)),
            "PayableExpression" => Some(self.parse_payable_expression(source)),
            "PositionalArgumentList" => Some(self.parse_positional_argument_list(source)),
            "PossiblySeparatedPairsOfHexDigits" => {
                Some(self.parse_possibly_separated_pairs_of_hex_digits(source))
            }
            "PragmaDirective" => Some(self.parse_pragma_directive(source)),
            "PrimaryExpression" => Some(self.parse_primary_expression(source)),
            "RawIdentifier" => Some(self.parse_raw_identifier(source)),
            "ReceiveFunctionAttribute" => Some(self.parse_receive_function_attribute(source)),
            "ReceiveFunctionDefinition" => Some(self.parse_receive_function_definition(source)),
            "ReservedKeyword" => Some(self.parse_reserved_keyword(source)),
            "ReturnStatement" => Some(self.parse_return_statement(source)),
            "RevertStatement" => Some(self.parse_revert_statement(source)),
            "SelectedImport" => Some(self.parse_selected_import(source)),
            "SelectingImportDirective" => Some(self.parse_selecting_import_directive(source)),
            "ShiftExpression" => Some(self.parse_shift_expression(source)),
            "SignedFixedType" => Some(self.parse_signed_fixed_type(source)),
            "SignedIntegerType" => Some(self.parse_signed_integer_type(source)),
            "SimpleImportDirective" => Some(self.parse_simple_import_directive(source)),
            "SimpleStatement" => Some(self.parse_simple_statement(source)),
            "SingleLineComment" => Some(self.parse_single_line_comment(source)),
            "SingleQuotedAsciiStringLiteral" => {
                Some(self.parse_single_quoted_ascii_string_literal(source))
            }
            "SingleQuotedUnicodeStringLiteral" => {
                Some(self.parse_single_quoted_unicode_string_literal(source))
            }
            "SourceUnit" => Some(self.parse_source_unit(source)),
            "StarImportDirective" => Some(self.parse_star_import_directive(source)),
            "StateVariableAttribute" => Some(self.parse_state_variable_attribute(source)),
            "StateVariableDeclaration" => Some(self.parse_state_variable_declaration(source)),
            "Statement" => Some(self.parse_statement(source)),
            "StructDefinition" => Some(self.parse_struct_definition(source)),
            "StructMember" => Some(self.parse_struct_member(source)),
            "TrailingTrivia" => Some(self.parse_trailing_trivia(source)),
            "TryStatement" => Some(self.parse_try_statement(source)),
            "TupleDeconstructionStatement" => {
                Some(self.parse_tuple_deconstruction_statement(source))
            }
            "TypeExpression" => Some(self.parse_type_expression(source)),
            "TypeName" => Some(self.parse_type_name(source)),
            "UnaryPrefixExpression" => Some(self.parse_unary_prefix_expression(source)),
            "UnarySuffixExpression" => Some(self.parse_unary_suffix_expression(source)),
            "UncheckedBlock" => Some(self.parse_unchecked_block(source)),
            "UnicodeEscape" => Some(self.parse_unicode_escape(source)),
            "UnicodeStringLiteral" => Some(self.parse_unicode_string_literal(source)),
            "UnsignedFixedType" => Some(self.parse_unsigned_fixed_type(source)),
            "UnsignedIntegerType" => Some(self.parse_unsigned_integer_type(source)),
            "UserDefinedValueTypeDefinition" => {
                Some(self.parse_user_defined_value_type_definition(source))
            }
            "UsingDirective" => Some(self.parse_using_directive(source)),
            "VariableDeclarationStatement" => {
                Some(self.parse_variable_declaration_statement(source))
            }
            "VersionPragma" => Some(self.parse_version_pragma(source)),
            "VersionPragmaOperator" => Some(self.parse_version_pragma_operator(source)),
            "VersionPragmaSpecifier" => Some(self.parse_version_pragma_specifier(source)),
            "VersionPragmaValue" => Some(self.parse_version_pragma_value(source)),
            "WhileStatement" => Some(self.parse_while_statement(source)),
            "Whitespace" => Some(self.parse_whitespace(source)),
            "YulAssignmentStatement" => Some(self.parse_yul_assignment_statement(source)),
            "YulBlock" => Some(self.parse_yul_block(source)),
            "YulBreakStatement" => Some(self.parse_yul_break_statement(source)),
            "YulContinueStatement" => Some(self.parse_yul_continue_statement(source)),
            "YulDecimalNumberLiteral" => Some(self.parse_yul_decimal_number_literal(source)),
            "YulExpression" => Some(self.parse_yul_expression(source)),
            "YulForStatement" => Some(self.parse_yul_for_statement(source)),
            "YulFunctionCall" => Some(self.parse_yul_function_call(source)),
            "YulFunctionDefinition" => Some(self.parse_yul_function_definition(source)),
            "YulHexLiteral" => Some(self.parse_yul_hex_literal(source)),
            "YulIdentifier" => Some(self.parse_yul_identifier(source)),
            "YulIdentifierPath" => Some(self.parse_yul_identifier_path(source)),
            "YulIfStatement" => Some(self.parse_yul_if_statement(source)),
            "YulKeyword" => Some(self.parse_yul_keyword(source)),
            "YulLeaveStatement" => Some(self.parse_yul_leave_statement(source)),
            "YulLiteral" => Some(self.parse_yul_literal(source)),
            "YulStatement" => Some(self.parse_yul_statement(source)),
            "YulSwitchStatement" => Some(self.parse_yul_switch_statement(source)),
            "YulVariableDeclaration" => Some(self.parse_yul_variable_declaration(source)),
            _ => None,
        }
    }
}
