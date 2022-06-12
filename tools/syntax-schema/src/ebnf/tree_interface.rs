#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[inline]
fn usize_is_zero(v: &usize) -> bool {
    *v == 0
}

/// «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
pub mod comment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<comment::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(skip)]
        pub slash_star: (),
        pub content: Box<comment::Content>,
        #[serde(skip)]
        pub star_slash: (),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Content {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub _c2s: Vec<Box<comment::_C2>>,
        #[serde(default, skip_serializing_if = "usize_is_zero")]
        pub star_chars: usize,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C2 {
        StarChar(()),
        _S3(Box<comment::_S3>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S3 {
        #[serde(default, skip_serializing_if = "usize_is_zero")]
        pub star_chars: usize,
        #[serde(skip)]
        pub _1: (),
    }
}

/// «Whitespace» = 1…*{ '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}' } ;
pub mod whitespace {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// grouped = '(' expression ')' ;
pub mod grouped {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<grouped::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(skip)]
        pub open_paren_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_0: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_1: ignore::N,
        #[serde(skip)]
        pub close_paren_char: (),
    }
}

/// optional = '[' expression ']' ;
pub mod optional {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<optional::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(skip)]
        pub open_bracket_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_0: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_1: ignore::N,
        #[serde(skip)]
        pub close_bracket_char: (),
    }
}

/// repetitionSeparator = '/' expression ;
pub mod repetition_separator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<repetition_separator::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(skip)]
        pub slash_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
        pub expression: expression::N,
    }
}

/// «IGNORE» = { «Whitespace» | «Comment» } ;
pub mod ignore {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<Box<ignore::_C1>>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        Whitespace(whitespace::N),
        Comment(comment::N),
    }
}

/// «EOF» = '$' ;
pub mod eof {
    #[allow(unused_imports)]
    use super::*;
    pub type N = ();
}

/// «HexDigit» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
pub mod hex_digit {
    #[allow(unused_imports)]
    use super::*;
    pub type N = ();
}

/// «IdentifierStart» = '_' | 'a'…'z' | 'A'…'Z' ;
pub mod identifier_start {
    #[allow(unused_imports)]
    use super::*;
    pub type N = ();
}

/// «Number» = 1…*{ '0'…'9' } ;
pub mod number {
    #[allow(unused_imports)]
    use super::*;
    pub type N = usize;
}

/// «IdentifierFollow» = «IdentifierStart» | '0'…'9' ;
pub mod identifier_follow {
    #[allow(unused_imports)]
    use super::*;
    pub type N = ();
}

/// «StringChar» = ¬( '\'' | '\\' ) | '\\' ( '\'' | '\\' | 'u{' 1…6*{ «HexDigit» } '}' ) ;
pub mod string_char {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<string_char::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        NotQuoteOrBackslash(()),
        Escape(Box<string_char::Escape>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Escape {
        #[serde(skip)]
        pub backslash_char: (),
        pub quote_or_backslash_or_hex_escape: Box<string_char::QuoteOrBackslashOrHexEscape>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum QuoteOrBackslashOrHexEscape {
        _0(()),
        _S1(Box<string_char::_S1>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(skip)]
        pub u_open_brace: (),
        #[serde(default, skip_serializing_if = "usize_is_zero")]
        pub _1: usize,
        #[serde(skip)]
        pub close_brace_char: (),
    }
}

/// repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
pub mod repetition_prefix {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<repetition_prefix::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub _c1: Box<repetition_prefix::_C1>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
        #[serde(skip)]
        pub star_char: (),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C1 {
        _S2(Box<repetition_prefix::_S2>),
        _S6(Box<repetition_prefix::_S6>),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S6 {
        #[serde(skip)]
        pub ellipsis_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "usize_is_zero")]
        pub number: number::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(default, skip_serializing_if = "usize_is_zero")]
        pub number: number::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub _s4: Option<Box<repetition_prefix::_S4>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S4 {
        #[serde(skip)]
        pub ellipsis_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub number: Option<number::N>,
    }
}

/// «RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;
pub mod raw_identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<raw_identifier::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(skip)]
        pub _0: (),
        #[serde(default, skip_serializing_if = "usize_is_zero")]
        pub _1: usize,
    }
}

/// «SingleCharString» = '\'' «StringChar» '\'' ;
pub mod single_char_string {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<single_char_string::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(skip)]
        pub quote_char_0: (),
        pub string_char: string_char::N,
        #[serde(skip)]
        pub quote_char_1: (),
    }
}

/// «String» = '\'' { «StringChar» } '\'' ;
pub mod string {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<string::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(skip)]
        pub quote_char_0: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub string_chars: Vec<string_char::N>,
        #[serde(skip)]
        pub quote_char_1: (),
    }
}

/// repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
pub mod repeated {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<repeated::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub repetition_prefix: Option<repetition_prefix::N>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_0: ignore::N,
        #[serde(skip)]
        pub open_brace_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_1: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_2: ignore::N,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub repetition_separator: Option<repetition_separator::N>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_3: ignore::N,
        #[serde(skip)]
        pub close_brace_char: (),
    }
}

/// «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
pub mod identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<identifier::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        _S1(Box<identifier::_S1>),
        RawIdentifier(raw_identifier::N),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(skip)]
        pub open_double_angle_char: (),
        pub raw_identifier: raw_identifier::N,
        #[serde(skip)]
        pub close_double_angle_char: (),
    }
}

/// charRange = «SingleCharString» '…' «SingleCharString» ;
pub mod char_range {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<char_range::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub single_char_string_0: single_char_string::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_0: ignore::N,
        #[serde(skip)]
        pub ellipsis_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_1: ignore::N,
        pub single_char_string_1: single_char_string::N,
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
    pub type N = Box<primary::_C0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum _C0 {
        ProductionReference(production_reference::N),
        Grouped(grouped::N),
        Optional(optional::N),
        Repeated(repeated::N),
        CharRange(char_range::N),
        Dollar(()),
        String(string::N),
    }
}

/// negation = [ '¬' ] primary ;
pub mod negation {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<negation::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub not_char: Option<()>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
        pub primary: primary::N,
    }
}

/// difference = negation [ '-' negation ] ;
pub mod difference {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<difference::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub negation: negation::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub _s2: Option<Box<difference::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(skip)]
        pub minus_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
        pub negation: negation::N,
    }
}

/// sequence = 1…*{ difference } ;
pub mod sequence {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<Box<sequence::_S1>>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S1 {
        pub difference: difference::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
    }
}

/// expression = 1…*{ sequence / '|' } ;
pub mod expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<expression::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub _s1s: Vec<Box<expression::_S1>>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub _s2s: Vec<Box<expression::_S2>>,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        #[serde(skip)]
        pub bar_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S1 {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub sequence: sequence::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
    }
}

/// production = «Identifier» '=' expression ';' ;
pub mod production {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<production::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        pub identifier: identifier::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_0: ignore::N,
        #[serde(skip)]
        pub equal_char: (),
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_1: ignore::N,
        pub expression: expression::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_2: ignore::N,
        #[serde(skip)]
        pub semicolon_char: (),
    }
}

/// grammar = «IGNORE» { production } $ ;
pub mod grammar {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<grammar::_S0>;
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S0 {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_0: ignore::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_1: ignore::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub _s2s: Vec<Box<grammar::_S2>>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore_2: ignore::N,
        #[serde(skip)]
        pub end_marker: (),
    }
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct _S2 {
        pub production: production::N,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ignore: ignore::N,
    }
}
