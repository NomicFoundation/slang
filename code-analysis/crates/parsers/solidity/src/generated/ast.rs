// This file is generated via `cargo build`. Please don't edit by hand.

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
pub struct FixedSizeTerminalWithTrivia<const N: usize> {
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub leading_trivia: LeadingTrivia,
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub terminal: FixedSizeTerminal<N>,
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub trailing_trivia: TrailingTrivia,
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VariableSizeTerminal(pub usize);
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VariableSizeTerminalWithTrivia {
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub leading_trivia: LeadingTrivia,
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub terminal: VariableSizeTerminal,
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub trailing_trivia: TrailingTrivia,
}

/// «BooleanLiteral» = 'true' | 'false' ;
pub type BooleanLiteral = Box<boolean_literal::BooleanLiteral>;
pub mod boolean_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum BooleanLiteral {
        True(FixedSizeTerminal<4usize>),
        False(FixedSizeTerminal<5usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: Box<boolean_literal::BooleanLiteral>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
pub type DecimalInteger = decimal_integer::DecimalInteger;
pub mod decimal_integer {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub underscore: Option<FixedSizeTerminal<1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_2: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DecimalInteger {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_1_repeated: Vec<decimal_integer::Sequence1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: decimal_integer::DecimalInteger,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
pub type EndOfLine = VariableSizeTerminal;
pub mod end_of_line {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
pub type FixedBytesType = fixed_bytes_type::FixedBytesType;
pub mod fixed_bytes_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FixedBytesType {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub bytes: FixedSizeTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub count: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: fixed_bytes_type::FixedBytesType,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
pub type HexByteEscape = hex_byte_escape::HexByteEscape;
pub mod hex_byte_escape {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct HexByteEscape {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub latin_small_letter_x: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0_repeated: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: hex_byte_escape::HexByteEscape,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «HexNumber» = '0x' «HexCharacter» { [ '_' ] «HexCharacter» } ;
pub type HexNumber = hex_number::HexNumber;
pub mod hex_number {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub underscore: Option<FixedSizeTerminal<1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_3: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_1: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_2_repeated: Vec<hex_number::Sequence2>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct HexNumber {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub zero_x: FixedSizeTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: hex_number::Sequence0,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: hex_number::HexNumber,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «MultilineComment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
pub type MultilineComment = multiline_comment::MultilineComment;
pub mod multiline_comment {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_repeated: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_2: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        NotStar(FixedSizeTerminal<1>),
        Sequence1(multiline_comment::Sequence1),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Content {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub choices_0_repeated: Vec<Box<multiline_comment::Choices0>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_repeated: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct MultilineComment {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub slash_star: FixedSizeTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: multiline_comment::Content,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_slash: FixedSizeTerminal<2usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: multiline_comment::MultilineComment,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
pub type NumberUnit = Box<number_unit::NumberUnit>;
pub mod number_unit {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum NumberUnit {
        Days(FixedSizeTerminal<4usize>),
        Ether(FixedSizeTerminal<5usize>),
        Finney(FixedSizeTerminal<6usize>),
        Gwei(FixedSizeTerminal<4usize>),
        Hours(FixedSizeTerminal<5usize>),
        Minutes(FixedSizeTerminal<7usize>),
        Seconds(FixedSizeTerminal<7usize>),
        Szabo(FixedSizeTerminal<5usize>),
        Weeks(FixedSizeTerminal<5usize>),
        Wei(FixedSizeTerminal<3usize>),
        Years(FixedSizeTerminal<5usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: Box<number_unit::NumberUnit>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
pub type PossiblySeparatedPairsOfHexDigits =
    possibly_separated_pairs_of_hex_digits::PossiblySeparatedPairsOfHexDigits;
pub mod possibly_separated_pairs_of_hex_digits {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub underscore: Option<FixedSizeTerminal<1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_2_repeated: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PossiblySeparatedPairsOfHexDigits {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0_repeated: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_1_repeated: Vec<possibly_separated_pairs_of_hex_digits::Sequence1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: possibly_separated_pairs_of_hex_digits::PossiblySeparatedPairsOfHexDigits,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
pub type RawIdentifier = raw_identifier::RawIdentifier;
pub mod raw_identifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RawIdentifier {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_1_repeated: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: raw_identifier::RawIdentifier,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
pub type ReservedKeyword = VariableSizeTerminal;
pub mod reserved_keyword {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
pub type SignedFixedType = signed_fixed_type::SignedFixedType;
pub mod signed_fixed_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_1_repeated: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub latin_small_letter_x: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_2_repeated: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SignedFixedType {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub fixed: FixedSizeTerminal<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<signed_fixed_type::Sequence0>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: signed_fixed_type::SignedFixedType,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
pub type SignedIntegerType = signed_integer_type::SignedIntegerType;
pub mod signed_integer_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SignedIntegerType {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub int: FixedSizeTerminal<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub byte_count: Option<VariableSizeTerminal>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: signed_integer_type::SignedIntegerType,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
pub type SingleLineComment = single_line_comment::SingleLineComment;
pub mod single_line_comment {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SingleLineComment {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub slash_slash: FixedSizeTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0_repeated: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: single_line_comment::SingleLineComment,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
pub type UnicodeEscape = unicode_escape::UnicodeEscape;
pub mod unicode_escape {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UnicodeEscape {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub latin_small_letter_u: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0_repeated: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: unicode_escape::UnicodeEscape,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
pub type VersionPragmaOperator = Box<version_pragma_operator::VersionPragmaOperator>;
pub mod version_pragma_operator {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum VersionPragmaOperator {
        Caret(FixedSizeTerminal<1>),
        Tilde(FixedSizeTerminal<1>),
        Equal(FixedSizeTerminal<1>),
        Less(FixedSizeTerminal<1>),
        Greater(FixedSizeTerminal<1>),
        LessEqual(FixedSizeTerminal<2usize>),
        GreaterEqual(FixedSizeTerminal<2usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: Box<version_pragma_operator::VersionPragmaOperator>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' }  { '.' 1…*{ '0'…'9' | 'x' | 'X' | '*' } } ;
pub type VersionPragmaValue = version_pragma_value::VersionPragmaValue;
pub mod version_pragma_value {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct VersionPragmaValue {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0_repeated_repeated: Vec<VariableSizeTerminal>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_repeated: Vec<FixedSizeTerminal<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: version_pragma_value::VersionPragmaValue,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
pub type Whitespace = VariableSizeTerminal;
pub mod whitespace {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
pub type YulDecimalNumberLiteral = Box<yul_decimal_number_literal::YulDecimalNumberLiteral>;
pub mod yul_decimal_number_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_1: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_2_repeated: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum YulDecimalNumberLiteral {
        Zero(FixedSizeTerminal<1>),
        Sequence0(yul_decimal_number_literal::Sequence0),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: Box<yul_decimal_number_literal::YulDecimalNumberLiteral>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
pub type YulHexLiteral = yul_hex_literal::YulHexLiteral;
pub mod yul_hex_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulHexLiteral {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub zero_x: FixedSizeTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0_repeated: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: yul_hex_literal::YulHexLiteral,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
pub type DecimalExponent = decimal_exponent::DecimalExponent;
pub mod decimal_exponent {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DecimalExponent {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub minus: Option<FixedSizeTerminal<1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_integer: DecimalInteger,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: decimal_exponent::DecimalExponent,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
pub type DecimalFloat = decimal_float::DecimalFloat;
pub mod decimal_float {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DecimalFloat {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_integer_0_: Option<DecimalInteger>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_integer_1_: DecimalInteger,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: decimal_float::DecimalFloat,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «EndOfFileTrivia» = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
pub type EndOfFileTrivia = Vec<Box<end_of_file_trivia::Choices0>>;
pub mod end_of_file_trivia {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Whitespace(Whitespace),
        MultilineComment(MultilineComment),
        SingleLineComment(SingleLineComment),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: Vec<Box<end_of_file_trivia::Choices0>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
pub type EscapeSequence = escape_sequence::EscapeSequence;
pub mod escape_sequence {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Filter1(FixedSizeTerminal<1>),
        HexByteEscape(HexByteEscape),
        UnicodeEscape(UnicodeEscape),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EscapeSequence {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub backslash: FixedSizeTerminal<1>,
        pub choices_0: Box<escape_sequence::Choices0>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: escape_sequence::EscapeSequence,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
pub type HexStringLiteral = hex_string_literal::HexStringLiteral;
pub mod hex_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DoubleQuoteAndPossiblySeparatedPairsOfHexDigitsAndDoubleQuote {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub opening_double_quote: FixedSizeTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub possibly_separated_pairs_of_hex_digits: Option<PossiblySeparatedPairsOfHexDigits>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub closing_double_quote: FixedSizeTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct QuoteAndPossiblySeparatedPairsOfHexDigitsAndQuote {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub opening_quote: FixedSizeTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub possibly_separated_pairs_of_hex_digits: Option<PossiblySeparatedPairsOfHexDigits>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub closing_quote: FixedSizeTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        DoubleQuoteAndPossiblySeparatedPairsOfHexDigitsAndDoubleQuote(
            hex_string_literal::DoubleQuoteAndPossiblySeparatedPairsOfHexDigitsAndDoubleQuote,
        ),
        QuoteAndPossiblySeparatedPairsOfHexDigitsAndQuote(
            hex_string_literal::QuoteAndPossiblySeparatedPairsOfHexDigitsAndQuote,
        ),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct HexStringLiteral {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub hex: FixedSizeTerminal<3usize>,
        pub choices_0: Box<hex_string_literal::Choices0>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: hex_string_literal::HexStringLiteral,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «LeadingTrivia» = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
pub type LeadingTrivia = Vec<Box<leading_trivia::Choices0>>;
pub mod leading_trivia {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Whitespace(Whitespace),
        EndOfLine(EndOfLine),
        MultilineComment(MultilineComment),
        SingleLineComment(SingleLineComment),
    }
}

/// «TrailingTrivia» = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
pub type TrailingTrivia = Option<trailing_trivia::TrailingTrivia>;
pub mod trailing_trivia {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Whitespace(Whitespace),
        MultilineComment(MultilineComment),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices1 {
        EndOfLine(EndOfLine),
        SingleLineComment(SingleLineComment),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct TrailingTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub choices_0_repeated: Vec<Box<trailing_trivia::Choices0>>,
        pub choices_1: Box<trailing_trivia::Choices1>,
    }
}

/// «UnsignedFixedType» = 'u' «SignedFixedType» ;
pub type UnsignedFixedType = unsigned_fixed_type::UnsignedFixedType;
pub mod unsigned_fixed_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UnsignedFixedType {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub latin_small_letter_u: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub signed_fixed_type: SignedFixedType,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: unsigned_fixed_type::UnsignedFixedType,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
pub type UnsignedIntegerType = unsigned_integer_type::UnsignedIntegerType;
pub mod unsigned_integer_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UnsignedIntegerType {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub latin_small_letter_u: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub signed_integer_type: SignedIntegerType,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: unsigned_integer_type::UnsignedIntegerType,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
pub type YulKeyword = Box<yul_keyword::YulKeyword>;
pub mod yul_keyword {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum YulKeyword {
        BooleanLiteral(BooleanLiteral),
        Terminal0(VariableSizeTerminal),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: Box<yul_keyword::YulKeyword>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// AddressType = 'address' [ 'payable' ] ;
pub type AddressType = address_type::AddressType;
pub mod address_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AddressType {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub address: FixedSizeTerminalWithTrivia<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub payable: Option<FixedSizeTerminalWithTrivia<7usize>>,
    }
}

/// ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
pub type ArrayLiteral = array_literal::ArrayLiteral;
pub mod array_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ExpressionRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression_repeated: Vec<Expression>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ArrayLiteral {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression_repeated_and_comma_repeated:
            array_literal::ExpressionRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket: FixedSizeTerminalWithTrivia<1usize>,
    }
}

/// BreakStatement = 'break' ';' ;
pub type BreakStatement = break_statement::BreakStatement;
pub mod break_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct BreakStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#break: FixedSizeTerminalWithTrivia<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// ContinueStatement = 'continue' ';' ;
pub type ContinueStatement = continue_statement::ContinueStatement;
pub mod continue_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ContinueStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#continue: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// DataLocation = 'memory' | 'storage' | 'calldata' ;
pub type DataLocation = Box<data_location::DataLocation>;
pub mod data_location {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DataLocation {
        Memory(FixedSizeTerminalWithTrivia<6usize>),
        Storage(FixedSizeTerminalWithTrivia<7usize>),
        Calldata(FixedSizeTerminalWithTrivia<8usize>),
    }
}

/// «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
pub type DecimalNumber = decimal_number::DecimalNumber;
pub mod decimal_number {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        DecimalInteger(DecimalInteger),
        DecimalFloat(DecimalFloat),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DecimalNumber {
        pub choices_0: Box<decimal_number::Choices0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub decimal_exponent: Option<DecimalExponent>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: decimal_number::DecimalNumber,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
pub type DoubleQuotedAsciiStringLiteral =
    double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral;
pub mod double_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Run {
        CharRepeated(VariableSizeTerminal),
        EscapeSequence(EscapeSequence),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DoubleQuotedAsciiStringLiteral {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub opening_double_quote: FixedSizeTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub run_repeated: Vec<Box<double_quoted_ascii_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub closing_double_quote: FixedSizeTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
pub type DoubleQuotedUnicodeStringLiteral =
    double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral;
pub mod double_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Run {
        CharRepeated(VariableSizeTerminal),
        EscapeSequence(EscapeSequence),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DoubleQuotedUnicodeStringLiteral {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub unicode_double_quote: FixedSizeTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub run_repeated: Vec<Box<double_quoted_unicode_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote: FixedSizeTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
pub type Keyword = Box<keyword::Keyword>;
pub mod keyword {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Keyword {
        BooleanLiteral(BooleanLiteral),
        FixedBytesType(FixedBytesType),
        NumberUnit(NumberUnit),
        ReservedKeyword(ReservedKeyword),
        SignedIntegerType(SignedIntegerType),
        UnsignedIntegerType(UnsignedIntegerType),
        Terminal0(VariableSizeTerminal),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: Box<keyword::Keyword>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// ParenthesisExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
pub type ParenthesisExpression = parenthesis_expression::ParenthesisExpression;
pub mod parenthesis_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ExpressionRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression_repeated: Vec<Option<Expression>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ParenthesisExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression_repeated_and_comma_repeated:
            parenthesis_expression::ExpressionRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
}

/// PositionalArgumentList = Expression  { ',' Expression } ;
pub type PositionalArgumentList = positional_argument_list::PositionalArgumentList;
pub mod positional_argument_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PositionalArgumentList {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression_repeated: Vec<Expression>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
}

/// «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
pub type SingleQuotedAsciiStringLiteral =
    single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral;
pub mod single_quoted_ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Run {
        CharRepeated(VariableSizeTerminal),
        EscapeSequence(EscapeSequence),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SingleQuotedAsciiStringLiteral {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub opening_quote: FixedSizeTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub run_repeated: Vec<Box<single_quoted_ascii_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub closing_quote: FixedSizeTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
pub type SingleQuotedUnicodeStringLiteral =
    single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral;
pub mod single_quoted_unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Run {
        CharRepeated(VariableSizeTerminal),
        EscapeSequence(EscapeSequence),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SingleQuotedUnicodeStringLiteral {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub unicode_quote: FixedSizeTerminal<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub run_repeated: Vec<Box<single_quoted_unicode_string_literal::Run>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote: FixedSizeTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// UncheckedBlock = 'unchecked' Block ;
pub type UncheckedBlock = unchecked_block::UncheckedBlock;
pub mod unchecked_block {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UncheckedBlock {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub unchecked: FixedSizeTerminalWithTrivia<9usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: Block,
    }
}

/// VersionPragmaSpecifier = 'solidity' 1…*{ «VersionPragmaOperator» «VersionPragmaValue» } ;
pub type VersionPragmaSpecifier = version_pragma_specifier::VersionPragmaSpecifier;
pub mod version_pragma_specifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        pub version_pragma_operator: version_pragma_operator::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub version_pragma_value: version_pragma_value::WithTrivia,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct VersionPragmaSpecifier {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub solidity: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0_repeated: Vec<version_pragma_specifier::Sequence0>,
    }
}

/// YulBreakStatement = 'break' ;
pub type YulBreakStatement = FixedSizeTerminalWithTrivia<5usize>;

/// YulContinueStatement = 'continue' ;
pub type YulContinueStatement = FixedSizeTerminalWithTrivia<8usize>;

/// «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
pub type YulIdentifier = RawIdentifier;
pub mod yul_identifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: RawIdentifier,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// YulLeaveStatement = 'leave' ;
pub type YulLeaveStatement = FixedSizeTerminalWithTrivia<5usize>;

/// «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
pub type AsciiStringLiteral = Box<ascii_string_literal::AsciiStringLiteral>;
pub mod ascii_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AsciiStringLiteral {
        SingleQuotedAsciiStringLiteral(SingleQuotedAsciiStringLiteral),
        DoubleQuotedAsciiStringLiteral(DoubleQuotedAsciiStringLiteral),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: Box<ascii_string_literal::AsciiStringLiteral>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
pub type AssemblyFlags = assembly_flags::AssemblyFlags;
pub mod assembly_flags {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DoubleQuotedAsciiStringLiteralRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quoted_ascii_string_literal_repeated:
            Vec<double_quoted_ascii_string_literal::WithTrivia>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AssemblyFlags {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quoted_ascii_string_literal_repeated_and_comma_repeated:
            assembly_flags::DoubleQuotedAsciiStringLiteralRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
}

/// ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
pub type ElementaryType = Box<elementary_type::ElementaryType>;
pub mod elementary_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ElementaryType {
        Bool(FixedSizeTerminalWithTrivia<4usize>),
        String(FixedSizeTerminalWithTrivia<6usize>),
        AddressType(AddressType),
        FixedBytesType(fixed_bytes_type::WithTrivia),
        SignedIntegerType(signed_integer_type::WithTrivia),
        UnsignedIntegerType(unsigned_integer_type::WithTrivia),
        SignedFixedType(signed_fixed_type::WithTrivia),
        UnsignedFixedType(unsigned_fixed_type::WithTrivia),
    }
}

/// «Identifier» = «RawIdentifier» - «Keyword» ;
pub type Identifier = RawIdentifier;
pub mod identifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub terminal: RawIdentifier,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «NumericLiteral» = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
pub type NumericLiteral = numeric_literal::NumericLiteral;
pub mod numeric_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        DecimalNumber(DecimalNumber),
        HexNumber(HexNumber),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct NumericLiteral {
        pub choices_0: Box<numeric_literal::Choices0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub number_unit: Option<NumberUnit>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: numeric_literal::NumericLiteral,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
pub type UnicodeStringLiteral = Box<unicode_string_literal::UnicodeStringLiteral>;
pub mod unicode_string_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum UnicodeStringLiteral {
        SingleQuotedUnicodeStringLiteral(SingleQuotedUnicodeStringLiteral),
        DoubleQuotedUnicodeStringLiteral(DoubleQuotedUnicodeStringLiteral),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        pub terminal: Box<unicode_string_literal::UnicodeStringLiteral>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: TrailingTrivia,
    }
}

/// YulFunctionCall = «YulIdentifier» '(' [ YulExpression  { ',' YulExpression } ] ')' ;
pub type YulFunctionCall = yul_function_call::YulFunctionCall;
pub mod yul_function_call {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulExpressionRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_expression_repeated: Vec<YulExpression>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndYulExpressionRepeatedAndCommaRepeatedAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_expression_repeated_and_comma_repeated:
            Option<yul_function_call::YulExpressionRepeatedAndCommaRepeated>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulFunctionCall {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_and_yul_expression_repeated_and_comma_repeated_and_close_paren:
            yul_function_call::OpenParenAndYulExpressionRepeatedAndCommaRepeatedAndCloseParen,
    }
}

/// YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
pub type YulFunctionDefinition = yul_function_definition::YulFunctionDefinition;
pub mod yul_function_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Arguments {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier_repeated: Vec<yul_identifier::WithTrivia>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndArgumentsAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub arguments: Option<yul_function_definition::Arguments>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Results {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier_repeated: Vec<yul_identifier::WithTrivia>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub minus_greater: FixedSizeTerminalWithTrivia<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub results: yul_function_definition::Results,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulFunctionDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier: yul_identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_and_arguments_and_close_paren:
            yul_function_definition::OpenParenAndArgumentsAndCloseParen,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<yul_function_definition::Sequence0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: YulBlock,
    }
}

/// YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
pub type YulIdentifierPath = yul_identifier_path::YulIdentifierPath;
pub mod yul_identifier_path {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulIdentifierPath {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier_repeated: Vec<yul_identifier::WithTrivia>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
}

/// ABICoderPragmaSpecifier = 'abicoder' «Identifier» ;
pub type AbiCoderPragmaSpecifier = abi_coder_pragma_specifier::AbiCoderPragmaSpecifier;
pub mod abi_coder_pragma_specifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AbiCoderPragmaSpecifier {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub abicoder: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
    }
}

/// DeleteStatement = 'delete' «Identifier» ';' ;
pub type DeleteStatement = delete_statement::DeleteStatement;
pub mod delete_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DeleteStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub delete: FixedSizeTerminalWithTrivia<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
pub type EnumDefinition = enum_definition::EnumDefinition;
pub mod enum_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct IdentifierRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_repeated: Vec<identifier::WithTrivia>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBraceAndIdentifierRepeatedAndCommaRepeatedAndCloseBrace {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_repeated_and_comma_repeated:
            enum_definition::IdentifierRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EnumDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#enum: FixedSizeTerminalWithTrivia<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_and_identifier_repeated_and_comma_repeated_and_close_brace:
            enum_definition::OpenBraceAndIdentifierRepeatedAndCommaRepeatedAndCloseBrace,
    }
}

/// ExperimentalPragmaSpecifier = 'experimental' «Identifier» ;
pub type ExperimentalPragmaSpecifier = experimental_pragma_specifier::ExperimentalPragmaSpecifier;
pub mod experimental_pragma_specifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ExperimentalPragmaSpecifier {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub experimental: FixedSizeTerminalWithTrivia<12usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
    }
}

/// IdentifierPath = «Identifier»  { '.' «Identifier» } ;
pub type IdentifierPath = identifier_path::IdentifierPath;
pub mod identifier_path {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct IdentifierPath {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_repeated: Vec<identifier::WithTrivia>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
}

/// ImportPath = «AsciiStringLiteral» ;
pub type ImportPath = ascii_string_literal::WithTrivia;

/// NamedArgument = «Identifier» ':' Expression ;
pub type NamedArgument = named_argument::NamedArgument;
pub mod named_argument {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct NamedArgument {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon: FixedSizeTerminalWithTrivia<1>,
        pub expression: Expression,
    }
}

/// ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
pub type ParameterDeclaration = parameter_declaration::ParameterDeclaration;
pub mod parameter_declaration {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ParameterDeclaration {
        pub type_name: TypeName,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub data_location: Option<DataLocation>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::WithTrivia>,
    }
}

/// SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
pub type SelectedImport = selected_import::SelectedImport;
pub mod selected_import {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#as: FixedSizeTerminalWithTrivia<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SelectedImport {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<selected_import::Sequence0>,
    }
}

/// UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
pub type UserDefinedValueTypeDefinition =
    user_defined_value_type_definition::UserDefinedValueTypeDefinition;
pub mod user_defined_value_type_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UserDefinedValueTypeDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#type: FixedSizeTerminalWithTrivia<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub is: FixedSizeTerminalWithTrivia<2usize>,
        pub elementary_type: ElementaryType,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
pub type YulLiteral = Box<yul_literal::YulLiteral>;
pub mod yul_literal {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum YulLiteral {
        YulDecimalNumberLiteral(yul_decimal_number_literal::WithTrivia),
        YulHexLiteral(yul_hex_literal::WithTrivia),
        AsciiStringLiteral(ascii_string_literal::WithTrivia),
        BooleanLiteral(boolean_literal::WithTrivia),
        HexStringLiteral(hex_string_literal::WithTrivia),
    }
}

/// MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
pub type MappingType = mapping_type::MappingType;
pub mod mapping_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices1 {
        ElementaryType(ElementaryType),
        IdentifierPath(IdentifierPath),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        pub choices_1: Box<mapping_type::Choices1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_greater: FixedSizeTerminalWithTrivia<2usize>,
        pub type_name: TypeName,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndSequence0AndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        pub sequence_0: mapping_type::Sequence0,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct MappingType {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub mapping: FixedSizeTerminalWithTrivia<7usize>,
        pub open_paren_and_sequence_0_and_close_paren:
            mapping_type::OpenParenAndSequence0AndCloseParen,
    }
}

/// NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
pub type NamedArgumentList = named_argument_list::NamedArgumentList;
pub mod named_argument_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct NamedArgumentRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub named_argument_repeated: Vec<NamedArgument>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct NamedArgumentList {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub named_argument_repeated_and_comma_repeated:
            Option<named_argument_list::NamedArgumentRepeatedAndCommaRepeated>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
}

/// OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
pub type OverrideSpecifier = override_specifier::OverrideSpecifier;
pub mod override_specifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct IdentifierPathRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path_repeated: Vec<IdentifierPath>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndIdentifierPathRepeatedAndCommaRepeatedAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path_repeated_and_comma_repeated:
            override_specifier::IdentifierPathRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OverrideSpecifier {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#override: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_and_identifier_path_repeated_and_comma_repeated_and_close_paren: Option<
            override_specifier::OpenParenAndIdentifierPathRepeatedAndCommaRepeatedAndCloseParen,
        >,
    }
}

/// ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
pub type ParameterList = parameter_list::ParameterList;
pub mod parameter_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ParameterDeclarationRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_declaration_repeated: Vec<ParameterDeclaration>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ParameterList {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_declaration_repeated_and_comma_repeated:
            Option<parameter_list::ParameterDeclarationRepeatedAndCommaRepeated>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
}

/// PragmaDirective = 'pragma' ( VersionPragmaSpecifier | ABICoderPragmaSpecifier | ExperimentalPragmaSpecifier ) ';' ;
pub type PragmaDirective = pragma_directive::PragmaDirective;
pub mod pragma_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        VersionPragmaSpecifier(VersionPragmaSpecifier),
        AbiCoderPragmaSpecifier(AbiCoderPragmaSpecifier),
        ExperimentalPragmaSpecifier(ExperimentalPragmaSpecifier),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PragmaDirective {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub pragma: FixedSizeTerminalWithTrivia<6usize>,
        pub choices_0: Box<pragma_directive::Choices0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
pub type SelectingImportDirective = selecting_import_directive::SelectingImportDirective;
pub mod selecting_import_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SelectedImportRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub selected_import_repeated: Vec<SelectedImport>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBraceAndSelectedImportRepeatedAndCommaRepeatedAndCloseBrace {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub selected_import_repeated_and_comma_repeated:
            selecting_import_directive::SelectedImportRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SelectingImportDirective { # [serde (default , skip_serializing_if = "DefaultTest::is_default")] pub open_brace_and_selected_import_repeated_and_comma_repeated_and_close_brace : selecting_import_directive :: OpenBraceAndSelectedImportRepeatedAndCommaRepeatedAndCloseBrace , # [serde (default , skip_serializing_if = "DefaultTest::is_default")] pub from : FixedSizeTerminalWithTrivia < 4usize > , pub import_path : ImportPath }
}

/// SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
pub type SimpleImportDirective = simple_import_directive::SimpleImportDirective;
pub mod simple_import_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#as: FixedSizeTerminalWithTrivia<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SimpleImportDirective {
        pub import_path: ImportPath,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0_repeated: Vec<simple_import_directive::Sequence0>,
    }
}

/// StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
pub type StarImportDirective = star_import_directive::StarImportDirective;
pub mod star_import_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct StarImportDirective {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star: FixedSizeTerminalWithTrivia<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#as: FixedSizeTerminalWithTrivia<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub from: FixedSizeTerminalWithTrivia<4usize>,
        pub import_path: ImportPath,
    }
}

/// YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
pub type YulExpression = Box<yul_expression::YulExpression>;
pub mod yul_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum YulExpression {
        YulIdentifierPath(YulIdentifierPath),
        YulFunctionCall(YulFunctionCall),
        YulLiteral(YulLiteral),
    }
}

/// ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
pub type ArgumentList = argument_list::ArgumentList;
pub mod argument_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        PositionalArgumentList(PositionalArgumentList),
        NamedArgumentList(NamedArgumentList),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ArgumentList {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub choices_0: Option<Box<argument_list::Choices0>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
}

/// CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
pub type CatchClause = catch_clause::CatchClause;
pub mod catch_clause {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::WithTrivia>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CatchClause {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub catch: FixedSizeTerminalWithTrivia<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<catch_clause::Sequence0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: Block,
    }
}

/// FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
pub type FunctionType = function_type::FunctionType;
pub mod function_type {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Internal(FixedSizeTerminalWithTrivia<8usize>),
        External(FixedSizeTerminalWithTrivia<8usize>),
        Private(FixedSizeTerminalWithTrivia<7usize>),
        Public(FixedSizeTerminalWithTrivia<6usize>),
        Pure(FixedSizeTerminalWithTrivia<4usize>),
        View(FixedSizeTerminalWithTrivia<4usize>),
        Payable(FixedSizeTerminalWithTrivia<7usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedSizeTerminalWithTrivia<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FunctionType {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub choices_0_repeated: Vec<Box<function_type::Choices0>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_1: Option<function_type::Sequence1>,
    }
}

/// ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
pub type ImportDirective = import_directive::ImportDirective;
pub mod import_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        SimpleImportDirective(SimpleImportDirective),
        StarImportDirective(StarImportDirective),
        SelectingImportDirective(SelectingImportDirective),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ImportDirective {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub import: FixedSizeTerminalWithTrivia<6usize>,
        pub choices_0: Box<import_directive::Choices0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// ModifierAttribute = OverrideSpecifier | 'virtual' ;
pub type ModifierAttribute = Box<modifier_attribute::ModifierAttribute>;
pub mod modifier_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ModifierAttribute {
        OverrideSpecifier(OverrideSpecifier),
        Virtual(FixedSizeTerminalWithTrivia<7usize>),
    }
}

/// StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
pub type StateVariableAttribute = Box<state_variable_attribute::StateVariableAttribute>;
pub mod state_variable_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum StateVariableAttribute {
        OverrideSpecifier(OverrideSpecifier),
        Constant(FixedSizeTerminalWithTrivia<8usize>),
        Immutable(FixedSizeTerminalWithTrivia<9usize>),
        Internal(FixedSizeTerminalWithTrivia<8usize>),
        Private(FixedSizeTerminalWithTrivia<7usize>),
        Public(FixedSizeTerminalWithTrivia<6usize>),
    }
}

/// YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
pub type YulAssignmentStatement = yul_assignment_statement::YulAssignmentStatement;
pub mod yul_assignment_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulIdentifierPathRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier_path_repeated: Vec<YulIdentifierPath>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulAssignmentStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier_path_repeated_and_comma_repeated:
            yul_assignment_statement::YulIdentifierPathRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedSizeTerminalWithTrivia<2usize>,
        pub yul_expression: YulExpression,
    }
}

/// YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
pub type YulForStatement = yul_for_statement::YulForStatement;
pub mod yul_for_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulForStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#for: FixedSizeTerminalWithTrivia<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block_0_: YulBlock,
        pub yul_expression: YulExpression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block_1_: YulBlock,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block_2_: YulBlock,
    }
}

/// YulIfStatement = 'if' YulExpression YulBlock ;
pub type YulIfStatement = yul_if_statement::YulIfStatement;
pub mod yul_if_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulIfStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#if: FixedSizeTerminalWithTrivia<2usize>,
        pub yul_expression: YulExpression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: YulBlock,
    }
}

/// YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
pub type YulSwitchStatement = yul_switch_statement::YulSwitchStatement;
pub mod yul_switch_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub case: FixedSizeTerminalWithTrivia<4usize>,
        pub yul_literal: YulLiteral,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices1 {
        Sequence2(yul_switch_statement::Sequence2),
        Default(FixedSizeTerminalWithTrivia<7usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        pub choices_1: Box<yul_switch_statement::Choices1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: YulBlock,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulSwitchStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub switch: FixedSizeTerminalWithTrivia<6usize>,
        pub yul_expression: YulExpression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0_repeated: Vec<yul_switch_statement::Sequence0>,
    }
}

/// YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
pub type YulVariableDeclaration = yul_variable_declaration::YulVariableDeclaration;
pub mod yul_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulIdentifierPathRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier_path_repeated: Vec<YulIdentifierPath>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon_equal: FixedSizeTerminalWithTrivia<2usize>,
        pub yul_expression: YulExpression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulVariableDeclaration {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#let: FixedSizeTerminalWithTrivia<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_identifier_path_repeated_and_comma_repeated:
            yul_variable_declaration::YulIdentifierPathRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<yul_variable_declaration::Sequence0>,
    }
}

/// EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
pub type EmitStatement = emit_statement::EmitStatement;
pub mod emit_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EmitStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub emit: FixedSizeTerminalWithTrivia<4usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path: IdentifierPath,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: ArgumentList,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
pub type InheritanceSpecifier = inheritance_specifier::InheritanceSpecifier;
pub mod inheritance_specifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct InheritanceSpecifier {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path: IdentifierPath,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: Option<ArgumentList>,
    }
}

/// ModifierInvocation = IdentifierPath [ ArgumentList ] ;
pub type ModifierInvocation = modifier_invocation::ModifierInvocation;
pub mod modifier_invocation {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ModifierInvocation {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path: IdentifierPath,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: Option<ArgumentList>,
    }
}

/// NewExpression = 'new' IdentifierPath ArgumentList ;
pub type NewExpression = new_expression::NewExpression;
pub mod new_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct NewExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub new: FixedSizeTerminalWithTrivia<3usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path: IdentifierPath,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: ArgumentList,
    }
}

/// PayableExpression = 'payable' ArgumentList ;
pub type PayableExpression = payable_expression::PayableExpression;
pub mod payable_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PayableExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub payable: FixedSizeTerminalWithTrivia<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: ArgumentList,
    }
}

/// RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
pub type RevertStatement = revert_statement::RevertStatement;
pub mod revert_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RevertStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub revert: FixedSizeTerminalWithTrivia<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path: Option<IdentifierPath>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub argument_list: ArgumentList,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
pub type TypeName = type_name::TypeName;
pub mod type_name {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        ElementaryType(ElementaryType),
        FunctionType(FunctionType),
        MappingType(MappingType),
        IdentifierPath(IdentifierPath),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBracketAndExpressionAndCloseBracket {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<Expression>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct TypeName {
        pub choices_0: Box<type_name::Choices0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_and_expression_and_close_bracket_repeated:
            Vec<type_name::OpenBracketAndExpressionAndCloseBracket>,
    }
}

/// YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
pub type YulStatement = Box<yul_statement::YulStatement>;
pub mod yul_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum YulStatement {
        YulBlock(YulBlock),
        YulVariableDeclaration(YulVariableDeclaration),
        YulFunctionDefinition(YulFunctionDefinition),
        YulAssignmentStatement(YulAssignmentStatement),
        YulFunctionCall(YulFunctionCall),
        YulIfStatement(YulIfStatement),
        YulForStatement(YulForStatement),
        YulSwitchStatement(YulSwitchStatement),
        YulLeaveStatement(YulLeaveStatement),
        YulBreakStatement(YulBreakStatement),
        YulContinueStatement(YulContinueStatement),
    }
}

/// ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
pub type ConstructorAttribute = Box<constructor_attribute::ConstructorAttribute>;
pub mod constructor_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ConstructorAttribute {
        ModifierInvocation(ModifierInvocation),
        Internal(FixedSizeTerminalWithTrivia<8usize>),
        Payable(FixedSizeTerminalWithTrivia<7usize>),
        Public(FixedSizeTerminalWithTrivia<6usize>),
    }
}

/// ErrorParameter = TypeName [ «Identifier» ] ;
pub type ErrorParameter = error_parameter::ErrorParameter;
pub mod error_parameter {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ErrorParameter {
        pub type_name: TypeName,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::WithTrivia>,
    }
}

/// EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
pub type EventParameter = event_parameter::EventParameter;
pub mod event_parameter {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EventParameter {
        pub type_name: TypeName,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub indexed: Option<FixedSizeTerminalWithTrivia<7usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: Option<identifier::WithTrivia>,
    }
}

/// FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
pub type FallbackFunctionAttribute = Box<fallback_function_attribute::FallbackFunctionAttribute>;
pub mod fallback_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum FallbackFunctionAttribute {
        ModifierInvocation(ModifierInvocation),
        OverrideSpecifier(OverrideSpecifier),
        External(FixedSizeTerminalWithTrivia<8usize>),
        Payable(FixedSizeTerminalWithTrivia<7usize>),
        Pure(FixedSizeTerminalWithTrivia<4usize>),
        View(FixedSizeTerminalWithTrivia<4usize>),
        Virtual(FixedSizeTerminalWithTrivia<7usize>),
    }
}

/// FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
pub type FunctionAttribute = Box<function_attribute::FunctionAttribute>;
pub mod function_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum FunctionAttribute {
        ModifierInvocation(ModifierInvocation),
        OverrideSpecifier(OverrideSpecifier),
        External(FixedSizeTerminalWithTrivia<8usize>),
        Internal(FixedSizeTerminalWithTrivia<8usize>),
        Payable(FixedSizeTerminalWithTrivia<7usize>),
        Private(FixedSizeTerminalWithTrivia<7usize>),
        Public(FixedSizeTerminalWithTrivia<6usize>),
        Pure(FixedSizeTerminalWithTrivia<4usize>),
        View(FixedSizeTerminalWithTrivia<4usize>),
        Virtual(FixedSizeTerminalWithTrivia<7usize>),
    }
}

/// InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
pub type InheritanceSpecifierList = inheritance_specifier_list::InheritanceSpecifierList;
pub mod inheritance_specifier_list {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct InheritanceSpecifierRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifier_repeated: Vec<InheritanceSpecifier>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct InheritanceSpecifierList {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub is: FixedSizeTerminalWithTrivia<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifier_repeated_and_comma_repeated:
            inheritance_specifier_list::InheritanceSpecifierRepeatedAndCommaRepeated,
    }
}

/// ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
pub type ReceiveFunctionAttribute = Box<receive_function_attribute::ReceiveFunctionAttribute>;
pub mod receive_function_attribute {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ReceiveFunctionAttribute {
        ModifierInvocation(ModifierInvocation),
        OverrideSpecifier(OverrideSpecifier),
        External(FixedSizeTerminalWithTrivia<8usize>),
        Payable(FixedSizeTerminalWithTrivia<7usize>),
        Virtual(FixedSizeTerminalWithTrivia<7usize>),
    }
}

/// StructMember = TypeName «Identifier» ';' ;
pub type StructMember = struct_member::StructMember;
pub mod struct_member {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct StructMember {
        pub type_name: TypeName,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// TypeExpression = 'type' '(' TypeName ')' ;
pub type TypeExpression = type_expression::TypeExpression;
pub mod type_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndTypeNameAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        pub type_name: TypeName,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct TypeExpression {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#type: FixedSizeTerminalWithTrivia<4usize>,
        pub open_paren_and_type_name_and_close_paren:
            type_expression::OpenParenAndTypeNameAndCloseParen,
    }
}

/// UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
pub type UsingDirective = using_directive::UsingDirective;
pub mod using_directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct IdentifierPathRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path_repeated: Vec<IdentifierPath>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBraceAndIdentifierPathRepeatedAndCommaRepeatedAndCloseBrace {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier_path_repeated_and_comma_repeated:
            using_directive::IdentifierPathRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        IdentifierPath(IdentifierPath),
        OpenBraceAndIdentifierPathRepeatedAndCommaRepeatedAndCloseBrace(
            using_directive::OpenBraceAndIdentifierPathRepeatedAndCommaRepeatedAndCloseBrace,
        ),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices1 {
        Star(FixedSizeTerminalWithTrivia<1>),
        TypeName(TypeName),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UsingDirective {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub using: FixedSizeTerminalWithTrivia<5usize>,
        pub choices_0: Box<using_directive::Choices0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#for: FixedSizeTerminalWithTrivia<3usize>,
        pub choices_1: Box<using_directive::Choices1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub global: Option<FixedSizeTerminalWithTrivia<6usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// YulBlock = '{' { YulStatement } '}' ;
pub type YulBlock = yul_block::YulBlock;
pub mod yul_block {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct YulBlock {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_statement_repeated: Vec<YulStatement>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
}

/// AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
pub type AssemblyStatement = assembly_statement::AssemblyStatement;
pub mod assembly_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AssemblyStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub assembly: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub double_quote_evmasm_double_quote: Option<FixedSizeTerminalWithTrivia<8usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub assembly_flags: Option<AssemblyFlags>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub yul_block: YulBlock,
    }
}

/// Directive = PragmaDirective | ImportDirective | UsingDirective ;
pub type Directive = Box<directive::Directive>;
pub mod directive {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Directive {
        PragmaDirective(PragmaDirective),
        ImportDirective(ImportDirective),
        UsingDirective(UsingDirective),
    }
}

/// ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
pub type ErrorDefinition = error_definition::ErrorDefinition;
pub mod error_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ErrorParameterRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub error_parameter_repeated: Vec<ErrorParameter>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndErrorParameterRepeatedAndCommaRepeatedAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub error_parameter_repeated_and_comma_repeated:
            Option<error_definition::ErrorParameterRepeatedAndCommaRepeated>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ErrorDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub error: FixedSizeTerminalWithTrivia<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_and_error_parameter_repeated_and_comma_repeated_and_close_paren:
            error_definition::OpenParenAndErrorParameterRepeatedAndCommaRepeatedAndCloseParen,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
pub type EventDefinition = event_definition::EventDefinition;
pub mod event_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EventParameterRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub event_parameter_repeated: Vec<EventParameter>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndEventParameterRepeatedAndCommaRepeatedAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub event_parameter_repeated_and_comma_repeated:
            Option<event_definition::EventParameterRepeatedAndCommaRepeated>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EventDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub event: FixedSizeTerminalWithTrivia<5usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_and_event_parameter_repeated_and_comma_repeated_and_close_paren:
            event_definition::OpenParenAndEventParameterRepeatedAndCommaRepeatedAndCloseParen,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub anonymous: Option<FixedSizeTerminalWithTrivia<9usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | «NumericLiteral» | «BooleanLiteral» | «Identifier» ;
pub type PrimaryExpression = Expression;
pub mod primary_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PrimaryExpression {
        PayableExpression(PayableExpression),
        TypeExpression(TypeExpression),
        NewExpression(NewExpression),
        ParenthesisExpression(ParenthesisExpression),
        ArrayLiteral(ArrayLiteral),
        AsciiStringLiteral(ascii_string_literal::WithTrivia),
        UnicodeStringLiteral(unicode_string_literal::WithTrivia),
        HexStringLiteral(hex_string_literal::WithTrivia),
        NumericLiteral(numeric_literal::WithTrivia),
        BooleanLiteral(boolean_literal::WithTrivia),
        Identifier(identifier::WithTrivia),
    }
    pub type E = Box<primary_expression::PrimaryExpression>;
}

/// StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
pub type StructDefinition = struct_definition::StructDefinition;
pub mod struct_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBraceAndStructMemberRepeatedAndCloseBrace {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub struct_member_repeated: Vec<StructMember>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct StructDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#struct: FixedSizeTerminalWithTrivia<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_and_struct_member_repeated_and_close_brace:
            struct_definition::OpenBraceAndStructMemberRepeatedAndCloseBrace,
    }
}

/// IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
pub type IndexAccessExpression = Expression;
pub mod index_access_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon: FixedSizeTerminalWithTrivia<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<Expression>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<Expression>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_1: Option<index_access_expression::Sequence1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBracketAndSequence0AndCloseBracket {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: index_access_expression::Sequence0,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag3 {
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_and_sequence_0_and_close_bracket:
            index_access_expression::OpenBracketAndSequence0AndCloseBracket,
    }
    pub type E = index_access_expression::Anonexpfrag3;
}

/// MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
pub type MemberAccessExpression = Expression;
pub mod member_access_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Identifier(identifier::WithTrivia),
        Address(FixedSizeTerminalWithTrivia<7usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub period: FixedSizeTerminalWithTrivia<1>,
        pub choices_0: Box<member_access_expression::Choices0>,
    }
    pub type E = member_access_expression::Anonexpfrag4;
}

/// FunctionCallExpression = Expression [ '{' NamedArgument  { ',' NamedArgument } '}' ] ArgumentList ;
pub type FunctionCallExpression = Expression;
pub mod function_call_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct NamedArgumentRepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub named_argument_repeated: Vec<NamedArgument>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBraceAndNamedArgumentRepeatedAndCommaRepeatedAndCloseBrace {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub named_argument_repeated_and_comma_repeated:
            function_call_expression::NamedArgumentRepeatedAndCommaRepeated,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 { pub expression : Expression , # [serde (default , skip_serializing_if = "DefaultTest::is_default")] pub open_brace_and_named_argument_repeated_and_comma_repeated_and_close_brace : Option < function_call_expression :: OpenBraceAndNamedArgumentRepeatedAndCommaRepeatedAndCloseBrace > , # [serde (default , skip_serializing_if = "DefaultTest::is_default")] pub argument_list : ArgumentList }
    pub type E = function_call_expression::Anonexpfrag4;
}

/// UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
pub type UnaryPrefixExpression = Expression;
pub mod unary_prefix_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        PlusPlus(FixedSizeTerminalWithTrivia<2usize>),
        MinusMinus(FixedSizeTerminalWithTrivia<2usize>),
        Bang(FixedSizeTerminalWithTrivia<1>),
        Tilde(FixedSizeTerminalWithTrivia<1>),
        Minus(FixedSizeTerminalWithTrivia<1>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag3 {
        pub choices_0: Box<unary_prefix_expression::Choices0>,
        pub expression: Expression,
    }
    pub type E = unary_prefix_expression::Anonexpfrag3;
}

/// UnarySuffixExpression = Expression ( '++' | '--' ) ;
pub type UnarySuffixExpression = Expression;
pub mod unary_suffix_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        PlusPlus(FixedSizeTerminalWithTrivia<2usize>),
        MinusMinus(FixedSizeTerminalWithTrivia<2usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag3 {
        pub expression: Expression,
        pub choices_0: Box<unary_suffix_expression::Choices0>,
    }
    pub type E = unary_suffix_expression::Anonexpfrag3;
}

/// ExponentiationExpression = Expression '**' Expression ;
pub type ExponentiationExpression = Expression;
pub mod exponentiation_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_star: FixedSizeTerminalWithTrivia<2usize>,
        pub expression_1_: Expression,
    }
    pub type E = exponentiation_expression::Anonexpfrag4;
}

/// MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
pub type MulDivModExpression = Expression;
pub mod mul_div_mod_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0: FixedSizeTerminalWithTrivia<1>,
        pub expression_1_: Expression,
    }
    pub type E = mul_div_mod_expression::Anonexpfrag4;
}

/// AddSubExpression = Expression ( '+' | '-' ) Expression ;
pub type AddSubExpression = Expression;
pub mod add_sub_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub filter_0: FixedSizeTerminalWithTrivia<1>,
        pub expression_1_: Expression,
    }
    pub type E = add_sub_expression::Anonexpfrag4;
}

/// ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
pub type ShiftExpression = Expression;
pub mod shift_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        LessLess(FixedSizeTerminalWithTrivia<2usize>),
        GreaterGreater(FixedSizeTerminalWithTrivia<2usize>),
        GreaterGreaterGreater(FixedSizeTerminalWithTrivia<3usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        pub choices_0: Box<shift_expression::Choices0>,
        pub expression_1_: Expression,
    }
    pub type E = shift_expression::Anonexpfrag4;
}

/// BitAndExpression = Expression '&' Expression ;
pub type BitAndExpression = Expression;
pub mod bit_and_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ampersand: FixedSizeTerminalWithTrivia<1>,
        pub expression_1_: Expression,
    }
    pub type E = bit_and_expression::Anonexpfrag4;
}

/// BitXOrExpression = Expression '^' Expression ;
pub type BitXOrExpression = Expression;
pub mod bit_x_or_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub caret: FixedSizeTerminalWithTrivia<1>,
        pub expression_1_: Expression,
    }
    pub type E = bit_x_or_expression::Anonexpfrag4;
}

/// BitOrExpression = Expression '|' Expression ;
pub type BitOrExpression = Expression;
pub mod bit_or_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub pipe: FixedSizeTerminalWithTrivia<1>,
        pub expression_1_: Expression,
    }
    pub type E = bit_or_expression::Anonexpfrag4;
}

/// OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
pub type OrderComparisonExpression = Expression;
pub mod order_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Less(FixedSizeTerminalWithTrivia<1>),
        Greater(FixedSizeTerminalWithTrivia<1>),
        LessEqual(FixedSizeTerminalWithTrivia<2usize>),
        GreaterEqual(FixedSizeTerminalWithTrivia<2usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        pub choices_0: Box<order_comparison_expression::Choices0>,
        pub expression_1_: Expression,
    }
    pub type E = order_comparison_expression::Anonexpfrag4;
}

/// EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
pub type EqualityComparisonExpression = Expression;
pub mod equality_comparison_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        EqualEqual(FixedSizeTerminalWithTrivia<2usize>),
        BangEqual(FixedSizeTerminalWithTrivia<2usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        pub choices_0: Box<equality_comparison_expression::Choices0>,
        pub expression_1_: Expression,
    }
    pub type E = equality_comparison_expression::Anonexpfrag4;
}

/// AndExpression = Expression '&&' Expression ;
pub type AndExpression = Expression;
pub mod and_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ampersand_ampersand: FixedSizeTerminalWithTrivia<2usize>,
        pub expression_1_: Expression,
    }
    pub type E = and_expression::Anonexpfrag4;
}

/// OrExpression = Expression '||' Expression ;
pub type OrExpression = Expression;
pub mod or_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub pipe_pipe: FixedSizeTerminalWithTrivia<2usize>,
        pub expression_1_: Expression,
    }
    pub type E = or_expression::Anonexpfrag4;
}

/// ConditionalExpression = Expression '?' Expression ':' Expression ;
pub type ConditionalExpression = Expression;
pub mod conditional_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub question: FixedSizeTerminalWithTrivia<1>,
        pub expression_0_: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub colon: FixedSizeTerminalWithTrivia<1>,
        pub expression_1_: Expression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag3 {
        pub expression: Expression,
        pub sequence_0: conditional_expression::Sequence0,
    }
    pub type E = conditional_expression::Anonexpfrag3;
}

/// AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
pub type AssignmentExpression = Expression;
pub mod assignment_expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Equal(FixedSizeTerminalWithTrivia<1>),
        PipeEqual(FixedSizeTerminalWithTrivia<2usize>),
        CaretEqual(FixedSizeTerminalWithTrivia<2usize>),
        AmpersandEqual(FixedSizeTerminalWithTrivia<2usize>),
        LessLessEqual(FixedSizeTerminalWithTrivia<3usize>),
        GreaterGreaterEqual(FixedSizeTerminalWithTrivia<3usize>),
        GreaterGreaterGreaterEqual(FixedSizeTerminalWithTrivia<4usize>),
        PlusEqual(FixedSizeTerminalWithTrivia<2usize>),
        MinusEqual(FixedSizeTerminalWithTrivia<2usize>),
        StarEqual(FixedSizeTerminalWithTrivia<2usize>),
        SlashEqual(FixedSizeTerminalWithTrivia<2usize>),
        PercentEqual(FixedSizeTerminalWithTrivia<2usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Anonexpfrag4 {
        pub expression_0_: Expression,
        pub choices_0: Box<assignment_expression::Choices0>,
        pub expression_1_: Expression,
    }
    pub type E = assignment_expression::Anonexpfrag4;
}

/// Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
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
        MemberAccessExpression(member_access_expression::E),
        IndexAccessExpression(index_access_expression::E),
        PrimaryExpression(primary_expression::E),
    }
}

/// ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
pub type ConstantDefinition = constant_definition::ConstantDefinition;
pub mod constant_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ConstantDefinition {
        pub type_name: TypeName,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub constant: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal: FixedSizeTerminalWithTrivia<1>,
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
pub type DoWhileStatement = do_while_statement::DoWhileStatement;
pub mod do_while_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndExpressionAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DoWhileStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#do: FixedSizeTerminalWithTrivia<2usize>,
        pub statement: Statement,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#while: FixedSizeTerminalWithTrivia<5usize>,
        pub open_paren_and_expression_and_close_paren:
            do_while_statement::OpenParenAndExpressionAndCloseParen,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// ExpressionStatement = Expression ';' ;
pub type ExpressionStatement = expression_statement::ExpressionStatement;
pub mod expression_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ExpressionStatement {
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
pub type IfStatement = if_statement::IfStatement;
pub mod if_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndExpressionAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#else: FixedSizeTerminalWithTrivia<4usize>,
        pub statement: Statement,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct IfStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#if: FixedSizeTerminalWithTrivia<2usize>,
        pub open_paren_and_expression_and_close_paren:
            if_statement::OpenParenAndExpressionAndCloseParen,
        pub statement: Statement,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<if_statement::Sequence0>,
    }
}

/// ReturnStatement = 'return' [ Expression ] ';' ;
pub type ReturnStatement = return_statement::ReturnStatement;
pub mod return_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ReturnStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#return: FixedSizeTerminalWithTrivia<6usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<Expression>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
pub type StateVariableDeclaration = state_variable_declaration::StateVariableDeclaration;
pub mod state_variable_declaration {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal: FixedSizeTerminalWithTrivia<1>,
        pub expression: Expression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct StateVariableDeclaration {
        pub type_name: TypeName,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub state_variable_attribute_repeated: Vec<StateVariableAttribute>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<state_variable_declaration::Sequence0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
pub type TryStatement = try_statement::TryStatement;
pub mod try_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedSizeTerminalWithTrivia<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct TryStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#try: FixedSizeTerminalWithTrivia<3usize>,
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<try_statement::Sequence0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: Block,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub catch_clause_repeated: Vec<CatchClause>,
    }
}

/// TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
pub type TupleDeconstructionStatement =
    tuple_deconstruction_statement::TupleDeconstructionStatement;
pub mod tuple_deconstruction_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub type_name: Option<TypeName>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0RepeatedAndCommaRepeated {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0_repeated: Vec<Option<tuple_deconstruction_statement::Sequence0>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub comma_repeated: Vec<FixedSizeTerminalWithTrivia<1usize>>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndSequence0RepeatedAndCommaRepeatedAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0_repeated_and_comma_repeated:
            Option<tuple_deconstruction_statement::Sequence0RepeatedAndCommaRepeated>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct TupleDeconstructionStatement { # [serde (default , skip_serializing_if = "DefaultTest::is_default")] pub open_paren_and_sequence_0_repeated_and_comma_repeated_and_close_paren : tuple_deconstruction_statement :: OpenParenAndSequence0RepeatedAndCommaRepeatedAndCloseParen , # [serde (default , skip_serializing_if = "DefaultTest::is_default")] pub equal : FixedSizeTerminalWithTrivia < 1 > , pub expression : Expression , # [serde (default , skip_serializing_if = "DefaultTest::is_default")] pub semicolon : FixedSizeTerminalWithTrivia < 1 > }
}

/// VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
pub type VariableDeclarationStatement =
    variable_declaration_statement::VariableDeclarationStatement;
pub mod variable_declaration_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal: FixedSizeTerminalWithTrivia<1>,
        pub expression: Expression,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct VariableDeclarationStatement {
        pub type_name: TypeName,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub data_location: Option<DataLocation>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<variable_declaration_statement::Sequence0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon: FixedSizeTerminalWithTrivia<1>,
    }
}

/// WhileStatement = 'while' '(' Expression ')' Statement ;
pub type WhileStatement = while_statement::WhileStatement;
pub mod while_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndExpressionAndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WhileStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#while: FixedSizeTerminalWithTrivia<5usize>,
        pub open_paren_and_expression_and_close_paren:
            while_statement::OpenParenAndExpressionAndCloseParen,
        pub statement: Statement,
    }
}

/// SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
pub type SimpleStatement = Box<simple_statement::SimpleStatement>;
pub mod simple_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SimpleStatement {
        TupleDeconstructionStatement(TupleDeconstructionStatement),
        VariableDeclarationStatement(VariableDeclarationStatement),
        ExpressionStatement(ExpressionStatement),
    }
}

/// ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
pub type ForStatement = for_statement::ForStatement;
pub mod for_statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices1 {
        SimpleStatement(SimpleStatement),
        Semicolon(FixedSizeTerminalWithTrivia<1>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices2 {
        ExpressionStatement(ExpressionStatement),
        Semicolon(FixedSizeTerminalWithTrivia<1>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        pub choices_1: Box<for_statement::Choices1>,
        pub choices_2: Box<for_statement::Choices2>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Option<Expression>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenParenAndSequence0AndCloseParen {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren: FixedSizeTerminalWithTrivia<1usize>,
        pub sequence_0: for_statement::Sequence0,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ForStatement {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#for: FixedSizeTerminalWithTrivia<3usize>,
        pub open_paren_and_sequence_0_and_close_paren:
            for_statement::OpenParenAndSequence0AndCloseParen,
        pub statement: Statement,
    }
}

/// Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
pub type Statement = Box<statement::Statement>;
pub mod statement {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        DeleteStatement(DeleteStatement),
        AssemblyStatement(AssemblyStatement),
    }
}

/// Block = '{' { Statement | UncheckedBlock } '}' ;
pub type Block = block::Block;
pub mod block {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Statement(Statement),
        UncheckedBlock(UncheckedBlock),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Block {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub choices_0_repeated: Vec<Box<block::Choices0>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
}

/// ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
pub type ConstructorDefinition = constructor_definition::ConstructorDefinition;
pub mod constructor_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ConstructorDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub constructor: FixedSizeTerminalWithTrivia<11usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub constructor_attribute_repeated: Vec<ConstructorAttribute>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub block: Block,
    }
}

/// FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
pub type FallbackFunctionDefinition = fallback_function_definition::FallbackFunctionDefinition;
pub mod fallback_function_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedSizeTerminalWithTrivia<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices1 {
        Semicolon(FixedSizeTerminalWithTrivia<1>),
        Block(Block),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FallbackFunctionDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub fallback: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub fallback_function_attribute_repeated: Vec<FallbackFunctionAttribute>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_0: Option<fallback_function_definition::Sequence0>,
        pub choices_1: Box<fallback_function_definition::Choices1>,
    }
}

/// FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
pub type FunctionDefinition = function_definition::FunctionDefinition;
pub mod function_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Identifier(identifier::WithTrivia),
        Fallback(FixedSizeTerminalWithTrivia<8usize>),
        Receive(FixedSizeTerminalWithTrivia<7usize>),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Sequence1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub returns: FixedSizeTerminalWithTrivia<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices2 {
        Semicolon(FixedSizeTerminalWithTrivia<1>),
        Block(Block),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FunctionDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function: FixedSizeTerminalWithTrivia<8usize>,
        pub choices_0: Box<function_definition::Choices0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub function_attribute_repeated: Vec<FunctionAttribute>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequence_1: Option<function_definition::Sequence1>,
        pub choices_2: Box<function_definition::Choices2>,
    }
}

/// ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
pub type ModifierDefinition = modifier_definition::ModifierDefinition;
pub mod modifier_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Semicolon(FixedSizeTerminalWithTrivia<1>),
        Block(Block),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ModifierDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub modifier: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: Option<ParameterList>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub modifier_attribute_repeated: Vec<ModifierAttribute>,
        pub choices_0: Box<modifier_definition::Choices0>,
    }
}

/// ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
pub type ReceiveFunctionDefinition = receive_function_definition::ReceiveFunctionDefinition;
pub mod receive_function_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Semicolon(FixedSizeTerminalWithTrivia<1>),
        Block(Block),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ReceiveFunctionDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub receive: FixedSizeTerminalWithTrivia<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub parameter_list: ParameterList,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub receive_function_attribute_repeated: Vec<ReceiveFunctionAttribute>,
        pub choices_0: Box<receive_function_definition::Choices0>,
    }
}

/// ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
pub type ContractBodyElement = Box<contract_body_element::ContractBodyElement>;
pub mod contract_body_element {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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
pub type ContractDefinition = contract_definition::ContractDefinition;
pub mod contract_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBraceAndContractBodyElementRepeatedAndCloseBrace {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub contract_body_element_repeated: Vec<ContractBodyElement>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ContractDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub r#abstract: Option<FixedSizeTerminalWithTrivia<8usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub contract: FixedSizeTerminalWithTrivia<8usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifier_list: Option<InheritanceSpecifierList>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_and_contract_body_element_repeated_and_close_brace:
            contract_definition::OpenBraceAndContractBodyElementRepeatedAndCloseBrace,
    }
}

/// InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
pub type InterfaceDefinition = interface_definition::InterfaceDefinition;
pub mod interface_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBraceAndContractBodyElementRepeatedAndCloseBrace {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub contract_body_element_repeated: Vec<ContractBodyElement>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct InterfaceDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub interface: FixedSizeTerminalWithTrivia<9usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub inheritance_specifier_list: Option<InheritanceSpecifierList>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_and_contract_body_element_repeated_and_close_brace:
            interface_definition::OpenBraceAndContractBodyElementRepeatedAndCloseBrace,
    }
}

/// LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
pub type LibraryDefinition = library_definition::LibraryDefinition;
pub mod library_definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenBraceAndContractBodyElementRepeatedAndCloseBrace {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace: FixedSizeTerminalWithTrivia<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub contract_body_element_repeated: Vec<ContractBodyElement>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace: FixedSizeTerminalWithTrivia<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct LibraryDefinition {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub library: FixedSizeTerminalWithTrivia<7usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_and_contract_body_element_repeated_and_close_brace:
            library_definition::OpenBraceAndContractBodyElementRepeatedAndCloseBrace,
    }
}

/// Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
pub type Definition = Box<definition::Definition>;
pub mod definition {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// SourceUnit = «LeadingTrivia» { Directive | Definition } «EndOfFileTrivia» $ ;
pub type SourceUnit = source_unit::SourceUnit;
pub mod source_unit {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Choices0 {
        Directive(Directive),
        Definition(Definition),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SourceUnit {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub choices_0_repeated: Vec<Box<source_unit::Choices0>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub end_of_file_trivia: EndOfFileTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub end_of_input: (),
    }
}
