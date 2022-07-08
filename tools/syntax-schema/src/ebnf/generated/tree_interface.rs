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
    pub leading: LeadingTrivia,
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub content: FixedSizeTerminal<N>,
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub trailing: TrailingTrivia,
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VariableSizeTerminal(pub usize);
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VariableSizeTerminalWithTrivia {
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub leading: LeadingTrivia,
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub content: VariableSizeTerminal,
    #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
    pub trailing: TrailingTrivia,
}

/// «Comment» = '(*' { ¬'*' | 1…*{ '*' } ¬( '*' | ')' ) } { '*' } '*)' ;
pub type Comment = comment::_T0;
pub mod comment {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_chars: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T2 {
        NotStarChar(FixedSizeTerminal<1>),
        _T3(comment::_T3),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Content {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2s: Vec<Box<comment::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_chars: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_star: FixedSizeTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: comment::Content,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_close_paren: FixedSizeTerminal<2usize>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: comment::_T0,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// «Number» = 1…*{ '0'…'9' } ;
pub type Number = VariableSizeTerminal;
pub mod number {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// «RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;
pub type RawIdentifier = raw_identifier::_T0;
pub mod raw_identifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _0: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: VariableSizeTerminal,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: raw_identifier::_T0,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// «StringChar» = ¬( '\'' | '\\' ) | '\\' ( '\'' | '\\' | 'u{' 1…6*{ «HexDigit» } '}' ) ;
pub type StringChar = Box<string_char::_T0>;
pub mod string_char {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub u_open_brace: FixedSizeTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum QuoteOrBackslashOrHexEscape {
        _0(FixedSizeTerminal<1>),
        _T1(string_char::_T1),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Escape {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub backslash_char: FixedSizeTerminal<1>,
        pub quote_or_backslash_or_hex_escape: Box<string_char::QuoteOrBackslashOrHexEscape>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        NotQuoteOrBackslash(FixedSizeTerminal<1>),
        Escape(string_char::Escape),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        pub content: Box<string_char::_T0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// «Whitespace» = 1…*{ '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}' } ;
pub type Whitespace = VariableSizeTerminal;
pub mod whitespace {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: VariableSizeTerminal,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
pub type Identifier = Box<identifier::_T0>;
pub mod identifier {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_double_angle_char: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub raw_identifier: RawIdentifier,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_double_angle_char: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        _T1(identifier::_T1),
        RawIdentifier(RawIdentifier),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        pub content: Box<identifier::_T0>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// «LeadingTrivia» = { «Whitespace» | «Comment» } ;
pub type LeadingTrivia = Vec<Box<leading_trivia::_T1>>;
pub mod leading_trivia {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        Whitespace(Whitespace),
        Comment(Comment),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: Vec<Box<leading_trivia::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// «SingleCharString» = '\'' «StringChar» '\'' ;
pub type SingleCharString = single_char_string::_T0;
pub mod single_char_string {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_1: FixedSizeTerminal<1>,
        pub string_char: StringChar,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_2: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        pub content: single_char_string::_T0,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// «String» = '\'' { «StringChar» } '\'' ;
pub type String = string::_T0;
pub mod string {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_1: FixedSizeTerminal<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub string_chars: Vec<StringChar>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_2: FixedSizeTerminal<1>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: string::_T0,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// «TrailingTrivia» = { «Whitespace» | «Comment» } ;
pub type TrailingTrivia = Vec<Box<trailing_trivia::_T1>>;
pub mod trailing_trivia {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        Whitespace(Whitespace),
        Comment(Comment),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct WithTrivia {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading: LeadingTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: Vec<Box<trailing_trivia::_T1>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing: TrailingTrivia,
    }
}

/// charRange = «SingleCharString» '…' «SingleCharString» ;
pub type CharRange = char_range::_T0;
pub mod char_range {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub single_char_string_1: single_char_string::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ellipsis_char: FixedSizeTerminalWithTrivia<1>,
        pub single_char_string_2: single_char_string::WithTrivia,
    }
}

/// grouped = '(' expression ')' ;
pub type Grouped = grouped::_T0;
pub mod grouped {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedSizeTerminalWithTrivia<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedSizeTerminalWithTrivia<1>,
    }
}

/// optional = '[' expression ']' ;
pub type Optional = optional::_T0;
pub mod optional {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_char: FixedSizeTerminalWithTrivia<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket_char: FixedSizeTerminalWithTrivia<1>,
    }
}

/// productionReference = «Identifier» ;
pub type ProductionReference = identifier::WithTrivia;

/// repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
pub type RepetitionPrefix = repetition_prefix::_T0;
pub mod repetition_prefix {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ellipsis_char: FixedSizeTerminalWithTrivia<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub number: Option<number::WithTrivia>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub number: number::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t3: Option<repetition_prefix::_T3>,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ellipsis_char: FixedSizeTerminalWithTrivia<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub number: number::WithTrivia,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(repetition_prefix::_T2),
        _T4(repetition_prefix::_T4),
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<repetition_prefix::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_char: FixedSizeTerminalWithTrivia<1>,
    }
}

/// repetitionSeparator = '/' expression ;
pub type RepetitionSeparator = repetition_separator::_T0;
pub mod repetition_separator {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub slash_char: FixedSizeTerminalWithTrivia<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Expression,
    }
}

/// repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
pub type Repeated = repeated::_T0;
pub mod repeated {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub repetition_prefix: Option<RepetitionPrefix>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedSizeTerminalWithTrivia<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub repetition_separator: Option<RepetitionSeparator>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedSizeTerminalWithTrivia<1>,
    }
}

/// primary = productionReference | grouped | optional | repeated | charRange | «EOF» | «String» ;
pub type Primary = Box<primary::_T0>;
pub mod primary {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum _T0 {
        ProductionReference(ProductionReference),
        Grouped(Grouped),
        Optional(Optional),
        Repeated(Repeated),
        CharRange(CharRange),
        DollarChar(FixedSizeTerminalWithTrivia<1>),
        String(string::WithTrivia),
    }
}

/// negation = [ '¬' ] primary ;
pub type Negation = negation::_T0;
pub mod negation {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub not_char: Option<FixedSizeTerminalWithTrivia<1>>,
        pub primary: Primary,
    }
}

/// difference = negation [ '-' negation ] ;
pub type Difference = difference::_T0;
pub mod difference {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub minus_char: FixedSizeTerminalWithTrivia<1>,
        pub negation: Negation,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub negation: Negation,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<difference::_T1>,
    }
}

/// sequence = 1…*{ difference } ;
pub type Sequence = Vec<Difference>;

/// expression = 1…*{ sequence / '|' } ;
pub type Expression = expression::_T0;
pub mod expression {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub elements: Vec<Sequence>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub separators: Vec<FixedSizeTerminalWithTrivia<1>>,
    }
}

/// production = «Identifier» '=' expression ';' ;
pub type Production = production::_T0;
pub mod production {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        pub identifier: identifier::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedSizeTerminalWithTrivia<1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: Expression,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedSizeTerminalWithTrivia<1>,
    }
}

/// grammar = «LeadingTrivia» { production } «TrailingTrivia» $ ;
pub type Grammar = grammar::_T0;
pub mod grammar {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub leading_trivia: leading_trivia::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub productions: Vec<Production>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub trailing_trivia: trailing_trivia::WithTrivia,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub end_marker: (),
    }
}
