#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
pub trait DefaultTest {
    fn is_default(&self) -> bool {
        false
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FixedTerminal<const N: usize>();

/// «Comment» = '(*' { ¬'*' | 1…*{ '*' } ¬( '*' | ')' ) } { '*' } '*)' ;
pub mod comment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<comment::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_star: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub content: Box<comment::Content>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_close_paren: FixedTerminal<2usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct Content {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t2s: Vec<Box<comment::_T2>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_chars: usize,
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

/// «EOF» = '$' ;
pub mod eof {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// «HexDigit» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
pub mod hex_digit {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// «IdentifierStart» = '_' | 'a'…'z' | 'A'…'Z' ;
pub mod identifier_start {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// «Number» = 1…*{ '0'…'9' } ;
pub mod number {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «Whitespace» = 1…*{ '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}' } ;
pub mod whitespace {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «IGNORE» = { «Whitespace» | «Comment» } ;
pub mod ignore {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<Box<ignore::_T1>>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        Whitespace(whitespace::N),
        Comment(comment::N),
    }
}

/// «IdentifierFollow» = «IdentifierStart» | '0'…'9' ;
pub mod identifier_follow {
    #[allow(unused_imports)]
    use super::*;
    pub type N = FixedTerminal<1usize>;
}

/// «StringChar» = ¬( '\'' | '\\' ) | '\\' ( '\'' | '\\' | 'u{' 1…6*{ «HexDigit» } '}' ) ;
pub mod string_char {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<string_char::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        NotQuoteOrBackslash(FixedTerminal<1usize>),
        Escape(Box<string_char::Escape>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Escape {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub backslash_char: FixedTerminal<1usize>,
        pub quote_or_backslash_or_hex_escape: Box<string_char::QuoteOrBackslashOrHexEscape>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum QuoteOrBackslashOrHexEscape {
        _0(FixedTerminal<1usize>),
        _T1(Box<string_char::_T1>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub u_open_brace: FixedTerminal<2usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _1: usize,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
}

/// «RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;
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

/// «SingleCharString» = '\'' «StringChar» '\'' ;
pub mod single_char_string {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<single_char_string::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_0: FixedTerminal<1usize>,
        pub string_char: string_char::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_1: FixedTerminal<1usize>,
    }
}

/// «String» = '\'' { «StringChar» } '\'' ;
pub mod string {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<string::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_0: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub string_chars: Vec<string_char::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub quote_char_1: FixedTerminal<1usize>,
    }
}

/// grouped = '(' expression ')' ;
pub mod grouped {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<grouped::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_paren_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_paren_char: FixedTerminal<1usize>,
    }
}

/// optional = '[' expression ']' ;
pub mod optional {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<optional::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_bracket_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_bracket_char: FixedTerminal<1usize>,
    }
}

/// repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
pub mod repetition_prefix {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<repetition_prefix::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub _t1: Box<repetition_prefix::_T1>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub star_char: FixedTerminal<1usize>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T1 {
        _T2(Box<repetition_prefix::_T2>),
        _T4(Box<repetition_prefix::_T4>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T4 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ellipsis_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub number: number::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T2 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub number: number::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t3: Option<Box<repetition_prefix::_T3>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T3 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ellipsis_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub number: Option<number::N>,
    }
}

/// repetitionSeparator = '/' expression ;
pub mod repetition_separator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<repetition_separator::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub slash_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: expression::N,
    }
}

/// «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
pub mod identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<identifier::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        _T1(Box<identifier::_T1>),
        RawIdentifier(raw_identifier::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_double_angle_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub raw_identifier: raw_identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_double_angle_char: FixedTerminal<1usize>,
    }
}

/// charRange = «SingleCharString» '…' «SingleCharString» ;
pub mod char_range {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<char_range::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub single_char_string_0: single_char_string::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ellipsis_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        pub single_char_string_1: single_char_string::N,
    }
}

/// repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
pub mod repeated {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<repeated::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub repetition_prefix: Option<repetition_prefix::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub open_brace_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub repetition_separator: Option<repetition_separator::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_3: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub close_brace_char: FixedTerminal<1usize>,
    }
}

/// productionReference = «Identifier» ;
pub mod production_reference {
    #[allow(unused_imports)]
    use super::*;
    pub type N = identifier::N;
}

/// primary = productionReference | grouped | optional | repeated | charRange | «EOF» | «String» ;
pub mod primary {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<primary::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _T0 {
        ProductionReference(production_reference::N),
        Grouped(grouped::N),
        Optional(optional::N),
        Repeated(repeated::N),
        CharRange(char_range::N),
        Dollar(FixedTerminal<1usize>),
        String(string::N),
    }
}

/// negation = [ '¬' ] primary ;
pub mod negation {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<negation::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub not_char: Option<FixedTerminal<1usize>>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        pub primary: primary::N,
    }
}

/// difference = negation [ '-' negation ] ;
pub mod difference {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<difference::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub negation: negation::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub _t1: Option<Box<difference::_T1>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T1 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub minus_char: FixedTerminal<1usize>,
        pub negation: negation::N,
    }
}

/// sequence = 1…*{ difference } ;
pub mod sequence {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<difference::N>;
}

/// expression = 1…*{ sequence / '|' } ;
pub mod expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<expression::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub sequences: Vec<sequence::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub bar_chars: Vec<FixedTerminal<1usize>>,
    }
}

/// production = «Identifier» '=' expression ';' ;
pub mod production {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<production::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _T0 {
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub equal_char: FixedTerminal<1usize>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub semicolon_char: FixedTerminal<1usize>,
    }
}

/// grammar = «IGNORE» { production } $ ;
pub mod grammar {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<grammar::_T0>;
    #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct _T0 {
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub productions: Vec<production::N>,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
        pub end_marker: (),
    }
}
