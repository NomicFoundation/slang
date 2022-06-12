use super::parser_interface::*;
use super::tree_interface::*;
use chumsky::prelude::*;
use chumsky::primitive::Just;
use chumsky::Parser;
#[allow(dead_code)]
fn repetition_mapper<E, S>((e, es): (E, Vec<(S, E)>)) -> (Vec<E>, Vec<S>) {
    let mut expressions = vec![e];
    let mut separators = vec![];
    for (s, e) in es.into_iter() {
        separators.push(s);
        expressions.push(e);
    }
    (expressions, separators)
}
#[allow(dead_code)]
fn difference<M, MO, S, SO>(minuend: M, subtrahend: S) -> impl Parser<char, MO, Error = ErrorType>
where
    M: Clone + Parser<char, MO, Error = ErrorType>,
    S: Parser<char, SO, Error = ErrorType>,
{
    let minuend_end = minuend.clone().map_with_span(|_, span| span.end).rewind();
    let subtrahend_end = subtrahend
        .map_with_span(|_, span| span.end)
        .rewind()
        .or_else(|_| Ok(0));
    minuend_end
        .then(subtrahend_end)
        .validate(|(m, s), span, emit| {
            if m == s {
                emit(Simple::custom(span, "subtrahend matches minuend"))
            }
        })
        .ignore_then(minuend)
}
#[allow(dead_code)]
#[inline]
fn terminal(str: &str) -> Just<char, &str, ErrorType> {
    just(str)
}

impl Parsers {
    pub fn new() -> Self {
        let mut ignore_parser = Recursive::declare();

        let mut expression_parser = Recursive::declare();

        // «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
        let comment_parser = terminal("/*")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '*')
                        .map(|_| FixedTerminal::<1>())
                        .map(|v| Box::new(comment::_C2::StarChar(v))),
                    just('*')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| v.len())
                        .then(
                            filter(|&c: &char| c != '*' && c != '/').map(|_| FixedTerminal::<1>()),
                        )
                        .map(|v| Box::new(comment::_S3::new(v)))
                        .map(|v| Box::new(comment::_C2::_S3(v))),
                ))
                .repeated()
                .then(
                    just('*')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .map(|v| v.len()),
                )
                .map(|v| Box::new(comment::Content::new(v))),
            )
            .then(terminal("*/").ignored().map(|_| FixedTerminal::<2usize>()))
            .map(|v| Box::new(comment::_S0::new(v)))
            .boxed();

        // «Whitespace» = 1…*{ '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}' } ;
        let whitespace_parser = filter(|&c: &char| c == '\t' || c == '\n' || c == '\r' || c == ' ')
            .map(|_| FixedTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| v.len())
            .boxed();

        // grouped = '(' expression ')' ;
        let grouped_parser = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(grouped::_S0::new(v)))
            .boxed();

        // optional = '[' expression ']' ;
        let optional_parser = just('[')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(just(']').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(optional::_S0::new(v)))
            .boxed();

        // repetitionSeparator = '/' expression ;
        let repetition_separator_parser = just('/')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .map(|v| Box::new(repetition_separator::_S0::new(v)))
            .boxed();

        // «IGNORE» = { «Whitespace» | «Comment» } ;
        ignore_parser.define(
            choice((
                whitespace_parser
                    .clone()
                    .map(|v| Box::new(ignore::_C1::Whitespace(v))),
                comment_parser
                    .clone()
                    .map(|v| Box::new(ignore::_C1::Comment(v))),
            ))
            .repeated()
            .boxed(),
        );

        // «EOF» = '$' ;
        let eof_parser = just('$').map(|_| FixedTerminal::<1>()).boxed();

        // «HexDigit» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
        let hex_digit_parser = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        })
        .map(|_| FixedTerminal::<1>())
        .boxed();

        // «IdentifierStart» = '_' | 'a'…'z' | 'A'…'Z' ;
        let identifier_start_parser =
            filter(|&c: &char| c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z'))
                .map(|_| FixedTerminal::<1>())
                .boxed();

        // «Number» = 1…*{ '0'…'9' } ;
        let number_parser = filter(|&c: &char| ('0' <= c && c <= '9'))
            .map(|_| FixedTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| v.len())
            .boxed();

        // «IdentifierFollow» = «IdentifierStart» | '0'…'9' ;
        let identifier_follow_parser = filter(|&c: &char| {
            c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || ('0' <= c && c <= '9')
        })
        .map(|_| FixedTerminal::<1>())
        .boxed();

        // «StringChar» = ¬( '\'' | '\\' ) | '\\' ( '\'' | '\\' | 'u{' 1…6*{ «HexDigit» } '}' ) ;
        let string_char_parser = choice((
            filter(|&c: &char| c != '\'' && c != '\\')
                .map(|_| FixedTerminal::<1>())
                .map(|v| Box::new(string_char::_C0::NotQuoteOrBackslash(v))),
            just('\\')
                .map(|_| FixedTerminal::<1>())
                .then(choice((
                    choice::<_, ErrorType>((terminal("'").ignored(), terminal("\\").ignored()))
                        .map(|_| FixedTerminal::<1usize>())
                        .map(|v| Box::new(string_char::QuoteOrBackslashOrHexEscape::_0(v))),
                    terminal("u{")
                        .ignored()
                        .map(|_| FixedTerminal::<2usize>())
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')
                            })
                            .map(|_| FixedTerminal::<1>())
                            .repeated()
                            .at_least(1usize)
                            .at_most(6usize)
                            .map(|v| v.len()),
                        )
                        .then(just('}').map(|_| FixedTerminal::<1>()))
                        .map(|v| Box::new(string_char::_S1::new(v)))
                        .map(|v| Box::new(string_char::QuoteOrBackslashOrHexEscape::_S1(v))),
                )))
                .map(|v| Box::new(string_char::Escape::new(v)))
                .map(|v| Box::new(string_char::_C0::Escape(v))),
        ))
        .boxed();

        // repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
        let repetition_prefix_parser = choice((
            number_parser
                .clone()
                .then(ignore_parser.clone())
                .then(
                    just('…')
                        .map(|_| FixedTerminal::<1>())
                        .then(ignore_parser.clone())
                        .then(number_parser.clone().or_not())
                        .map(|v| Box::new(repetition_prefix::_S4::new(v)))
                        .or_not(),
                )
                .map(|v| Box::new(repetition_prefix::_S2::new(v)))
                .map(|v| Box::new(repetition_prefix::_C1::_S2(v))),
            just('…')
                .map(|_| FixedTerminal::<1>())
                .then(ignore_parser.clone())
                .then(number_parser.clone())
                .map(|v| Box::new(repetition_prefix::_S6::new(v)))
                .map(|v| Box::new(repetition_prefix::_C1::_S6(v))),
        ))
        .then(ignore_parser.clone())
        .then(just('*').map(|_| FixedTerminal::<1>()))
        .map(|v| Box::new(repetition_prefix::_S0::new(v)))
        .boxed();

        // «RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;
        let raw_identifier_parser =
            filter(|&c: &char| c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z'))
                .map(|_| FixedTerminal::<1>())
                .then(
                    filter(|&c: &char| {
                        c == '_'
                            || ('a' <= c && c <= 'z')
                            || ('A' <= c && c <= 'Z')
                            || ('0' <= c && c <= '9')
                    })
                    .map(|_| FixedTerminal::<1>())
                    .repeated()
                    .map(|v| v.len()),
                )
                .map(|v| Box::new(raw_identifier::_S0::new(v)))
                .boxed();

        // «SingleCharString» = '\'' «StringChar» '\'' ;
        let single_char_string_parser = just('\'')
            .map(|_| FixedTerminal::<1>())
            .then(string_char_parser.clone())
            .then(just('\'').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(single_char_string::_S0::new(v)))
            .boxed();

        // «String» = '\'' { «StringChar» } '\'' ;
        let string_parser = just('\'')
            .map(|_| FixedTerminal::<1>())
            .then(string_char_parser.clone().repeated())
            .then(just('\'').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(string::_S0::new(v)))
            .boxed();

        // repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
        let repeated_parser = repetition_prefix_parser
            .clone()
            .or_not()
            .then(ignore_parser.clone())
            .then(just('{').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(repetition_separator_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(repeated::_S0::new(v)))
            .boxed();

        // «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
        let identifier_parser = choice((
            just('«')
                .map(|_| FixedTerminal::<1>())
                .then(raw_identifier_parser.clone())
                .then(just('»').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(identifier::_S1::new(v)))
                .map(|v| Box::new(identifier::_C0::_S1(v))),
            raw_identifier_parser
                .clone()
                .map(|v| Box::new(identifier::_C0::RawIdentifier(v))),
        ))
        .boxed();

        // charRange = «SingleCharString» '…' «SingleCharString» ;
        let char_range_parser = single_char_string_parser
            .clone()
            .then(ignore_parser.clone())
            .then(just('…').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(single_char_string_parser.clone())
            .map(|v| Box::new(char_range::_S0::new(v)))
            .boxed();

        // productionReference = «Identifier» ;
        let production_reference_parser = identifier_parser.clone().boxed();

        // primary = productionReference | grouped | optional | repeated | charRange | «EOF» | «String» ;
        let primary_parser = choice((
            production_reference_parser
                .clone()
                .map(|v| Box::new(primary::_C0::ProductionReference(v))),
            grouped_parser
                .clone()
                .map(|v| Box::new(primary::_C0::Grouped(v))),
            optional_parser
                .clone()
                .map(|v| Box::new(primary::_C0::Optional(v))),
            repeated_parser
                .clone()
                .map(|v| Box::new(primary::_C0::Repeated(v))),
            char_range_parser
                .clone()
                .map(|v| Box::new(primary::_C0::CharRange(v))),
            terminal("$")
                .ignored()
                .map(|_| FixedTerminal::<1usize>())
                .map(|v| Box::new(primary::_C0::Dollar(v))),
            string_parser
                .clone()
                .map(|v| Box::new(primary::_C0::String(v))),
        ))
        .boxed();

        // negation = [ '¬' ] primary ;
        let negation_parser = just('¬')
            .map(|_| FixedTerminal::<1>())
            .or_not()
            .then(ignore_parser.clone())
            .then(primary_parser.clone())
            .map(|v| Box::new(negation::_S0::new(v)))
            .boxed();

        // difference = negation [ '-' negation ] ;
        let difference_parser = negation_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('-')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(negation_parser.clone())
                    .map(|v| Box::new(difference::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(difference::_S0::new(v)))
            .boxed();

        // sequence = 1…*{ difference } ;
        let sequence_parser = difference_parser
            .clone()
            .then(ignore_parser.clone())
            .map(|v| Box::new(sequence::_S1::new(v)))
            .repeated()
            .at_least(1usize)
            .boxed();

        // expression = 1…*{ sequence / '|' } ;
        expression_parser.define(
            sequence_parser
                .clone()
                .then(ignore_parser.clone())
                .map(|v| Box::new(expression::_S1::new(v)))
                .then(
                    just('|')
                        .map(|_| FixedTerminal::<1>())
                        .then(ignore_parser.clone())
                        .map(|v| Box::new(expression::_S2::new(v)))
                        .then(
                            sequence_parser
                                .clone()
                                .then(ignore_parser.clone())
                                .map(|v| Box::new(expression::_S1::new(v))),
                        )
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::_S0::new(v)))
                .boxed(),
        );

        // production = «Identifier» '=' expression ';' ;
        let production_parser = identifier_parser
            .clone()
            .then(ignore_parser.clone())
            .then(just('=').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(production::_S0::new(v)))
            .boxed();

        // grammar = «IGNORE» { production } $ ;
        let grammar_parser = ignore_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                production_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(grammar::_S2::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(end())
            .map(|v| Box::new(grammar::_S0::new(v)))
            .boxed();

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
