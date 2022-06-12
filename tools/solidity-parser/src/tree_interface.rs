#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
pub trait DefaultTest {
    fn is_default(&self) -> bool {
        false
    }
}
impl<T: DefaultTest> DefaultTest for Box<T> {
    fn is_default(&self) -> bool {
        self.as_ref().is_default()
    }
}
impl<T> DefaultTest for Vec<T> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}
impl<T> DefaultTest for Option<T> {
    fn is_default(&self) -> bool {
        self.is_none()
    }
}
impl DefaultTest for () {
    fn is_default(&self) -> bool {
        true
    }
}
impl DefaultTest for usize {
    fn is_default(&self) -> bool {
        *self == 0
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FixedTerminal<const N: usize>();
impl<const N: usize> DefaultTest for FixedTerminal<N> {
    fn is_default(&self) -> bool {
        true
    }
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
    pub type N = Box<break_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#break: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
pub mod comment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<comment::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub slash_star: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _c2s: Vec<Box<comment::_C2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_chars: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_slash: FixedTerminal<2usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C2 {
        StarChar(FixedTerminal<1usize>),
        _S3(Box<comment::_S3>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_chars: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: FixedTerminal<1usize>,
    }
}

/// ContinueStatement = 'continue' ';' ;
pub mod continue_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<continue_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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

/// EqualityComparisonOperator = '==' | '!=' ;
pub mod equality_comparison_operator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<2usize>;
}

/// «LineComment» = '//' { ¬( '\u{a}' | '\u{d}' ) } ;
pub mod line_comment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<line_comment::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub slash_slash: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
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
    pub type N = Box<positional_argument_list::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s1s: Vec<Box<positional_argument_list::_S1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<positional_argument_list::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S1 {
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
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
    pub type N = Box<unchecked_block::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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

/// «Whitespace» = 1…*{ '\u{20}' | '\u{9}' | '\u{d}' | '\u{a}' } ;
pub mod whitespace {
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

/// «IGNORE» = { «Whitespace» | «Comment» | «LineComment» } ;
pub mod ignore {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<Box<ignore::_C1>>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        Whitespace(whitespace::N),
        Comment(comment::N),
        LineComment(line_comment::N),
    }
}

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

/// «DecimalInteger» = 1…*{ '0'…'9' / [ '_' ] } ;
pub mod decimal_integer {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_integer::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<FixedTerminal<1usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub underscore_chars: Vec<Option<FixedTerminal<1usize>>>,
    }
}

/// «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
pub mod fixed_bytes_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fixed_bytes_type::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<fixed_type::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub fixed: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2: Option<Box<fixed_type::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
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

/// «NumberUnit» = 'wei' | 'gwei' | 'ether' | 'seconds' | 'minutes' | 'hours' | 'days' | 'weeks' | 'years' ;
pub mod number_unit {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «PragmaDirective» = 'pragma' ¬';' { ¬';' } ';' ;
pub mod pragma_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<pragma_directive::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub pragma: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char_0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_semicolon_chars: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char_1: FixedTerminal<1usize>,
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
    pub type N = Box<signed_integer_type::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub int: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
pub mod yul_decimal_number_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_decimal_number_literal::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        Zero(FixedTerminal<1usize>),
        _S1(Box<yul_decimal_number_literal::_S1>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
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

/// «YulHexLiteral» = '0x' ( '0'…'9' | 'a'…'f' | 'A'…'F' ) { '0'…'9' | 'a'…'f' | 'A'…'F' } ;
pub mod yul_hex_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_hex_literal::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub zero_x: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _2: usize,
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
    pub type N = Box<decimal_exponent::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<decimal_float::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_integer_0: Option<decimal_integer::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_integer_1: decimal_integer::N,
    }
}

/// «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
pub mod hex_byte_escape {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<hex_byte_escape::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<hex_number::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub zero_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _2: Box<hex_number::_S1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<FixedTerminal<1usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub underscore_chars: Vec<Option<FixedTerminal<1usize>>>,
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
    pub type N = Box<possibly_separated_pairs_of_hex_digits::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expressions: Vec<usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub underscore_chars: Vec<Option<FixedTerminal<1usize>>>,
    }
}

/// «UfixedType» = 'u' «FixedType» ;
pub mod ufixed_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<ufixed_type::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<unicode_escape::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<unsigned_integer_type::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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

/// «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
pub mod decimal_number {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<decimal_number::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub _c1: Box<decimal_number::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_exponent: Option<decimal_exponent::N>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        DecimalInteger(decimal_integer::N),
        DecimalFloat(decimal_float::N),
    }
}

/// ElementaryType = 'bool' | 'string' | 'bytes' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | «FixedType» | «UfixedType» ;
pub mod elementary_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<elementary_type::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        _0(usize),
        SignedIntegerType(signed_integer_type::N),
        UnsignedIntegerType(unsigned_integer_type::N),
        FixedBytesType(fixed_bytes_type::N),
        FixedType(fixed_type::N),
        UfixedType(ufixed_type::N),
    }
}

/// «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
pub mod escape_sequence {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<escape_sequence::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub backslash_char: FixedTerminal<1usize>,
        pub _c1: Box<escape_sequence::_C1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        _0(FixedTerminal<1usize>),
        HexByteEscape(hex_byte_escape::N),
        UnicodeEscape(unicode_escape::N),
    }
}

/// «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
pub mod hex_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<hex_string_literal::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub hex: FixedTerminal<3usize>,
        pub _c1: Box<hex_string_literal::_C1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        _S2(Box<hex_string_literal::_S2>),
        _S4(Box<hex_string_literal::_S4>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub possibly_separated_pairs_of_hex_digits:
            Option<possibly_separated_pairs_of_hex_digits::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_1: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_char_0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub possibly_separated_pairs_of_hex_digits:
            Option<possibly_separated_pairs_of_hex_digits::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_char_1: FixedTerminal<1usize>,
    }
}

/// «Keyword» = 'pragma' | 'abstract' | 'anonymous' | 'address' | 'as' | 'assembly' | 'bool' | 'break' | 'bytes' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | 'fixed' | 'ufixed' ;
pub mod keyword {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<keyword::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        _0(usize),
        SignedIntegerType(signed_integer_type::N),
        UnsignedIntegerType(unsigned_integer_type::N),
        FixedBytesType(fixed_bytes_type::N),
        _4(usize),
    }
}

/// «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
pub mod raw_identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<raw_identifier::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
    }
}

/// «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
pub mod double_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<double_quoted_ascii_string_literal::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_char_0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub runs: Vec<Box<double_quoted_ascii_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_char_1: FixedTerminal<1usize>,
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
    pub type N = Box<double_quoted_unicode_string_literal::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<elementary_type_with_payable::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        _S1(Box<elementary_type_with_payable::_S1>),
        ElementaryType(elementary_type::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub address: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub payable: Option<FixedTerminal<7usize>>,
    }
}

/// ElementaryTypeWithoutPayable = 'address' | ElementaryType ;
pub mod elementary_type_without_payable {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<elementary_type_without_payable::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        Address(FixedTerminal<7usize>),
        ElementaryType(elementary_type::N),
    }
}

/// NumericLiteral = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
pub mod numeric_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<numeric_literal::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub _c1: Box<numeric_literal::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Option<usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        DecimalNumber(decimal_number::N),
        HexNumber(hex_number::N),
    }
}

/// «ReservedWord» = «Keyword» | «ReservedKeyword» | «NumberUnit» | «BooleanLiteral» ;
pub mod reserved_word {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<reserved_word::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        Keyword(keyword::N),
        _1(usize),
    }
}

/// «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
pub mod single_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<single_quoted_ascii_string_literal::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub runs: Vec<Box<single_quoted_ascii_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_1: FixedTerminal<1usize>,
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
    pub type N = Box<single_quoted_unicode_string_literal::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<ascii_string_literal::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        SingleQuotedAsciiStringLiteral(single_quoted_ascii_string_literal::N),
        DoubleQuotedAsciiStringLiteral(double_quoted_ascii_string_literal::N),
    }
}

/// AssemblyFlags = '(' 1…*{ «DoubleQuotedAsciiStringLiteral» / ',' } ')' ;
pub mod assembly_flags {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<assembly_flags::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Box<assembly_flags::_S1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<assembly_flags::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<assembly_flags::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quoted_ascii_string_literal: double_quoted_ascii_string_literal::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
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
    pub type N = Box<unicode_string_literal::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        SingleQuotedUnicodeStringLiteral(single_quoted_unicode_string_literal::N),
        DoubleQuotedUnicodeStringLiteral(double_quoted_unicode_string_literal::N),
    }
}

/// YulFunctionCall = ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) '(' { YulExpression / ',' } ')' ;
pub mod yul_function_call {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_function_call::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub _c1: Box<yul_function_call::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Option<Box<yul_function_call::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<yul_function_call::_S3>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4s: Vec<Box<yul_function_call::_S4>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        pub yul_expression: yul_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        YulIdentifier(yul_identifier::N),
        _1(usize),
    }
}

/// YulFunctionDefinition = 'function' «YulIdentifier» '(' { «YulIdentifier» / ',' } ')' [ '->' 1…*{ «YulIdentifier» / ',' } ] YulBlock ;
pub mod yul_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_function_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Option<Box<yul_function_definition::_S1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s5: Option<Box<yul_function_definition::_S5>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub minus_greater: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s7s: Box<yul_function_definition::_S6>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S6 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s7s: Vec<Box<yul_function_definition::_S7>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s8s: Vec<Box<yul_function_definition::_S8>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S8 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S7 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<yul_function_definition::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<yul_function_definition::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// YulPath = «YulIdentifier» { '.' ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) } ;
pub mod yul_path {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_path::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<yul_path::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub _c3: Box<yul_path::_C3>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C3 {
        YulIdentifier(yul_identifier::N),
        _1(usize),
    }
}

/// EnumDefinition = 'enum' «Identifier» '{' 1…*{ «Identifier» / ',' } '}' ;
pub mod enum_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<enum_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#enum: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Box<enum_definition::_S1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<enum_definition::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<enum_definition::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// IdentifierPath = 1…*{ «Identifier» / '.' } ;
pub mod identifier_path {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<identifier_path::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s1s: Vec<Box<identifier_path::_S1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<identifier_path::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
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
    pub type N = Box<literal::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
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
    pub type N = Box<named_argument::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub expression: expression::N,
    }
}

/// ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
pub mod parameter_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<parameter_declaration::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Option<usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::N>,
    }
}

/// SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
pub mod selected_import {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<selected_import::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2: Option<Box<selected_import::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#as: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
    }
}

/// UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryTypeWithPayable ';' ;
pub mod user_defined_value_type_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<user_defined_value_type_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#type: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub is: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub elementary_type_with_payable: elementary_type_with_payable::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
pub mod yul_literal {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_literal::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
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
    pub type N = Box<mapping_type::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub mapping: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub _c1: Box<mapping_type::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_greater: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        ElementaryTypeWithoutPayable(elementary_type_without_payable::N),
        IdentifierPath(identifier_path::N),
    }
}

/// NamedArgumentList = '{' { NamedArgument / ',' } '}' ;
pub mod named_argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<named_argument_list::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Option<Box<named_argument_list::_S1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<named_argument_list::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<named_argument_list::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub named_argument: named_argument::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// NonEmptyParameterList = '(' 1…*{ ParameterDeclaration / ',' } ')' ;
pub mod non_empty_parameter_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<non_empty_parameter_list::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Box<non_empty_parameter_list::_S1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<non_empty_parameter_list::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<non_empty_parameter_list::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub parameter_declaration: parameter_declaration::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// OverrideSpecifier = 'override' [ '(' 1…1*{ IdentifierPath / ',' } ')' ] ;
pub mod override_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<override_specifier::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#override: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2: Option<Box<override_specifier::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_paths: Vec<identifier_path::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
}

/// ParameterList = '(' { ParameterDeclaration / ',' } ')' ;
pub mod parameter_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<parameter_list::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Option<Box<parameter_list::_S1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<parameter_list::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<parameter_list::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub parameter_declaration: parameter_declaration::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// SelectingImportDirective = '{' 1…*{ SelectedImport / ',' } '}' 'from' ImportPath ;
pub mod selecting_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<selecting_import_directive::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Box<selecting_import_directive::_S1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub from: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub import_path: import_path::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<selecting_import_directive::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<selecting_import_directive::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub selected_import: selected_import::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
pub mod simple_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<simple_import_directive::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub import_path: import_path::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<simple_import_directive::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#as: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
pub mod star_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<star_import_directive::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#as: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub from: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub import_path: import_path::N,
    }
}

/// YulExpression = YulPath | YulFunctionCall | YulLiteral ;
pub mod yul_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_expression::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        YulPath(yul_path::N),
        YulFunctionCall(yul_function_call::N),
        YulLiteral(yul_literal::N),
    }
}

/// ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
pub mod argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<argument_list::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _c2: Option<Box<argument_list::_C2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C2 {
        PositionalArgumentList(positional_argument_list::N),
        NamedArgumentList(named_argument_list::N),
    }
}

/// CatchClause = 'catch' [ [ «Identifier» ] NonEmptyParameterList ] Block ;
pub mod catch_clause {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<catch_clause::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub catch: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2: Option<Box<catch_clause::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: block::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}

/// FunctionType = 'function' ParameterList { VisibilitySpecifier | StateMutabilitySpecifier } [ 'returns' NonEmptyParameterList ] ;
pub mod function_type {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_type::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: parameter_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<function_type::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4: Option<Box<function_type::_S4>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
pub mod import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<import_directive::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub import: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub _c1: Box<import_directive::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        SimpleImportDirective(simple_import_directive::N),
        StarImportDirective(star_import_directive::N),
        SelectingImportDirective(selecting_import_directive::N),
    }
}

/// MethodAttribute = 'virtual' | OverrideSpecifier ;
pub mod method_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<method_attribute::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        Virtual(FixedTerminal<7usize>),
        OverrideSpecifier(override_specifier::N),
    }
}

/// StateVariableAttribute = 'public' | 'private' | 'internal' | 'constant' | OverrideSpecifier | 'immutable' ;
pub mod state_variable_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<state_variable_attribute::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        _0(usize),
        OverrideSpecifier(override_specifier::N),
        Immutable(FixedTerminal<9usize>),
    }
}

/// YulAssignment = YulPath ( ':=' YulExpression | 1…*{ ',' YulPath } ':=' YulFunctionCall ) ;
pub mod yul_assignment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_assignment::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_path: yul_path::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub _c1: Box<yul_assignment::_C1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        _S2(Box<yul_assignment::_S2>),
        _S3(Box<yul_assignment::_S3>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s5s: Vec<Box<yul_assignment::_S5>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub yul_function_call: yul_function_call::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_path: yul_path::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub yul_expression: yul_expression::N,
    }
}

/// YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
pub mod yul_for_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_for_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#for: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block_0: yul_block::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub yul_expression: yul_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block_1: yul_block::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block_2: yul_block::N,
    }
}

/// YulIfStatement = 'if' YulExpression YulBlock ;
pub mod yul_if_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_if_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#if: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub yul_expression: yul_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
}

/// YulSwitchStatement = 'switch' YulExpression ( 1…*{ 'case' YulLiteral YulBlock } [ 'default' YulBlock ] | 'default' YulBlock ) ;
pub mod yul_switch_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_switch_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub switch: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub yul_expression: yul_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub _c1: Box<yul_switch_statement::_C1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        _S2(Box<yul_switch_statement::_S2>),
        _S7(Box<yul_switch_statement::_S7>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S7 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub default: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4s: Vec<Box<yul_switch_statement::_S4>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s6: Option<Box<yul_switch_statement::_S6>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S6 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub default: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub case: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub yul_literal: yul_literal::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// YulVariableDeclaration = 'let' «YulIdentifier» [ ':=' YulExpression | [ ',' «YulIdentifier» ] [ ':=' YulFunctionCall ] ] ;
pub mod yul_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_variable_declaration::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#let: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _c2: Option<Box<yul_variable_declaration::_C2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C2 {
        _S3(Box<yul_variable_declaration::_S3>),
        _S4(Box<yul_variable_declaration::_S4>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s6: Option<Box<yul_variable_declaration::_S6>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s8: Option<Box<yul_variable_declaration::_S8>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S8 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub yul_function_call: yul_function_call::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S6 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub yul_expression: yul_expression::N,
    }
}

/// InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
pub mod inheritance_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<inheritance_specifier::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<modifier_invocation::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<type_name::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub _c1: Box<type_name::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<type_name::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
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
    pub type N = Box<yul_statement::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
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
    pub type N = Box<constructor_attribute::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        ModifierInvocation(modifier_invocation::N),
        _1(usize),
    }
}

/// ErrorParameter = TypeName [ «Identifier» ] ;
pub mod error_parameter {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<error_parameter::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<event_parameter::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub indexed: Option<FixedTerminal<7usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::N>,
    }
}

/// FallbackFunctionAttribute = 'external' | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub mod fallback_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fallback_function_attribute::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
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
    pub type N = Box<function_attribute::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
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
    pub type N = Box<inheritance_specifier_list::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub is: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Box<inheritance_specifier_list::_S1>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<inheritance_specifier_list::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<inheritance_specifier_list::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifier: inheritance_specifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// PrimaryExpression = 'payable' ArgumentList | 'type' '(' TypeName ')' | 'new' TypeName | '(' 1…*{ [ Expression ] / ',' } ')' | '[' 1…*{ Expression / ',' } ']' | «Identifier» | Literal | ElementaryTypeWithoutPayable ;
pub mod primary_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<primary_expression::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        _S1(Box<primary_expression::_S1>),
        _S2(Box<primary_expression::_S2>),
        _S3(Box<primary_expression::_S3>),
        _S4(Box<primary_expression::_S4>),
        _S9(Box<primary_expression::_S9>),
        Identifier(identifier::N),
        Literal(literal::N),
        ElementaryTypeWithoutPayable(elementary_type_without_payable::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S9 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s11s: Box<primary_expression::_S10>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S10 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s11s: Vec<Box<primary_expression::_S11>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s12s: Vec<Box<primary_expression::_S12>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S12 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S11 {
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s7s: Box<primary_expression::_S5>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s7s: Vec<Box<primary_expression::_S7>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s8s: Vec<Box<primary_expression::_S8>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S8 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S7 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub new: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub type_name: type_name::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#type: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub payable: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: argument_list::N,
    }
}

/// ReceiveFunctionAttribute = 'external' | 'payable' | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub mod receive_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<receive_function_attribute::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
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
    pub type N = Box<struct_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#struct: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<struct_definition::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// UsingDirective = 'using' ( IdentifierPath | '{' 1…*{ IdentifierPath / ',' } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
pub mod using_directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<using_directive::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub using: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub _c1: Box<using_directive::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#for: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub _c6: Box<using_directive::_C6>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub global: Option<FixedTerminal<6usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C6 {
        Star(FixedTerminal<1usize>),
        TypeName(type_name::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        IdentifierPath(identifier_path::N),
        _S2(Box<using_directive::_S2>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4s: Box<using_directive::_S3>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4s: Vec<Box<using_directive::_S4>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s5s: Vec<Box<using_directive::_S5>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path: identifier_path::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// VariableDeclaration = TypeName [ DataLocation ] «Identifier» ;
pub mod variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<variable_declaration::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Option<usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
    }
}

/// YulBlock = '{' { YulStatement } '}' ;
pub mod yul_block {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<yul_block::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<yul_block::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub yul_statement: yul_statement::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
pub mod assembly_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<assembly_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub assembly: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_evmasm_double_quote: Option<FixedTerminal<8usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub assembly_flags: Option<assembly_flags::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: yul_block::N,
    }
}

/// Directive = «PragmaDirective» | ImportDirective | UsingDirective ;
pub mod directive {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<directive::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        PragmaDirective(pragma_directive::N),
        ImportDirective(import_directive::N),
        UsingDirective(using_directive::N),
    }
}

/// ErrorDefinition = 'error' «Identifier» '(' { ErrorParameter / ',' } ')' ';' ;
pub mod error_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<error_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub error: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Option<Box<error_definition::_S1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<error_definition::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<error_definition::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub error_parameter: error_parameter::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// EventDefinition = 'event' «Identifier» '(' { EventParameter / ',' } ')' [ 'anonymous' ] ';' ;
pub mod event_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<event_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub event: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Option<Box<event_definition::_S1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub anonymous: Option<FixedTerminal<9usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<event_definition::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<event_definition::_S3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub event_parameter: event_parameter::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// IndexAccessExpression = PrimaryExpression { '[' [ Expression ] [ ':' [ Expression ] ] ']' } ;
pub mod index_access_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<index_access_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub primary_expression: primary_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<index_access_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s5: Option<Box<index_access_expression::_S5>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
    }
}

/// VariableDeclarationTuple = '(' { ',' } VariableDeclaration { ',' [ VariableDeclaration ] } ')' ;
pub mod variable_declaration_tuple {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<variable_declaration_tuple::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<variable_declaration_tuple::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub variable_declaration: variable_declaration::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4s: Vec<Box<variable_declaration_tuple::_S4>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub variable_declaration: Option<variable_declaration::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// MemberAccessExpression = IndexAccessExpression { '.' ( «Identifier» | 'address' ) } ;
pub mod member_access_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<member_access_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub index_access_expression: index_access_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<member_access_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub _c3: Box<member_access_expression::_C3>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C3 {
        Identifier(identifier::N),
        Address(FixedTerminal<7usize>),
    }
}

/// FunctionCallOptionsExpression = MemberAccessExpression { '{' 1…*{ NamedArgument / ',' } '}' } ;
pub mod function_call_options_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_call_options_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub member_access_expression: member_access_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<function_call_options_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4s: Box<function_call_options_expression::_S3>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4s: Vec<Box<function_call_options_expression::_S4>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s5s: Vec<Box<function_call_options_expression::_S5>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S4 {
        pub named_argument: named_argument::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// FunctionCallExpression = FunctionCallOptionsExpression { ArgumentList } ;
pub mod function_call_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_call_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub function_call_options_expression: function_call_options_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<function_call_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: argument_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// UnaryPrefixExpression = [ UnaryPrefixOperator ] FunctionCallExpression ;
pub mod unary_prefix_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unary_prefix_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: Option<usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub function_call_expression: function_call_expression::N,
    }
}

/// UnarySuffixExpression = UnaryPrefixExpression [ UnarySuffixOperator ] ;
pub mod unary_suffix_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<unary_suffix_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub unary_prefix_expression: unary_prefix_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: Option<FixedTerminal<2usize>>,
    }
}

/// ExpExpression = UnarySuffixExpression { '**' Expression } ;
pub mod exp_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<exp_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub unary_suffix_expression: unary_suffix_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<exp_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_star: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// MulDivModExpression = ExpExpression { MulDivModOperator ExpExpression } ;
pub mod mul_div_mod_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<mul_div_mod_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub exp_expression: exp_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<mul_div_mod_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub exp_expression: exp_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// AddSubExpression = MulDivModExpression { AddSubOperator MulDivModExpression } ;
pub mod add_sub_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<add_sub_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub mul_div_mod_expression: mul_div_mod_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<add_sub_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub mul_div_mod_expression: mul_div_mod_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// ShiftExpression = AddSubExpression { ShiftOperator AddSubExpression } ;
pub mod shift_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<shift_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub add_sub_expression: add_sub_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<shift_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub add_sub_expression: add_sub_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// BitAndExpression = ShiftExpression { '&' ShiftExpression } ;
pub mod bit_and_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<bit_and_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub shift_expression: shift_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<bit_and_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ampersand_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub shift_expression: shift_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// BitXOrExpression = BitAndExpression { '^' BitAndExpression } ;
pub mod bit_x_or_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<bit_x_or_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub bit_and_expression: bit_and_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<bit_x_or_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub caret_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub bit_and_expression: bit_and_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// BitOrExpression = BitXOrExpression { '|' BitXOrExpression } ;
pub mod bit_or_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<bit_or_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub bit_x_or_expression: bit_x_or_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<bit_or_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub bar_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub bit_x_or_expression: bit_x_or_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// OrderComparisonExpression = BitOrExpression { OrderComparisonOperator BitOrExpression } ;
pub mod order_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<order_comparison_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub bit_or_expression: bit_or_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<order_comparison_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub bit_or_expression: bit_or_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// EqualityComparisonExpression = OrderComparisonExpression { EqualityComparisonOperator OrderComparisonExpression } ;
pub mod equality_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<equality_comparison_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub order_comparison_expression: order_comparison_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<equality_comparison_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub order_comparison_expression: order_comparison_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// AndExpression = EqualityComparisonExpression { '&&' EqualityComparisonExpression } ;
pub mod and_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<and_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub equality_comparison_expression: equality_comparison_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<and_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ampersand_ampersand: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub equality_comparison_expression: equality_comparison_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// OrExpression = AndExpression { '||' AndExpression } ;
pub mod or_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<or_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub and_expression: and_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<or_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub bar_bar: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub and_expression: and_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// ConditionalExpression = OrExpression [ '?' Expression ':' Expression ] ;
pub mod conditional_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<conditional_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub or_expression: or_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2: Option<Box<conditional_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub question_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub expression_0: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub expression_1: expression::N,
    }
}

/// AssignmentExpression = ConditionalExpression { AssignmentOperator Expression } ;
pub mod assignment_expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<assignment_expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub conditional_expression: conditional_expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<assignment_expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
    }
}

/// Expression = AssignmentExpression ;
pub mod expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = assignment_expression::N;
}

/// ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
pub mod constant_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<constant_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub constant: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
pub mod do_while_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<do_while_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#do: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub statement: statement::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#while: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// EmitStatement = 'emit' Expression ArgumentList ';' ;
pub mod emit_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<emit_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub emit: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: argument_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// ExpressionStatement = Expression ';' ;
pub mod expression_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<expression_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
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
    pub type N = Box<if_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#if: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub statement: statement::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2: Option<Box<if_statement::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#else: FixedTerminal<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub statement: statement::N,
    }
}

/// ReturnStatement = 'return' [ Expression ] ';' ;
pub mod return_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<return_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#return: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// RevertStatement = 'revert' Expression ArgumentList ';' ;
pub mod revert_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<revert_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub revert: FixedTerminal<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: argument_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
pub mod state_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<state_variable_declaration::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub type_name: type_name::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<state_variable_declaration::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4: Option<Box<state_variable_declaration::_S4>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub expression: expression::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub state_variable_attribute: state_variable_attribute::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// TryStatement = 'try' Expression [ 'returns' NonEmptyParameterList ] Block CatchClause { CatchClause } ;
pub mod try_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<try_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#try: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2: Option<Box<try_statement::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: block::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub catch_clause: catch_clause::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4s: Vec<Box<try_statement::_S4>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub catch_clause: catch_clause::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
}

/// VariableDeclarationStatement = ( VariableDeclaration [ '=' Expression ] | VariableDeclarationTuple '=' Expression ) ';' ;
pub mod variable_declaration_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<variable_declaration_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub _c1: Box<variable_declaration_statement::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        _S2(Box<variable_declaration_statement::_S2>),
        _S5(Box<variable_declaration_statement::_S5>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S5 {
        pub variable_declaration_tuple: variable_declaration_tuple::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub expression: expression::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub variable_declaration: variable_declaration::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4: Option<Box<variable_declaration_statement::_S4>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub expression: expression::N,
    }
}

/// WhileStatement = 'while' '(' Expression ')' Statement ;
pub mod while_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<while_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#while: FixedTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub statement: statement::N,
    }
}

/// SimpleStatement = VariableDeclarationStatement | ExpressionStatement ;
pub mod simple_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<simple_statement::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        VariableDeclarationStatement(variable_declaration_statement::N),
        ExpressionStatement(expression_statement::N),
    }
}

/// ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
pub mod for_statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<for_statement::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#for: FixedTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub _c1: Box<for_statement::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        pub _c2: Box<for_statement::_C2>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<expression::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        pub statement: statement::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C2 {
        ExpressionStatement(expression_statement::N),
        Semicolon(FixedTerminal<1usize>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        SimpleStatement(simple_statement::N),
        Semicolon(FixedTerminal<1usize>),
    }
}

/// Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | AssemblyStatement ;
pub mod statement {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<statement::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// Block = '{' { Statement | UncheckedBlock } '}' ;
pub mod block {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<block::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<block::_S3>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        pub _c2: Box<block::_C2>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C2 {
        Statement(statement::N),
        UncheckedBlock(unchecked_block::N),
    }
}

/// ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
pub mod constructor_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<constructor_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub constructor: FixedTerminal<11usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: parameter_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<constructor_definition::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: block::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub constructor_attribute: constructor_attribute::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
pub mod fallback_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<fallback_function_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub fallback: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: parameter_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<fallback_function_definition::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s4: Option<Box<fallback_function_definition::_S4>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub _c5: Box<fallback_function_definition::_C5>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C5 {
        Semicolon(FixedTerminal<1usize>),
        Block(block::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub fallback_function_attribute: fallback_function_attribute::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
pub mod function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<function_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        pub _c1: Box<function_definition::_C1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: parameter_list::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<function_definition::_S3>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s5: Option<Box<function_definition::_S5>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        pub _c6: Box<function_definition::_C6>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C6 {
        Semicolon(FixedTerminal<1usize>),
        Block(block::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S5 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub non_empty_parameter_list: non_empty_parameter_list::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        pub function_attribute: function_attribute::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        Identifier(identifier::N),
        _1(usize),
    }
}

/// ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { MethodAttribute } ( ';' | Block ) ;
pub mod modifier_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<modifier_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub modifier: FixedTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: Option<parameter_list::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<modifier_definition::_S3>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub _c4: Box<modifier_definition::_C4>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C4 {
        Semicolon(FixedTerminal<1usize>),
        Block(block::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        pub method_attribute: method_attribute::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// ReceiveFunctionDefinition = 'receive' '(' ')' { ReceiveFunctionAttribute } ( ';' | Block ) ;
pub mod receive_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<receive_function_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub receive: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<receive_function_definition::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        pub _c3: Box<receive_function_definition::_C3>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C3 {
        Semicolon(FixedTerminal<1usize>),
        Block(block::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub receive_function_attribute: receive_function_attribute::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
pub mod contract_body_element {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<contract_body_element::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
pub mod contract_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<contract_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#abstract: Option<FixedTerminal<8usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub contract: FixedTerminal<8usize>,
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
        pub _s4s: Vec<Box<contract_definition::_S4>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_5: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S4 {
        pub contract_body_element: contract_body_element::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
pub mod interface_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<interface_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub interface: FixedTerminal<9usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifier_list: Option<inheritance_specifier_list::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<interface_definition::_S3>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_4: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        pub contract_body_element: contract_body_element::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
pub mod library_definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<library_definition::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub library: FixedTerminal<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s2s: Vec<Box<library_definition::_S2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub contract_body_element: contract_body_element::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
}

/// Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
pub mod definition {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<definition::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// SourceUnit = «IGNORE» { Directive | Definition } $ ;
pub mod source_unit {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<source_unit::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _s3s: Vec<Box<source_unit::_S3>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub end_marker: (),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        pub _c2: Box<source_unit::_C2>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C2 {
        Directive(directive::N),
        Definition(definition::N),
    }
}
