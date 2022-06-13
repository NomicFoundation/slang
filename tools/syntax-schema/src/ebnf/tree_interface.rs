#[doc = "«Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;"]
pub mod comment {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<comment::_S0>;
    pub struct _S0 {
        pub slash_star: usize,
        pub content: Box<comment::Content>,
        pub star_slash: usize,
    }
    pub struct Content {
        pub _c2s: Vec<Box<comment::_C2>>,
        pub star_chars: Vec<char>,
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
#[doc = "«Whitespace» = 1…*{ '\\u{9}' | '\\u{a}' | '\\u{d}' | '\\u{20}' } ;"]
pub mod whitespace {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<char>;
}
#[doc = "grouped = '(' expression ')' ;"]
pub mod grouped {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<grouped::_S0>;
    pub struct _S0 {
        pub open_paren_char: char,
        pub expression: expression::N,
        pub close_paren_char: char,
    }
}
#[doc = "optional = '[' expression ']' ;"]
pub mod optional {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<optional::_S0>;
    pub struct _S0 {
        pub open_bracket_char: char,
        pub expression: expression::N,
        pub close_bracket_char: char,
    }
}
#[doc = "repetitionSeparator = '/' expression ;"]
pub mod repetition_separator {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<repetition_separator::_S0>;
    pub struct _S0 {
        pub slash_char: char,
        pub expression: expression::N,
    }
}
#[doc = "«IGNORE» = { «Whitespace» | «Comment» } ;"]
pub mod ignore {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<Box<ignore::_C1>>;
    pub enum _C1 {
        Whitespace(whitespace::N),
        Comment(comment::N),
    }
}
#[doc = "«EOF» = '$' ;"]
pub mod eof {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "«HexDigit» = '0'…'9' | 'a'…'f' | 'A'…'F' ;"]
pub mod hex_digit {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "«IdentifierStart» = '_' | 'a'…'z' | 'A'…'Z' ;"]
pub mod identifier_start {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "«Number» = 1…*{ '0'…'9' } ;"]
pub mod number {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<char>;
}
#[doc = "«IdentifierFollow» = «IdentifierStart» | '0'…'9' ;"]
pub mod identifier_follow {
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}
#[doc = "«StringChar» = ¬( '\\'' | '\\\\' ) | '\\\\' ( '\\'' | '\\\\' | 'u{' 1…6*{ «HexDigit» } '}' ) ;"]
pub mod string_char {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<string_char::_C0>;
    pub enum _C0 {
        NotQuoteOrBackslash(char),
        Escape(Box<string_char::Escape>),
    }
    pub struct Escape {
        pub backslash_char: char,
        pub quote_or_backslash_or_hex_escape: Box<string_char::QuoteOrBackslashOrHexEscape>,
    }
    pub enum QuoteOrBackslashOrHexEscape {
        _0(usize),
        _S1(Box<string_char::_S1>),
    }
    pub struct _S1 {
        pub u_open_brace: usize,
        pub _1: Vec<char>,
        pub close_brace_char: char,
    }
}
#[doc = "repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;"]
pub mod repetition_prefix {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<repetition_prefix::_S0>;
    pub struct _S0 {
        pub _c1: Box<repetition_prefix::_C1>,
        pub star_char: char,
    }
    pub enum _C1 {
        _S2(Box<repetition_prefix::_S2>),
        _S6(Box<repetition_prefix::_S6>),
    }
    pub struct _S6 {
        pub ellipsis_char: char,
        pub number: number::N,
    }
    pub struct _S2 {
        pub number: number::N,
        pub _s4: Option<Box<repetition_prefix::_S4>>,
    }
    pub struct _S4 {
        pub ellipsis_char: char,
        pub number: Option<number::N>,
    }
}
#[doc = "«RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;"]
pub mod raw_identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<raw_identifier::_S0>;
    pub struct _S0 {
        pub _0: char,
        pub _1: Vec<char>,
    }
}
#[doc = "«SingleCharString» = '\\'' «StringChar» '\\'' ;"]
pub mod single_char_string {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<single_char_string::_S0>;
    pub struct _S0 {
        pub quote_char_0: char,
        pub string_char: string_char::N,
        pub quote_char_1: char,
    }
}
#[doc = "«String» = '\\'' { «StringChar» } '\\'' ;"]
pub mod string {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<string::_S0>;
    pub struct _S0 {
        pub quote_char_0: char,
        pub string_chars: Vec<string_char::N>,
        pub quote_char_1: char,
    }
}
#[doc = "repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;"]
pub mod repeated {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<repeated::_S0>;
    pub struct _S0 {
        pub repetition_prefix: Option<repetition_prefix::N>,
        pub open_brace_char: char,
        pub expression: expression::N,
        pub repetition_separator: Option<repetition_separator::N>,
        pub close_brace_char: char,
    }
}
#[doc = "«Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;"]
pub mod identifier {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<identifier::_C0>;
    pub enum _C0 {
        _S1(Box<identifier::_S1>),
        RawIdentifier(raw_identifier::N),
    }
    pub struct _S1 {
        pub open_double_angle_char: char,
        pub raw_identifier: raw_identifier::N,
        pub close_double_angle_char: char,
    }
}
#[doc = "charRange = «SingleCharString» '…' «SingleCharString» ;"]
pub mod char_range {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<char_range::_S0>;
    pub struct _S0 {
        pub single_char_string_0: single_char_string::N,
        pub ellipsis_char: char,
        pub single_char_string_1: single_char_string::N,
    }
}
#[doc = "productionReference = «Identifier» ;"]
pub mod production_reference {
    #[allow(unused_imports)]
    use super::*;
    pub type N = identifier::N;
}
#[doc = "primary = productionReference | grouped | optional | repeated | charRange | «EOF» | «String» ;"]
pub mod primary {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<primary::_C0>;
    pub enum _C0 {
        ProductionReference(production_reference::N),
        Grouped(grouped::N),
        Optional(optional::N),
        Repeated(repeated::N),
        CharRange(char_range::N),
        Dollar(usize),
        String(string::N),
    }
}
#[doc = "negation = [ '¬' ] primary ;"]
pub mod negation {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<negation::_S0>;
    pub struct _S0 {
        pub not_char: Option<char>,
        pub primary: primary::N,
    }
}
#[doc = "difference = negation [ '-' negation ] ;"]
pub mod difference {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<difference::_S0>;
    pub struct _S0 {
        pub negation: negation::N,
        pub _s2: Option<Box<difference::_S2>>,
    }
    pub struct _S2 {
        pub minus_char: char,
        pub negation: negation::N,
    }
}
#[doc = "sequence = 1…*{ difference } ;"]
pub mod sequence {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<difference::N>;
}
#[doc = "expression = 1…*{ sequence / '|' } ;"]
pub mod expression {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<expression::_S0>;
    pub struct _S0 {
        pub sequences: Vec<sequence::N>,
        pub bar_chars: Vec<char>,
    }
}
#[doc = "production = «Identifier» '=' expression ';' ;"]
pub mod production {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<production::_S0>;
    pub struct _S0 {
        pub identifier: identifier::N,
        pub equal_char: char,
        pub expression: expression::N,
        pub semicolon_char: char,
    }
}
#[doc = "grammar = «IGNORE» { production } $ ;"]
pub mod grammar {
    #[allow(unused_imports)]
    use super::*;
    pub type N = Box<grammar::_S0>;
    pub struct _S0 {
        pub ignore: ignore::N,
        pub productions: Vec<production::N>,
        pub end_marker: (),
    }
}
