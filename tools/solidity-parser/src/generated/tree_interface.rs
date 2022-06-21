#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
pub trait DefaultTest {
    fn is_default(&self) -> bool {
        false
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FixedTerminal<const N: usize>();

/// «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
pub mod ascii_escape {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// «BooleanLiteral» = 'true' | 'false' ;
pub mod boolean_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
pub mod comment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<comment::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub slash_star: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2s: Vec<Box<comment::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_chars: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_slash: FixedTerminal<2usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T2 {
        NotStarChar(FixedTerminal<1usize>),
        _T3(Box<comment::_T3>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_chars: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: FixedTerminal<1usize>,
    }
}

/// «DecimalInteger» = 1…*{ '0'…'9' / [ '_' ] } ;
pub mod decimal_integer {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_integer::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: Vec<FixedTerminal<1usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub underscore_chars: Vec<Option<FixedTerminal<1usize>>>,
    }
}

/// «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
pub mod fixed_bytes_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fixed_bytes_type::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub bytes: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// «FixedType» = 'fixed' [ '1'…'9' { '0'…'9' } 'x' '1'…'9' { '0'…'9' } ] ;
pub mod fixed_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fixed_type::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub fixed: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<fixed_type::_T1>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _2: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _3: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _4: usize,
    }
}

/// «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
pub mod hex_character {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
pub mod identifier_start {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// «LineComment» = '//' { ¬( '\u{a}' | '\u{d}' ) } ;
pub mod line_comment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<line_comment::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub slash_slash: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// «NumberUnit» = 'wei' | 'gwei' | 'ether' | 'seconds' | 'minutes' | 'hours' | 'days' | 'weeks' | 'years' ;
pub mod number_unit {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «PragmaDirective» = 'pragma' 1…*{ ¬';' } ';' ;
pub mod pragma_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<pragma_directive::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub pragma: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub not_semicolon_chars: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
pub mod reserved_keyword {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «SignedIntegerType» = 'int' ( '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ) ;
pub mod signed_integer_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<signed_integer_type::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub int: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// «Whitespace» = 1…*{ '\u{20}' | '\u{9}' | '\u{d}' | '\u{a}' } ;
pub mod whitespace {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
pub mod yul_decimal_number_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_decimal_number_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        ZeroChar(FixedTerminal<1usize>),
        _T1(Box<yul_decimal_number_literal::_T1>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// «YulEVMBuiltinFunctionName» = 'stop' | 'add' | 'sub' | 'mul' | 'div' | 'sdiv' | 'mod' | 'smod' | 'exp' | 'not' | 'lt' | 'gt' | 'slt' | 'sgt' | 'eq' | 'iszero' | 'and' | 'or' | 'xor' | 'byte' | 'shl' | 'shr' | 'sar' | 'addmod' | 'mulmod' | 'signextend' | 'keccak256' | 'pop' | 'mload' | 'mstore' | 'mstore8' | 'sload' | 'sstore' | 'msize' | 'gas' | 'address' | 'balance' | 'selfbalance' | 'caller' | 'callvalue' | 'calldataload' | 'calldatasize' | 'calldatacopy' | 'extcodesize' | 'extcodecopy' | 'returndatasize' | 'returndatacopy' | 'extcodehash' | 'create' | 'create2' | 'call' | 'callcode' | 'delegatecall' | 'staticcall' | 'return' | 'revert' | 'selfdestruct' | 'invalid' | 'log0' | 'log1' | 'log2' | 'log3' | 'log4' | 'chainid' | 'origin' | 'gasprice' | 'Blockhash' | 'coinbase' | 'timestamp' | 'number' | 'difficulty' | 'gaslimit' | 'basefee' ;
pub mod yul_evm_builtin_function_name {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «YulHexLiteral» = '0x' 1…*{ '0'…'9' | 'a'…'f' | 'A'…'F' } ;
pub mod yul_hex_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_hex_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub zero_x: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// «YulKeyword» = 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'if' | 'leave' | 'let' | 'switch' | 'hex' ;
pub mod yul_keyword {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
pub mod decimal_exponent {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_exponent::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub minus_char: Option<FixedTerminal<1usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_integer: decimal_integer::N,
    }
}

/// «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
pub mod decimal_float {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_float::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_integer_1: Option<decimal_integer::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_integer_2: decimal_integer::N,
    }
}

/// «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
pub mod hex_byte_escape {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<hex_byte_escape::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// «HexNumber» = '0' 'x' 1…*{ «HexCharacter» / [ '_' ] } ;
pub mod hex_number {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<hex_number::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub zero_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _2: Box<hex_number::_T1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: Vec<FixedTerminal<1usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub underscore_chars: Vec<Option<FixedTerminal<1usize>>>,
    }
}

/// «IGNORE» = { «Whitespace» | «Comment» | «LineComment» } ;
pub mod ignore {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<Box<ignore::_T1>>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        Whitespace(whitespace::N),
        Comment(comment::N),
        LineComment(line_comment::N),
    }
}

/// «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
pub mod identifier_part {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// «PossiblySeparatedPairsOfHexDigits» = 1…*{ 2…2*{ «HexCharacter» } / [ '_' ] } ;
pub mod possibly_separated_pairs_of_hex_digits {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<possibly_separated_pairs_of_hex_digits::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: Vec<usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub underscore_chars: Vec<Option<FixedTerminal<1usize>>>,
    }
}

/// «UfixedType» = 'u' «FixedType» ;
pub mod ufixed_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<ufixed_type::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub fixed_type: fixed_type::N,
    }
}

/// «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
pub mod unicode_escape {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unicode_escape::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
pub mod unsigned_integer_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unsigned_integer_type::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub signed_integer_type: signed_integer_type::N,
    }
}

/// «YulReservedWord» = «YulKeyword» | «YulEVMBuiltinFunctionName» | «BooleanLiteral» ;
pub mod yul_reserved_word {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// AddSubOperator = '+' | '-' ;
pub mod add_sub_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// AssignmentOperator = '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ;
pub mod assignment_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// BreakStatement = 'break' ';' ;
pub mod break_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<break_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#break: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// ContinueStatement = 'continue' ';' ;
pub mod continue_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<continue_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#continue: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// DataLocation = 'memory' | 'storage' | 'calldata' ;
pub mod data_location {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
pub mod decimal_number {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_number::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<decimal_number::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_exponent: Option<decimal_exponent::N>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        DecimalInteger(decimal_integer::N),
        DecimalFloat(decimal_float::N),
    }
}

/// ElementaryType = 'bool' | 'string' | 'bytes' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | «FixedType» | «UfixedType» ;
pub mod elementary_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<elementary_type::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        _0(usize),
        SignedIntegerType(signed_integer_type::N),
        UnsignedIntegerType(unsigned_integer_type::N),
        FixedBytesType(fixed_bytes_type::N),
        FixedType(fixed_type::N),
        UfixedType(ufixed_type::N),
    }
}

/// EqualityComparisonOperator = '==' | '!=' ;
pub mod equality_comparison_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<2usize>;
}

/// «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
pub mod escape_sequence {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<escape_sequence::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub backslash_char: FixedTerminal<1usize>,
        pub _t1: Box<escape_sequence::_T1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        _0(FixedTerminal<1usize>),
        HexByteEscape(hex_byte_escape::N),
        UnicodeEscape(unicode_escape::N),
    }
}

/// «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
pub mod hex_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<hex_string_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub hex: FixedTerminal<3usize>,
        pub _t1: Box<hex_string_literal::_T1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(Box<hex_string_literal::_T2>),
        _T3(Box<hex_string_literal::_T3>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_1: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub possibly_separated_pairs_of_hex_digits:
            Option<possibly_separated_pairs_of_hex_digits::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_2: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_char_1: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub possibly_separated_pairs_of_hex_digits:
            Option<possibly_separated_pairs_of_hex_digits::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_char_2: FixedTerminal<1usize>,
    }
}

/// «Keyword» = 'pragma' | 'abstract' | 'anonymous' | 'address' | 'as' | 'assembly' | 'bool' | 'break' | 'bytes' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | 'fixed' | 'ufixed' ;
pub mod keyword {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<keyword::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        _0(usize),
        SignedIntegerType(signed_integer_type::N),
        UnsignedIntegerType(unsigned_integer_type::N),
        FixedBytesType(fixed_bytes_type::N),
        _4(usize),
    }
}

/// MulDivModOperator = '*' | '/' | '%' ;
pub mod mul_div_mod_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// OrderComparisonOperator = '<' | '>' | '<=' | '>=' ;
pub mod order_comparison_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// PositionalArgumentList = 1…*{ Expression / ',' } ;
pub mod positional_argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<positional_argument_list::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
pub mod raw_identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<raw_identifier::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// ShiftOperator = '<<' | '>>' | '>>>' ;
pub mod shift_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// StateMutabilitySpecifier = 'pure' | 'view' | 'payable' ;
pub mod state_mutability_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// UnaryPrefixOperator = '++' | '--' | '!' | '~' | 'delete' | '-' ;
pub mod unary_prefix_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// UnarySuffixOperator = '++' | '--' ;
pub mod unary_suffix_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<2usize>;
}

/// UncheckedBlock = 'unchecked' Block ;
pub mod unchecked_block {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unchecked_block::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub unchecked: FixedTerminal<9usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: block::N,
    }
}

/// VisibilitySpecifier = 'internal' | 'external' | 'private' | 'public' ;
pub mod visibility_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// YulBreakStatement = 'break' ;
pub mod yul_break_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<5usize>;
}

/// YulContinueStatement = 'continue' ;
pub mod yul_continue_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<8usize>;
}

/// YulLeaveStatement = 'leave' ;
pub mod yul_leave_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<5usize>;
}

/// «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
pub mod double_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<double_quoted_ascii_string_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_char_1: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub runs: Vec<Box<double_quoted_ascii_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_char_2: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Run {
        Chars(usize),
        EscapeSequence(escape_sequence::N),
    }
}

/// «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
pub mod double_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<double_quoted_unicode_string_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub unicode_double_quote: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub runs: Vec<Box<double_quoted_unicode_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Run {
        Chars(usize),
        EscapeSequence(escape_sequence::N),
    }
}

/// ElementaryTypeWithPayable = 'address' [ 'payable' ] | ElementaryType ;
pub mod elementary_type_with_payable {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<elementary_type_with_payable::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        _T1(Box<elementary_type_with_payable::_T1>),
        ElementaryType(elementary_type::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub address: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub payable: Option<FixedTerminal<7usize>>,
    }
}

/// ElementaryTypeWithoutPayable = 'address' | ElementaryType ;
pub mod elementary_type_without_payable {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<elementary_type_without_payable::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        Address(FixedTerminal<7usize>),
        ElementaryType(elementary_type::N),
    }
}

/// NumericLiteral = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
pub mod numeric_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<numeric_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<numeric_literal::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _2: Option<usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        DecimalNumber(decimal_number::N),
        HexNumber(hex_number::N),
    }
}

/// «ReservedWord» = «Keyword» | «ReservedKeyword» | «NumberUnit» | «BooleanLiteral» ;
pub mod reserved_word {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<reserved_word::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        Keyword(keyword::N),
        _1(usize),
    }
}

/// «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
pub mod single_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<single_quoted_ascii_string_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_1: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub runs: Vec<Box<single_quoted_ascii_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_2: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Run {
        Chars(usize),
        EscapeSequence(escape_sequence::N),
    }
}

/// «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
pub mod single_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<single_quoted_unicode_string_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub unicode_quote: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub runs: Vec<Box<single_quoted_unicode_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Run {
        Chars(usize),
        EscapeSequence(escape_sequence::N),
    }
}

/// «YulIdentifier» = «RawIdentifier» - «YulReservedWord» ;
pub mod yul_identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = raw_identifier::N;
}

/// «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
pub mod ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<ascii_string_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        SingleQuotedAsciiStringLiteral(single_quoted_ascii_string_literal::N),
        DoubleQuotedAsciiStringLiteral(double_quoted_ascii_string_literal::N),
    }
}

/// AssemblyFlags = '(' 1…*{ «DoubleQuotedAsciiStringLiteral» / ',' } ')' ;
pub mod assembly_flags {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<assembly_flags::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quoted_ascii_string_literals: Box<assembly_flags::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quoted_ascii_string_literals: Vec<double_quoted_ascii_string_literal::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// «Identifier» = «RawIdentifier» - «ReservedWord» ;
pub mod identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = raw_identifier::N;
}

/// «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
pub mod unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unicode_string_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        SingleQuotedUnicodeStringLiteral(single_quoted_unicode_string_literal::N),
        DoubleQuotedUnicodeStringLiteral(double_quoted_unicode_string_literal::N),
    }
}

/// YulFunctionCall = ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) '(' { YulExpression / ',' } ')' ;
pub mod yul_function_call {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_function_call::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<yul_function_call::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_expressions: Option<Box<yul_function_call::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_expressions: Vec<yul_expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        YulIdentifier(yul_identifier::N),
        _1(usize),
    }
}

/// YulFunctionDefinition = 'function' «YulIdentifier» '(' { «YulIdentifier» / ',' } ')' [ '->' 1…*{ «YulIdentifier» / ',' } ] YulBlock ;
pub mod yul_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_function_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifiers: Option<Box<yul_function_definition::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2: Option<Box<yul_function_definition::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_6: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub minus_greater: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifiers: Box<yul_function_definition::_T3>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifiers: Vec<yul_identifier::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifiers: Vec<yul_identifier::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// YulPath = «YulIdentifier» { '.' ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) } ;
pub mod yul_path {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_path::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2s: Vec<Box<yul_path::_T2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_char: FixedTerminal<1usize>,
        pub _t3: Box<yul_path::_T3>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T3 {
        YulIdentifier(yul_identifier::N),
        _1(usize),
    }
}

/// EnumDefinition = 'enum' «Identifier» '{' 1…*{ «Identifier» / ',' } '}' ;
pub mod enum_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<enum_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#enum: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifiers: Box<enum_definition::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifiers: Vec<identifier::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// IdentifierPath = 1…*{ «Identifier» / '.' } ;
pub mod identifier_path {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<identifier_path::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifiers: Vec<identifier::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// ImportPath = «AsciiStringLiteral» ;
pub mod import_path {
    #[allow(unused_imports)]
    use super::*;
    pub type N = ascii_string_literal::N;
}

/// Literal = «AsciiStringLiteral» | «UnicodeStringLiteral» | NumericLiteral | «HexStringLiteral» | «BooleanLiteral» ;
pub mod literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        AsciiStringLiteral(ascii_string_literal::N),
        UnicodeStringLiteral(unicode_string_literal::N),
        NumericLiteral(numeric_literal::N),
        HexStringLiteral(hex_string_literal::N),
        _4(usize),
    }
}

/// NamedArgument = «Identifier» ':' Expression ;
pub mod named_argument {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<named_argument::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub expression: expression::N,
    }
}

/// ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
pub mod parameter_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<parameter_declaration::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _2: Option<usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::N>,
    }
}

/// SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
pub mod selected_import {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<selected_import::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<selected_import::_T1>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#as: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
    }
}

/// UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryTypeWithPayable ';' ;
pub mod user_defined_value_type_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<user_defined_value_type_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#type: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub is: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub elementary_type_with_payable: elementary_type_with_payable::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
pub mod yul_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_literal::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        YulDecimalNumberLiteral(yul_decimal_number_literal::N),
        YulHexLiteral(yul_hex_literal::N),
        AsciiStringLiteral(ascii_string_literal::N),
        _3(usize),
        HexStringLiteral(hex_string_literal::N),
    }
}

/// MappingType = 'mapping' '(' ( ElementaryTypeWithoutPayable | IdentifierPath ) '=>' TypeName ')' ;
pub mod mapping_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<mapping_type::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub mapping: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub _t1: Box<mapping_type::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_greater: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        ElementaryTypeWithoutPayable(elementary_type_without_payable::N),
        IdentifierPath(identifier_path::N),
    }
}

/// NamedArgumentList = '{' { NamedArgument / ',' } '}' ;
pub mod named_argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<named_argument_list::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub named_arguments: Option<Box<named_argument_list::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub named_arguments: Vec<named_argument::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// NonEmptyParameterList = '(' 1…*{ ParameterDeclaration / ',' } ')' ;
pub mod non_empty_parameter_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<non_empty_parameter_list::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_declarations: Box<non_empty_parameter_list::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_declarations: Vec<parameter_declaration::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// OverrideSpecifier = 'override' [ '(' 1…1*{ IdentifierPath / ',' } ')' ] ;
pub mod override_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<override_specifier::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#override: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<override_specifier::_T1>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_paths: Box<override_specifier::_T2>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_paths: Vec<identifier_path::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// ParameterList = '(' { ParameterDeclaration / ',' } ')' ;
pub mod parameter_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<parameter_list::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_declarations: Option<Box<parameter_list::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_declarations: Vec<parameter_declaration::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// SelectingImportDirective = '{' 1…*{ SelectedImport / ',' } '}' 'from' ImportPath ;
pub mod selecting_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<selecting_import_directive::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub selected_imports: Box<selecting_import_directive::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub from: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub import_path: import_path::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub selected_imports: Vec<selected_import::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
pub mod simple_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<simple_import_directive::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub import_path: import_path::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2s: Vec<Box<simple_import_directive::_T2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#as: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
    }
}

/// StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
pub mod star_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<star_import_directive::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#as: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub from: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub import_path: import_path::N,
    }
}

/// YulExpression = YulPath | YulFunctionCall | YulLiteral ;
pub mod yul_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_expression::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        YulPath(yul_path::N),
        YulFunctionCall(yul_function_call::N),
        YulLiteral(yul_literal::N),
    }
}

/// ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
pub mod argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<argument_list::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<argument_list::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        PositionalArgumentList(positional_argument_list::N),
        NamedArgumentList(named_argument_list::N),
    }
}

/// CatchClause = 'catch' [ [ «Identifier» ] NonEmptyParameterList ] Block ;
pub mod catch_clause {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<catch_clause::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub catch: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<catch_clause::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: block::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}

/// FunctionType = 'function' ParameterList { VisibilitySpecifier | StateMutabilitySpecifier } [ 'returns' NonEmptyParameterList ] ;
pub mod function_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_type::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: parameter_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _4: Vec<usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2: Option<Box<function_type::_T2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}

/// ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
pub mod import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<import_directive::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub import: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub _t1: Box<import_directive::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        SimpleImportDirective(simple_import_directive::N),
        StarImportDirective(star_import_directive::N),
        SelectingImportDirective(selecting_import_directive::N),
    }
}

/// MethodAttribute = 'virtual' | OverrideSpecifier ;
pub mod method_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<method_attribute::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        Virtual(FixedTerminal<7usize>),
        OverrideSpecifier(override_specifier::N),
    }
}

/// StateVariableAttribute = 'public' | 'private' | 'internal' | 'constant' | OverrideSpecifier | 'immutable' ;
pub mod state_variable_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<state_variable_attribute::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        _0(usize),
        OverrideSpecifier(override_specifier::N),
        Immutable(FixedTerminal<9usize>),
    }
}

/// YulAssignment = YulPath ( ':=' YulExpression | 1…*{ ',' YulPath } ':=' YulFunctionCall ) ;
pub mod yul_assignment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_assignment::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_path: yul_path::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub _t1: Box<yul_assignment::_T1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(Box<yul_assignment::_T2>),
        _T3(Box<yul_assignment::_T3>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t5s: Vec<Box<yul_assignment::_T5>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedTerminal<2usize>,
        pub yul_function_call: yul_function_call::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_path: yul_path::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedTerminal<2usize>,
        pub yul_expression: yul_expression::N,
    }
}

/// YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
pub mod yul_for_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_for_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#for: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block_1: yul_block::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub yul_expression: yul_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block_2: yul_block::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block_3: yul_block::N,
    }
}

/// YulIfStatement = 'if' YulExpression YulBlock ;
pub mod yul_if_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_if_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#if: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub yul_expression: yul_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
}

/// YulSwitchStatement = 'switch' YulExpression ( 1…*{ 'case' YulLiteral YulBlock } [ 'default' YulBlock ] | 'default' YulBlock ) ;
pub mod yul_switch_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_switch_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub switch: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub yul_expression: yul_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub _t1: Box<yul_switch_statement::_T1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(Box<yul_switch_statement::_T2>),
        _T6(Box<yul_switch_statement::_T6>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T6 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub default: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t4s: Vec<Box<yul_switch_statement::_T4>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t5: Option<Box<yul_switch_statement::_T5>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub default: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub case: FixedTerminal<4usize>,
        pub yul_literal: yul_literal::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
}

/// YulVariableDeclaration = 'let' «YulIdentifier» [ ':=' YulExpression | [ ',' «YulIdentifier» ] [ ':=' YulFunctionCall ] ] ;
pub mod yul_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_variable_declaration::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#let: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<yul_variable_declaration::_T1>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(Box<yul_variable_declaration::_T2>),
        _T3(Box<yul_variable_declaration::_T3>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t4: Option<Box<yul_variable_declaration::_T4>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t5: Option<Box<yul_variable_declaration::_T5>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedTerminal<2usize>,
        pub yul_function_call: yul_function_call::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedTerminal<2usize>,
        pub yul_expression: yul_expression::N,
    }
}

/// InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
pub mod inheritance_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<inheritance_specifier::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path: identifier_path::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: Option<argument_list::N>,
    }
}

/// ModifierInvocation = IdentifierPath [ ArgumentList ] ;
pub mod modifier_invocation {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<modifier_invocation::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path: identifier_path::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: Option<argument_list::N>,
    }
}

/// TypeName = ( ElementaryTypeWithPayable | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
pub mod type_name {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<type_name::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<type_name::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t3s: Vec<Box<type_name::_T3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        ElementaryTypeWithPayable(elementary_type_with_payable::N),
        FunctionType(function_type::N),
        MappingType(mapping_type::N),
        IdentifierPath(identifier_path::N),
    }
}

/// YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignment | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
pub mod yul_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        YulBlock(yul_block::N),
        YulVariableDeclaration(yul_variable_declaration::N),
        YulFunctionDefinition(yul_function_definition::N),
        YulAssignment(yul_assignment::N),
        YulFunctionCall(yul_function_call::N),
        YulIfStatement(yul_if_statement::N),
        YulForStatement(yul_for_statement::N),
        YulSwitchStatement(yul_switch_statement::N),
        _8(usize),
    }
}

/// ConstructorAttribute = ModifierInvocation | 'payable' | 'internal' | 'public' ;
pub mod constructor_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<constructor_attribute::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        ModifierInvocation(modifier_invocation::N),
        _1(usize),
    }
}

/// ErrorParameter = TypeName [ «Identifier» ] ;
pub mod error_parameter {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<error_parameter::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::N>,
    }
}

/// EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
pub mod event_parameter {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<event_parameter::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub indexed: Option<FixedTerminal<7usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::N>,
    }
}

/// Expression = 1…*{ Expression / AssignmentOperator } | Expression [ '?' Expression ':' Expression ] | 1…*{ Expression / '||' } | 1…*{ Expression / '&&' } | 1…*{ Expression / EqualityComparisonOperator } | 1…*{ Expression / OrderComparisonOperator } | 1…*{ Expression / '|' } | 1…*{ Expression / '^' } | 1…*{ Expression / '&' } | 1…*{ Expression / ShiftOperator } | 1…*{ Expression / AddSubOperator } | 1…*{ Expression / MulDivModOperator } | 1…*{ Expression / '**' } | Expression [ UnarySuffixOperator ] | [ UnaryPrefixOperator ] Expression | Expression { ArgumentList } | Expression { '{' 1…*{ NamedArgument / ',' } '}' } | Expression { '.' ( «Identifier» | 'address' ) } | Expression { '[' [ Expression ] [ ':' [ Expression ] ] ']' } | 'payable' ArgumentList | 'type' '(' TypeName ')' | 'new' TypeName | '(' 1…*{ [ Expression ] / ',' } ')' | '[' 1…*{ Expression / ',' } ']' | «Identifier» | Literal | ElementaryTypeWithoutPayable ;
pub mod expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<expression::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        Expressions_1(Box<expression::AssignmentExpression>),
        ConditionalExpression(Box<expression::ConditionalExpression>),
        Expressions_2(Box<expression::OrExpression>),
        Expressions_3(Box<expression::AndExpression>),
        Expressions_4(Box<expression::EqualityComparisonExpression>),
        Expressions_5(Box<expression::OrderComparisonExpression>),
        Expressions_6(Box<expression::BitOrExpression>),
        Expressions_7(Box<expression::BitXOrExpression>),
        Expressions_8(Box<expression::BitAndExpression>),
        Expressions_9(Box<expression::ShiftExpression>),
        Expressions_10(Box<expression::AddSubExpression>),
        Expressions_11(Box<expression::MulDivModExpression>),
        Expressions_12(Box<expression::ExponentiationExpression>),
        UnarySuffixExpression(Box<expression::UnarySuffixExpression>),
        UnaryPrefixExpression(Box<expression::UnaryPrefixExpression>),
        FunctionCallExpression(Box<expression::FunctionCallExpression>),
        FunctionCallOptionsExpression(Box<expression::FunctionCallOptionsExpression>),
        MemberAccessExpression(Box<expression::MemberAccessExpression>),
        IndexAccessExpression(Box<expression::IndexAccessExpression>),
        _T12(Box<expression::_T12>),
        _T13(Box<expression::_T13>),
        _T14(Box<expression::_T14>),
        _T15(Box<expression::_T15>),
        _T17(Box<expression::_T17>),
        Identifier(identifier::N),
        Literal(literal::N),
        ElementaryTypeWithoutPayable(elementary_type_without_payable::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T17 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Box<expression::_T18>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T18 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T15 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Box<expression::_T16>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T16 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<Option<expression::N>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T14 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub new: FixedTerminal<3usize>,
        pub type_name: type_name::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T13 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#type: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T12 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub payable: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: argument_list::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct IndexAccessExpression {
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t10s: Vec<Box<expression::_T10>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T10 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t11: Option<Box<expression::_T11>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T11 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct MemberAccessExpression {
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t7s: Vec<Box<expression::_T7>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T7 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_char: FixedTerminal<1usize>,
        pub _t8: Box<expression::_T8>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T8 {
        Identifier(identifier::N),
        Address(FixedTerminal<7usize>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct FunctionCallOptionsExpression {
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t4s: Vec<Box<expression::_T4>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub named_arguments: Box<expression::_T5>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub named_arguments: Vec<named_argument::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct FunctionCallExpression {
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_lists: Vec<argument_list::N>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UnaryPrefixExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: Option<usize>,
        pub expression: expression::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UnarySuffixExpression {
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Option<FixedTerminal<2usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct ExponentiationExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_stars: Vec<FixedTerminal<2usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct MulDivModExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct AddSubExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct ShiftExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Vec<usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct BitAndExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ampersand_chars: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct BitXOrExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub caret_chars: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct BitOrExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub bar_chars: Vec<FixedTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct OrderComparisonExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Vec<usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct EqualityComparisonExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Vec<FixedTerminal<2usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct AndExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ampersand_ampersands: Vec<FixedTerminal<2usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct OrExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub bar_bars: Vec<FixedTerminal<2usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct ConditionalExpression {
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<expression::_T1>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub question_char: FixedTerminal<1usize>,
        pub expression_1: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_char: FixedTerminal<1usize>,
        pub expression_2: expression::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct AssignmentExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Vec<usize>,
    }
}

/// FallbackFunctionAttribute = 'external' | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub mod fallback_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fallback_function_attribute::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        _0(usize),
        ModifierInvocation(modifier_invocation::N),
        Virtual(FixedTerminal<7usize>),
        OverrideSpecifier(override_specifier::N),
    }
}

/// FunctionAttribute = VisibilitySpecifier | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub mod function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_attribute::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        _0(usize),
        ModifierInvocation(modifier_invocation::N),
        Virtual(FixedTerminal<7usize>),
        OverrideSpecifier(override_specifier::N),
    }
}

/// InheritanceSpecifierList = 'is' 1…*{ InheritanceSpecifier / ',' } ;
pub mod inheritance_specifier_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<inheritance_specifier_list::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub is: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifiers: Box<inheritance_specifier_list::_T1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifiers: Vec<inheritance_specifier::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// ReceiveFunctionAttribute = 'external' | 'payable' | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub mod receive_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<receive_function_attribute::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        _0(usize),
        ModifierInvocation(modifier_invocation::N),
        Virtual(FixedTerminal<7usize>),
        OverrideSpecifier(override_specifier::N),
    }
}

/// StructDefinition = 'struct' «Identifier» '{' 1…*{ TypeName «Identifier» ';' } '}' ;
pub mod struct_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<struct_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#struct: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2s: Vec<Box<struct_definition::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T2 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// UsingDirective = 'using' ( IdentifierPath | '{' 1…*{ IdentifierPath / ',' } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
pub mod using_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<using_directive::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub using: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub _t1: Box<using_directive::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#for: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub _t4: Box<using_directive::_T4>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub global: Option<FixedTerminal<6usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T4 {
        StarChar(FixedTerminal<1usize>),
        TypeName(type_name::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        IdentifierPath(identifier_path::N),
        _T2(Box<using_directive::_T2>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_paths: Box<using_directive::_T3>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_paths: Vec<identifier_path::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// VariableDeclaration = TypeName [ DataLocation ] «Identifier» ;
pub mod variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<variable_declaration::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _2: Option<usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
    }
}

/// YulBlock = '{' { YulStatement } '}' ;
pub mod yul_block {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_block::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_statements: Vec<yul_statement::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
}

/// AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
pub mod assembly_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<assembly_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub assembly: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_evmasm_double_quote: Option<FixedTerminal<8usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub assembly_flags: Option<assembly_flags::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
}

/// ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
pub mod constant_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<constant_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub constant: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// Directive = «PragmaDirective» | ImportDirective | UsingDirective ;
pub mod directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<directive::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        PragmaDirective(pragma_directive::N),
        ImportDirective(import_directive::N),
        UsingDirective(using_directive::N),
    }
}

/// DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
pub mod do_while_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<do_while_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#do: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub statement: statement::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#while: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_6: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// EmitStatement = 'emit' Expression ArgumentList ';' ;
pub mod emit_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<emit_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub emit: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: argument_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// ErrorDefinition = 'error' «Identifier» '(' { ErrorParameter / ',' } ')' ';' ;
pub mod error_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<error_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub error: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub error_parameters: Option<Box<error_definition::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub error_parameters: Vec<error_parameter::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// EventDefinition = 'event' «Identifier» '(' { EventParameter / ',' } ')' [ 'anonymous' ] ';' ;
pub mod event_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<event_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub event: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub event_parameters: Option<Box<event_definition::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub anonymous: Option<FixedTerminal<9usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_6: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub event_parameters: Vec<event_parameter::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// ExpressionStatement = Expression ';' ;
pub mod expression_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<expression_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
pub mod if_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<if_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#if: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub statement: statement::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<if_statement::_T1>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#else: FixedTerminal<4usize>,
        pub statement: statement::N,
    }
}

/// ReturnStatement = 'return' [ Expression ] ';' ;
pub mod return_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<return_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#return: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// RevertStatement = 'revert' Expression ArgumentList ';' ;
pub mod revert_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<revert_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub revert: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: argument_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
pub mod state_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<state_variable_declaration::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub state_variable_attributes: Vec<state_variable_attribute::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2: Option<Box<state_variable_declaration::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedTerminal<1usize>,
        pub expression: expression::N,
    }
}

/// TryStatement = 'try' Expression [ 'returns' NonEmptyParameterList ] Block 1…*{ CatchClause } ;
pub mod try_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<try_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#try: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<try_statement::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: block::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub catch_clauses: Vec<catch_clause::N>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}

/// VariableDeclarationTuple = '(' { ',' } VariableDeclaration { ',' [ VariableDeclaration ] } ')' ;
pub mod variable_declaration_tuple {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<variable_declaration_tuple::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_chars: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub variable_declaration: variable_declaration::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t3s: Vec<Box<variable_declaration_tuple::_T3>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub variable_declaration: Option<variable_declaration::N>,
    }
}

/// WhileStatement = 'while' '(' Expression ')' Statement ;
pub mod while_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<while_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#while: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub statement: statement::N,
    }
}

/// VariableDeclarationStatement = ( VariableDeclaration [ '=' Expression ] | VariableDeclarationTuple '=' Expression ) ';' ;
pub mod variable_declaration_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<variable_declaration_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<variable_declaration_statement::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(Box<variable_declaration_statement::_T2>),
        _T4(Box<variable_declaration_statement::_T4>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T4 {
        pub variable_declaration_tuple: variable_declaration_tuple::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedTerminal<1usize>,
        pub expression: expression::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T2 {
        pub variable_declaration: variable_declaration::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t3: Option<Box<variable_declaration_statement::_T3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedTerminal<1usize>,
        pub expression: expression::N,
    }
}

/// SimpleStatement = VariableDeclarationStatement | ExpressionStatement ;
pub mod simple_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<simple_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        VariableDeclarationStatement(variable_declaration_statement::N),
        ExpressionStatement(expression_statement::N),
    }
}

/// ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
pub mod for_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<for_statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#for: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub _t1: Box<for_statement::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub _t2: Box<for_statement::_T2>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_6: ignore::N,
        pub statement: statement::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T2 {
        ExpressionStatement(expression_statement::N),
        SemicolonChar(FixedTerminal<1usize>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        SimpleStatement(simple_statement::N),
        SemicolonChar(FixedTerminal<1usize>),
    }
}

/// Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | AssemblyStatement ;
pub mod statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<statement::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
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

/// Block = '{' { Statement | UncheckedBlock } '}' ;
pub mod block {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<block::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2s: Vec<Box<block::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T2 {
        Statement(statement::N),
        UncheckedBlock(unchecked_block::N),
    }
}

/// ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
pub mod constructor_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<constructor_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub constructor: FixedTerminal<11usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: parameter_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub constructor_attributes: Vec<constructor_attribute::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: block::N,
    }
}

/// FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
pub mod fallback_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fallback_function_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub fallback: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: parameter_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub fallback_function_attributes: Vec<fallback_function_attribute::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2: Option<Box<fallback_function_definition::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub _t3: Box<fallback_function_definition::_T3>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T3 {
        SemicolonChar(FixedTerminal<1usize>),
        Block(block::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}

/// FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
pub mod function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub _t1: Box<function_definition::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: parameter_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function_attributes: Vec<function_attribute::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t3: Option<Box<function_definition::_T3>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        pub _t4: Box<function_definition::_T4>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T4 {
        SemicolonChar(FixedTerminal<1usize>),
        Block(block::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        Identifier(identifier::N),
        _1(usize),
    }
}

/// ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { MethodAttribute } ( ';' | Block ) ;
pub mod modifier_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<modifier_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub modifier: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: Option<parameter_list::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub method_attributes: Vec<method_attribute::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub _t2: Box<modifier_definition::_T2>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T2 {
        SemicolonChar(FixedTerminal<1usize>),
        Block(block::N),
    }
}

/// ReceiveFunctionDefinition = 'receive' '(' ')' { ReceiveFunctionAttribute } ( ';' | Block ) ;
pub mod receive_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<receive_function_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub receive: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub receive_function_attributes: Vec<receive_function_attribute::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub _t2: Box<receive_function_definition::_T2>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T2 {
        SemicolonChar(FixedTerminal<1usize>),
        Block(block::N),
    }
}

/// ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
pub mod contract_body_element {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<contract_body_element::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
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

/// ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
pub mod contract_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<contract_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#abstract: Option<FixedTerminal<8usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub contract: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifier_list: Option<inheritance_specifier_list::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub contract_body_elements: Vec<contract_body_element::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_6: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
}

/// InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
pub mod interface_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<interface_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub interface: FixedTerminal<9usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifier_list: Option<inheritance_specifier_list::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub contract_body_elements: Vec<contract_body_element::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
}

/// LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
pub mod library_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<library_definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub library: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub contract_body_elements: Vec<contract_body_element::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
}

/// Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
pub mod definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<definition::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
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

/// SourceUnit = «IGNORE» { Directive | Definition } $ ;
pub mod source_unit {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<source_unit::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2s: Vec<Box<source_unit::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub end_marker: (),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T2 {
        Directive(directive::N),
        Definition(definition::N),
    }
}
