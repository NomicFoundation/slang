use chumsky::prelude::*;
use chumsky::Parser;
pub type ErrorType = Simple<char>;
pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;
fn repetition_mapper<E, S>((e, es): (E, Vec<(S, E)>)) -> (Vec<E>, Vec<S>) {
    let mut expressions = vec![e];
    let mut separators = vec![];
    for (s, e) in es.into_iter() {
        separators.push(s);
        expressions.push(e);
    }
    (expressions, separators)
}

// prelude attribute is no longer neccessary

mod comment {
    // «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } 1…*{ '*' } '/' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub _0: usize,
        pub _c2s: Vec<_C2>,
        pub star_chars: Vec<char>,
        pub slash_char: char,
    }
    impl _S0 {
        pub fn new(
            (((_0, _c2s), star_chars), slash_char): (((usize, Vec<_C2>), Vec<char>), char),
        ) -> Self {
            Self {
                _0,
                _c2s,
                star_chars,
                slash_char,
            }
        }
    }
    pub enum _C2 {
        StarChar(char),
        S3(_S3),
    }
    pub struct _S3 {
        pub star_chars: Vec<char>,
        pub _1: char,
    }
    impl _S3 {
        pub fn new((star_chars, _1): (Vec<char>, char)) -> Self {
            Self { star_chars, _1 }
        }
    }
}

mod whitespace {
    // «Whitespace» = '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}

mod grouped {
    // grouped = '(' expression ')' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub open_paren_char: char,
        pub expression: expression::N,
        pub close_paren_char: char,
    }
    impl _S0 {
        pub fn new(
            ((open_paren_char, expression), close_paren_char): ((char, expression::N), char),
        ) -> Self {
            Self {
                open_paren_char,
                expression,
                close_paren_char,
            }
        }
    }
}

mod optional {
    // optional = '[' expression ']' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub open_bracket_char: char,
        pub expression: expression::N,
        pub close_bracket_char: char,
    }
    impl _S0 {
        pub fn new(
            ((open_bracket_char, expression), close_bracket_char): ((char, expression::N), char),
        ) -> Self {
            Self {
                open_bracket_char,
                expression,
                close_bracket_char,
            }
        }
    }
}

mod repetition_separator {
    // repetitionSeparator = '/' expression ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub slash_char: char,
        pub expression: expression::N,
    }
    impl _S0 {
        pub fn new((slash_char, expression): (char, expression::N)) -> Self {
            Self {
                slash_char,
                expression,
            }
        }
    }
}

mod ignore {
    // «IGNORE» = { «Whitespace» | «Comment» } ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<_C1>;
    pub enum _C1 {
        _0(char),
        Comment(comment::N),
    }
}

mod eof {
    // «EOF» = '$' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}

mod hex_digit {
    // «HexDigit» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}

mod identifier_start {
    // «IdentifierStart» = '_' | 'a'…'z' | 'A'…'Z' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}

mod number {
    // «Number» = 1…*{ '0'…'9' } ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<char>;
}

mod identifier_follow {
    // «IdentifierFollow» = «IdentifierStart» | '0'…'9' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = char;
}

mod string_char {
    // «StringChar» = ¬( '\'' | '\\' ) | '\\' ( '\'' | '\\' | 'u{' 1…6*{ «HexDigit» } '}' ) ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _C0;
    pub enum _C0 {
        _0(char),
        S1(_S1),
    }
    pub struct _S1 {
        pub backslash_char: char,
        pub _c2: _C2,
    }
    impl _S1 {
        pub fn new((backslash_char, _c2): (char, _C2)) -> Self {
            Self {
                backslash_char,
                _c2,
            }
        }
    }
    pub enum _C2 {
        QuoteChar(char),
        BackslashChar(char),
        S3(_S3),
    }
    pub struct _S3 {
        pub _0: usize,
        pub _1: Vec<char>,
        pub close_brace_char: char,
    }
    impl _S3 {
        pub fn new(((_0, _1), close_brace_char): ((usize, Vec<char>), char)) -> Self {
            Self {
                _0,
                _1,
                close_brace_char,
            }
        }
    }
}

mod repetition_prefix {
    // repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub _c1: _C1,
        pub star_char: char,
    }
    impl _S0 {
        pub fn new((_c1, star_char): (_C1, char)) -> Self {
            Self { _c1, star_char }
        }
    }
    pub enum _C1 {
        S2(_S2),
        S6(_S6),
    }
    pub struct _S6 {
        pub ellipsis_char: char,
        pub number: number::N,
    }
    impl _S6 {
        pub fn new((ellipsis_char, number): (char, number::N)) -> Self {
            Self {
                ellipsis_char,
                number,
            }
        }
    }
    pub struct _S2 {
        pub number: number::N,
        pub _s4: Option<_S4>,
    }
    impl _S2 {
        pub fn new((number, _s4): (number::N, Option<_S4>)) -> Self {
            Self { number, _s4 }
        }
    }
    pub struct _S4 {
        pub ellipsis_char: char,
        pub number: Option<number::N>,
    }
    impl _S4 {
        pub fn new((ellipsis_char, number): (char, Option<number::N>)) -> Self {
            Self {
                ellipsis_char,
                number,
            }
        }
    }
}

mod raw_identifier {
    // «RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub _0: char,
        pub _1: Vec<char>,
    }
    impl _S0 {
        pub fn new((_0, _1): (char, Vec<char>)) -> Self {
            Self { _0, _1 }
        }
    }
}

mod single_char_string {
    // «SingleCharString» = '\'' «StringChar» '\'' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub quote_char_0: char,
        pub string_char: string_char::N,
        pub quote_char_1: char,
    }
    impl _S0 {
        pub fn new(
            ((quote_char_0, string_char), quote_char_1): ((char, string_char::N), char),
        ) -> Self {
            Self {
                quote_char_0,
                string_char,
                quote_char_1,
            }
        }
    }
}

mod string {
    // «String» = '\'' { «StringChar» } '\'' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub quote_char_0: char,
        pub string_chars: Vec<string_char::N>,
        pub quote_char_1: char,
    }
    impl _S0 {
        pub fn new(
            ((quote_char_0, string_chars), quote_char_1): ((char, Vec<string_char::N>), char),
        ) -> Self {
            Self {
                quote_char_0,
                string_chars,
                quote_char_1,
            }
        }
    }
}

mod repeated {
    // repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub repetition_prefix: Option<repetition_prefix::N>,
        pub open_brace_char: char,
        pub expression: expression::N,
        pub repetition_separator: Option<repetition_separator::N>,
        pub close_brace_char: char,
    }
    impl _S0 {
        pub fn new(
            (
                (((repetition_prefix, open_brace_char), expression), repetition_separator),
                close_brace_char,
            ): (
                (
                    ((Option<repetition_prefix::N>, char), expression::N),
                    Option<repetition_separator::N>,
                ),
                char,
            ),
        ) -> Self {
            Self {
                repetition_prefix,
                open_brace_char,
                expression,
                repetition_separator,
                close_brace_char,
            }
        }
    }
}

mod identifier {
    // «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _C0;
    pub enum _C0 {
        S1(_S1),
        RawIdentifier(raw_identifier::N),
    }
    pub struct _S1 {
        pub open_double_angle_char: char,
        pub raw_identifier: raw_identifier::N,
        pub close_double_angle_char: char,
    }
    impl _S1 {
        pub fn new(
            ((open_double_angle_char, raw_identifier), close_double_angle_char): (
                (char, raw_identifier::N),
                char,
            ),
        ) -> Self {
            Self {
                open_double_angle_char,
                raw_identifier,
                close_double_angle_char,
            }
        }
    }
}

mod char_range {
    // charRange = «SingleCharString» '…' «SingleCharString» ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub single_char_string_0: single_char_string::N,
        pub ellipsis_char: char,
        pub single_char_string_1: single_char_string::N,
    }
    impl _S0 {
        pub fn new(
            ((single_char_string_0, ellipsis_char), single_char_string_1): (
                (single_char_string::N, char),
                single_char_string::N,
            ),
        ) -> Self {
            Self {
                single_char_string_0,
                ellipsis_char,
                single_char_string_1,
            }
        }
    }
}

mod production_reference {
    // productionReference = «Identifier» ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = identifier::N;
}

mod primary {
    // primary = productionReference | grouped | optional | repeated | charRange | «EOF» | «String» ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _C0;
    pub enum _C0 {
        ProductionReference(production_reference::N),
        Grouped(grouped::N),
        Optional(optional::N),
        Repeated(repeated::N),
        CharRange(char_range::N),
        DollarChar(char),
        String(string::N),
    }
}

mod negation {
    // negation = [ '¬' ] primary ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub not_char: Option<char>,
        pub primary: primary::N,
    }
    impl _S0 {
        pub fn new((not_char, primary): (Option<char>, primary::N)) -> Self {
            Self { not_char, primary }
        }
    }
}

mod difference {
    // difference = negation [ '-' negation ] ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub negation: negation::N,
        pub _s2: Option<_S2>,
    }
    impl _S0 {
        pub fn new((negation, _s2): (negation::N, Option<_S2>)) -> Self {
            Self { negation, _s2 }
        }
    }
    pub struct _S2 {
        pub minus_char: char,
        pub negation: negation::N,
    }
    impl _S2 {
        pub fn new((minus_char, negation): (char, negation::N)) -> Self {
            Self {
                minus_char,
                negation,
            }
        }
    }
}

mod sequence {
    // sequence = 1…*{ difference } ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<difference::N>;
}

mod expression {
    // expression = 1…*{ sequence / '|' } ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub sequences: Vec<sequence::N>,
        pub bar_chars: Vec<char>,
    }
    impl _S0 {
        pub fn new((sequences, bar_chars): (Vec<sequence::N>, Vec<char>)) -> Self {
            Self {
                sequences,
                bar_chars,
            }
        }
    }
}

mod production {
    // production = «Identifier» '=' expression ';' ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub identifier: identifier::N,
        pub equal_char: char,
        pub expression: expression::N,
        pub semicolon_char: char,
    }
    impl _S0 {
        pub fn new(
            (((identifier, equal_char), expression), semicolon_char): (
                ((identifier::N, char), expression::N),
                char,
            ),
        ) -> Self {
            Self {
                identifier,
                equal_char,
                expression,
                semicolon_char,
            }
        }
    }
}

mod grammar {
    // grammar = «IGNORE» { production } $ ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = _S0;
    pub struct _S0 {
        pub ignore: ignore::N,
        pub productions: Vec<production::N>,
        pub _2: (),
    }
    impl _S0 {
        pub fn new(((ignore, productions), _2): ((ignore::N, Vec<production::N>), ())) -> Self {
            Self {
                ignore,
                productions,
                _2,
            }
        }
    }
}

#[allow(dead_code)]
pub struct Parsers {
    pub comment: ParserType<comment::N>,
    pub whitespace: ParserType<whitespace::N>,
    pub grouped: ParserType<grouped::N>,
    pub optional: ParserType<optional::N>,
    pub repetition_separator: ParserType<repetition_separator::N>,
    pub ignore: ParserType<ignore::N>,
    pub eof: ParserType<eof::N>,
    pub hex_digit: ParserType<hex_digit::N>,
    pub identifier_start: ParserType<identifier_start::N>,
    pub number: ParserType<number::N>,
    pub identifier_follow: ParserType<identifier_follow::N>,
    pub string_char: ParserType<string_char::N>,
    pub repetition_prefix: ParserType<repetition_prefix::N>,
    pub raw_identifier: ParserType<raw_identifier::N>,
    pub single_char_string: ParserType<single_char_string::N>,
    pub string: ParserType<string::N>,
    pub repeated: ParserType<repeated::N>,
    pub identifier: ParserType<identifier::N>,
    pub char_range: ParserType<char_range::N>,
    pub production_reference: ParserType<production_reference::N>,
    pub primary: ParserType<primary::N>,
    pub negation: ParserType<negation::N>,
    pub difference: ParserType<difference::N>,
    pub sequence: ParserType<sequence::N>,
    pub expression: ParserType<expression::N>,
    pub production: ParserType<production::N>,
    pub grammar: ParserType<grammar::N>,
}

impl Parsers {
    pub fn new() -> Self {
        let mut ignore_parser = Recursive::declare();

        let mut expression_parser = Recursive::declare();

        // «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } 1…*{ '*' } '/' ;
        let comment_parser = just("/*")
            .map(|_| 2usize)
            .then(
                choice((
                    filter(|&c: &char| c != '*').map(comment::_C2::StarChar),
                    just('*')
                        .repeated()
                        .at_least(1usize)
                        .then(filter(|&c: &char| c != '*' && c != '/'))
                        .map(comment::_S3::new)
                        .map(comment::_C2::S3),
                ))
                .repeated(),
            )
            .then(just('*').repeated().at_least(1usize))
            .then(just('/'))
            .map(comment::_S0::new);

        // «Whitespace» = '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}' ;
        let whitespace_parser = filter(|&c: &char| c == '\t' || c == '\n' || c == '\r' || c == ' ');

        // grouped = '(' expression ')' ;
        let grouped_parser = just('(')
            .then(expression_parser.clone())
            .then(just(')'))
            .map(grouped::_S0::new);

        // optional = '[' expression ']' ;
        let optional_parser = just('[')
            .then(expression_parser.clone())
            .then(just(']'))
            .map(optional::_S0::new);

        // repetitionSeparator = '/' expression ;
        let repetition_separator_parser = just('/')
            .then(expression_parser.clone())
            .map(repetition_separator::_S0::new);

        // «IGNORE» = { «Whitespace» | «Comment» } ;
        ignore_parser.define(
            choice((
                filter(|&c: &char| c == '\t' || c == '\n' || c == '\r' || c == ' ')
                    .map(ignore::_C1::_0),
                comment_parser.clone().map(ignore::_C1::Comment),
            ))
            .repeated(),
        );

        // «EOF» = '$' ;
        let eof_parser = just('$');

        // «HexDigit» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
        let hex_digit_parser = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        });

        // «IdentifierStart» = '_' | 'a'…'z' | 'A'…'Z' ;
        let identifier_start_parser =
            filter(|&c: &char| c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z'));

        // «Number» = 1…*{ '0'…'9' } ;
        let number_parser = filter(|&c: &char| ('0' <= c && c <= '9'))
            .repeated()
            .at_least(1usize);

        // «IdentifierFollow» = «IdentifierStart» | '0'…'9' ;
        let identifier_follow_parser = filter(|&c: &char| {
            c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || ('0' <= c && c <= '9')
        });

        // «StringChar» = ¬( '\'' | '\\' ) | '\\' ( '\'' | '\\' | 'u{' 1…6*{ «HexDigit» } '}' ) ;
        let string_char_parser = choice((
            filter(|&c: &char| c != '\'' && c != '\\').map(string_char::_C0::_0),
            just('\\')
                .then(choice((
                    just('\'').map(string_char::_C2::QuoteChar),
                    just('\\').map(string_char::_C2::BackslashChar),
                    just("u{")
                        .map(|_| 2usize)
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')
                            })
                            .repeated()
                            .at_least(1usize)
                            .at_most(6usize),
                        )
                        .then(just('}'))
                        .map(string_char::_S3::new)
                        .map(string_char::_C2::S3),
                )))
                .map(string_char::_S1::new)
                .map(string_char::_C0::S1),
        ));

        // repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
        let repetition_prefix_parser = choice((
            number_parser
                .clone()
                .then(
                    just('…')
                        .then(number_parser.clone().or_not())
                        .map(repetition_prefix::_S4::new)
                        .or_not(),
                )
                .map(repetition_prefix::_S2::new)
                .map(repetition_prefix::_C1::S2),
            just('…')
                .then(number_parser.clone())
                .map(repetition_prefix::_S6::new)
                .map(repetition_prefix::_C1::S6),
        ))
        .then(just('*'))
        .map(repetition_prefix::_S0::new);

        // «RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;
        let raw_identifier_parser =
            filter(|&c: &char| c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z'))
                .then(
                    filter(|&c: &char| {
                        c == '_'
                            || ('a' <= c && c <= 'z')
                            || ('A' <= c && c <= 'Z')
                            || ('0' <= c && c <= '9')
                    })
                    .repeated(),
                )
                .map(raw_identifier::_S0::new);

        // «SingleCharString» = '\'' «StringChar» '\'' ;
        let single_char_string_parser = just('\'')
            .then(string_char_parser.clone())
            .then(just('\''))
            .map(single_char_string::_S0::new);

        // «String» = '\'' { «StringChar» } '\'' ;
        let string_parser = just('\'')
            .then(string_char_parser.clone().repeated())
            .then(just('\''))
            .map(string::_S0::new);

        // repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
        let repeated_parser = repetition_prefix_parser
            .clone()
            .or_not()
            .then(just('{'))
            .then(expression_parser.clone())
            .then(repetition_separator_parser.clone().or_not())
            .then(just('}'))
            .map(repeated::_S0::new);

        // «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
        let identifier_parser = choice((
            just('«')
                .then(raw_identifier_parser.clone())
                .then(just('»'))
                .map(identifier::_S1::new)
                .map(identifier::_C0::S1),
            raw_identifier_parser
                .clone()
                .map(identifier::_C0::RawIdentifier),
        ));

        // charRange = «SingleCharString» '…' «SingleCharString» ;
        let char_range_parser = single_char_string_parser
            .clone()
            .then(just('…'))
            .then(single_char_string_parser.clone())
            .map(char_range::_S0::new);

        // productionReference = «Identifier» ;
        let production_reference_parser = identifier_parser.clone();

        // primary = productionReference | grouped | optional | repeated | charRange | «EOF» | «String» ;
        let primary_parser = choice((
            production_reference_parser
                .clone()
                .map(primary::_C0::ProductionReference),
            grouped_parser.clone().map(primary::_C0::Grouped),
            optional_parser.clone().map(primary::_C0::Optional),
            repeated_parser.clone().map(primary::_C0::Repeated),
            char_range_parser.clone().map(primary::_C0::CharRange),
            just('$').map(primary::_C0::DollarChar),
            string_parser.clone().map(primary::_C0::String),
        ));

        // negation = [ '¬' ] primary ;
        let negation_parser = just('¬')
            .or_not()
            .then(primary_parser.clone())
            .map(negation::_S0::new);

        // difference = negation [ '-' negation ] ;
        let difference_parser = negation_parser
            .clone()
            .then(
                just('-')
                    .then(negation_parser.clone())
                    .map(difference::_S2::new)
                    .or_not(),
            )
            .map(difference::_S0::new);

        // sequence = 1…*{ difference } ;
        let sequence_parser = difference_parser.clone().repeated().at_least(1usize);

        // expression = 1…*{ sequence / '|' } ;
        expression_parser.define(
            sequence_parser
                .clone()
                .then(just('|').then(sequence_parser.clone()).repeated())
                .map(repetition_mapper)
                .map(expression::_S0::new),
        );

        // production = «Identifier» '=' expression ';' ;
        let production_parser = identifier_parser
            .clone()
            .then(just('='))
            .then(expression_parser.clone())
            .then(just(';'))
            .map(production::_S0::new);

        // grammar = «IGNORE» { production } $ ;
        let grammar_parser = ignore_parser
            .clone()
            .then(production_parser.clone().repeated())
            .then(end())
            .map(grammar::_S0::new);

        Self {
            comment: comment_parser.boxed(),
            whitespace: whitespace_parser.boxed(),
            grouped: grouped_parser.boxed(),
            optional: optional_parser.boxed(),
            repetition_separator: repetition_separator_parser.boxed(),
            ignore: ignore_parser.boxed(),
            eof: eof_parser.boxed(),
            hex_digit: hex_digit_parser.boxed(),
            identifier_start: identifier_start_parser.boxed(),
            number: number_parser.boxed(),
            identifier_follow: identifier_follow_parser.boxed(),
            string_char: string_char_parser.boxed(),
            repetition_prefix: repetition_prefix_parser.boxed(),
            raw_identifier: raw_identifier_parser.boxed(),
            single_char_string: single_char_string_parser.boxed(),
            string: string_parser.boxed(),
            repeated: repeated_parser.boxed(),
            identifier: identifier_parser.boxed(),
            char_range: char_range_parser.boxed(),
            production_reference: production_reference_parser.boxed(),
            primary: primary_parser.boxed(),
            negation: negation_parser.boxed(),
            difference: difference_parser.boxed(),
            sequence: sequence_parser.boxed(),
            expression: expression_parser.boxed(),
            production: production_parser.boxed(),
            grammar: grammar_parser.boxed(),
        }
    }
}
