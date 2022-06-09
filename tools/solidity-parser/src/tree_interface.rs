#[doc = "AddSubOperator = '+' | '-' ;"]
pub mod add_sub_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "AssignmentOperator = '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ;"]
pub mod assignment_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "BreakStatement = 'break' ';' ;"]
pub mod break_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<break_statement::_S0>;
    pub struct _S0 {
        pub r#break: usize,
        pub semicolon_char: char,
    }
}
#[doc = "«Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } 1…*{ '*' } '/' ;"]
pub mod comment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<comment::_S0>;
    pub struct _S0 {
        pub slash_star: usize,
        pub _c2s: Vec<Box<comment::_C2>>,
        pub star_chars: Vec<char>,
        pub slash_char: char,
    }
    pub enum _C2 {
        StarChar(char),
        _S3(Box<comment::_S3>),
    }
    pub struct _S3 {
        pub star_chars: Vec<char>,
        pub _1: char,
    }
}
#[doc = "ContinueStatement = 'continue' ';' ;"]
pub mod continue_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<continue_statement::_S0>;
    pub struct _S0 {
        pub r#continue: usize,
        pub semicolon_char: char,
    }
}
#[doc = "DataLocation = 'memory' | 'storage' | 'calldata' ;"]
pub mod data_location {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "EqualityComparisonOperator = '==' | '!=' ;"]
pub mod equality_comparison_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "«LineComment» = '//' { ¬( '\\u{a}' | '\\u{d}' ) } ;"]
pub mod line_comment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<line_comment::_S0>;
    pub struct _S0 {
        pub slash_slash: usize,
        pub _1: Vec<char>,
    }
}
#[doc = "MulDivModOperator = '*' | '/' | '%' ;"]
pub mod mul_div_mod_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "OrderComparisonOperator = '<' | '>' | '<=' | '>=' ;"]
pub mod order_comparison_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "PositionalArgumentList = 1…*{ Expression / ',' } ;"]
pub mod positional_argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<positional_argument_list::_S0>;
    pub struct _S0 {
        pub expressions: Vec<expression::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "ShiftOperator = '<<' | '>>' | '>>>' ;"]
pub mod shift_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "StateMutabilitySpecifier = 'pure' | 'view' | 'payable' ;"]
pub mod state_mutability_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "UnaryPrefixOperator = '++' | '--' | '!' | '~' | 'delete' | '-' ;"]
pub mod unary_prefix_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "UnarySuffixOperator = '++' | '--' ;"]
pub mod unary_suffix_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "UncheckedBlock = 'unchecked' Block ;"]
pub mod unchecked_block {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unchecked_block::_S0>;
    pub struct _S0 {
        pub unchecked: usize,
        pub block: block::N,
    }
}
#[doc = "VisibilitySpecifier = 'internal' | 'external' | 'private' | 'public' ;"]
pub mod visibility_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "«Whitespace» = '\\u{20}' | '\\u{9}' | '\\u{d}' | '\\u{a}' ;"]
pub mod whitespace {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "YulBreakStatement = 'break' ;"]
pub mod yul_break_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "YulContinueStatement = 'continue' ;"]
pub mod yul_continue_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "YulLeaveStatement = 'leave' ;"]
pub mod yul_leave_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "«IGNORE» = { «Whitespace» | «Comment» | «LineComment» } ;"]
pub mod ignore {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<Box<ignore::_C1>>;
    pub enum _C1 {
        _0(char),
        Comment(comment::N),
        LineComment(line_comment::N),
    }
}
#[doc = "«AsciiEscape» = 'n' | 'r' | 't' | '\\'' | '\"' | '\\\\' | '\\u{a}' | '\\u{d}' ;"]
pub mod ascii_escape {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "«BooleanLiteral» = 'true' | 'false' ;"]
pub mod boolean_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "«DecimalInteger» = 1…*{ '0'…'9' / [ '_' ] } ;"]
pub mod decimal_integer {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_integer::_S0>;
    pub struct _S0 {
        pub expressions: Vec<char>,
        pub underscore_chars: Vec<Option<char>>,
    }
}
#[doc = "«FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;"]
pub mod fixed_bytes_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fixed_bytes_type::_S0>;
    pub struct _S0 {
        pub bytes: usize,
        pub _1: usize,
    }
}
#[doc = "«FixedType» = 'fixed' [ '1'…'9' { '0'…'9' } 'x' '1'…'9' { '0'…'9' } ] ;"]
pub mod fixed_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fixed_type::_S0>;
    pub struct _S0 {
        pub fixed: usize,
        pub _s2: Option<Box<fixed_type::_S2>>,
    }
    pub struct _S2 {
        pub _0: char,
        pub _1: Vec<char>,
        pub _2: char,
        pub _3: char,
        pub _4: Vec<char>,
    }
}
#[doc = "«HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;"]
pub mod hex_character {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "«IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;"]
pub mod identifier_start {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "«NumberUnit» = 'wei' | 'gwei' | 'ether' | 'seconds' | 'minutes' | 'hours' | 'days' | 'weeks' | 'years' ;"]
pub mod number_unit {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "«PragmaDirective» = 'pragma' ¬';' { ¬';' } ';' ;"]
pub mod pragma_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<pragma_directive::_S0>;
    pub struct _S0 {
        pub pragma: usize,
        pub semicolon_char_0: char,
        pub semicolon_chars: Vec<char>,
        pub semicolon_char_1: char,
    }
}
#[doc = "«ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;"]
pub mod reserved_keyword {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "«SignedIntegerType» = 'int' ( '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ) ;"]
pub mod signed_integer_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<signed_integer_type::_S0>;
    pub struct _S0 {
        pub int: usize,
        pub _1: usize,
    }
}
#[doc = "«YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;"]
pub mod yul_decimal_number_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_decimal_number_literal::_C0>;
    pub enum _C0 {
        ZeroChar(char),
        _S1(Box<yul_decimal_number_literal::_S1>),
    }
    pub struct _S1 {
        pub _0: char,
        pub _1: Vec<char>,
    }
}
#[doc = "«YulEVMBuiltinFunctionName» = 'stop' | 'add' | 'sub' | 'mul' | 'div' | 'sdiv' | 'mod' | 'smod' | 'exp' | 'not' | 'lt' | 'gt' | 'slt' | 'sgt' | 'eq' | 'iszero' | 'and' | 'or' | 'xor' | 'byte' | 'shl' | 'shr' | 'sar' | 'addmod' | 'mulmod' | 'signextend' | 'keccak256' | 'pop' | 'mload' | 'mstore' | 'mstore8' | 'sload' | 'sstore' | 'msize' | 'gas' | 'address' | 'balance' | 'selfbalance' | 'caller' | 'callvalue' | 'calldataload' | 'calldatasize' | 'calldatacopy' | 'extcodesize' | 'extcodecopy' | 'returndatasize' | 'returndatacopy' | 'extcodehash' | 'create' | 'create2' | 'call' | 'callcode' | 'delegatecall' | 'staticcall' | 'return' | 'revert' | 'selfdestruct' | 'invalid' | 'log0' | 'log1' | 'log2' | 'log3' | 'log4' | 'chainid' | 'origin' | 'gasprice' | 'Blockhash' | 'coinbase' | 'timestamp' | 'number' | 'difficulty' | 'gaslimit' | 'basefee' ;"]
pub mod yul_evm_builtin_function_name {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "«YulHexLiteral» = '0x' ( '0'…'9' | 'a'…'f' | 'A'…'F' ) { '0'…'9' | 'a'…'f' | 'A'…'F' } ;"]
pub mod yul_hex_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_hex_literal::_S0>;
    pub struct _S0 {
        pub zero_x: usize,
        pub _1: char,
        pub _2: Vec<char>,
    }
}
#[doc = "«YulKeyword» = 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'if' | 'leave' | 'let' | 'switch' | 'hex' ;"]
pub mod yul_keyword {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "«DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;"]
pub mod decimal_exponent {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_exponent::_S0>;
    pub struct _S0 {
        pub _0: char,
        pub minus_char: Option<char>,
        pub decimal_integer: decimal_integer::N,
    }
}
#[doc = "«DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;"]
pub mod decimal_float {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_float::_S0>;
    pub struct _S0 {
        pub decimal_integer_0: Option<decimal_integer::N>,
        pub period_char: char,
        pub decimal_integer_1: decimal_integer::N,
    }
}
#[doc = "«HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;"]
pub mod hex_byte_escape {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<hex_byte_escape::_S0>;
    pub struct _S0 {
        pub _0: char,
        pub _1: Vec<char>,
    }
}
#[doc = "«HexNumber» = '0' 'x' 1…*{ «HexCharacter» / [ '_' ] } ;"]
pub mod hex_number {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<hex_number::_S0>;
    pub struct _S0 {
        pub zero_char: char,
        pub _1: char,
        pub _2: Box<hex_number::_S1>,
    }
    pub struct _S1 {
        pub expressions: Vec<char>,
        pub underscore_chars: Vec<Option<char>>,
    }
}
#[doc = "«IdentifierPart» = «IdentifierStart» | '0'…'9' ;"]
pub mod identifier_part {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "«PossiblySeparatedPairsOfHexDigits» = 1…*{ 2…2*{ «HexCharacter» } / [ '_' ] } ;"]
pub mod possibly_separated_pairs_of_hex_digits {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<possibly_separated_pairs_of_hex_digits::_S0>;
    pub struct _S0 {
        pub expressions: Vec<Vec<char>>,
        pub underscore_chars: Vec<Option<char>>,
    }
}
#[doc = "«UfixedType» = 'u' «FixedType» ;"]
pub mod ufixed_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<ufixed_type::_S0>;
    pub struct _S0 {
        pub _0: char,
        pub fixed_type: fixed_type::N,
    }
}
#[doc = "«UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;"]
pub mod unicode_escape {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unicode_escape::_S0>;
    pub struct _S0 {
        pub _0: char,
        pub _1: Vec<char>,
    }
}
#[doc = "«UnsignedIntegerType» = 'u' «SignedIntegerType» ;"]
pub mod unsigned_integer_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unsigned_integer_type::_S0>;
    pub struct _S0 {
        pub _0: char,
        pub signed_integer_type: signed_integer_type::N,
    }
}
#[doc = "«YulReservedWord» = «YulKeyword» | «YulEVMBuiltinFunctionName» | «BooleanLiteral» ;"]
pub mod yul_reserved_word {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}
#[doc = "«DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;"]
pub mod decimal_number {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_number::_S0>;
    pub struct _S0 {
        pub _c1: Box<decimal_number::_C1>,
        pub decimal_exponent: Option<decimal_exponent::N>,
    }
    pub enum _C1 {
        DecimalInteger(decimal_integer::N),
        DecimalFloat(decimal_float::N),
    }
}
#[doc = "ElementaryType = 'bool' | 'string' | 'bytes' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | «FixedType» | «UfixedType» ;"]
pub mod elementary_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<elementary_type::_C0>;
    pub enum _C0 {
        Bool(usize),
        String(usize),
        Bytes(usize),
        SignedIntegerType(signed_integer_type::N),
        UnsignedIntegerType(unsigned_integer_type::N),
        FixedBytesType(fixed_bytes_type::N),
        FixedType(fixed_type::N),
        UfixedType(ufixed_type::N),
    }
}
#[doc = "«EscapeSequence» = '\\\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;"]
pub mod escape_sequence {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<escape_sequence::_S0>;
    pub struct _S0 {
        pub backslash_char: char,
        pub _c1: Box<escape_sequence::_C1>,
    }
    pub enum _C1 {
        _0(char),
        HexByteEscape(hex_byte_escape::N),
        UnicodeEscape(unicode_escape::N),
    }
}
#[doc = "«HexStringLiteral» = 'hex' ( '\"' [ «PossiblySeparatedPairsOfHexDigits» ] '\"' | '\\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\\'' ) ;"]
pub mod hex_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<hex_string_literal::_S0>;
    pub struct _S0 {
        pub hex: usize,
        pub _c1: Box<hex_string_literal::_C1>,
    }
    pub enum _C1 {
        _S2(Box<hex_string_literal::_S2>),
        _S4(Box<hex_string_literal::_S4>),
    }
    pub struct _S4 {
        pub quote_char_0: char,
        pub possibly_separated_pairs_of_hex_digits:
            Option<possibly_separated_pairs_of_hex_digits::N>,
        pub quote_char_1: char,
    }
    pub struct _S2 {
        pub double_quote_char_0: char,
        pub possibly_separated_pairs_of_hex_digits:
            Option<possibly_separated_pairs_of_hex_digits::N>,
        pub double_quote_char_1: char,
    }
}
#[doc = "«Keyword» = 'pragma' | 'abstract' | 'anonymous' | 'address' | 'as' | 'assembly' | 'bool' | 'break' | 'bytes' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | 'fixed' | 'ufixed' ;"]
pub mod keyword {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<keyword::_C0>;
    pub enum _C0 {
        Pragma(usize),
        Abstract(usize),
        Anonymous(usize),
        Address(usize),
        As(usize),
        Assembly(usize),
        Bool(usize),
        Break(usize),
        Bytes(usize),
        Calldata(usize),
        Catch(usize),
        Constant(usize),
        Constructor(usize),
        Continue(usize),
        Contract(usize),
        Delete(usize),
        Do(usize),
        Else(usize),
        Emit(usize),
        Enum(usize),
        Event(usize),
        External(usize),
        Fallback(usize),
        False(usize),
        For(usize),
        Function(usize),
        Hex(usize),
        If(usize),
        Immutable(usize),
        Import(usize),
        Indexed(usize),
        Interface(usize),
        Internal(usize),
        Is(usize),
        Library(usize),
        Mapping(usize),
        Memory(usize),
        Modifier(usize),
        New(usize),
        Override(usize),
        Payable(usize),
        Private(usize),
        Public(usize),
        Pure(usize),
        Receive(usize),
        Return(usize),
        Returns(usize),
        Storage(usize),
        String(usize),
        Struct(usize),
        True(usize),
        Try(usize),
        Type(usize),
        Unchecked(usize),
        Using(usize),
        View(usize),
        Virtual(usize),
        While(usize),
        SignedIntegerType(signed_integer_type::N),
        UnsignedIntegerType(unsigned_integer_type::N),
        FixedBytesType(fixed_bytes_type::N),
        Fixed(usize),
        Ufixed(usize),
    }
}
#[doc = "«RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;"]
pub mod raw_identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<raw_identifier::_S0>;
    pub struct _S0 {
        pub _0: char,
        pub _1: Vec<char>,
    }
}
#[doc = "«DoubleQuotedAsciiStringLiteral» = '\"' { '\\u{20}'…'~' - ( '\"' | '\\\\' ) | «EscapeSequence» } '\"' ;"]
pub mod double_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<double_quoted_ascii_string_literal::_S0>;
    pub struct _S0 {
        pub double_quote_char_0: char,
        pub _c2s: Vec<Box<double_quoted_ascii_string_literal::_C2>>,
        pub double_quote_char_1: char,
    }
    pub enum _C2 {
        _0(char),
        EscapeSequence(escape_sequence::N),
    }
}
#[doc = "«DoubleQuotedUnicodeStringLiteral» = 'unicode\"' { ¬( '\"' | '\\\\' | '\\u{a}' | '\\u{d}' ) | «EscapeSequence» } '\"' ;"]
pub mod double_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<double_quoted_unicode_string_literal::_S0>;
    pub struct _S0 {
        pub unicode_double_quote: usize,
        pub _c2s: Vec<Box<double_quoted_unicode_string_literal::_C2>>,
        pub double_quote_char: char,
    }
    pub enum _C2 {
        _0(char),
        EscapeSequence(escape_sequence::N),
    }
}
#[doc = "ElementaryTypeWithPayable = 'address' [ 'payable' ] | ElementaryType ;"]
pub mod elementary_type_with_payable {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<elementary_type_with_payable::_C0>;
    pub enum _C0 {
        _S1(Box<elementary_type_with_payable::_S1>),
        ElementaryType(elementary_type::N),
    }
    pub struct _S1 {
        pub address: usize,
        pub payable: Option<usize>,
    }
}
#[doc = "ElementaryTypeWithoutPayable = 'address' | ElementaryType ;"]
pub mod elementary_type_without_payable {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<elementary_type_without_payable::_C0>;
    pub enum _C0 {
        Address(usize),
        ElementaryType(elementary_type::N),
    }
}
#[doc = "NumericLiteral = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;"]
pub mod numeric_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<numeric_literal::_S0>;
    pub struct _S0 {
        pub _c1: Box<numeric_literal::_C1>,
        pub _1: Option<usize>,
    }
    pub enum _C1 {
        DecimalNumber(decimal_number::N),
        HexNumber(hex_number::N),
    }
}
#[doc = "«ReservedWord» = «Keyword» | «ReservedKeyword» | «NumberUnit» | «BooleanLiteral» ;"]
pub mod reserved_word {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<reserved_word::_C0>;
    pub enum _C0 {
        Keyword(keyword::N),
        _1(usize),
        _2(usize),
        _3(usize),
    }
}
#[doc = "«SingleQuotedAsciiStringLiteral» = '\\'' { '\\u{20}'…'~' - ( '\\'' | '\\\\' ) | «EscapeSequence» } '\\'' ;"]
pub mod single_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<single_quoted_ascii_string_literal::_S0>;
    pub struct _S0 {
        pub quote_char_0: char,
        pub _c2s: Vec<Box<single_quoted_ascii_string_literal::_C2>>,
        pub quote_char_1: char,
    }
    pub enum _C2 {
        _0(char),
        EscapeSequence(escape_sequence::N),
    }
}
#[doc = "«SingleQuotedUnicodeStringLiteral» = 'unicode\\'' { ¬( '\\'' | '\\\\' | '\\u{a}' | '\\u{d}' ) | «EscapeSequence» } '\\'' ;"]
pub mod single_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<single_quoted_unicode_string_literal::_S0>;
    pub struct _S0 {
        pub unicode_quote: usize,
        pub _c2s: Vec<Box<single_quoted_unicode_string_literal::_C2>>,
        pub quote_char: char,
    }
    pub enum _C2 {
        _0(char),
        EscapeSequence(escape_sequence::N),
    }
}
#[doc = "«YulIdentifier» = «RawIdentifier» - «YulReservedWord» ;"]
pub mod yul_identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = raw_identifier::N;
}
#[doc = "«AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;"]
pub mod ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<ascii_string_literal::_C0>;
    pub enum _C0 {
        SingleQuotedAsciiStringLiteral(single_quoted_ascii_string_literal::N),
        DoubleQuotedAsciiStringLiteral(double_quoted_ascii_string_literal::N),
    }
}
#[doc = "AssemblyFlags = '(' 1…*{ «DoubleQuotedAsciiStringLiteral» / ',' } ')' ;"]
pub mod assembly_flags {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<assembly_flags::_S0>;
    pub struct _S0 {
        pub open_paren_char: char,
        pub double_quoted_ascii_string_literals: Box<assembly_flags::_S1>,
        pub close_paren_char: char,
    }
    pub struct _S1 {
        pub double_quoted_ascii_string_literals: Vec<double_quoted_ascii_string_literal::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "«Identifier» = «RawIdentifier» - «ReservedWord» ;"]
pub mod identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = raw_identifier::N;
}
#[doc = "«UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;"]
pub mod unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unicode_string_literal::_C0>;
    pub enum _C0 {
        SingleQuotedUnicodeStringLiteral(single_quoted_unicode_string_literal::N),
        DoubleQuotedUnicodeStringLiteral(double_quoted_unicode_string_literal::N),
    }
}
#[doc = "YulFunctionCall = ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) '(' { YulExpression / ',' } ')' ;"]
pub mod yul_function_call {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_function_call::_S0>;
    pub struct _S0 {
        pub _c1: Box<yul_function_call::_C1>,
        pub open_paren_char: char,
        pub yul_expressions: Option<Box<yul_function_call::_S2>>,
        pub close_paren_char: char,
    }
    pub struct _S2 {
        pub yul_expressions: Vec<yul_expression::N>,
        pub comma_chars: Vec<char>,
    }
    pub enum _C1 {
        YulIdentifier(yul_identifier::N),
        _1(usize),
    }
}
#[doc = "YulFunctionDefinition = 'function' «YulIdentifier» '(' { «YulIdentifier» / ',' } ')' [ '->' 1…*{ «YulIdentifier» / ',' } ] YulBlock ;"]
pub mod yul_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_function_definition::_S0>;
    pub struct _S0 {
        pub function: usize,
        pub yul_identifier: yul_identifier::N,
        pub open_paren_char: char,
        pub yul_identifiers: Option<Box<yul_function_definition::_S1>>,
        pub close_paren_char: char,
        pub _s3: Option<Box<yul_function_definition::_S3>>,
        pub yul_block: yul_block::N,
    }
    pub struct _S3 {
        pub minus_greater: usize,
        pub yul_identifiers: Box<yul_function_definition::_S4>,
    }
    pub struct _S4 {
        pub yul_identifiers: Vec<yul_identifier::N>,
        pub comma_chars: Vec<char>,
    }
    pub struct _S1 {
        pub yul_identifiers: Vec<yul_identifier::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "YulPath = «YulIdentifier» { '.' ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) } ;"]
pub mod yul_path {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_path::_S0>;
    pub struct _S0 {
        pub yul_identifier: yul_identifier::N,
        pub _s2s: Vec<Box<yul_path::_S2>>,
    }
    pub struct _S2 {
        pub period_char: char,
        pub _c3: Box<yul_path::_C3>,
    }
    pub enum _C3 {
        YulIdentifier(yul_identifier::N),
        _1(usize),
    }
}
#[doc = "EnumDefinition = 'enum' «Identifier» '{' 1…*{ «Identifier» / ',' } '}' ;"]
pub mod enum_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<enum_definition::_S0>;
    pub struct _S0 {
        pub r#enum: usize,
        pub identifier: identifier::N,
        pub open_brace_char: char,
        pub identifiers: Box<enum_definition::_S1>,
        pub close_brace_char: char,
    }
    pub struct _S1 {
        pub identifiers: Vec<identifier::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "IdentifierPath = 1…*{ «Identifier» / '.' } ;"]
pub mod identifier_path {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<identifier_path::_S0>;
    pub struct _S0 {
        pub identifiers: Vec<identifier::N>,
        pub period_chars: Vec<char>,
    }
}
#[doc = "ImportPath = «AsciiStringLiteral» ;"]
pub mod import_path {
    #[allow(unused_imports)]
    use super::*;
    pub type N = ascii_string_literal::N;
}
#[doc = "Literal = «AsciiStringLiteral» | «UnicodeStringLiteral» | NumericLiteral | «HexStringLiteral» | «BooleanLiteral» ;"]
pub mod literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<literal::_C0>;
    pub enum _C0 {
        AsciiStringLiteral(ascii_string_literal::N),
        UnicodeStringLiteral(unicode_string_literal::N),
        NumericLiteral(numeric_literal::N),
        HexStringLiteral(hex_string_literal::N),
        _4(usize),
    }
}
#[doc = "NamedArgument = «Identifier» ':' Expression ;"]
pub mod named_argument {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<named_argument::_S0>;
    pub struct _S0 {
        pub identifier: identifier::N,
        pub colon_char: char,
        pub expression: expression::N,
    }
}
#[doc = "ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;"]
pub mod parameter_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<parameter_declaration::_S0>;
    pub struct _S0 {
        pub type_name: type_name::N,
        pub _1: Option<usize>,
        pub identifier: Option<identifier::N>,
    }
}
#[doc = "SelectedImport = «Identifier» [ 'as' «Identifier» ] ;"]
pub mod selected_import {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<selected_import::_S0>;
    pub struct _S0 {
        pub identifier: identifier::N,
        pub _s2: Option<Box<selected_import::_S2>>,
    }
    pub struct _S2 {
        pub r#as: usize,
        pub identifier: identifier::N,
    }
}
#[doc = "UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryTypeWithPayable ';' ;"]
pub mod user_defined_value_type_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<user_defined_value_type_definition::_S0>;
    pub struct _S0 {
        pub r#type: usize,
        pub identifier: identifier::N,
        pub is: usize,
        pub elementary_type_with_payable: elementary_type_with_payable::N,
        pub semicolon_char: char,
    }
}
#[doc = "YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;"]
pub mod yul_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_literal::_C0>;
    pub enum _C0 {
        YulDecimalNumberLiteral(yul_decimal_number_literal::N),
        YulHexLiteral(yul_hex_literal::N),
        AsciiStringLiteral(ascii_string_literal::N),
        _3(usize),
        HexStringLiteral(hex_string_literal::N),
    }
}
#[doc = "MappingType = 'mapping' '(' ( ElementaryTypeWithoutPayable | IdentifierPath ) '=>' TypeName ')' ;"]
pub mod mapping_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<mapping_type::_S0>;
    pub struct _S0 {
        pub mapping: usize,
        pub open_paren_char: char,
        pub _c1: Box<mapping_type::_C1>,
        pub equal_greater: usize,
        pub type_name: type_name::N,
        pub close_paren_char: char,
    }
    pub enum _C1 {
        ElementaryTypeWithoutPayable(elementary_type_without_payable::N),
        IdentifierPath(identifier_path::N),
    }
}
#[doc = "NamedArgumentList = '{' { NamedArgument / ',' } '}' ;"]
pub mod named_argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<named_argument_list::_S0>;
    pub struct _S0 {
        pub open_brace_char: char,
        pub named_arguments: Option<Box<named_argument_list::_S1>>,
        pub close_brace_char: char,
    }
    pub struct _S1 {
        pub named_arguments: Vec<named_argument::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "NonEmptyParameterList = '(' 1…*{ ParameterDeclaration / ',' } ')' ;"]
pub mod non_empty_parameter_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<non_empty_parameter_list::_S0>;
    pub struct _S0 {
        pub open_paren_char: char,
        pub parameter_declarations: Box<non_empty_parameter_list::_S1>,
        pub close_paren_char: char,
    }
    pub struct _S1 {
        pub parameter_declarations: Vec<parameter_declaration::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "OverrideSpecifier = 'override' [ '(' 1…1*{ IdentifierPath / ',' } ')' ] ;"]
pub mod override_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<override_specifier::_S0>;
    pub struct _S0 {
        pub r#override: usize,
        pub _s2: Option<Box<override_specifier::_S2>>,
    }
    pub struct _S2 {
        pub open_paren_char: char,
        pub identifier_paths: Box<override_specifier::_S3>,
        pub close_paren_char: char,
    }
    pub struct _S3 {
        pub identifier_paths: Vec<identifier_path::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "ParameterList = '(' { ParameterDeclaration / ',' } ')' ;"]
pub mod parameter_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<parameter_list::_S0>;
    pub struct _S0 {
        pub open_paren_char: char,
        pub parameter_declarations: Option<Box<parameter_list::_S1>>,
        pub close_paren_char: char,
    }
    pub struct _S1 {
        pub parameter_declarations: Vec<parameter_declaration::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "SelectingImportDirective = '{' 1…*{ SelectedImport / ',' } '}' 'from' ImportPath ;"]
pub mod selecting_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<selecting_import_directive::_S0>;
    pub struct _S0 {
        pub open_brace_char: char,
        pub selected_imports: Box<selecting_import_directive::_S1>,
        pub close_brace_char: char,
        pub from: usize,
        pub import_path: import_path::N,
    }
    pub struct _S1 {
        pub selected_imports: Vec<selected_import::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "SimpleImportDirective = ImportPath { 'as' «Identifier» } ;"]
pub mod simple_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<simple_import_directive::_S0>;
    pub struct _S0 {
        pub import_path: import_path::N,
        pub _s2s: Vec<Box<simple_import_directive::_S2>>,
    }
    pub struct _S2 {
        pub r#as: usize,
        pub identifier: identifier::N,
    }
}
#[doc = "StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;"]
pub mod star_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<star_import_directive::_S0>;
    pub struct _S0 {
        pub star_char: char,
        pub r#as: usize,
        pub identifier: identifier::N,
        pub from: usize,
        pub import_path: import_path::N,
    }
}
#[doc = "YulExpression = YulPath | YulFunctionCall | YulLiteral ;"]
pub mod yul_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_expression::_C0>;
    pub enum _C0 {
        YulPath(yul_path::N),
        YulFunctionCall(yul_function_call::N),
        YulLiteral(yul_literal::N),
    }
}
#[doc = "ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;"]
pub mod argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<argument_list::_S0>;
    pub struct _S0 {
        pub open_paren_char: char,
        pub _c2: Option<Box<argument_list::_C2>>,
        pub close_paren_char: char,
    }
    pub enum _C2 {
        PositionalArgumentList(positional_argument_list::N),
        NamedArgumentList(named_argument_list::N),
    }
}
#[doc = "CatchClause = 'catch' [ [ «Identifier» ] NonEmptyParameterList ] Block ;"]
pub mod catch_clause {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<catch_clause::_S0>;
    pub struct _S0 {
        pub catch: usize,
        pub _s2: Option<Box<catch_clause::_S2>>,
        pub block: block::N,
    }
    pub struct _S2 {
        pub identifier: Option<identifier::N>,
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}
#[doc = "FunctionType = 'function' ParameterList { VisibilitySpecifier | StateMutabilitySpecifier } [ 'returns' NonEmptyParameterList ] ;"]
pub mod function_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_type::_S0>;
    pub struct _S0 {
        pub function: usize,
        pub parameter_list: parameter_list::N,
        pub _2: Vec<usize>,
        pub _s3: Option<Box<function_type::_S3>>,
    }
    pub struct _S3 {
        pub returns: usize,
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}
#[doc = "ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;"]
pub mod import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<import_directive::_S0>;
    pub struct _S0 {
        pub import: usize,
        pub _c1: Box<import_directive::_C1>,
        pub semicolon_char: char,
    }
    pub enum _C1 {
        SimpleImportDirective(simple_import_directive::N),
        StarImportDirective(star_import_directive::N),
        SelectingImportDirective(selecting_import_directive::N),
    }
}
#[doc = "MethodAttribute = 'virtual' | OverrideSpecifier ;"]
pub mod method_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<method_attribute::_C0>;
    pub enum _C0 {
        Virtual(usize),
        OverrideSpecifier(override_specifier::N),
    }
}
#[doc = "StateVariableAttribute = 'public' | 'private' | 'internal' | 'constant' | OverrideSpecifier | 'immutable' ;"]
pub mod state_variable_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<state_variable_attribute::_C0>;
    pub enum _C0 {
        Public(usize),
        Private(usize),
        Internal(usize),
        Constant(usize),
        OverrideSpecifier(override_specifier::N),
        Immutable(usize),
    }
}
#[doc = "YulAssignment = YulPath ( ':=' YulExpression | 1…*{ ',' YulPath } ':=' YulFunctionCall ) ;"]
pub mod yul_assignment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_assignment::_S0>;
    pub struct _S0 {
        pub yul_path: yul_path::N,
        pub _c1: Box<yul_assignment::_C1>,
    }
    pub enum _C1 {
        _S2(Box<yul_assignment::_S2>),
        _S3(Box<yul_assignment::_S3>),
    }
    pub struct _S3 {
        pub _s5s: Vec<Box<yul_assignment::_S5>>,
        pub colon_equal: usize,
        pub yul_function_call: yul_function_call::N,
    }
    pub struct _S5 {
        pub comma_char: char,
        pub yul_path: yul_path::N,
    }
    pub struct _S2 {
        pub colon_equal: usize,
        pub yul_expression: yul_expression::N,
    }
}
#[doc = "YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;"]
pub mod yul_for_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_for_statement::_S0>;
    pub struct _S0 {
        pub r#for: usize,
        pub yul_block_0: yul_block::N,
        pub yul_expression: yul_expression::N,
        pub yul_block_1: yul_block::N,
        pub yul_block_2: yul_block::N,
    }
}
#[doc = "YulIfStatement = 'if' YulExpression YulBlock ;"]
pub mod yul_if_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_if_statement::_S0>;
    pub struct _S0 {
        pub r#if: usize,
        pub yul_expression: yul_expression::N,
        pub yul_block: yul_block::N,
    }
}
#[doc = "YulSwitchStatement = 'switch' YulExpression ( 1…*{ 'case' YulLiteral YulBlock } [ 'default' YulBlock ] | 'default' YulBlock ) ;"]
pub mod yul_switch_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_switch_statement::_S0>;
    pub struct _S0 {
        pub switch: usize,
        pub yul_expression: yul_expression::N,
        pub _c1: Box<yul_switch_statement::_C1>,
    }
    pub enum _C1 {
        _S2(Box<yul_switch_statement::_S2>),
        _S7(Box<yul_switch_statement::_S7>),
    }
    pub struct _S7 {
        pub default: usize,
        pub yul_block: yul_block::N,
    }
    pub struct _S2 {
        pub _s4s: Vec<Box<yul_switch_statement::_S4>>,
        pub _s6: Option<Box<yul_switch_statement::_S6>>,
    }
    pub struct _S6 {
        pub default: usize,
        pub yul_block: yul_block::N,
    }
    pub struct _S4 {
        pub case: usize,
        pub yul_literal: yul_literal::N,
        pub yul_block: yul_block::N,
    }
}
#[doc = "YulVariableDeclaration = 'let' «YulIdentifier» [ ':=' YulExpression | [ ',' «YulIdentifier» ] [ ':=' YulFunctionCall ] ] ;"]
pub mod yul_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_variable_declaration::_S0>;
    pub struct _S0 {
        pub r#let: usize,
        pub yul_identifier: yul_identifier::N,
        pub _c2: Option<Box<yul_variable_declaration::_C2>>,
    }
    pub enum _C2 {
        _S3(Box<yul_variable_declaration::_S3>),
        _S4(Box<yul_variable_declaration::_S4>),
    }
    pub struct _S4 {
        pub _s6: Option<Box<yul_variable_declaration::_S6>>,
        pub _s8: Option<Box<yul_variable_declaration::_S8>>,
    }
    pub struct _S8 {
        pub colon_equal: usize,
        pub yul_function_call: yul_function_call::N,
    }
    pub struct _S6 {
        pub comma_char: char,
        pub yul_identifier: yul_identifier::N,
    }
    pub struct _S3 {
        pub colon_equal: usize,
        pub yul_expression: yul_expression::N,
    }
}
#[doc = "InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;"]
pub mod inheritance_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<inheritance_specifier::_S0>;
    pub struct _S0 {
        pub identifier_path: identifier_path::N,
        pub argument_list: Option<argument_list::N>,
    }
}
#[doc = "ModifierInvocation = IdentifierPath [ ArgumentList ] ;"]
pub mod modifier_invocation {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<modifier_invocation::_S0>;
    pub struct _S0 {
        pub identifier_path: identifier_path::N,
        pub argument_list: Option<argument_list::N>,
    }
}
#[doc = "TypeName = ( ElementaryTypeWithPayable | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;"]
pub mod type_name {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<type_name::_S0>;
    pub struct _S0 {
        pub _c1: Box<type_name::_C1>,
        pub _s3s: Vec<Box<type_name::_S3>>,
    }
    pub struct _S3 {
        pub open_bracket_char: char,
        pub expression: Option<expression::N>,
        pub close_bracket_char: char,
    }
    pub enum _C1 {
        ElementaryTypeWithPayable(elementary_type_with_payable::N),
        FunctionType(function_type::N),
        MappingType(mapping_type::N),
        IdentifierPath(identifier_path::N),
    }
}
#[doc = "YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignment | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;"]
pub mod yul_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_statement::_C0>;
    pub enum _C0 {
        YulBlock(yul_block::N),
        YulVariableDeclaration(yul_variable_declaration::N),
        YulFunctionDefinition(yul_function_definition::N),
        YulAssignment(yul_assignment::N),
        YulFunctionCall(yul_function_call::N),
        YulIfStatement(yul_if_statement::N),
        YulForStatement(yul_for_statement::N),
        YulSwitchStatement(yul_switch_statement::N),
        Leave(usize),
        Break(usize),
        Continue(usize),
    }
}
#[doc = "ConstructorAttribute = ModifierInvocation | 'payable' | 'internal' | 'public' ;"]
pub mod constructor_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<constructor_attribute::_C0>;
    pub enum _C0 {
        ModifierInvocation(modifier_invocation::N),
        Payable(usize),
        Internal(usize),
        Public(usize),
    }
}
#[doc = "ErrorParameter = TypeName [ «Identifier» ] ;"]
pub mod error_parameter {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<error_parameter::_S0>;
    pub struct _S0 {
        pub type_name: type_name::N,
        pub identifier: Option<identifier::N>,
    }
}
#[doc = "EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;"]
pub mod event_parameter {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<event_parameter::_S0>;
    pub struct _S0 {
        pub type_name: type_name::N,
        pub indexed: Option<usize>,
        pub identifier: Option<identifier::N>,
    }
}
#[doc = "FallbackFunctionAttribute = 'external' | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;"]
pub mod fallback_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fallback_function_attribute::_C0>;
    pub enum _C0 {
        External(usize),
        _1(usize),
        ModifierInvocation(modifier_invocation::N),
        Virtual(usize),
        OverrideSpecifier(override_specifier::N),
    }
}
#[doc = "FunctionAttribute = VisibilitySpecifier | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;"]
pub mod function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_attribute::_C0>;
    pub enum _C0 {
        _0(usize),
        _1(usize),
        ModifierInvocation(modifier_invocation::N),
        Virtual(usize),
        OverrideSpecifier(override_specifier::N),
    }
}
#[doc = "InheritanceSpecifierList = 'is' 1…*{ InheritanceSpecifier / ',' } ;"]
pub mod inheritance_specifier_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<inheritance_specifier_list::_S0>;
    pub struct _S0 {
        pub is: usize,
        pub inheritance_specifiers: Box<inheritance_specifier_list::_S1>,
    }
    pub struct _S1 {
        pub inheritance_specifiers: Vec<inheritance_specifier::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "PrimaryExpression = 'payable' ArgumentList | 'type' '(' TypeName ')' | 'new' TypeName | '(' 1…*{ [ Expression ] / ',' } ')' | '[' 1…*{ Expression / ',' } ']' | «Identifier» | Literal | ElementaryTypeWithoutPayable ;"]
pub mod primary_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<primary_expression::_C0>;
    pub enum _C0 {
        _S1(Box<primary_expression::_S1>),
        _S2(Box<primary_expression::_S2>),
        _S3(Box<primary_expression::_S3>),
        _S4(Box<primary_expression::_S4>),
        _S7(Box<primary_expression::_S7>),
        Identifier(identifier::N),
        Literal(literal::N),
        ElementaryTypeWithoutPayable(elementary_type_without_payable::N),
    }
    pub struct _S7 {
        pub open_bracket_char: char,
        pub expressions: Box<primary_expression::_S8>,
        pub close_bracket_char: char,
    }
    pub struct _S8 {
        pub expressions: Vec<expression::N>,
        pub comma_chars: Vec<char>,
    }
    pub struct _S4 {
        pub open_paren_char: char,
        pub expressions: Box<primary_expression::_S5>,
        pub close_paren_char: char,
    }
    pub struct _S5 {
        pub expressions: Vec<Option<expression::N>>,
        pub comma_chars: Vec<char>,
    }
    pub struct _S3 {
        pub new: usize,
        pub type_name: type_name::N,
    }
    pub struct _S2 {
        pub r#type: usize,
        pub open_paren_char: char,
        pub type_name: type_name::N,
        pub close_paren_char: char,
    }
    pub struct _S1 {
        pub payable: usize,
        pub argument_list: argument_list::N,
    }
}
#[doc = "ReceiveFunctionAttribute = 'external' | 'payable' | ModifierInvocation | 'virtual' | OverrideSpecifier ;"]
pub mod receive_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<receive_function_attribute::_C0>;
    pub enum _C0 {
        External(usize),
        Payable(usize),
        ModifierInvocation(modifier_invocation::N),
        Virtual(usize),
        OverrideSpecifier(override_specifier::N),
    }
}
#[doc = "StructDefinition = 'struct' «Identifier» '{' 1…*{ TypeName «Identifier» ';' } '}' ;"]
pub mod struct_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<struct_definition::_S0>;
    pub struct _S0 {
        pub r#struct: usize,
        pub identifier: identifier::N,
        pub open_brace_char: char,
        pub _s2s: Vec<Box<struct_definition::_S2>>,
        pub close_brace_char: char,
    }
    pub struct _S2 {
        pub type_name: type_name::N,
        pub identifier: identifier::N,
        pub semicolon_char: char,
    }
}
#[doc = "UsingDirective = 'using' ( IdentifierPath | '{' 1…*{ IdentifierPath / ',' } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;"]
pub mod using_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<using_directive::_S0>;
    pub struct _S0 {
        pub using: usize,
        pub _c1: Box<using_directive::_C1>,
        pub r#for: usize,
        pub _c4: Box<using_directive::_C4>,
        pub global: Option<usize>,
        pub semicolon_char: char,
    }
    pub enum _C4 {
        StarChar(char),
        TypeName(type_name::N),
    }
    pub enum _C1 {
        IdentifierPath(identifier_path::N),
        _S2(Box<using_directive::_S2>),
    }
    pub struct _S2 {
        pub open_brace_char: char,
        pub identifier_paths: Box<using_directive::_S3>,
        pub close_brace_char: char,
    }
    pub struct _S3 {
        pub identifier_paths: Vec<identifier_path::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "VariableDeclaration = TypeName [ DataLocation ] «Identifier» ;"]
pub mod variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<variable_declaration::_S0>;
    pub struct _S0 {
        pub type_name: type_name::N,
        pub _1: Option<usize>,
        pub identifier: identifier::N,
    }
}
#[doc = "YulBlock = '{' { YulStatement } '}' ;"]
pub mod yul_block {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_block::_S0>;
    pub struct _S0 {
        pub open_brace_char: char,
        pub yul_statements: Vec<yul_statement::N>,
        pub close_brace_char: char,
    }
}
#[doc = "AssemblyStatement = 'assembly' [ '\"evmasm\"' ] [ AssemblyFlags ] YulBlock ;"]
pub mod assembly_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<assembly_statement::_S0>;
    pub struct _S0 {
        pub assembly: usize,
        pub double_quote_evmasm_double_quote: Option<usize>,
        pub assembly_flags: Option<assembly_flags::N>,
        pub yul_block: yul_block::N,
    }
}
#[doc = "Directive = «PragmaDirective» | ImportDirective | UsingDirective ;"]
pub mod directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<directive::_C0>;
    pub enum _C0 {
        PragmaDirective(pragma_directive::N),
        ImportDirective(import_directive::N),
        UsingDirective(using_directive::N),
    }
}
#[doc = "ErrorDefinition = 'error' «Identifier» '(' { ErrorParameter / ',' } ')' ';' ;"]
pub mod error_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<error_definition::_S0>;
    pub struct _S0 {
        pub error: usize,
        pub identifier: identifier::N,
        pub open_paren_char: char,
        pub error_parameters: Option<Box<error_definition::_S1>>,
        pub close_paren_char: char,
        pub semicolon_char: char,
    }
    pub struct _S1 {
        pub error_parameters: Vec<error_parameter::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "EventDefinition = 'event' «Identifier» '(' { EventParameter / ',' } ')' [ 'anonymous' ] ';' ;"]
pub mod event_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<event_definition::_S0>;
    pub struct _S0 {
        pub event: usize,
        pub identifier: identifier::N,
        pub open_paren_char: char,
        pub event_parameters: Option<Box<event_definition::_S1>>,
        pub close_paren_char: char,
        pub anonymous: Option<usize>,
        pub semicolon_char: char,
    }
    pub struct _S1 {
        pub event_parameters: Vec<event_parameter::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "IndexAccessExpression = PrimaryExpression { '[' [ Expression ] [ ':' [ Expression ] ] ']' } ;"]
pub mod index_access_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<index_access_expression::_S0>;
    pub struct _S0 {
        pub primary_expression: primary_expression::N,
        pub _s2s: Vec<Box<index_access_expression::_S2>>,
    }
    pub struct _S2 {
        pub open_bracket_char: char,
        pub expression: Option<expression::N>,
        pub _s5: Option<Box<index_access_expression::_S5>>,
        pub close_bracket_char: char,
    }
    pub struct _S5 {
        pub colon_char: char,
        pub expression: Option<expression::N>,
    }
}
#[doc = "VariableDeclarationTuple = '(' { ',' } VariableDeclaration { ',' [ VariableDeclaration ] } ')' ;"]
pub mod variable_declaration_tuple {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<variable_declaration_tuple::_S0>;
    pub struct _S0 {
        pub open_paren_char: char,
        pub comma_chars: Vec<char>,
        pub variable_declaration: variable_declaration::N,
        pub _s3s: Vec<Box<variable_declaration_tuple::_S3>>,
        pub close_paren_char: char,
    }
    pub struct _S3 {
        pub comma_char: char,
        pub variable_declaration: Option<variable_declaration::N>,
    }
}
#[doc = "MemberAccessExpression = IndexAccessExpression { '.' ( «Identifier» | 'address' ) } ;"]
pub mod member_access_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<member_access_expression::_S0>;
    pub struct _S0 {
        pub index_access_expression: index_access_expression::N,
        pub _s2s: Vec<Box<member_access_expression::_S2>>,
    }
    pub struct _S2 {
        pub period_char: char,
        pub _c3: Box<member_access_expression::_C3>,
    }
    pub enum _C3 {
        Identifier(identifier::N),
        Address(usize),
    }
}
#[doc = "FunctionCallOptionsExpression = MemberAccessExpression { '{' 1…*{ NamedArgument / ',' } '}' } ;"]
pub mod function_call_options_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_call_options_expression::_S0>;
    pub struct _S0 {
        pub member_access_expression: member_access_expression::N,
        pub _s2s: Vec<Box<function_call_options_expression::_S2>>,
    }
    pub struct _S2 {
        pub open_brace_char: char,
        pub named_arguments: Box<function_call_options_expression::_S3>,
        pub close_brace_char: char,
    }
    pub struct _S3 {
        pub named_arguments: Vec<named_argument::N>,
        pub comma_chars: Vec<char>,
    }
}
#[doc = "FunctionCallExpression = FunctionCallOptionsExpression { ArgumentList } ;"]
pub mod function_call_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_call_expression::_S0>;
    pub struct _S0 {
        pub function_call_options_expression: function_call_options_expression::N,
        pub argument_lists: Vec<argument_list::N>,
    }
}
#[doc = "UnaryPrefixExpression = UnaryPrefixOperator FunctionCallExpression ;"]
pub mod unary_prefix_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unary_prefix_expression::_S0>;
    pub struct _S0 {
        pub _0: usize,
        pub function_call_expression: function_call_expression::N,
    }
}
#[doc = "UnarySuffixExpression = UnaryPrefixExpression UnarySuffixOperator ;"]
pub mod unary_suffix_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unary_suffix_expression::_S0>;
    pub struct _S0 {
        pub unary_prefix_expression: unary_prefix_expression::N,
        pub _1: usize,
    }
}
#[doc = "ExpExpression = UnarySuffixExpression '**' Expression ;"]
pub mod exp_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<exp_expression::_S0>;
    pub struct _S0 {
        pub unary_suffix_expression: unary_suffix_expression::N,
        pub star_star: usize,
        pub expression: expression::N,
    }
}
#[doc = "MulDivModExpression = ExpExpression { MulDivModOperator ExpExpression } ;"]
pub mod mul_div_mod_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<mul_div_mod_expression::_S0>;
    pub struct _S0 {
        pub exp_expression: exp_expression::N,
        pub _s2s: Vec<Box<mul_div_mod_expression::_S2>>,
    }
    pub struct _S2 {
        pub _0: char,
        pub exp_expression: exp_expression::N,
    }
}
#[doc = "AddSubExpression = MulDivModExpression { AddSubOperator MulDivModExpression } ;"]
pub mod add_sub_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<add_sub_expression::_S0>;
    pub struct _S0 {
        pub mul_div_mod_expression: mul_div_mod_expression::N,
        pub _s2s: Vec<Box<add_sub_expression::_S2>>,
    }
    pub struct _S2 {
        pub _0: char,
        pub mul_div_mod_expression: mul_div_mod_expression::N,
    }
}
#[doc = "ShiftExpression = AddSubExpression { ShiftOperator AddSubExpression } ;"]
pub mod shift_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<shift_expression::_S0>;
    pub struct _S0 {
        pub add_sub_expression: add_sub_expression::N,
        pub _s2s: Vec<Box<shift_expression::_S2>>,
    }
    pub struct _S2 {
        pub _0: usize,
        pub add_sub_expression: add_sub_expression::N,
    }
}
#[doc = "BitAndExpression = ShiftExpression { '&' ShiftExpression } ;"]
pub mod bit_and_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<bit_and_expression::_S0>;
    pub struct _S0 {
        pub shift_expression: shift_expression::N,
        pub _s2s: Vec<Box<bit_and_expression::_S2>>,
    }
    pub struct _S2 {
        pub ampersand_char: char,
        pub shift_expression: shift_expression::N,
    }
}
#[doc = "BitXOrExpression = BitAndExpression { '^' BitAndExpression } ;"]
pub mod bit_x_or_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<bit_x_or_expression::_S0>;
    pub struct _S0 {
        pub bit_and_expression: bit_and_expression::N,
        pub _s2s: Vec<Box<bit_x_or_expression::_S2>>,
    }
    pub struct _S2 {
        pub caret_char: char,
        pub bit_and_expression: bit_and_expression::N,
    }
}
#[doc = "BitOrExpression = BitXOrExpression { '|' BitXOrExpression } ;"]
pub mod bit_or_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<bit_or_expression::_S0>;
    pub struct _S0 {
        pub bit_x_or_expression: bit_x_or_expression::N,
        pub _s2s: Vec<Box<bit_or_expression::_S2>>,
    }
    pub struct _S2 {
        pub bar_char: char,
        pub bit_x_or_expression: bit_x_or_expression::N,
    }
}
#[doc = "OrderComparisonExpression = BitOrExpression { OrderComparisonOperator BitOrExpression } ;"]
pub mod order_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<order_comparison_expression::_S0>;
    pub struct _S0 {
        pub bit_or_expression: bit_or_expression::N,
        pub _s2s: Vec<Box<order_comparison_expression::_S2>>,
    }
    pub struct _S2 {
        pub _0: usize,
        pub bit_or_expression: bit_or_expression::N,
    }
}
#[doc = "EqualityComparisonExpression = OrderComparisonExpression { EqualityComparisonOperator OrderComparisonExpression } ;"]
pub mod equality_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<equality_comparison_expression::_S0>;
    pub struct _S0 {
        pub order_comparison_expression: order_comparison_expression::N,
        pub _s2s: Vec<Box<equality_comparison_expression::_S2>>,
    }
    pub struct _S2 {
        pub _0: usize,
        pub order_comparison_expression: order_comparison_expression::N,
    }
}
#[doc = "AndExpression = EqualityComparisonExpression { '&&' EqualityComparisonExpression } ;"]
pub mod and_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<and_expression::_S0>;
    pub struct _S0 {
        pub equality_comparison_expression: equality_comparison_expression::N,
        pub _s2s: Vec<Box<and_expression::_S2>>,
    }
    pub struct _S2 {
        pub ampersand_ampersand: usize,
        pub equality_comparison_expression: equality_comparison_expression::N,
    }
}
#[doc = "OrExpression = AndExpression { '||' AndExpression } ;"]
pub mod or_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<or_expression::_S0>;
    pub struct _S0 {
        pub and_expression: and_expression::N,
        pub _s2s: Vec<Box<or_expression::_S2>>,
    }
    pub struct _S2 {
        pub bar_bar: usize,
        pub and_expression: and_expression::N,
    }
}
#[doc = "ConditionalExpression = OrExpression '?' Expression ':' Expression ;"]
pub mod conditional_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<conditional_expression::_S0>;
    pub struct _S0 {
        pub or_expression: or_expression::N,
        pub question_char: char,
        pub expression_0: expression::N,
        pub colon_char: char,
        pub expression_1: expression::N,
    }
}
#[doc = "AssignmentExpression = ConditionalExpression AssignmentOperator Expression ;"]
pub mod assignment_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<assignment_expression::_S0>;
    pub struct _S0 {
        pub conditional_expression: conditional_expression::N,
        pub _1: usize,
        pub expression: expression::N,
    }
}
#[doc = "Expression = AssignmentExpression ;"]
pub mod expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = assignment_expression::N;
}
#[doc = "ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;"]
pub mod constant_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<constant_definition::_S0>;
    pub struct _S0 {
        pub type_name: type_name::N,
        pub constant: usize,
        pub identifier: identifier::N,
        pub equal_char: char,
        pub expression: expression::N,
        pub semicolon_char: char,
    }
}
#[doc = "DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;"]
pub mod do_while_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<do_while_statement::_S0>;
    pub struct _S0 {
        pub r#do: usize,
        pub statement: statement::N,
        pub r#while: usize,
        pub open_paren_char: char,
        pub expression: expression::N,
        pub close_paren_char: char,
        pub semicolon_char: char,
    }
}
#[doc = "EmitStatement = 'emit' Expression ArgumentList ';' ;"]
pub mod emit_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<emit_statement::_S0>;
    pub struct _S0 {
        pub emit: usize,
        pub expression: expression::N,
        pub argument_list: argument_list::N,
        pub semicolon_char: char,
    }
}
#[doc = "ExpressionStatement = Expression ';' ;"]
pub mod expression_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<expression_statement::_S0>;
    pub struct _S0 {
        pub expression: expression::N,
        pub semicolon_char: char,
    }
}
#[doc = "IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;"]
pub mod if_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<if_statement::_S0>;
    pub struct _S0 {
        pub r#if: usize,
        pub open_paren_char: char,
        pub expression: expression::N,
        pub close_paren_char: char,
        pub statement: statement::N,
        pub _s2: Option<Box<if_statement::_S2>>,
    }
    pub struct _S2 {
        pub r#else: usize,
        pub statement: statement::N,
    }
}
#[doc = "ReturnStatement = 'return' [ Expression ] ';' ;"]
pub mod return_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<return_statement::_S0>;
    pub struct _S0 {
        pub r#return: usize,
        pub expression: Option<expression::N>,
        pub semicolon_char: char,
    }
}
#[doc = "RevertStatement = 'revert' Expression ArgumentList ';' ;"]
pub mod revert_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<revert_statement::_S0>;
    pub struct _S0 {
        pub revert: usize,
        pub expression: expression::N,
        pub argument_list: argument_list::N,
        pub semicolon_char: char,
    }
}
#[doc = "StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;"]
pub mod state_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<state_variable_declaration::_S0>;
    pub struct _S0 {
        pub type_name: type_name::N,
        pub state_variable_attributes: Vec<state_variable_attribute::N>,
        pub identifier: identifier::N,
        pub _s3: Option<Box<state_variable_declaration::_S3>>,
        pub semicolon_char: char,
    }
    pub struct _S3 {
        pub equal_char: char,
        pub expression: expression::N,
    }
}
#[doc = "TryStatement = 'try' Expression [ 'returns' NonEmptyParameterList ] Block CatchClause { CatchClause } ;"]
pub mod try_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<try_statement::_S0>;
    pub struct _S0 {
        pub r#try: usize,
        pub expression: expression::N,
        pub _s2: Option<Box<try_statement::_S2>>,
        pub block: block::N,
        pub catch_clause: catch_clause::N,
        pub catch_clauses: Vec<catch_clause::N>,
    }
    pub struct _S2 {
        pub returns: usize,
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}
#[doc = "VariableDeclarationStatement = ( VariableDeclaration [ '=' Expression ] | VariableDeclarationTuple '=' Expression ) ';' ;"]
pub mod variable_declaration_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<variable_declaration_statement::_S0>;
    pub struct _S0 {
        pub _c1: Box<variable_declaration_statement::_C1>,
        pub semicolon_char: char,
    }
    pub enum _C1 {
        _S2(Box<variable_declaration_statement::_S2>),
        _S5(Box<variable_declaration_statement::_S5>),
    }
    pub struct _S5 {
        pub variable_declaration_tuple: variable_declaration_tuple::N,
        pub equal_char: char,
        pub expression: expression::N,
    }
    pub struct _S2 {
        pub variable_declaration: variable_declaration::N,
        pub _s4: Option<Box<variable_declaration_statement::_S4>>,
    }
    pub struct _S4 {
        pub equal_char: char,
        pub expression: expression::N,
    }
}
#[doc = "WhileStatement = 'while' '(' Expression ')' Statement ;"]
pub mod while_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<while_statement::_S0>;
    pub struct _S0 {
        pub r#while: usize,
        pub open_paren_char: char,
        pub expression: expression::N,
        pub close_paren_char: char,
        pub statement: statement::N,
    }
}
#[doc = "SimpleStatement = VariableDeclarationStatement | ExpressionStatement ;"]
pub mod simple_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<simple_statement::_C0>;
    pub enum _C0 {
        VariableDeclarationStatement(variable_declaration_statement::N),
        ExpressionStatement(expression_statement::N),
    }
}
#[doc = "ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;"]
pub mod for_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<for_statement::_S0>;
    pub struct _S0 {
        pub r#for: usize,
        pub open_paren_char: char,
        pub _c1: Box<for_statement::_C1>,
        pub _c2: Box<for_statement::_C2>,
        pub expression: Option<expression::N>,
        pub close_paren_char: char,
        pub statement: statement::N,
    }
    pub enum _C2 {
        ExpressionStatement(expression_statement::N),
        SemicolonChar(char),
    }
    pub enum _C1 {
        SimpleStatement(simple_statement::N),
        SemicolonChar(char),
    }
}
#[doc = "Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | AssemblyStatement ;"]
pub mod statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<statement::_C0>;
    pub enum _C0 {
        Block(block::N),
        SimpleStatement(simple_statement::N),
        IfStatement(if_statement::N),
        ForStatement(for_statement::N),
        WhileStatement(while_statement::N),
        DoWhileStatement(do_while_statement::N),
        ContinueStatement(continue_statement::N),
        BreakStatement(break_statement::N),
        TryStatement(try_statement::N),
        ReturnStatement(return_statement::N),
        EmitStatement(emit_statement::N),
        RevertStatement(revert_statement::N),
        AssemblyStatement(assembly_statement::N),
    }
}
#[doc = "Block = '{' { Statement | UncheckedBlock } '}' ;"]
pub mod block {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<block::_S0>;
    pub struct _S0 {
        pub open_brace_char: char,
        pub _c2s: Vec<Box<block::_C2>>,
        pub close_brace_char: char,
    }
    pub enum _C2 {
        Statement(statement::N),
        UncheckedBlock(unchecked_block::N),
    }
}
#[doc = "ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;"]
pub mod constructor_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<constructor_definition::_S0>;
    pub struct _S0 {
        pub constructor: usize,
        pub parameter_list: parameter_list::N,
        pub constructor_attributes: Vec<constructor_attribute::N>,
        pub block: block::N,
    }
}
#[doc = "FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;"]
pub mod fallback_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fallback_function_definition::_S0>;
    pub struct _S0 {
        pub fallback: usize,
        pub parameter_list: parameter_list::N,
        pub fallback_function_attributes: Vec<fallback_function_attribute::N>,
        pub _s3: Option<Box<fallback_function_definition::_S3>>,
        pub _c4: Box<fallback_function_definition::_C4>,
    }
    pub enum _C4 {
        SemicolonChar(char),
        Block(block::N),
    }
    pub struct _S3 {
        pub returns: usize,
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}
#[doc = "FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;"]
pub mod function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_definition::_S0>;
    pub struct _S0 {
        pub function: usize,
        pub _c1: Box<function_definition::_C1>,
        pub parameter_list: parameter_list::N,
        pub function_attributes: Vec<function_attribute::N>,
        pub _s4: Option<Box<function_definition::_S4>>,
        pub _c5: Box<function_definition::_C5>,
    }
    pub enum _C5 {
        SemicolonChar(char),
        Block(block::N),
    }
    pub struct _S4 {
        pub returns: usize,
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
    pub enum _C1 {
        Identifier(identifier::N),
        Fallback(usize),
        Receive(usize),
    }
}
#[doc = "ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { MethodAttribute } ( ';' | Block ) ;"]
pub mod modifier_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<modifier_definition::_S0>;
    pub struct _S0 {
        pub modifier: usize,
        pub identifier: identifier::N,
        pub parameter_list: Option<parameter_list::N>,
        pub method_attributes: Vec<method_attribute::N>,
        pub _c3: Box<modifier_definition::_C3>,
    }
    pub enum _C3 {
        SemicolonChar(char),
        Block(block::N),
    }
}
#[doc = "ReceiveFunctionDefinition = 'receive' '(' ')' { ReceiveFunctionAttribute } ( ';' | Block ) ;"]
pub mod receive_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<receive_function_definition::_S0>;
    pub struct _S0 {
        pub receive: usize,
        pub open_paren_char: char,
        pub close_paren_char: char,
        pub receive_function_attributes: Vec<receive_function_attribute::N>,
        pub _c2: Box<receive_function_definition::_C2>,
    }
    pub enum _C2 {
        SemicolonChar(char),
        Block(block::N),
    }
}
#[doc = "ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;"]
pub mod contract_body_element {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<contract_body_element::_C0>;
    pub enum _C0 {
        UsingDirective(using_directive::N),
        ConstructorDefinition(constructor_definition::N),
        FunctionDefinition(function_definition::N),
        FallbackFunctionDefinition(fallback_function_definition::N),
        ReceiveFunctionDefinition(receive_function_definition::N),
        ModifierDefinition(modifier_definition::N),
        StructDefinition(struct_definition::N),
        EnumDefinition(enum_definition::N),
        UserDefinedValueTypeDefinition(user_defined_value_type_definition::N),
        EventDefinition(event_definition::N),
        ErrorDefinition(error_definition::N),
        StateVariableDeclaration(state_variable_declaration::N),
    }
}
#[doc = "ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;"]
pub mod contract_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<contract_definition::_S0>;
    pub struct _S0 {
        pub r#abstract: Option<usize>,
        pub contract: usize,
        pub identifier: identifier::N,
        pub inheritance_specifier_list: Option<inheritance_specifier_list::N>,
        pub open_brace_char: char,
        pub contract_body_elements: Vec<contract_body_element::N>,
        pub close_brace_char: char,
    }
}
#[doc = "InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;"]
pub mod interface_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<interface_definition::_S0>;
    pub struct _S0 {
        pub interface: usize,
        pub identifier: identifier::N,
        pub inheritance_specifier_list: Option<inheritance_specifier_list::N>,
        pub open_brace_char: char,
        pub contract_body_elements: Vec<contract_body_element::N>,
        pub close_brace_char: char,
    }
}
#[doc = "LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;"]
pub mod library_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<library_definition::_S0>;
    pub struct _S0 {
        pub library: usize,
        pub identifier: identifier::N,
        pub open_brace_char: char,
        pub contract_body_elements: Vec<contract_body_element::N>,
        pub close_brace_char: char,
    }
}
#[doc = "Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;"]
pub mod definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<definition::_C0>;
    pub enum _C0 {
        ContractDefinition(contract_definition::N),
        InterfaceDefinition(interface_definition::N),
        LibraryDefinition(library_definition::N),
        FunctionDefinition(function_definition::N),
        ConstantDefinition(constant_definition::N),
        StructDefinition(struct_definition::N),
        EnumDefinition(enum_definition::N),
        UserDefinedValueTypeDefinition(user_defined_value_type_definition::N),
        ErrorDefinition(error_definition::N),
    }
}
#[doc = "SourceUnit = «IGNORE» { Directive | Definition } $ ;"]
pub mod source_unit {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<source_unit::_S0>;
    pub struct _S0 {
        pub ignore: ignore::N,
        pub _c2s: Vec<Box<source_unit::_C2>>,
        pub end_marker: (),
    }
    pub enum _C2 {
        Directive(directive::N),
        Definition(definition::N),
    }
}
