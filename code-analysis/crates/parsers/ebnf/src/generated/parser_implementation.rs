use super::parser_interface::*;
use super::tree_interface::*;
use chumsky::prelude::*;
use chumsky::primitive::Just;
use chumsky::Parser;
#[allow(dead_code)]
fn repetition_mapper<E, S>((e, es): (E, Vec<(S, E)>)) -> (Vec<E>, Vec<S>) {
    let mut elements = vec![e];
    let mut separators = vec![];
    for (s, e) in es.into_iter() {
        separators.push(s);
        elements.push(e);
    }
    (elements, separators)
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
        let mut expression_parser = Recursive::declare();

        // «Comment» = '(*' { ¬'*' | 1…*{ '*' } ¬( '*' | ')' ) } { '*' } '*)' ;
        let comment_parser = terminal("(*")
            .ignored()
            .map(|_| FixedSizeTerminal::<2usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '*')
                        .map(|_| FixedSizeTerminal::<1>())
                        .map(|v| Box::new(comment::_T2::NotStarChar(v))),
                    just('*')
                        .map(|_| FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .then(
                            filter(|&c: &char| c != '*' && c != ')')
                                .map(|_| FixedSizeTerminal::<1>()),
                        )
                        .map(|v| comment::_T3::from_parse(v))
                        .map(|v| Box::new(comment::_T2::_T3(v))),
                ))
                .repeated()
                .then(
                    just('*')
                        .map(|_| FixedSizeTerminal::<1>())
                        .repeated()
                        .map(|v| VariableSizeTerminal(v.len())),
                )
                .map(|v| comment::Content::from_parse(v)),
            )
            .then(
                terminal("*)")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<2usize>()),
            )
            .map(|v| comment::_T0::from_parse(v))
            .boxed();

        // «Number» = 1…*{ '0'…'9' } ;
        let number_parser = filter(|&c: &char| ('0' <= c && c <= '9'))
            .map(|_| FixedSizeTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| VariableSizeTerminal(v.len()))
            .boxed();

        // «RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;
        let raw_identifier_parser =
            filter(|&c: &char| c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z'))
                .map(|_| FixedSizeTerminal::<1>())
                .then(
                    filter(|&c: &char| {
                        c == '_'
                            || ('a' <= c && c <= 'z')
                            || ('A' <= c && c <= 'Z')
                            || ('0' <= c && c <= '9')
                    })
                    .map(|_| FixedSizeTerminal::<1>())
                    .repeated()
                    .map(|v| VariableSizeTerminal(v.len())),
                )
                .map(|v| raw_identifier::_T0::from_parse(v))
                .boxed();

        // «StringChar» = ¬( '\'' | '\\' ) | '\\' ( '\'' | '\\' | 'u{' 1…6*{ «HexDigit» } '}' ) ;
        let string_char_parser = choice((
            filter(|&c: &char| c != '\'' && c != '\\')
                .map(|_| FixedSizeTerminal::<1>())
                .map(|v| Box::new(string_char::_T0::NotQuoteOrBackslash(v))),
            just('\\')
                .map(|_| FixedSizeTerminal::<1>())
                .then(choice((
                    filter(|&c: &char| c == '\'' || c == '\\')
                        .map(|_| FixedSizeTerminal::<1>())
                        .map(|v| Box::new(string_char::QuoteOrBackslashOrHexEscape::_0(v))),
                    terminal("u{")
                        .ignored()
                        .map(|_| FixedSizeTerminal::<2usize>())
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')
                            })
                            .map(|_| FixedSizeTerminal::<1>())
                            .repeated()
                            .at_least(1usize)
                            .at_most(6usize)
                            .map(|v| VariableSizeTerminal(v.len())),
                        )
                        .then(just('}').map(|_| FixedSizeTerminal::<1>()))
                        .map(|v| string_char::_T1::from_parse(v))
                        .map(|v| Box::new(string_char::QuoteOrBackslashOrHexEscape::_T1(v))),
                )))
                .map(|v| string_char::Escape::from_parse(v))
                .map(|v| Box::new(string_char::_T0::Escape(v))),
        ))
        .boxed();

        // «Whitespace» = 1…*{ '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}' } ;
        let whitespace_parser = filter(|&c: &char| c == '\t' || c == '\n' || c == '\r' || c == ' ')
            .map(|_| FixedSizeTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| VariableSizeTerminal(v.len()))
            .boxed();

        // «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
        let identifier_parser = choice((
            just('«')
                .map(|_| FixedSizeTerminal::<1>())
                .then(raw_identifier_parser.clone())
                .then(just('»').map(|_| FixedSizeTerminal::<1>()))
                .map(|v| identifier::_T1::from_parse(v))
                .map(|v| Box::new(identifier::_T0::_T1(v))),
            raw_identifier_parser
                .clone()
                .map(|v| Box::new(identifier::_T0::RawIdentifier(v))),
        ))
        .boxed();

        // «LeadingTrivia» = { «Whitespace» | «Comment» } ;
        let leading_trivia_parser = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(leading_trivia::_T1::Whitespace(v))),
            comment_parser
                .clone()
                .map(|v| Box::new(leading_trivia::_T1::Comment(v))),
        ))
        .repeated()
        .boxed();

        // «SingleCharString» = '\'' «StringChar» '\'' ;
        let single_char_string_parser = just('\'')
            .map(|_| FixedSizeTerminal::<1>())
            .then(string_char_parser.clone())
            .then(just('\'').map(|_| FixedSizeTerminal::<1>()))
            .map(|v| single_char_string::_T0::from_parse(v))
            .boxed();

        // «String» = '\'' { «StringChar» } '\'' ;
        let string_parser = just('\'')
            .map(|_| FixedSizeTerminal::<1>())
            .then(string_char_parser.clone().repeated())
            .then(just('\'').map(|_| FixedSizeTerminal::<1>()))
            .map(|v| string::_T0::from_parse(v))
            .boxed();

        // «TrailingTrivia» = { «Whitespace» | «Comment» } ;
        let trailing_trivia_parser = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::_T1::Whitespace(v))),
            comment_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::_T1::Comment(v))),
        ))
        .repeated()
        .boxed();

        // charRange = «SingleCharString» '…' «SingleCharString» ;
        let char_range_parser = leading_trivia_parser
            .clone()
            .then(single_char_string_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| single_char_string::WithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('…').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(single_char_string_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| single_char_string::WithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .map(|v| char_range::_T0::from_parse(v))
            .boxed();

        // grouped = '(' expression ')' ;
        let grouped_parser = leading_trivia_parser
            .clone()
            .then(just('(').map(|_| FixedSizeTerminal::<1>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(expression_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(')').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .map(|v| grouped::_T0::from_parse(v))
            .boxed();

        // optional = '[' expression ']' ;
        let optional_parser = leading_trivia_parser
            .clone()
            .then(just('[').map(|_| FixedSizeTerminal::<1>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(expression_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(']').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .map(|v| optional::_T0::from_parse(v))
            .boxed();

        // productionReference = «Identifier» ;
        let production_reference_parser = leading_trivia_parser
            .clone()
            .then(identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(|((leading, content), trailing)| identifier::WithTrivia {
                leading,
                content,
                trailing,
            })
            .boxed();

        // repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
        let repetition_prefix_parser = choice((
            leading_trivia_parser
                .clone()
                .then(number_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading, content), trailing)| number::WithTrivia {
                    leading,
                    content,
                    trailing,
                })
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('…').map(|_| FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(
                            |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                leading,
                                content,
                                trailing,
                            },
                        )
                        .then(
                            leading_trivia_parser
                                .clone()
                                .then(number_parser.clone())
                                .then(trailing_trivia_parser.clone())
                                .map(|((leading, content), trailing)| number::WithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                })
                                .or_not(),
                        )
                        .map(|v| repetition_prefix::_T3::from_parse(v))
                        .or_not(),
                )
                .map(|v| repetition_prefix::_T2::from_parse(v))
                .map(|v| Box::new(repetition_prefix::_T1::_T2(v))),
            leading_trivia_parser
                .clone()
                .then(just('…').map(|_| FixedSizeTerminal::<1>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(number_parser.clone())
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading, content), trailing)| number::WithTrivia {
                            leading,
                            content,
                            trailing,
                        }),
                )
                .map(|v| repetition_prefix::_T4::from_parse(v))
                .map(|v| Box::new(repetition_prefix::_T1::_T4(v))),
        ))
        .then(
            leading_trivia_parser
                .clone()
                .then(just('*').map(|_| FixedSizeTerminal::<1>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                ),
        )
        .map(|v| repetition_prefix::_T0::from_parse(v))
        .boxed();

        // repetitionSeparator = '/' expression ;
        let repetition_separator_parser = leading_trivia_parser
            .clone()
            .then(just('/').map(|_| FixedSizeTerminal::<1>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(expression_parser.clone())
            .map(|v| repetition_separator::_T0::from_parse(v))
            .boxed();

        // repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
        let repeated_parser = repetition_prefix_parser
            .clone()
            .or_not()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('{').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(expression_parser.clone())
            .then(repetition_separator_parser.clone().or_not())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('}').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .map(|v| repeated::_T0::from_parse(v))
            .boxed();

        // primary = productionReference | grouped | optional | repeated | charRange | «EOF» | «String» ;
        let primary_parser = choice((
            production_reference_parser
                .clone()
                .map(|v| Box::new(primary::_T0::ProductionReference(v))),
            grouped_parser
                .clone()
                .map(|v| Box::new(primary::_T0::Grouped(v))),
            optional_parser
                .clone()
                .map(|v| Box::new(primary::_T0::Optional(v))),
            repeated_parser
                .clone()
                .map(|v| Box::new(primary::_T0::Repeated(v))),
            char_range_parser
                .clone()
                .map(|v| Box::new(primary::_T0::CharRange(v))),
            leading_trivia_parser
                .clone()
                .then(just('$').map(|_| FixedSizeTerminal::<1>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(primary::_T0::DollarChar(v))),
            leading_trivia_parser
                .clone()
                .then(string_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading, content), trailing)| string::WithTrivia {
                    leading,
                    content,
                    trailing,
                })
                .map(|v| Box::new(primary::_T0::String(v))),
        ))
        .boxed();

        // negation = [ '¬' ] primary ;
        let negation_parser = leading_trivia_parser
            .clone()
            .then(just('¬').map(|_| FixedSizeTerminal::<1>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .or_not()
            .then(primary_parser.clone())
            .map(|v| negation::_T0::from_parse(v))
            .boxed();

        // difference = negation [ '-' negation ] ;
        let difference_parser = negation_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('-').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(negation_parser.clone())
                    .map(|v| difference::_T1::from_parse(v))
                    .or_not(),
            )
            .map(|v| difference::_T0::from_parse(v))
            .boxed();

        // sequence = 1…*{ difference } ;
        let sequence_parser = difference_parser
            .clone()
            .repeated()
            .at_least(1usize)
            .boxed();

        // expression = sequence  { '|' sequence } ;
        expression_parser.define(
            sequence_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('|').map(|_| FixedSizeTerminal::<1usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(
                            |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                leading,
                                content,
                                trailing,
                            },
                        )
                        .then(sequence_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|(elements, separators)| expression::_T0 {
                    elements,
                    separators,
                })
                .boxed(),
        );

        // production = «Identifier» '=' expression ';' ;
        let production_parser = leading_trivia_parser
            .clone()
            .then(identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(|((leading, content), trailing)| identifier::WithTrivia {
                leading,
                content,
                trailing,
            })
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('=').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(expression_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .map(|v| production::_T0::from_parse(v))
            .boxed();

        // grammar = «LeadingTrivia» { production } «TrailingTrivia» $ ;
        let grammar_parser = leading_trivia_parser
            .clone()
            .then(leading_trivia_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| leading_trivia::WithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(production_parser.clone().repeated())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(trailing_trivia_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| trailing_trivia::WithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(end())
            .map(|v| grammar::_T0::from_parse(v))
            .boxed();

        Self {
            comment: comment_parser,
            number: number_parser,
            raw_identifier: raw_identifier_parser,
            string_char: string_char_parser,
            whitespace: whitespace_parser,
            identifier: identifier_parser,
            leading_trivia: leading_trivia_parser,
            single_char_string: single_char_string_parser,
            string: string_parser,
            trailing_trivia: trailing_trivia_parser,
            char_range: char_range_parser,
            grouped: grouped_parser,
            optional: optional_parser,
            production_reference: production_reference_parser,
            repetition_prefix: repetition_prefix_parser,
            repetition_separator: repetition_separator_parser,
            repeated: repeated_parser,
            primary: primary_parser,
            negation: negation_parser,
            difference: difference_parser,
            sequence: sequence_parser,
            expression: expression_parser.boxed(),
            production: production_parser,
            grammar: grammar_parser,
        }
    }
}
