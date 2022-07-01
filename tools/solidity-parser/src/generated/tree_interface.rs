#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
pub trait DefaultTest {
    fn is_default(&self) -> bool {
        false
    }
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FixedSizeTerminal<const N: usize>();
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FixedSizeTerminalWithNoise<const N: usize> {
    pub leading: Ignore,
    pub content: FixedSizeTerminal<N>,
    pub trailing: Ignore,
}
pub type VariableSizeTerminal = usize;
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VariableSizeTerminalWithNoise {
    pub leading: Ignore,
    pub content: VariableSizeTerminal,
    pub trailing: Ignore,
}

/// «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
pub type Comment = comment::_T0;
pub mod comment {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub star_chars: usize,
        pub _1: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T2 {
        NotStarChar(FixedSizeTerminal<1>),
        _T3(comment::_T3),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Content {
        pub _t2s: Vec<Box<comment::_T2>>,
        pub star_chars: usize,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub slash_star: FixedSizeTerminal<2usize>,
        pub content: comment::Content,
        pub star_slash: FixedSizeTerminal<2usize>,
    }
}

/// «DecimalInteger» = 1…*{ '0'…'9' / [ '_' ] } ;
pub type DecimalInteger = decimal_integer::_T0;
pub mod decimal_integer {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub elements: Vec<FixedSizeTerminal<1>>,
        pub separators: Vec<Option<FixedSizeTerminal<1>>>,
    }
}

/// «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
pub type FixedBytesType = fixed_bytes_type::_T0;
pub mod fixed_bytes_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub bytes: FixedSizeTerminal<5usize>,
        pub _1: VariableSizeTerminal,
    }
}

/// «FixedType» = 'fixed' [ '1'…'9' { '0'…'9' } 'x' '1'…'9' { '0'…'9' } ] ;
pub type FixedType = fixed_type::_T0;
pub mod fixed_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub _0: FixedSizeTerminal<1>,
        pub _1: usize,
        pub _2: FixedSizeTerminal<1>,
        pub _3: FixedSizeTerminal<1>,
        pub _4: usize,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub fixed: FixedSizeTerminal<5usize>,
        pub _t1: Option<fixed_type::_T1>,
    }
}

/// «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
pub type HexByteEscape = hex_byte_escape::_T0;
pub mod hex_byte_escape {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _0: FixedSizeTerminal<1>,
        pub _1: usize,
    }
}

/// «HexNumber» = '0' 'x' 1…*{ «HexCharacter» / [ '_' ] } ;
pub type HexNumber = hex_number::_T0;
pub mod hex_number {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<FixedSizeTerminal<1>>,
        pub separators: Vec<Option<FixedSizeTerminal<1>>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub zero_char: FixedSizeTerminal<1>,
        pub _1: FixedSizeTerminal<1>,
        pub _2: hex_number::_T1,
    }
}

/// «LineComment» = '//' { ¬( '\u{a}' | '\u{d}' ) } ;
pub type LineComment = line_comment::_T0;
pub mod line_comment {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub slash_slash: FixedSizeTerminal<2usize>,
        pub _1: usize,
    }
}

/// «PossiblySeparatedPairsOfHexDigits» = 1…*{ 2…2*{ «HexCharacter» } / [ '_' ] } ;
pub type PossiblySeparatedPairsOfHexDigits = possibly_separated_pairs_of_hex_digits::_T0;
pub mod possibly_separated_pairs_of_hex_digits {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub elements: Vec<usize>,
        pub separators: Vec<Option<FixedSizeTerminal<1>>>,
    }
}

/// «PragmaDirective» = 'pragma' 1…*{ ¬';' } ';' ;
pub type PragmaDirective = pragma_directive::_T0;
pub mod pragma_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub pragma: FixedSizeTerminal<6usize>,
        pub not_semicolon_chars: usize,
        pub semicolon_char: FixedSizeTerminal<1>,
    }
}

/// «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
pub type RawIdentifier = raw_identifier::_T0;
pub mod raw_identifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _0: FixedSizeTerminal<1>,
        pub _1: usize,
    }
}

/// «SignedIntegerType» = 'int' ( '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ) ;
pub type SignedIntegerType = signed_integer_type::_T0;
pub mod signed_integer_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub int: FixedSizeTerminal<3usize>,
        pub _1: VariableSizeTerminal,
    }
}

/// «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
pub type UnicodeEscape = unicode_escape::_T0;
pub mod unicode_escape {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _0: FixedSizeTerminal<1>,
        pub _1: usize,
    }
}

/// «Whitespace» = 1…*{ '\u{20}' | '\u{9}' | '\u{d}' | '\u{a}' } ;
pub type Whitespace = usize;

/// «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
pub type YulDecimalNumberLiteral = Box<yul_decimal_number_literal::_T0>;
pub mod yul_decimal_number_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub _0: FixedSizeTerminal<1>,
        pub _1: usize,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        ZeroChar(FixedSizeTerminal<1>),
        _T1(yul_decimal_number_literal::_T1),
    }
}

/// «YulHexLiteral» = '0x' 1…*{ '0'…'9' | 'a'…'f' | 'A'…'F' } ;
pub type YulHexLiteral = yul_hex_literal::_T0;
pub mod yul_hex_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub zero_x: FixedSizeTerminal<2usize>,
        pub _1: usize,
    }
}

/// «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
pub type DecimalExponent = decimal_exponent::_T0;
pub mod decimal_exponent {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _0: FixedSizeTerminal<1>,
        pub minus_char: Option<FixedSizeTerminal<1>>,
        pub decimal_integer: DecimalInteger,
    }
}

/// «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
pub type DecimalFloat = decimal_float::_T0;
pub mod decimal_float {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub decimal_integer_1: Option<DecimalInteger>,
        pub period_char: FixedSizeTerminal<1>,
        pub decimal_integer_2: DecimalInteger,
    }
}

/// «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
pub type EscapeSequence = escape_sequence::_T0;
pub mod escape_sequence {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        _0(FixedSizeTerminal<1>),
        HexByteEscape(HexByteEscape),
        UnicodeEscape(UnicodeEscape),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub backslash_char: FixedSizeTerminal<1>,
        pub _t1: Box<escape_sequence::_T1>,
    }
}

/// «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
pub type HexStringLiteral = hex_string_literal::_T0;
pub mod hex_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub double_quote_char_1: FixedSizeTerminal<1>,
        pub possibly_separated_pairs_of_hex_digits: Option<PossiblySeparatedPairsOfHexDigits>,
        pub double_quote_char_2: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub quote_char_1: FixedSizeTerminal<1>,
        pub possibly_separated_pairs_of_hex_digits: Option<PossiblySeparatedPairsOfHexDigits>,
        pub quote_char_2: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(hex_string_literal::_T2),
        _T3(hex_string_literal::_T3),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub hex: FixedSizeTerminal<3usize>,
        pub _t1: Box<hex_string_literal::_T1>,
    }
}

/// «IGNORE» = { «Whitespace» | «Comment» | «LineComment» } ;
pub type Ignore = Vec<Box<ignore::_T1>>;
pub mod ignore {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        Whitespace(Whitespace),
        Comment(Comment),
        LineComment(LineComment),
    }
}

/// «UfixedType» = 'u' «FixedType» ;
pub type UfixedType = ufixed_type::_T0;
pub mod ufixed_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _0: FixedSizeTerminal<1>,
        pub fixed_type: FixedType,
    }
}

/// «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
pub type UnsignedIntegerType = unsigned_integer_type::_T0;
pub mod unsigned_integer_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _0: FixedSizeTerminal<1>,
        pub signed_integer_type: SignedIntegerType,
    }
}

/// «YulIdentifier» = «RawIdentifier» - «YulReservedWord» ;
pub type YulIdentifier = RawIdentifier;

/// BreakStatement = 'break' ';' ;
pub type BreakStatement = break_statement::_T0;
pub mod break_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#break: FixedSizeTerminalWithNoise<5usize>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// ContinueStatement = 'continue' ';' ;
pub type ContinueStatement = continue_statement::_T0;
pub mod continue_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#continue: FixedSizeTerminalWithNoise<8usize>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
pub type DecimalNumber = decimal_number::_T0;
pub mod decimal_number {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        DecimalInteger(DecimalInteger),
        DecimalFloat(DecimalFloat),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<decimal_number::_T1>,
        pub decimal_exponent: Option<DecimalExponent>,
    }
}

/// «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
pub type DoubleQuotedAsciiStringLiteral = double_quoted_ascii_string_literal::_T0;
pub mod double_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Run {
        Chars(usize),
        EscapeSequence(EscapeSequence),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub double_quote_char_1: FixedSizeTerminal<1>,
        pub runs: Vec<Box<double_quoted_ascii_string_literal::Run>>,
        pub double_quote_char_2: FixedSizeTerminal<1>,
    }
}

/// «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
pub type DoubleQuotedUnicodeStringLiteral = double_quoted_unicode_string_literal::_T0;
pub mod double_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Run {
        Chars(usize),
        EscapeSequence(EscapeSequence),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub unicode_double_quote: FixedSizeTerminal<8usize>,
        pub runs: Vec<Box<double_quoted_unicode_string_literal::Run>>,
        pub double_quote_char: FixedSizeTerminal<1>,
    }
}

/// ElementaryType = 'bool' | 'string' | 'bytes' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | «FixedType» | «UfixedType» ;
pub type ElementaryType = Box<elementary_type::_T0>;
pub mod elementary_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        _0(VariableSizeTerminalWithNoise),
        SignedIntegerType(SignedIntegerType),
        UnsignedIntegerType(UnsignedIntegerType),
        FixedBytesType(FixedBytesType),
        FixedType(FixedType),
        UfixedType(UfixedType),
    }
}

/// «Keyword» = 'pragma' | 'abstract' | 'anonymous' | 'address' | 'as' | 'assembly' | 'bool' | 'break' | 'bytes' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | 'fixed' | 'ufixed' ;
pub type Keyword = Box<keyword::_T0>;
pub mod keyword {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        _0(VariableSizeTerminal),
        SignedIntegerType(SignedIntegerType),
        UnsignedIntegerType(UnsignedIntegerType),
        FixedBytesType(FixedBytesType),
        _4(VariableSizeTerminal),
    }
}

/// PositionalArgumentList = 1…*{ Expression / ',' } ;
pub type PositionalArgumentList = positional_argument_list::_T0;
pub mod positional_argument_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub elements: Vec<Expression>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
}

/// «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
pub type SingleQuotedAsciiStringLiteral = single_quoted_ascii_string_literal::_T0;
pub mod single_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Run {
        Chars(usize),
        EscapeSequence(EscapeSequence),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub quote_char_1: FixedSizeTerminal<1>,
        pub runs: Vec<Box<single_quoted_ascii_string_literal::Run>>,
        pub quote_char_2: FixedSizeTerminal<1>,
    }
}

/// «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
pub type SingleQuotedUnicodeStringLiteral = single_quoted_unicode_string_literal::_T0;
pub mod single_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Run {
        Chars(usize),
        EscapeSequence(EscapeSequence),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub unicode_quote: FixedSizeTerminal<8usize>,
        pub runs: Vec<Box<single_quoted_unicode_string_literal::Run>>,
        pub quote_char: FixedSizeTerminal<1>,
    }
}

/// UncheckedBlock = 'unchecked' Block ;
pub type UncheckedBlock = unchecked_block::_T0;
pub mod unchecked_block {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub unchecked: FixedSizeTerminalWithNoise<9usize>,
        pub block: Block,
    }
}

/// YulFunctionCall = ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) '(' { YulExpression / ',' } ')' ;
pub type YulFunctionCall = yul_function_call::_T0;
pub mod yul_function_call {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        YulIdentifier(YulIdentifier),
        _1(VariableSizeTerminalWithNoise),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub elements: Vec<YulExpression>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<yul_function_call::_T1>,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub yul_expressions: Option<yul_function_call::_T2>,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// YulFunctionDefinition = 'function' «YulIdentifier» '(' { «YulIdentifier» / ',' } ')' [ '->' 1…*{ «YulIdentifier» / ',' } ] YulBlock ;
pub type YulFunctionDefinition = yul_function_definition::_T0;
pub mod yul_function_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<YulIdentifier>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub elements: Vec<YulIdentifier>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub minus_greater: FixedSizeTerminalWithNoise<2usize>,
        pub yul_identifiers: yul_function_definition::_T3,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub function: FixedSizeTerminalWithNoise<8usize>,
        pub yul_identifier: YulIdentifier,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub yul_identifiers: Option<yul_function_definition::_T1>,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
        pub _t2: Option<yul_function_definition::_T2>,
        pub yul_block: YulBlock,
    }
}

/// YulPath = «YulIdentifier» { '.' ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) } ;
pub type YulPath = yul_path::_T0;
pub mod yul_path {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T3 {
        YulIdentifier(YulIdentifier),
        _1(VariableSizeTerminalWithNoise),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub period_char: FixedSizeTerminalWithNoise<1>,
        pub _t3: Box<yul_path::_T3>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub yul_identifier: YulIdentifier,
        pub _t2s: Vec<yul_path::_T2>,
    }
}

/// «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
pub type AsciiStringLiteral = Box<ascii_string_literal::_T0>;
pub mod ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        SingleQuotedAsciiStringLiteral(SingleQuotedAsciiStringLiteral),
        DoubleQuotedAsciiStringLiteral(DoubleQuotedAsciiStringLiteral),
    }
}

/// AssemblyFlags = '(' 1…*{ «DoubleQuotedAsciiStringLiteral» / ',' } ')' ;
pub type AssemblyFlags = assembly_flags::_T0;
pub mod assembly_flags {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<DoubleQuotedAsciiStringLiteral>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub double_quoted_ascii_string_literals: assembly_flags::_T1,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// ElementaryTypeWithPayable = 'address' [ 'payable' ] | ElementaryType ;
pub type ElementaryTypeWithPayable = Box<elementary_type_with_payable::_T0>;
pub mod elementary_type_with_payable {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub address: FixedSizeTerminalWithNoise<7usize>,
        pub payable: Option<FixedSizeTerminalWithNoise<7usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        _T1(elementary_type_with_payable::_T1),
        ElementaryType(ElementaryType),
    }
}

/// ElementaryTypeWithoutPayable = 'address' | ElementaryType ;
pub type ElementaryTypeWithoutPayable = Box<elementary_type_without_payable::_T0>;
pub mod elementary_type_without_payable {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        Address(FixedSizeTerminalWithNoise<7usize>),
        ElementaryType(ElementaryType),
    }
}

/// NumericLiteral = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
pub type NumericLiteral = numeric_literal::_T0;
pub mod numeric_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        DecimalNumber(DecimalNumber),
        HexNumber(HexNumber),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<numeric_literal::_T1>,
        pub _1: Option<VariableSizeTerminalWithNoise>,
    }
}

/// «ReservedWord» = «Keyword» | «ReservedKeyword» | «NumberUnit» | «BooleanLiteral» ;
pub type ReservedWord = Box<reserved_word::_T0>;
pub mod reserved_word {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        Keyword(Keyword),
        _1(VariableSizeTerminal),
    }
}

/// «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
pub type UnicodeStringLiteral = Box<unicode_string_literal::_T0>;
pub mod unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        SingleQuotedUnicodeStringLiteral(SingleQuotedUnicodeStringLiteral),
        DoubleQuotedUnicodeStringLiteral(DoubleQuotedUnicodeStringLiteral),
    }
}

/// «Identifier» = «RawIdentifier» - «ReservedWord» ;
pub type Identifier = RawIdentifier;

/// ImportPath = «AsciiStringLiteral» ;
pub type ImportPath = AsciiStringLiteral;

/// Literal = «AsciiStringLiteral» | «UnicodeStringLiteral» | NumericLiteral | «HexStringLiteral» | «BooleanLiteral» ;
pub type Literal = Box<literal::_T0>;
pub mod literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        AsciiStringLiteral(AsciiStringLiteral),
        UnicodeStringLiteral(UnicodeStringLiteral),
        NumericLiteral(NumericLiteral),
        HexStringLiteral(HexStringLiteral),
        _4(VariableSizeTerminalWithNoise),
    }
}

/// YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
pub type YulLiteral = Box<yul_literal::_T0>;
pub mod yul_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        YulDecimalNumberLiteral(YulDecimalNumberLiteral),
        YulHexLiteral(YulHexLiteral),
        AsciiStringLiteral(AsciiStringLiteral),
        _3(VariableSizeTerminalWithNoise),
        HexStringLiteral(HexStringLiteral),
    }
}

/// EnumDefinition = 'enum' «Identifier» '{' 1…*{ «Identifier» / ',' } '}' ;
pub type EnumDefinition = enum_definition::_T0;
pub mod enum_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<Identifier>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#enum: FixedSizeTerminalWithNoise<4usize>,
        pub identifier: Identifier,
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub identifiers: enum_definition::_T1,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// IdentifierPath = 1…*{ «Identifier» / '.' } ;
pub type IdentifierPath = identifier_path::_T0;
pub mod identifier_path {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub elements: Vec<Identifier>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
}

/// NamedArgument = «Identifier» ':' Expression ;
pub type NamedArgument = named_argument::_T0;
pub mod named_argument {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub identifier: Identifier,
        pub colon_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Expression,
    }
}

/// ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
pub type ParameterDeclaration = parameter_declaration::_T0;
pub mod parameter_declaration {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: TypeName,
        pub _1: Option<VariableSizeTerminalWithNoise>,
        pub identifier: Option<Identifier>,
    }
}

/// SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
pub type SelectedImport = selected_import::_T0;
pub mod selected_import {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub r#as: FixedSizeTerminalWithNoise<2usize>,
        pub identifier: Identifier,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub identifier: Identifier,
        pub _t1: Option<selected_import::_T1>,
    }
}

/// SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
pub type SimpleImportDirective = simple_import_directive::_T0;
pub mod simple_import_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub r#as: FixedSizeTerminalWithNoise<2usize>,
        pub identifier: Identifier,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub import_path: ImportPath,
        pub _t2s: Vec<simple_import_directive::_T2>,
    }
}

/// StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
pub type StarImportDirective = star_import_directive::_T0;
pub mod star_import_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub star_char: FixedSizeTerminalWithNoise<1>,
        pub r#as: FixedSizeTerminalWithNoise<2usize>,
        pub identifier: Identifier,
        pub from: FixedSizeTerminalWithNoise<4usize>,
        pub import_path: ImportPath,
    }
}

/// UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryTypeWithPayable ';' ;
pub type UserDefinedValueTypeDefinition = user_defined_value_type_definition::_T0;
pub mod user_defined_value_type_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#type: FixedSizeTerminalWithNoise<4usize>,
        pub identifier: Identifier,
        pub is: FixedSizeTerminalWithNoise<2usize>,
        pub elementary_type_with_payable: ElementaryTypeWithPayable,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// YulExpression = YulPath | YulFunctionCall | YulLiteral ;
pub type YulExpression = Box<yul_expression::_T0>;
pub mod yul_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        YulPath(YulPath),
        YulFunctionCall(YulFunctionCall),
        YulLiteral(YulLiteral),
    }
}

/// MappingType = 'mapping' '(' ( ElementaryTypeWithoutPayable | IdentifierPath ) '=>' TypeName ')' ;
pub type MappingType = mapping_type::_T0;
pub mod mapping_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        ElementaryTypeWithoutPayable(ElementaryTypeWithoutPayable),
        IdentifierPath(IdentifierPath),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub mapping: FixedSizeTerminalWithNoise<7usize>,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub _t1: Box<mapping_type::_T1>,
        pub equal_greater: FixedSizeTerminalWithNoise<2usize>,
        pub type_name: TypeName,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// NamedArgumentList = '{' { NamedArgument / ',' } '}' ;
pub type NamedArgumentList = named_argument_list::_T0;
pub mod named_argument_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<NamedArgument>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub named_arguments: Option<named_argument_list::_T1>,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// NonEmptyParameterList = '(' 1…*{ ParameterDeclaration / ',' } ')' ;
pub type NonEmptyParameterList = non_empty_parameter_list::_T0;
pub mod non_empty_parameter_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<ParameterDeclaration>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub parameter_declarations: non_empty_parameter_list::_T1,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// OverrideSpecifier = 'override' [ '(' 1…1*{ IdentifierPath / ',' } ')' ] ;
pub type OverrideSpecifier = override_specifier::_T0;
pub mod override_specifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub elements: Vec<IdentifierPath>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub identifier_paths: override_specifier::_T2,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#override: FixedSizeTerminalWithNoise<8usize>,
        pub _t1: Option<override_specifier::_T1>,
    }
}

/// ParameterList = '(' { ParameterDeclaration / ',' } ')' ;
pub type ParameterList = parameter_list::_T0;
pub mod parameter_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<ParameterDeclaration>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub parameter_declarations: Option<parameter_list::_T1>,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// SelectingImportDirective = '{' 1…*{ SelectedImport / ',' } '}' 'from' ImportPath ;
pub type SelectingImportDirective = selecting_import_directive::_T0;
pub mod selecting_import_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<SelectedImport>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub selected_imports: selecting_import_directive::_T1,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
        pub from: FixedSizeTerminalWithNoise<4usize>,
        pub import_path: ImportPath,
    }
}

/// YulAssignment = YulPath ( ':=' YulExpression | 1…*{ ',' YulPath } ':=' YulFunctionCall ) ;
pub type YulAssignment = yul_assignment::_T0;
pub mod yul_assignment {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub colon_equal: FixedSizeTerminalWithNoise<2usize>,
        pub yul_expression: YulExpression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T5 {
        pub comma_char: FixedSizeTerminalWithNoise<1>,
        pub yul_path: YulPath,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub _t5s: Vec<yul_assignment::_T5>,
        pub colon_equal: FixedSizeTerminalWithNoise<2usize>,
        pub yul_function_call: YulFunctionCall,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(yul_assignment::_T2),
        _T3(yul_assignment::_T3),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub yul_path: YulPath,
        pub _t1: Box<yul_assignment::_T1>,
    }
}

/// YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
pub type YulForStatement = yul_for_statement::_T0;
pub mod yul_for_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#for: FixedSizeTerminalWithNoise<3usize>,
        pub yul_block_1: YulBlock,
        pub yul_expression: YulExpression,
        pub yul_block_2: YulBlock,
        pub yul_block_3: YulBlock,
    }
}

/// YulIfStatement = 'if' YulExpression YulBlock ;
pub type YulIfStatement = yul_if_statement::_T0;
pub mod yul_if_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#if: FixedSizeTerminalWithNoise<2usize>,
        pub yul_expression: YulExpression,
        pub yul_block: YulBlock,
    }
}

/// YulSwitchStatement = 'switch' YulExpression ( 1…*{ 'case' YulLiteral YulBlock } [ 'default' YulBlock ] | 'default' YulBlock ) ;
pub type YulSwitchStatement = yul_switch_statement::_T0;
pub mod yul_switch_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T4 {
        pub case: FixedSizeTerminalWithNoise<4usize>,
        pub yul_literal: YulLiteral,
        pub yul_block: YulBlock,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T5 {
        pub default: FixedSizeTerminalWithNoise<7usize>,
        pub yul_block: YulBlock,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub _t4s: Vec<yul_switch_statement::_T4>,
        pub _t5: Option<yul_switch_statement::_T5>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T6 {
        pub default: FixedSizeTerminalWithNoise<7usize>,
        pub yul_block: YulBlock,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(yul_switch_statement::_T2),
        _T6(yul_switch_statement::_T6),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub switch: FixedSizeTerminalWithNoise<6usize>,
        pub yul_expression: YulExpression,
        pub _t1: Box<yul_switch_statement::_T1>,
    }
}

/// YulVariableDeclaration = 'let' «YulIdentifier» [ ':=' YulExpression | [ ',' «YulIdentifier» ] [ ':=' YulFunctionCall ] ] ;
pub type YulVariableDeclaration = yul_variable_declaration::_T0;
pub mod yul_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub colon_equal: FixedSizeTerminalWithNoise<2usize>,
        pub yul_expression: YulExpression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T4 {
        pub comma_char: FixedSizeTerminalWithNoise<1>,
        pub yul_identifier: YulIdentifier,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T5 {
        pub colon_equal: FixedSizeTerminalWithNoise<2usize>,
        pub yul_function_call: YulFunctionCall,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub _t4: Option<yul_variable_declaration::_T4>,
        pub _t5: Option<yul_variable_declaration::_T5>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(yul_variable_declaration::_T2),
        _T3(yul_variable_declaration::_T3),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#let: FixedSizeTerminalWithNoise<3usize>,
        pub yul_identifier: YulIdentifier,
        pub _t1: Option<Box<yul_variable_declaration::_T1>>,
    }
}

/// ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
pub type ArgumentList = argument_list::_T0;
pub mod argument_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        PositionalArgumentList(PositionalArgumentList),
        NamedArgumentList(NamedArgumentList),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub _t1: Option<Box<argument_list::_T1>>,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// CatchClause = 'catch' [ [ «Identifier» ] NonEmptyParameterList ] Block ;
pub type CatchClause = catch_clause::_T0;
pub mod catch_clause {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub identifier: Option<Identifier>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub catch: FixedSizeTerminalWithNoise<5usize>,
        pub _t1: Option<catch_clause::_T1>,
        pub block: Block,
    }
}

/// FunctionType = 'function' ParameterList { VisibilitySpecifier | StateMutabilitySpecifier } [ 'returns' NonEmptyParameterList ] ;
pub type FunctionType = function_type::_T0;
pub mod function_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub returns: FixedSizeTerminalWithNoise<7usize>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub function: FixedSizeTerminalWithNoise<8usize>,
        pub parameter_list: ParameterList,
        pub _2: Vec<VariableSizeTerminalWithNoise>,
        pub _t2: Option<function_type::_T2>,
    }
}

/// ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
pub type ImportDirective = import_directive::_T0;
pub mod import_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        SimpleImportDirective(SimpleImportDirective),
        StarImportDirective(StarImportDirective),
        SelectingImportDirective(SelectingImportDirective),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub import: FixedSizeTerminalWithNoise<6usize>,
        pub _t1: Box<import_directive::_T1>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// MethodAttribute = 'virtual' | OverrideSpecifier ;
pub type MethodAttribute = Box<method_attribute::_T0>;
pub mod method_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        Virtual(FixedSizeTerminalWithNoise<7usize>),
        OverrideSpecifier(OverrideSpecifier),
    }
}

/// StateVariableAttribute = 'public' | 'private' | 'internal' | 'constant' | OverrideSpecifier | 'immutable' ;
pub type StateVariableAttribute = Box<state_variable_attribute::_T0>;
pub mod state_variable_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        _0(VariableSizeTerminalWithNoise),
        OverrideSpecifier(OverrideSpecifier),
        Immutable(FixedSizeTerminalWithNoise<9usize>),
    }
}

/// YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignment | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
pub type YulStatement = Box<yul_statement::_T0>;
pub mod yul_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        YulBlock(YulBlock),
        YulVariableDeclaration(YulVariableDeclaration),
        YulFunctionDefinition(YulFunctionDefinition),
        YulAssignment(YulAssignment),
        YulFunctionCall(YulFunctionCall),
        YulIfStatement(YulIfStatement),
        YulForStatement(YulForStatement),
        YulSwitchStatement(YulSwitchStatement),
        _8(VariableSizeTerminalWithNoise),
    }
}

/// InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
pub type InheritanceSpecifier = inheritance_specifier::_T0;
pub mod inheritance_specifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub identifier_path: IdentifierPath,
        pub argument_list: Option<ArgumentList>,
    }
}

/// ModifierInvocation = IdentifierPath [ ArgumentList ] ;
pub type ModifierInvocation = modifier_invocation::_T0;
pub mod modifier_invocation {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub identifier_path: IdentifierPath,
        pub argument_list: Option<ArgumentList>,
    }
}

/// TypeName = ( ElementaryTypeWithPayable | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
pub type TypeName = type_name::_T0;
pub mod type_name {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        ElementaryTypeWithPayable(ElementaryTypeWithPayable),
        FunctionType(FunctionType),
        MappingType(MappingType),
        IdentifierPath(IdentifierPath),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub open_bracket_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Option<Expression>,
        pub close_bracket_char: FixedSizeTerminalWithNoise<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<type_name::_T1>,
        pub _t3s: Vec<type_name::_T3>,
    }
}

/// YulBlock = '{' { YulStatement } '}' ;
pub type YulBlock = yul_block::_T0;
pub mod yul_block {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub yul_statements: Vec<YulStatement>,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
pub type AssemblyStatement = assembly_statement::_T0;
pub mod assembly_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub assembly: FixedSizeTerminalWithNoise<8usize>,
        pub double_quote_evmasm_double_quote: Option<FixedSizeTerminalWithNoise<8usize>>,
        pub assembly_flags: Option<AssemblyFlags>,
        pub yul_block: YulBlock,
    }
}

/// ConstructorAttribute = ModifierInvocation | 'payable' | 'internal' | 'public' ;
pub type ConstructorAttribute = Box<constructor_attribute::_T0>;
pub mod constructor_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        ModifierInvocation(ModifierInvocation),
        _1(VariableSizeTerminalWithNoise),
    }
}

/// ErrorParameter = TypeName [ «Identifier» ] ;
pub type ErrorParameter = error_parameter::_T0;
pub mod error_parameter {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: TypeName,
        pub identifier: Option<Identifier>,
    }
}

/// EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
pub type EventParameter = event_parameter::_T0;
pub mod event_parameter {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: TypeName,
        pub indexed: Option<FixedSizeTerminalWithNoise<7usize>>,
        pub identifier: Option<Identifier>,
    }
}

/// FallbackFunctionAttribute = 'external' | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub type FallbackFunctionAttribute = Box<fallback_function_attribute::_T0>;
pub mod fallback_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        _0(VariableSizeTerminalWithNoise),
        ModifierInvocation(ModifierInvocation),
        Virtual(FixedSizeTerminalWithNoise<7usize>),
        OverrideSpecifier(OverrideSpecifier),
    }
}

/// FunctionAttribute = VisibilitySpecifier | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub type FunctionAttribute = Box<function_attribute::_T0>;
pub mod function_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        _0(VariableSizeTerminalWithNoise),
        ModifierInvocation(ModifierInvocation),
        Virtual(FixedSizeTerminalWithNoise<7usize>),
        OverrideSpecifier(OverrideSpecifier),
    }
}

/// InheritanceSpecifierList = 'is' 1…*{ InheritanceSpecifier / ',' } ;
pub type InheritanceSpecifierList = inheritance_specifier_list::_T0;
pub mod inheritance_specifier_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<InheritanceSpecifier>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub is: FixedSizeTerminalWithNoise<2usize>,
        pub inheritance_specifiers: inheritance_specifier_list::_T1,
    }
}

/// PrimaryExpression = 'payable' ArgumentList | 'type' '(' TypeName ')' | 'new' TypeName | '(' 1…*{ [ Expression ] / ',' } ')' | '[' 1…*{ Expression / ',' } ']' | «Identifier» | Literal | ElementaryTypeWithoutPayable ;
pub type PrimaryExpression = Expression;
pub mod primary_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub payable: FixedSizeTerminalWithNoise<7usize>,
        pub argument_list: ArgumentList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub r#type: FixedSizeTerminalWithNoise<4usize>,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub type_name: TypeName,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub new: FixedSizeTerminalWithNoise<3usize>,
        pub type_name: TypeName,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T5 {
        pub elements: Vec<Option<Expression>>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T4 {
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub expressions: primary_expression::_T5,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T7 {
        pub elements: Vec<Expression>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T6 {
        pub open_bracket_char: FixedSizeTerminalWithNoise<1>,
        pub expressions: primary_expression::_T7,
        pub close_bracket_char: FixedSizeTerminalWithNoise<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        _T1(primary_expression::_T1),
        _T2(primary_expression::_T2),
        _T3(primary_expression::_T3),
        _T4(primary_expression::_T4),
        _T6(primary_expression::_T6),
        Identifier(Identifier),
        Literal(Literal),
        ElementaryTypeWithoutPayable(ElementaryTypeWithoutPayable),
    }
    pub type E = Box<primary_expression::_T0>;
}

/// ReceiveFunctionAttribute = 'external' | 'payable' | ModifierInvocation | 'virtual' | OverrideSpecifier ;
pub type ReceiveFunctionAttribute = Box<receive_function_attribute::_T0>;
pub mod receive_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        _0(VariableSizeTerminalWithNoise),
        ModifierInvocation(ModifierInvocation),
        Virtual(FixedSizeTerminalWithNoise<7usize>),
        OverrideSpecifier(OverrideSpecifier),
    }
}

/// StructDefinition = 'struct' «Identifier» '{' 1…*{ TypeName «Identifier» ';' } '}' ;
pub type StructDefinition = struct_definition::_T0;
pub mod struct_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub type_name: TypeName,
        pub identifier: Identifier,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#struct: FixedSizeTerminalWithNoise<6usize>,
        pub identifier: Identifier,
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub _t2s: Vec<struct_definition::_T2>,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// UsingDirective = 'using' ( IdentifierPath | '{' 1…*{ IdentifierPath / ',' } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
pub type UsingDirective = using_directive::_T0;
pub mod using_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub elements: Vec<IdentifierPath>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub identifier_paths: using_directive::_T3,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        IdentifierPath(IdentifierPath),
        _T2(using_directive::_T2),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T4 {
        StarChar(FixedSizeTerminalWithNoise<1>),
        TypeName(TypeName),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub using: FixedSizeTerminalWithNoise<5usize>,
        pub _t1: Box<using_directive::_T1>,
        pub r#for: FixedSizeTerminalWithNoise<3usize>,
        pub _t4: Box<using_directive::_T4>,
        pub global: Option<FixedSizeTerminalWithNoise<6usize>>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// VariableDeclaration = TypeName [ DataLocation ] «Identifier» ;
pub type VariableDeclaration = variable_declaration::_T0;
pub mod variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: TypeName,
        pub _1: Option<VariableSizeTerminalWithNoise>,
        pub identifier: Identifier,
    }
}

/// Directive = «PragmaDirective» | ImportDirective | UsingDirective ;
pub type Directive = Box<directive::_T0>;
pub mod directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        PragmaDirective(PragmaDirective),
        ImportDirective(ImportDirective),
        UsingDirective(UsingDirective),
    }
}

/// ErrorDefinition = 'error' «Identifier» '(' { ErrorParameter / ',' } ')' ';' ;
pub type ErrorDefinition = error_definition::_T0;
pub mod error_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<ErrorParameter>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub error: FixedSizeTerminalWithNoise<5usize>,
        pub identifier: Identifier,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub error_parameters: Option<error_definition::_T1>,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// EventDefinition = 'event' «Identifier» '(' { EventParameter / ',' } ')' [ 'anonymous' ] ';' ;
pub type EventDefinition = event_definition::_T0;
pub mod event_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<EventParameter>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub event: FixedSizeTerminalWithNoise<5usize>,
        pub identifier: Identifier,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub event_parameters: Option<event_definition::_T1>,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
        pub anonymous: Option<FixedSizeTerminalWithNoise<9usize>>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
pub type IndexAccessExpression = Expression;
pub mod index_access_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub colon_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Option<Expression>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Operator {
        pub open_bracket_char: FixedSizeTerminalWithNoise<1>,
        pub expression_2: Option<Expression>,
        pub _t1: Option<index_access_expression::_T1>,
        pub close_bracket_char: FixedSizeTerminalWithNoise<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: index_access_expression::Operator,
    }
}

/// VariableDeclarationTuple = '(' { ',' } VariableDeclaration { ',' [ VariableDeclaration ] } ')' ;
pub type VariableDeclarationTuple = variable_declaration_tuple::_T0;
pub mod variable_declaration_tuple {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub comma_char: FixedSizeTerminalWithNoise<1>,
        pub variable_declaration: Option<VariableDeclaration>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub comma_chars: usize,
        pub variable_declaration: VariableDeclaration,
        pub _t3s: Vec<variable_declaration_tuple::_T3>,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
pub type MemberAccessExpression = Expression;
pub mod member_access_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        Identifier(Identifier),
        Address(FixedSizeTerminalWithNoise<7usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Operator {
        pub period_char: FixedSizeTerminalWithNoise<1>,
        pub _t1: Box<member_access_expression::_T1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: member_access_expression::Operator,
    }
}

/// FunctionCallOptionsExpression = Expression '{' 1…*{ NamedArgument / ',' } '}' ;
pub type FunctionCallOptionsExpression = Expression;
pub mod function_call_options_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub elements: Vec<NamedArgument>,
        pub separators: Vec<FixedSizeTerminalWithNoise<1>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Operator {
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub named_arguments: function_call_options_expression::_T1,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: function_call_options_expression::Operator,
    }
}

/// FunctionCallExpression = Expression ArgumentList ;
pub type FunctionCallExpression = Expression;
pub mod function_call_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: ArgumentList,
    }
}

/// UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | 'delete' | '-' ) Expression ;
pub type UnaryPrefixExpression = Expression;
pub mod unary_prefix_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub operator: VariableSizeTerminalWithNoise,
        pub right: Expression,
    }
}

/// UnarySuffixExpression = Expression ( '++' | '--' ) ;
pub type UnarySuffixExpression = Expression;
pub mod unary_suffix_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<2usize>,
    }
}

/// ExponentiationExpression = Expression '**' Expression ;
pub type ExponentiationExpression = Expression;
pub mod exponentiation_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<2usize>,
        pub right: Expression,
    }
}

/// MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
pub type MulDivModExpression = Expression;
pub mod mul_div_mod_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<1>,
        pub right: Expression,
    }
}

/// AddSubExpression = Expression ( '+' | '-' ) Expression ;
pub type AddSubExpression = Expression;
pub mod add_sub_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<1>,
        pub right: Expression,
    }
}

/// ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
pub type ShiftExpression = Expression;
pub mod shift_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: VariableSizeTerminalWithNoise,
        pub right: Expression,
    }
}

/// BitAndExpression = Expression '&' Expression ;
pub type BitAndExpression = Expression;
pub mod bit_and_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<1>,
        pub right: Expression,
    }
}

/// BitXOrExpression = Expression '^' Expression ;
pub type BitXOrExpression = Expression;
pub mod bit_x_or_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<1>,
        pub right: Expression,
    }
}

/// BitOrExpression = Expression '|' Expression ;
pub type BitOrExpression = Expression;
pub mod bit_or_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<1>,
        pub right: Expression,
    }
}

/// OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
pub type OrderComparisonExpression = Expression;
pub mod order_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: VariableSizeTerminalWithNoise,
        pub right: Expression,
    }
}

/// EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
pub type EqualityComparisonExpression = Expression;
pub mod equality_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<2usize>,
        pub right: Expression,
    }
}

/// AndExpression = Expression '&&' Expression ;
pub type AndExpression = Expression;
pub mod and_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<2usize>,
        pub right: Expression,
    }
}

/// OrExpression = Expression '||' Expression ;
pub type OrExpression = Expression;
pub mod or_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: FixedSizeTerminalWithNoise<2usize>,
        pub right: Expression,
    }
}

/// ConditionalExpression = Expression '?' Expression ':' Expression ;
pub type ConditionalExpression = Expression;
pub mod conditional_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub question_char: FixedSizeTerminalWithNoise<1>,
        pub expression_1: Expression,
        pub colon_char: FixedSizeTerminalWithNoise<1>,
        pub expression_2: Expression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: conditional_expression::_T1,
    }
}

/// AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
pub type AssignmentExpression = Expression;
pub mod assignment_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct E {
        pub left: Expression,
        pub operator: VariableSizeTerminalWithNoise,
        pub right: Expression,
    }
}

/// Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | FunctionCallOptionsExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
pub type Expression = Box<expression::Expression>;
pub mod expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
pub type ConstantDefinition = constant_definition::_T0;
pub mod constant_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: TypeName,
        pub constant: FixedSizeTerminalWithNoise<8usize>,
        pub identifier: Identifier,
        pub equal_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Expression,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
pub type DoWhileStatement = do_while_statement::_T0;
pub mod do_while_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#do: FixedSizeTerminalWithNoise<2usize>,
        pub statement: Statement,
        pub r#while: FixedSizeTerminalWithNoise<5usize>,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Expression,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// EmitStatement = 'emit' Expression ArgumentList ';' ;
pub type EmitStatement = emit_statement::_T0;
pub mod emit_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub emit: FixedSizeTerminalWithNoise<4usize>,
        pub expression: Expression,
        pub argument_list: ArgumentList,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// ExpressionStatement = Expression ';' ;
pub type ExpressionStatement = expression_statement::_T0;
pub mod expression_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub expression: Expression,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
pub type IfStatement = if_statement::_T0;
pub mod if_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub r#else: FixedSizeTerminalWithNoise<4usize>,
        pub statement: Statement,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#if: FixedSizeTerminalWithNoise<2usize>,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Expression,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
        pub statement: Statement,
        pub _t1: Option<if_statement::_T1>,
    }
}

/// ReturnStatement = 'return' [ Expression ] ';' ;
pub type ReturnStatement = return_statement::_T0;
pub mod return_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#return: FixedSizeTerminalWithNoise<6usize>,
        pub expression: Option<Expression>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// RevertStatement = 'revert' Expression ArgumentList ';' ;
pub type RevertStatement = revert_statement::_T0;
pub mod revert_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub revert: FixedSizeTerminalWithNoise<6usize>,
        pub expression: Expression,
        pub argument_list: ArgumentList,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
pub type StateVariableDeclaration = state_variable_declaration::_T0;
pub mod state_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub equal_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Expression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub type_name: TypeName,
        pub state_variable_attributes: Vec<StateVariableAttribute>,
        pub identifier: Identifier,
        pub _t2: Option<state_variable_declaration::_T2>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// TryStatement = 'try' Expression [ 'returns' NonEmptyParameterList ] Block 1…*{ CatchClause } ;
pub type TryStatement = try_statement::_T0;
pub mod try_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        pub returns: FixedSizeTerminalWithNoise<7usize>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#try: FixedSizeTerminalWithNoise<3usize>,
        pub expression: Expression,
        pub _t1: Option<try_statement::_T1>,
        pub block: Block,
        pub catch_clauses: Vec<CatchClause>,
    }
}

/// VariableDeclarationStatement = ( VariableDeclaration [ '=' Expression ] | VariableDeclarationTuple '=' Expression ) ';' ;
pub type VariableDeclarationStatement = variable_declaration_statement::_T0;
pub mod variable_declaration_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub equal_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Expression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub variable_declaration: VariableDeclaration,
        pub _t3: Option<variable_declaration_statement::_T3>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T4 {
        pub variable_declaration_tuple: VariableDeclarationTuple,
        pub equal_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Expression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(variable_declaration_statement::_T2),
        _T4(variable_declaration_statement::_T4),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<variable_declaration_statement::_T1>,
        pub semicolon_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// WhileStatement = 'while' '(' Expression ')' Statement ;
pub type WhileStatement = while_statement::_T0;
pub mod while_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#while: FixedSizeTerminalWithNoise<5usize>,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub expression: Expression,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
        pub statement: Statement,
    }
}

/// SimpleStatement = VariableDeclarationStatement | ExpressionStatement ;
pub type SimpleStatement = Box<simple_statement::_T0>;
pub mod simple_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        VariableDeclarationStatement(VariableDeclarationStatement),
        ExpressionStatement(ExpressionStatement),
    }
}

/// ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
pub type ForStatement = for_statement::_T0;
pub mod for_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        SimpleStatement(SimpleStatement),
        SemicolonChar(FixedSizeTerminalWithNoise<1>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T2 {
        ExpressionStatement(ExpressionStatement),
        SemicolonChar(FixedSizeTerminalWithNoise<1>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#for: FixedSizeTerminalWithNoise<3usize>,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub _t1: Box<for_statement::_T1>,
        pub _t2: Box<for_statement::_T2>,
        pub expression: Option<Expression>,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
        pub statement: Statement,
    }
}

/// Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | AssemblyStatement ;
pub type Statement = Box<statement::_T0>;
pub mod statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
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
pub type Block = block::_T0;
pub mod block {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T2 {
        Statement(Statement),
        UncheckedBlock(UncheckedBlock),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub _t2s: Vec<Box<block::_T2>>,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
pub type ConstructorDefinition = constructor_definition::_T0;
pub mod constructor_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub constructor: FixedSizeTerminalWithNoise<11usize>,
        pub parameter_list: ParameterList,
        pub constructor_attributes: Vec<ConstructorAttribute>,
        pub block: Block,
    }
}

/// FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
pub type FallbackFunctionDefinition = fallback_function_definition::_T0;
pub mod fallback_function_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        pub returns: FixedSizeTerminalWithNoise<7usize>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T3 {
        SemicolonChar(FixedSizeTerminalWithNoise<1>),
        Block(Block),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub fallback: FixedSizeTerminalWithNoise<8usize>,
        pub parameter_list: ParameterList,
        pub fallback_function_attributes: Vec<FallbackFunctionAttribute>,
        pub _t2: Option<fallback_function_definition::_T2>,
        pub _t3: Box<fallback_function_definition::_T3>,
    }
}

/// FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
pub type FunctionDefinition = function_definition::_T0;
pub mod function_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        Identifier(Identifier),
        _1(VariableSizeTerminalWithNoise),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        pub returns: FixedSizeTerminalWithNoise<7usize>,
        pub non_empty_parameter_list: NonEmptyParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T4 {
        SemicolonChar(FixedSizeTerminalWithNoise<1>),
        Block(Block),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub function: FixedSizeTerminalWithNoise<8usize>,
        pub _t1: Box<function_definition::_T1>,
        pub parameter_list: ParameterList,
        pub function_attributes: Vec<FunctionAttribute>,
        pub _t3: Option<function_definition::_T3>,
        pub _t4: Box<function_definition::_T4>,
    }
}

/// ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { MethodAttribute } ( ';' | Block ) ;
pub type ModifierDefinition = modifier_definition::_T0;
pub mod modifier_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T2 {
        SemicolonChar(FixedSizeTerminalWithNoise<1>),
        Block(Block),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub modifier: FixedSizeTerminalWithNoise<8usize>,
        pub identifier: Identifier,
        pub parameter_list: Option<ParameterList>,
        pub method_attributes: Vec<MethodAttribute>,
        pub _t2: Box<modifier_definition::_T2>,
    }
}

/// ReceiveFunctionDefinition = 'receive' '(' ')' { ReceiveFunctionAttribute } ( ';' | Block ) ;
pub type ReceiveFunctionDefinition = receive_function_definition::_T0;
pub mod receive_function_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T2 {
        SemicolonChar(FixedSizeTerminalWithNoise<1>),
        Block(Block),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub receive: FixedSizeTerminalWithNoise<7usize>,
        pub open_paren_char: FixedSizeTerminalWithNoise<1>,
        pub close_paren_char: FixedSizeTerminalWithNoise<1>,
        pub receive_function_attributes: Vec<ReceiveFunctionAttribute>,
        pub _t2: Box<receive_function_definition::_T2>,
    }
}

/// ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
pub type ContractBodyElement = Box<contract_body_element::_T0>;
pub mod contract_body_element {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
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
pub type ContractDefinition = contract_definition::_T0;
pub mod contract_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub r#abstract: Option<FixedSizeTerminalWithNoise<8usize>>,
        pub contract: FixedSizeTerminalWithNoise<8usize>,
        pub identifier: Identifier,
        pub inheritance_specifier_list: Option<InheritanceSpecifierList>,
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub contract_body_elements: Vec<ContractBodyElement>,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
pub type InterfaceDefinition = interface_definition::_T0;
pub mod interface_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub interface: FixedSizeTerminalWithNoise<9usize>,
        pub identifier: Identifier,
        pub inheritance_specifier_list: Option<InheritanceSpecifierList>,
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub contract_body_elements: Vec<ContractBodyElement>,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
pub type LibraryDefinition = library_definition::_T0;
pub mod library_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub library: FixedSizeTerminalWithNoise<7usize>,
        pub identifier: Identifier,
        pub open_brace_char: FixedSizeTerminalWithNoise<1>,
        pub contract_body_elements: Vec<ContractBodyElement>,
        pub close_brace_char: FixedSizeTerminalWithNoise<1>,
    }
}

/// Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
pub type Definition = Box<definition::_T0>;
pub mod definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
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
pub type SourceUnit = source_unit::_T0;
pub mod source_unit {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T2 {
        Directive(Directive),
        Definition(Definition),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub ignore: Ignore,
        pub _t2s: Vec<Box<source_unit::_T2>>,
        pub end_marker: (),
    }
}
