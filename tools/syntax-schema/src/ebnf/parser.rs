use chumsky::prelude::*;
use chumsky::Parser;
pub type ErrorType = Simple<char>;
pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;

// prelude attribute is no longer neccessary

mod comment {
    // «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } 1…*{ '*' } '/' ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub usize, pub Vec<C0>, pub Vec<char>, pub char);
    pub enum C0 {
        _0(char),
        _1((Vec<char>, char)),
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
    pub struct N(pub char, pub expression::N, pub char);
}

mod optional {
    // optional = '[' expression ']' ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub char, pub expression::N, pub char);
}

mod repetition_separator {
    // repetitionSeparator = '/' expression ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub char, pub expression::N);
}

mod ignore {
    // «IGNORE» = { «Whitespace» | «Comment» } ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = Vec<C0>;
    pub enum C0 {
        _0(char),
        _1(comment::N),
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
    pub type N = C0;
    pub enum C1 {
        _0(char),
        _1(char),
        _2((usize, Vec<char>, char)),
    }
    pub enum C0 {
        _0(char),
        _1((char, C1)),
    }
}

mod repetition_prefix {
    // repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub C0, pub char);
    pub enum C0 {
        _0((number::N, Option<(char, Option<number::N>)>)),
        _1((char, number::N)),
    }
}

mod raw_identifier {
    // «RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub char, pub Vec<char>);
}

mod single_char_string {
    // «SingleCharString» = '\'' «StringChar» '\'' ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub char, pub string_char::N, pub char);
}

mod string {
    // «String» = '\'' { «StringChar» } '\'' ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub char, pub Vec<string_char::N>, pub char);
}

mod repeated {
    // repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(
        pub Option<repetition_prefix::N>,
        pub char,
        pub expression::N,
        pub Option<repetition_separator::N>,
        pub char,
    );
}

mod identifier {
    // «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
    #[allow(unused_imports)]
    use super::*;
    pub type N = C0;
    pub enum C0 {
        _0((char, raw_identifier::N, char)),
        _1(raw_identifier::N),
    }
}

mod char_range {
    // charRange = «SingleCharString» '…' «SingleCharString» ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(
        pub single_char_string::N,
        pub char,
        pub single_char_string::N,
    );
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
    pub type N = C0;
    pub enum C0 {
        _0(production_reference::N),
        _1(grouped::N),
        _2(optional::N),
        _3(repeated::N),
        _4(char_range::N),
        _5(char),
        _6(string::N),
    }
}

mod negation {
    // negation = [ '¬' ] primary ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub Option<char>, pub primary::N);
}

mod difference {
    // difference = negation [ '-' negation ] ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub negation::N, pub Option<(char, negation::N)>);
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
    pub struct N(pub sequence::N, pub Vec<(char, sequence::N)>);
}

mod production {
    // production = «Identifier» '=' expression ';' ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub identifier::N, pub char, pub expression::N, pub char);
}

mod grammar {
    // grammar = «IGNORE» { production } $ ;
    #[allow(unused_imports)]
    use super::*;
    pub struct N(pub ignore::N, pub Vec<production::N>, pub ());
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
                    filter(|&c: &char| c != '*').map(comment::C0::_0),
                    just('*')
                        .repeated()
                        .at_least(1usize)
                        .then(filter(|&c: &char| c != '*' && c != '/'))
                        .map(comment::C0::_1),
                ))
                .repeated(),
            )
            .then(just('*').repeated().at_least(1usize))
            .then(just('/'))
            .map(|(((_0, _1), _2), _3)| (_0, _1, _2, _3))
            .map(|(_0, _1, _2, _3)| comment::N(_0, _1, _2, _3));

        // «Whitespace» = '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}' ;
        let whitespace_parser = filter(|&c: &char| c == '\t' || c == '\n' || c == '\r' || c == ' ');

        // grouped = '(' expression ')' ;
        let grouped_parser = just('(')
            .then(expression_parser.clone())
            .then(just(')'))
            .map(|((_0, _1), _2)| (_0, _1, _2))
            .map(|(_0, _1, _2)| grouped::N(_0, _1, _2));

        // optional = '[' expression ']' ;
        let optional_parser = just('[')
            .then(expression_parser.clone())
            .then(just(']'))
            .map(|((_0, _1), _2)| (_0, _1, _2))
            .map(|(_0, _1, _2)| optional::N(_0, _1, _2));

        // repetitionSeparator = '/' expression ;
        let repetition_separator_parser = just('/')
            .then(expression_parser.clone())
            .map(|(_0, _1)| repetition_separator::N(_0, _1));

        // «IGNORE» = { «Whitespace» | «Comment» } ;
        ignore_parser.define(
            choice((
                filter(|&c: &char| c == '\t' || c == '\n' || c == '\r' || c == ' ')
                    .map(ignore::C0::_0),
                comment_parser.clone().map(ignore::C0::_1),
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
            filter(|&c: &char| c != '\'' && c != '\\').map(string_char::C0::_0),
            just('\\')
                .then(choice((
                    just('\'').map(string_char::C1::_0),
                    just('\\').map(string_char::C1::_1),
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
                        .map(|((_0, _1), _2)| (_0, _1, _2))
                        .map(string_char::C1::_2),
                )))
                .map(string_char::C0::_1),
        ));

        // repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
        let repetition_prefix_parser = choice((
            number_parser
                .clone()
                .then(just('…').then(number_parser.clone().or_not()).or_not())
                .map(repetition_prefix::C0::_0),
            just('…')
                .then(number_parser.clone())
                .map(repetition_prefix::C0::_1),
        ))
        .then(just('*'))
        .map(|(_0, _1)| repetition_prefix::N(_0, _1));

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
                .map(|(_0, _1)| raw_identifier::N(_0, _1));

        // «SingleCharString» = '\'' «StringChar» '\'' ;
        let single_char_string_parser = just('\'')
            .then(string_char_parser.clone())
            .then(just('\''))
            .map(|((_0, _1), _2)| (_0, _1, _2))
            .map(|(_0, _1, _2)| single_char_string::N(_0, _1, _2));

        // «String» = '\'' { «StringChar» } '\'' ;
        let string_parser = just('\'')
            .then(string_char_parser.clone().repeated())
            .then(just('\''))
            .map(|((_0, _1), _2)| (_0, _1, _2))
            .map(|(_0, _1, _2)| string::N(_0, _1, _2));

        // repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
        let repeated_parser = repetition_prefix_parser
            .clone()
            .or_not()
            .then(just('{'))
            .then(expression_parser.clone())
            .then(repetition_separator_parser.clone().or_not())
            .then(just('}'))
            .map(|((((_0, _1), _2), _3), _4)| (_0, _1, _2, _3, _4))
            .map(|(_0, _1, _2, _3, _4)| repeated::N(_0, _1, _2, _3, _4));

        // «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
        let identifier_parser = choice((
            just('«')
                .then(raw_identifier_parser.clone())
                .then(just('»'))
                .map(|((_0, _1), _2)| (_0, _1, _2))
                .map(identifier::C0::_0),
            raw_identifier_parser.clone().map(identifier::C0::_1),
        ));

        // charRange = «SingleCharString» '…' «SingleCharString» ;
        let char_range_parser = single_char_string_parser
            .clone()
            .then(just('…'))
            .then(single_char_string_parser.clone())
            .map(|((_0, _1), _2)| (_0, _1, _2))
            .map(|(_0, _1, _2)| char_range::N(_0, _1, _2));

        // productionReference = «Identifier» ;
        let production_reference_parser = identifier_parser.clone();

        // primary = productionReference | grouped | optional | repeated | charRange | «EOF» | «String» ;
        let primary_parser = choice((
            production_reference_parser.clone().map(primary::C0::_0),
            grouped_parser.clone().map(primary::C0::_1),
            optional_parser.clone().map(primary::C0::_2),
            repeated_parser.clone().map(primary::C0::_3),
            char_range_parser.clone().map(primary::C0::_4),
            just('$').map(primary::C0::_5),
            string_parser.clone().map(primary::C0::_6),
        ));

        // negation = [ '¬' ] primary ;
        let negation_parser = just('¬')
            .or_not()
            .then(primary_parser.clone())
            .map(|(_0, _1)| negation::N(_0, _1));

        // difference = negation [ '-' negation ] ;
        let difference_parser = negation_parser
            .clone()
            .then(just('-').then(negation_parser.clone()).or_not())
            .map(|(_0, _1)| difference::N(_0, _1));

        // sequence = 1…*{ difference } ;
        let sequence_parser = difference_parser.clone().repeated().at_least(1usize);

        // expression = 1…*{ sequence / '|' } ;
        expression_parser.define(
            sequence_parser
                .clone()
                .then(just('|').then(sequence_parser.clone()).repeated())
                .map(|(_0, _1)| expression::N(_0, _1)),
        );

        // production = «Identifier» '=' expression ';' ;
        let production_parser = identifier_parser
            .clone()
            .then(just('='))
            .then(expression_parser.clone())
            .then(just(';'))
            .map(|(((_0, _1), _2), _3)| (_0, _1, _2, _3))
            .map(|(_0, _1, _2, _3)| production::N(_0, _1, _2, _3));

        // grammar = «IGNORE» { production } $ ;
        let grammar_parser = ignore_parser
            .clone()
            .then(production_parser.clone().repeated())
            .then(end())
            .map(|((_0, _1), _2)| (_0, _1, _2))
            .map(|(_0, _1, _2)| grammar::N(_0, _1, _2));

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
