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
        let mut yul_expression_parser = Recursive::declare();

        let mut type_name_parser = Recursive::declare();

        let mut yul_block_parser = Recursive::declare();

        let mut expression_parser = Recursive::declare();

        let mut statement_parser = Recursive::declare();

        let mut block_parser = Recursive::declare();

        // «DecimalInteger» = 1…*{ '0'…'9' / [ '_' ] } ;
        let decimal_integer_parser = filter(|&c: &char| ('0' <= c && c <= '9'))
            .map(|_| FixedSizeTerminal::<1>())
            .then(
                just('_')
                    .map(|_| FixedSizeTerminal::<1>())
                    .or_not()
                    .then(
                        filter(|&c: &char| ('0' <= c && c <= '9'))
                            .map(|_| FixedSizeTerminal::<1>()),
                    )
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|(elements, separators)| decimal_integer::_T0 {
                elements,
                separators,
            })
            .boxed();

        // «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
        let end_of_line_parser = filter(|&c: &char| c == '\r' || c == '\n')
            .map(|_| FixedSizeTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| VariableSizeTerminal(v.len()))
            .boxed();

        // «FixedBytesType» = 'bytes' [ '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ] ;
        let fixed_bytes_type_parser = terminal("bytes")
            .ignored()
            .map(|_| FixedSizeTerminal::<5usize>())
            .then(
                choice::<_, ErrorType>((
                    terminal("1").ignore_then(choice((
                        terminal("0").map(|_| 2usize),
                        terminal("1").map(|_| 2usize),
                        terminal("2").map(|_| 2usize),
                        terminal("3").map(|_| 2usize),
                        terminal("4").map(|_| 2usize),
                        terminal("5").map(|_| 2usize),
                        terminal("6").map(|_| 2usize),
                        terminal("7").map(|_| 2usize),
                        terminal("8").map(|_| 2usize),
                        terminal("9").map(|_| 2usize),
                        empty().map(|_| 1usize),
                    ))),
                    terminal("2").ignore_then(choice((
                        terminal("0").map(|_| 2usize),
                        terminal("1").map(|_| 2usize),
                        terminal("2").map(|_| 2usize),
                        terminal("3").map(|_| 2usize),
                        terminal("4").map(|_| 2usize),
                        terminal("5").map(|_| 2usize),
                        terminal("6").map(|_| 2usize),
                        terminal("7").map(|_| 2usize),
                        terminal("8").map(|_| 2usize),
                        terminal("9").map(|_| 2usize),
                        empty().map(|_| 1usize),
                    ))),
                    terminal("3").ignore_then(choice((
                        terminal("0").map(|_| 2usize),
                        terminal("1").map(|_| 2usize),
                        terminal("2").map(|_| 2usize),
                        empty().map(|_| 1usize),
                    ))),
                    terminal("4").map(|_| 1usize),
                    terminal("5").map(|_| 1usize),
                    terminal("6").map(|_| 1usize),
                    terminal("7").map(|_| 1usize),
                    terminal("8").map(|_| 1usize),
                    terminal("9").map(|_| 1usize),
                ))
                .map(VariableSizeTerminal)
                .or_not(),
            )
            .map(|v| fixed_bytes_type::_T0::from_parse(v))
            .boxed();

        // «FixedType» = 'fixed' [ '1'…'9' { '0'…'9' } 'x' '1'…'9' { '0'…'9' } ] ;
        let fixed_type_parser = terminal("fixed")
            .ignored()
            .map(|_| FixedSizeTerminal::<5usize>())
            .then(
                filter(|&c: &char| ('1' <= c && c <= '9'))
                    .map(|_| FixedSizeTerminal::<1>())
                    .then(
                        filter(|&c: &char| ('0' <= c && c <= '9'))
                            .map(|_| FixedSizeTerminal::<1>())
                            .repeated()
                            .map(|v| VariableSizeTerminal(v.len())),
                    )
                    .then(just('x').map(|_| FixedSizeTerminal::<1>()))
                    .then(
                        filter(|&c: &char| ('1' <= c && c <= '9'))
                            .map(|_| FixedSizeTerminal::<1>()),
                    )
                    .then(
                        filter(|&c: &char| ('0' <= c && c <= '9'))
                            .map(|_| FixedSizeTerminal::<1>())
                            .repeated()
                            .map(|v| VariableSizeTerminal(v.len())),
                    )
                    .map(|v| fixed_type::_T1::from_parse(v))
                    .or_not(),
            )
            .map(|v| fixed_type::_T0::from_parse(v))
            .boxed();

        // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
        let hex_byte_escape_parser = just('x')
            .map(|_| FixedSizeTerminal::<1>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedSizeTerminal::<1>())
                .repeated()
                .exactly(2usize)
                .map(|v| VariableSizeTerminal(v.len())),
            )
            .map(|v| hex_byte_escape::_T0::from_parse(v))
            .boxed();

        // «HexNumber» = '0x' 1…*{ «HexCharacter» / [ '_' ] } ;
        let hex_number_parser = terminal("0x")
            .ignored()
            .map(|_| FixedSizeTerminal::<2usize>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedSizeTerminal::<1>())
                .then(
                    just('_')
                        .map(|_| FixedSizeTerminal::<1>())
                        .or_not()
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')
                            })
                            .map(|_| FixedSizeTerminal::<1>()),
                        )
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|(elements, separators)| hex_number::_T1 {
                    elements,
                    separators,
                }),
            )
            .map(|v| hex_number::_T0::from_parse(v))
            .boxed();

        // «MultilineComment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
        let multiline_comment_parser = terminal("/*")
            .ignored()
            .map(|_| FixedSizeTerminal::<2usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '*')
                        .map(|_| FixedSizeTerminal::<1>())
                        .map(|v| Box::new(multiline_comment::_T2::NotStarChar(v))),
                    just('*')
                        .map(|_| FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .then(
                            filter(|&c: &char| c != '*' && c != '/')
                                .map(|_| FixedSizeTerminal::<1>()),
                        )
                        .map(|v| multiline_comment::_T3::from_parse(v))
                        .map(|v| Box::new(multiline_comment::_T2::_T3(v))),
                ))
                .repeated()
                .then(
                    just('*')
                        .map(|_| FixedSizeTerminal::<1>())
                        .repeated()
                        .map(|v| VariableSizeTerminal(v.len())),
                )
                .map(|v| multiline_comment::Content::from_parse(v)),
            )
            .then(
                terminal("*/")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<2usize>()),
            )
            .map(|v| multiline_comment::_T0::from_parse(v))
            .boxed();

        // «PossiblySeparatedPairsOfHexDigits» = 1…*{ 2…2*{ «HexCharacter» } / [ '_' ] } ;
        let possibly_separated_pairs_of_hex_digits_parser = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        })
        .map(|_| FixedSizeTerminal::<1>())
        .repeated()
        .exactly(2usize)
        .map(|v| VariableSizeTerminal(v.len()))
        .then(
            just('_')
                .map(|_| FixedSizeTerminal::<1>())
                .or_not()
                .then(
                    filter(|&c: &char| {
                        ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                    })
                    .map(|_| FixedSizeTerminal::<1>())
                    .repeated()
                    .exactly(2usize)
                    .map(|v| VariableSizeTerminal(v.len())),
                )
                .repeated(),
        )
        .map(repetition_mapper)
        .map(
            |(elements, separators)| possibly_separated_pairs_of_hex_digits::_T0 {
                elements,
                separators,
            },
        )
        .boxed();

        // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
        let raw_identifier_parser = filter(|&c: &char| {
            c == '_' || c == '$' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
        })
        .map(|_| FixedSizeTerminal::<1>())
        .then(
            filter(|&c: &char| {
                c == '_'
                    || c == '$'
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

        // «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
        let signed_integer_type_parser = terminal("int")
            .ignored()
            .map(|_| FixedSizeTerminal::<3usize>())
            .then(
                choice::<_, ErrorType>((
                    terminal("1").ignore_then(choice((
                        terminal("04").map(|_| 3usize),
                        terminal("12").map(|_| 3usize),
                        terminal("2").ignore_then(choice((
                            terminal("0").map(|_| 3usize),
                            terminal("8").map(|_| 3usize),
                        ))),
                        terminal("36").map(|_| 3usize),
                        terminal("44").map(|_| 3usize),
                        terminal("52").map(|_| 3usize),
                        terminal("6").ignore_then(choice((
                            terminal("0").map(|_| 3usize),
                            terminal("8").map(|_| 3usize),
                            empty().map(|_| 2usize),
                        ))),
                        terminal("76").map(|_| 3usize),
                        terminal("84").map(|_| 3usize),
                        terminal("92").map(|_| 3usize),
                    ))),
                    terminal("2").ignore_then(choice((
                        terminal("0").ignore_then(choice((
                            terminal("0").map(|_| 3usize),
                            terminal("8").map(|_| 3usize),
                        ))),
                        terminal("16").map(|_| 3usize),
                        terminal("24").map(|_| 3usize),
                        terminal("32").map(|_| 3usize),
                        terminal("4").ignore_then(choice((
                            terminal("0").map(|_| 3usize),
                            terminal("8").map(|_| 3usize),
                            empty().map(|_| 2usize),
                        ))),
                        terminal("56").map(|_| 3usize),
                    ))),
                    terminal("32").map(|_| 2usize),
                    terminal("4").ignore_then(choice((
                        terminal("0").map(|_| 2usize),
                        terminal("8").map(|_| 2usize),
                    ))),
                    terminal("56").map(|_| 2usize),
                    terminal("64").map(|_| 2usize),
                    terminal("72").map(|_| 2usize),
                    terminal("8").ignore_then(choice((
                        terminal("0").map(|_| 2usize),
                        terminal("8").map(|_| 2usize),
                        empty().map(|_| 1usize),
                    ))),
                    terminal("96").map(|_| 2usize),
                ))
                .map(VariableSizeTerminal)
                .or_not(),
            )
            .map(|v| signed_integer_type::_T0::from_parse(v))
            .boxed();

        // «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
        let single_line_comment_parser = terminal("//")
            .ignored()
            .map(|_| FixedSizeTerminal::<2usize>())
            .then(
                filter(|&c: &char| c != '\r' && c != '\n')
                    .map(|_| FixedSizeTerminal::<1>())
                    .repeated()
                    .map(|v| VariableSizeTerminal(v.len())),
            )
            .map(|v| single_line_comment::_T0::from_parse(v))
            .boxed();

        // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
        let unicode_escape_parser = just('u')
            .map(|_| FixedSizeTerminal::<1>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedSizeTerminal::<1>())
                .repeated()
                .exactly(4usize)
                .map(|v| VariableSizeTerminal(v.len())),
            )
            .map(|v| unicode_escape::_T0::from_parse(v))
            .boxed();

        // «VersionPragmaValue» = 1…*{ 1…*{ '0'…'9' | 'x' | 'X' | '*' } / '.' } ;
        let version_pragma_value_parser =
            filter(|&c: &char| ('0' <= c && c <= '9') || c == 'x' || c == 'X' || c == '*')
                .map(|_| FixedSizeTerminal::<1>())
                .repeated()
                .at_least(1usize)
                .map(|v| VariableSizeTerminal(v.len()))
                .then(
                    just('.')
                        .map(|_| FixedSizeTerminal::<1>())
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9') || c == 'x' || c == 'X' || c == '*'
                            })
                            .map(|_| FixedSizeTerminal::<1>())
                            .repeated()
                            .at_least(1usize)
                            .map(|v| VariableSizeTerminal(v.len())),
                        )
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|(elements, separators)| version_pragma_value::_T0 {
                    elements,
                    separators,
                })
                .boxed();

        // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
        let whitespace_parser = filter(|&c: &char| c == ' ' || c == '\t')
            .map(|_| FixedSizeTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| VariableSizeTerminal(v.len()))
            .boxed();

        // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
        let yul_decimal_number_literal_parser = choice((
            just('0')
                .map(|_| FixedSizeTerminal::<1>())
                .map(|v| Box::new(yul_decimal_number_literal::_T0::ZeroChar(v))),
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .map(|_| FixedSizeTerminal::<1>())
                .then(
                    filter(|&c: &char| ('0' <= c && c <= '9'))
                        .map(|_| FixedSizeTerminal::<1>())
                        .repeated()
                        .map(|v| VariableSizeTerminal(v.len())),
                )
                .map(|v| yul_decimal_number_literal::_T1::from_parse(v))
                .map(|v| Box::new(yul_decimal_number_literal::_T0::_T1(v))),
        ))
        .boxed();

        // «YulHexLiteral» = '0x' 1…*{ '0'…'9' | 'a'…'f' | 'A'…'F' } ;
        let yul_hex_literal_parser = terminal("0x")
            .ignored()
            .map(|_| FixedSizeTerminal::<2usize>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedSizeTerminal::<1>())
                .repeated()
                .at_least(1usize)
                .map(|v| VariableSizeTerminal(v.len())),
            )
            .map(|v| yul_hex_literal::_T0::from_parse(v))
            .boxed();

        // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
        let decimal_exponent_parser = filter(|&c: &char| c == 'e' || c == 'E')
            .map(|_| FixedSizeTerminal::<1>())
            .then(just('-').map(|_| FixedSizeTerminal::<1>()).or_not())
            .then(decimal_integer_parser.clone())
            .map(|v| decimal_exponent::_T0::from_parse(v))
            .boxed();

        // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
        let decimal_float_parser = decimal_integer_parser
            .clone()
            .or_not()
            .then(just('.').map(|_| FixedSizeTerminal::<1>()))
            .then(decimal_integer_parser.clone())
            .map(|v| decimal_float::_T0::from_parse(v))
            .boxed();

        // «EndOfFileTrivia» = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
        let end_of_file_trivia_parser = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(end_of_file_trivia::_T1::Whitespace(v))),
            multiline_comment_parser
                .clone()
                .map(|v| Box::new(end_of_file_trivia::_T1::MultilineComment(v))),
            single_line_comment_parser
                .clone()
                .map(|v| Box::new(end_of_file_trivia::_T1::SingleLineComment(v))),
        ))
        .repeated()
        .boxed();

        // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
        let escape_sequence_parser = just('\\')
            .map(|_| FixedSizeTerminal::<1>())
            .then(choice((
                filter(|&c: &char| {
                    c == 'n'
                        || c == 'r'
                        || c == 't'
                        || c == '\''
                        || c == '"'
                        || c == '\\'
                        || c == '\n'
                        || c == '\r'
                })
                .map(|_| FixedSizeTerminal::<1>())
                .map(|v| Box::new(escape_sequence::_T1::_0(v))),
                hex_byte_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::_T1::HexByteEscape(v))),
                unicode_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::_T1::UnicodeEscape(v))),
            )))
            .map(|v| escape_sequence::_T0::from_parse(v))
            .boxed();

        // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
        let hex_string_literal_parser = terminal("hex")
            .ignored()
            .map(|_| FixedSizeTerminal::<3usize>())
            .then(choice((
                just('"')
                    .map(|_| FixedSizeTerminal::<1>())
                    .then(
                        possibly_separated_pairs_of_hex_digits_parser
                            .clone()
                            .or_not(),
                    )
                    .then(just('"').map(|_| FixedSizeTerminal::<1>()))
                    .map(|v| hex_string_literal::_T2::from_parse(v))
                    .map(|v| Box::new(hex_string_literal::_T1::_T2(v))),
                just('\'')
                    .map(|_| FixedSizeTerminal::<1>())
                    .then(
                        possibly_separated_pairs_of_hex_digits_parser
                            .clone()
                            .or_not(),
                    )
                    .then(just('\'').map(|_| FixedSizeTerminal::<1>()))
                    .map(|v| hex_string_literal::_T3::from_parse(v))
                    .map(|v| Box::new(hex_string_literal::_T1::_T3(v))),
            )))
            .map(|v| hex_string_literal::_T0::from_parse(v))
            .boxed();

        // «LeadingTrivia» = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
        let leading_trivia_parser = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(leading_trivia::_T1::Whitespace(v))),
            end_of_line_parser
                .clone()
                .map(|v| Box::new(leading_trivia::_T1::EndOfLine(v))),
            multiline_comment_parser
                .clone()
                .map(|v| Box::new(leading_trivia::_T1::MultilineComment(v))),
            single_line_comment_parser
                .clone()
                .map(|v| Box::new(leading_trivia::_T1::SingleLineComment(v))),
        ))
        .repeated()
        .boxed();

        // «TrailingTrivia» = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
        let trailing_trivia_parser = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::_T2::Whitespace(v))),
            multiline_comment_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::_T2::MultilineComment(v))),
        ))
        .repeated()
        .then(choice((
            end_of_line_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::_T3::EndOfLine(v))),
            single_line_comment_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::_T3::SingleLineComment(v))),
        )))
        .map(|v| trailing_trivia::_T0::from_parse(v))
        .or_not()
        .boxed();

        // «UfixedType» = 'u' «FixedType» ;
        let ufixed_type_parser = just('u')
            .map(|_| FixedSizeTerminal::<1>())
            .then(fixed_type_parser.clone())
            .map(|v| ufixed_type::_T0::from_parse(v))
            .boxed();

        // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
        let unsigned_integer_type_parser = just('u')
            .map(|_| FixedSizeTerminal::<1>())
            .then(signed_integer_type_parser.clone())
            .map(|v| unsigned_integer_type::_T0::from_parse(v))
            .boxed();

        // «YulIdentifier» = «RawIdentifier» - «YulReservedWord» ;
        let yul_identifier_parser = difference(
            raw_identifier_parser.clone(),
            choice::<_, ErrorType>((
                terminal("Blockhash").map(|_| 9usize),
                terminal("a").ignore_then(choice((
                    terminal("dd").ignore_then(choice((
                        terminal("mod").map(|_| 6usize),
                        terminal("ress").map(|_| 7usize),
                        empty().map(|_| 3usize),
                    ))),
                    terminal("nd").map(|_| 3usize),
                ))),
                terminal("b").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("lance").map(|_| 7usize),
                        terminal("sefee").map(|_| 7usize),
                    ))),
                    terminal("reak").map(|_| 5usize),
                    terminal("yte").map(|_| 4usize),
                ))),
                terminal("c").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("ll").ignore_then(choice((
                            terminal("code").map(|_| 8usize),
                            terminal("data").ignore_then(choice((
                                terminal("copy").map(|_| 12usize),
                                terminal("load").map(|_| 12usize),
                                terminal("size").map(|_| 12usize),
                            ))),
                            terminal("er").map(|_| 6usize),
                            terminal("value").map(|_| 9usize),
                            empty().map(|_| 4usize),
                        ))),
                        terminal("se").map(|_| 4usize),
                    ))),
                    terminal("hainid").map(|_| 7usize),
                    terminal("o").ignore_then(choice((
                        terminal("inbase").map(|_| 8usize),
                        terminal("ntinue").map(|_| 8usize),
                    ))),
                    terminal("reate").ignore_then(choice((
                        terminal("2").map(|_| 7usize),
                        empty().map(|_| 6usize),
                    ))),
                ))),
                terminal("d").ignore_then(choice((
                    terminal("e").ignore_then(choice((
                        terminal("fault").map(|_| 7usize),
                        terminal("legatecall").map(|_| 12usize),
                    ))),
                    terminal("i").ignore_then(choice((
                        terminal("fficulty").map(|_| 10usize),
                        terminal("v").map(|_| 3usize),
                    ))),
                ))),
                terminal("e").ignore_then(choice((
                    terminal("q").map(|_| 2usize),
                    terminal("x").ignore_then(choice((
                        terminal("p").map(|_| 3usize),
                        terminal("tcode").ignore_then(choice((
                            terminal("copy").map(|_| 11usize),
                            terminal("hash").map(|_| 11usize),
                            terminal("size").map(|_| 11usize),
                        ))),
                    ))),
                ))),
                terminal("f").ignore_then(choice((
                    terminal("alse").map(|_| 5usize),
                    terminal("or").map(|_| 3usize),
                    terminal("unction").map(|_| 8usize),
                ))),
                terminal("g").ignore_then(choice((
                    terminal("as").ignore_then(choice((
                        terminal("limit").map(|_| 8usize),
                        terminal("price").map(|_| 8usize),
                        empty().map(|_| 3usize),
                    ))),
                    terminal("t").map(|_| 2usize),
                ))),
                terminal("hex").map(|_| 3usize),
                terminal("i").ignore_then(choice((
                    terminal("f").map(|_| 2usize),
                    terminal("nvalid").map(|_| 7usize),
                    terminal("szero").map(|_| 6usize),
                ))),
                terminal("keccak256").map(|_| 9usize),
                terminal("l").ignore_then(choice((
                    terminal("e").ignore_then(choice((
                        terminal("ave").map(|_| 5usize),
                        terminal("t").map(|_| 3usize),
                    ))),
                    terminal("og").ignore_then(choice((
                        terminal("0").map(|_| 4usize),
                        terminal("1").map(|_| 4usize),
                        terminal("2").map(|_| 4usize),
                        terminal("3").map(|_| 4usize),
                        terminal("4").map(|_| 4usize),
                    ))),
                    terminal("t").map(|_| 2usize),
                ))),
                terminal("m").ignore_then(choice((
                    terminal("load").map(|_| 5usize),
                    terminal("od").map(|_| 3usize),
                    terminal("s").ignore_then(choice((
                        terminal("ize").map(|_| 5usize),
                        terminal("tore").ignore_then(choice((
                            terminal("8").map(|_| 7usize),
                            empty().map(|_| 6usize),
                        ))),
                    ))),
                    terminal("ul").ignore_then(choice((
                        terminal("mod").map(|_| 6usize),
                        empty().map(|_| 3usize),
                    ))),
                ))),
                terminal("n").ignore_then(choice((
                    terminal("ot").map(|_| 3usize),
                    terminal("umber").map(|_| 6usize),
                ))),
                terminal("or").ignore_then(choice((
                    terminal("igin").map(|_| 6usize),
                    empty().map(|_| 2usize),
                ))),
                terminal("pop").map(|_| 3usize),
                terminal("re").ignore_then(choice((
                    terminal("turn").ignore_then(choice((
                        terminal("data").ignore_then(choice((
                            terminal("copy").map(|_| 14usize),
                            terminal("size").map(|_| 14usize),
                        ))),
                        empty().map(|_| 6usize),
                    ))),
                    terminal("vert").map(|_| 6usize),
                ))),
                terminal("s").ignore_then(choice((
                    terminal("ar").map(|_| 3usize),
                    terminal("div").map(|_| 4usize),
                    terminal("elf").ignore_then(choice((
                        terminal("balance").map(|_| 11usize),
                        terminal("destruct").map(|_| 12usize),
                    ))),
                    terminal("gt").map(|_| 3usize),
                    terminal("h").ignore_then(choice((
                        terminal("l").map(|_| 3usize),
                        terminal("r").map(|_| 3usize),
                    ))),
                    terminal("ignextend").map(|_| 10usize),
                    terminal("l").ignore_then(choice((
                        terminal("oad").map(|_| 5usize),
                        terminal("t").map(|_| 3usize),
                    ))),
                    terminal("mod").map(|_| 4usize),
                    terminal("store").map(|_| 6usize),
                    terminal("t").ignore_then(choice((
                        terminal("aticcall").map(|_| 10usize),
                        terminal("op").map(|_| 4usize),
                    ))),
                    terminal("ub").map(|_| 3usize),
                    terminal("witch").map(|_| 6usize),
                ))),
                terminal("t").ignore_then(choice((
                    terminal("imestamp").map(|_| 9usize),
                    terminal("rue").map(|_| 4usize),
                ))),
                terminal("xor").map(|_| 3usize),
            ))
            .map(VariableSizeTerminal),
        )
        .boxed();

        // AddressType = 'address' [ 'payable' ] ;
        let address_type_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("address")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<7usize>()),
            )
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
                    .then(
                        terminal("payable")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<7usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .or_not(),
            )
            .map(|v| address_type::_T0::from_parse(v))
            .boxed();

        // ArrayLiteral = '[' 1…*{ Expression / ',' } ']' ;
        let array_literal_parser = leading_trivia_parser
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
            .then(
                expression_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(expression_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| array_literal::_T1 {
                        elements,
                        separators,
                    }),
            )
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
            .map(|v| array_literal::_T0::from_parse(v))
            .boxed();

        // BreakStatement = 'break' ';' ;
        let break_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("break")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<5usize>()),
            )
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
            .map(|v| break_statement::_T0::from_parse(v))
            .boxed();

        // ContinueStatement = 'continue' ';' ;
        let continue_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("continue")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
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
            .map(|v| continue_statement::_T0::from_parse(v))
            .boxed();

        // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
        let decimal_number_parser = choice((
            decimal_integer_parser
                .clone()
                .map(|v| Box::new(decimal_number::_T1::DecimalInteger(v))),
            decimal_float_parser
                .clone()
                .map(|v| Box::new(decimal_number::_T1::DecimalFloat(v))),
        ))
        .then(decimal_exponent_parser.clone().or_not())
        .map(|v| decimal_number::_T0::from_parse(v))
        .boxed();

        // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
        let double_quoted_ascii_string_literal_parser = just('"')
            .map(|_| FixedSizeTerminal::<1>())
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '"' && c != '\\')
                        .map(|_| FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .map(|v| Box::new(double_quoted_ascii_string_literal::Run::Chars(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(double_quoted_ascii_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('"').map(|_| FixedSizeTerminal::<1>()))
            .map(|v| double_quoted_ascii_string_literal::_T0::from_parse(v))
            .boxed();

        // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
        let double_quoted_unicode_string_literal_parser = terminal("unicode\"")
            .ignored()
            .map(|_| FixedSizeTerminal::<8usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '"' && c != '\\' && c != '\n' && c != '\r')
                        .map(|_| FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .map(|v| Box::new(double_quoted_unicode_string_literal::Run::Chars(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(double_quoted_unicode_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('"').map(|_| FixedSizeTerminal::<1>()))
            .map(|v| double_quoted_unicode_string_literal::_T0::from_parse(v))
            .boxed();

        // «Keyword» = 'pragma' | 'abstract' | 'anonymous' | 'address' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | 'fixed' | 'ufixed' ;
        let keyword_parser = choice((
            choice::<_, ErrorType>((
                terminal("a").ignore_then(choice((
                    terminal("bstract").map(|_| 8usize),
                    terminal("ddress").map(|_| 7usize),
                    terminal("nonymous").map(|_| 9usize),
                    terminal("s").ignore_then(choice((
                        terminal("sembly").map(|_| 8usize),
                        empty().map(|_| 2usize),
                    ))),
                ))),
                terminal("b").ignore_then(choice((
                    terminal("ool").map(|_| 4usize),
                    terminal("reak").map(|_| 5usize),
                ))),
                terminal("c").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("lldata").map(|_| 8usize),
                        terminal("tch").map(|_| 5usize),
                    ))),
                    terminal("on").ignore_then(choice((
                        terminal("st").ignore_then(choice((
                            terminal("ant").map(|_| 8usize),
                            terminal("ructor").map(|_| 11usize),
                        ))),
                        terminal("t").ignore_then(choice((
                            terminal("inue").map(|_| 8usize),
                            terminal("ract").map(|_| 8usize),
                        ))),
                    ))),
                ))),
                terminal("d").ignore_then(choice((
                    terminal("elete").map(|_| 6usize),
                    terminal("o").map(|_| 2usize),
                ))),
                terminal("e").ignore_then(choice((
                    terminal("lse").map(|_| 4usize),
                    terminal("mit").map(|_| 4usize),
                    terminal("num").map(|_| 4usize),
                    terminal("vent").map(|_| 5usize),
                    terminal("xternal").map(|_| 8usize),
                ))),
                terminal("f").ignore_then(choice((
                    terminal("al").ignore_then(choice((
                        terminal("lback").map(|_| 8usize),
                        terminal("se").map(|_| 5usize),
                    ))),
                    terminal("or").map(|_| 3usize),
                    terminal("unction").map(|_| 8usize),
                ))),
                terminal("hex").map(|_| 3usize),
                terminal("i").ignore_then(choice((
                    terminal("f").map(|_| 2usize),
                    terminal("m").ignore_then(choice((
                        terminal("mutable").map(|_| 9usize),
                        terminal("port").map(|_| 6usize),
                    ))),
                    terminal("n").ignore_then(choice((
                        terminal("dexed").map(|_| 7usize),
                        terminal("ter").ignore_then(choice((
                            terminal("face").map(|_| 9usize),
                            terminal("nal").map(|_| 8usize),
                        ))),
                    ))),
                    terminal("s").map(|_| 2usize),
                ))),
                terminal("library").map(|_| 7usize),
                terminal("m").ignore_then(choice((
                    terminal("apping").map(|_| 7usize),
                    terminal("emory").map(|_| 6usize),
                    terminal("odifier").map(|_| 8usize),
                ))),
                terminal("new").map(|_| 3usize),
                terminal("override").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("ayable").map(|_| 7usize),
                    terminal("r").ignore_then(choice((
                        terminal("agma").map(|_| 6usize),
                        terminal("ivate").map(|_| 7usize),
                    ))),
                    terminal("u").ignore_then(choice((
                        terminal("blic").map(|_| 6usize),
                        terminal("re").map(|_| 4usize),
                    ))),
                ))),
                terminal("re").ignore_then(choice((
                    terminal("ceive").map(|_| 7usize),
                    terminal("turn").ignore_then(choice((
                        terminal("s").map(|_| 7usize),
                        empty().map(|_| 6usize),
                    ))),
                ))),
                terminal("st").ignore_then(choice((
                    terminal("orage").map(|_| 7usize),
                    terminal("r").ignore_then(choice((
                        terminal("ing").map(|_| 6usize),
                        terminal("uct").map(|_| 6usize),
                    ))),
                ))),
                terminal("t").ignore_then(choice((
                    terminal("r").ignore_then(choice((
                        terminal("ue").map(|_| 4usize),
                        terminal("y").map(|_| 3usize),
                    ))),
                    terminal("ype").map(|_| 4usize),
                ))),
                terminal("u").ignore_then(choice((
                    terminal("nchecked").map(|_| 9usize),
                    terminal("sing").map(|_| 5usize),
                ))),
                terminal("vi").ignore_then(choice((
                    terminal("ew").map(|_| 4usize),
                    terminal("rtual").map(|_| 7usize),
                ))),
                terminal("while").map(|_| 5usize),
            ))
            .map(VariableSizeTerminal)
            .map(|v| Box::new(keyword::_T0::_0(v))),
            signed_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::_T0::SignedIntegerType(v))),
            unsigned_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::_T0::UnsignedIntegerType(v))),
            fixed_bytes_type_parser
                .clone()
                .map(|v| Box::new(keyword::_T0::FixedBytesType(v))),
            choice::<_, ErrorType>((
                terminal("fixed").map(|_| 5usize),
                terminal("ufixed").map(|_| 6usize),
            ))
            .map(VariableSizeTerminal)
            .map(|v| Box::new(keyword::_T0::_4(v))),
        ))
        .boxed();

        // ParenthesisExpression = '(' 1…*{ [ Expression ] / ',' } ')' ;
        let parenthesis_expression_parser = leading_trivia_parser
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
            .then(
                expression_parser
                    .clone()
                    .or_not()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(expression_parser.clone().or_not())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| parenthesis_expression::_T1 {
                        elements,
                        separators,
                    }),
            )
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
            .map(|v| parenthesis_expression::_T0::from_parse(v))
            .boxed();

        // PositionalArgumentList = 1…*{ Expression / ',' } ;
        let positional_argument_list_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(expression_parser.clone())
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|(elements, separators)| positional_argument_list::_T0 {
                elements,
                separators,
            })
            .boxed();

        // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_ascii_string_literal_parser = just('\'')
            .map(|_| FixedSizeTerminal::<1>())
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '\'' && c != '\\')
                        .map(|_| FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .map(|v| Box::new(single_quoted_ascii_string_literal::Run::Chars(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(single_quoted_ascii_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('\'').map(|_| FixedSizeTerminal::<1>()))
            .map(|v| single_quoted_ascii_string_literal::_T0::from_parse(v))
            .boxed();

        // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_unicode_string_literal_parser = terminal("unicode'")
            .ignored()
            .map(|_| FixedSizeTerminal::<8usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '\'' && c != '\\' && c != '\n' && c != '\r')
                        .map(|_| FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .map(|v| Box::new(single_quoted_unicode_string_literal::Run::Chars(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(single_quoted_unicode_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('\'').map(|_| FixedSizeTerminal::<1>()))
            .map(|v| single_quoted_unicode_string_literal::_T0::from_parse(v))
            .boxed();

        // UncheckedBlock = 'unchecked' Block ;
        let unchecked_block_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("unchecked")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<9usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(block_parser.clone())
            .map(|v| unchecked_block::_T0::from_parse(v))
            .boxed();

        // VersionPragmaSpecifier = 'solidity' 1…*{ «VersionPragmaOperator» «VersionPragmaValue» } ;
        let version_pragma_specifier_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("solidity")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
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
                    .then(choice::<_, ErrorType>((
                        terminal("<").ignore_then(choice((
                            terminal("=").map(|_| 2usize),
                            empty().map(|_| 1usize),
                        ))),
                        terminal("=").map(|_| 1usize),
                        terminal(">").ignore_then(choice((
                            terminal("=").map(|_| 2usize),
                            empty().map(|_| 1usize),
                        ))),
                        terminal("^").map(|_| 1usize),
                        terminal("~").map(|_| 1usize),
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                            leading,
                            content: VariableSizeTerminal(content),
                            trailing,
                        },
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(version_pragma_value_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading, content), trailing)| {
                                version_pragma_value::WithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                }
                            }),
                    )
                    .map(|v| version_pragma_specifier::_T2::from_parse(v))
                    .repeated()
                    .at_least(1usize),
            )
            .map(|v| version_pragma_specifier::_T0::from_parse(v))
            .boxed();

        // YulFunctionCall = ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) '(' { YulExpression / ',' } ')' ;
        let yul_function_call_parser = choice((
            leading_trivia_parser
                .clone()
                .then(yul_identifier_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| yul_identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(yul_function_call::_T1::YulIdentifier(v))),
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("Blockhash").map(|_| 9usize),
                    terminal("a").ignore_then(choice((
                        terminal("dd").ignore_then(choice((
                            terminal("mod").map(|_| 6usize),
                            terminal("ress").map(|_| 7usize),
                            empty().map(|_| 3usize),
                        ))),
                        terminal("nd").map(|_| 3usize),
                    ))),
                    terminal("b").ignore_then(choice((
                        terminal("a").ignore_then(choice((
                            terminal("lance").map(|_| 7usize),
                            terminal("sefee").map(|_| 7usize),
                        ))),
                        terminal("yte").map(|_| 4usize),
                    ))),
                    terminal("c").ignore_then(choice((
                        terminal("all").ignore_then(choice((
                            terminal("code").map(|_| 8usize),
                            terminal("data").ignore_then(choice((
                                terminal("copy").map(|_| 12usize),
                                terminal("load").map(|_| 12usize),
                                terminal("size").map(|_| 12usize),
                            ))),
                            terminal("er").map(|_| 6usize),
                            terminal("value").map(|_| 9usize),
                            empty().map(|_| 4usize),
                        ))),
                        terminal("hainid").map(|_| 7usize),
                        terminal("oinbase").map(|_| 8usize),
                        terminal("reate").ignore_then(choice((
                            terminal("2").map(|_| 7usize),
                            empty().map(|_| 6usize),
                        ))),
                    ))),
                    terminal("d").ignore_then(choice((
                        terminal("elegatecall").map(|_| 12usize),
                        terminal("i").ignore_then(choice((
                            terminal("fficulty").map(|_| 10usize),
                            terminal("v").map(|_| 3usize),
                        ))),
                    ))),
                    terminal("e").ignore_then(choice((
                        terminal("q").map(|_| 2usize),
                        terminal("x").ignore_then(choice((
                            terminal("p").map(|_| 3usize),
                            terminal("tcode").ignore_then(choice((
                                terminal("copy").map(|_| 11usize),
                                terminal("hash").map(|_| 11usize),
                                terminal("size").map(|_| 11usize),
                            ))),
                        ))),
                    ))),
                    terminal("g").ignore_then(choice((
                        terminal("as").ignore_then(choice((
                            terminal("limit").map(|_| 8usize),
                            terminal("price").map(|_| 8usize),
                            empty().map(|_| 3usize),
                        ))),
                        terminal("t").map(|_| 2usize),
                    ))),
                    terminal("i").ignore_then(choice((
                        terminal("nvalid").map(|_| 7usize),
                        terminal("szero").map(|_| 6usize),
                    ))),
                    terminal("keccak256").map(|_| 9usize),
                    terminal("l").ignore_then(choice((
                        terminal("og").ignore_then(choice((
                            terminal("0").map(|_| 4usize),
                            terminal("1").map(|_| 4usize),
                            terminal("2").map(|_| 4usize),
                            terminal("3").map(|_| 4usize),
                            terminal("4").map(|_| 4usize),
                        ))),
                        terminal("t").map(|_| 2usize),
                    ))),
                    terminal("m").ignore_then(choice((
                        terminal("load").map(|_| 5usize),
                        terminal("od").map(|_| 3usize),
                        terminal("s").ignore_then(choice((
                            terminal("ize").map(|_| 5usize),
                            terminal("tore").ignore_then(choice((
                                terminal("8").map(|_| 7usize),
                                empty().map(|_| 6usize),
                            ))),
                        ))),
                        terminal("ul").ignore_then(choice((
                            terminal("mod").map(|_| 6usize),
                            empty().map(|_| 3usize),
                        ))),
                    ))),
                    terminal("n").ignore_then(choice((
                        terminal("ot").map(|_| 3usize),
                        terminal("umber").map(|_| 6usize),
                    ))),
                    terminal("or").ignore_then(choice((
                        terminal("igin").map(|_| 6usize),
                        empty().map(|_| 2usize),
                    ))),
                    terminal("pop").map(|_| 3usize),
                    terminal("re").ignore_then(choice((
                        terminal("turn").ignore_then(choice((
                            terminal("data").ignore_then(choice((
                                terminal("copy").map(|_| 14usize),
                                terminal("size").map(|_| 14usize),
                            ))),
                            empty().map(|_| 6usize),
                        ))),
                        terminal("vert").map(|_| 6usize),
                    ))),
                    terminal("s").ignore_then(choice((
                        terminal("ar").map(|_| 3usize),
                        terminal("div").map(|_| 4usize),
                        terminal("elf").ignore_then(choice((
                            terminal("balance").map(|_| 11usize),
                            terminal("destruct").map(|_| 12usize),
                        ))),
                        terminal("gt").map(|_| 3usize),
                        terminal("h").ignore_then(choice((
                            terminal("l").map(|_| 3usize),
                            terminal("r").map(|_| 3usize),
                        ))),
                        terminal("ignextend").map(|_| 10usize),
                        terminal("l").ignore_then(choice((
                            terminal("oad").map(|_| 5usize),
                            terminal("t").map(|_| 3usize),
                        ))),
                        terminal("mod").map(|_| 4usize),
                        terminal("store").map(|_| 6usize),
                        terminal("t").ignore_then(choice((
                            terminal("aticcall").map(|_| 10usize),
                            terminal("op").map(|_| 4usize),
                        ))),
                        terminal("ub").map(|_| 3usize),
                    ))),
                    terminal("timestamp").map(|_| 9usize),
                    terminal("xor").map(|_| 3usize),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(yul_function_call::_T1::_1(v))),
        ))
        .then(
            leading_trivia_parser
                .clone()
                .then(just('(').map(|_| FixedSizeTerminal::<1>()))
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
            yul_expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(
                            |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                leading,
                                content,
                                trailing,
                            },
                        )
                        .then(yul_expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|(elements, separators)| yul_function_call::_T2 {
                    elements,
                    separators,
                })
                .or_not(),
        )
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
        .map(|v| yul_function_call::_T0::from_parse(v))
        .boxed();

        // YulFunctionDefinition = 'function' «YulIdentifier» '(' { «YulIdentifier» / ',' } ')' [ '->' 1…*{ «YulIdentifier» / ',' } ] YulBlock ;
        let yul_function_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("function")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
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
                    .then(yul_identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| yul_identifier::WithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
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
                    .then(yul_identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| yul_identifier::WithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
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
                                    .then(yul_identifier_parser.clone())
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading, content), trailing)| {
                                        yul_identifier::WithTrivia {
                                            leading,
                                            content,
                                            trailing,
                                        }
                                    }),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| yul_function_definition::_T1 {
                        elements,
                        separators,
                    })
                    .or_not(),
            )
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
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("->")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
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
                            .then(yul_identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| yul_identifier::WithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading, content), trailing)| {
                                        FixedSizeTerminalWithTrivia {
                                            leading,
                                            content,
                                            trailing,
                                        }
                                    })
                                    .then(
                                        leading_trivia_parser
                                            .clone()
                                            .then(yul_identifier_parser.clone())
                                            .then(trailing_trivia_parser.clone())
                                            .map(|((leading, content), trailing)| {
                                                yul_identifier::WithTrivia {
                                                    leading,
                                                    content,
                                                    trailing,
                                                }
                                            }),
                                    )
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|(elements, separators)| yul_function_definition::_T3 {
                                elements,
                                separators,
                            }),
                    )
                    .map(|v| yul_function_definition::_T2::from_parse(v))
                    .or_not(),
            )
            .then(yul_block_parser.clone())
            .map(|v| yul_function_definition::_T0::from_parse(v))
            .boxed();

        // YulPath = «YulIdentifier» { '.' ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) } ;
        let yul_path_parser = leading_trivia_parser
            .clone()
            .then(yul_identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| yul_identifier::WithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('.').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(choice((
                        leading_trivia_parser
                            .clone()
                            .then(yul_identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| yul_identifier::WithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .map(|v| Box::new(yul_path::_T3::YulIdentifier(v))),
                        leading_trivia_parser
                            .clone()
                            .then(choice::<_, ErrorType>((
                                terminal("Blockhash").map(|_| 9usize),
                                terminal("a").ignore_then(choice((
                                    terminal("dd").ignore_then(choice((
                                        terminal("mod").map(|_| 6usize),
                                        terminal("ress").map(|_| 7usize),
                                        empty().map(|_| 3usize),
                                    ))),
                                    terminal("nd").map(|_| 3usize),
                                ))),
                                terminal("b").ignore_then(choice((
                                    terminal("a").ignore_then(choice((
                                        terminal("lance").map(|_| 7usize),
                                        terminal("sefee").map(|_| 7usize),
                                    ))),
                                    terminal("yte").map(|_| 4usize),
                                ))),
                                terminal("c").ignore_then(choice((
                                    terminal("all").ignore_then(choice((
                                        terminal("code").map(|_| 8usize),
                                        terminal("data").ignore_then(choice((
                                            terminal("copy").map(|_| 12usize),
                                            terminal("load").map(|_| 12usize),
                                            terminal("size").map(|_| 12usize),
                                        ))),
                                        terminal("er").map(|_| 6usize),
                                        terminal("value").map(|_| 9usize),
                                        empty().map(|_| 4usize),
                                    ))),
                                    terminal("hainid").map(|_| 7usize),
                                    terminal("oinbase").map(|_| 8usize),
                                    terminal("reate").ignore_then(choice((
                                        terminal("2").map(|_| 7usize),
                                        empty().map(|_| 6usize),
                                    ))),
                                ))),
                                terminal("d").ignore_then(choice((
                                    terminal("elegatecall").map(|_| 12usize),
                                    terminal("i").ignore_then(choice((
                                        terminal("fficulty").map(|_| 10usize),
                                        terminal("v").map(|_| 3usize),
                                    ))),
                                ))),
                                terminal("e").ignore_then(choice((
                                    terminal("q").map(|_| 2usize),
                                    terminal("x").ignore_then(choice((
                                        terminal("p").map(|_| 3usize),
                                        terminal("tcode").ignore_then(choice((
                                            terminal("copy").map(|_| 11usize),
                                            terminal("hash").map(|_| 11usize),
                                            terminal("size").map(|_| 11usize),
                                        ))),
                                    ))),
                                ))),
                                terminal("g").ignore_then(choice((
                                    terminal("as").ignore_then(choice((
                                        terminal("limit").map(|_| 8usize),
                                        terminal("price").map(|_| 8usize),
                                        empty().map(|_| 3usize),
                                    ))),
                                    terminal("t").map(|_| 2usize),
                                ))),
                                terminal("i").ignore_then(choice((
                                    terminal("nvalid").map(|_| 7usize),
                                    terminal("szero").map(|_| 6usize),
                                ))),
                                terminal("keccak256").map(|_| 9usize),
                                terminal("l").ignore_then(choice((
                                    terminal("og").ignore_then(choice((
                                        terminal("0").map(|_| 4usize),
                                        terminal("1").map(|_| 4usize),
                                        terminal("2").map(|_| 4usize),
                                        terminal("3").map(|_| 4usize),
                                        terminal("4").map(|_| 4usize),
                                    ))),
                                    terminal("t").map(|_| 2usize),
                                ))),
                                terminal("m").ignore_then(choice((
                                    terminal("load").map(|_| 5usize),
                                    terminal("od").map(|_| 3usize),
                                    terminal("s").ignore_then(choice((
                                        terminal("ize").map(|_| 5usize),
                                        terminal("tore").ignore_then(choice((
                                            terminal("8").map(|_| 7usize),
                                            empty().map(|_| 6usize),
                                        ))),
                                    ))),
                                    terminal("ul").ignore_then(choice((
                                        terminal("mod").map(|_| 6usize),
                                        empty().map(|_| 3usize),
                                    ))),
                                ))),
                                terminal("n").ignore_then(choice((
                                    terminal("ot").map(|_| 3usize),
                                    terminal("umber").map(|_| 6usize),
                                ))),
                                terminal("or").ignore_then(choice((
                                    terminal("igin").map(|_| 6usize),
                                    empty().map(|_| 2usize),
                                ))),
                                terminal("pop").map(|_| 3usize),
                                terminal("re").ignore_then(choice((
                                    terminal("turn").ignore_then(choice((
                                        terminal("data").ignore_then(choice((
                                            terminal("copy").map(|_| 14usize),
                                            terminal("size").map(|_| 14usize),
                                        ))),
                                        empty().map(|_| 6usize),
                                    ))),
                                    terminal("vert").map(|_| 6usize),
                                ))),
                                terminal("s").ignore_then(choice((
                                    terminal("ar").map(|_| 3usize),
                                    terminal("div").map(|_| 4usize),
                                    terminal("elf").ignore_then(choice((
                                        terminal("balance").map(|_| 11usize),
                                        terminal("destruct").map(|_| 12usize),
                                    ))),
                                    terminal("gt").map(|_| 3usize),
                                    terminal("h").ignore_then(choice((
                                        terminal("l").map(|_| 3usize),
                                        terminal("r").map(|_| 3usize),
                                    ))),
                                    terminal("ignextend").map(|_| 10usize),
                                    terminal("l").ignore_then(choice((
                                        terminal("oad").map(|_| 5usize),
                                        terminal("t").map(|_| 3usize),
                                    ))),
                                    terminal("mod").map(|_| 4usize),
                                    terminal("store").map(|_| 6usize),
                                    terminal("t").ignore_then(choice((
                                        terminal("aticcall").map(|_| 10usize),
                                        terminal("op").map(|_| 4usize),
                                    ))),
                                    terminal("ub").map(|_| 3usize),
                                ))),
                                terminal("timestamp").map(|_| 9usize),
                                terminal("xor").map(|_| 3usize),
                            )))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                                    leading,
                                    content: VariableSizeTerminal(content),
                                    trailing,
                                },
                            )
                            .map(|v| Box::new(yul_path::_T3::_1(v))),
                    )))
                    .map(|v| yul_path::_T2::from_parse(v))
                    .repeated(),
            )
            .map(|v| yul_path::_T0::from_parse(v))
            .boxed();

        // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
        let ascii_string_literal_parser = choice((
            single_quoted_ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(ascii_string_literal::_T0::SingleQuotedAsciiStringLiteral(v))),
            double_quoted_ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(ascii_string_literal::_T0::DoubleQuotedAsciiStringLiteral(v))),
        ))
        .boxed();

        // AssemblyFlags = '(' 1…*{ «DoubleQuotedAsciiStringLiteral» / ',' } ')' ;
        let assembly_flags_parser = leading_trivia_parser
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
            .then(
                leading_trivia_parser
                    .clone()
                    .then(double_quoted_ascii_string_literal_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| {
                        double_quoted_ascii_string_literal::WithTrivia {
                            leading,
                            content,
                            trailing,
                        }
                    })
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
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
                                    .then(double_quoted_ascii_string_literal_parser.clone())
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading, content), trailing)| {
                                        double_quoted_ascii_string_literal::WithTrivia {
                                            leading,
                                            content,
                                            trailing,
                                        }
                                    }),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| assembly_flags::_T1 {
                        elements,
                        separators,
                    }),
            )
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
            .map(|v| assembly_flags::_T0::from_parse(v))
            .boxed();

        // ElementaryType = 'bool' | 'string' | AddressType | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | «FixedType» | «UfixedType» ;
        let elementary_type_parser = choice((
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("bool").map(|_| 4usize),
                    terminal("string").map(|_| 6usize),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(elementary_type::_T0::_0(v))),
            address_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_T0::AddressType(v))),
            leading_trivia_parser
                .clone()
                .then(signed_integer_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| signed_integer_type::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(elementary_type::_T0::SignedIntegerType(v))),
            leading_trivia_parser
                .clone()
                .then(unsigned_integer_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| unsigned_integer_type::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(elementary_type::_T0::UnsignedIntegerType(v))),
            leading_trivia_parser
                .clone()
                .then(fixed_bytes_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| fixed_bytes_type::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(elementary_type::_T0::FixedBytesType(v))),
            leading_trivia_parser
                .clone()
                .then(fixed_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading, content), trailing)| fixed_type::WithTrivia {
                    leading,
                    content,
                    trailing,
                })
                .map(|v| Box::new(elementary_type::_T0::FixedType(v))),
            leading_trivia_parser
                .clone()
                .then(ufixed_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading, content), trailing)| ufixed_type::WithTrivia {
                    leading,
                    content,
                    trailing,
                })
                .map(|v| Box::new(elementary_type::_T0::UfixedType(v))),
        ))
        .boxed();

        // «NumericLiteral» = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
        let numeric_literal_parser = choice((
            decimal_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::_T1::DecimalNumber(v))),
            hex_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::_T1::HexNumber(v))),
        ))
        .then(
            choice::<_, ErrorType>((
                terminal("days").map(|_| 4usize),
                terminal("ether").map(|_| 5usize),
                terminal("gwei").map(|_| 4usize),
                terminal("hours").map(|_| 5usize),
                terminal("minutes").map(|_| 7usize),
                terminal("seconds").map(|_| 7usize),
                terminal("we").ignore_then(choice((
                    terminal("eks").map(|_| 5usize),
                    terminal("i").map(|_| 3usize),
                ))),
                terminal("years").map(|_| 5usize),
            ))
            .map(VariableSizeTerminal)
            .or_not(),
        )
        .map(|v| numeric_literal::_T0::from_parse(v))
        .boxed();

        // «ReservedWord» = «Keyword» | «ReservedKeyword» | «NumberUnit» | «BooleanLiteral» ;
        let reserved_word_parser = choice((
            keyword_parser
                .clone()
                .map(|v| Box::new(reserved_word::_T0::Keyword(v))),
            choice::<_, ErrorType>((
                terminal("a").ignore_then(choice((
                    terminal("fter").map(|_| 5usize),
                    terminal("lias").map(|_| 5usize),
                    terminal("pply").map(|_| 5usize),
                    terminal("uto").map(|_| 4usize),
                ))),
                terminal("byte").map(|_| 4usize),
                terminal("c").ignore_then(choice((
                    terminal("ase").map(|_| 4usize),
                    terminal("opyof").map(|_| 6usize),
                ))),
                terminal("d").ignore_then(choice((
                    terminal("ays").map(|_| 4usize),
                    terminal("ef").ignore_then(choice((
                        terminal("ault").map(|_| 7usize),
                        terminal("ine").map(|_| 6usize),
                    ))),
                ))),
                terminal("ether").map(|_| 5usize),
                terminal("f").ignore_then(choice((
                    terminal("alse").map(|_| 5usize),
                    terminal("inal").map(|_| 5usize),
                ))),
                terminal("gwei").map(|_| 4usize),
                terminal("hours").map(|_| 5usize),
                terminal("i").ignore_then(choice((
                    terminal("mplements").map(|_| 10usize),
                    terminal("n").ignore_then(choice((
                        terminal("line").map(|_| 6usize),
                        empty().map(|_| 2usize),
                    ))),
                ))),
                terminal("let").map(|_| 3usize),
                terminal("m").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("cro").map(|_| 5usize),
                        terminal("tch").map(|_| 5usize),
                    ))),
                    terminal("inutes").map(|_| 7usize),
                    terminal("utable").map(|_| 7usize),
                ))),
                terminal("null").map(|_| 4usize),
                terminal("of").map(|_| 2usize),
                terminal("p").ignore_then(choice((
                    terminal("artial").map(|_| 7usize),
                    terminal("romise").map(|_| 7usize),
                ))),
                terminal("re").ignore_then(choice((
                    terminal("ference").map(|_| 9usize),
                    terminal("locatable").map(|_| 11usize),
                ))),
                terminal("s").ignore_then(choice((
                    terminal("e").ignore_then(choice((
                        terminal("aled").map(|_| 6usize),
                        terminal("conds").map(|_| 7usize),
                    ))),
                    terminal("izeof").map(|_| 6usize),
                    terminal("tatic").map(|_| 6usize),
                    terminal("upports").map(|_| 8usize),
                    terminal("witch").map(|_| 6usize),
                ))),
                terminal("t").ignore_then(choice((
                    terminal("rue").map(|_| 4usize),
                    terminal("ype").ignore_then(choice((
                        terminal("def").map(|_| 7usize),
                        terminal("of").map(|_| 6usize),
                    ))),
                ))),
                terminal("var").map(|_| 3usize),
                terminal("we").ignore_then(choice((
                    terminal("eks").map(|_| 5usize),
                    terminal("i").map(|_| 3usize),
                ))),
                terminal("years").map(|_| 5usize),
            ))
            .map(VariableSizeTerminal)
            .map(|v| Box::new(reserved_word::_T0::_1(v))),
        ))
        .boxed();

        // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
        let unicode_string_literal_parser = choice((
            single_quoted_unicode_string_literal_parser
                .clone()
                .map(|v| {
                    Box::new(unicode_string_literal::_T0::SingleQuotedUnicodeStringLiteral(v))
                }),
            double_quoted_unicode_string_literal_parser
                .clone()
                .map(|v| {
                    Box::new(unicode_string_literal::_T0::DoubleQuotedUnicodeStringLiteral(v))
                }),
        ))
        .boxed();

        // «Identifier» = «RawIdentifier» - «ReservedWord» ;
        let identifier_parser =
            difference(raw_identifier_parser.clone(), reserved_word_parser.clone()).boxed();

        // ImportPath = «AsciiStringLiteral» ;
        let import_path_parser = leading_trivia_parser
            .clone()
            .then(ascii_string_literal_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| ascii_string_literal::WithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .boxed();

        // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
        let yul_literal_parser = choice((
            leading_trivia_parser
                .clone()
                .then(yul_decimal_number_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| yul_decimal_number_literal::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(yul_literal::_T0::YulDecimalNumberLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(yul_hex_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| yul_hex_literal::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(yul_literal::_T0::YulHexLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(ascii_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| ascii_string_literal::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(yul_literal::_T0::AsciiStringLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("false").map(|_| 5usize),
                    terminal("true").map(|_| 4usize),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(yul_literal::_T0::_3(v))),
            leading_trivia_parser
                .clone()
                .then(hex_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| hex_string_literal::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(yul_literal::_T0::HexStringLiteral(v))),
        ))
        .boxed();

        // ABICoderPragmaSpecifier = 'abicoder' «Identifier» ;
        let abi_coder_pragma_specifier_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("abicoder")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
            .map(|v| abi_coder_pragma_specifier::_T0::from_parse(v))
            .boxed();

        // EnumDefinition = 'enum' «Identifier» '{' 1…*{ «Identifier» / ',' } '}' ;
        let enum_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("enum")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<4usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
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
            .then(
                leading_trivia_parser
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
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
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
                                    .then(identifier_parser.clone())
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                                        leading,
                                        content,
                                        trailing,
                                    }),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| enum_definition::_T1 {
                        elements,
                        separators,
                    }),
            )
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
            .map(|v| enum_definition::_T0::from_parse(v))
            .boxed();

        // ExperimentalPragmaSpecifier = 'experimental' «Identifier» ;
        let experimental_pragma_specifier_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("experimental")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<12usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
            .map(|v| experimental_pragma_specifier::_T0::from_parse(v))
            .boxed();

        // IdentifierPath = 1…*{ «Identifier» / '.' } ;
        let identifier_path_parser = leading_trivia_parser
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
                    .then(just('.').map(|_| FixedSizeTerminal::<1>()))
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
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading, content), trailing)| identifier::WithTrivia {
                                leading,
                                content,
                                trailing,
                            }),
                    )
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|(elements, separators)| identifier_path::_T0 {
                elements,
                separators,
            })
            .boxed();

        // NamedArgument = «Identifier» ':' Expression ;
        let named_argument_parser = leading_trivia_parser
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
                    .then(just(':').map(|_| FixedSizeTerminal::<1>()))
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
            .map(|v| named_argument::_T0::from_parse(v))
            .boxed();

        // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
        let parameter_declaration_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(choice::<_, ErrorType>((
                        terminal("calldata").map(|_| 8usize),
                        terminal("memory").map(|_| 6usize),
                        terminal("storage").map(|_| 7usize),
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                            leading,
                            content: VariableSizeTerminal(content),
                            trailing,
                        },
                    )
                    .or_not(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    })
                    .or_not(),
            )
            .map(|v| parameter_declaration::_T0::from_parse(v))
            .boxed();

        // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
        let selected_import_parser = leading_trivia_parser
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
                    .then(
                        terminal("as")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
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
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading, content), trailing)| identifier::WithTrivia {
                                leading,
                                content,
                                trailing,
                            }),
                    )
                    .map(|v| selected_import::_T1::from_parse(v))
                    .or_not(),
            )
            .map(|v| selected_import::_T0::from_parse(v))
            .boxed();

        // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
        let simple_import_directive_parser = import_path_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("as")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
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
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading, content), trailing)| identifier::WithTrivia {
                                leading,
                                content,
                                trailing,
                            }),
                    )
                    .map(|v| simple_import_directive::_T2::from_parse(v))
                    .repeated(),
            )
            .map(|v| simple_import_directive::_T0::from_parse(v))
            .boxed();

        // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
        let star_import_directive_parser = leading_trivia_parser
            .clone()
            .then(just('*').map(|_| FixedSizeTerminal::<1>()))
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
                    .then(
                        terminal("as")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("from")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<4usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(import_path_parser.clone())
            .map(|v| star_import_directive::_T0::from_parse(v))
            .boxed();

        // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
        let user_defined_value_type_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("type")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<4usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("is")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(elementary_type_parser.clone())
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
            .map(|v| user_defined_value_type_definition::_T0::from_parse(v))
            .boxed();

        // YulExpression = YulPath | YulFunctionCall | YulLiteral ;
        yul_expression_parser.define(
            choice((
                yul_path_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::_T0::YulPath(v))),
                yul_function_call_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::_T0::YulFunctionCall(v))),
                yul_literal_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::_T0::YulLiteral(v))),
            ))
            .boxed(),
        );

        // MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
        let mapping_type_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("mapping")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<7usize>()),
            )
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
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(choice((
                elementary_type_parser
                    .clone()
                    .map(|v| Box::new(mapping_type::_T1::ElementaryType(v))),
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(mapping_type::_T1::IdentifierPath(v))),
            )))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("=>")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(type_name_parser.clone())
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
            .map(|v| mapping_type::_T0::from_parse(v))
            .boxed();

        // NamedArgumentList = '{' { NamedArgument / ',' } '}' ;
        let named_argument_list_parser = leading_trivia_parser
            .clone()
            .then(just('{').map(|_| FixedSizeTerminal::<1>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(
                named_argument_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(named_argument_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| named_argument_list::_T1 {
                        elements,
                        separators,
                    })
                    .or_not(),
            )
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
            .map(|v| named_argument_list::_T0::from_parse(v))
            .boxed();

        // OverrideSpecifier = 'override' [ '(' 1…1*{ IdentifierPath / ',' } ')' ] ;
        let override_specifier_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("override")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
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
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(
                        identifier_path_parser
                            .clone()
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading, content), trailing)| {
                                        FixedSizeTerminalWithTrivia {
                                            leading,
                                            content,
                                            trailing,
                                        }
                                    })
                                    .then(identifier_path_parser.clone())
                                    .repeated()
                                    .at_most(1usize - 1),
                            )
                            .map(repetition_mapper)
                            .map(|(elements, separators)| override_specifier::_T2 {
                                elements,
                                separators,
                            }),
                    )
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
                    .map(|v| override_specifier::_T1::from_parse(v))
                    .or_not(),
            )
            .map(|v| override_specifier::_T0::from_parse(v))
            .boxed();

        // ParameterList = '(' { ParameterDeclaration / ',' } ')' ;
        let parameter_list_parser = leading_trivia_parser
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
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(parameter_declaration_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| parameter_list::_T1 {
                        elements,
                        separators,
                    })
                    .or_not(),
            )
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
            .map(|v| parameter_list::_T0::from_parse(v))
            .boxed();

        // PragmaDirective = 'pragma' ( VersionPragmaSpecifier | ABICoderPragmaSpecifier | ExperimentalPragmaSpecifier ) ';' ;
        let pragma_directive_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("pragma")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<6usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(choice((
                version_pragma_specifier_parser
                    .clone()
                    .map(|v| Box::new(pragma_directive::_T1::VersionPragmaSpecifier(v))),
                abi_coder_pragma_specifier_parser
                    .clone()
                    .map(|v| Box::new(pragma_directive::_T1::AbiCoderPragmaSpecifier(v))),
                experimental_pragma_specifier_parser
                    .clone()
                    .map(|v| Box::new(pragma_directive::_T1::ExperimentalPragmaSpecifier(v))),
            )))
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
            .map(|v| pragma_directive::_T0::from_parse(v))
            .boxed();

        // SelectingImportDirective = '{' 1…*{ SelectedImport / ',' } '}' 'from' ImportPath ;
        let selecting_import_directive_parser = leading_trivia_parser
            .clone()
            .then(just('{').map(|_| FixedSizeTerminal::<1>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(
                selected_import_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(selected_import_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| selecting_import_directive::_T1 {
                        elements,
                        separators,
                    }),
            )
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
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("from")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<4usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(import_path_parser.clone())
            .map(|v| selecting_import_directive::_T0::from_parse(v))
            .boxed();

        // YulAssignment = YulPath ( ':=' YulExpression | 1…*{ ',' YulPath } ':=' YulFunctionCall ) ;
        let yul_assignment_parser = yul_path_parser
            .clone()
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal(":=")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(yul_expression_parser.clone())
                    .map(|v| yul_assignment::_T2::from_parse(v))
                    .map(|v| Box::new(yul_assignment::_T1::_T2(v))),
                leading_trivia_parser
                    .clone()
                    .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(yul_path_parser.clone())
                    .map(|v| yul_assignment::_T5::from_parse(v))
                    .repeated()
                    .at_least(1usize)
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                terminal(":=")
                                    .ignored()
                                    .map(|_| FixedSizeTerminal::<2usize>()),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            ),
                    )
                    .then(yul_function_call_parser.clone())
                    .map(|v| yul_assignment::_T3::from_parse(v))
                    .map(|v| Box::new(yul_assignment::_T1::_T3(v))),
            )))
            .map(|v| yul_assignment::_T0::from_parse(v))
            .boxed();

        // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
        let yul_for_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("for")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<3usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(yul_block_parser.clone())
            .then(yul_expression_parser.clone())
            .then(yul_block_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| yul_for_statement::_T0::from_parse(v))
            .boxed();

        // YulIfStatement = 'if' YulExpression YulBlock ;
        let yul_if_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("if")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<2usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(yul_expression_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| yul_if_statement::_T0::from_parse(v))
            .boxed();

        // YulSwitchStatement = 'switch' YulExpression ( 1…*{ 'case' YulLiteral YulBlock } [ 'default' YulBlock ] | 'default' YulBlock ) ;
        let yul_switch_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("switch")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<6usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(yul_expression_parser.clone())
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("case")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<4usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(yul_literal_parser.clone())
                    .then(yul_block_parser.clone())
                    .map(|v| yul_switch_statement::_T4::from_parse(v))
                    .repeated()
                    .at_least(1usize)
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                terminal("default")
                                    .ignored()
                                    .map(|_| FixedSizeTerminal::<7usize>()),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(yul_block_parser.clone())
                            .map(|v| yul_switch_statement::_T5::from_parse(v))
                            .or_not(),
                    )
                    .map(|v| yul_switch_statement::_T2::from_parse(v))
                    .map(|v| Box::new(yul_switch_statement::_T1::_T2(v))),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("default")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<7usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(yul_block_parser.clone())
                    .map(|v| yul_switch_statement::_T6::from_parse(v))
                    .map(|v| Box::new(yul_switch_statement::_T1::_T6(v))),
            )))
            .map(|v| yul_switch_statement::_T0::from_parse(v))
            .boxed();

        // YulVariableDeclaration = 'let' «YulIdentifier» [ ':=' YulExpression | [ ',' «YulIdentifier» ] [ ':=' YulFunctionCall ] ] ;
        let yul_variable_declaration_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("let")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<3usize>()),
            )
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
                    .then(yul_identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| yul_identifier::WithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(
                choice((
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal(":=")
                                .ignored()
                                .map(|_| FixedSizeTerminal::<2usize>()),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(
                            |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                leading,
                                content,
                                trailing,
                            },
                        )
                        .then(yul_expression_parser.clone())
                        .map(|v| yul_variable_declaration::_T2::from_parse(v))
                        .map(|v| Box::new(yul_variable_declaration::_T1::_T2(v))),
                    leading_trivia_parser
                        .clone()
                        .then(just(',').map(|_| FixedSizeTerminal::<1>()))
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
                                .then(yul_identifier_parser.clone())
                                .then(trailing_trivia_parser.clone())
                                .map(
                                    |((leading, content), trailing)| yul_identifier::WithTrivia {
                                        leading,
                                        content,
                                        trailing,
                                    },
                                ),
                        )
                        .map(|v| yul_variable_declaration::_T4::from_parse(v))
                        .or_not()
                        .then(
                            leading_trivia_parser
                                .clone()
                                .then(
                                    terminal(":=")
                                        .ignored()
                                        .map(|_| FixedSizeTerminal::<2usize>()),
                                )
                                .then(trailing_trivia_parser.clone())
                                .map(
                                    |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                        leading,
                                        content,
                                        trailing,
                                    },
                                )
                                .then(yul_function_call_parser.clone())
                                .map(|v| yul_variable_declaration::_T5::from_parse(v))
                                .or_not(),
                        )
                        .map(|v| yul_variable_declaration::_T3::from_parse(v))
                        .map(|v| Box::new(yul_variable_declaration::_T1::_T3(v))),
                ))
                .or_not(),
            )
            .map(|v| yul_variable_declaration::_T0::from_parse(v))
            .boxed();

        // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
        let argument_list_parser = leading_trivia_parser
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
            .then(
                choice((
                    positional_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::_T1::PositionalArgumentList(v))),
                    named_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::_T1::NamedArgumentList(v))),
                ))
                .or_not(),
            )
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
            .map(|v| argument_list::_T0::from_parse(v))
            .boxed();

        // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
        let catch_clause_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("catch")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<5usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    })
                    .or_not()
                    .then(parameter_list_parser.clone())
                    .map(|v| catch_clause::_T1::from_parse(v))
                    .or_not(),
            )
            .then(block_parser.clone())
            .map(|v| catch_clause::_T0::from_parse(v))
            .boxed();

        // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
        let function_type_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("function")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(parameter_list_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(choice::<_, ErrorType>((
                        terminal("external").map(|_| 8usize),
                        terminal("internal").map(|_| 8usize),
                        terminal("p").ignore_then(choice((
                            terminal("ayable").map(|_| 7usize),
                            terminal("rivate").map(|_| 7usize),
                            terminal("u").ignore_then(choice((
                                terminal("blic").map(|_| 6usize),
                                terminal("re").map(|_| 4usize),
                            ))),
                        ))),
                        terminal("view").map(|_| 4usize),
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                            leading,
                            content: VariableSizeTerminal(content),
                            trailing,
                        },
                    )
                    .repeated(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("returns")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<7usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(parameter_list_parser.clone())
                    .map(|v| function_type::_T2::from_parse(v))
                    .or_not(),
            )
            .map(|v| function_type::_T0::from_parse(v))
            .boxed();

        // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
        let import_directive_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("import")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<6usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(choice((
                simple_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::_T1::SimpleImportDirective(v))),
                star_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::_T1::StarImportDirective(v))),
                selecting_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::_T1::SelectingImportDirective(v))),
            )))
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
            .map(|v| import_directive::_T0::from_parse(v))
            .boxed();

        // ModifierAttribute = OverrideSpecifier | 'virtual' ;
        let modifier_attribute_parser = choice((
            override_specifier_parser
                .clone()
                .map(|v| Box::new(modifier_attribute::_T0::OverrideSpecifier(v))),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("virtual")
                        .ignored()
                        .map(|_| FixedSizeTerminal::<7usize>()),
                )
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(modifier_attribute::_T0::Virtual(v))),
        ))
        .boxed();

        // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
        let state_variable_attribute_parser = choice((
            override_specifier_parser
                .clone()
                .map(|v| Box::new(state_variable_attribute::_T0::OverrideSpecifier(v))),
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("constant").map(|_| 8usize),
                    terminal("i").ignore_then(choice((
                        terminal("mmutable").map(|_| 9usize),
                        terminal("nternal").map(|_| 8usize),
                    ))),
                    terminal("p").ignore_then(choice((
                        terminal("rivate").map(|_| 7usize),
                        terminal("ublic").map(|_| 6usize),
                    ))),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(state_variable_attribute::_T0::_1(v))),
        ))
        .boxed();

        // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignment | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
        let yul_statement_parser = choice((
            yul_block_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulBlock(v))),
            yul_variable_declaration_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulVariableDeclaration(v))),
            yul_function_definition_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulFunctionDefinition(v))),
            yul_assignment_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulAssignment(v))),
            yul_function_call_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulFunctionCall(v))),
            yul_if_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulIfStatement(v))),
            yul_for_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulForStatement(v))),
            yul_switch_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulSwitchStatement(v))),
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("break").map(|_| 5usize),
                    terminal("continue").map(|_| 8usize),
                    terminal("leave").map(|_| 5usize),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(yul_statement::_T0::_8(v))),
        ))
        .boxed();

        // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
        let inheritance_specifier_parser = identifier_path_parser
            .clone()
            .then(argument_list_parser.clone().or_not())
            .map(|v| inheritance_specifier::_T0::from_parse(v))
            .boxed();

        // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
        let modifier_invocation_parser = identifier_path_parser
            .clone()
            .then(argument_list_parser.clone().or_not())
            .map(|v| modifier_invocation::_T0::from_parse(v))
            .boxed();

        // NewExpression = 'new' IdentifierPath ArgumentList ;
        let new_expression_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("new")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<3usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(identifier_path_parser.clone())
            .then(argument_list_parser.clone())
            .map(|v| new_expression::_T0::from_parse(v))
            .boxed();

        // PayableExpression = 'payable' ArgumentList ;
        let payable_expression_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("payable")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<7usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(argument_list_parser.clone())
            .map(|v| payable_expression::_T0::from_parse(v))
            .boxed();

        // TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ElementaryType ;
        type_name_parser.define(
            choice((
                elementary_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::_T1::ElementaryType(v))),
                function_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::_T1::FunctionType(v))),
                mapping_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::_T1::MappingType(v))),
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(type_name::_T1::IdentifierPath(v))),
            ))
            .then(
                leading_trivia_parser
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
                    .then(expression_parser.clone().or_not())
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
                    .map(|v| type_name::_T3::from_parse(v))
                    .repeated(),
            )
            .then(elementary_type_parser.clone())
            .map(|v| type_name::_T0::from_parse(v))
            .boxed(),
        );

        // YulBlock = '{' { YulStatement } '}' ;
        yul_block_parser.define(
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
                )
                .then(yul_statement_parser.clone().repeated())
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
                .map(|v| yul_block::_T0::from_parse(v))
                .boxed(),
        );

        // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
        let assembly_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("assembly")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
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
                    .then(
                        terminal("\"evmasm\"")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<8usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .or_not(),
            )
            .then(assembly_flags_parser.clone().or_not())
            .then(yul_block_parser.clone())
            .map(|v| assembly_statement::_T0::from_parse(v))
            .boxed();

        // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
        let constructor_attribute_parser = choice((
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(constructor_attribute::_T0::ModifierInvocation(v))),
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("internal").map(|_| 8usize),
                    terminal("p").ignore_then(choice((
                        terminal("ayable").map(|_| 7usize),
                        terminal("ublic").map(|_| 6usize),
                    ))),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(constructor_attribute::_T0::_1(v))),
        ))
        .boxed();

        // ErrorParameter = TypeName [ «Identifier» ] ;
        let error_parameter_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    })
                    .or_not(),
            )
            .map(|v| error_parameter::_T0::from_parse(v))
            .boxed();

        // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
        let event_parameter_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("indexed")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<7usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .or_not(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    })
                    .or_not(),
            )
            .map(|v| event_parameter::_T0::from_parse(v))
            .boxed();

        // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
        let fallback_function_attribute_parser = choice((
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(fallback_function_attribute::_T0::ModifierInvocation(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(fallback_function_attribute::_T0::OverrideSpecifier(v))),
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("external").map(|_| 8usize),
                    terminal("p").ignore_then(choice((
                        terminal("ayable").map(|_| 7usize),
                        terminal("ure").map(|_| 4usize),
                    ))),
                    terminal("vi").ignore_then(choice((
                        terminal("ew").map(|_| 4usize),
                        terminal("rtual").map(|_| 7usize),
                    ))),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(fallback_function_attribute::_T0::_2(v))),
        ))
        .boxed();

        // FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
        let function_attribute_parser = choice((
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(function_attribute::_T0::ModifierInvocation(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(function_attribute::_T0::OverrideSpecifier(v))),
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("external").map(|_| 8usize),
                    terminal("internal").map(|_| 8usize),
                    terminal("p").ignore_then(choice((
                        terminal("ayable").map(|_| 7usize),
                        terminal("rivate").map(|_| 7usize),
                        terminal("u").ignore_then(choice((
                            terminal("blic").map(|_| 6usize),
                            terminal("re").map(|_| 4usize),
                        ))),
                    ))),
                    terminal("vi").ignore_then(choice((
                        terminal("ew").map(|_| 4usize),
                        terminal("rtual").map(|_| 7usize),
                    ))),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(function_attribute::_T0::_2(v))),
        ))
        .boxed();

        // InheritanceSpecifierList = 'is' 1…*{ InheritanceSpecifier / ',' } ;
        let inheritance_specifier_list_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("is")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<2usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(
                inheritance_specifier_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(inheritance_specifier_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| inheritance_specifier_list::_T1 {
                        elements,
                        separators,
                    }),
            )
            .map(|v| inheritance_specifier_list::_T0::from_parse(v))
            .boxed();

        // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
        let receive_function_attribute_parser = choice((
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(receive_function_attribute::_T0::ModifierInvocation(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(receive_function_attribute::_T0::OverrideSpecifier(v))),
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("external").map(|_| 8usize),
                    terminal("payable").map(|_| 7usize),
                    terminal("virtual").map(|_| 7usize),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(receive_function_attribute::_T0::_2(v))),
        ))
        .boxed();

        // StructMember = TypeName «Identifier» ';' ;
        let struct_member_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
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
            .map(|v| struct_member::_T0::from_parse(v))
            .boxed();

        // TypeExpression = 'type' '(' TypeName ')' ;
        let type_expression_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("type")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<4usize>()),
            )
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
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(type_name_parser.clone())
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
            .map(|v| type_expression::_T0::from_parse(v))
            .boxed();

        // UsingDirective = 'using' ( IdentifierPath | '{' 1…*{ IdentifierPath / ',' } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
        let using_directive_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("using")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<5usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(choice((
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(using_directive::_T1::IdentifierPath(v))),
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
                    )
                    .then(
                        identifier_path_parser
                            .clone()
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading, content), trailing)| {
                                        FixedSizeTerminalWithTrivia {
                                            leading,
                                            content,
                                            trailing,
                                        }
                                    })
                                    .then(identifier_path_parser.clone())
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|(elements, separators)| using_directive::_T3 {
                                elements,
                                separators,
                            }),
                    )
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
                    .map(|v| using_directive::_T2::from_parse(v))
                    .map(|v| Box::new(using_directive::_T1::_T2(v))),
            )))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("for")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<3usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(choice((
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
                    )
                    .map(|v| Box::new(using_directive::_T4::StarChar(v))),
                type_name_parser
                    .clone()
                    .map(|v| Box::new(using_directive::_T4::TypeName(v))),
            )))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("global")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<6usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .or_not(),
            )
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
            .map(|v| using_directive::_T0::from_parse(v))
            .boxed();

        // Directive = PragmaDirective | ImportDirective | UsingDirective ;
        let directive_parser = choice((
            pragma_directive_parser
                .clone()
                .map(|v| Box::new(directive::_T0::PragmaDirective(v))),
            import_directive_parser
                .clone()
                .map(|v| Box::new(directive::_T0::ImportDirective(v))),
            using_directive_parser
                .clone()
                .map(|v| Box::new(directive::_T0::UsingDirective(v))),
        ))
        .boxed();

        // ErrorDefinition = 'error' «Identifier» '(' { ErrorParameter / ',' } ')' ';' ;
        let error_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("error")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<5usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
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
                error_parameter_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(error_parameter_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| error_definition::_T1 {
                        elements,
                        separators,
                    })
                    .or_not(),
            )
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
            .map(|v| error_definition::_T0::from_parse(v))
            .boxed();

        // EventDefinition = 'event' «Identifier» '(' { EventParameter / ',' } ')' [ 'anonymous' ] ';' ;
        let event_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("event")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<5usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
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
                event_parameter_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(event_parameter_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(elements, separators)| event_definition::_T1 {
                        elements,
                        separators,
                    })
                    .or_not(),
            )
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
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("anonymous")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<9usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .or_not(),
            )
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
            .map(|v| event_definition::_T0::from_parse(v))
            .boxed();

        // PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | «NumericLiteral» | «BooleanLiteral» | «Identifier» ;
        let primary_expression_parser = choice((
            payable_expression_parser
                .clone()
                .map(|v| Box::new(primary_expression::_T0::PayableExpression(v))),
            type_expression_parser
                .clone()
                .map(|v| Box::new(primary_expression::_T0::TypeExpression(v))),
            new_expression_parser
                .clone()
                .map(|v| Box::new(primary_expression::_T0::NewExpression(v))),
            parenthesis_expression_parser
                .clone()
                .map(|v| Box::new(primary_expression::_T0::ParenthesisExpression(v))),
            array_literal_parser
                .clone()
                .map(|v| Box::new(primary_expression::_T0::ArrayLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(ascii_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| ascii_string_literal::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(primary_expression::_T0::AsciiStringLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(unicode_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| unicode_string_literal::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(primary_expression::_T0::UnicodeStringLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(hex_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| hex_string_literal::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(primary_expression::_T0::HexStringLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(numeric_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| numeric_literal::WithTrivia {
                        leading,
                        content,
                        trailing,
                    },
                )
                .map(|v| Box::new(primary_expression::_T0::NumericLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(choice::<_, ErrorType>((
                    terminal("false").map(|_| 5usize),
                    terminal("true").map(|_| 4usize),
                )))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                        leading,
                        content: VariableSizeTerminal(content),
                        trailing,
                    },
                )
                .map(|v| Box::new(primary_expression::_T0::_9(v))),
            leading_trivia_parser
                .clone()
                .then(identifier_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading, content), trailing)| identifier::WithTrivia {
                    leading,
                    content,
                    trailing,
                })
                .map(|v| Box::new(primary_expression::_T0::Identifier(v))),
        ))
        .map(|v| Box::new(expression::Expression::PrimaryExpression(v)))
        .boxed();

        // StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
        let struct_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("struct")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<6usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
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
            .then(struct_member_parser.clone().repeated().at_least(1usize))
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
            .map(|v| struct_definition::_T0::from_parse(v))
            .boxed();

        // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
        let index_access_expression_parser = primary_expression_parser
            .clone()
            .then(
                leading_trivia_parser
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
                    .then(expression_parser.clone().or_not())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(':').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(expression_parser.clone().or_not())
                            .map(|v| index_access_expression::_T1::from_parse(v))
                            .or_not(),
                    )
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
                    .map(|v| index_access_expression::Operator::from_parse(v))
                    .repeated(),
            )
            .map(|(operand, operators)| {
                if operators.is_empty() {
                    operand
                } else {
                    operators
                        .into_iter()
                        .fold(operand, |left_operand, operator| {
                            Box::new(expression::Expression::IndexAccessExpression(
                                index_access_expression::E {
                                    left_operand,
                                    operator,
                                },
                            ))
                        })
                }
            })
            .boxed();

        // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
        let member_access_expression_parser = index_access_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('.').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(choice((
                        leading_trivia_parser
                            .clone()
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading, content), trailing)| identifier::WithTrivia {
                                leading,
                                content,
                                trailing,
                            })
                            .map(|v| Box::new(member_access_expression::_T1::Identifier(v))),
                        leading_trivia_parser
                            .clone()
                            .then(
                                terminal("address")
                                    .ignored()
                                    .map(|_| FixedSizeTerminal::<7usize>()),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .map(|v| Box::new(member_access_expression::_T1::Address(v))),
                    )))
                    .map(|v| member_access_expression::Operator::from_parse(v))
                    .repeated(),
            )
            .map(|(operand, operators)| {
                if operators.is_empty() {
                    operand
                } else {
                    operators
                        .into_iter()
                        .fold(operand, |left_operand, operator| {
                            Box::new(expression::Expression::MemberAccessExpression(
                                member_access_expression::E {
                                    left_operand,
                                    operator,
                                },
                            ))
                        })
                }
            })
            .boxed();

        // FunctionCallOptionsExpression = Expression '{' 1…*{ NamedArgument / ',' } '}' ;
        let function_call_options_expression_parser = member_access_expression_parser
            .clone()
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
                    )
                    .then(
                        named_argument_parser
                            .clone()
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading, content), trailing)| {
                                        FixedSizeTerminalWithTrivia {
                                            leading,
                                            content,
                                            trailing,
                                        }
                                    })
                                    .then(named_argument_parser.clone())
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(
                                |(elements, separators)| function_call_options_expression::_T1 {
                                    elements,
                                    separators,
                                },
                            ),
                    )
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
                    .map(|v| function_call_options_expression::Operator::from_parse(v))
                    .repeated(),
            )
            .map(|(operand, operators)| {
                if operators.is_empty() {
                    operand
                } else {
                    operators
                        .into_iter()
                        .fold(operand, |left_operand, operator| {
                            Box::new(expression::Expression::FunctionCallOptionsExpression(
                                function_call_options_expression::E {
                                    left_operand,
                                    operator,
                                },
                            ))
                        })
                }
            })
            .boxed();

        // FunctionCallExpression = Expression ArgumentList ;
        let function_call_expression_parser = function_call_options_expression_parser
            .clone()
            .then(argument_list_parser.clone().repeated())
            .map(|(operand, operators)| {
                if operators.is_empty() {
                    operand
                } else {
                    operators
                        .into_iter()
                        .fold(operand, |left_operand, operator| {
                            Box::new(expression::Expression::FunctionCallExpression(
                                function_call_expression::E {
                                    left_operand,
                                    operator,
                                },
                            ))
                        })
                }
            })
            .boxed();

        // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | 'delete' | '-' ) Expression ;
        let unary_prefix_expression_parser = leading_trivia_parser
            .clone()
            .then(choice::<_, ErrorType>((
                terminal("!").map(|_| 1usize),
                terminal("++").map(|_| 2usize),
                terminal("-").ignore_then(choice((
                    terminal("-").map(|_| 2usize),
                    empty().map(|_| 1usize),
                ))),
                terminal("delete").map(|_| 6usize),
                terminal("~").map(|_| 1usize),
            )))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                    leading,
                    content: VariableSizeTerminal(content),
                    trailing,
                },
            )
            .repeated()
            .then(function_call_expression_parser.clone())
            .map(|(mut operators, operand)| {
                if operators.is_empty() {
                    operand
                } else {
                    operators.reverse();
                    operators
                        .into_iter()
                        .fold(operand, |right_operand, operator| {
                            Box::new(expression::Expression::UnaryPrefixExpression(
                                unary_prefix_expression::E {
                                    operator,
                                    right_operand,
                                },
                            ))
                        })
                }
            })
            .boxed();

        // UnarySuffixExpression = Expression ( '++' | '--' ) ;
        let unary_suffix_expression_parser = unary_prefix_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        choice::<_, ErrorType>((
                            terminal("++").ignored(),
                            terminal("--").ignored(),
                        ))
                        .map(|_| FixedSizeTerminal::<2usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .repeated(),
            )
            .map(|(operand, operators)| {
                if operators.is_empty() {
                    operand
                } else {
                    operators
                        .into_iter()
                        .fold(operand, |left_operand, operator| {
                            Box::new(expression::Expression::UnarySuffixExpression(
                                unary_suffix_expression::E {
                                    left_operand,
                                    operator,
                                },
                            ))
                        })
                }
            })
            .boxed();

        // ExponentiationExpression = Expression '**' Expression ;
        let exponentiation_expression_parser = unary_suffix_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("**")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(unary_suffix_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    let mut last_operand = first_operand;
                    let mut operand_operator_pairs = vec![];
                    for (operator, right_operand) in operator_operand_pairs.into_iter() {
                        let left_operand = std::mem::replace(&mut last_operand, right_operand);
                        operand_operator_pairs.push((left_operand, operator))
                    }
                    operand_operator_pairs.into_iter().rfold(
                        last_operand,
                        |right_operand, (left_operand, operator)| {
                            Box::new(expression::Expression::ExponentiationExpression(
                                exponentiation_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
        let mul_div_mod_expression_parser = exponentiation_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        filter(|&c: &char| c == '*' || c == '/' || c == '%')
                            .map(|_| FixedSizeTerminal::<1>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(exponentiation_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::MulDivModExpression(
                                mul_div_mod_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // AddSubExpression = Expression ( '+' | '-' ) Expression ;
        let add_sub_expression_parser = mul_div_mod_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        filter(|&c: &char| c == '+' || c == '-').map(|_| FixedSizeTerminal::<1>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(mul_div_mod_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::AddSubExpression(
                                add_sub_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
        let shift_expression_parser = add_sub_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(choice::<_, ErrorType>((
                        terminal("<<").map(|_| 2usize),
                        terminal(">>").ignore_then(choice((
                            terminal(">").map(|_| 3usize),
                            empty().map(|_| 2usize),
                        ))),
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                            leading,
                            content: VariableSizeTerminal(content),
                            trailing,
                        },
                    )
                    .then(add_sub_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::ShiftExpression(
                                shift_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // BitAndExpression = Expression '&' Expression ;
        let bit_and_expression_parser = shift_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('&').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(shift_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::BitAndExpression(
                                bit_and_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // BitXOrExpression = Expression '^' Expression ;
        let bit_x_or_expression_parser = bit_and_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('^').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(bit_and_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::BitXOrExpression(
                                bit_x_or_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // BitOrExpression = Expression '|' Expression ;
        let bit_or_expression_parser = bit_x_or_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('|').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(bit_x_or_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::BitOrExpression(
                                bit_or_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
        let order_comparison_expression_parser = bit_or_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(choice::<_, ErrorType>((
                        terminal("<").ignore_then(choice((
                            terminal("=").map(|_| 2usize),
                            empty().map(|_| 1usize),
                        ))),
                        terminal(">").ignore_then(choice((
                            terminal("=").map(|_| 2usize),
                            empty().map(|_| 1usize),
                        ))),
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                            leading,
                            content: VariableSizeTerminal(content),
                            trailing,
                        },
                    )
                    .then(bit_or_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::OrderComparisonExpression(
                                order_comparison_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
        let equality_comparison_expression_parser = order_comparison_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        choice::<_, ErrorType>((
                            terminal("!=").ignored(),
                            terminal("==").ignored(),
                        ))
                        .map(|_| FixedSizeTerminal::<2usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(order_comparison_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::EqualityComparisonExpression(
                                equality_comparison_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // AndExpression = Expression '&&' Expression ;
        let and_expression_parser = equality_comparison_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("&&")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(equality_comparison_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::AndExpression(and_expression::E {
                                left_operand,
                                operator,
                                right_operand,
                            }))
                        },
                    )
                }
            })
            .boxed();

        // OrExpression = Expression '||' Expression ;
        let or_expression_parser = and_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("||")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<2usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(and_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::OrExpression(or_expression::E {
                                left_operand,
                                operator,
                                right_operand,
                            }))
                        },
                    )
                }
            })
            .boxed();

        // ConditionalExpression = Expression '?' Expression ':' Expression ;
        let conditional_expression_parser = or_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('?').map(|_| FixedSizeTerminal::<1>()))
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
                            .then(just(':').map(|_| FixedSizeTerminal::<1>()))
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
                    .map(|v| conditional_expression::_T1::from_parse(v))
                    .repeated(),
            )
            .map(|(operand, operators)| {
                if operators.is_empty() {
                    operand
                } else {
                    operators
                        .into_iter()
                        .fold(operand, |left_operand, operator| {
                            Box::new(expression::Expression::ConditionalExpression(
                                conditional_expression::E {
                                    left_operand,
                                    operator,
                                },
                            ))
                        })
                }
            })
            .boxed();

        // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
        let assignment_expression_parser = conditional_expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(choice::<_, ErrorType>((
                        terminal("%=").map(|_| 2usize),
                        terminal("&=").map(|_| 2usize),
                        terminal("*=").map(|_| 2usize),
                        terminal("+=").map(|_| 2usize),
                        terminal("-=").map(|_| 2usize),
                        terminal("/=").map(|_| 2usize),
                        terminal("<<=").map(|_| 3usize),
                        terminal("=").map(|_| 1usize),
                        terminal(">>").ignore_then(choice((
                            terminal("=").map(|_| 3usize),
                            terminal(">=").map(|_| 4usize),
                        ))),
                        terminal("^=").map(|_| 2usize),
                        terminal("|=").map(|_| 2usize),
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                            leading,
                            content: VariableSizeTerminal(content),
                            trailing,
                        },
                    )
                    .then(conditional_expression_parser.clone())
                    .repeated(),
            )
            .map(|(first_operand, operator_operand_pairs)| {
                if operator_operand_pairs.is_empty() {
                    first_operand
                } else {
                    operator_operand_pairs.into_iter().fold(
                        first_operand,
                        |left_operand, (operator, right_operand)| {
                            Box::new(expression::Expression::AssignmentExpression(
                                assignment_expression::E {
                                    left_operand,
                                    operator,
                                    right_operand,
                                },
                            ))
                        },
                    )
                }
            })
            .boxed();

        // Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | FunctionCallOptionsExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
        expression_parser.define(assignment_expression_parser.clone().boxed());

        // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
        let constant_definition_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("constant")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<8usize>()),
                    )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
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
            .map(|v| constant_definition::_T0::from_parse(v))
            .boxed();

        // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
        let do_while_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("do")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<2usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(statement_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("while")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<5usize>()),
                    )
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
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
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
            .map(|v| do_while_statement::_T0::from_parse(v))
            .boxed();

        // EmitStatement = 'emit' Expression ArgumentList ';' ;
        let emit_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("emit")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<4usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(expression_parser.clone())
            .then(argument_list_parser.clone())
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
            .map(|v| emit_statement::_T0::from_parse(v))
            .boxed();

        // ExpressionStatement = Expression ';' ;
        let expression_statement_parser = expression_parser
            .clone()
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
            .map(|v| expression_statement::_T0::from_parse(v))
            .boxed();

        // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
        let if_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("if")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<2usize>()),
            )
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
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
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
            .then(statement_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("else")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<4usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(statement_parser.clone())
                    .map(|v| if_statement::_T1::from_parse(v))
                    .or_not(),
            )
            .map(|v| if_statement::_T0::from_parse(v))
            .boxed();

        // ReturnStatement = 'return' [ Expression ] ';' ;
        let return_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("return")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<6usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(expression_parser.clone().or_not())
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
            .map(|v| return_statement::_T0::from_parse(v))
            .boxed();

        // RevertStatement = 'revert' Expression ArgumentList ';' ;
        let revert_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("revert")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<6usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(expression_parser.clone())
            .then(argument_list_parser.clone())
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
            .map(|v| revert_statement::_T0::from_parse(v))
            .boxed();

        // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
        let state_variable_declaration_parser = type_name_parser
            .clone()
            .then(state_variable_attribute_parser.clone().repeated())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
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
                    )
                    .then(expression_parser.clone())
                    .map(|v| state_variable_declaration::_T2::from_parse(v))
                    .or_not(),
            )
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
            .map(|v| state_variable_declaration::_T0::from_parse(v))
            .boxed();

        // TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
        let try_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("try")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<3usize>()),
            )
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
                    .then(
                        terminal("returns")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<7usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(parameter_list_parser.clone())
                    .map(|v| try_statement::_T1::from_parse(v))
                    .or_not(),
            )
            .then(block_parser.clone())
            .then(catch_clause_parser.clone().repeated().at_least(1usize))
            .map(|v| try_statement::_T0::from_parse(v))
            .boxed();

        // TupleDeconstructionStatement = '(' { [ [ TypeName ] «Identifier» ] / ',' } ')' '=' Expression ';' ;
        let tuple_deconstruction_statement_parser = leading_trivia_parser
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
            .then(
                type_name_parser
                    .clone()
                    .or_not()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading, content), trailing)| identifier::WithTrivia {
                                leading,
                                content,
                                trailing,
                            }),
                    )
                    .map(|v| tuple_deconstruction_statement::_T2::from_parse(v))
                    .or_not()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').map(|_| FixedSizeTerminal::<1>()))
                            .then(trailing_trivia_parser.clone())
                            .map(
                                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                                    leading,
                                    content,
                                    trailing,
                                },
                            )
                            .then(
                                type_name_parser
                                    .clone()
                                    .or_not()
                                    .then(
                                        leading_trivia_parser
                                            .clone()
                                            .then(identifier_parser.clone())
                                            .then(trailing_trivia_parser.clone())
                                            .map(|((leading, content), trailing)| {
                                                identifier::WithTrivia {
                                                    leading,
                                                    content,
                                                    trailing,
                                                }
                                            }),
                                    )
                                    .map(|v| tuple_deconstruction_statement::_T2::from_parse(v))
                                    .or_not(),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(
                        |(elements, separators)| tuple_deconstruction_statement::_T1 {
                            elements,
                            separators,
                        },
                    )
                    .or_not(),
            )
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
            .map(|v| tuple_deconstruction_statement::_T0::from_parse(v))
            .boxed();

        // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
        let variable_declaration_statement_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(choice::<_, ErrorType>((
                        terminal("calldata").map(|_| 8usize),
                        terminal("memory").map(|_| 6usize),
                        terminal("storage").map(|_| 7usize),
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                            leading,
                            content: VariableSizeTerminal(content),
                            trailing,
                        },
                    )
                    .or_not(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
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
                    )
                    .then(expression_parser.clone())
                    .map(|v| variable_declaration_statement::_T1::from_parse(v))
                    .or_not(),
            )
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
            .map(|v| variable_declaration_statement::_T0::from_parse(v))
            .boxed();

        // WhileStatement = 'while' '(' Expression ')' Statement ;
        let while_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("while")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<5usize>()),
            )
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
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
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
            .then(statement_parser.clone())
            .map(|v| while_statement::_T0::from_parse(v))
            .boxed();

        // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
        let simple_statement_parser = choice((
            tuple_deconstruction_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::_T0::TupleDeconstructionStatement(v))),
            variable_declaration_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::_T0::VariableDeclarationStatement(v))),
            expression_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::_T0::ExpressionStatement(v))),
        ))
        .boxed();

        // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
        let for_statement_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("for")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<3usize>()),
            )
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
                    .then(just('(').map(|_| FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(choice((
                simple_statement_parser
                    .clone()
                    .map(|v| Box::new(for_statement::_T1::SimpleStatement(v))),
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
                    )
                    .map(|v| Box::new(for_statement::_T1::SemicolonChar(v))),
            )))
            .then(choice((
                expression_statement_parser
                    .clone()
                    .map(|v| Box::new(for_statement::_T2::ExpressionStatement(v))),
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
                    )
                    .map(|v| Box::new(for_statement::_T2::SemicolonChar(v))),
            )))
            .then(expression_parser.clone().or_not())
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
            .then(statement_parser.clone())
            .map(|v| for_statement::_T0::from_parse(v))
            .boxed();

        // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | AssemblyStatement ;
        statement_parser.define(
            choice((
                block_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::Block(v))),
                simple_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::SimpleStatement(v))),
                if_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::IfStatement(v))),
                for_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::ForStatement(v))),
                while_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::WhileStatement(v))),
                do_while_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::DoWhileStatement(v))),
                continue_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::ContinueStatement(v))),
                break_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::BreakStatement(v))),
                try_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::TryStatement(v))),
                return_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::ReturnStatement(v))),
                emit_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::EmitStatement(v))),
                revert_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::RevertStatement(v))),
                assembly_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_T0::AssemblyStatement(v))),
            ))
            .boxed(),
        );

        // Block = '{' { Statement | UncheckedBlock } '}' ;
        block_parser.define(
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
                )
                .then(
                    choice((
                        statement_parser
                            .clone()
                            .map(|v| Box::new(block::_T2::Statement(v))),
                        unchecked_block_parser
                            .clone()
                            .map(|v| Box::new(block::_T2::UncheckedBlock(v))),
                    ))
                    .repeated(),
                )
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
                .map(|v| block::_T0::from_parse(v))
                .boxed(),
        );

        // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
        let constructor_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("constructor")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<11usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(parameter_list_parser.clone())
            .then(constructor_attribute_parser.clone().repeated())
            .then(block_parser.clone())
            .map(|v| constructor_definition::_T0::from_parse(v))
            .boxed();

        // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
        let fallback_function_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("fallback")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(parameter_list_parser.clone())
            .then(fallback_function_attribute_parser.clone().repeated())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("returns")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<7usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(parameter_list_parser.clone())
                    .map(|v| fallback_function_definition::_T2::from_parse(v))
                    .or_not(),
            )
            .then(choice((
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
                    )
                    .map(|v| Box::new(fallback_function_definition::_T3::SemicolonChar(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(fallback_function_definition::_T3::Block(v))),
            )))
            .map(|v| fallback_function_definition::_T0::from_parse(v))
            .boxed();

        // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
        let function_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("function")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    })
                    .map(|v| Box::new(function_definition::_T1::Identifier(v))),
                leading_trivia_parser
                    .clone()
                    .then(choice::<_, ErrorType>((
                        terminal("fallback").map(|_| 8usize),
                        terminal("receive").map(|_| 7usize),
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| VariableSizeTerminalWithTrivia {
                            leading,
                            content: VariableSizeTerminal(content),
                            trailing,
                        },
                    )
                    .map(|v| Box::new(function_definition::_T1::_1(v))),
            )))
            .then(parameter_list_parser.clone())
            .then(function_attribute_parser.clone().repeated())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("returns")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<7usize>()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    )
                    .then(parameter_list_parser.clone())
                    .map(|v| function_definition::_T3::from_parse(v))
                    .or_not(),
            )
            .then(choice((
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
                    )
                    .map(|v| Box::new(function_definition::_T4::SemicolonChar(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(function_definition::_T4::Block(v))),
            )))
            .map(|v| function_definition::_T0::from_parse(v))
            .boxed();

        // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
        let modifier_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("modifier")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
            .then(parameter_list_parser.clone().or_not())
            .then(modifier_attribute_parser.clone().repeated())
            .then(choice((
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
                    )
                    .map(|v| Box::new(modifier_definition::_T2::SemicolonChar(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(modifier_definition::_T2::Block(v))),
            )))
            .map(|v| modifier_definition::_T0::from_parse(v))
            .boxed();

        // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
        let receive_function_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("receive")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<7usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .then(parameter_list_parser.clone())
            .then(receive_function_attribute_parser.clone().repeated())
            .then(choice((
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
                    )
                    .map(|v| Box::new(receive_function_definition::_T2::SemicolonChar(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(receive_function_definition::_T2::Block(v))),
            )))
            .map(|v| receive_function_definition::_T0::from_parse(v))
            .boxed();

        // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
        let contract_body_element_parser = choice((
            using_directive_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::UsingDirective(v))),
            constructor_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::ConstructorDefinition(v))),
            function_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::FunctionDefinition(v))),
            fallback_function_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::FallbackFunctionDefinition(v))),
            receive_function_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::ReceiveFunctionDefinition(v))),
            modifier_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::ModifierDefinition(v))),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::StructDefinition(v))),
            enum_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::EnumDefinition(v))),
            user_defined_value_type_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::_T0::UserDefinedValueTypeDefinition(
                    v,
                ))
            }),
            event_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::EventDefinition(v))),
            error_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::ErrorDefinition(v))),
            state_variable_declaration_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::StateVariableDeclaration(v))),
        ))
        .boxed();

        // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let contract_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("abstract")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<8usize>()),
            )
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading, content), trailing)| FixedSizeTerminalWithTrivia {
                    leading,
                    content,
                    trailing,
                },
            )
            .or_not()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("contract")
                            .ignored()
                            .map(|_| FixedSizeTerminal::<8usize>()),
                    )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
            .then(inheritance_specifier_list_parser.clone().or_not())
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
            .then(contract_body_element_parser.clone().repeated())
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
            .map(|v| contract_definition::_T0::from_parse(v))
            .boxed();

        // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let interface_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("interface")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<9usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
            .then(inheritance_specifier_list_parser.clone().or_not())
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
            .then(contract_body_element_parser.clone().repeated())
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
            .map(|v| interface_definition::_T0::from_parse(v))
            .boxed();

        // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
        let library_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("library")
                    .ignored()
                    .map(|_| FixedSizeTerminal::<7usize>()),
            )
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
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| identifier::WithTrivia {
                        leading,
                        content,
                        trailing,
                    }),
            )
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
            .then(contract_body_element_parser.clone().repeated())
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
            .map(|v| library_definition::_T0::from_parse(v))
            .boxed();

        // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
        let definition_parser = choice((
            contract_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::ContractDefinition(v))),
            interface_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::InterfaceDefinition(v))),
            library_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::LibraryDefinition(v))),
            function_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::FunctionDefinition(v))),
            constant_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::ConstantDefinition(v))),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::StructDefinition(v))),
            enum_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::EnumDefinition(v))),
            user_defined_value_type_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::UserDefinedValueTypeDefinition(v))),
            error_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::ErrorDefinition(v))),
        ))
        .boxed();

        // SourceUnit = «LeadingTrivia» { Directive | Definition } «EndOfFileTrivia» $ ;
        let source_unit_parser = leading_trivia_parser
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
            .then(
                choice((
                    directive_parser
                        .clone()
                        .map(|v| Box::new(source_unit::_T2::Directive(v))),
                    definition_parser
                        .clone()
                        .map(|v| Box::new(source_unit::_T2::Definition(v))),
                ))
                .repeated(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(end_of_file_trivia_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading, content), trailing)| end_of_file_trivia::WithTrivia {
                            leading,
                            content,
                            trailing,
                        },
                    ),
            )
            .then(end())
            .map(|v| source_unit::_T0::from_parse(v))
            .boxed();

        Self {
            decimal_integer: decimal_integer_parser,
            end_of_line: end_of_line_parser,
            fixed_bytes_type: fixed_bytes_type_parser,
            fixed_type: fixed_type_parser,
            hex_byte_escape: hex_byte_escape_parser,
            hex_number: hex_number_parser,
            multiline_comment: multiline_comment_parser,
            possibly_separated_pairs_of_hex_digits: possibly_separated_pairs_of_hex_digits_parser,
            raw_identifier: raw_identifier_parser,
            signed_integer_type: signed_integer_type_parser,
            single_line_comment: single_line_comment_parser,
            unicode_escape: unicode_escape_parser,
            version_pragma_value: version_pragma_value_parser,
            whitespace: whitespace_parser,
            yul_decimal_number_literal: yul_decimal_number_literal_parser,
            yul_hex_literal: yul_hex_literal_parser,
            decimal_exponent: decimal_exponent_parser,
            decimal_float: decimal_float_parser,
            end_of_file_trivia: end_of_file_trivia_parser,
            escape_sequence: escape_sequence_parser,
            hex_string_literal: hex_string_literal_parser,
            leading_trivia: leading_trivia_parser,
            trailing_trivia: trailing_trivia_parser,
            ufixed_type: ufixed_type_parser,
            unsigned_integer_type: unsigned_integer_type_parser,
            yul_identifier: yul_identifier_parser,
            address_type: address_type_parser,
            array_literal: array_literal_parser,
            break_statement: break_statement_parser,
            continue_statement: continue_statement_parser,
            decimal_number: decimal_number_parser,
            double_quoted_ascii_string_literal: double_quoted_ascii_string_literal_parser,
            double_quoted_unicode_string_literal: double_quoted_unicode_string_literal_parser,
            keyword: keyword_parser,
            parenthesis_expression: parenthesis_expression_parser,
            positional_argument_list: positional_argument_list_parser,
            single_quoted_ascii_string_literal: single_quoted_ascii_string_literal_parser,
            single_quoted_unicode_string_literal: single_quoted_unicode_string_literal_parser,
            unchecked_block: unchecked_block_parser,
            version_pragma_specifier: version_pragma_specifier_parser,
            yul_function_call: yul_function_call_parser,
            yul_function_definition: yul_function_definition_parser,
            yul_path: yul_path_parser,
            ascii_string_literal: ascii_string_literal_parser,
            assembly_flags: assembly_flags_parser,
            elementary_type: elementary_type_parser,
            numeric_literal: numeric_literal_parser,
            reserved_word: reserved_word_parser,
            unicode_string_literal: unicode_string_literal_parser,
            identifier: identifier_parser,
            import_path: import_path_parser,
            yul_literal: yul_literal_parser,
            abi_coder_pragma_specifier: abi_coder_pragma_specifier_parser,
            enum_definition: enum_definition_parser,
            experimental_pragma_specifier: experimental_pragma_specifier_parser,
            identifier_path: identifier_path_parser,
            named_argument: named_argument_parser,
            parameter_declaration: parameter_declaration_parser,
            selected_import: selected_import_parser,
            simple_import_directive: simple_import_directive_parser,
            star_import_directive: star_import_directive_parser,
            user_defined_value_type_definition: user_defined_value_type_definition_parser,
            yul_expression: yul_expression_parser.boxed(),
            mapping_type: mapping_type_parser,
            named_argument_list: named_argument_list_parser,
            override_specifier: override_specifier_parser,
            parameter_list: parameter_list_parser,
            pragma_directive: pragma_directive_parser,
            selecting_import_directive: selecting_import_directive_parser,
            yul_assignment: yul_assignment_parser,
            yul_for_statement: yul_for_statement_parser,
            yul_if_statement: yul_if_statement_parser,
            yul_switch_statement: yul_switch_statement_parser,
            yul_variable_declaration: yul_variable_declaration_parser,
            argument_list: argument_list_parser,
            catch_clause: catch_clause_parser,
            function_type: function_type_parser,
            import_directive: import_directive_parser,
            modifier_attribute: modifier_attribute_parser,
            state_variable_attribute: state_variable_attribute_parser,
            yul_statement: yul_statement_parser,
            inheritance_specifier: inheritance_specifier_parser,
            modifier_invocation: modifier_invocation_parser,
            new_expression: new_expression_parser,
            payable_expression: payable_expression_parser,
            type_name: type_name_parser.boxed(),
            yul_block: yul_block_parser.boxed(),
            assembly_statement: assembly_statement_parser,
            constructor_attribute: constructor_attribute_parser,
            error_parameter: error_parameter_parser,
            event_parameter: event_parameter_parser,
            fallback_function_attribute: fallback_function_attribute_parser,
            function_attribute: function_attribute_parser,
            inheritance_specifier_list: inheritance_specifier_list_parser,
            receive_function_attribute: receive_function_attribute_parser,
            struct_member: struct_member_parser,
            type_expression: type_expression_parser,
            using_directive: using_directive_parser,
            directive: directive_parser,
            error_definition: error_definition_parser,
            event_definition: event_definition_parser,
            primary_expression: primary_expression_parser,
            struct_definition: struct_definition_parser,
            index_access_expression: index_access_expression_parser,
            member_access_expression: member_access_expression_parser,
            function_call_options_expression: function_call_options_expression_parser,
            function_call_expression: function_call_expression_parser,
            unary_prefix_expression: unary_prefix_expression_parser,
            unary_suffix_expression: unary_suffix_expression_parser,
            exponentiation_expression: exponentiation_expression_parser,
            mul_div_mod_expression: mul_div_mod_expression_parser,
            add_sub_expression: add_sub_expression_parser,
            shift_expression: shift_expression_parser,
            bit_and_expression: bit_and_expression_parser,
            bit_x_or_expression: bit_x_or_expression_parser,
            bit_or_expression: bit_or_expression_parser,
            order_comparison_expression: order_comparison_expression_parser,
            equality_comparison_expression: equality_comparison_expression_parser,
            and_expression: and_expression_parser,
            or_expression: or_expression_parser,
            conditional_expression: conditional_expression_parser,
            assignment_expression: assignment_expression_parser,
            expression: expression_parser.boxed(),
            constant_definition: constant_definition_parser,
            do_while_statement: do_while_statement_parser,
            emit_statement: emit_statement_parser,
            expression_statement: expression_statement_parser,
            if_statement: if_statement_parser,
            return_statement: return_statement_parser,
            revert_statement: revert_statement_parser,
            state_variable_declaration: state_variable_declaration_parser,
            try_statement: try_statement_parser,
            tuple_deconstruction_statement: tuple_deconstruction_statement_parser,
            variable_declaration_statement: variable_declaration_statement_parser,
            while_statement: while_statement_parser,
            simple_statement: simple_statement_parser,
            for_statement: for_statement_parser,
            statement: statement_parser.boxed(),
            block: block_parser.boxed(),
            constructor_definition: constructor_definition_parser,
            fallback_function_definition: fallback_function_definition_parser,
            function_definition: function_definition_parser,
            modifier_definition: modifier_definition_parser,
            receive_function_definition: receive_function_definition_parser,
            contract_body_element: contract_body_element_parser,
            contract_definition: contract_definition_parser,
            interface_definition: interface_definition_parser,
            library_definition: library_definition_parser,
            definition: definition_parser,
            source_unit: source_unit_parser,
        }
    }
}
