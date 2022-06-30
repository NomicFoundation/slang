#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
pub trait DefaultTest {
    fn is_default(&self) -> bool {
        false
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FixedTerminal<const N: usize>();
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct WithNoise<T>
where
    T: Sized + Clone + PartialEq + Hash + Serialize + Deserialize + Default,
{
    leading: Ignore,
    content: T,
    trailing: Ignore,
}

/// «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
pub type AsciiEscape = FixedTerminal<1>;

/// «BooleanLiteral» = 'true' | 'false' ;
pub type BooleanLiteral = usize;

/// «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
pub type Comment = Box<comment::_T0>;
pub mod comment {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T2 {
        pub star_chars: usize,
        pub _1: FixedTerminal<1>,
    }
    pub enum Comment {
        NotStarChar(FixedTerminal<1>),
        _T2(Box<comment::_T2>),
    }
    pub struct Content {
        pub comments: Vec<Box<comment::Comment>>,
        pub star_chars: usize,
    }
    pub struct _T0 {
        pub slash_star: FixedTerminal<2usize>,
        pub content: Box<comment::Content>,
        pub star_slash: FixedTerminal<2usize>,
    }
}

/// «DecimalInteger» = 1…*{ '0'…'9' / [ '_' ] } ;
pub type DecimalInteger = decimal_integer::_T0;
pub mod decimal_integer {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub elements: Vec<FixedTerminal<1>>,
        pub separators: Vec<Option<FixedTerminal<1>>>,
    }
}

/// «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
pub type FixedBytesType = Box<fixed_bytes_type::_T0>;
pub mod fixed_bytes_type {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub bytes: FixedTerminal<5usize>,
        pub _1: usize,
    }
}

/// «FixedType» = 'fixed' [ '1'…'9' { '0'…'9' } 'x' '1'…'9' { '0'…'9' } ] ;
pub type FixedType = Box<fixed_type::_T0>;
pub mod fixed_type {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub _0: FixedTerminal<1>,
        pub _1: usize,
        pub _2: FixedTerminal<1>,
        pub _3: FixedTerminal<1>,
        pub _4: usize,
    }
    pub struct _T0 {
        pub fixed: FixedTerminal<5usize>,
        pub _t1: Option<Box<fixed_type::_T1>>,
    }
}

/// «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
pub type HexCharacter = FixedTerminal<1>;

/// «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
pub type IdentifierStart = FixedTerminal<1>;

/// «LineComment» = '//' { ¬( '\u{a}' | '\u{d}' ) } ;
pub type LineComment = Box<line_comment::_T0>;
pub mod line_comment {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub slash_slash: FixedTerminal<2usize>,
        pub _1: usize,
    }
}

/// «NumberUnit» = 'wei' | 'gwei' | 'ether' | 'seconds' | 'minutes' | 'hours' | 'days' | 'weeks' | 'years' ;
pub type NumberUnit = usize;

/// «PragmaDirective» = 'pragma' 1…*{ ¬';' } ';' ;
pub type PragmaDirective = Box<pragma_directive::_T0>;
pub mod pragma_directive {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub pragma: FixedTerminal<6usize>,
        pub not_semicolon_chars: usize,
        pub semicolon_char: FixedTerminal<1>,
    }
}

/// «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
pub type ReservedKeyword = usize;

/// «SignedIntegerType» = 'int' ( '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ) ;
pub type SignedIntegerType = Box<signed_integer_type::_T0>;
pub mod signed_integer_type {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub int: FixedTerminal<3usize>,
        pub _1: usize,
    }
}

/// «Whitespace» = 1…*{ '\u{20}' | '\u{9}' | '\u{d}' | '\u{a}' } ;
pub type Whitespace = usize;

/// «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
pub type YulDecimalNumberLiteral = Box<yul_decimal_number_literal::YulDecimalNumberLiteral>;
pub mod yul_decimal_number_literal {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub _0: FixedTerminal<1>,
        pub _1: usize,
    }
    pub enum YulDecimalNumberLiteral {
        ZeroChar(FixedTerminal<1>),
        _T0(Box<yul_decimal_number_literal::_T0>),
    }
}

/// «YulEVMBuiltinFunctionName» = 'stop' | 'add' | 'sub' | 'mul' | 'div' | 'sdiv' | 'mod' | 'smod' | 'exp' | 'not' | 'lt' | 'gt' | 'slt' | 'sgt' | 'eq' | 'iszero' | 'and' | 'or' | 'xor' | 'byte' | 'shl' | 'shr' | 'sar' | 'addmod' | 'mulmod' | 'signextend' | 'keccak256' | 'pop' | 'mload' | 'mstore' | 'mstore8' | 'sload' | 'sstore' | 'msize' | 'gas' | 'address' | 'balance' | 'selfbalance' | 'caller' | 'callvalue' | 'calldataload' | 'calldatasize' | 'calldatacopy' | 'extcodesize' | 'extcodecopy' | 'returndatasize' | 'returndatacopy' | 'extcodehash' | 'create' | 'create2' | 'call' | 'callcode' | 'delegatecall' | 'staticcall' | 'return' | 'revert' | 'selfdestruct' | 'invalid' | 'log0' | 'log1' | 'log2' | 'log3' | 'log4' | 'chainid' | 'origin' | 'gasprice' | 'Blockhash' | 'coinbase' | 'timestamp' | 'number' | 'difficulty' | 'gaslimit' | 'basefee' ;
pub type YulEvmBuiltinFunctionName = usize;

/// «YulHexLiteral» = '0x' 1…*{ '0'…'9' | 'a'…'f' | 'A'…'F' } ;
pub type YulHexLiteral = Box<yul_hex_literal::_T0>;
pub mod yul_hex_literal {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub zero_x: FixedTerminal<2usize>,
        pub _1: usize,
    }
}

/// «YulKeyword» = 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'if' | 'leave' | 'let' | 'switch' | 'hex' ;
pub type YulKeyword = usize;

/// «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
pub type DecimalExponent = Box<decimal_exponent::_T0>;
pub mod decimal_exponent {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub _0: FixedTerminal<1>,
        pub minus_char: Option<FixedTerminal<1>>,
        pub decimal_integer: DecimalInteger,
    }
}

/// «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
pub type DecimalFloat = Box<decimal_float::_T0>;
pub mod decimal_float {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub decimal_integer_1: Option<DecimalInteger>,
        pub period_char: FixedTerminal<1>,
        pub decimal_integer_2: DecimalInteger,
    }
}

/// «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
pub type HexByteEscape = Box<hex_byte_escape::_T0>;
pub mod hex_byte_escape {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub _0: FixedTerminal<1>,
        pub _1: usize,
    }
}

/// «HexNumber» = '0' 'x' 1…*{ «HexCharacter» / [ '_' ] } ;
pub type HexNumber = Box<hex_number::_T0>;
pub mod hex_number {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<FixedTerminal<1>>,
        pub separators: Vec<Option<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub zero_char: FixedTerminal<1>,
        pub _1: FixedTerminal<1>,
        pub _2: hex_number::_T1,
    }
}

/// «IGNORE» = { «Whitespace» | «Comment» | «LineComment» } ;
pub type Ignore = Vec<Box<ignore::Ignore>>;
pub mod ignore {
    #[allow(unused_imports)]
    use super::*;
    pub enum Ignore {
        Whitespace(Whitespace),
        Comment(Comment),
        LineComment(LineComment),
    }
}

/// «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
pub type IdentifierPart = FixedTerminal<1>;

/// «PossiblySeparatedPairsOfHexDigits» = 1…*{ 2…2*{ «HexCharacter» } / [ '_' ] } ;
pub type PossiblySeparatedPairsOfHexDigits = possibly_separated_pairs_of_hex_digits::_T0;
pub mod possibly_separated_pairs_of_hex_digits {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub elements: Vec<usize>,
        pub separators: Vec<Option<FixedTerminal<1>>>,
    }
}

/// «UfixedType» = 'u' «FixedType» ;
pub type UfixedType = Box<ufixed_type::_T0>;
pub mod ufixed_type {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub _0: FixedTerminal<1>,
        pub fixed_type: FixedType,
    }
}

/// «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
pub type UnicodeEscape = Box<unicode_escape::_T0>;
pub mod unicode_escape {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub _0: FixedTerminal<1>,
        pub _1: usize,
    }
}

/// «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
pub type UnsignedIntegerType = Box<unsigned_integer_type::_T0>;
pub mod unsigned_integer_type {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub _0: FixedTerminal<1>,
        pub signed_integer_type: SignedIntegerType,
    }
}

/// «YulReservedWord» = «YulKeyword» | «YulEVMBuiltinFunctionName» | «BooleanLiteral» ;
pub type YulReservedWord = usize;

/// AddSubExpression = Expression ( '+' | '-' ) Expression ;
pub type AddSubExpression = Expression;
pub mod add_sub_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<1>>,
        pub right: Expression,
    }
}

/// AndExpression = Expression '&&' Expression ;
pub type AndExpression = Expression;
pub mod and_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<2usize>>,
        pub right: Expression,
    }
}

/// AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
pub type AssignmentExpression = Expression;
pub mod assignment_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<usize>,
        pub right: Expression,
    }
}

/// BitAndExpression = Expression '&' Expression ;
pub type BitAndExpression = Expression;
pub mod bit_and_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<1>>,
        pub right: Expression,
    }
}

/// BitOrExpression = Expression '|' Expression ;
pub type BitOrExpression = Expression;
pub mod bit_or_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<1>>,
        pub right: Expression,
    }
}

/// BitXOrExpression = Expression '^' Expression ;
pub type BitXOrExpression = Expression;
pub mod bit_x_or_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<1>>,
        pub right: Expression,
    }
}

/// BreakStatement = 'break' ';' ;
pub type BreakStatement = Box<break_statement::_T0>;
pub mod break_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub r#break: WithNoise<FixedTerminal<5usize>>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// ConditionalExpression = Expression '?' Expression ':' Expression ;
pub type ConditionalExpression = Expression;
pub mod conditional_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub question_char: WithNoise<FixedTerminal<1>>,
        pub expression_1: Expression,
        pub colon_char: WithNoise<FixedTerminal<1>>,
        pub expression_2: Expression,
    }
    pub struct E {
        pub left: Expression,
        pub operator: Box<conditional_expression::_T1>,
    }
}

/// ContinueStatement = 'continue' ';' ;
pub type ContinueStatement = Box<continue_statement::_T0>;
pub mod continue_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub r#continue: WithNoise<FixedTerminal<8usize>>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// DataLocation = 'memory' | 'storage' | 'calldata' ;
pub type DataLocation = WithNoise<usize>;

/// «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
pub type DecimalNumber = Box<decimal_number::_T0>;
pub mod decimal_number {
    #[allow(unused_imports)]
    use super::*;
    pub enum DecimalNumber {
        DecimalInteger(DecimalInteger),
        DecimalFloat(DecimalFloat),
    }
    pub struct _T0 {
        pub decimal_number: Box<decimal_number::DecimalNumber>,
        pub decimal_exponent: Option<DecimalExponent>,
    }
}

/// ElementaryType = 'bool' | 'string' | 'bytes' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | «FixedType» | «UfixedType» ;
pub type ElementaryType = Box<elementary_type::ElementaryType>;
pub mod elementary_type {
    #[allow(unused_imports)]
    use super::*;
    pub enum ElementaryType {
        _0(WithNoise<usize>),
        SignedIntegerType(SignedIntegerType),
        UnsignedIntegerType(UnsignedIntegerType),
        FixedBytesType(FixedBytesType),
        FixedType(FixedType),
        UfixedType(UfixedType),
    }
}

/// EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
pub type EqualityComparisonExpression = Expression;
pub mod equality_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<2usize>>,
        pub right: Expression,
    }
}

/// «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
pub type EscapeSequence = Box<escape_sequence::_T0>;
pub mod escape_sequence {
    #[allow(unused_imports)]
    use super::*;
    pub enum EscapeSequence {
        _0(FixedTerminal<1>),
        HexByteEscape(HexByteEscape),
        UnicodeEscape(UnicodeEscape),
    }
    pub struct _T0 {
        pub backslash_char: FixedTerminal<1>,
        pub escape_sequence: Box<escape_sequence::EscapeSequence>,
    }
}

/// ExponentiationExpression = Expression '**' Expression ;
pub type ExponentiationExpression = Expression;
pub mod exponentiation_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<2usize>>,
        pub right: Expression,
    }
}

/// «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
pub type HexStringLiteral = Box<hex_string_literal::_T0>;
pub mod hex_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub double_quote_char_1: FixedTerminal<1>,
        pub possibly_separated_pairs_of_hex_digits: Option<PossiblySeparatedPairsOfHexDigits>,
        pub double_quote_char_2: FixedTerminal<1>,
    }
    pub struct _T2 {
        pub quote_char_1: FixedTerminal<1>,
        pub possibly_separated_pairs_of_hex_digits: Option<PossiblySeparatedPairsOfHexDigits>,
        pub quote_char_2: FixedTerminal<1>,
    }
    pub enum HexStringLiteral {
        _T1(Box<hex_string_literal::_T1>),
        _T2(Box<hex_string_literal::_T2>),
    }
    pub struct _T0 {
        pub hex: FixedTerminal<3usize>,
        pub hex_string_literal: Box<hex_string_literal::HexStringLiteral>,
    }
}

/// IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
pub type IndexAccessExpression = Expression;
pub mod index_access_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub colon_char: WithNoise<FixedTerminal<1>>,
        pub expression: Option<Expression>,
    }
    pub struct Operator {
        pub open_bracket_char: WithNoise<FixedTerminal<1>>,
        pub expression_2: Option<Expression>,
        pub _t1: Option<Box<index_access_expression::_T1>>,
        pub close_bracket_char: WithNoise<FixedTerminal<1>>,
    }
    pub struct E {
        pub left: Expression,
        pub operator: Box<index_access_expression::Operator>,
    }
}

/// «Keyword» = 'pragma' | 'abstract' | 'anonymous' | 'address' | 'as' | 'assembly' | 'bool' | 'break' | 'bytes' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | 'fixed' | 'ufixed' ;
pub type Keyword = Box<keyword::Keyword>;
pub mod keyword {
    #[allow(unused_imports)]
    use super::*;
    pub enum Keyword {
        _0(usize),
        SignedIntegerType(SignedIntegerType),
        UnsignedIntegerType(UnsignedIntegerType),
        FixedBytesType(FixedBytesType),
        _4(usize),
    }
}

/// MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
pub type MulDivModExpression = Expression;
pub mod mul_div_mod_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<1>>,
        pub right: Expression,
    }
}

/// OrExpression = Expression '||' Expression ;
pub type OrExpression = Expression;
pub mod or_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<2usize>>,
        pub right: Expression,
    }
}

/// OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
pub type OrderComparisonExpression = Expression;
pub mod order_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<usize>,
        pub right: Expression,
    }
}

/// PositionalArgumentList = 1…*{ Expression / ',' } ;
pub type PositionalArgumentList = positional_argument_list::_T0;
pub mod positional_argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub elements: Vec<Expression>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
}

/// «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
pub type RawIdentifier = Box<raw_identifier::_T0>;
pub mod raw_identifier {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub _0: FixedTerminal<1>,
        pub _1: usize,
    }
}

/// ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
pub type ShiftExpression = Expression;
pub mod shift_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<usize>,
        pub right: Expression,
    }
}

/// StateMutabilitySpecifier = 'pure' | 'view' | 'payable' ;
pub type StateMutabilitySpecifier = WithNoise<usize>;

/// UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | 'delete' | '-' ) Expression ;
pub type UnaryPrefixExpression = Expression;
pub mod unary_prefix_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub operator: WithNoise<usize>,
        pub right: Expression,
    }
}

/// UnarySuffixExpression = Expression ( '++' | '--' ) ;
pub type UnarySuffixExpression = Expression;
pub mod unary_suffix_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: WithNoise<FixedTerminal<2usize>>,
    }
}

/// UncheckedBlock = 'unchecked' Block ;
pub type UncheckedBlock = Box<unchecked_block::_T0>;
pub mod unchecked_block {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub unchecked: WithNoise<FixedTerminal<9usize>>,
        pub block: Block,
    }
}

/// VisibilitySpecifier = 'internal' | 'external' | 'private' | 'public' ;
pub type VisibilitySpecifier = WithNoise<usize>;

/// YulBreakStatement = 'break' ;
pub type YulBreakStatement = WithNoise<FixedTerminal<5usize>>;

/// YulContinueStatement = 'continue' ;
pub type YulContinueStatement = WithNoise<FixedTerminal<8usize>>;

/// YulLeaveStatement = 'leave' ;
pub type YulLeaveStatement = WithNoise<FixedTerminal<5usize>>;

/// «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
pub type DoubleQuotedAsciiStringLiteral = Box<double_quoted_ascii_string_literal::_T0>;
pub mod double_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub enum DoubleQuotedAsciiStringLiteral {
        Chars(usize),
        EscapeSequence(EscapeSequence),
    }
    pub struct _T0 {
        pub double_quote_char_1: FixedTerminal<1>,
        pub double_quoted_ascii_string_literals:
            Vec<Box<double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral>>,
        pub double_quote_char_2: FixedTerminal<1>,
    }
}

/// «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
pub type DoubleQuotedUnicodeStringLiteral = Box<double_quoted_unicode_string_literal::_T0>;
pub mod double_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub enum DoubleQuotedUnicodeStringLiteral {
        Chars(usize),
        EscapeSequence(EscapeSequence),
    }
    pub struct _T0 {
        pub unicode_double_quote: FixedTerminal<8usize>,
        pub double_quoted_unicode_string_literals:
            Vec<Box<double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral>>,
        pub double_quote_char: FixedTerminal<1>,
    }
}

/// ElementaryTypeWithPayable = 'address' [ 'payable' ] | ElementaryType ;
pub type ElementaryTypeWithPayable = Box<elementary_type_with_payable::ElementaryTypeWithPayable>;
pub mod elementary_type_with_payable {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub address: WithNoise<FixedTerminal<7usize>>,
        pub payable: Option<WithNoise<FixedTerminal<7usize>>>,
    }
    pub enum ElementaryTypeWithPayable {
        _T0(Box<elementary_type_with_payable::_T0>),
        ElementaryType(ElementaryType),
    }
}

/// ElementaryTypeWithoutPayable = 'address' | ElementaryType ;
pub type ElementaryTypeWithoutPayable =
    Box<elementary_type_without_payable::ElementaryTypeWithoutPayable>;
pub mod elementary_type_without_payable {
    #[allow(unused_imports)]
    use super::*;
    pub enum ElementaryTypeWithoutPayable {
        Address(WithNoise<FixedTerminal<7usize>>),
        ElementaryType(ElementaryType),
    }
}

/// NumericLiteral = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
pub type NumericLiteral = Box<numeric_literal::_T0>;
pub mod numeric_literal {
    #[allow(unused_imports)]
    use super::*;
    pub enum NumericLiteral {
        DecimalNumber(DecimalNumber),
        HexNumber(HexNumber),
    }
    pub struct _T0 {
        pub numeric_literal: Box<numeric_literal::NumericLiteral>,
        pub _1: Option<WithNoise<usize>>,
    }
}

/// «ReservedWord» = «Keyword» | «ReservedKeyword» | «NumberUnit» | «BooleanLiteral» ;
pub type ReservedWord = Box<reserved_word::ReservedWord>;
pub mod reserved_word {
    #[allow(unused_imports)]
    use super::*;
    pub enum ReservedWord {
        Keyword(Keyword),
        _1(usize),
    }
}

/// «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
pub type SingleQuotedAsciiStringLiteral = Box<single_quoted_ascii_string_literal::_T0>;
pub mod single_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub enum SingleQuotedAsciiStringLiteral {
        Chars(usize),
        EscapeSequence(EscapeSequence),
    }
    pub struct _T0 {
        pub quote_char_1: FixedTerminal<1>,
        pub single_quoted_ascii_string_literals:
            Vec<Box<single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral>>,
        pub quote_char_2: FixedTerminal<1>,
    }
}

/// «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
pub type SingleQuotedUnicodeStringLiteral = Box<single_quoted_unicode_string_literal::_T0>;
pub mod single_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub enum SingleQuotedUnicodeStringLiteral {
        Chars(usize),
        EscapeSequence(EscapeSequence),
    }
    pub struct _T0 {
        pub unicode_quote: FixedTerminal<8usize>,
        pub single_quoted_unicode_string_literals:
            Vec<Box<single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral>>,
        pub quote_char: FixedTerminal<1>,
    }
}

/// «YulIdentifier» = «RawIdentifier» - «YulReservedWord» ;
pub type YulIdentifier = RawIdentifier;

/// «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
pub type AsciiStringLiteral = Box<ascii_string_literal::AsciiStringLiteral>;
pub mod ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub enum AsciiStringLiteral {
        SingleQuotedAsciiStringLiteral(SingleQuotedAsciiStringLiteral),
        DoubleQuotedAsciiStringLiteral(DoubleQuotedAsciiStringLiteral),
    }
}

/// AssemblyFlags = '(' 1…*{ «DoubleQuotedAsciiStringLiteral» / ',' } ')' ;
pub type AssemblyFlags = Box<assembly_flags::_T0>;
pub mod assembly_flags {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<DoubleQuotedAsciiStringLiteral>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub double_quoted_ascii_string_literals: assembly_flags::_T1,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
}

/// «Identifier» = «RawIdentifier» - «ReservedWord» ;
pub type Identifier = RawIdentifier;

/// «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
pub type UnicodeStringLiteral = Box<unicode_string_literal::UnicodeStringLiteral>;
pub mod unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    pub enum UnicodeStringLiteral {
        SingleQuotedUnicodeStringLiteral(SingleQuotedUnicodeStringLiteral),
        DoubleQuotedUnicodeStringLiteral(DoubleQuotedUnicodeStringLiteral),
    }
}

/// YulFunctionCall = ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) '(' { YulExpression / ',' } ')' ;
pub type YulFunctionCall = Box<yul_function_call::_T0>;
pub mod yul_function_call {
    #[allow(unused_imports)]
    use super::*;
    pub enum YulFunctionCall {
        YulIdentifier(YulIdentifier),
        _1(WithNoise<usize>),
    }
    pub struct _T1 {
        pub elements: Vec<YulExpression>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub yul_function_call: Box<yul_function_call::YulFunctionCall>,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub yul_expressions: Option<yul_function_call::_T1>,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
}

/// YulFunctionDefinition = 'function' «YulIdentifier» '(' { «YulIdentifier» / ',' } ')' [ '->' 1…*{ «YulIdentifier» / ',' } ] YulBlock ;
pub type YulFunctionDefinition = Box<yul_function_definition::_T0>;
pub mod yul_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<YulIdentifier>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T3 {
        pub elements: Vec<YulIdentifier>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T2 {
        pub minus_greater: WithNoise<FixedTerminal<2usize>>,
        pub yul_identifiers: yul_function_definition::_T3,
    }
    pub struct _T0 {
        pub function: WithNoise<FixedTerminal<8usize>>,
        pub yul_identifier: YulIdentifier,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub yul_identifiers: Option<yul_function_definition::_T1>,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
        pub _t2: Option<Box<yul_function_definition::_T2>>,
        pub yul_block: YulBlock,
    }
}

/// YulPath = «YulIdentifier» { '.' ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) } ;
pub type YulPath = Box<yul_path::_T0>;
pub mod yul_path {
    #[allow(unused_imports)]
    use super::*;
    pub enum YulPath {
        YulIdentifier(YulIdentifier),
        _1(WithNoise<usize>),
    }
    pub struct _T2 {
        pub period_char: WithNoise<FixedTerminal<1>>,
        pub yul_path: Box<yul_path::YulPath>,
    }
    pub struct _T0 {
        pub yul_identifier: YulIdentifier,
        pub _t2s: Vec<Box<yul_path::_T2>>,
    }
}

/// EnumDefinition = 'enum' «Identifier» '{' 1…*{ «Identifier» / ',' } '}' ;
pub type EnumDefinition = Box<enum_definition::_T0>;
pub mod enum_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<Identifier>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub r#enum: WithNoise<FixedTerminal<4usize>>,
        pub identifier: Identifier,
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub identifiers: enum_definition::_T1,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
}

/// IdentifierPath = 1…*{ «Identifier» / '.' } ;
pub type IdentifierPath = identifier_path::_T0;
pub mod identifier_path {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub elements: Vec<Identifier>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
}

/// ImportPath = «AsciiStringLiteral» ;
pub type ImportPath = AsciiStringLiteral;

/// Literal = «AsciiStringLiteral» | «UnicodeStringLiteral» | NumericLiteral | «HexStringLiteral» | «BooleanLiteral» ;
pub type Literal = Box<literal::Literal>;
pub mod literal {
    #[allow(unused_imports)]
    use super::*;
    pub enum Literal {
        AsciiStringLiteral(AsciiStringLiteral),
        UnicodeStringLiteral(UnicodeStringLiteral),
        NumericLiteral(NumericLiteral),
        HexStringLiteral(HexStringLiteral),
        _4(WithNoise<usize>),
    }
}

/// MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
pub type MemberAccessExpression = Expression;
pub mod member_access_expression {
    #[allow(unused_imports)]
    use super::*;
    pub enum MemberAccessExpression {
        Identifier(Identifier),
        Address(WithNoise<FixedTerminal<7usize>>),
    }
    pub struct Operator {
        pub period_char: WithNoise<FixedTerminal<1>>,
        pub member_access_expression: Box<member_access_expression::MemberAccessExpression>,
    }
    pub struct E {
        pub left: Expression,
        pub operator: Box<member_access_expression::Operator>,
    }
}

/// NamedArgument = «Identifier» ':' Expression ;
pub type NamedArgument = Box<named_argument::_T0>;
pub mod named_argument {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub identifier: Identifier,
        pub colon_char: WithNoise<FixedTerminal<1>>,
        pub expression: Expression,
    }
}

/// ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
pub type ParameterDeclaration = Box<parameter_declaration::_T0>;
pub mod parameter_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub type_name: TypeName,
        pub _1: Option<WithNoise<usize>>,
        pub identifier: Option<Identifier>,
    }
}

/// SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
pub type SelectedImport = Box<selected_import::_T0>;
pub mod selected_import {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub r#as: WithNoise<FixedTerminal<2usize>>,
        pub identifier: Identifier,
    }
    pub struct _T0 {
        pub identifier: Identifier,
        pub _t1: Option<Box<selected_import::_T1>>,
    }
}

/// UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryTypeWithPayable ';' ;
pub type UserDefinedValueTypeDefinition = Box<user_defined_value_type_definition::_T0>;
pub mod user_defined_value_type_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub r#type: WithNoise<FixedTerminal<4usize>>,
        pub identifier: Identifier,
        pub is: WithNoise<FixedTerminal<2usize>>,
        pub elementary_type_with_payable: ElementaryTypeWithPayable,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
pub type YulLiteral = Box<yul_literal::YulLiteral>;
pub mod yul_literal {
    #[allow(unused_imports)]
    use super::*;
    pub enum YulLiteral {
        YulDecimalNumberLiteral(YulDecimalNumberLiteral),
        YulHexLiteral(YulHexLiteral),
        AsciiStringLiteral(AsciiStringLiteral),
        _3(WithNoise<usize>),
        HexStringLiteral(HexStringLiteral),
    }
}

/// FunctionCallOptionsExpression = Expression '{' 1…*{ NamedArgument / ',' } '}' ;
pub type FunctionCallOptionsExpression = Expression;
pub mod function_call_options_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<NamedArgument>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct Operator {
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub named_arguments: function_call_options_expression::_T1,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
    pub struct E {
        pub left: Expression,
        pub operator: Box<function_call_options_expression::Operator>,
    }
}

/// MappingType = 'mapping' '(' ( ElementaryTypeWithoutPayable | IdentifierPath ) '=>' TypeName ')' ;
pub type MappingType = Box<mapping_type::_T0>;
pub mod mapping_type {
    #[allow(unused_imports)]
    use super::*;
    pub enum MappingType {
        ElementaryTypeWithoutPayable(ElementaryTypeWithoutPayable),
        IdentifierPath(IdentifierPath),
    }
    pub struct _T0 {
        pub mapping: WithNoise<FixedTerminal<7usize>>,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub mapping_type: Box<mapping_type::MappingType>,
        pub equal_greater: WithNoise<FixedTerminal<2usize>>,
        pub type_name: TypeName,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
}

/// NamedArgumentList = '{' { NamedArgument / ',' } '}' ;
pub type NamedArgumentList = Box<named_argument_list::_T0>;
pub mod named_argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<NamedArgument>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub named_arguments: Option<named_argument_list::_T1>,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
}

/// NonEmptyParameterList = '(' 1…*{ ParameterDeclaration / ',' } ')' ;
pub type NonEmptyParameterList = Box<non_empty_parameter_list::_T0>;
pub mod non_empty_parameter_list {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<ParameterDeclaration>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub parameter_declarations: non_empty_parameter_list::_T1,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
}

/// OverrideSpecifier = 'override' [ '(' 1…1*{ IdentifierPath / ',' } ')' ] ;
pub type OverrideSpecifier = Box<override_specifier::_T0>;
pub mod override_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T2 {
        pub elements: Vec<IdentifierPath>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T1 {
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub identifier_paths: override_specifier::_T2,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
    pub struct _T0 {
        pub r#override: WithNoise<FixedTerminal<8usize>>,
        pub _t1: Option<Box<override_specifier::_T1>>,
    }
}

/// ParameterList = '(' { ParameterDeclaration / ',' } ')' ;
pub type ParameterList = Box<parameter_list::_T0>;
pub mod parameter_list {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<ParameterDeclaration>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub parameter_declarations: Option<parameter_list::_T1>,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
}

/// SelectingImportDirective = '{' 1…*{ SelectedImport / ',' } '}' 'from' ImportPath ;
pub type SelectingImportDirective = Box<selecting_import_directive::_T0>;
pub mod selecting_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<SelectedImport>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub selected_imports: selecting_import_directive::_T1,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
        pub from: WithNoise<FixedTerminal<4usize>>,
        pub import_path: ImportPath,
    }
}

/// SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
pub type SimpleImportDirective = Box<simple_import_directive::_T0>;
pub mod simple_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T2 {
        pub r#as: WithNoise<FixedTerminal<2usize>>,
        pub identifier: Identifier,
    }
    pub struct _T0 {
        pub import_path: ImportPath,
        pub _t2s: Vec<Box<simple_import_directive::_T2>>,
    }
}

/// StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
pub type StarImportDirective = Box<star_import_directive::_T0>;
pub mod star_import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub star_char: WithNoise<FixedTerminal<1>>,
        pub r#as: WithNoise<FixedTerminal<2usize>>,
        pub identifier: Identifier,
        pub from: WithNoise<FixedTerminal<4usize>>,
        pub import_path: ImportPath,
    }
}

/// YulExpression = YulPath | YulFunctionCall | YulLiteral ;
pub type YulExpression = Box<yul_expression::YulExpression>;
pub mod yul_expression {
    #[allow(unused_imports)]
    use super::*;
    pub enum YulExpression {
        YulPath(YulPath),
        YulFunctionCall(YulFunctionCall),
        YulLiteral(YulLiteral),
    }
}

/// ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
pub type ArgumentList = Box<argument_list::_T0>;
pub mod argument_list {
    #[allow(unused_imports)]
    use super::*;
    pub enum ArgumentList {
        PositionalArgumentList(PositionalArgumentList),
        NamedArgumentList(NamedArgumentList),
    }
    pub struct _T0 {
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub argument_list: Option<Box<argument_list::ArgumentList>>,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
}

/// CatchClause = 'catch' [ [ «Identifier» ] NonEmptyParameterList ] Block ;
pub type CatchClause = Box<catch_clause::_T0>;
pub mod catch_clause {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub identifier: Option<Identifier>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    pub struct _T0 {
        pub catch: WithNoise<FixedTerminal<5usize>>,
        pub _t1: Option<Box<catch_clause::_T1>>,
        pub block: Block,
    }
}

/// FunctionType = 'function' ParameterList { VisibilitySpecifier | StateMutabilitySpecifier } [ 'returns' NonEmptyParameterList ] ;
pub type FunctionType = Box<function_type::_T0>;
pub mod function_type {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T2 {
        pub returns: WithNoise<FixedTerminal<7usize>>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    pub struct _T0 {
        pub function: WithNoise<FixedTerminal<8usize>>,
        pub parameter_list: ParameterList,
        pub _2: Vec<WithNoise<usize>>,
        pub _t2: Option<Box<function_type::_T2>>,
    }
}

/// ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
pub type ImportDirective = Box<import_directive::_T0>;
pub mod import_directive {
    #[allow(unused_imports)]
    use super::*;
    pub enum ImportDirective {
        SimpleImportDirective(SimpleImportDirective),
        StarImportDirective(StarImportDirective),
        SelectingImportDirective(SelectingImportDirective),
    }
    pub struct _T0 {
        pub import: WithNoise<FixedTerminal<6usize>>,
        pub import_directive: Box<import_directive::ImportDirective>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// MethodAttribute = 'virtual' | OverrideSpecifier ;
pub type MethodAttribute = Box<method_attribute::MethodAttribute>;
pub mod method_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub enum MethodAttribute {
        Virtual(WithNoise<FixedTerminal<7usize>>),
        OverrideSpecifier(OverrideSpecifier),
    }
}

/// StateVariableAttribute = 'public' | 'private' | 'internal' | 'constant' | OverrideSpecifier | 'immutable' ;
pub type StateVariableAttribute = Box<state_variable_attribute::StateVariableAttribute>;
pub mod state_variable_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub enum StateVariableAttribute {
        _0(WithNoise<usize>),
        OverrideSpecifier(OverrideSpecifier),
        Immutable(WithNoise<FixedTerminal<9usize>>),
    }
}

/// YulAssignment = YulPath ( ':=' YulExpression | 1…*{ ',' YulPath } ':=' YulFunctionCall ) ;
pub type YulAssignment = Box<yul_assignment::_T0>;
pub mod yul_assignment {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub colon_equal: WithNoise<FixedTerminal<2usize>>,
        pub yul_expression: YulExpression,
    }
    pub struct _T4 {
        pub comma_char: WithNoise<FixedTerminal<1>>,
        pub yul_path: YulPath,
    }
    pub struct _T2 {
        pub _t4s: Vec<Box<yul_assignment::_T4>>,
        pub colon_equal: WithNoise<FixedTerminal<2usize>>,
        pub yul_function_call: YulFunctionCall,
    }
    pub enum YulAssignment {
        _T1(Box<yul_assignment::_T1>),
        _T2(Box<yul_assignment::_T2>),
    }
    pub struct _T0 {
        pub yul_path: YulPath,
        pub yul_assignment: Box<yul_assignment::YulAssignment>,
    }
}

/// YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
pub type YulForStatement = Box<yul_for_statement::_T0>;
pub mod yul_for_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub r#for: WithNoise<FixedTerminal<3usize>>,
        pub yul_block_1: YulBlock,
        pub yul_expression: YulExpression,
        pub yul_block_2: YulBlock,
        pub yul_block_3: YulBlock,
    }
}

/// YulIfStatement = 'if' YulExpression YulBlock ;
pub type YulIfStatement = Box<yul_if_statement::_T0>;
pub mod yul_if_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub r#if: WithNoise<FixedTerminal<2usize>>,
        pub yul_expression: YulExpression,
        pub yul_block: YulBlock,
    }
}

/// YulSwitchStatement = 'switch' YulExpression ( 1…*{ 'case' YulLiteral YulBlock } [ 'default' YulBlock ] | 'default' YulBlock ) ;
pub type YulSwitchStatement = Box<yul_switch_statement::_T0>;
pub mod yul_switch_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T3 {
        pub case: WithNoise<FixedTerminal<4usize>>,
        pub yul_literal: YulLiteral,
        pub yul_block: YulBlock,
    }
    pub struct _T4 {
        pub default: WithNoise<FixedTerminal<7usize>>,
        pub yul_block: YulBlock,
    }
    pub struct _T1 {
        pub _t3s: Vec<Box<yul_switch_statement::_T3>>,
        pub _t4: Option<Box<yul_switch_statement::_T4>>,
    }
    pub struct _T5 {
        pub default: WithNoise<FixedTerminal<7usize>>,
        pub yul_block: YulBlock,
    }
    pub enum YulSwitchStatement {
        _T1(Box<yul_switch_statement::_T1>),
        _T5(Box<yul_switch_statement::_T5>),
    }
    pub struct _T0 {
        pub switch: WithNoise<FixedTerminal<6usize>>,
        pub yul_expression: YulExpression,
        pub yul_switch_statement: Box<yul_switch_statement::YulSwitchStatement>,
    }
}

/// YulVariableDeclaration = 'let' «YulIdentifier» [ ':=' YulExpression | [ ',' «YulIdentifier» ] [ ':=' YulFunctionCall ] ] ;
pub type YulVariableDeclaration = Box<yul_variable_declaration::_T0>;
pub mod yul_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub colon_equal: WithNoise<FixedTerminal<2usize>>,
        pub yul_expression: YulExpression,
    }
    pub struct _T3 {
        pub comma_char: WithNoise<FixedTerminal<1>>,
        pub yul_identifier: YulIdentifier,
    }
    pub struct _T4 {
        pub colon_equal: WithNoise<FixedTerminal<2usize>>,
        pub yul_function_call: YulFunctionCall,
    }
    pub struct _T2 {
        pub _t3: Option<Box<yul_variable_declaration::_T3>>,
        pub _t4: Option<Box<yul_variable_declaration::_T4>>,
    }
    pub enum YulVariableDeclaration {
        _T1(Box<yul_variable_declaration::_T1>),
        _T2(Box<yul_variable_declaration::_T2>),
    }
    pub struct _T0 {
        pub r#let: WithNoise<FixedTerminal<3usize>>,
        pub yul_identifier: YulIdentifier,
        pub yul_variable_declaration: Option<Box<yul_variable_declaration::YulVariableDeclaration>>,
    }
}

/// FunctionCallExpression = Expression ArgumentList ;
pub type FunctionCallExpression = Expression;
pub mod function_call_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct E {
        pub left: Expression,
        pub operator: ArgumentList,
    }
}

/// InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
pub type InheritanceSpecifier = Box<inheritance_specifier::_T0>;
pub mod inheritance_specifier {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub identifier_path: IdentifierPath,
        pub argument_list: Option<ArgumentList>,
    }
}

/// ModifierInvocation = IdentifierPath [ ArgumentList ] ;
pub type ModifierInvocation = Box<modifier_invocation::_T0>;
pub mod modifier_invocation {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub identifier_path: IdentifierPath,
        pub argument_list: Option<ArgumentList>,
    }
}

/// TypeName = ( ElementaryTypeWithPayable | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
pub type TypeName = Box<type_name::_T0>;
pub mod type_name {
    #[allow(unused_imports)]
    use super::*;
    pub enum TypeName {
        ElementaryTypeWithPayable(ElementaryTypeWithPayable),
        FunctionType(FunctionType),
        MappingType(MappingType),
        IdentifierPath(IdentifierPath),
    }
    pub struct _T2 {
        pub open_bracket_char: WithNoise<FixedTerminal<1>>,
        pub expression: Option<Expression>,
        pub close_bracket_char: WithNoise<FixedTerminal<1>>,
    }
    pub struct _T0 {
        pub type_name: Box<type_name::TypeName>,
        pub _t2s: Vec<Box<type_name::_T2>>,
    }
}

/// YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignment | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
pub type YulStatement = Box<yul_statement::YulStatement>;
pub mod yul_statement {
    #[allow(unused_imports)]
    use super::*;
    pub enum YulStatement {
        YulBlock(YulBlock),
        YulVariableDeclaration(YulVariableDeclaration),
        YulFunctionDefinition(YulFunctionDefinition),
        YulAssignment(YulAssignment),
        YulFunctionCall(YulFunctionCall),
        YulIfStatement(YulIfStatement),
        YulForStatement(YulForStatement),
        YulSwitchStatement(YulSwitchStatement),
        _8(WithNoise<usize>),
    }
}

/// ConstructorAttribute = ModifierInvocation | 'payable' | 'internal' | 'public' ;
pub type ConstructorAttribute = Box<constructor_attribute::ConstructorAttribute>;
pub mod constructor_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub enum ConstructorAttribute {
        ModifierInvocation(ModifierInvocation),
        _1(WithNoise<usize>),
    }
}

/// ErrorParameter = TypeName [ «Identifier» ] ;
pub type ErrorParameter = Box<error_parameter::_T0>;
pub mod error_parameter {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub type_name: TypeName,
        pub identifier: Option<Identifier>,
    }
}

/// EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
pub type EventParameter = Box<event_parameter::_T0>;
pub mod event_parameter {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub type_name: TypeName,
        pub indexed: Option<WithNoise<FixedTerminal<7usize>>>,
        pub identifier: Option<Identifier>,
    }
}

/// FallbackFunctionAttribute = 'external' | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub type FallbackFunctionAttribute = Box<fallback_function_attribute::FallbackFunctionAttribute>;
pub mod fallback_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub enum FallbackFunctionAttribute {
        _0(WithNoise<usize>),
        ModifierInvocation(ModifierInvocation),
        Virtual(WithNoise<FixedTerminal<7usize>>),
        OverrideSpecifier(OverrideSpecifier),
    }
}

/// FunctionAttribute = VisibilitySpecifier | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub type FunctionAttribute = Box<function_attribute::FunctionAttribute>;
pub mod function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub enum FunctionAttribute {
        _0(WithNoise<usize>),
        ModifierInvocation(ModifierInvocation),
        Virtual(WithNoise<FixedTerminal<7usize>>),
        OverrideSpecifier(OverrideSpecifier),
    }
}

/// InheritanceSpecifierList = 'is' 1…*{ InheritanceSpecifier / ',' } ;
pub type InheritanceSpecifierList = Box<inheritance_specifier_list::_T0>;
pub mod inheritance_specifier_list {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<InheritanceSpecifier>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub is: WithNoise<FixedTerminal<2usize>>,
        pub inheritance_specifiers: inheritance_specifier_list::_T1,
    }
}

/// PrimaryExpression = 'payable' ArgumentList | 'type' '(' TypeName ')' | 'new' TypeName | '(' 1…*{ [ Expression ] / ',' } ')' | '[' 1…*{ Expression / ',' } ']' | «Identifier» | Literal | ElementaryTypeWithoutPayable ;
pub type PrimaryExpression = Expression;
pub mod primary_expression {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub payable: WithNoise<FixedTerminal<7usize>>,
        pub argument_list: ArgumentList,
    }
    pub struct _T1 {
        pub r#type: WithNoise<FixedTerminal<4usize>>,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub type_name: TypeName,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
    pub struct _T2 {
        pub new: WithNoise<FixedTerminal<3usize>>,
        pub type_name: TypeName,
    }
    pub struct _T4 {
        pub elements: Vec<Option<Expression>>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T3 {
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub expressions: primary_expression::_T4,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
    pub struct _T6 {
        pub elements: Vec<Expression>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T5 {
        pub open_bracket_char: WithNoise<FixedTerminal<1>>,
        pub expressions: primary_expression::_T6,
        pub close_bracket_char: WithNoise<FixedTerminal<1>>,
    }
    pub enum PrimaryExpression {
        _T0(Box<primary_expression::_T0>),
        _T1(Box<primary_expression::_T1>),
        _T2(Box<primary_expression::_T2>),
        _T3(Box<primary_expression::_T3>),
        _T5(Box<primary_expression::_T5>),
        Identifier(Identifier),
        Literal(Literal),
        ElementaryTypeWithoutPayable(ElementaryTypeWithoutPayable),
    }
    pub type E = Box<primary_expression::PrimaryExpression>;
}

/// ReceiveFunctionAttribute = 'external' | 'payable' | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub type ReceiveFunctionAttribute = Box<receive_function_attribute::ReceiveFunctionAttribute>;
pub mod receive_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    pub enum ReceiveFunctionAttribute {
        _0(WithNoise<usize>),
        ModifierInvocation(ModifierInvocation),
        Virtual(WithNoise<FixedTerminal<7usize>>),
        OverrideSpecifier(OverrideSpecifier),
    }
}

/// StructDefinition = 'struct' «Identifier» '{' 1…*{ TypeName «Identifier» ';' } '}' ;
pub type StructDefinition = Box<struct_definition::_T0>;
pub mod struct_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T2 {
        pub type_name: TypeName,
        pub identifier: Identifier,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
    pub struct _T0 {
        pub r#struct: WithNoise<FixedTerminal<6usize>>,
        pub identifier: Identifier,
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub _t2s: Vec<Box<struct_definition::_T2>>,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
}

/// UsingDirective = 'using' ( IdentifierPath | '{' 1…*{ IdentifierPath / ',' } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
pub type UsingDirective = Box<using_directive::_T0>;
pub mod using_directive {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T2 {
        pub elements: Vec<IdentifierPath>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T1 {
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub identifier_paths: using_directive::_T2,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
    pub enum UsingDirective {
        IdentifierPath(IdentifierPath),
        _T1(Box<using_directive::_T1>),
    }
    pub enum UsingDirective {
        StarChar(WithNoise<FixedTerminal<1>>),
        TypeName(TypeName),
    }
    pub struct _T0 {
        pub using: WithNoise<FixedTerminal<5usize>>,
        pub using_directive_1: Box<using_directive::UsingDirective>,
        pub r#for: WithNoise<FixedTerminal<3usize>>,
        pub using_directive_2: Box<using_directive::UsingDirective>,
        pub global: Option<WithNoise<FixedTerminal<6usize>>>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// VariableDeclaration = TypeName [ DataLocation ] «Identifier» ;
pub type VariableDeclaration = Box<variable_declaration::_T0>;
pub mod variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub type_name: TypeName,
        pub _1: Option<WithNoise<usize>>,
        pub identifier: Identifier,
    }
}

/// YulBlock = '{' { YulStatement } '}' ;
pub type YulBlock = Box<yul_block::_T0>;
pub mod yul_block {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub yul_statements: Vec<YulStatement>,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
}

/// AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
pub type AssemblyStatement = Box<assembly_statement::_T0>;
pub mod assembly_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub assembly: WithNoise<FixedTerminal<8usize>>,
        pub double_quote_evmasm_double_quote: Option<WithNoise<FixedTerminal<8usize>>>,
        pub assembly_flags: Option<AssemblyFlags>,
        pub yul_block: YulBlock,
    }
}

/// Directive = «PragmaDirective» | ImportDirective | UsingDirective ;
pub type Directive = Box<directive::Directive>;
pub mod directive {
    #[allow(unused_imports)]
    use super::*;
    pub enum Directive {
        PragmaDirective(PragmaDirective),
        ImportDirective(ImportDirective),
        UsingDirective(UsingDirective),
    }
}

/// ErrorDefinition = 'error' «Identifier» '(' { ErrorParameter / ',' } ')' ';' ;
pub type ErrorDefinition = Box<error_definition::_T0>;
pub mod error_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<ErrorParameter>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub error: WithNoise<FixedTerminal<5usize>>,
        pub identifier: Identifier,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub error_parameters: Option<error_definition::_T1>,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// EventDefinition = 'event' «Identifier» '(' { EventParameter / ',' } ')' [ 'anonymous' ] ';' ;
pub type EventDefinition = Box<event_definition::_T0>;
pub mod event_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub elements: Vec<EventParameter>,
        pub separators: Vec<WithNoise<FixedTerminal<1>>>,
    }
    pub struct _T0 {
        pub event: WithNoise<FixedTerminal<5usize>>,
        pub identifier: Identifier,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub event_parameters: Option<event_definition::_T1>,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
        pub anonymous: Option<WithNoise<FixedTerminal<9usize>>>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | FunctionCallOptionsExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
pub type Expression = Box<expression::Expression>;
pub mod expression {
    #[allow(unused_imports)]
    use super::*;
    pub enum Expression {
        AssignmentExpression(assignment_expression::E),
        ConditionalExpression(conditional_expression::E),
        OrExpression(or_expression::E),
        AndExpression(and_expression::E),
        EqualityComparisonExpression(equality_comparison_expression::E),
        OrderComparisonExpression(order_comparison_expression::E),
        BitOrExpression(bit_or_expression::E),
        BitXOrExpression(bit_x_or_expression::E),
        BitAndExpression(bit_and_expression::E),
        ShiftExpression(shift_expression::E),
        AddSubExpression(add_sub_expression::E),
        MulDivModExpression(mul_div_mod_expression::E),
        ExponentiationExpression(exponentiation_expression::E),
        UnarySuffixExpression(unary_suffix_expression::E),
        UnaryPrefixExpression(unary_prefix_expression::E),
        FunctionCallExpression(function_call_expression::E),
        FunctionCallOptionsExpression(function_call_options_expression::E),
        MemberAccessExpression(member_access_expression::E),
        IndexAccessExpression(index_access_expression::E),
        PrimaryExpression(primary_expression::E),
    }
}

/// VariableDeclarationTuple = '(' { ',' } VariableDeclaration { ',' [ VariableDeclaration ] } ')' ;
pub type VariableDeclarationTuple = Box<variable_declaration_tuple::_T0>;
pub mod variable_declaration_tuple {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T3 {
        pub comma_char: WithNoise<FixedTerminal<1>>,
        pub variable_declaration: Option<VariableDeclaration>,
    }
    pub struct _T0 {
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub comma_chars: usize,
        pub variable_declaration: VariableDeclaration,
        pub _t3s: Vec<Box<variable_declaration_tuple::_T3>>,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
    }
}

/// ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
pub type ConstantDefinition = Box<constant_definition::_T0>;
pub mod constant_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub type_name: TypeName,
        pub constant: WithNoise<FixedTerminal<8usize>>,
        pub identifier: Identifier,
        pub equal_char: WithNoise<FixedTerminal<1>>,
        pub expression: Expression,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
pub type DoWhileStatement = Box<do_while_statement::_T0>;
pub mod do_while_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub r#do: WithNoise<FixedTerminal<2usize>>,
        pub statement: Statement,
        pub r#while: WithNoise<FixedTerminal<5usize>>,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub expression: Expression,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// EmitStatement = 'emit' Expression ArgumentList ';' ;
pub type EmitStatement = Box<emit_statement::_T0>;
pub mod emit_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub emit: WithNoise<FixedTerminal<4usize>>,
        pub expression: Expression,
        pub argument_list: ArgumentList,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// ExpressionStatement = Expression ';' ;
pub type ExpressionStatement = Box<expression_statement::_T0>;
pub mod expression_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub expression: Expression,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
pub type IfStatement = Box<if_statement::_T0>;
pub mod if_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub r#else: WithNoise<FixedTerminal<4usize>>,
        pub statement: Statement,
    }
    pub struct _T0 {
        pub r#if: WithNoise<FixedTerminal<2usize>>,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub expression: Expression,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
        pub statement: Statement,
        pub _t1: Option<Box<if_statement::_T1>>,
    }
}

/// ReturnStatement = 'return' [ Expression ] ';' ;
pub type ReturnStatement = Box<return_statement::_T0>;
pub mod return_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub r#return: WithNoise<FixedTerminal<6usize>>,
        pub expression: Option<Expression>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// RevertStatement = 'revert' Expression ArgumentList ';' ;
pub type RevertStatement = Box<revert_statement::_T0>;
pub mod revert_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub revert: WithNoise<FixedTerminal<6usize>>,
        pub expression: Expression,
        pub argument_list: ArgumentList,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
pub type StateVariableDeclaration = Box<state_variable_declaration::_T0>;
pub mod state_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T2 {
        pub equal_char: WithNoise<FixedTerminal<1>>,
        pub expression: Expression,
    }
    pub struct _T0 {
        pub type_name: TypeName,
        pub state_variable_attributes: Vec<StateVariableAttribute>,
        pub identifier: Identifier,
        pub _t2: Option<Box<state_variable_declaration::_T2>>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// TryStatement = 'try' Expression [ 'returns' NonEmptyParameterList ] Block 1…*{ CatchClause } ;
pub type TryStatement = Box<try_statement::_T0>;
pub mod try_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T1 {
        pub returns: WithNoise<FixedTerminal<7usize>>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    pub struct _T0 {
        pub r#try: WithNoise<FixedTerminal<3usize>>,
        pub expression: Expression,
        pub _t1: Option<Box<try_statement::_T1>>,
        pub block: Block,
        pub catch_clauses: Vec<CatchClause>,
    }
}

/// VariableDeclarationStatement = ( VariableDeclaration [ '=' Expression ] | VariableDeclarationTuple '=' Expression ) ';' ;
pub type VariableDeclarationStatement = Box<variable_declaration_statement::_T0>;
pub mod variable_declaration_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T2 {
        pub equal_char: WithNoise<FixedTerminal<1>>,
        pub expression: Expression,
    }
    pub struct _T1 {
        pub variable_declaration: VariableDeclaration,
        pub _t2: Option<Box<variable_declaration_statement::_T2>>,
    }
    pub struct _T3 {
        pub variable_declaration_tuple: VariableDeclarationTuple,
        pub equal_char: WithNoise<FixedTerminal<1>>,
        pub expression: Expression,
    }
    pub enum VariableDeclarationStatement {
        _T1(Box<variable_declaration_statement::_T1>),
        _T3(Box<variable_declaration_statement::_T3>),
    }
    pub struct _T0 {
        pub variable_declaration_statement:
            Box<variable_declaration_statement::VariableDeclarationStatement>,
        pub semicolon_char: WithNoise<FixedTerminal<1>>,
    }
}

/// WhileStatement = 'while' '(' Expression ')' Statement ;
pub type WhileStatement = Box<while_statement::_T0>;
pub mod while_statement {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub r#while: WithNoise<FixedTerminal<5usize>>,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub expression: Expression,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
        pub statement: Statement,
    }
}

/// SimpleStatement = VariableDeclarationStatement | ExpressionStatement ;
pub type SimpleStatement = Box<simple_statement::SimpleStatement>;
pub mod simple_statement {
    #[allow(unused_imports)]
    use super::*;
    pub enum SimpleStatement {
        VariableDeclarationStatement(VariableDeclarationStatement),
        ExpressionStatement(ExpressionStatement),
    }
}

/// ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
pub type ForStatement = Box<for_statement::_T0>;
pub mod for_statement {
    #[allow(unused_imports)]
    use super::*;
    pub enum ForStatement {
        SimpleStatement(SimpleStatement),
        SemicolonChar(WithNoise<FixedTerminal<1>>),
    }
    pub enum ForStatement {
        ExpressionStatement(ExpressionStatement),
        SemicolonChar(WithNoise<FixedTerminal<1>>),
    }
    pub struct _T0 {
        pub r#for: WithNoise<FixedTerminal<3usize>>,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub for_statement_1: Box<for_statement::ForStatement>,
        pub for_statement_2: Box<for_statement::ForStatement>,
        pub expression: Option<Expression>,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
        pub statement: Statement,
    }
}

/// Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | AssemblyStatement ;
pub type Statement = Box<statement::Statement>;
pub mod statement {
    #[allow(unused_imports)]
    use super::*;
    pub enum Statement {
        Block(Block),
        SimpleStatement(SimpleStatement),
        IfStatement(IfStatement),
        ForStatement(ForStatement),
        WhileStatement(WhileStatement),
        DoWhileStatement(DoWhileStatement),
        ContinueStatement(ContinueStatement),
        BreakStatement(BreakStatement),
        TryStatement(TryStatement),
        ReturnStatement(ReturnStatement),
        EmitStatement(EmitStatement),
        RevertStatement(RevertStatement),
        AssemblyStatement(AssemblyStatement),
    }
}

/// Block = '{' { Statement | UncheckedBlock } '}' ;
pub type Block = Box<block::_T0>;
pub mod block {
    #[allow(unused_imports)]
    use super::*;
    pub enum Block {
        Statement(Statement),
        UncheckedBlock(UncheckedBlock),
    }
    pub struct _T0 {
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub blocks: Vec<Box<block::Block>>,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
}

/// ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
pub type ConstructorDefinition = Box<constructor_definition::_T0>;
pub mod constructor_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub constructor: WithNoise<FixedTerminal<11usize>>,
        pub parameter_list: ParameterList,
        pub constructor_attributes: Vec<ConstructorAttribute>,
        pub block: Block,
    }
}

/// FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
pub type FallbackFunctionDefinition = Box<fallback_function_definition::_T0>;
pub mod fallback_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T2 {
        pub returns: WithNoise<FixedTerminal<7usize>>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    pub enum FallbackFunctionDefinition {
        SemicolonChar(WithNoise<FixedTerminal<1>>),
        Block(Block),
    }
    pub struct _T0 {
        pub fallback: WithNoise<FixedTerminal<8usize>>,
        pub parameter_list: ParameterList,
        pub fallback_function_attributes: Vec<FallbackFunctionAttribute>,
        pub _t2: Option<Box<fallback_function_definition::_T2>>,
        pub fallback_function_definition:
            Box<fallback_function_definition::FallbackFunctionDefinition>,
    }
}

/// FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
pub type FunctionDefinition = Box<function_definition::_T0>;
pub mod function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub enum FunctionDefinition {
        Identifier(Identifier),
        _1(WithNoise<usize>),
    }
    pub struct _T2 {
        pub returns: WithNoise<FixedTerminal<7usize>>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    pub enum FunctionDefinition {
        SemicolonChar(WithNoise<FixedTerminal<1>>),
        Block(Block),
    }
    pub struct _T0 {
        pub function: WithNoise<FixedTerminal<8usize>>,
        pub function_definition_1: Box<function_definition::FunctionDefinition>,
        pub parameter_list: ParameterList,
        pub function_attributes: Vec<FunctionAttribute>,
        pub _t2: Option<Box<function_definition::_T2>>,
        pub function_definition_2: Box<function_definition::FunctionDefinition>,
    }
}

/// ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { MethodAttribute } ( ';' | Block ) ;
pub type ModifierDefinition = Box<modifier_definition::_T0>;
pub mod modifier_definition {
    #[allow(unused_imports)]
    use super::*;
    pub enum ModifierDefinition {
        SemicolonChar(WithNoise<FixedTerminal<1>>),
        Block(Block),
    }
    pub struct _T0 {
        pub modifier: WithNoise<FixedTerminal<8usize>>,
        pub identifier: Identifier,
        pub parameter_list: Option<ParameterList>,
        pub method_attributes: Vec<MethodAttribute>,
        pub modifier_definition: Box<modifier_definition::ModifierDefinition>,
    }
}

/// ReceiveFunctionDefinition = 'receive' '(' ')' { ReceiveFunctionAttribute } ( ';' | Block ) ;
pub type ReceiveFunctionDefinition = Box<receive_function_definition::_T0>;
pub mod receive_function_definition {
    #[allow(unused_imports)]
    use super::*;
    pub enum ReceiveFunctionDefinition {
        SemicolonChar(WithNoise<FixedTerminal<1>>),
        Block(Block),
    }
    pub struct _T0 {
        pub receive: WithNoise<FixedTerminal<7usize>>,
        pub open_paren_char: WithNoise<FixedTerminal<1>>,
        pub close_paren_char: WithNoise<FixedTerminal<1>>,
        pub receive_function_attributes: Vec<ReceiveFunctionAttribute>,
        pub receive_function_definition:
            Box<receive_function_definition::ReceiveFunctionDefinition>,
    }
}

/// ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
pub type ContractBodyElement = Box<contract_body_element::ContractBodyElement>;
pub mod contract_body_element {
    #[allow(unused_imports)]
    use super::*;
    pub enum ContractBodyElement {
        UsingDirective(UsingDirective),
        ConstructorDefinition(ConstructorDefinition),
        FunctionDefinition(FunctionDefinition),
        FallbackFunctionDefinition(FallbackFunctionDefinition),
        ReceiveFunctionDefinition(ReceiveFunctionDefinition),
        ModifierDefinition(ModifierDefinition),
        StructDefinition(StructDefinition),
        EnumDefinition(EnumDefinition),
        UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
        EventDefinition(EventDefinition),
        ErrorDefinition(ErrorDefinition),
        StateVariableDeclaration(StateVariableDeclaration),
    }
}

/// ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
pub type ContractDefinition = Box<contract_definition::_T0>;
pub mod contract_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub r#abstract: Option<WithNoise<FixedTerminal<8usize>>>,
        pub contract: WithNoise<FixedTerminal<8usize>>,
        pub identifier: Identifier,
        pub inheritance_specifier_list: Option<InheritanceSpecifierList>,
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub contract_body_elements: Vec<ContractBodyElement>,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
}

/// InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
pub type InterfaceDefinition = Box<interface_definition::_T0>;
pub mod interface_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub interface: WithNoise<FixedTerminal<9usize>>,
        pub identifier: Identifier,
        pub inheritance_specifier_list: Option<InheritanceSpecifierList>,
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub contract_body_elements: Vec<ContractBodyElement>,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
}

/// LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
pub type LibraryDefinition = Box<library_definition::_T0>;
pub mod library_definition {
    #[allow(unused_imports)]
    use super::*;
    pub struct _T0 {
        pub library: WithNoise<FixedTerminal<7usize>>,
        pub identifier: Identifier,
        pub open_brace_char: WithNoise<FixedTerminal<1>>,
        pub contract_body_elements: Vec<ContractBodyElement>,
        pub close_brace_char: WithNoise<FixedTerminal<1>>,
    }
}

/// Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
pub type Definition = Box<definition::Definition>;
pub mod definition {
    #[allow(unused_imports)]
    use super::*;
    pub enum Definition {
        ContractDefinition(ContractDefinition),
        InterfaceDefinition(InterfaceDefinition),
        LibraryDefinition(LibraryDefinition),
        FunctionDefinition(FunctionDefinition),
        ConstantDefinition(ConstantDefinition),
        StructDefinition(StructDefinition),
        EnumDefinition(EnumDefinition),
        UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
        ErrorDefinition(ErrorDefinition),
    }
}

/// SourceUnit = «IGNORE» { Directive | Definition } $ ;
pub type SourceUnit = Box<source_unit::_T0>;
pub mod source_unit {
    #[allow(unused_imports)]
    use super::*;
    pub enum SourceUnit {
        Directive(Directive),
        Definition(Definition),
    }
    pub struct _T0 {
        pub ignore: Ignore,
        pub source_units: Vec<Box<source_unit::SourceUnit>>,
        pub end_marker: (),
    }
}
