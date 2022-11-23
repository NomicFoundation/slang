// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{cst, lex, parse::Parsers};
use chumsky::Parser;
use semver::Version;
use std::rc::Rc;
pub struct Language {
    parsers: Parsers<'static>,
    version: Version,
}
impl Language {
    pub fn new(version: Version) -> Self {
        Self {
            parsers: Parsers::new(&version),
            version,
        }
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl Language {
    // ABICoderPragmaSpecifier = 'abicoder' «Identifier» ;
    pub fn parse_abi_coder_pragma_specifier(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .abi_coder_pragma_specifier
            .parse_recovery(source);
        node.unwrap()
    }

    // AddSubExpression = Expression ( '+' | '-' ) Expression ;
    pub fn parse_add_sub_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.add_sub_expression.parse_recovery(source);
        node.unwrap()
    }

    // AddressType = 'address' [ 'payable' ] ;
    pub fn parse_address_type(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.address_type.parse_recovery(source);
        node.unwrap()
    }

    // AndExpression = Expression '&&' Expression ;
    pub fn parse_and_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.and_expression.parse_recovery(source);
        node.unwrap()
    }

    // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
    pub fn parse_argument_list(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.argument_list.parse_recovery(source);
        node.unwrap()
    }

    // ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
    pub fn parse_array_literal(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.array_literal.parse_recovery(source);
        node.unwrap()
    }

    // «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
    pub fn parse_ascii_escape(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.ascii_escape.parse_recovery(source);
        node.unwrap()
    }

    // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
    pub fn parse_ascii_string_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.ascii_string_literal.parse_recovery(source);
        node.unwrap()
    }

    // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
    pub fn parse_assembly_flags(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.assembly_flags.parse_recovery(source);
        node.unwrap()
    }

    // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
    pub fn parse_assembly_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.assembly_statement.parse_recovery(source);
        node.unwrap()
    }

    // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
    pub fn parse_assignment_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.assignment_expression.parse_recovery(source);
        node.unwrap()
    }

    // BitAndExpression = Expression '&' Expression ;
    pub fn parse_bit_and_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.bit_and_expression.parse_recovery(source);
        node.unwrap()
    }

    // BitOrExpression = Expression '|' Expression ;
    pub fn parse_bit_or_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.bit_or_expression.parse_recovery(source);
        node.unwrap()
    }

    // BitXOrExpression = Expression '^' Expression ;
    pub fn parse_bit_x_or_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.bit_x_or_expression.parse_recovery(source);
        node.unwrap()
    }

    // Block = '{' { Statement | UncheckedBlock } '}' ;
    pub fn parse_block(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.block.parse_recovery(source);
        node.unwrap()
    }

    // «BooleanLiteral» = 'true' | 'false' ;
    pub fn parse_boolean_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.boolean_literal.parse_recovery(source);
        node.unwrap()
    }

    // BreakStatement = 'break' ';' ;
    pub fn parse_break_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.break_statement.parse_recovery(source);
        node.unwrap()
    }

    // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
    pub fn parse_catch_clause(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.catch_clause.parse_recovery(source);
        node.unwrap()
    }

    // ConditionalExpression = Expression '?' Expression ':' Expression ;
    pub fn parse_conditional_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.conditional_expression.parse_recovery(source);
        node.unwrap()
    }

    // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
    pub fn parse_constant_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.constant_definition.parse_recovery(source);
        node.unwrap()
    }

    // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
    pub fn parse_constructor_attribute(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.constructor_attribute.parse_recovery(source);
        node.unwrap()
    }

    // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
    pub fn parse_constructor_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.constructor_definition.parse_recovery(source);
        node.unwrap()
    }

    // ContinueStatement = 'continue' ';' ;
    pub fn parse_continue_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.continue_statement.parse_recovery(source);
        node.unwrap()
    }

    // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
    pub fn parse_contract_body_element(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.contract_body_element.parse_recovery(source);
        node.unwrap()
    }

    // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    pub fn parse_contract_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.contract_definition.parse_recovery(source);
        node.unwrap()
    }

    // DataLocation = 'memory' | 'storage' | 'calldata' ;
    pub fn parse_data_location(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.data_location.parse_recovery(source);
        node.unwrap()
    }

    // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
    pub fn parse_decimal_exponent(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.decimal_exponent.parse_recovery(source);
        node.unwrap()
    }

    // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
    pub fn parse_decimal_float(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.decimal_float.parse_recovery(source);
        node.unwrap()
    }

    // «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
    pub fn parse_decimal_integer(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.decimal_integer.parse_recovery(source);
        node.unwrap()
    }

    // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
    pub fn parse_decimal_number(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.decimal_number.parse_recovery(source);
        node.unwrap()
    }

    // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
    pub fn parse_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.definition.parse_recovery(source);
        node.unwrap()
    }

    // DeleteStatement = 'delete' «Identifier» ';' ;
    pub fn parse_delete_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.delete_statement.parse_recovery(source);
        node.unwrap()
    }

    // Directive = PragmaDirective | ImportDirective | UsingDirective ;
    pub fn parse_directive(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.directive.parse_recovery(source);
        node.unwrap()
    }

    // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
    pub fn parse_do_while_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.do_while_statement.parse_recovery(source);
        node.unwrap()
    }

    // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
    pub fn parse_double_quoted_ascii_string_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self
            .parsers
            .double_quoted_ascii_string_literal
            .parse_recovery(source);
        node.unwrap()
    }

    // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
    pub fn parse_double_quoted_unicode_string_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self
            .parsers
            .double_quoted_unicode_string_literal
            .parse_recovery(source);
        node.unwrap()
    }

    // ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
    pub fn parse_elementary_type(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.elementary_type.parse_recovery(source);
        node.unwrap()
    }

    // EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
    pub fn parse_emit_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.emit_statement.parse_recovery(source);
        node.unwrap()
    }

    // EndOfFileTrivia = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
    pub fn parse_end_of_file_trivia(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.end_of_file_trivia.parse_recovery(source);
        node.unwrap()
    }

    // «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
    pub fn parse_end_of_line(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.end_of_line.parse_recovery(source);
        node.unwrap()
    }

    // EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
    pub fn parse_enum_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.enum_definition.parse_recovery(source);
        node.unwrap()
    }

    // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
    pub fn parse_equality_comparison_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .equality_comparison_expression
            .parse_recovery(source);
        node.unwrap()
    }

    // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
    pub fn parse_error_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.error_definition.parse_recovery(source);
        node.unwrap()
    }

    // ErrorParameter = TypeName [ «Identifier» ] ;
    pub fn parse_error_parameter(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.error_parameter.parse_recovery(source);
        node.unwrap()
    }

    // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
    pub fn parse_escape_sequence(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.escape_sequence.parse_recovery(source);
        node.unwrap()
    }

    // EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
    pub fn parse_event_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.event_definition.parse_recovery(source);
        node.unwrap()
    }

    // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
    pub fn parse_event_parameter(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.event_parameter.parse_recovery(source);
        node.unwrap()
    }

    // ExperimentalPragmaSpecifier = 'experimental' «Identifier» ;
    pub fn parse_experimental_pragma_specifier(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .experimental_pragma_specifier
            .parse_recovery(source);
        node.unwrap()
    }

    // (* 0.0.0 *) ExponentiationExpression = Expression '**' Expression ;
    // (* 0.6.0 *) ExponentiationExpression = Expression '**' Expression ;
    pub fn parse_exponentiation_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .exponentiation_expression
            .parse_recovery(source);
        node.unwrap()
    }

    // Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    pub fn parse_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.expression.parse_recovery(source);
        node.unwrap()
    }

    // ExpressionStatement = Expression ';' ;
    pub fn parse_expression_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.expression_statement.parse_recovery(source);
        node.unwrap()
    }

    // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
    pub fn parse_fallback_function_attribute(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .fallback_function_attribute
            .parse_recovery(source);
        node.unwrap()
    }

    // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    pub fn parse_fallback_function_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .fallback_function_definition
            .parse_recovery(source);
        node.unwrap()
    }

    // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
    pub fn parse_fixed_bytes_type(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.fixed_bytes_type.parse_recovery(source);
        node.unwrap()
    }

    // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
    pub fn parse_for_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.for_statement.parse_recovery(source);
        node.unwrap()
    }

    // FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
    pub fn parse_function_attribute(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.function_attribute.parse_recovery(source);
        node.unwrap()
    }

    // FunctionCallExpression = Expression [ '{' NamedArgument  { ',' NamedArgument } '}' ] ArgumentList ;
    pub fn parse_function_call_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.function_call_expression.parse_recovery(source);
        node.unwrap()
    }

    // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    pub fn parse_function_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.function_definition.parse_recovery(source);
        node.unwrap()
    }

    // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
    pub fn parse_function_type(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.function_type.parse_recovery(source);
        node.unwrap()
    }

    // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
    pub fn parse_hex_byte_escape(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.hex_byte_escape.parse_recovery(source);
        node.unwrap()
    }

    // «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
    pub fn parse_hex_character(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.hex_character.parse_recovery(source);
        node.unwrap()
    }

    // «HexNumber» = '0x' «HexCharacter» { [ '_' ] «HexCharacter» } ;
    pub fn parse_hex_number(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.hex_number.parse_recovery(source);
        node.unwrap()
    }

    // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
    pub fn parse_hex_string_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.hex_string_literal.parse_recovery(source);
        node.unwrap()
    }

    // «Identifier» = «RawIdentifier» - «Keyword» ;
    pub fn parse_identifier(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.identifier.parse_recovery(source);
        node.unwrap()
    }

    // «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
    pub fn parse_identifier_part(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.identifier_part.parse_recovery(source);
        node.unwrap()
    }

    // IdentifierPath = «Identifier»  { '.' «Identifier» } ;
    pub fn parse_identifier_path(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.identifier_path.parse_recovery(source);
        node.unwrap()
    }

    // «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
    pub fn parse_identifier_start(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.identifier_start.parse_recovery(source);
        node.unwrap()
    }

    // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
    pub fn parse_if_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.if_statement.parse_recovery(source);
        node.unwrap()
    }

    // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
    pub fn parse_import_directive(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.import_directive.parse_recovery(source);
        node.unwrap()
    }

    // ImportPath = «AsciiStringLiteral» ;
    pub fn parse_import_path(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.import_path.parse_recovery(source);
        node.unwrap()
    }

    // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
    pub fn parse_index_access_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.index_access_expression.parse_recovery(source);
        node.unwrap()
    }

    // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
    pub fn parse_inheritance_specifier(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.inheritance_specifier.parse_recovery(source);
        node.unwrap()
    }

    // InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
    pub fn parse_inheritance_specifier_list(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .inheritance_specifier_list
            .parse_recovery(source);
        node.unwrap()
    }

    // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    pub fn parse_interface_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.interface_definition.parse_recovery(source);
        node.unwrap()
    }

    // «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
    pub fn parse_keyword(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.keyword.parse_recovery(source);
        node.unwrap()
    }

    // LeadingTrivia = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
    pub fn parse_leading_trivia(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.leading_trivia.parse_recovery(source);
        node.unwrap()
    }

    // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
    pub fn parse_library_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.library_definition.parse_recovery(source);
        node.unwrap()
    }

    // MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
    pub fn parse_mapping_type(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.mapping_type.parse_recovery(source);
        node.unwrap()
    }

    // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
    pub fn parse_member_access_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.member_access_expression.parse_recovery(source);
        node.unwrap()
    }

    // ModifierAttribute = OverrideSpecifier | 'virtual' ;
    pub fn parse_modifier_attribute(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.modifier_attribute.parse_recovery(source);
        node.unwrap()
    }

    // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
    pub fn parse_modifier_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.modifier_definition.parse_recovery(source);
        node.unwrap()
    }

    // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
    pub fn parse_modifier_invocation(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.modifier_invocation.parse_recovery(source);
        node.unwrap()
    }

    // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
    pub fn parse_mul_div_mod_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.mul_div_mod_expression.parse_recovery(source);
        node.unwrap()
    }

    // «MultilineComment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
    pub fn parse_multiline_comment(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.multiline_comment.parse_recovery(source);
        node.unwrap()
    }

    // NamedArgument = «Identifier» ':' Expression ;
    pub fn parse_named_argument(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.named_argument.parse_recovery(source);
        node.unwrap()
    }

    // NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
    pub fn parse_named_argument_list(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.named_argument_list.parse_recovery(source);
        node.unwrap()
    }

    // NewExpression = 'new' IdentifierPath ArgumentList ;
    pub fn parse_new_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.new_expression.parse_recovery(source);
        node.unwrap()
    }

    // «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
    pub fn parse_number_unit(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.number_unit.parse_recovery(source);
        node.unwrap()
    }

    // «NumericLiteral» = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
    pub fn parse_numeric_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.numeric_literal.parse_recovery(source);
        node.unwrap()
    }

    // OrExpression = Expression '||' Expression ;
    pub fn parse_or_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.or_expression.parse_recovery(source);
        node.unwrap()
    }

    // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
    pub fn parse_order_comparison_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .order_comparison_expression
            .parse_recovery(source);
        node.unwrap()
    }

    // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
    pub fn parse_override_specifier(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.override_specifier.parse_recovery(source);
        node.unwrap()
    }

    // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
    pub fn parse_parameter_declaration(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.parameter_declaration.parse_recovery(source);
        node.unwrap()
    }

    // ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
    pub fn parse_parameter_list(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.parameter_list.parse_recovery(source);
        node.unwrap()
    }

    // ParenthesisExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
    pub fn parse_parenthesis_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.parenthesis_expression.parse_recovery(source);
        node.unwrap()
    }

    // PayableExpression = 'payable' ArgumentList ;
    pub fn parse_payable_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.payable_expression.parse_recovery(source);
        node.unwrap()
    }

    // PositionalArgumentList = Expression  { ',' Expression } ;
    pub fn parse_positional_argument_list(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.positional_argument_list.parse_recovery(source);
        node.unwrap()
    }

    // «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
    pub fn parse_possibly_separated_pairs_of_hex_digits(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self
            .parsers
            .possibly_separated_pairs_of_hex_digits
            .parse_recovery(source);
        node.unwrap()
    }

    // PragmaDirective = 'pragma' ( VersionPragmaSpecifier | ABICoderPragmaSpecifier | ExperimentalPragmaSpecifier ) ';' ;
    pub fn parse_pragma_directive(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.pragma_directive.parse_recovery(source);
        node.unwrap()
    }

    // PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | «NumericLiteral» | «BooleanLiteral» | «Identifier» ;
    pub fn parse_primary_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.primary_expression.parse_recovery(source);
        node.unwrap()
    }

    // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
    pub fn parse_raw_identifier(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.raw_identifier.parse_recovery(source);
        node.unwrap()
    }

    // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
    pub fn parse_receive_function_attribute(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .receive_function_attribute
            .parse_recovery(source);
        node.unwrap()
    }

    // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
    pub fn parse_receive_function_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .receive_function_definition
            .parse_recovery(source);
        node.unwrap()
    }

    // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
    pub fn parse_reserved_keyword(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.reserved_keyword.parse_recovery(source);
        node.unwrap()
    }

    // ReturnStatement = 'return' [ Expression ] ';' ;
    pub fn parse_return_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.return_statement.parse_recovery(source);
        node.unwrap()
    }

    // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
    pub fn parse_revert_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.revert_statement.parse_recovery(source);
        node.unwrap()
    }

    // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
    pub fn parse_selected_import(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.selected_import.parse_recovery(source);
        node.unwrap()
    }

    // SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
    pub fn parse_selecting_import_directive(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .selecting_import_directive
            .parse_recovery(source);
        node.unwrap()
    }

    // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
    pub fn parse_shift_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.shift_expression.parse_recovery(source);
        node.unwrap()
    }

    // «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
    pub fn parse_signed_fixed_type(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.signed_fixed_type.parse_recovery(source);
        node.unwrap()
    }

    // «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
    pub fn parse_signed_integer_type(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.signed_integer_type.parse_recovery(source);
        node.unwrap()
    }

    // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
    pub fn parse_simple_import_directive(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.simple_import_directive.parse_recovery(source);
        node.unwrap()
    }

    // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
    pub fn parse_simple_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.simple_statement.parse_recovery(source);
        node.unwrap()
    }

    // «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
    pub fn parse_single_line_comment(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.single_line_comment.parse_recovery(source);
        node.unwrap()
    }

    // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
    pub fn parse_single_quoted_ascii_string_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self
            .parsers
            .single_quoted_ascii_string_literal
            .parse_recovery(source);
        node.unwrap()
    }

    // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
    pub fn parse_single_quoted_unicode_string_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self
            .parsers
            .single_quoted_unicode_string_literal
            .parse_recovery(source);
        node.unwrap()
    }

    // SourceUnit = LeadingTrivia { Directive | Definition } EndOfFileTrivia ;
    pub fn parse_source_unit(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.source_unit.parse_recovery(source);
        node.unwrap()
    }

    // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
    pub fn parse_star_import_directive(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.star_import_directive.parse_recovery(source);
        node.unwrap()
    }

    // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
    pub fn parse_state_variable_attribute(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.state_variable_attribute.parse_recovery(source);
        node.unwrap()
    }

    // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
    pub fn parse_state_variable_declaration(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .state_variable_declaration
            .parse_recovery(source);
        node.unwrap()
    }

    // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
    pub fn parse_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.statement.parse_recovery(source);
        node.unwrap()
    }

    // StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
    pub fn parse_struct_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.struct_definition.parse_recovery(source);
        node.unwrap()
    }

    // StructMember = TypeName «Identifier» ';' ;
    pub fn parse_struct_member(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.struct_member.parse_recovery(source);
        node.unwrap()
    }

    // TrailingTrivia = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
    pub fn parse_trailing_trivia(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.trailing_trivia.parse_recovery(source);
        node.unwrap()
    }

    // TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
    pub fn parse_try_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.try_statement.parse_recovery(source);
        node.unwrap()
    }

    // TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
    pub fn parse_tuple_deconstruction_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .tuple_deconstruction_statement
            .parse_recovery(source);
        node.unwrap()
    }

    // TypeExpression = 'type' '(' TypeName ')' ;
    pub fn parse_type_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.type_expression.parse_recovery(source);
        node.unwrap()
    }

    // TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
    pub fn parse_type_name(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.type_name.parse_recovery(source);
        node.unwrap()
    }

    // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
    pub fn parse_unary_prefix_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.unary_prefix_expression.parse_recovery(source);
        node.unwrap()
    }

    // UnarySuffixExpression = Expression ( '++' | '--' ) ;
    pub fn parse_unary_suffix_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.unary_suffix_expression.parse_recovery(source);
        node.unwrap()
    }

    // UncheckedBlock = 'unchecked' Block ;
    pub fn parse_unchecked_block(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.unchecked_block.parse_recovery(source);
        node.unwrap()
    }

    // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
    pub fn parse_unicode_escape(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.unicode_escape.parse_recovery(source);
        node.unwrap()
    }

    // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
    pub fn parse_unicode_string_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.unicode_string_literal.parse_recovery(source);
        node.unwrap()
    }

    // «UnsignedFixedType» = 'u' «SignedFixedType» ;
    pub fn parse_unsigned_fixed_type(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.unsigned_fixed_type.parse_recovery(source);
        node.unwrap()
    }

    // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
    pub fn parse_unsigned_integer_type(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.unsigned_integer_type.parse_recovery(source);
        node.unwrap()
    }

    // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
    pub fn parse_user_defined_value_type_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .user_defined_value_type_definition
            .parse_recovery(source);
        node.unwrap()
    }

    // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
    pub fn parse_using_directive(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.using_directive.parse_recovery(source);
        node.unwrap()
    }

    // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
    pub fn parse_variable_declaration_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self
            .parsers
            .variable_declaration_statement
            .parse_recovery(source);
        node.unwrap()
    }

    // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
    pub fn parse_version_pragma_operator(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.version_pragma_operator.parse_recovery(source);
        node.unwrap()
    }

    // VersionPragmaSpecifier = 'solidity' 1…*{ «VersionPragmaOperator» «VersionPragmaValue» } ;
    pub fn parse_version_pragma_specifier(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.version_pragma_specifier.parse_recovery(source);
        node.unwrap()
    }

    // «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' }  { '.' 1…*{ '0'…'9' | 'x' | 'X' | '*' } } ;
    pub fn parse_version_pragma_value(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.version_pragma_value.parse_recovery(source);
        node.unwrap()
    }

    // WhileStatement = 'while' '(' Expression ')' Statement ;
    pub fn parse_while_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.while_statement.parse_recovery(source);
        node.unwrap()
    }

    // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
    pub fn parse_whitespace(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.whitespace.parse_recovery(source);
        node.unwrap()
    }

    // YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
    pub fn parse_yul_assignment_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_assignment_statement.parse_recovery(source);
        node.unwrap()
    }

    // YulBlock = '{' { YulStatement } '}' ;
    pub fn parse_yul_block(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_block.parse_recovery(source);
        node.unwrap()
    }

    // YulBreakStatement = 'break' ;
    pub fn parse_yul_break_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_break_statement.parse_recovery(source);
        node.unwrap()
    }

    // YulContinueStatement = 'continue' ;
    pub fn parse_yul_continue_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_continue_statement.parse_recovery(source);
        node.unwrap()
    }

    // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
    pub fn parse_yul_decimal_number_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self
            .parsers
            .yul_decimal_number_literal
            .parse_recovery(source);
        node.unwrap()
    }

    // YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
    pub fn parse_yul_expression(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_expression.parse_recovery(source);
        node.unwrap()
    }

    // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
    pub fn parse_yul_for_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_for_statement.parse_recovery(source);
        node.unwrap()
    }

    // YulFunctionCall = «YulIdentifier» '(' [ YulExpression  { ',' YulExpression } ] ')' ;
    pub fn parse_yul_function_call(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_function_call.parse_recovery(source);
        node.unwrap()
    }

    // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
    pub fn parse_yul_function_definition(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_function_definition.parse_recovery(source);
        node.unwrap()
    }

    // «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
    pub fn parse_yul_hex_literal(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.yul_hex_literal.parse_recovery(source);
        node.unwrap()
    }

    // «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
    pub fn parse_yul_identifier(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.yul_identifier.parse_recovery(source);
        node.unwrap()
    }

    // YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
    pub fn parse_yul_identifier_path(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_identifier_path.parse_recovery(source);
        node.unwrap()
    }

    // YulIfStatement = 'if' YulExpression YulBlock ;
    pub fn parse_yul_if_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_if_statement.parse_recovery(source);
        node.unwrap()
    }

    // «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
    pub fn parse_yul_keyword(&self, source: &str) -> Rc<lex::Node> {
        let (node, _errs) = self.parsers.yul_keyword.parse_recovery(source);
        node.unwrap()
    }

    // YulLeaveStatement = 'leave' ;
    pub fn parse_yul_leave_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_leave_statement.parse_recovery(source);
        node.unwrap()
    }

    // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
    pub fn parse_yul_literal(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_literal.parse_recovery(source);
        node.unwrap()
    }

    // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
    pub fn parse_yul_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_statement.parse_recovery(source);
        node.unwrap()
    }

    // YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
    pub fn parse_yul_switch_statement(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_switch_statement.parse_recovery(source);
        node.unwrap()
    }

    // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
    pub fn parse_yul_variable_declaration(&self, source: &str) -> Rc<cst::Node> {
        let (node, _errs) = self.parsers.yul_variable_declaration.parse_recovery(source);
        node.unwrap()
    }
}
