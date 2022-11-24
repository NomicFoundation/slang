// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::cst::RcNodeExtensions as CSTRcNodeExtensions;
use super::parse::Parsers;
use chumsky::Parser;
use napi::bindgen_prelude::*;
use semver::Version;
#[napi]
pub struct Language {
    parsers: Parsers<'static>,
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
impl Language {
    // ABICoderPragmaSpecifier = 'abicoder' «Identifier» ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_abi_coder_pragma_specifier(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .abi_coder_pragma_specifier
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // AddSubExpression = Expression ( '+' | '-' ) Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_add_sub_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .add_sub_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // AddressType = 'address' [ 'payable' ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_address_type(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.address_type.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // AndExpression = Expression '&&' Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_and_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.and_expression.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_argument_list(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.argument_list.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_array_literal(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.array_literal.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_ascii_escape(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.ascii_escape.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_ascii_string_literal(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .ascii_string_literal
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_assembly_flags(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.assembly_flags.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_assembly_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .assembly_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_assignment_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .assignment_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // BitAndExpression = Expression '&' Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_bit_and_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .bit_and_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // BitOrExpression = Expression '|' Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_bit_or_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .bit_or_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // BitXOrExpression = Expression '^' Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_bit_x_or_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .bit_x_or_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // Block = '{' { Statement | UncheckedBlock } '}' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_block(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.block.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «BooleanLiteral» = 'true' | 'false' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_boolean_literal(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.boolean_literal.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // BreakStatement = 'break' ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_break_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.break_statement.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_catch_clause(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.catch_clause.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ConditionalExpression = Expression '?' Expression ':' Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_conditional_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .conditional_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_constant_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .constant_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_constructor_attribute(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .constructor_attribute
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_constructor_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .constructor_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ContinueStatement = 'continue' ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_continue_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .continue_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_contract_body_element(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .contract_body_element
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_contract_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .contract_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // DataLocation = 'memory' | 'storage' | 'calldata' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_data_location(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.data_location.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_decimal_exponent(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .decimal_exponent
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_decimal_float(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.decimal_float.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_decimal_integer(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.decimal_integer.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_decimal_number(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.decimal_number.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.definition.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // DeleteStatement = 'delete' «Identifier» ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_delete_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .delete_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // Directive = PragmaDirective | ImportDirective | UsingDirective ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_directive(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.directive.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_do_while_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .do_while_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_double_quoted_ascii_string_literal(
        &self,
        env: Env,
        source: String,
    ) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .double_quoted_ascii_string_literal
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_double_quoted_unicode_string_literal(
        &self,
        env: Env,
        source: String,
    ) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .double_quoted_unicode_string_literal
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_elementary_type(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.elementary_type.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_emit_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.emit_statement.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // EndOfFileTrivia = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_end_of_file_trivia(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .end_of_file_trivia
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_end_of_line(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.end_of_line.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_enum_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.enum_definition.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_equality_comparison_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .equality_comparison_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_error_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .error_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ErrorParameter = TypeName [ «Identifier» ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_error_parameter(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.error_parameter.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_escape_sequence(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.escape_sequence.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_event_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .event_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_event_parameter(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.event_parameter.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ExperimentalPragmaSpecifier = 'experimental' «Identifier» ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_experimental_pragma_specifier(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .experimental_pragma_specifier
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // (* 0.0.0 *) ExponentiationExpression = Expression '**' Expression ;
    // (* 0.6.0 *) ExponentiationExpression = Expression '**' Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_exponentiation_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .exponentiation_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.expression.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ExpressionStatement = Expression ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_expression_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .expression_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_fallback_function_attribute(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .fallback_function_attribute
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_fallback_function_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .fallback_function_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_fixed_bytes_type(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .fixed_bytes_type
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_for_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.for_statement.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_function_attribute(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .function_attribute
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // FunctionCallExpression = Expression [ '{' NamedArgument  { ',' NamedArgument } '}' ] ArgumentList ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_function_call_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .function_call_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_function_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .function_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_function_type(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.function_type.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_hex_byte_escape(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.hex_byte_escape.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_hex_character(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.hex_character.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «HexNumber» = '0x' «HexCharacter» { [ '_' ] «HexCharacter» } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_hex_number(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.hex_number.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_hex_string_literal(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .hex_string_literal
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «Identifier» = «RawIdentifier» - «Keyword» ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_identifier(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.identifier.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_identifier_part(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.identifier_part.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // IdentifierPath = «Identifier»  { '.' «Identifier» } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_identifier_path(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.identifier_path.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_identifier_start(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .identifier_start
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_if_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.if_statement.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_import_directive(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .import_directive
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ImportPath = «AsciiStringLiteral» ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_import_path(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.import_path.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_index_access_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .index_access_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_inheritance_specifier(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .inheritance_specifier
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_inheritance_specifier_list(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .inheritance_specifier_list
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_interface_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .interface_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_keyword(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.keyword.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // LeadingTrivia = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_leading_trivia(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.leading_trivia.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_library_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .library_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_mapping_type(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.mapping_type.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_member_access_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .member_access_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ModifierAttribute = OverrideSpecifier | 'virtual' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_modifier_attribute(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .modifier_attribute
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_modifier_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .modifier_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_modifier_invocation(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .modifier_invocation
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_mul_div_mod_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .mul_div_mod_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «MultilineComment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_multiline_comment(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .multiline_comment
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // NamedArgument = «Identifier» ':' Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_named_argument(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.named_argument.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_named_argument_list(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .named_argument_list
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // NewExpression = 'new' IdentifierPath ArgumentList ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_new_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.new_expression.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_number_unit(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.number_unit.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «NumericLiteral» = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_numeric_literal(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.numeric_literal.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // OrExpression = Expression '||' Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_or_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.or_expression.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_order_comparison_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .order_comparison_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_override_specifier(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .override_specifier
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_parameter_declaration(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .parameter_declaration
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_parameter_list(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.parameter_list.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ParenthesisExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_parenthesis_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .parenthesis_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // PayableExpression = 'payable' ArgumentList ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_payable_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .payable_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // PositionalArgumentList = Expression  { ',' Expression } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_positional_argument_list(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .positional_argument_list
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_possibly_separated_pairs_of_hex_digits(
        &self,
        env: Env,
        source: String,
    ) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .possibly_separated_pairs_of_hex_digits
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // PragmaDirective = 'pragma' ( VersionPragmaSpecifier | ABICoderPragmaSpecifier | ExperimentalPragmaSpecifier ) ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_pragma_directive(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .pragma_directive
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | «NumericLiteral» | «BooleanLiteral» | «Identifier» ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_primary_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .primary_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_raw_identifier(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.raw_identifier.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_receive_function_attribute(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .receive_function_attribute
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_receive_function_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .receive_function_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_reserved_keyword(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .reserved_keyword
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ReturnStatement = 'return' [ Expression ] ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_return_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .return_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_revert_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .revert_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_selected_import(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.selected_import.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_selecting_import_directive(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .selecting_import_directive
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_shift_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .shift_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_signed_fixed_type(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .signed_fixed_type
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_signed_integer_type(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .signed_integer_type
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_simple_import_directive(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .simple_import_directive
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_simple_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .simple_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_single_line_comment(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .single_line_comment
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_single_quoted_ascii_string_literal(
        &self,
        env: Env,
        source: String,
    ) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .single_quoted_ascii_string_literal
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_single_quoted_unicode_string_literal(
        &self,
        env: Env,
        source: String,
    ) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .single_quoted_unicode_string_literal
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // SourceUnit = LeadingTrivia { Directive | Definition } EndOfFileTrivia ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_source_unit(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.source_unit.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_star_import_directive(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .star_import_directive
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_state_variable_attribute(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .state_variable_attribute
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_state_variable_declaration(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .state_variable_declaration
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.statement.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_struct_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .struct_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // StructMember = TypeName «Identifier» ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_struct_member(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.struct_member.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // TrailingTrivia = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_trailing_trivia(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.trailing_trivia.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_try_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.try_statement.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_tuple_deconstruction_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .tuple_deconstruction_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // TypeExpression = 'type' '(' TypeName ')' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_type_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.type_expression.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_type_name(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.type_name.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_unary_prefix_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .unary_prefix_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // UnarySuffixExpression = Expression ( '++' | '--' ) ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_unary_suffix_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .unary_suffix_expression
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // UncheckedBlock = 'unchecked' Block ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_unchecked_block(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.unchecked_block.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_unicode_escape(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.unicode_escape.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_unicode_string_literal(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .unicode_string_literal
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «UnsignedFixedType» = 'u' «SignedFixedType» ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_unsigned_fixed_type(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .unsigned_fixed_type
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_unsigned_integer_type(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .unsigned_integer_type
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_user_defined_value_type_definition(
        &self,
        env: Env,
        source: String,
    ) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .user_defined_value_type_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_using_directive(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.using_directive.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_variable_declaration_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .variable_declaration_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_version_pragma_operator(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .version_pragma_operator
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // VersionPragmaSpecifier = 'solidity' 1…*{ «VersionPragmaOperator» «VersionPragmaValue» } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_version_pragma_specifier(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .version_pragma_specifier
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' }  { '.' 1…*{ '0'…'9' | 'x' | 'X' | '*' } } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_version_pragma_value(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .version_pragma_value
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // WhileStatement = 'while' '(' Expression ')' Statement ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_while_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.while_statement.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_whitespace(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.whitespace.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_assignment_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_assignment_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulBlock = '{' { YulStatement } '}' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_block(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.yul_block.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulBreakStatement = 'break' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_break_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_break_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulContinueStatement = 'continue' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_continue_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_continue_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_yul_decimal_number_literal(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_decimal_number_literal
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_expression(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.yul_expression.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_for_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_for_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulFunctionCall = «YulIdentifier» '(' [ YulExpression  { ',' YulExpression } ] ')' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_function_call(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_function_call
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_function_definition(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_function_definition
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_yul_hex_literal(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.yul_hex_literal.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_yul_identifier(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.yul_identifier.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_identifier_path(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_identifier_path
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulIfStatement = 'if' YulExpression YulBlock ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_if_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_if_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
    #[napi(ts_return_type = "CSTTokenNode")]
    pub fn parse_yul_keyword(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.yul_keyword.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulLeaveStatement = 'leave' ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_leave_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_leave_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_literal(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.yul_literal.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self.parsers.yul_statement.parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_switch_statement(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_switch_statement
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }

    // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
    #[napi(ts_return_type = "CSTRuleNode")]
    pub fn parse_yul_variable_declaration(&self, env: Env, source: String) -> napi::JsObject {
        let (node, _errs) = self
            .parsers
            .yul_variable_declaration
            .parse_recovery(source.as_str());
        node.unwrap().to_js(&env)
    }
}
