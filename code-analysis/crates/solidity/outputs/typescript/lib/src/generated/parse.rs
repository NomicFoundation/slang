// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::cst;
use super::kinds;
use super::lex;
use chumsky::prelude::*;
use chumsky::Parser;
use semver::Version;
use std::ops::Range;
use std::rc::Rc;
mod factory {
    use super::cst;
    use super::kinds;
    use super::lex;
    use std::ops::Range;
    use std::rc::Rc;
    pub fn lex_none() -> Rc<lex::Node> {
        Rc::new(lex::Node::None)
    }
    pub fn lex_chars(range: Range<usize>) -> Rc<lex::Node> {
        Rc::new(lex::Node::Chars(range))
    }
    pub fn lex_sequence(elements: Vec<Rc<lex::Node>>) -> Rc<lex::Node> {
        Rc::new(if elements.is_empty() {
            lex::Node::None
        } else {
            lex::Node::Sequence(elements)
        })
    }
    pub fn lex_choice(number: usize, element: Rc<lex::Node>) -> Rc<lex::Node> {
        Rc::new(lex::Node::Choice(number, element))
    }
    pub fn lex_named(kind: kinds::Token, element: Rc<lex::Node>) -> Rc<lex::Node> {
        Rc::new(lex::Node::Named(kind, element))
    }
    pub fn cst_none() -> Rc<cst::Node> {
        Rc::new(cst::Node::None)
    }
    pub fn cst_rule(kind: kinds::Rule, children: Vec<Rc<cst::Node>>) -> Rc<cst::Node> {
        Rc::new(cst::Node::Rule { kind, children })
    }
    pub fn cst_trivia_token(range: Range<usize>, kind: kinds::Token) -> Rc<cst::Node> {
        Rc::new(cst::Node::Token {
            range,
            kind,
            trivia: vec![],
        })
    }
    pub fn cst_token(
        range: Range<usize>,
        kind: kinds::Token,
        leading_trivia: Rc<cst::Node>,
        trailing_trivia: Rc<cst::Node>,
    ) -> Rc<cst::Node> {
        let mut trivia = vec![];
        if *leading_trivia != cst::Node::None {
            trivia.push(leading_trivia)
        }
        if *trailing_trivia != cst::Node::None {
            trivia.push(trailing_trivia)
        }
        Rc::new(cst::Node::Token {
            range,
            kind,
            trivia,
        })
    }
    pub fn cst_group(children: Vec<Rc<cst::Node>>) -> Rc<cst::Node> {
        if children.is_empty() {
            cst_none()
        } else {
            Rc::new(cst::Node::Group { children })
        }
    }
}
pub type SpanType = Range<usize>;
pub type ErrorType = Simple<char, SpanType>;
pub type ParserType<'p, T> = BoxedParser<'p, char, T, ErrorType>;
#[allow(dead_code)]
fn difference<M, S, T>(minuend: M, subtrahend: S) -> impl Parser<char, T, Error = ErrorType>
where
    M: Clone + Parser<char, T, Error = ErrorType>,
    S: Parser<char, T, Error = ErrorType>,
{
    let minuend_end = minuend
        .clone()
        .map_with_span(|_, span: SpanType| span.end())
        .rewind();
    let subtrahend_end = subtrahend
        .map_with_span(|_, span: SpanType| span.end())
        .rewind()
        .or_else(|_| Ok(0));
    minuend_end
        .then(subtrahend_end)
        .validate(|(m, s), span, emit| {
            if m == s {
                emit(Simple::custom(span, "subtrahend matches minuend"))
            }
        })
        .ignore_then(minuend)
}

#[allow(dead_code)]
pub struct Parsers<'a> {
    /// ABICoderPragmaSpecifier = 'abicoder' «Identifier» ;
    pub abi_coder_pragma_specifier: ParserType<'a, Rc<cst::Node>>,

    /// AddSubExpression = Expression ( '+' | '-' ) Expression ;
    pub add_sub_expression: ParserType<'a, Rc<cst::Node>>,

    /// AddressType = 'address' [ 'payable' ] ;
    pub address_type: ParserType<'a, Rc<cst::Node>>,

    /// AndExpression = Expression '&&' Expression ;
    pub and_expression: ParserType<'a, Rc<cst::Node>>,

    /// ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
    pub argument_list: ParserType<'a, Rc<cst::Node>>,

    /// ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
    pub array_literal: ParserType<'a, Rc<cst::Node>>,

    /// «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
    pub ascii_escape: ParserType<'a, Rc<lex::Node>>,

    /// «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
    pub ascii_string_literal: ParserType<'a, Rc<lex::Node>>,

    /// AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
    pub assembly_flags: ParserType<'a, Rc<cst::Node>>,

    /// AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
    pub assembly_statement: ParserType<'a, Rc<cst::Node>>,

    /// AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
    pub assignment_expression: ParserType<'a, Rc<cst::Node>>,

    /// BitAndExpression = Expression '&' Expression ;
    pub bit_and_expression: ParserType<'a, Rc<cst::Node>>,

    /// BitOrExpression = Expression '|' Expression ;
    pub bit_or_expression: ParserType<'a, Rc<cst::Node>>,

    /// BitXOrExpression = Expression '^' Expression ;
    pub bit_x_or_expression: ParserType<'a, Rc<cst::Node>>,

    /// Block = '{' { Statement | UncheckedBlock } '}' ;
    pub block: ParserType<'a, Rc<cst::Node>>,

    /// «BooleanLiteral» = 'true' | 'false' ;
    pub boolean_literal: ParserType<'a, Rc<lex::Node>>,

    /// BreakStatement = 'break' ';' ;
    pub break_statement: ParserType<'a, Rc<cst::Node>>,

    /// CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
    pub catch_clause: ParserType<'a, Rc<cst::Node>>,

    /// ConditionalExpression = Expression '?' Expression ':' Expression ;
    pub conditional_expression: ParserType<'a, Rc<cst::Node>>,

    /// ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
    pub constant_definition: ParserType<'a, Rc<cst::Node>>,

    /// ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
    pub constructor_attribute: ParserType<'a, Rc<cst::Node>>,

    /// ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
    pub constructor_definition: ParserType<'a, Rc<cst::Node>>,

    /// ContinueStatement = 'continue' ';' ;
    pub continue_statement: ParserType<'a, Rc<cst::Node>>,

    /// ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
    pub contract_body_element: ParserType<'a, Rc<cst::Node>>,

    /// ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    pub contract_definition: ParserType<'a, Rc<cst::Node>>,

    /// DataLocation = 'memory' | 'storage' | 'calldata' ;
    pub data_location: ParserType<'a, Rc<cst::Node>>,

    /// «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
    pub decimal_exponent: ParserType<'a, Rc<lex::Node>>,

    /// «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
    pub decimal_float: ParserType<'a, Rc<lex::Node>>,

    /// «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
    pub decimal_integer: ParserType<'a, Rc<lex::Node>>,

    /// «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
    pub decimal_number: ParserType<'a, Rc<lex::Node>>,

    /// Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
    pub definition: ParserType<'a, Rc<cst::Node>>,

    /// DeleteStatement = 'delete' «Identifier» ';' ;
    pub delete_statement: ParserType<'a, Rc<cst::Node>>,

    /// Directive = PragmaDirective | ImportDirective | UsingDirective ;
    pub directive: ParserType<'a, Rc<cst::Node>>,

    /// DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
    pub do_while_statement: ParserType<'a, Rc<cst::Node>>,

    /// «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
    pub double_quoted_ascii_string_literal: ParserType<'a, Rc<lex::Node>>,

    /// «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
    pub double_quoted_unicode_string_literal: ParserType<'a, Rc<lex::Node>>,

    /// ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
    pub elementary_type: ParserType<'a, Rc<cst::Node>>,

    /// EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
    pub emit_statement: ParserType<'a, Rc<cst::Node>>,

    /// EndOfFileTrivia = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
    pub end_of_file_trivia: ParserType<'a, Rc<cst::Node>>,

    /// «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
    pub end_of_line: ParserType<'a, Rc<lex::Node>>,

    /// EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
    pub enum_definition: ParserType<'a, Rc<cst::Node>>,

    /// EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
    pub equality_comparison_expression: ParserType<'a, Rc<cst::Node>>,

    /// ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
    pub error_definition: ParserType<'a, Rc<cst::Node>>,

    /// ErrorParameter = TypeName [ «Identifier» ] ;
    pub error_parameter: ParserType<'a, Rc<cst::Node>>,

    /// «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
    pub escape_sequence: ParserType<'a, Rc<lex::Node>>,

    /// EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
    pub event_definition: ParserType<'a, Rc<cst::Node>>,

    /// EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
    pub event_parameter: ParserType<'a, Rc<cst::Node>>,

    /// ExperimentalPragmaSpecifier = 'experimental' «Identifier» ;
    pub experimental_pragma_specifier: ParserType<'a, Rc<cst::Node>>,

    /// (* 0.0.0 *) ExponentiationExpression = Expression '**' Expression ;
    /// (* 0.6.0 *) ExponentiationExpression = Expression '**' Expression ;
    pub exponentiation_expression: ParserType<'a, Rc<cst::Node>>,

    /// Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    pub expression: ParserType<'a, Rc<cst::Node>>,

    /// ExpressionStatement = Expression ';' ;
    pub expression_statement: ParserType<'a, Rc<cst::Node>>,

    /// FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
    pub fallback_function_attribute: ParserType<'a, Rc<cst::Node>>,

    /// FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    pub fallback_function_definition: ParserType<'a, Rc<cst::Node>>,

    /// «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
    pub fixed_bytes_type: ParserType<'a, Rc<lex::Node>>,

    /// ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
    pub for_statement: ParserType<'a, Rc<cst::Node>>,

    /// FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
    pub function_attribute: ParserType<'a, Rc<cst::Node>>,

    /// FunctionCallExpression = Expression [ '{' NamedArgument  { ',' NamedArgument } '}' ] ArgumentList ;
    pub function_call_expression: ParserType<'a, Rc<cst::Node>>,

    /// FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    pub function_definition: ParserType<'a, Rc<cst::Node>>,

    /// FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
    pub function_type: ParserType<'a, Rc<cst::Node>>,

    /// «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
    pub hex_byte_escape: ParserType<'a, Rc<lex::Node>>,

    /// «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
    pub hex_character: ParserType<'a, Rc<lex::Node>>,

    /// «HexNumber» = '0x' «HexCharacter» { [ '_' ] «HexCharacter» } ;
    pub hex_number: ParserType<'a, Rc<lex::Node>>,

    /// «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
    pub hex_string_literal: ParserType<'a, Rc<lex::Node>>,

    /// «Identifier» = «RawIdentifier» - «Keyword» ;
    pub identifier: ParserType<'a, Rc<lex::Node>>,

    /// «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
    pub identifier_part: ParserType<'a, Rc<lex::Node>>,

    /// IdentifierPath = «Identifier»  { '.' «Identifier» } ;
    pub identifier_path: ParserType<'a, Rc<cst::Node>>,

    /// «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
    pub identifier_start: ParserType<'a, Rc<lex::Node>>,

    /// IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
    pub if_statement: ParserType<'a, Rc<cst::Node>>,

    /// ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
    pub import_directive: ParserType<'a, Rc<cst::Node>>,

    /// ImportPath = «AsciiStringLiteral» ;
    pub import_path: ParserType<'a, Rc<cst::Node>>,

    /// IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
    pub index_access_expression: ParserType<'a, Rc<cst::Node>>,

    /// InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
    pub inheritance_specifier: ParserType<'a, Rc<cst::Node>>,

    /// InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
    pub inheritance_specifier_list: ParserType<'a, Rc<cst::Node>>,

    /// InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    pub interface_definition: ParserType<'a, Rc<cst::Node>>,

    /// «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
    pub keyword: ParserType<'a, Rc<lex::Node>>,

    /// LeadingTrivia = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
    pub leading_trivia: ParserType<'a, Rc<cst::Node>>,

    /// LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
    pub library_definition: ParserType<'a, Rc<cst::Node>>,

    /// MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
    pub mapping_type: ParserType<'a, Rc<cst::Node>>,

    /// MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
    pub member_access_expression: ParserType<'a, Rc<cst::Node>>,

    /// ModifierAttribute = OverrideSpecifier | 'virtual' ;
    pub modifier_attribute: ParserType<'a, Rc<cst::Node>>,

    /// ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
    pub modifier_definition: ParserType<'a, Rc<cst::Node>>,

    /// ModifierInvocation = IdentifierPath [ ArgumentList ] ;
    pub modifier_invocation: ParserType<'a, Rc<cst::Node>>,

    /// MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
    pub mul_div_mod_expression: ParserType<'a, Rc<cst::Node>>,

    /// «MultilineComment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
    pub multiline_comment: ParserType<'a, Rc<lex::Node>>,

    /// NamedArgument = «Identifier» ':' Expression ;
    pub named_argument: ParserType<'a, Rc<cst::Node>>,

    /// NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
    pub named_argument_list: ParserType<'a, Rc<cst::Node>>,

    /// NewExpression = 'new' IdentifierPath ArgumentList ;
    pub new_expression: ParserType<'a, Rc<cst::Node>>,

    /// «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
    pub number_unit: ParserType<'a, Rc<lex::Node>>,

    /// «NumericLiteral» = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
    pub numeric_literal: ParserType<'a, Rc<lex::Node>>,

    /// OrExpression = Expression '||' Expression ;
    pub or_expression: ParserType<'a, Rc<cst::Node>>,

    /// OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
    pub order_comparison_expression: ParserType<'a, Rc<cst::Node>>,

    /// OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
    pub override_specifier: ParserType<'a, Rc<cst::Node>>,

    /// ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
    pub parameter_declaration: ParserType<'a, Rc<cst::Node>>,

    /// ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
    pub parameter_list: ParserType<'a, Rc<cst::Node>>,

    /// ParenthesisExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
    pub parenthesis_expression: ParserType<'a, Rc<cst::Node>>,

    /// PayableExpression = 'payable' ArgumentList ;
    pub payable_expression: ParserType<'a, Rc<cst::Node>>,

    /// PositionalArgumentList = Expression  { ',' Expression } ;
    pub positional_argument_list: ParserType<'a, Rc<cst::Node>>,

    /// «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
    pub possibly_separated_pairs_of_hex_digits: ParserType<'a, Rc<lex::Node>>,

    /// PragmaDirective = 'pragma' ( VersionPragmaSpecifier | ABICoderPragmaSpecifier | ExperimentalPragmaSpecifier ) ';' ;
    pub pragma_directive: ParserType<'a, Rc<cst::Node>>,

    /// PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | «NumericLiteral» | «BooleanLiteral» | «Identifier» ;
    pub primary_expression: ParserType<'a, Rc<cst::Node>>,

    /// «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
    pub raw_identifier: ParserType<'a, Rc<lex::Node>>,

    /// ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
    pub receive_function_attribute: ParserType<'a, Rc<cst::Node>>,

    /// ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
    pub receive_function_definition: ParserType<'a, Rc<cst::Node>>,

    /// «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
    pub reserved_keyword: ParserType<'a, Rc<lex::Node>>,

    /// ReturnStatement = 'return' [ Expression ] ';' ;
    pub return_statement: ParserType<'a, Rc<cst::Node>>,

    /// RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
    pub revert_statement: ParserType<'a, Rc<cst::Node>>,

    /// SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
    pub selected_import: ParserType<'a, Rc<cst::Node>>,

    /// SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
    pub selecting_import_directive: ParserType<'a, Rc<cst::Node>>,

    /// ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
    pub shift_expression: ParserType<'a, Rc<cst::Node>>,

    /// «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
    pub signed_fixed_type: ParserType<'a, Rc<lex::Node>>,

    /// «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
    pub signed_integer_type: ParserType<'a, Rc<lex::Node>>,

    /// SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
    pub simple_import_directive: ParserType<'a, Rc<cst::Node>>,

    /// SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
    pub simple_statement: ParserType<'a, Rc<cst::Node>>,

    /// «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
    pub single_line_comment: ParserType<'a, Rc<lex::Node>>,

    /// «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
    pub single_quoted_ascii_string_literal: ParserType<'a, Rc<lex::Node>>,

    /// «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
    pub single_quoted_unicode_string_literal: ParserType<'a, Rc<lex::Node>>,

    /// SourceUnit = LeadingTrivia { Directive | Definition } EndOfFileTrivia ;
    pub source_unit: ParserType<'a, Rc<cst::Node>>,

    /// StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
    pub star_import_directive: ParserType<'a, Rc<cst::Node>>,

    /// StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
    pub state_variable_attribute: ParserType<'a, Rc<cst::Node>>,

    /// StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
    pub state_variable_declaration: ParserType<'a, Rc<cst::Node>>,

    /// Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
    pub statement: ParserType<'a, Rc<cst::Node>>,

    /// StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
    pub struct_definition: ParserType<'a, Rc<cst::Node>>,

    /// StructMember = TypeName «Identifier» ';' ;
    pub struct_member: ParserType<'a, Rc<cst::Node>>,

    /// TrailingTrivia = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
    pub trailing_trivia: ParserType<'a, Rc<cst::Node>>,

    /// TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
    pub try_statement: ParserType<'a, Rc<cst::Node>>,

    /// TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
    pub tuple_deconstruction_statement: ParserType<'a, Rc<cst::Node>>,

    /// TypeExpression = 'type' '(' TypeName ')' ;
    pub type_expression: ParserType<'a, Rc<cst::Node>>,

    /// TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
    pub type_name: ParserType<'a, Rc<cst::Node>>,

    /// UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
    pub unary_prefix_expression: ParserType<'a, Rc<cst::Node>>,

    /// UnarySuffixExpression = Expression ( '++' | '--' ) ;
    pub unary_suffix_expression: ParserType<'a, Rc<cst::Node>>,

    /// UncheckedBlock = 'unchecked' Block ;
    pub unchecked_block: ParserType<'a, Rc<cst::Node>>,

    /// «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
    pub unicode_escape: ParserType<'a, Rc<lex::Node>>,

    /// «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
    pub unicode_string_literal: ParserType<'a, Rc<lex::Node>>,

    /// «UnsignedFixedType» = 'u' «SignedFixedType» ;
    pub unsigned_fixed_type: ParserType<'a, Rc<lex::Node>>,

    /// «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
    pub unsigned_integer_type: ParserType<'a, Rc<lex::Node>>,

    /// UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
    pub user_defined_value_type_definition: ParserType<'a, Rc<cst::Node>>,

    /// UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
    pub using_directive: ParserType<'a, Rc<cst::Node>>,

    /// VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
    pub variable_declaration_statement: ParserType<'a, Rc<cst::Node>>,

    /// «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
    pub version_pragma_operator: ParserType<'a, Rc<lex::Node>>,

    /// VersionPragmaSpecifier = 'solidity' 1…*{ «VersionPragmaOperator» «VersionPragmaValue» } ;
    pub version_pragma_specifier: ParserType<'a, Rc<cst::Node>>,

    /// «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' }  { '.' 1…*{ '0'…'9' | 'x' | 'X' | '*' } } ;
    pub version_pragma_value: ParserType<'a, Rc<lex::Node>>,

    /// WhileStatement = 'while' '(' Expression ')' Statement ;
    pub while_statement: ParserType<'a, Rc<cst::Node>>,

    /// «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
    pub whitespace: ParserType<'a, Rc<lex::Node>>,

    /// YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
    pub yul_assignment_statement: ParserType<'a, Rc<cst::Node>>,

    /// YulBlock = '{' { YulStatement } '}' ;
    pub yul_block: ParserType<'a, Rc<cst::Node>>,

    /// YulBreakStatement = 'break' ;
    pub yul_break_statement: ParserType<'a, Rc<cst::Node>>,

    /// YulContinueStatement = 'continue' ;
    pub yul_continue_statement: ParserType<'a, Rc<cst::Node>>,

    /// «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
    pub yul_decimal_number_literal: ParserType<'a, Rc<lex::Node>>,

    /// YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
    pub yul_expression: ParserType<'a, Rc<cst::Node>>,

    /// YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
    pub yul_for_statement: ParserType<'a, Rc<cst::Node>>,

    /// YulFunctionCall = «YulIdentifier» '(' [ YulExpression  { ',' YulExpression } ] ')' ;
    pub yul_function_call: ParserType<'a, Rc<cst::Node>>,

    /// YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
    pub yul_function_definition: ParserType<'a, Rc<cst::Node>>,

    /// «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
    pub yul_hex_literal: ParserType<'a, Rc<lex::Node>>,

    /// «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
    pub yul_identifier: ParserType<'a, Rc<lex::Node>>,

    /// YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
    pub yul_identifier_path: ParserType<'a, Rc<cst::Node>>,

    /// YulIfStatement = 'if' YulExpression YulBlock ;
    pub yul_if_statement: ParserType<'a, Rc<cst::Node>>,

    /// «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
    pub yul_keyword: ParserType<'a, Rc<lex::Node>>,

    /// YulLeaveStatement = 'leave' ;
    pub yul_leave_statement: ParserType<'a, Rc<cst::Node>>,

    /// YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
    pub yul_literal: ParserType<'a, Rc<cst::Node>>,

    /// YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
    pub yul_statement: ParserType<'a, Rc<cst::Node>>,

    /// YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
    pub yul_switch_statement: ParserType<'a, Rc<cst::Node>>,

    /// YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
    pub yul_variable_declaration: ParserType<'a, Rc<cst::Node>>,
}

impl<'a> Parsers<'a> {
    pub fn new(version: &Version) -> Self {
        // Declare all versions -----------------------------

        let version_0_6_0 = &Version::parse("0.6.0").unwrap();

        // Declare all productions --------------------------

        let mut abi_coder_pragma_specifier_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut add_sub_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut address_type_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut and_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut argument_list_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut array_literal_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut ascii_escape_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut ascii_string_literal_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut assembly_flags_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut assembly_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut assignment_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut bit_and_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut bit_or_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut bit_x_or_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut block_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut boolean_literal_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut break_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut catch_clause_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut conditional_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut constant_definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut constructor_attribute_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut constructor_definition_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut continue_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut contract_body_element_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut contract_definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut data_location_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut decimal_exponent_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut decimal_float_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut decimal_integer_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut decimal_number_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut delete_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut directive_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut do_while_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut double_quoted_ascii_string_literal_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut double_quoted_unicode_string_literal_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut elementary_type_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut emit_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut end_of_file_trivia_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut end_of_line_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut enum_definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut equality_comparison_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut error_definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut error_parameter_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut escape_sequence_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut event_definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut event_parameter_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut experimental_pragma_specifier_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut exponentiation_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut expression_statement_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut fallback_function_attribute_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut fallback_function_definition_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut fixed_bytes_type_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut for_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut function_attribute_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut function_call_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut function_definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut function_type_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut hex_byte_escape_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut hex_character_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut hex_number_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut hex_string_literal_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut identifier_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut identifier_part_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut identifier_path_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut identifier_start_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut if_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut import_directive_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut import_path_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut index_access_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut inheritance_specifier_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut inheritance_specifier_list_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut interface_definition_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut keyword_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut leading_trivia_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut library_definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut mapping_type_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut member_access_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut modifier_attribute_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut modifier_definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut modifier_invocation_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut mul_div_mod_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut multiline_comment_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut named_argument_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut named_argument_list_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut new_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut number_unit_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut numeric_literal_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut or_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut order_comparison_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut override_specifier_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut parameter_declaration_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut parameter_list_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut parenthesis_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut payable_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut positional_argument_list_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut possibly_separated_pairs_of_hex_digits_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut pragma_directive_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut primary_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut raw_identifier_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut receive_function_attribute_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut receive_function_definition_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut reserved_keyword_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut return_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut revert_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut selected_import_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut selecting_import_directive_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut shift_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut signed_fixed_type_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut signed_integer_type_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut simple_import_directive_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut simple_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut single_line_comment_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut single_quoted_ascii_string_literal_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut single_quoted_unicode_string_literal_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut source_unit_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut star_import_directive_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut state_variable_attribute_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut state_variable_declaration_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut struct_definition_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut struct_member_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut trailing_trivia_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut try_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut tuple_deconstruction_statement_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut type_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut type_name_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut unary_prefix_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut unary_suffix_expression_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut unchecked_block_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut unicode_escape_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut unicode_string_literal_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut unsigned_fixed_type_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut unsigned_integer_type_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut user_defined_value_type_definition_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut using_directive_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut variable_declaration_statement_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut version_pragma_operator_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut version_pragma_specifier_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut version_pragma_value_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut while_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut whitespace_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut yul_assignment_statement_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_block_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_break_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_continue_statement_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_decimal_number_literal_parser =
            Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut yul_expression_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_for_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_function_call_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_function_definition_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_hex_literal_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut yul_identifier_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut yul_identifier_path_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_if_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_keyword_parser = Recursive::<char, Rc<lex::Node>, ErrorType>::declare();
        let mut yul_leave_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_literal_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_statement_parser = Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_switch_statement_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();
        let mut yul_variable_declaration_parser =
            Recursive::<char, Rc<cst::Node>, ErrorType>::declare();

        // Macros -------------------------------------------

        #[allow(unused_macros)]
        macro_rules! lex_terminal {
            ($ kind : ident , $ literal : literal) => {
                just($literal).map_with_span(|_, span: SpanType| {
                    factory::lex_named(
                        kinds::Token::$kind,
                        factory::lex_chars(span.start()..span.end()),
                    )
                })
            };
            ($ kind : ident , $ filter : expr) => {
                filter($filter).map_with_span(|_, span: SpanType| {
                    factory::lex_named(
                        kinds::Token::$kind,
                        factory::lex_chars(span.start()..span.end()),
                    )
                })
            };
            ($ literal : literal) => {
                just($literal)
                    .map_with_span(|_, span: SpanType| factory::lex_chars(span.start()..span.end()))
            };
            ($ filter : expr) => {
                filter($filter)
                    .map_with_span(|_, span: SpanType| factory::lex_chars(span.start()..span.end()))
            };
        }
        #[allow(unused_macros)]
        macro_rules! lex_rule {
            ($ rule : ident) => {
                $rule.clone()
            };
        }
        #[allow(unused_macros)]
        macro_rules ! lex_choice { ($ kind : ident , $ ($ expr : expr) , *) => { lex_choice ! ($ ($ expr) , *) . map (| element | factory :: lex_named (kinds :: Token :: $ kind , element)) } ; ($ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr . map (| v | factory :: lex_choice (0 , v))) , *)) } ; }
        #[allow(unused_macros)]
        macro_rules ! lex_seq { ($ kind : ident , $ ($ expr : expr) , *) => { lex_seq ! ($ ($ expr) , *) . map (| element | factory :: lex_named (kinds :: Token :: $ kind , element)) } ; ($ a : expr , $ ($ b : expr) , *) => { $ a $ (. then ($ b)) * . map_with_span (| _ , span : SpanType | factory :: lex_chars (span . start () .. span . end ())) } ; }
        #[allow(unused_macros)]
        macro_rules! lex_zero_or_more {
            ($ kind : ident , $ expr : expr) => {
                lex_zero_or_more!($expr)
                    .map(|element| factory::lex_named(kinds::Token::$kind, element))
            };
            ($ expr : expr) => {
                $expr.repeated().map(|v| factory::lex_sequence(v))
            };
        }
        #[allow(unused_macros)]
        macro_rules! lex_one_or_more {
            ($ kind : ident , $ expr : expr) => {
                lex_one_or_more!($expr)
                    .map(|element| factory::lex_named(kinds::Token::$kind, element))
            };
            ($ expr : expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|v| factory::lex_sequence(v))
            };
        }
        #[allow(unused_macros)]
        macro_rules! lex_repeated {
            ($ kind : ident , $ expr : expr , $ min : literal , $ max : literal) => {
                lex_repeated!($expr, $min, $max)
                    .map(|element| factory::lex_named(kinds::Token::$kind, element))
            };
            ($ expr : expr , $ min : literal , $ max : literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|v| factory::lex_sequence(v))
            };
        }
        #[allow(unused_macros)]
        macro_rules! lex_optional {
            ($ expr : expr) => {
                $expr
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| factory::lex_none()))
            };
        }
        #[allow(unused_macros)]
        macro_rules! lex_separated_by {
            ($ kind : ident , $ expr : expr , $ separator : expr) => {
                lex_separated_by!($expr, $separator)
                    .map(|element| factory::lex_named(kinds::Token::$kind, element))
            };
            ($ expr : expr , $ separator : expr) => {
                $expr
                    .then($separator.then($expr).repeated())
                    .map(|(first, rest)| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        factory::lex_sequence(v)
                    })
            };
        }
        #[allow(unused_macros)]
        macro_rules ! lex_trie { ($ ($ expr : expr) , *) => (choice :: < _ , ErrorType > (($ ($ expr) , *)) . map_with_span (| kind , span : SpanType | factory :: lex_named (kind , factory :: lex_chars (span . start () .. span . end ())))) }
        #[allow(unused_macros)]
        macro_rules! trieleaf {
            ($ kind : ident , $ string : literal) => {
                just($string).to(kinds::Token::$kind)
            };
            ($ kind : ident) => {
                empty().to(kinds::Token::$kind)
            };
        }
        #[allow(unused_macros)]
        macro_rules ! trieprefix { ($ string : literal , [$ ($ expr : expr) , *]) => (just ($ string) . ignore_then (choice :: < _ , ErrorType > (($ ($ expr) , *)))) }
        #[allow(unused_macros)]
        macro_rules! trivia_terminal {
            ($ kind : ident , $ literal : literal) => {
                just($literal).map_with_span(|_, span: SpanType| {
                    factory::cst_trivia_token(span.start()..span.end(), kinds::Token::$kind)
                })
            };
            ($ kind : ident , $ filter : expr) => {
                filter($filter).map_with_span(|_, span: SpanType| {
                    factory::cst_trivia_token(span.start()..span.end(), kinds::Token::$kind)
                })
            };
        }
        #[allow(unused_macros)]
        macro_rules ! trivia_token { ($ token_rule : ident) => { $ token_rule . clone () . map (| token : Rc < lex :: Node > | { if let lex :: Node :: Named (kind , element) = token . as_ref () { factory :: cst_trivia_token (element . range () , * kind) } else { unreachable ! ("a token rule should always return a named token, but rule {} returned {:?}" , stringify ! ($ token_rule) , token) } }) } ; }
        #[allow(unused_macros)]
        macro_rules ! trivia_trie { ($ ($ expr : expr) , *) => (choice :: < _ , ErrorType > (($ ($ expr) , *)) . map_with_span (| kind , span : SpanType | factory :: cst_trivia_token (span . start () .. span . end () , kind))) }
        #[allow(unused_macros)]
        macro_rules! terminal {
            ($ kind : ident , $ literal : literal) => {
                leading_trivia_parser
                    .clone()
                    .then(
                        just($literal).map_with_span(|_, span: SpanType| span.start()..span.end()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, range), trailing_trivia)| {
                        factory::cst_token(
                            range,
                            kinds::Token::$kind,
                            leading_trivia,
                            trailing_trivia,
                        )
                    })
            };
            ($ kind : ident , $ filter : expr) => {
                leading_trivia_parser
                    .clone()
                    .then(
                        filter($filter).map_with_span(|_, span: SpanType| span.start()..span.end()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, range), trailing_trivia)| {
                        factory::cst_token(
                            range,
                            kinds::Token::$kind,
                            leading_trivia,
                            trailing_trivia,
                        )
                    })
            };
        }
        #[allow(unused_macros)]
        macro_rules ! token { ($ token_rule : ident) => { leading_trivia_parser . clone () . then ($ token_rule . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , token) , trailing_trivia) : ((_ , Rc < lex :: Node >) , _) | { if let lex :: Node :: Named (kind , element) = token . as_ref () { factory :: cst_token (element . range () , * kind , leading_trivia , trailing_trivia) } else { unreachable ! ("a token rule should always return a named token, but rule {} returned {:?}" , stringify ! ($ token_rule) , token) } }) } ; }
        #[allow(unused_macros)]
        macro_rules! rule {
            ($ rule : ident) => {
                $rule.clone()
            };
        }
        #[allow(unused_macros)]
        macro_rules ! choice { ($ kind : ident , $ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr) , *)) } ; ($ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr) , *)) } ; }
        #[allow(unused_macros)]
        macro_rules ! seq { (@ exp $ head : expr , $ ($ tail : expr) , +) => { $ head . then (seq ! (@ exp $ ($ tail) , +)) } ; (@ exp $ head : expr) => { $ head } ; (@ args [$ ($ accum : expr ,) *] , $ current : expr , $ head : expr , $ ($ tail : expr) , +) => { seq ! (@ args [$ ($ accum ,) * $ current . 0 ,] , $ current . 1 , $ ($ tail) , +) } ; (@ args [$ ($ accum : expr ,) *] , $ current : expr , $ head : expr) => { vec ! [$ ($ accum ,) * $ current] } ; ($ kind : ident , $ ($ expr : expr) , +) => { seq ! (@ exp $ ($ expr) , +) . map (| v | factory :: cst_rule (kinds :: Rule :: $ kind , seq ! (@ args [] , v , $ ($ expr) , +))) } ; ($ ($ expr : expr) , +) => { seq ! (@ exp $ ($ expr) , +) . map (| v | factory :: cst_group (seq ! (@ args [] , v , $ ($ expr) , +))) } ; }
        #[allow(unused_macros)]
        macro_rules! zero_or_more {
            ($ kind : ident , $ expr : expr) => {
                $expr
                    .repeated()
                    .map(|children| factory::cst_rule(kinds::Rule::$kind, children))
            };
            ($ expr : expr) => {
                $expr
                    .repeated()
                    .map(|children| factory::cst_group(children))
            };
        }
        #[allow(unused_macros)]
        macro_rules! one_or_more {
            ($ kind : ident , $ expr : expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|children| factory::cst_rule(kinds::Rule::$kind, children))
            };
            ($ expr : expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|children| factory::cst_group(children))
            };
        }
        #[allow(unused_macros)]
        macro_rules! repeated {
            ($ kind : ident , $ expr : expr , $ min : literal , $ max : literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|children| factory::cst_rule(kinds::Rule::$kind, children))
            };
            ($ expr : expr , $ min : literal , $ max : literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|children| factory::cst_group(children))
            };
        }
        #[allow(unused_macros)]
        macro_rules! optional {
            ($ expr : expr) => {
                $expr
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| factory::cst_none()))
            };
        }
        #[allow(unused_macros)]
        macro_rules! separated_by {
            ($ kind : ident , $ expr : expr , $ separator : expr) => {
                $expr
                    .then($separator.then($expr).repeated())
                    .map(|(first, rest)| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        factory::cst_rule(kinds::Rule::$kind, v)
                    })
            };
            ($ expr : expr , $ separator : expr) => {
                $expr
                    .then($separator.then($expr).repeated())
                    .map(|(first, rest)| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        factory::cst_group(v)
                    })
            };
        }
        #[allow(unused_macros)]
        macro_rules! left_associative_binary_expression {
            ($ kind : ident , $ next_sibling : expr , $ operator : expr) => {
                $next_sibling
                    .clone()
                    .then($operator.then($next_sibling.clone()).repeated())
                    .map(|(first, rest)| {
                        if rest.is_empty() {
                            first
                        } else {
                            rest.into_iter().fold(
                                first,
                                |left_operand, (operator, right_operand)| {
                                    factory::cst_rule(
                                        kinds::Rule::$kind,
                                        vec![left_operand, operator, right_operand],
                                    )
                                },
                            )
                        }
                    })
            };
        }
        #[allow(unused_macros)]
        macro_rules! right_associative_binary_expression {
            ($ kind : ident , $ next_sibling : expr , $ operator : expr) => {
                $next_sibling
                    .clone()
                    .then($operator.then($next_sibling.clone()).repeated())
                    .map(|(first, rest)| {
                        if rest.is_empty() {
                            first
                        } else {
                            let mut last_operand = first;
                            let mut operand_operator_pairs = vec![];
                            for (operator, right_operand) in rest.into_iter() {
                                let left_operand =
                                    std::mem::replace(&mut last_operand, right_operand);
                                operand_operator_pairs.push((left_operand, operator))
                            }
                            operand_operator_pairs.into_iter().rfold(
                                last_operand,
                                |right_operand, (left_operand, operator)| {
                                    factory::cst_rule(
                                        kinds::Rule::$kind,
                                        vec![left_operand, operator, right_operand],
                                    )
                                },
                            )
                        }
                    })
            };
        }
        #[allow(unused_macros)]
        macro_rules! unary_prefix_expression {
            ($ kind : ident , $ next_sibling : expr , $ operator : expr) => {
                $operator
                    .repeated()
                    .then($next_sibling.clone())
                    .map(|(mut operators, operand)| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators.reverse();
                            operators
                                .into_iter()
                                .fold(operand, |right_operand, operator| {
                                    factory::cst_rule(
                                        kinds::Rule::$kind,
                                        vec![operator, right_operand],
                                    )
                                })
                        }
                    })
            };
        }
        #[allow(unused_macros)]
        macro_rules! unary_suffix_expression {
            ($ kind : ident , $ next_sibling : expr , $ operator : expr) => {
                $next_sibling
                    .clone()
                    .then($operator.repeated())
                    .map(|(operand, operators)| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators
                                .into_iter()
                                .fold(operand, |left_operand, operator| {
                                    factory::cst_rule(
                                        kinds::Rule::$kind,
                                        vec![left_operand, operator],
                                    )
                                })
                        }
                    })
            };
        }
        #[allow(unused_macros)]
        macro_rules! delimited_by {
            ($ kind : ident , $ open : expr , $ expr : expr , $ close : expr) => {
                seq!($kind, $open, $expr, $close)
            };
            ($ open : expr , $ expr : expr , $ close : expr) => {
                seq!($open, $expr, $close)
            };
        }
        #[allow(unused_macros)]
        macro_rules ! trie { ($ ($ expr : expr) , *) => (leading_trivia_parser . clone () . then (choice :: < _ , ErrorType > (($ ($ expr) , *)) . map_with_span (| kind , span : SpanType | (kind , span . start () .. span . end ()))) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , (kind , range)) , trailing_trivia) | { factory :: cst_token (range , kind , leading_trivia , trailing_trivia) })) }

        // Define all productions ---------------------------

        // ABICoderPragmaSpecifier = 'abicoder' «Identifier» ;
        {
            abi_coder_pragma_specifier_parser.define(
                seq!(
                    AbicoderPragmaSpecifier,
                    terminal!(Abicoder, "abicoder"),
                    token!(identifier_parser)
                )
                .boxed(),
            );
        }

        // AddSubExpression = Expression ( '+' | '-' ) Expression ;
        {
            add_sub_expression_parser.define(
                left_associative_binary_expression!(
                    AddSubExpression,
                    mul_div_mod_expression_parser,
                    choice!(terminal!(Plus, "+"), terminal!(Minus, "-"))
                )
                .boxed(),
            );
        }

        // AddressType = 'address' [ 'payable' ] ;
        {
            address_type_parser.define(
                seq!(
                    AddressType,
                    terminal!(Address, "address"),
                    optional!(terminal!(Payable, "payable"))
                )
                .boxed(),
            );
        }

        // AndExpression = Expression '&&' Expression ;
        {
            and_expression_parser.define(
                left_associative_binary_expression!(
                    AndExpression,
                    equality_comparison_expression_parser,
                    terminal!(AmpersandAmpersand, "&&")
                )
                .boxed(),
            );
        }

        // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
        {
            argument_list_parser.define(
                delimited_by!(
                    ArgumentList,
                    terminal!(OpenParen, "("),
                    optional!(choice!(
                        rule!(positional_argument_list_parser),
                        rule!(named_argument_list_parser)
                    )),
                    terminal!(CloseParen, ")")
                )
                .boxed(),
            );
        }

        // ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
        {
            array_literal_parser.define(
                delimited_by!(
                    ArrayLiteral,
                    terminal!(OpenBracket, "["),
                    separated_by!(
                        SeparatedExpressions,
                        rule!(expression_parser),
                        terminal!(Comma, ",")
                    ),
                    terminal!(CloseBracket, "]")
                )
                .boxed(),
            );
        }

        // «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
        {
            ascii_escape_parser.define(
                lex_terminal!(AsciiEscape, |&c: &char| c == 'n'
                    || c == 'r'
                    || c == 't'
                    || c == '\''
                    || c == '"'
                    || c == '\\'
                    || c == '\n'
                    || c == '\r')
                .boxed(),
            );
        }

        // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
        {
            ascii_string_literal_parser.define(
                lex_choice!(
                    AsciiStringLiteral,
                    lex_rule!(single_quoted_ascii_string_literal_parser),
                    lex_rule!(double_quoted_ascii_string_literal_parser)
                )
                .boxed(),
            );
        }

        // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
        {
            assembly_flags_parser.define(
                delimited_by!(
                    AssemblyFlags,
                    terminal!(OpenParen, "("),
                    separated_by!(
                        SeparatedDoubleQuotedAsciiStringLiterals,
                        token!(double_quoted_ascii_string_literal_parser),
                        terminal!(Comma, ",")
                    ),
                    terminal!(CloseParen, ")")
                )
                .boxed(),
            );
        }

        // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
        {
            assembly_statement_parser.define(
                seq!(
                    AssemblyStatement,
                    terminal!(Assembly, "assembly"),
                    optional!(terminal!(DoubleQuoteEvmasmDoubleQuote, "\"evmasm\"")),
                    optional!(rule!(assembly_flags_parser)),
                    rule!(yul_block_parser)
                )
                .boxed(),
            );
        }

        // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
        {
            assignment_expression_parser.define(
                left_associative_binary_expression!(
                    AssignmentExpression,
                    conditional_expression_parser,
                    choice!(
                        terminal!(Equal, "="),
                        terminal!(PipeEqual, "|="),
                        terminal!(CaretEqual, "^="),
                        terminal!(AmpersandEqual, "&="),
                        terminal!(LessLessEqual, "<<="),
                        terminal!(GreaterGreaterEqual, ">>="),
                        terminal!(GreaterGreaterGreaterEqual, ">>>="),
                        terminal!(PlusEqual, "+="),
                        terminal!(MinusEqual, "-="),
                        terminal!(StarEqual, "*="),
                        terminal!(SlashEqual, "/="),
                        terminal!(PercentEqual, "%=")
                    )
                )
                .boxed(),
            );
        }

        // BitAndExpression = Expression '&' Expression ;
        {
            bit_and_expression_parser.define(
                left_associative_binary_expression!(
                    BitAndExpression,
                    shift_expression_parser,
                    terminal!(Ampersand, "&")
                )
                .boxed(),
            );
        }

        // BitOrExpression = Expression '|' Expression ;
        {
            bit_or_expression_parser.define(
                left_associative_binary_expression!(
                    BitOrExpression,
                    bit_x_or_expression_parser,
                    terminal!(Pipe, "|")
                )
                .boxed(),
            );
        }

        // BitXOrExpression = Expression '^' Expression ;
        {
            bit_x_or_expression_parser.define(
                left_associative_binary_expression!(
                    BitXOrExpression,
                    bit_and_expression_parser,
                    terminal!(Caret, "^")
                )
                .boxed(),
            );
        }

        // Block = '{' { Statement | UncheckedBlock } '}' ;
        {
            block_parser.define(
                delimited_by!(
                    Block,
                    terminal!(OpenBrace, "{"),
                    zero_or_more!(choice!(
                        rule!(statement_parser),
                        rule!(unchecked_block_parser)
                    )),
                    terminal!(CloseBrace, "}")
                )
                .boxed(),
            );
        }

        // «BooleanLiteral» = 'true' | 'false' ;
        {
            boolean_literal_parser
                .define(lex_trie!(trieleaf!(False, "false"), trieleaf!(True, "true")).boxed());
        }

        // BreakStatement = 'break' ';' ;
        {
            break_statement_parser.define(
                seq!(
                    BreakStatement,
                    terminal!(Break, "break"),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
        {
            catch_clause_parser.define(
                seq!(
                    CatchClause,
                    terminal!(Catch, "catch"),
                    optional!(seq!(
                        optional!(token!(identifier_parser)),
                        rule!(parameter_list_parser)
                    )),
                    rule!(block_parser)
                )
                .boxed(),
            );
        }

        // ConditionalExpression = Expression '?' Expression ':' Expression ;
        {
            conditional_expression_parser.define(
                unary_suffix_expression!(
                    ConditionalExpression,
                    or_expression_parser,
                    seq!(
                        terminal!(Question, "?"),
                        rule!(expression_parser),
                        terminal!(Colon, ":"),
                        rule!(expression_parser)
                    )
                )
                .boxed(),
            );
        }

        // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
        {
            constant_definition_parser.define(
                seq!(
                    ConstantDefinition,
                    rule!(type_name_parser),
                    terminal!(Constant, "constant"),
                    token!(identifier_parser),
                    terminal!(Equal, "="),
                    rule!(expression_parser),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
        {
            constructor_attribute_parser.define(
                choice!(
                    ConstructorAttribute,
                    rule!(modifier_invocation_parser),
                    terminal!(Internal, "internal"),
                    terminal!(Payable, "payable"),
                    terminal!(Public, "public")
                )
                .boxed(),
            );
        }

        // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
        {
            constructor_definition_parser.define(
                seq!(
                    ConstructorDefinition,
                    terminal!(Constructor, "constructor"),
                    rule!(parameter_list_parser),
                    zero_or_more!(ConstructorAttributes, rule!(constructor_attribute_parser)),
                    rule!(block_parser)
                )
                .boxed(),
            );
        }

        // ContinueStatement = 'continue' ';' ;
        {
            continue_statement_parser.define(
                seq!(
                    ContinueStatement,
                    terminal!(Continue, "continue"),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
        {
            contract_body_element_parser.define(
                choice!(
                    ContractBodyElement,
                    rule!(using_directive_parser),
                    rule!(constructor_definition_parser),
                    rule!(function_definition_parser),
                    rule!(fallback_function_definition_parser),
                    rule!(receive_function_definition_parser),
                    rule!(modifier_definition_parser),
                    rule!(struct_definition_parser),
                    rule!(enum_definition_parser),
                    rule!(user_defined_value_type_definition_parser),
                    rule!(event_definition_parser),
                    rule!(error_definition_parser),
                    rule!(state_variable_declaration_parser)
                )
                .boxed(),
            );
        }

        // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        {
            contract_definition_parser.define(
                seq!(
                    ContractDefinition,
                    optional!(terminal!(Abstract, "abstract")),
                    terminal!(Contract, "contract"),
                    token!(identifier_parser),
                    optional!(rule!(inheritance_specifier_list_parser)),
                    delimited_by!(
                        DelimitedContractBodyElements,
                        terminal!(OpenBrace, "{"),
                        zero_or_more!(ContractBodyElements, rule!(contract_body_element_parser)),
                        terminal!(CloseBrace, "}")
                    )
                )
                .boxed(),
            );
        }

        // DataLocation = 'memory' | 'storage' | 'calldata' ;
        {
            data_location_parser.define(
                choice!(
                    DataLocation,
                    terminal!(Memory, "memory"),
                    terminal!(Storage, "storage"),
                    terminal!(Calldata, "calldata")
                )
                .boxed(),
            );
        }

        // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
        {
            decimal_exponent_parser.define(
                lex_seq!(
                    DecimalExponent,
                    lex_terminal!(|&c: &char| c == 'e' || c == 'E'),
                    lex_optional!(lex_terminal!(Minus, '-')),
                    lex_rule!(decimal_integer_parser)
                )
                .boxed(),
            );
        }

        // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
        {
            decimal_float_parser.define(
                lex_seq!(
                    DecimalFloat,
                    lex_optional!(lex_rule!(decimal_integer_parser)),
                    lex_terminal!(Period, '.'),
                    lex_rule!(decimal_integer_parser)
                )
                .boxed(),
            );
        }

        // «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
        {
            decimal_integer_parser.define(
                lex_seq!(
                    DecimalInteger,
                    lex_terminal!(|&c: &char| ('0' <= c && c <= '9')),
                    lex_zero_or_more!(lex_seq!(
                        lex_optional!(lex_terminal!(Underscore, '_')),
                        lex_terminal!(|&c: &char| ('0' <= c && c <= '9'))
                    ))
                )
                .boxed(),
            );
        }

        // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
        {
            decimal_number_parser.define(
                lex_seq!(
                    DecimalNumber,
                    lex_choice!(
                        lex_rule!(decimal_integer_parser),
                        lex_rule!(decimal_float_parser)
                    ),
                    lex_optional!(lex_rule!(decimal_exponent_parser))
                )
                .boxed(),
            );
        }

        // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
        {
            definition_parser.define(
                choice!(
                    Definition,
                    rule!(contract_definition_parser),
                    rule!(interface_definition_parser),
                    rule!(library_definition_parser),
                    rule!(function_definition_parser),
                    rule!(constant_definition_parser),
                    rule!(struct_definition_parser),
                    rule!(enum_definition_parser),
                    rule!(user_defined_value_type_definition_parser),
                    rule!(error_definition_parser)
                )
                .boxed(),
            );
        }

        // DeleteStatement = 'delete' «Identifier» ';' ;
        {
            delete_statement_parser.define(
                seq!(
                    DeleteStatement,
                    terminal!(Delete, "delete"),
                    token!(identifier_parser),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // Directive = PragmaDirective | ImportDirective | UsingDirective ;
        {
            directive_parser.define(
                choice!(
                    Directive,
                    rule!(pragma_directive_parser),
                    rule!(import_directive_parser),
                    rule!(using_directive_parser)
                )
                .boxed(),
            );
        }

        // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
        {
            do_while_statement_parser.define(
                seq!(
                    DoWhileStatement,
                    terminal!(Do, "do"),
                    rule!(statement_parser),
                    terminal!(While, "while"),
                    delimited_by!(
                        DelimitedExpression,
                        terminal!(OpenParen, "("),
                        rule!(expression_parser),
                        terminal!(CloseParen, ")")
                    ),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
        {
            double_quoted_ascii_string_literal_parser.define(
                lex_seq!(
                    DoubleQuotedAsciiStringLiteral,
                    lex_terminal!(DoubleQuote, "\""),
                    lex_zero_or_more!(
                        Runs,
                        lex_choice!(
                            Run,
                            lex_one_or_more!(
                                Chars,
                                lex_terminal!(Char, |&c: &char| (' ' <= c && c <= '~')
                                    && c != '"'
                                    && c != '\\')
                            ),
                            lex_rule!(escape_sequence_parser)
                        )
                    ),
                    lex_terminal!(DoubleQuote, "\"")
                )
                .boxed(),
            );
        }

        // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
        {
            double_quoted_unicode_string_literal_parser.define(
                lex_seq!(
                    DoubleQuotedUnicodeStringLiteral,
                    lex_terminal!(UnicodeDoubleQuote, "unicode\""),
                    lex_zero_or_more!(
                        Runs,
                        lex_choice!(
                            Run,
                            lex_one_or_more!(
                                Chars,
                                lex_terminal!(Char, |&c: &char| c != '"'
                                    && c != '\\'
                                    && c != '\n'
                                    && c != '\r')
                            ),
                            lex_rule!(escape_sequence_parser)
                        )
                    ),
                    lex_terminal!(DoubleQuote, "\"")
                )
                .boxed(),
            );
        }

        // ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
        {
            elementary_type_parser.define(
                choice!(
                    ElementaryType,
                    terminal!(Bool, "bool"),
                    terminal!(String, "string"),
                    rule!(address_type_parser),
                    token!(fixed_bytes_type_parser),
                    token!(signed_integer_type_parser),
                    token!(unsigned_integer_type_parser),
                    token!(signed_fixed_type_parser),
                    token!(unsigned_fixed_type_parser)
                )
                .boxed(),
            );
        }

        // EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
        {
            emit_statement_parser.define(
                seq!(
                    EmitStatement,
                    terminal!(Emit, "emit"),
                    rule!(identifier_path_parser),
                    rule!(argument_list_parser),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // EndOfFileTrivia = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
        {
            end_of_file_trivia_parser.define(
                zero_or_more!(
                    EndOfFileTrivia,
                    choice!(
                        trivia_token!(whitespace_parser),
                        trivia_token!(multiline_comment_parser),
                        trivia_token!(single_line_comment_parser)
                    )
                )
                .boxed(),
            );
        }

        // «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
        {
            end_of_line_parser.define(
                lex_one_or_more!(EndOfLine, lex_terminal!(|&c: &char| c == '\r' || c == '\n'))
                    .boxed(),
            );
        }

        // EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
        {
            enum_definition_parser.define(
                seq!(
                    EnumDefinition,
                    terminal!(Enum, "enum"),
                    token!(identifier_parser),
                    delimited_by!(
                        DelimitedSeparatedIdentifiers,
                        terminal!(OpenBrace, "{"),
                        separated_by!(
                            SeparatedIdentifiers,
                            token!(identifier_parser),
                            terminal!(Comma, ",")
                        ),
                        terminal!(CloseBrace, "}")
                    )
                )
                .boxed(),
            );
        }

        // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
        {
            equality_comparison_expression_parser.define(
                left_associative_binary_expression!(
                    EqualityComparisonExpression,
                    order_comparison_expression_parser,
                    choice!(terminal!(EqualEqual, "=="), terminal!(BangEqual, "!="))
                )
                .boxed(),
            );
        }

        // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
        {
            error_definition_parser.define(
                seq!(
                    ErrorDefinition,
                    terminal!(Error, "error"),
                    token!(identifier_parser),
                    delimited_by!(
                        DelimitedSeparatedErrorParameters,
                        terminal!(OpenParen, "("),
                        optional!(separated_by!(
                            SeparatedErrorParameters,
                            rule!(error_parameter_parser),
                            terminal!(Comma, ",")
                        )),
                        terminal!(CloseParen, ")")
                    ),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // ErrorParameter = TypeName [ «Identifier» ] ;
        {
            error_parameter_parser.define(
                seq!(
                    ErrorParameter,
                    rule!(type_name_parser),
                    optional!(token!(identifier_parser))
                )
                .boxed(),
            );
        }

        // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
        {
            escape_sequence_parser.define(
                lex_seq!(
                    EscapeSequence,
                    lex_terminal!(Backslash, '\\'),
                    lex_trie!(
                        trieleaf!(Linefeed, "\n"),
                        trieleaf!(CarriageReturn, "\r"),
                        trieleaf!(DoubleQuote, "\""),
                        trieleaf!(Quote, "'"),
                        trieleaf!(Backslash, "\\"),
                        trieleaf!(LatinSmallLetterN, "n"),
                        trieleaf!(LatinSmallLetterR, "r"),
                        trieleaf!(LatinSmallLetterT, "t")
                    )
                )
                .boxed(),
            );
        }

        // EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
        {
            event_definition_parser.define(
                seq!(
                    EventDefinition,
                    terminal!(Event, "event"),
                    token!(identifier_parser),
                    delimited_by!(
                        DelimitedSeparatedEventParameters,
                        terminal!(OpenParen, "("),
                        optional!(separated_by!(
                            SeparatedEventParameters,
                            rule!(event_parameter_parser),
                            terminal!(Comma, ",")
                        )),
                        terminal!(CloseParen, ")")
                    ),
                    optional!(terminal!(Anonymous, "anonymous")),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
        {
            event_parameter_parser.define(
                seq!(
                    EventParameter,
                    rule!(type_name_parser),
                    optional!(terminal!(Indexed, "indexed")),
                    optional!(token!(identifier_parser))
                )
                .boxed(),
            );
        }

        // ExperimentalPragmaSpecifier = 'experimental' «Identifier» ;
        {
            experimental_pragma_specifier_parser.define(
                seq!(
                    ExperimentalPragmaSpecifier,
                    terminal!(Experimental, "experimental"),
                    token!(identifier_parser)
                )
                .boxed(),
            );
        }

        // (* 0.0.0 *) ExponentiationExpression = Expression '**' Expression ;
        // (* 0.6.0 *) ExponentiationExpression = Expression '**' Expression ;
        if version_0_6_0 <= version {
            exponentiation_expression_parser.define(
                right_associative_binary_expression!(
                    ExponentiationExpression,
                    unary_suffix_expression_parser,
                    terminal!(StarStar, "**")
                )
                .boxed(),
            );
        } else {
            exponentiation_expression_parser.define(
                left_associative_binary_expression!(
                    ExponentiationExpression,
                    unary_suffix_expression_parser,
                    terminal!(StarStar, "**")
                )
                .boxed(),
            );
        }

        // Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
        {
            expression_parser.define(rule!(assignment_expression_parser).boxed());
        }

        // ExpressionStatement = Expression ';' ;
        {
            expression_statement_parser.define(
                seq!(
                    ExpressionStatement,
                    rule!(expression_parser),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
        {
            fallback_function_attribute_parser.define(
                choice!(
                    FallbackFunctionAttribute,
                    rule!(modifier_invocation_parser),
                    rule!(override_specifier_parser),
                    terminal!(External, "external"),
                    terminal!(Payable, "payable"),
                    terminal!(Pure, "pure"),
                    terminal!(View, "view"),
                    terminal!(Virtual, "virtual")
                )
                .boxed(),
            );
        }

        // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
        {
            fallback_function_definition_parser.define(
                seq!(
                    FallbackFunctionDefinition,
                    terminal!(Fallback, "fallback"),
                    rule!(parameter_list_parser),
                    zero_or_more!(
                        FallbackFunctionAttributes,
                        rule!(fallback_function_attribute_parser)
                    ),
                    optional!(seq!(
                        terminal!(Returns, "returns"),
                        rule!(parameter_list_parser)
                    )),
                    choice!(terminal!(Semicolon, ";"), rule!(block_parser))
                )
                .boxed(),
            );
        }

        // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
        {
            fixed_bytes_type_parser.define(
                lex_seq!(
                    FixedBytesType,
                    lex_terminal!(Bytes, "bytes"),
                    lex_trie!(
                        trieprefix!(
                            "1",
                            [
                                trieleaf!(OneZero, "0"),
                                trieleaf!(OneOne, "1"),
                                trieleaf!(OneTwo, "2"),
                                trieleaf!(OneThree, "3"),
                                trieleaf!(OneFour, "4"),
                                trieleaf!(OneFive, "5"),
                                trieleaf!(OneSix, "6"),
                                trieleaf!(OneSeven, "7"),
                                trieleaf!(OneEight, "8"),
                                trieleaf!(OneNine, "9"),
                                trieleaf!(One)
                            ]
                        ),
                        trieprefix!(
                            "2",
                            [
                                trieleaf!(TwoZero, "0"),
                                trieleaf!(TwoOne, "1"),
                                trieleaf!(TwoTwo, "2"),
                                trieleaf!(TwoThree, "3"),
                                trieleaf!(TwoFour, "4"),
                                trieleaf!(TwoFive, "5"),
                                trieleaf!(TwoSix, "6"),
                                trieleaf!(TwoSeven, "7"),
                                trieleaf!(TwoEight, "8"),
                                trieleaf!(TwoNine, "9"),
                                trieleaf!(Two)
                            ]
                        ),
                        trieprefix!(
                            "3",
                            [
                                trieleaf!(ThreeZero, "0"),
                                trieleaf!(ThreeOne, "1"),
                                trieleaf!(ThreeTwo, "2"),
                                trieleaf!(Three)
                            ]
                        ),
                        trieleaf!(Four, "4"),
                        trieleaf!(Five, "5"),
                        trieleaf!(Six, "6"),
                        trieleaf!(Seven, "7"),
                        trieleaf!(Eight, "8"),
                        trieleaf!(Nine, "9")
                    )
                )
                .boxed(),
            );
        }

        // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
        {
            for_statement_parser.define(
                seq!(
                    ForStatement,
                    terminal!(For, "for"),
                    delimited_by!(
                        terminal!(OpenParen, "("),
                        seq!(
                            choice!(rule!(simple_statement_parser), terminal!(Semicolon, ";")),
                            choice!(
                                rule!(expression_statement_parser),
                                terminal!(Semicolon, ";")
                            ),
                            optional!(rule!(expression_parser))
                        ),
                        terminal!(CloseParen, ")")
                    ),
                    rule!(statement_parser)
                )
                .boxed(),
            );
        }

        // FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
        {
            function_attribute_parser.define(
                choice!(
                    FunctionAttribute,
                    rule!(modifier_invocation_parser),
                    rule!(override_specifier_parser),
                    terminal!(External, "external"),
                    terminal!(Internal, "internal"),
                    terminal!(Payable, "payable"),
                    terminal!(Private, "private"),
                    terminal!(Public, "public"),
                    terminal!(Pure, "pure"),
                    terminal!(View, "view"),
                    terminal!(Virtual, "virtual")
                )
                .boxed(),
            );
        }

        // FunctionCallExpression = Expression [ '{' NamedArgument  { ',' NamedArgument } '}' ] ArgumentList ;
        {
            function_call_expression_parser.define(
                unary_suffix_expression!(
                    FunctionCallExpression,
                    member_access_expression_parser,
                    seq!(
                        optional!(delimited_by!(
                            DelimitedSeparatedNamedArguments,
                            terminal!(OpenBrace, "{"),
                            separated_by!(
                                SeparatedNamedArguments,
                                rule!(named_argument_parser),
                                terminal!(Comma, ",")
                            ),
                            terminal!(CloseBrace, "}")
                        )),
                        rule!(argument_list_parser)
                    )
                )
                .boxed(),
            );
        }

        // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
        {
            function_definition_parser.define(
                seq!(
                    FunctionDefinition,
                    terminal!(Function, "function"),
                    choice!(
                        token!(identifier_parser),
                        terminal!(Fallback, "fallback"),
                        terminal!(Receive, "receive")
                    ),
                    rule!(parameter_list_parser),
                    zero_or_more!(FunctionAttributes, rule!(function_attribute_parser)),
                    optional!(seq!(
                        terminal!(Returns, "returns"),
                        rule!(parameter_list_parser)
                    )),
                    choice!(terminal!(Semicolon, ";"), rule!(block_parser))
                )
                .boxed(),
            );
        }

        // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
        {
            function_type_parser.define(
                seq!(
                    FunctionType,
                    terminal!(Function, "function"),
                    rule!(parameter_list_parser),
                    zero_or_more!(choice!(
                        terminal!(Internal, "internal"),
                        terminal!(External, "external"),
                        terminal!(Private, "private"),
                        terminal!(Public, "public"),
                        terminal!(Pure, "pure"),
                        terminal!(View, "view"),
                        terminal!(Payable, "payable")
                    )),
                    optional!(seq!(
                        terminal!(Returns, "returns"),
                        rule!(parameter_list_parser)
                    ))
                )
                .boxed(),
            );
        }

        // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
        {
            hex_byte_escape_parser.define(
                lex_seq!(
                    HexByteEscape,
                    lex_terminal!(LatinSmallLetterX, 'x'),
                    lex_repeated!(
                        lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                            || ('a' <= c && c <= 'f')
                            || ('A' <= c && c <= 'F')),
                        2usize,
                        2usize
                    )
                )
                .boxed(),
            );
        }

        // «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
        {
            hex_character_parser.define(
                lex_terminal!(HexCharacter, |&c: &char| ('0' <= c && c <= '9')
                    || ('a' <= c && c <= 'f')
                    || ('A' <= c && c <= 'F'))
                .boxed(),
            );
        }

        // «HexNumber» = '0x' «HexCharacter» { [ '_' ] «HexCharacter» } ;
        {
            hex_number_parser.define(
                lex_seq!(
                    HexNumber,
                    lex_terminal!(ZeroX, "0x"),
                    lex_seq!(
                        lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                            || ('a' <= c && c <= 'f')
                            || ('A' <= c && c <= 'F')),
                        lex_zero_or_more!(lex_seq!(
                            lex_optional!(lex_terminal!(Underscore, '_')),
                            lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                || ('a' <= c && c <= 'f')
                                || ('A' <= c && c <= 'F'))
                        ))
                    )
                )
                .boxed(),
            );
        }

        // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
        {
            hex_string_literal_parser.define(
                lex_seq!(
                    HexStringLiteral,
                    lex_terminal!(Hex, "hex"),
                    lex_choice!(
                        lex_seq!(
                            DelimitedPossiblySeparatedPairsOfHexDigits,
                            lex_terminal!(DoubleQuote, "\""),
                            lex_optional!(lex_rule!(possibly_separated_pairs_of_hex_digits_parser)),
                            lex_terminal!(DoubleQuote, "\"")
                        ),
                        lex_seq!(
                            DelimitedPossiblySeparatedPairsOfHexDigits,
                            lex_terminal!(Quote, "'"),
                            lex_optional!(lex_rule!(possibly_separated_pairs_of_hex_digits_parser)),
                            lex_terminal!(Quote, "'")
                        )
                    )
                )
                .boxed(),
            );
        }

        // «Identifier» = «RawIdentifier» - «Keyword» ;
        {
            identifier_parser.define(
                difference(
                    lex_rule!(raw_identifier_parser),
                    lex_trie!(trieleaf!(False, "false"), trieleaf!(True, "true")),
                )
                .boxed(),
            );
        }

        // «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
        {
            identifier_part_parser.define(
                lex_terminal!(IdentifierPart, |&c: &char| c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z')
                    || ('0' <= c && c <= '9'))
                .boxed(),
            );
        }

        // IdentifierPath = «Identifier»  { '.' «Identifier» } ;
        {
            identifier_path_parser.define(
                separated_by!(
                    IdentifierPath,
                    token!(identifier_parser),
                    terminal!(Period, ".")
                )
                .boxed(),
            );
        }

        // «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
        {
            identifier_start_parser.define(
                lex_terminal!(IdentifierStart, |&c: &char| c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z'))
                .boxed(),
            );
        }

        // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
        {
            if_statement_parser.define(
                seq!(
                    IfStatement,
                    terminal!(If, "if"),
                    delimited_by!(
                        DelimitedExpression,
                        terminal!(OpenParen, "("),
                        rule!(expression_parser),
                        terminal!(CloseParen, ")")
                    ),
                    rule!(statement_parser),
                    optional!(seq!(terminal!(Else, "else"), rule!(statement_parser)))
                )
                .boxed(),
            );
        }

        // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
        {
            import_directive_parser.define(
                seq!(
                    ImportDirective,
                    terminal!(Import, "import"),
                    choice!(
                        rule!(simple_import_directive_parser),
                        rule!(star_import_directive_parser),
                        rule!(selecting_import_directive_parser)
                    ),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // ImportPath = «AsciiStringLiteral» ;
        {
            import_path_parser.define(token!(ascii_string_literal_parser).boxed());
        }

        // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
        {
            index_access_expression_parser.define(
                unary_suffix_expression!(
                    IndexAccessExpression,
                    primary_expression_parser,
                    delimited_by!(
                        terminal!(OpenBracket, "["),
                        seq!(
                            optional!(rule!(expression_parser)),
                            optional!(seq!(
                                terminal!(Colon, ":"),
                                optional!(rule!(expression_parser))
                            ))
                        ),
                        terminal!(CloseBracket, "]")
                    )
                )
                .boxed(),
            );
        }

        // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
        {
            inheritance_specifier_parser.define(
                seq!(
                    InheritanceSpecifier,
                    rule!(identifier_path_parser),
                    optional!(rule!(argument_list_parser))
                )
                .boxed(),
            );
        }

        // InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
        {
            inheritance_specifier_list_parser.define(
                seq!(
                    InheritanceSpecifierList,
                    terminal!(Is, "is"),
                    separated_by!(
                        SeparatedInheritanceSpecifiers,
                        rule!(inheritance_specifier_parser),
                        terminal!(Comma, ",")
                    )
                )
                .boxed(),
            );
        }

        // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        {
            interface_definition_parser.define(
                seq!(
                    InterfaceDefinition,
                    terminal!(Interface, "interface"),
                    token!(identifier_parser),
                    optional!(rule!(inheritance_specifier_list_parser)),
                    delimited_by!(
                        DelimitedContractBodyElements,
                        terminal!(OpenBrace, "{"),
                        zero_or_more!(ContractBodyElements, rule!(contract_body_element_parser)),
                        terminal!(CloseBrace, "}")
                    )
                )
                .boxed(),
            );
        }

        // «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
        {
            keyword_parser
                .define(lex_trie!(trieleaf!(False, "false"), trieleaf!(True, "true")).boxed());
        }

        // LeadingTrivia = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
        {
            leading_trivia_parser.define(
                zero_or_more!(
                    LeadingTrivia,
                    choice!(
                        trivia_token!(whitespace_parser),
                        trivia_token!(end_of_line_parser),
                        trivia_token!(multiline_comment_parser),
                        trivia_token!(single_line_comment_parser)
                    )
                )
                .boxed(),
            );
        }

        // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
        {
            library_definition_parser.define(
                seq!(
                    LibraryDefinition,
                    terminal!(Library, "library"),
                    token!(identifier_parser),
                    delimited_by!(
                        DelimitedContractBodyElements,
                        terminal!(OpenBrace, "{"),
                        zero_or_more!(ContractBodyElements, rule!(contract_body_element_parser)),
                        terminal!(CloseBrace, "}")
                    )
                )
                .boxed(),
            );
        }

        // MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
        {
            mapping_type_parser.define(
                seq!(
                    MappingType,
                    terminal!(Mapping, "mapping"),
                    delimited_by!(
                        terminal!(OpenParen, "("),
                        seq!(
                            choice!(rule!(elementary_type_parser), rule!(identifier_path_parser)),
                            terminal!(EqualGreater, "=>"),
                            rule!(type_name_parser)
                        ),
                        terminal!(CloseParen, ")")
                    )
                )
                .boxed(),
            );
        }

        // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
        {
            member_access_expression_parser.define(
                unary_suffix_expression!(
                    MemberAccessExpression,
                    index_access_expression_parser,
                    seq!(
                        terminal!(Period, "."),
                        choice!(token!(identifier_parser), terminal!(Address, "address"))
                    )
                )
                .boxed(),
            );
        }

        // ModifierAttribute = OverrideSpecifier | 'virtual' ;
        {
            modifier_attribute_parser.define(
                choice!(
                    ModifierAttribute,
                    rule!(override_specifier_parser),
                    terminal!(Virtual, "virtual")
                )
                .boxed(),
            );
        }

        // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
        {
            modifier_definition_parser.define(
                seq!(
                    ModifierDefinition,
                    terminal!(Modifier, "modifier"),
                    token!(identifier_parser),
                    optional!(rule!(parameter_list_parser)),
                    zero_or_more!(ModifierAttributes, rule!(modifier_attribute_parser)),
                    choice!(terminal!(Semicolon, ";"), rule!(block_parser))
                )
                .boxed(),
            );
        }

        // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
        {
            modifier_invocation_parser.define(
                seq!(
                    ModifierInvocation,
                    rule!(identifier_path_parser),
                    optional!(rule!(argument_list_parser))
                )
                .boxed(),
            );
        }

        // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
        {
            mul_div_mod_expression_parser.define(
                left_associative_binary_expression!(
                    MulDivModExpression,
                    exponentiation_expression_parser,
                    choice!(
                        terminal!(Star, "*"),
                        terminal!(Slash, "/"),
                        terminal!(Percent, "%")
                    )
                )
                .boxed(),
            );
        }

        // «MultilineComment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
        {
            multiline_comment_parser.define(
                lex_seq!(
                    MultilineComment,
                    lex_terminal!(SlashStar, "/*"),
                    lex_seq!(
                        Content,
                        lex_zero_or_more!(lex_choice!(
                            lex_terminal!(NotStar, |&c: &char| c != '*'),
                            lex_seq!(
                                lex_one_or_more!(Stars, lex_terminal!(Star, '*')),
                                lex_terminal!(|&c: &char| c != '*' && c != '/')
                            )
                        )),
                        lex_zero_or_more!(Stars, lex_terminal!(Star, '*'))
                    ),
                    lex_terminal!(StarSlash, "*/")
                )
                .boxed(),
            );
        }

        // NamedArgument = «Identifier» ':' Expression ;
        {
            named_argument_parser.define(
                seq!(
                    NamedArgument,
                    token!(identifier_parser),
                    terminal!(Colon, ":"),
                    rule!(expression_parser)
                )
                .boxed(),
            );
        }

        // NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
        {
            named_argument_list_parser.define(
                delimited_by!(
                    NamedArgumentList,
                    terminal!(OpenBrace, "{"),
                    optional!(separated_by!(
                        SeparatedNamedArguments,
                        rule!(named_argument_parser),
                        terminal!(Comma, ",")
                    )),
                    terminal!(CloseBrace, "}")
                )
                .boxed(),
            );
        }

        // NewExpression = 'new' IdentifierPath ArgumentList ;
        {
            new_expression_parser.define(
                seq!(
                    NewExpression,
                    terminal!(New, "new"),
                    rule!(identifier_path_parser),
                    rule!(argument_list_parser)
                )
                .boxed(),
            );
        }

        // «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
        {
            number_unit_parser.define(
                lex_trie!(
                    trieleaf!(Days, "days"),
                    trieleaf!(Ether, "ether"),
                    trieleaf!(Finney, "finney"),
                    trieleaf!(Gwei, "gwei"),
                    trieleaf!(Hours, "hours"),
                    trieleaf!(Minutes, "minutes"),
                    trieprefix!(
                        "s",
                        [trieleaf!(Seconds, "econds"), trieleaf!(Szabo, "zabo")]
                    ),
                    trieprefix!("we", [trieleaf!(Weeks, "eks"), trieleaf!(Wei, "i")]),
                    trieleaf!(Years, "years")
                )
                .boxed(),
            );
        }

        // «NumericLiteral» = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
        {
            numeric_literal_parser.define(
                lex_seq!(
                    NumericLiteral,
                    lex_choice!(
                        lex_rule!(decimal_number_parser),
                        lex_rule!(hex_number_parser)
                    ),
                    lex_optional!(lex_trie!(
                        trieleaf!(Days, "days"),
                        trieleaf!(Ether, "ether"),
                        trieleaf!(Finney, "finney"),
                        trieleaf!(Gwei, "gwei"),
                        trieleaf!(Hours, "hours"),
                        trieleaf!(Minutes, "minutes"),
                        trieprefix!(
                            "s",
                            [trieleaf!(Seconds, "econds"), trieleaf!(Szabo, "zabo")]
                        ),
                        trieprefix!("we", [trieleaf!(Weeks, "eks"), trieleaf!(Wei, "i")]),
                        trieleaf!(Years, "years")
                    ))
                )
                .boxed(),
            );
        }

        // OrExpression = Expression '||' Expression ;
        {
            or_expression_parser.define(
                left_associative_binary_expression!(
                    OrExpression,
                    and_expression_parser,
                    terminal!(PipePipe, "||")
                )
                .boxed(),
            );
        }

        // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
        {
            order_comparison_expression_parser.define(
                left_associative_binary_expression!(
                    OrderComparisonExpression,
                    bit_or_expression_parser,
                    choice!(
                        terminal!(Less, "<"),
                        terminal!(Greater, ">"),
                        terminal!(LessEqual, "<="),
                        terminal!(GreaterEqual, ">=")
                    )
                )
                .boxed(),
            );
        }

        // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
        {
            override_specifier_parser.define(
                seq!(
                    OverrideSpecifier,
                    terminal!(Override, "override"),
                    optional!(delimited_by!(
                        DelimitedSeparatedIdentifierPaths,
                        terminal!(OpenParen, "("),
                        separated_by!(
                            SeparatedIdentifierPaths,
                            rule!(identifier_path_parser),
                            terminal!(Comma, ",")
                        ),
                        terminal!(CloseParen, ")")
                    ))
                )
                .boxed(),
            );
        }

        // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
        {
            parameter_declaration_parser.define(
                seq!(
                    ParameterDeclaration,
                    rule!(type_name_parser),
                    optional!(rule!(data_location_parser)),
                    optional!(token!(identifier_parser))
                )
                .boxed(),
            );
        }

        // ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
        {
            parameter_list_parser.define(
                delimited_by!(
                    ParameterList,
                    terminal!(OpenParen, "("),
                    optional!(separated_by!(
                        SeparatedParameterDeclarations,
                        rule!(parameter_declaration_parser),
                        terminal!(Comma, ",")
                    )),
                    terminal!(CloseParen, ")")
                )
                .boxed(),
            );
        }

        // ParenthesisExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
        {
            parenthesis_expression_parser.define(
                delimited_by!(
                    ParenthesisExpression,
                    terminal!(OpenParen, "("),
                    separated_by!(
                        SeparatedExpressions,
                        optional!(rule!(expression_parser)),
                        terminal!(Comma, ",")
                    ),
                    terminal!(CloseParen, ")")
                )
                .boxed(),
            );
        }

        // PayableExpression = 'payable' ArgumentList ;
        {
            payable_expression_parser.define(
                seq!(
                    PayableExpression,
                    terminal!(Payable, "payable"),
                    rule!(argument_list_parser)
                )
                .boxed(),
            );
        }

        // PositionalArgumentList = Expression  { ',' Expression } ;
        {
            positional_argument_list_parser.define(
                separated_by!(
                    PositionalArgumentList,
                    rule!(expression_parser),
                    terminal!(Comma, ",")
                )
                .boxed(),
            );
        }

        // «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
        {
            possibly_separated_pairs_of_hex_digits_parser.define(
                lex_seq!(
                    PossiblySeparatedPairsOfHexDigits,
                    lex_repeated!(
                        lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                            || ('a' <= c && c <= 'f')
                            || ('A' <= c && c <= 'F')),
                        2usize,
                        2usize
                    ),
                    lex_zero_or_more!(lex_seq!(
                        lex_optional!(lex_terminal!(Underscore, '_')),
                        lex_repeated!(
                            lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                || ('a' <= c && c <= 'f')
                                || ('A' <= c && c <= 'F')),
                            2usize,
                            2usize
                        )
                    ))
                )
                .boxed(),
            );
        }

        // PragmaDirective = 'pragma' ( VersionPragmaSpecifier | ABICoderPragmaSpecifier | ExperimentalPragmaSpecifier ) ';' ;
        {
            pragma_directive_parser.define(
                seq!(
                    PragmaDirective,
                    terminal!(Pragma, "pragma"),
                    choice!(
                        rule!(version_pragma_specifier_parser),
                        rule!(abi_coder_pragma_specifier_parser),
                        rule!(experimental_pragma_specifier_parser)
                    ),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | «NumericLiteral» | «BooleanLiteral» | «Identifier» ;
        {
            primary_expression_parser.define(
                choice!(
                    PrimaryExpression,
                    rule!(payable_expression_parser),
                    rule!(type_expression_parser),
                    rule!(new_expression_parser),
                    rule!(parenthesis_expression_parser),
                    rule!(array_literal_parser),
                    token!(ascii_string_literal_parser),
                    token!(unicode_string_literal_parser),
                    token!(hex_string_literal_parser),
                    token!(numeric_literal_parser),
                    token!(boolean_literal_parser),
                    token!(identifier_parser)
                )
                .boxed(),
            );
        }

        // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
        {
            raw_identifier_parser.define(
                lex_seq!(
                    RawIdentifier,
                    lex_terminal!(|&c: &char| c == '_'
                        || c == '$'
                        || ('a' <= c && c <= 'z')
                        || ('A' <= c && c <= 'Z')),
                    lex_zero_or_more!(lex_terminal!(|&c: &char| c == '_'
                        || c == '$'
                        || ('a' <= c && c <= 'z')
                        || ('A' <= c && c <= 'Z')
                        || ('0' <= c && c <= '9')))
                )
                .boxed(),
            );
        }

        // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
        {
            receive_function_attribute_parser.define(
                choice!(
                    ReceiveFunctionAttribute,
                    rule!(modifier_invocation_parser),
                    rule!(override_specifier_parser),
                    terminal!(External, "external"),
                    terminal!(Payable, "payable"),
                    terminal!(Virtual, "virtual")
                )
                .boxed(),
            );
        }

        // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
        {
            receive_function_definition_parser.define(
                seq!(
                    ReceiveFunctionDefinition,
                    terminal!(Receive, "receive"),
                    rule!(parameter_list_parser),
                    zero_or_more!(
                        ReceiveFunctionAttributes,
                        rule!(receive_function_attribute_parser)
                    ),
                    choice!(terminal!(Semicolon, ";"), rule!(block_parser))
                )
                .boxed(),
            );
        }

        // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
        {
            reserved_keyword_parser.define(
                lex_trie!(
                    trieprefix!(
                        "a",
                        [
                            trieleaf!(After, "fter"),
                            trieleaf!(Alias, "lias"),
                            trieleaf!(Apply, "pply"),
                            trieleaf!(Auto, "uto")
                        ]
                    ),
                    trieleaf!(Byte, "byte"),
                    trieprefix!("c", [trieleaf!(Case, "ase"), trieleaf!(Copyof, "opyof")]),
                    trieprefix!(
                        "def",
                        [trieleaf!(Default, "ault"), trieleaf!(Define, "ine")]
                    ),
                    trieleaf!(Final, "final"),
                    trieprefix!(
                        "i",
                        [
                            trieleaf!(Implements, "mplements"),
                            trieprefix!("n", [trieleaf!(Inline, "line"), trieleaf!(In)])
                        ]
                    ),
                    trieleaf!(Let, "let"),
                    trieprefix!(
                        "m",
                        [
                            trieprefix!("a", [trieleaf!(Macro, "cro"), trieleaf!(Match, "tch")]),
                            trieleaf!(Mutable, "utable")
                        ]
                    ),
                    trieleaf!(Null, "null"),
                    trieleaf!(Of, "of"),
                    trieprefix!(
                        "p",
                        [trieleaf!(Partial, "artial"), trieleaf!(Promise, "romise")]
                    ),
                    trieprefix!(
                        "re",
                        [
                            trieleaf!(Reference, "ference"),
                            trieleaf!(Relocatable, "locatable")
                        ]
                    ),
                    trieprefix!(
                        "s",
                        [
                            trieleaf!(Sealed, "ealed"),
                            trieleaf!(Sizeof, "izeof"),
                            trieleaf!(Static, "tatic"),
                            trieleaf!(Supports, "upports"),
                            trieleaf!(Switch, "witch")
                        ]
                    ),
                    trieprefix!("type", [trieleaf!(Typedef, "def"), trieleaf!(Typeof, "of")]),
                    trieleaf!(Var, "var")
                )
                .boxed(),
            );
        }

        // ReturnStatement = 'return' [ Expression ] ';' ;
        {
            return_statement_parser.define(
                seq!(
                    ReturnStatement,
                    terminal!(Return, "return"),
                    optional!(rule!(expression_parser)),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
        {
            revert_statement_parser.define(
                seq!(
                    RevertStatement,
                    terminal!(Revert, "revert"),
                    optional!(rule!(identifier_path_parser)),
                    rule!(argument_list_parser),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
        {
            selected_import_parser.define(
                seq!(
                    SelectedImport,
                    token!(identifier_parser),
                    optional!(seq!(terminal!(As, "as"), token!(identifier_parser)))
                )
                .boxed(),
            );
        }

        // SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
        {
            selecting_import_directive_parser.define(
                seq!(
                    SelectingImportDirective,
                    delimited_by!(
                        DelimitedSeparatedSelectedImports,
                        terminal!(OpenBrace, "{"),
                        separated_by!(
                            SeparatedSelectedImports,
                            rule!(selected_import_parser),
                            terminal!(Comma, ",")
                        ),
                        terminal!(CloseBrace, "}")
                    ),
                    terminal!(From, "from"),
                    rule!(import_path_parser)
                )
                .boxed(),
            );
        }

        // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
        {
            shift_expression_parser.define(
                left_associative_binary_expression!(
                    ShiftExpression,
                    add_sub_expression_parser,
                    choice!(
                        terminal!(LessLess, "<<"),
                        terminal!(GreaterGreater, ">>"),
                        terminal!(GreaterGreaterGreater, ">>>")
                    )
                )
                .boxed(),
            );
        }

        // «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
        {
            signed_fixed_type_parser.define(
                lex_seq!(
                    SignedFixedType,
                    lex_terminal!(Fixed, "fixed"),
                    lex_optional!(lex_seq!(
                        lex_one_or_more!(lex_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
                        lex_terminal!(LatinSmallLetterX, 'x'),
                        lex_one_or_more!(lex_terminal!(|&c: &char| ('0' <= c && c <= '9')))
                    ))
                )
                .boxed(),
            );
        }

        // «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
        {
            signed_integer_type_parser.define(
                lex_seq!(
                    SignedIntegerType,
                    lex_terminal!(Int, "int"),
                    lex_optional!(lex_trie!(
                        trieprefix!(
                            "1",
                            [
                                trieleaf!(OneZeroFour, "04"),
                                trieleaf!(OneOneTwo, "12"),
                                trieprefix!(
                                    "2",
                                    [trieleaf!(OneTwoZero, "0"), trieleaf!(OneTwoEight, "8")]
                                ),
                                trieleaf!(OneThreeSix, "36"),
                                trieleaf!(OneFourFour, "44"),
                                trieleaf!(OneFiveTwo, "52"),
                                trieprefix!(
                                    "6",
                                    [
                                        trieleaf!(OneSixZero, "0"),
                                        trieleaf!(OneSixEight, "8"),
                                        trieleaf!(OneSix)
                                    ]
                                ),
                                trieleaf!(OneSevenSix, "76"),
                                trieleaf!(OneEightFour, "84"),
                                trieleaf!(OneNineTwo, "92")
                            ]
                        ),
                        trieprefix!(
                            "2",
                            [
                                trieprefix!(
                                    "0",
                                    [trieleaf!(TwoZeroZero, "0"), trieleaf!(TwoZeroEight, "8")]
                                ),
                                trieleaf!(TwoOneSix, "16"),
                                trieleaf!(TwoTwoFour, "24"),
                                trieleaf!(TwoThreeTwo, "32"),
                                trieprefix!(
                                    "4",
                                    [
                                        trieleaf!(TwoFourZero, "0"),
                                        trieleaf!(TwoFourEight, "8"),
                                        trieleaf!(TwoFour)
                                    ]
                                ),
                                trieleaf!(TwoFiveSix, "56")
                            ]
                        ),
                        trieleaf!(ThreeTwo, "32"),
                        trieprefix!("4", [trieleaf!(FourZero, "0"), trieleaf!(FourEight, "8")]),
                        trieleaf!(FiveSix, "56"),
                        trieleaf!(SixFour, "64"),
                        trieleaf!(SevenTwo, "72"),
                        trieprefix!(
                            "8",
                            [
                                trieleaf!(EightZero, "0"),
                                trieleaf!(EightEight, "8"),
                                trieleaf!(Eight)
                            ]
                        ),
                        trieleaf!(NineSix, "96")
                    ))
                )
                .boxed(),
            );
        }

        // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
        {
            simple_import_directive_parser.define(
                seq!(
                    SimpleImportDirective,
                    rule!(import_path_parser),
                    zero_or_more!(seq!(terminal!(As, "as"), token!(identifier_parser)))
                )
                .boxed(),
            );
        }

        // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
        {
            simple_statement_parser.define(
                choice!(
                    SimpleStatement,
                    rule!(tuple_deconstruction_statement_parser),
                    rule!(variable_declaration_statement_parser),
                    rule!(expression_statement_parser)
                )
                .boxed(),
            );
        }

        // «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
        {
            single_line_comment_parser.define(
                lex_seq!(
                    SingleLineComment,
                    lex_terminal!(SlashSlash, "//"),
                    lex_zero_or_more!(lex_terminal!(|&c: &char| c != '\r' && c != '\n'))
                )
                .boxed(),
            );
        }

        // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
        {
            single_quoted_ascii_string_literal_parser.define(
                lex_seq!(
                    SingleQuotedAsciiStringLiteral,
                    lex_terminal!(Quote, "'"),
                    lex_zero_or_more!(
                        Runs,
                        lex_choice!(
                            Run,
                            lex_one_or_more!(
                                Chars,
                                lex_terminal!(Char, |&c: &char| (' ' <= c && c <= '~')
                                    && c != '\''
                                    && c != '\\')
                            ),
                            lex_rule!(escape_sequence_parser)
                        )
                    ),
                    lex_terminal!(Quote, "'")
                )
                .boxed(),
            );
        }

        // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
        {
            single_quoted_unicode_string_literal_parser.define(
                lex_seq!(
                    SingleQuotedUnicodeStringLiteral,
                    lex_terminal!(UnicodeQuote, "unicode'"),
                    lex_zero_or_more!(
                        Runs,
                        lex_choice!(
                            Run,
                            lex_one_or_more!(
                                Chars,
                                lex_terminal!(Char, |&c: &char| c != '\''
                                    && c != '\\'
                                    && c != '\n'
                                    && c != '\r')
                            ),
                            lex_rule!(escape_sequence_parser)
                        )
                    ),
                    lex_terminal!(Quote, "'")
                )
                .boxed(),
            );
        }

        // SourceUnit = LeadingTrivia { Directive | Definition } EndOfFileTrivia ;
        {
            source_unit_parser.define(
                seq!(
                    SourceUnit,
                    rule!(leading_trivia_parser),
                    zero_or_more!(choice!(rule!(directive_parser), rule!(definition_parser))),
                    rule!(end_of_file_trivia_parser)
                )
                .boxed(),
            );
        }

        // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
        {
            star_import_directive_parser.define(
                seq!(
                    StarImportDirective,
                    terminal!(Star, "*"),
                    terminal!(As, "as"),
                    token!(identifier_parser),
                    terminal!(From, "from"),
                    rule!(import_path_parser)
                )
                .boxed(),
            );
        }

        // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
        {
            state_variable_attribute_parser.define(
                choice!(
                    StateVariableAttribute,
                    rule!(override_specifier_parser),
                    terminal!(Constant, "constant"),
                    terminal!(Immutable, "immutable"),
                    terminal!(Internal, "internal"),
                    terminal!(Private, "private"),
                    terminal!(Public, "public")
                )
                .boxed(),
            );
        }

        // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
        {
            state_variable_declaration_parser.define(
                seq!(
                    StateVariableDeclaration,
                    rule!(type_name_parser),
                    zero_or_more!(
                        StateVariableAttributes,
                        rule!(state_variable_attribute_parser)
                    ),
                    token!(identifier_parser),
                    optional!(seq!(terminal!(Equal, "="), rule!(expression_parser))),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
        {
            statement_parser.define(
                choice!(
                    Statement,
                    rule!(block_parser),
                    rule!(simple_statement_parser),
                    rule!(if_statement_parser),
                    rule!(for_statement_parser),
                    rule!(while_statement_parser),
                    rule!(do_while_statement_parser),
                    rule!(continue_statement_parser),
                    rule!(break_statement_parser),
                    rule!(try_statement_parser),
                    rule!(return_statement_parser),
                    rule!(emit_statement_parser),
                    rule!(revert_statement_parser),
                    rule!(delete_statement_parser),
                    rule!(assembly_statement_parser)
                )
                .boxed(),
            );
        }

        // StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
        {
            struct_definition_parser.define(
                seq!(
                    StructDefinition,
                    terminal!(Struct, "struct"),
                    token!(identifier_parser),
                    delimited_by!(
                        DelimitedStructMembers,
                        terminal!(OpenBrace, "{"),
                        one_or_more!(StructMembers, rule!(struct_member_parser)),
                        terminal!(CloseBrace, "}")
                    )
                )
                .boxed(),
            );
        }

        // StructMember = TypeName «Identifier» ';' ;
        {
            struct_member_parser.define(
                seq!(
                    StructMember,
                    rule!(type_name_parser),
                    token!(identifier_parser),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // TrailingTrivia = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
        {
            trailing_trivia_parser.define(
                optional!(seq!(
                    TrailingTrivia,
                    zero_or_more!(choice!(
                        trivia_token!(whitespace_parser),
                        trivia_token!(multiline_comment_parser)
                    )),
                    choice!(
                        trivia_token!(end_of_line_parser),
                        trivia_token!(single_line_comment_parser)
                    )
                ))
                .boxed(),
            );
        }

        // TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
        {
            try_statement_parser.define(
                seq!(
                    TryStatement,
                    terminal!(Try, "try"),
                    rule!(expression_parser),
                    optional!(seq!(
                        terminal!(Returns, "returns"),
                        rule!(parameter_list_parser)
                    )),
                    rule!(block_parser),
                    one_or_more!(CatchClauses, rule!(catch_clause_parser))
                )
                .boxed(),
            );
        }

        // TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
        {
            tuple_deconstruction_statement_parser.define(
                seq!(
                    TupleDeconstructionStatement,
                    delimited_by!(
                        terminal!(OpenParen, "("),
                        optional!(separated_by!(
                            optional!(seq!(
                                optional!(rule!(type_name_parser)),
                                token!(identifier_parser)
                            )),
                            terminal!(Comma, ",")
                        )),
                        terminal!(CloseParen, ")")
                    ),
                    terminal!(Equal, "="),
                    rule!(expression_parser),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // TypeExpression = 'type' '(' TypeName ')' ;
        {
            type_expression_parser.define(
                seq!(
                    TypeExpression,
                    terminal!(Type, "type"),
                    delimited_by!(
                        DelimitedTypeName,
                        terminal!(OpenParen, "("),
                        rule!(type_name_parser),
                        terminal!(CloseParen, ")")
                    )
                )
                .boxed(),
            );
        }

        // TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
        {
            type_name_parser.define(
                seq!(
                    TypeName,
                    choice!(
                        rule!(elementary_type_parser),
                        rule!(function_type_parser),
                        rule!(mapping_type_parser),
                        rule!(identifier_path_parser)
                    ),
                    zero_or_more!(
                        DelimitedExpressions,
                        delimited_by!(
                            DelimitedExpression,
                            terminal!(OpenBracket, "["),
                            optional!(rule!(expression_parser)),
                            terminal!(CloseBracket, "]")
                        )
                    )
                )
                .boxed(),
            );
        }

        // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
        {
            unary_prefix_expression_parser.define(
                unary_prefix_expression!(
                    UnaryPrefixExpression,
                    function_call_expression_parser,
                    choice!(
                        terminal!(PlusPlus, "++"),
                        terminal!(MinusMinus, "--"),
                        terminal!(Bang, "!"),
                        terminal!(Tilde, "~"),
                        terminal!(Minus, "-")
                    )
                )
                .boxed(),
            );
        }

        // UnarySuffixExpression = Expression ( '++' | '--' ) ;
        {
            unary_suffix_expression_parser.define(
                unary_suffix_expression!(
                    UnarySuffixExpression,
                    unary_prefix_expression_parser,
                    choice!(terminal!(PlusPlus, "++"), terminal!(MinusMinus, "--"))
                )
                .boxed(),
            );
        }

        // UncheckedBlock = 'unchecked' Block ;
        {
            unchecked_block_parser.define(
                seq!(
                    UncheckedBlock,
                    terminal!(Unchecked, "unchecked"),
                    rule!(block_parser)
                )
                .boxed(),
            );
        }

        // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
        {
            unicode_escape_parser.define(
                lex_seq!(
                    UnicodeEscape,
                    lex_terminal!(LatinSmallLetterU, 'u'),
                    lex_repeated!(
                        lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                            || ('a' <= c && c <= 'f')
                            || ('A' <= c && c <= 'F')),
                        4usize,
                        4usize
                    )
                )
                .boxed(),
            );
        }

        // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
        {
            unicode_string_literal_parser.define(
                lex_choice!(
                    UnicodeStringLiteral,
                    lex_rule!(single_quoted_unicode_string_literal_parser),
                    lex_rule!(double_quoted_unicode_string_literal_parser)
                )
                .boxed(),
            );
        }

        // «UnsignedFixedType» = 'u' «SignedFixedType» ;
        {
            unsigned_fixed_type_parser.define(
                lex_seq!(
                    UnsignedFixedType,
                    lex_terminal!(LatinSmallLetterU, 'u'),
                    lex_rule!(signed_fixed_type_parser)
                )
                .boxed(),
            );
        }

        // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
        {
            unsigned_integer_type_parser.define(
                lex_seq!(
                    UnsignedIntegerType,
                    lex_terminal!(LatinSmallLetterU, 'u'),
                    lex_rule!(signed_integer_type_parser)
                )
                .boxed(),
            );
        }

        // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
        {
            user_defined_value_type_definition_parser.define(
                seq!(
                    UserDefinedValueTypeDefinition,
                    terminal!(Type, "type"),
                    token!(identifier_parser),
                    terminal!(Is, "is"),
                    rule!(elementary_type_parser),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
        {
            using_directive_parser.define(
                seq!(
                    UsingDirective,
                    terminal!(Using, "using"),
                    choice!(
                        rule!(identifier_path_parser),
                        delimited_by!(
                            DelimitedSeparatedIdentifierPaths,
                            terminal!(OpenBrace, "{"),
                            separated_by!(
                                SeparatedIdentifierPaths,
                                rule!(identifier_path_parser),
                                terminal!(Comma, ",")
                            ),
                            terminal!(CloseBrace, "}")
                        )
                    ),
                    terminal!(For, "for"),
                    choice!(terminal!(Star, "*"), rule!(type_name_parser)),
                    optional!(terminal!(Global, "global")),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
        {
            variable_declaration_statement_parser.define(
                seq!(
                    VariableDeclarationStatement,
                    rule!(type_name_parser),
                    optional!(rule!(data_location_parser)),
                    token!(identifier_parser),
                    optional!(seq!(terminal!(Equal, "="), rule!(expression_parser))),
                    terminal!(Semicolon, ";")
                )
                .boxed(),
            );
        }

        // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
        {
            version_pragma_operator_parser.define(
                lex_trie!(
                    trieprefix!("<", [trieleaf!(LessEqual, "="), trieleaf!(Less)]),
                    trieleaf!(Equal, "="),
                    trieprefix!(">", [trieleaf!(GreaterEqual, "="), trieleaf!(Greater)]),
                    trieleaf!(Caret, "^"),
                    trieleaf!(Tilde, "~")
                )
                .boxed(),
            );
        }

        // VersionPragmaSpecifier = 'solidity' 1…*{ «VersionPragmaOperator» «VersionPragmaValue» } ;
        {
            version_pragma_specifier_parser.define(
                seq!(
                    VersionPragmaSpecifier,
                    terminal!(Solidity, "solidity"),
                    one_or_more!(seq!(
                        token!(version_pragma_operator_parser),
                        token!(version_pragma_value_parser)
                    ))
                )
                .boxed(),
            );
        }

        // «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' }  { '.' 1…*{ '0'…'9' | 'x' | 'X' | '*' } } ;
        {
            version_pragma_value_parser.define(
                lex_separated_by!(
                    VersionPragmaValue,
                    lex_one_or_more!(lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                        || c == 'x'
                        || c == 'X'
                        || c == '*')),
                    lex_terminal!(Period, ".")
                )
                .boxed(),
            );
        }

        // WhileStatement = 'while' '(' Expression ')' Statement ;
        {
            while_statement_parser.define(
                seq!(
                    WhileStatement,
                    terminal!(While, "while"),
                    delimited_by!(
                        DelimitedExpression,
                        terminal!(OpenParen, "("),
                        rule!(expression_parser),
                        terminal!(CloseParen, ")")
                    ),
                    rule!(statement_parser)
                )
                .boxed(),
            );
        }

        // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
        {
            whitespace_parser.define(
                lex_one_or_more!(Whitespace, lex_terminal!(|&c: &char| c == ' ' || c == '\t'))
                    .boxed(),
            );
        }

        // YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
        {
            yul_assignment_statement_parser.define(
                seq!(
                    YulAssignmentStatement,
                    separated_by!(
                        SeparatedYulIdentifierPaths,
                        rule!(yul_identifier_path_parser),
                        terminal!(Comma, ",")
                    ),
                    terminal!(ColonEqual, ":="),
                    rule!(yul_expression_parser)
                )
                .boxed(),
            );
        }

        // YulBlock = '{' { YulStatement } '}' ;
        {
            yul_block_parser.define(
                delimited_by!(
                    YulBlock,
                    terminal!(OpenBrace, "{"),
                    zero_or_more!(YulStatements, rule!(yul_statement_parser)),
                    terminal!(CloseBrace, "}")
                )
                .boxed(),
            );
        }

        // YulBreakStatement = 'break' ;
        {
            yul_break_statement_parser.define(terminal!(Break, "break").boxed());
        }

        // YulContinueStatement = 'continue' ;
        {
            yul_continue_statement_parser.define(terminal!(Continue, "continue").boxed());
        }

        // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
        {
            yul_decimal_number_literal_parser.define(lex_terminal!(Zero, "0").boxed());
        }

        // YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
        {
            yul_expression_parser.define(
                choice!(
                    YulExpression,
                    rule!(yul_identifier_path_parser),
                    rule!(yul_function_call_parser),
                    rule!(yul_literal_parser)
                )
                .boxed(),
            );
        }

        // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
        {
            yul_for_statement_parser.define(
                seq!(
                    YulForStatement,
                    terminal!(For, "for"),
                    rule!(yul_block_parser),
                    rule!(yul_expression_parser),
                    rule!(yul_block_parser),
                    rule!(yul_block_parser)
                )
                .boxed(),
            );
        }

        // YulFunctionCall = «YulIdentifier» '(' [ YulExpression  { ',' YulExpression } ] ')' ;
        {
            yul_function_call_parser.define(
                seq!(
                    YulFunctionCall,
                    token!(yul_identifier_parser),
                    delimited_by!(
                        DelimitedSeparatedYulExpressions,
                        terminal!(OpenParen, "("),
                        optional!(separated_by!(
                            SeparatedYulExpressions,
                            rule!(yul_expression_parser),
                            terminal!(Comma, ",")
                        )),
                        terminal!(CloseParen, ")")
                    )
                )
                .boxed(),
            );
        }

        // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
        {
            yul_function_definition_parser.define(
                seq!(
                    YulFunctionDefinition,
                    terminal!(Function, "function"),
                    token!(yul_identifier_parser),
                    delimited_by!(
                        DelimitedArguments,
                        terminal!(OpenParen, "("),
                        optional!(separated_by!(
                            Arguments,
                            token!(yul_identifier_parser),
                            terminal!(Comma, ",")
                        )),
                        terminal!(CloseParen, ")")
                    ),
                    optional!(seq!(
                        terminal!(MinusGreater, "->"),
                        separated_by!(
                            Results,
                            token!(yul_identifier_parser),
                            terminal!(Comma, ",")
                        )
                    )),
                    rule!(yul_block_parser)
                )
                .boxed(),
            );
        }

        // «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
        {
            yul_hex_literal_parser.define(
                lex_seq!(
                    YulHexLiteral,
                    lex_terminal!(ZeroX, "0x"),
                    lex_one_or_more!(lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                        || ('a' <= c && c <= 'f')
                        || ('A' <= c && c <= 'F')))
                )
                .boxed(),
            );
        }

        // «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
        {
            yul_identifier_parser.define(
                difference(
                    lex_rule!(raw_identifier_parser),
                    lex_trie!(
                        trieleaf!(Break, "break"),
                        trieprefix!(
                            "c",
                            [trieleaf!(Case, "ase"), trieleaf!(Continue, "ontinue")]
                        ),
                        trieleaf!(Default, "default"),
                        trieprefix!(
                            "f",
                            [
                                trieleaf!(False, "alse"),
                                trieleaf!(For, "or"),
                                trieleaf!(Function, "unction")
                            ]
                        ),
                        trieleaf!(Hex, "hex"),
                        trieleaf!(If, "if"),
                        trieprefix!("le", [trieleaf!(Leave, "ave"), trieleaf!(Let, "t")]),
                        trieleaf!(Switch, "switch"),
                        trieleaf!(True, "true")
                    ),
                )
                .boxed(),
            );
        }

        // YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
        {
            yul_identifier_path_parser.define(
                separated_by!(
                    YulIdentifierPath,
                    token!(yul_identifier_parser),
                    terminal!(Period, ".")
                )
                .boxed(),
            );
        }

        // YulIfStatement = 'if' YulExpression YulBlock ;
        {
            yul_if_statement_parser.define(
                seq!(
                    YulIfStatement,
                    terminal!(If, "if"),
                    rule!(yul_expression_parser),
                    rule!(yul_block_parser)
                )
                .boxed(),
            );
        }

        // «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
        {
            yul_keyword_parser.define(
                lex_trie!(
                    trieleaf!(Break, "break"),
                    trieprefix!(
                        "c",
                        [trieleaf!(Case, "ase"), trieleaf!(Continue, "ontinue")]
                    ),
                    trieleaf!(Default, "default"),
                    trieprefix!(
                        "f",
                        [
                            trieleaf!(False, "alse"),
                            trieleaf!(For, "or"),
                            trieleaf!(Function, "unction")
                        ]
                    ),
                    trieleaf!(Hex, "hex"),
                    trieleaf!(If, "if"),
                    trieprefix!("le", [trieleaf!(Leave, "ave"), trieleaf!(Let, "t")]),
                    trieleaf!(Switch, "switch"),
                    trieleaf!(True, "true")
                )
                .boxed(),
            );
        }

        // YulLeaveStatement = 'leave' ;
        {
            yul_leave_statement_parser.define(terminal!(Leave, "leave").boxed());
        }

        // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
        {
            yul_literal_parser.define(
                choice!(
                    YulLiteral,
                    token!(yul_decimal_number_literal_parser),
                    token!(yul_hex_literal_parser),
                    token!(ascii_string_literal_parser),
                    token!(boolean_literal_parser),
                    token!(hex_string_literal_parser)
                )
                .boxed(),
            );
        }

        // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
        {
            yul_statement_parser.define(
                choice!(
                    YulStatement,
                    rule!(yul_block_parser),
                    rule!(yul_variable_declaration_parser),
                    rule!(yul_function_definition_parser),
                    rule!(yul_assignment_statement_parser),
                    rule!(yul_function_call_parser),
                    rule!(yul_if_statement_parser),
                    rule!(yul_for_statement_parser),
                    rule!(yul_switch_statement_parser),
                    rule!(yul_leave_statement_parser),
                    rule!(yul_break_statement_parser),
                    rule!(yul_continue_statement_parser)
                )
                .boxed(),
            );
        }

        // YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
        {
            yul_switch_statement_parser.define(
                seq!(
                    YulSwitchStatement,
                    terminal!(Switch, "switch"),
                    rule!(yul_expression_parser),
                    one_or_more!(seq!(
                        choice!(
                            seq!(terminal!(Case, "case"), rule!(yul_literal_parser)),
                            terminal!(Default, "default")
                        ),
                        rule!(yul_block_parser)
                    ))
                )
                .boxed(),
            );
        }

        // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
        {
            yul_variable_declaration_parser.define(
                seq!(
                    YulVariableDeclaration,
                    terminal!(Let, "let"),
                    separated_by!(
                        SeparatedYulIdentifierPaths,
                        rule!(yul_identifier_path_parser),
                        terminal!(Comma, ",")
                    ),
                    optional!(seq!(
                        terminal!(ColonEqual, ":="),
                        rule!(yul_expression_parser)
                    ))
                )
                .boxed(),
            );
        }

        // Create the Parser object -------------------------

        Self {
            abi_coder_pragma_specifier: abi_coder_pragma_specifier_parser
                .then_ignore(end())
                .boxed(),
            add_sub_expression: add_sub_expression_parser.then_ignore(end()).boxed(),
            address_type: address_type_parser.then_ignore(end()).boxed(),
            and_expression: and_expression_parser.then_ignore(end()).boxed(),
            argument_list: argument_list_parser.then_ignore(end()).boxed(),
            array_literal: array_literal_parser.then_ignore(end()).boxed(),
            ascii_escape: ascii_escape_parser.then_ignore(end()).boxed(),
            ascii_string_literal: ascii_string_literal_parser.then_ignore(end()).boxed(),
            assembly_flags: assembly_flags_parser.then_ignore(end()).boxed(),
            assembly_statement: assembly_statement_parser.then_ignore(end()).boxed(),
            assignment_expression: assignment_expression_parser.then_ignore(end()).boxed(),
            bit_and_expression: bit_and_expression_parser.then_ignore(end()).boxed(),
            bit_or_expression: bit_or_expression_parser.then_ignore(end()).boxed(),
            bit_x_or_expression: bit_x_or_expression_parser.then_ignore(end()).boxed(),
            block: block_parser.then_ignore(end()).boxed(),
            boolean_literal: boolean_literal_parser.then_ignore(end()).boxed(),
            break_statement: break_statement_parser.then_ignore(end()).boxed(),
            catch_clause: catch_clause_parser.then_ignore(end()).boxed(),
            conditional_expression: conditional_expression_parser.then_ignore(end()).boxed(),
            constant_definition: constant_definition_parser.then_ignore(end()).boxed(),
            constructor_attribute: constructor_attribute_parser.then_ignore(end()).boxed(),
            constructor_definition: constructor_definition_parser.then_ignore(end()).boxed(),
            continue_statement: continue_statement_parser.then_ignore(end()).boxed(),
            contract_body_element: contract_body_element_parser.then_ignore(end()).boxed(),
            contract_definition: contract_definition_parser.then_ignore(end()).boxed(),
            data_location: data_location_parser.then_ignore(end()).boxed(),
            decimal_exponent: decimal_exponent_parser.then_ignore(end()).boxed(),
            decimal_float: decimal_float_parser.then_ignore(end()).boxed(),
            decimal_integer: decimal_integer_parser.then_ignore(end()).boxed(),
            decimal_number: decimal_number_parser.then_ignore(end()).boxed(),
            definition: definition_parser.then_ignore(end()).boxed(),
            delete_statement: delete_statement_parser.then_ignore(end()).boxed(),
            directive: directive_parser.then_ignore(end()).boxed(),
            do_while_statement: do_while_statement_parser.then_ignore(end()).boxed(),
            double_quoted_ascii_string_literal: double_quoted_ascii_string_literal_parser
                .then_ignore(end())
                .boxed(),
            double_quoted_unicode_string_literal: double_quoted_unicode_string_literal_parser
                .then_ignore(end())
                .boxed(),
            elementary_type: elementary_type_parser.then_ignore(end()).boxed(),
            emit_statement: emit_statement_parser.then_ignore(end()).boxed(),
            end_of_file_trivia: end_of_file_trivia_parser.then_ignore(end()).boxed(),
            end_of_line: end_of_line_parser.then_ignore(end()).boxed(),
            enum_definition: enum_definition_parser.then_ignore(end()).boxed(),
            equality_comparison_expression: equality_comparison_expression_parser
                .then_ignore(end())
                .boxed(),
            error_definition: error_definition_parser.then_ignore(end()).boxed(),
            error_parameter: error_parameter_parser.then_ignore(end()).boxed(),
            escape_sequence: escape_sequence_parser.then_ignore(end()).boxed(),
            event_definition: event_definition_parser.then_ignore(end()).boxed(),
            event_parameter: event_parameter_parser.then_ignore(end()).boxed(),
            experimental_pragma_specifier: experimental_pragma_specifier_parser
                .then_ignore(end())
                .boxed(),
            exponentiation_expression: exponentiation_expression_parser.then_ignore(end()).boxed(),
            expression: expression_parser.then_ignore(end()).boxed(),
            expression_statement: expression_statement_parser.then_ignore(end()).boxed(),
            fallback_function_attribute: fallback_function_attribute_parser
                .then_ignore(end())
                .boxed(),
            fallback_function_definition: fallback_function_definition_parser
                .then_ignore(end())
                .boxed(),
            fixed_bytes_type: fixed_bytes_type_parser.then_ignore(end()).boxed(),
            for_statement: for_statement_parser.then_ignore(end()).boxed(),
            function_attribute: function_attribute_parser.then_ignore(end()).boxed(),
            function_call_expression: function_call_expression_parser.then_ignore(end()).boxed(),
            function_definition: function_definition_parser.then_ignore(end()).boxed(),
            function_type: function_type_parser.then_ignore(end()).boxed(),
            hex_byte_escape: hex_byte_escape_parser.then_ignore(end()).boxed(),
            hex_character: hex_character_parser.then_ignore(end()).boxed(),
            hex_number: hex_number_parser.then_ignore(end()).boxed(),
            hex_string_literal: hex_string_literal_parser.then_ignore(end()).boxed(),
            identifier: identifier_parser.then_ignore(end()).boxed(),
            identifier_part: identifier_part_parser.then_ignore(end()).boxed(),
            identifier_path: identifier_path_parser.then_ignore(end()).boxed(),
            identifier_start: identifier_start_parser.then_ignore(end()).boxed(),
            if_statement: if_statement_parser.then_ignore(end()).boxed(),
            import_directive: import_directive_parser.then_ignore(end()).boxed(),
            import_path: import_path_parser.then_ignore(end()).boxed(),
            index_access_expression: index_access_expression_parser.then_ignore(end()).boxed(),
            inheritance_specifier: inheritance_specifier_parser.then_ignore(end()).boxed(),
            inheritance_specifier_list: inheritance_specifier_list_parser
                .then_ignore(end())
                .boxed(),
            interface_definition: interface_definition_parser.then_ignore(end()).boxed(),
            keyword: keyword_parser.then_ignore(end()).boxed(),
            leading_trivia: leading_trivia_parser.then_ignore(end()).boxed(),
            library_definition: library_definition_parser.then_ignore(end()).boxed(),
            mapping_type: mapping_type_parser.then_ignore(end()).boxed(),
            member_access_expression: member_access_expression_parser.then_ignore(end()).boxed(),
            modifier_attribute: modifier_attribute_parser.then_ignore(end()).boxed(),
            modifier_definition: modifier_definition_parser.then_ignore(end()).boxed(),
            modifier_invocation: modifier_invocation_parser.then_ignore(end()).boxed(),
            mul_div_mod_expression: mul_div_mod_expression_parser.then_ignore(end()).boxed(),
            multiline_comment: multiline_comment_parser.then_ignore(end()).boxed(),
            named_argument: named_argument_parser.then_ignore(end()).boxed(),
            named_argument_list: named_argument_list_parser.then_ignore(end()).boxed(),
            new_expression: new_expression_parser.then_ignore(end()).boxed(),
            number_unit: number_unit_parser.then_ignore(end()).boxed(),
            numeric_literal: numeric_literal_parser.then_ignore(end()).boxed(),
            or_expression: or_expression_parser.then_ignore(end()).boxed(),
            order_comparison_expression: order_comparison_expression_parser
                .then_ignore(end())
                .boxed(),
            override_specifier: override_specifier_parser.then_ignore(end()).boxed(),
            parameter_declaration: parameter_declaration_parser.then_ignore(end()).boxed(),
            parameter_list: parameter_list_parser.then_ignore(end()).boxed(),
            parenthesis_expression: parenthesis_expression_parser.then_ignore(end()).boxed(),
            payable_expression: payable_expression_parser.then_ignore(end()).boxed(),
            positional_argument_list: positional_argument_list_parser.then_ignore(end()).boxed(),
            possibly_separated_pairs_of_hex_digits: possibly_separated_pairs_of_hex_digits_parser
                .then_ignore(end())
                .boxed(),
            pragma_directive: pragma_directive_parser.then_ignore(end()).boxed(),
            primary_expression: primary_expression_parser.then_ignore(end()).boxed(),
            raw_identifier: raw_identifier_parser.then_ignore(end()).boxed(),
            receive_function_attribute: receive_function_attribute_parser
                .then_ignore(end())
                .boxed(),
            receive_function_definition: receive_function_definition_parser
                .then_ignore(end())
                .boxed(),
            reserved_keyword: reserved_keyword_parser.then_ignore(end()).boxed(),
            return_statement: return_statement_parser.then_ignore(end()).boxed(),
            revert_statement: revert_statement_parser.then_ignore(end()).boxed(),
            selected_import: selected_import_parser.then_ignore(end()).boxed(),
            selecting_import_directive: selecting_import_directive_parser
                .then_ignore(end())
                .boxed(),
            shift_expression: shift_expression_parser.then_ignore(end()).boxed(),
            signed_fixed_type: signed_fixed_type_parser.then_ignore(end()).boxed(),
            signed_integer_type: signed_integer_type_parser.then_ignore(end()).boxed(),
            simple_import_directive: simple_import_directive_parser.then_ignore(end()).boxed(),
            simple_statement: simple_statement_parser.then_ignore(end()).boxed(),
            single_line_comment: single_line_comment_parser.then_ignore(end()).boxed(),
            single_quoted_ascii_string_literal: single_quoted_ascii_string_literal_parser
                .then_ignore(end())
                .boxed(),
            single_quoted_unicode_string_literal: single_quoted_unicode_string_literal_parser
                .then_ignore(end())
                .boxed(),
            source_unit: source_unit_parser.then_ignore(end()).boxed(),
            star_import_directive: star_import_directive_parser.then_ignore(end()).boxed(),
            state_variable_attribute: state_variable_attribute_parser.then_ignore(end()).boxed(),
            state_variable_declaration: state_variable_declaration_parser
                .then_ignore(end())
                .boxed(),
            statement: statement_parser.then_ignore(end()).boxed(),
            struct_definition: struct_definition_parser.then_ignore(end()).boxed(),
            struct_member: struct_member_parser.then_ignore(end()).boxed(),
            trailing_trivia: trailing_trivia_parser.then_ignore(end()).boxed(),
            try_statement: try_statement_parser.then_ignore(end()).boxed(),
            tuple_deconstruction_statement: tuple_deconstruction_statement_parser
                .then_ignore(end())
                .boxed(),
            type_expression: type_expression_parser.then_ignore(end()).boxed(),
            type_name: type_name_parser.then_ignore(end()).boxed(),
            unary_prefix_expression: unary_prefix_expression_parser.then_ignore(end()).boxed(),
            unary_suffix_expression: unary_suffix_expression_parser.then_ignore(end()).boxed(),
            unchecked_block: unchecked_block_parser.then_ignore(end()).boxed(),
            unicode_escape: unicode_escape_parser.then_ignore(end()).boxed(),
            unicode_string_literal: unicode_string_literal_parser.then_ignore(end()).boxed(),
            unsigned_fixed_type: unsigned_fixed_type_parser.then_ignore(end()).boxed(),
            unsigned_integer_type: unsigned_integer_type_parser.then_ignore(end()).boxed(),
            user_defined_value_type_definition: user_defined_value_type_definition_parser
                .then_ignore(end())
                .boxed(),
            using_directive: using_directive_parser.then_ignore(end()).boxed(),
            variable_declaration_statement: variable_declaration_statement_parser
                .then_ignore(end())
                .boxed(),
            version_pragma_operator: version_pragma_operator_parser.then_ignore(end()).boxed(),
            version_pragma_specifier: version_pragma_specifier_parser.then_ignore(end()).boxed(),
            version_pragma_value: version_pragma_value_parser.then_ignore(end()).boxed(),
            while_statement: while_statement_parser.then_ignore(end()).boxed(),
            whitespace: whitespace_parser.then_ignore(end()).boxed(),
            yul_assignment_statement: yul_assignment_statement_parser.then_ignore(end()).boxed(),
            yul_block: yul_block_parser.then_ignore(end()).boxed(),
            yul_break_statement: yul_break_statement_parser.then_ignore(end()).boxed(),
            yul_continue_statement: yul_continue_statement_parser.then_ignore(end()).boxed(),
            yul_decimal_number_literal: yul_decimal_number_literal_parser
                .then_ignore(end())
                .boxed(),
            yul_expression: yul_expression_parser.then_ignore(end()).boxed(),
            yul_for_statement: yul_for_statement_parser.then_ignore(end()).boxed(),
            yul_function_call: yul_function_call_parser.then_ignore(end()).boxed(),
            yul_function_definition: yul_function_definition_parser.then_ignore(end()).boxed(),
            yul_hex_literal: yul_hex_literal_parser.then_ignore(end()).boxed(),
            yul_identifier: yul_identifier_parser.then_ignore(end()).boxed(),
            yul_identifier_path: yul_identifier_path_parser.then_ignore(end()).boxed(),
            yul_if_statement: yul_if_statement_parser.then_ignore(end()).boxed(),
            yul_keyword: yul_keyword_parser.then_ignore(end()).boxed(),
            yul_leave_statement: yul_leave_statement_parser.then_ignore(end()).boxed(),
            yul_literal: yul_literal_parser.then_ignore(end()).boxed(),
            yul_statement: yul_statement_parser.then_ignore(end()).boxed(),
            yul_switch_statement: yul_switch_statement_parser.then_ignore(end()).boxed(),
            yul_variable_declaration: yul_variable_declaration_parser.then_ignore(end()).boxed(),
        }
    }
}
