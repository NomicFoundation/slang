// This file is generated via `cargo build`. Please don't edit by hand.

use super::ast::*;
use super::ast_parser::*;
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

        // «BooleanLiteral» = 'true' | 'false' ;
        let boolean_literal_parser = choice((
            terminal("true")
                .to(FixedSizeTerminal::<4usize>())
                .map(|v| Box::new(boolean_literal::BooleanLiteral::True(v))),
            terminal("false")
                .to(FixedSizeTerminal::<5usize>())
                .map(|v| Box::new(boolean_literal::BooleanLiteral::False(v))),
        ))
        .boxed();

        // «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
        let decimal_integer_parser = filter(|&c: &char| ('0' <= c && c <= '9'))
            .to(FixedSizeTerminal::<1>())
            .then(
                just('_')
                    .to(FixedSizeTerminal::<1>())
                    .or_not()
                    .then(filter(|&c: &char| ('0' <= c && c <= '9')).to(FixedSizeTerminal::<1>()))
                    .map(|v| decimal_integer::Sequence1::from_parse(v))
                    .repeated(),
            )
            .map(|v| decimal_integer::DecimalInteger::from_parse(v))
            .boxed();

        // «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
        let end_of_line_parser = filter(|&c: &char| c == '\r' || c == '\n')
            .to(FixedSizeTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| VariableSizeTerminal(v.len()))
            .boxed();

        // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
        let fixed_bytes_type_parser = terminal("bytes")
            .to(FixedSizeTerminal::<5usize>())
            .then(choice::<_, ErrorType>((
                terminal("1").ignore_then(choice((
                    terminal("0").to(VariableSizeTerminal(2usize)),
                    terminal("1").to(VariableSizeTerminal(2usize)),
                    terminal("2").to(VariableSizeTerminal(2usize)),
                    terminal("3").to(VariableSizeTerminal(2usize)),
                    terminal("4").to(VariableSizeTerminal(2usize)),
                    terminal("5").to(VariableSizeTerminal(2usize)),
                    terminal("6").to(VariableSizeTerminal(2usize)),
                    terminal("7").to(VariableSizeTerminal(2usize)),
                    terminal("8").to(VariableSizeTerminal(2usize)),
                    terminal("9").to(VariableSizeTerminal(2usize)),
                    empty().to(VariableSizeTerminal(1usize)),
                ))),
                terminal("2").ignore_then(choice((
                    terminal("0").to(VariableSizeTerminal(2usize)),
                    terminal("1").to(VariableSizeTerminal(2usize)),
                    terminal("2").to(VariableSizeTerminal(2usize)),
                    terminal("3").to(VariableSizeTerminal(2usize)),
                    terminal("4").to(VariableSizeTerminal(2usize)),
                    terminal("5").to(VariableSizeTerminal(2usize)),
                    terminal("6").to(VariableSizeTerminal(2usize)),
                    terminal("7").to(VariableSizeTerminal(2usize)),
                    terminal("8").to(VariableSizeTerminal(2usize)),
                    terminal("9").to(VariableSizeTerminal(2usize)),
                    empty().to(VariableSizeTerminal(1usize)),
                ))),
                terminal("3").ignore_then(choice((
                    terminal("0").to(VariableSizeTerminal(2usize)),
                    terminal("1").to(VariableSizeTerminal(2usize)),
                    terminal("2").to(VariableSizeTerminal(2usize)),
                    empty().to(VariableSizeTerminal(1usize)),
                ))),
                terminal("4").to(VariableSizeTerminal(1usize)),
                terminal("5").to(VariableSizeTerminal(1usize)),
                terminal("6").to(VariableSizeTerminal(1usize)),
                terminal("7").to(VariableSizeTerminal(1usize)),
                terminal("8").to(VariableSizeTerminal(1usize)),
                terminal("9").to(VariableSizeTerminal(1usize)),
            )))
            .map(|v| fixed_bytes_type::FixedBytesType::from_parse(v))
            .boxed();

        // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
        let hex_byte_escape_parser = just('x')
            .to(FixedSizeTerminal::<1>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .to(FixedSizeTerminal::<1>())
                .repeated()
                .exactly(2usize)
                .map(|v| VariableSizeTerminal(v.len())),
            )
            .map(|v| hex_byte_escape::HexByteEscape::from_parse(v))
            .boxed();

        // «HexNumber» = '0x' «HexCharacter» { [ '_' ] «HexCharacter» } ;
        let hex_number_parser = terminal("0x")
            .to(FixedSizeTerminal::<2usize>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .to(FixedSizeTerminal::<1>())
                .then(
                    just('_')
                        .to(FixedSizeTerminal::<1>())
                        .or_not()
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')
                            })
                            .to(FixedSizeTerminal::<1>()),
                        )
                        .map(|v| hex_number::Sequence2::from_parse(v))
                        .repeated(),
                )
                .map(|v| hex_number::Sequence0::from_parse(v)),
            )
            .map(|v| hex_number::HexNumber::from_parse(v))
            .boxed();

        // «MultilineComment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
        let multiline_comment_parser = just("/*")
            .to(FixedSizeTerminal::<2usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '*')
                        .to(FixedSizeTerminal::<1>())
                        .map(|v| Box::new(multiline_comment::Choices0::NotStar(v))),
                    just('*')
                        .to(FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .then(filter(|&c: &char| c != '*' && c != '/').to(FixedSizeTerminal::<1>()))
                        .map(|v| multiline_comment::Sequence1::from_parse(v))
                        .map(|v| Box::new(multiline_comment::Choices0::Sequence1(v))),
                ))
                .repeated()
                .then(
                    just('*')
                        .to(FixedSizeTerminal::<1>())
                        .repeated()
                        .map(|v| VariableSizeTerminal(v.len())),
                )
                .map(|v| multiline_comment::Content::from_parse(v)),
            )
            .then(just("*/").to(FixedSizeTerminal::<2usize>()))
            .map(|v| multiline_comment::MultilineComment::from_parse(v))
            .boxed();

        // «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
        let number_unit_parser = choice((
            terminal("days")
                .to(FixedSizeTerminal::<4usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Days(v))),
            terminal("ether")
                .to(FixedSizeTerminal::<5usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Ether(v))),
            terminal("finney")
                .to(FixedSizeTerminal::<6usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Finney(v))),
            terminal("gwei")
                .to(FixedSizeTerminal::<4usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Gwei(v))),
            terminal("hours")
                .to(FixedSizeTerminal::<5usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Hours(v))),
            terminal("minutes")
                .to(FixedSizeTerminal::<7usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Minutes(v))),
            terminal("seconds")
                .to(FixedSizeTerminal::<7usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Seconds(v))),
            terminal("szabo")
                .to(FixedSizeTerminal::<5usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Szabo(v))),
            terminal("weeks")
                .to(FixedSizeTerminal::<5usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Weeks(v))),
            terminal("wei")
                .to(FixedSizeTerminal::<3usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Wei(v))),
            terminal("years")
                .to(FixedSizeTerminal::<5usize>())
                .map(|v| Box::new(number_unit::NumberUnit::Years(v))),
        ))
        .boxed();

        // «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
        let possibly_separated_pairs_of_hex_digits_parser = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        })
        .to(FixedSizeTerminal::<1>())
        .repeated()
        .exactly(2usize)
        .map(|v| VariableSizeTerminal(v.len()))
        .then(
            just('_')
                .to(FixedSizeTerminal::<1>())
                .or_not()
                .then(
                    filter(|&c: &char| {
                        ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                    })
                    .to(FixedSizeTerminal::<1>())
                    .repeated()
                    .exactly(2usize)
                    .map(|v| VariableSizeTerminal(v.len())),
                )
                .map(|v| possibly_separated_pairs_of_hex_digits::Sequence1::from_parse(v))
                .repeated(),
        )
        .map(|v| {
            possibly_separated_pairs_of_hex_digits::PossiblySeparatedPairsOfHexDigits::from_parse(v)
        })
        .boxed();

        // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
        let raw_identifier_parser = filter(|&c: &char| {
            c == '_' || c == '$' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
        })
        .to(FixedSizeTerminal::<1>())
        .then(
            filter(|&c: &char| {
                c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z')
                    || ('0' <= c && c <= '9')
            })
            .to(FixedSizeTerminal::<1>())
            .repeated()
            .map(|v| VariableSizeTerminal(v.len())),
        )
        .map(|v| raw_identifier::RawIdentifier::from_parse(v))
        .boxed();

        // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
        let reserved_keyword_parser = choice::<_, ErrorType>((
            terminal("a").ignore_then(choice((
                terminal("fter").to(VariableSizeTerminal(5usize)),
                terminal("lias").to(VariableSizeTerminal(5usize)),
                terminal("pply").to(VariableSizeTerminal(5usize)),
                terminal("uto").to(VariableSizeTerminal(4usize)),
            ))),
            terminal("byte").to(VariableSizeTerminal(4usize)),
            terminal("c").ignore_then(choice((
                terminal("ase").to(VariableSizeTerminal(4usize)),
                terminal("opyof").to(VariableSizeTerminal(6usize)),
            ))),
            terminal("def").ignore_then(choice((
                terminal("ault").to(VariableSizeTerminal(7usize)),
                terminal("ine").to(VariableSizeTerminal(6usize)),
            ))),
            terminal("final").to(VariableSizeTerminal(5usize)),
            terminal("i").ignore_then(choice((
                terminal("mplements").to(VariableSizeTerminal(10usize)),
                terminal("n").ignore_then(choice((
                    terminal("line").to(VariableSizeTerminal(6usize)),
                    empty().to(VariableSizeTerminal(2usize)),
                ))),
            ))),
            terminal("let").to(VariableSizeTerminal(3usize)),
            terminal("m").ignore_then(choice((
                terminal("a").ignore_then(choice((
                    terminal("cro").to(VariableSizeTerminal(5usize)),
                    terminal("tch").to(VariableSizeTerminal(5usize)),
                ))),
                terminal("utable").to(VariableSizeTerminal(7usize)),
            ))),
            terminal("null").to(VariableSizeTerminal(4usize)),
            terminal("of").to(VariableSizeTerminal(2usize)),
            terminal("p").ignore_then(choice((
                terminal("artial").to(VariableSizeTerminal(7usize)),
                terminal("romise").to(VariableSizeTerminal(7usize)),
            ))),
            terminal("re").ignore_then(choice((
                terminal("ference").to(VariableSizeTerminal(9usize)),
                terminal("locatable").to(VariableSizeTerminal(11usize)),
            ))),
            terminal("s").ignore_then(choice((
                terminal("ealed").to(VariableSizeTerminal(6usize)),
                terminal("izeof").to(VariableSizeTerminal(6usize)),
                terminal("tatic").to(VariableSizeTerminal(6usize)),
                terminal("upports").to(VariableSizeTerminal(8usize)),
                terminal("witch").to(VariableSizeTerminal(6usize)),
            ))),
            terminal("type").ignore_then(choice((
                terminal("def").to(VariableSizeTerminal(7usize)),
                terminal("of").to(VariableSizeTerminal(6usize)),
            ))),
            terminal("var").to(VariableSizeTerminal(3usize)),
        ))
        .boxed();

        // «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
        let signed_fixed_type_parser = terminal("fixed")
            .to(FixedSizeTerminal::<5usize>())
            .then(
                filter(|&c: &char| ('0' <= c && c <= '9'))
                    .to(FixedSizeTerminal::<1>())
                    .repeated()
                    .at_least(1usize)
                    .map(|v| VariableSizeTerminal(v.len()))
                    .then(just('x').to(FixedSizeTerminal::<1>()))
                    .then(
                        filter(|&c: &char| ('0' <= c && c <= '9'))
                            .to(FixedSizeTerminal::<1>())
                            .repeated()
                            .at_least(1usize)
                            .map(|v| VariableSizeTerminal(v.len())),
                    )
                    .map(|v| signed_fixed_type::Sequence0::from_parse(v))
                    .or_not(),
            )
            .map(|v| signed_fixed_type::SignedFixedType::from_parse(v))
            .boxed();

        // «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
        let signed_integer_type_parser = terminal("int")
            .to(FixedSizeTerminal::<3usize>())
            .then(
                choice::<_, ErrorType>((
                    terminal("1").ignore_then(choice((
                        terminal("04").to(VariableSizeTerminal(3usize)),
                        terminal("12").to(VariableSizeTerminal(3usize)),
                        terminal("2").ignore_then(choice((
                            terminal("0").to(VariableSizeTerminal(3usize)),
                            terminal("8").to(VariableSizeTerminal(3usize)),
                        ))),
                        terminal("36").to(VariableSizeTerminal(3usize)),
                        terminal("44").to(VariableSizeTerminal(3usize)),
                        terminal("52").to(VariableSizeTerminal(3usize)),
                        terminal("6").ignore_then(choice((
                            terminal("0").to(VariableSizeTerminal(3usize)),
                            terminal("8").to(VariableSizeTerminal(3usize)),
                            empty().to(VariableSizeTerminal(2usize)),
                        ))),
                        terminal("76").to(VariableSizeTerminal(3usize)),
                        terminal("84").to(VariableSizeTerminal(3usize)),
                        terminal("92").to(VariableSizeTerminal(3usize)),
                    ))),
                    terminal("2").ignore_then(choice((
                        terminal("0").ignore_then(choice((
                            terminal("0").to(VariableSizeTerminal(3usize)),
                            terminal("8").to(VariableSizeTerminal(3usize)),
                        ))),
                        terminal("16").to(VariableSizeTerminal(3usize)),
                        terminal("24").to(VariableSizeTerminal(3usize)),
                        terminal("32").to(VariableSizeTerminal(3usize)),
                        terminal("4").ignore_then(choice((
                            terminal("0").to(VariableSizeTerminal(3usize)),
                            terminal("8").to(VariableSizeTerminal(3usize)),
                            empty().to(VariableSizeTerminal(2usize)),
                        ))),
                        terminal("56").to(VariableSizeTerminal(3usize)),
                    ))),
                    terminal("32").to(VariableSizeTerminal(2usize)),
                    terminal("4").ignore_then(choice((
                        terminal("0").to(VariableSizeTerminal(2usize)),
                        terminal("8").to(VariableSizeTerminal(2usize)),
                    ))),
                    terminal("56").to(VariableSizeTerminal(2usize)),
                    terminal("64").to(VariableSizeTerminal(2usize)),
                    terminal("72").to(VariableSizeTerminal(2usize)),
                    terminal("8").ignore_then(choice((
                        terminal("0").to(VariableSizeTerminal(2usize)),
                        terminal("8").to(VariableSizeTerminal(2usize)),
                        empty().to(VariableSizeTerminal(1usize)),
                    ))),
                    terminal("96").to(VariableSizeTerminal(2usize)),
                ))
                .or_not(),
            )
            .map(|v| signed_integer_type::SignedIntegerType::from_parse(v))
            .boxed();

        // «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
        let single_line_comment_parser = terminal("//")
            .to(FixedSizeTerminal::<2usize>())
            .then(
                filter(|&c: &char| c != '\r' && c != '\n')
                    .to(FixedSizeTerminal::<1>())
                    .repeated()
                    .map(|v| VariableSizeTerminal(v.len())),
            )
            .map(|v| single_line_comment::SingleLineComment::from_parse(v))
            .boxed();

        // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
        let unicode_escape_parser = just('u')
            .to(FixedSizeTerminal::<1>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .to(FixedSizeTerminal::<1>())
                .repeated()
                .exactly(4usize)
                .map(|v| VariableSizeTerminal(v.len())),
            )
            .map(|v| unicode_escape::UnicodeEscape::from_parse(v))
            .boxed();

        // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
        let version_pragma_operator_parser = choice((
            just('^')
                .to(FixedSizeTerminal::<1>())
                .map(|v| Box::new(version_pragma_operator::VersionPragmaOperator::Caret(v))),
            just('~')
                .to(FixedSizeTerminal::<1>())
                .map(|v| Box::new(version_pragma_operator::VersionPragmaOperator::Tilde(v))),
            just('=')
                .to(FixedSizeTerminal::<1>())
                .map(|v| Box::new(version_pragma_operator::VersionPragmaOperator::Equal(v))),
            just('<')
                .to(FixedSizeTerminal::<1>())
                .map(|v| Box::new(version_pragma_operator::VersionPragmaOperator::Less(v))),
            just('>')
                .to(FixedSizeTerminal::<1>())
                .map(|v| Box::new(version_pragma_operator::VersionPragmaOperator::Greater(v))),
            terminal("<=")
                .to(FixedSizeTerminal::<2usize>())
                .map(|v| Box::new(version_pragma_operator::VersionPragmaOperator::LessEqual(v))),
            terminal(">=")
                .to(FixedSizeTerminal::<2usize>())
                .map(|v| Box::new(version_pragma_operator::VersionPragmaOperator::GreaterEqual(v))),
        ))
        .boxed();

        // «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' }  { '.' 1…*{ '0'…'9' | 'x' | 'X' | '*' } } ;
        let version_pragma_value_parser =
            filter(|&c: &char| ('0' <= c && c <= '9') || c == 'x' || c == 'X' || c == '*')
                .to(FixedSizeTerminal::<1>())
                .repeated()
                .at_least(1usize)
                .map(|v| VariableSizeTerminal(v.len()))
                .then(
                    just('.')
                        .to(FixedSizeTerminal::<1usize>())
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9') || c == 'x' || c == 'X' || c == '*'
                            })
                            .to(FixedSizeTerminal::<1>())
                            .repeated()
                            .at_least(1usize)
                            .map(|v| VariableSizeTerminal(v.len())),
                        )
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|(filter_0_repeated_repeated, period_repeated)| {
                    version_pragma_value::VersionPragmaValue {
                        filter_0_repeated_repeated,
                        period_repeated,
                    }
                })
                .boxed();

        // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
        let whitespace_parser = filter(|&c: &char| c == ' ' || c == '\t')
            .to(FixedSizeTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| VariableSizeTerminal(v.len()))
            .boxed();

        // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
        let yul_decimal_number_literal_parser = choice((
            just('0')
                .to(FixedSizeTerminal::<1>())
                .map(|v| Box::new(yul_decimal_number_literal::YulDecimalNumberLiteral::Zero(v))),
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .to(FixedSizeTerminal::<1>())
                .then(
                    filter(|&c: &char| ('0' <= c && c <= '9'))
                        .to(FixedSizeTerminal::<1>())
                        .repeated()
                        .map(|v| VariableSizeTerminal(v.len())),
                )
                .map(|v| yul_decimal_number_literal::Sequence0::from_parse(v))
                .map(|v| {
                    Box::new(yul_decimal_number_literal::YulDecimalNumberLiteral::Sequence0(v))
                }),
        ))
        .boxed();

        // «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
        let yul_hex_literal_parser = terminal("0x")
            .to(FixedSizeTerminal::<2usize>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .to(FixedSizeTerminal::<1>())
                .repeated()
                .at_least(1usize)
                .map(|v| VariableSizeTerminal(v.len())),
            )
            .map(|v| yul_hex_literal::YulHexLiteral::from_parse(v))
            .boxed();

        // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
        let decimal_exponent_parser = filter(|&c: &char| c == 'e' || c == 'E')
            .to(FixedSizeTerminal::<1>())
            .then(just('-').to(FixedSizeTerminal::<1>()).or_not())
            .then(decimal_integer_parser.clone())
            .map(|v| decimal_exponent::DecimalExponent::from_parse(v))
            .boxed();

        // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
        let decimal_float_parser = decimal_integer_parser
            .clone()
            .or_not()
            .then(just('.').to(FixedSizeTerminal::<1>()))
            .then(decimal_integer_parser.clone())
            .map(|v| decimal_float::DecimalFloat::from_parse(v))
            .boxed();

        // «EndOfFileTrivia» = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
        let end_of_file_trivia_parser = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(end_of_file_trivia::Choices0::Whitespace(v))),
            multiline_comment_parser
                .clone()
                .map(|v| Box::new(end_of_file_trivia::Choices0::MultilineComment(v))),
            single_line_comment_parser
                .clone()
                .map(|v| Box::new(end_of_file_trivia::Choices0::SingleLineComment(v))),
        ))
        .repeated()
        .boxed();

        // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
        let escape_sequence_parser = just('\\')
            .to(FixedSizeTerminal::<1>())
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
                .to(FixedSizeTerminal::<1>())
                .map(|v| Box::new(escape_sequence::Choices0::Filter1(v))),
                hex_byte_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::Choices0::HexByteEscape(v))),
                unicode_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::Choices0::UnicodeEscape(v))),
            )))
            .map(|v| escape_sequence::EscapeSequence::from_parse(v))
            .boxed();

        // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
        let hex_string_literal_parser = terminal ("hex") . to (FixedSizeTerminal :: < 3usize > ()) . then (choice ((just ('"') . to (FixedSizeTerminal :: < 1usize > ()) . then (possibly_separated_pairs_of_hex_digits_parser . clone () . or_not ()) . then (just ('"') . to (FixedSizeTerminal :: < 1usize > ())) . map (| v | hex_string_literal :: DoubleQuoteAndPossiblySeparatedPairsOfHexDigitsAndDoubleQuote :: from_parse (v)) . map (| v | Box :: new (hex_string_literal :: Choices0 :: DoubleQuoteAndPossiblySeparatedPairsOfHexDigitsAndDoubleQuote (v))) , just ('\'') . to (FixedSizeTerminal :: < 1usize > ()) . then (possibly_separated_pairs_of_hex_digits_parser . clone () . or_not ()) . then (just ('\'') . to (FixedSizeTerminal :: < 1usize > ())) . map (| v | hex_string_literal :: QuoteAndPossiblySeparatedPairsOfHexDigitsAndQuote :: from_parse (v)) . map (| v | Box :: new (hex_string_literal :: Choices0 :: QuoteAndPossiblySeparatedPairsOfHexDigitsAndQuote (v)))))) . map (| v | hex_string_literal :: HexStringLiteral :: from_parse (v)) . boxed () ;

        // «LeadingTrivia» = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
        let leading_trivia_parser = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(leading_trivia::Choices0::Whitespace(v))),
            end_of_line_parser
                .clone()
                .map(|v| Box::new(leading_trivia::Choices0::EndOfLine(v))),
            multiline_comment_parser
                .clone()
                .map(|v| Box::new(leading_trivia::Choices0::MultilineComment(v))),
            single_line_comment_parser
                .clone()
                .map(|v| Box::new(leading_trivia::Choices0::SingleLineComment(v))),
        ))
        .repeated()
        .boxed();

        // «TrailingTrivia» = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
        let trailing_trivia_parser = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::Choices0::Whitespace(v))),
            multiline_comment_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::Choices0::MultilineComment(v))),
        ))
        .repeated()
        .then(choice((
            end_of_line_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::Choices1::EndOfLine(v))),
            single_line_comment_parser
                .clone()
                .map(|v| Box::new(trailing_trivia::Choices1::SingleLineComment(v))),
        )))
        .map(|v| trailing_trivia::TrailingTrivia::from_parse(v))
        .or_not()
        .boxed();

        // «UnsignedFixedType» = 'u' «SignedFixedType» ;
        let unsigned_fixed_type_parser = just('u')
            .to(FixedSizeTerminal::<1>())
            .then(signed_fixed_type_parser.clone())
            .map(|v| unsigned_fixed_type::UnsignedFixedType::from_parse(v))
            .boxed();

        // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
        let unsigned_integer_type_parser = just('u')
            .to(FixedSizeTerminal::<1>())
            .then(signed_integer_type_parser.clone())
            .map(|v| unsigned_integer_type::UnsignedIntegerType::from_parse(v))
            .boxed();

        // «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
        let yul_keyword_parser = choice((
            boolean_literal_parser
                .clone()
                .map(|v| Box::new(yul_keyword::YulKeyword::BooleanLiteral(v))),
            choice::<_, ErrorType>((
                terminal("break").to(VariableSizeTerminal(5usize)),
                terminal("c").ignore_then(choice((
                    terminal("ase").to(VariableSizeTerminal(4usize)),
                    terminal("ontinue").to(VariableSizeTerminal(8usize)),
                ))),
                terminal("default").to(VariableSizeTerminal(7usize)),
                terminal("f").ignore_then(choice((
                    terminal("or").to(VariableSizeTerminal(3usize)),
                    terminal("unction").to(VariableSizeTerminal(8usize)),
                ))),
                terminal("hex").to(VariableSizeTerminal(3usize)),
                terminal("if").to(VariableSizeTerminal(2usize)),
                terminal("le").ignore_then(choice((
                    terminal("ave").to(VariableSizeTerminal(5usize)),
                    terminal("t").to(VariableSizeTerminal(3usize)),
                ))),
                terminal("switch").to(VariableSizeTerminal(6usize)),
            ))
            .map(|v| Box::new(yul_keyword::YulKeyword::Terminal0(v))),
        ))
        .boxed();

        // AddressType = 'address' [ 'payable' ] ;
        let address_type_parser = leading_trivia_parser
            .clone()
            .then(terminal("address").to(FixedSizeTerminal::<7usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("payable").to(FixedSizeTerminal::<7usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .or_not(),
            )
            .map(|v| address_type::AddressType::from_parse(v))
            .boxed();

        // ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
        let array_literal_parser = leading_trivia_parser
            .clone()
            .then(just('[').to(FixedSizeTerminal::<1usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                expression_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            })
                            .then(expression_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(expression_repeated, comma_repeated)| {
                        array_literal::ExpressionRepeatedAndCommaRepeated {
                            expression_repeated,
                            comma_repeated,
                        }
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(']').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| array_literal::ArrayLiteral::from_parse(v))
            .boxed();

        // BreakStatement = 'break' ';' ;
        let break_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("break").to(FixedSizeTerminal::<5usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| break_statement::BreakStatement::from_parse(v))
            .boxed();

        // ContinueStatement = 'continue' ';' ;
        let continue_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("continue").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| continue_statement::ContinueStatement::from_parse(v))
            .boxed();

        // DataLocation = 'memory' | 'storage' | 'calldata' ;
        let data_location_parser = choice((
            leading_trivia_parser
                .clone()
                .then(terminal("memory").to(FixedSizeTerminal::<6usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(data_location::DataLocation::Memory(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("storage").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(data_location::DataLocation::Storage(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("calldata").to(FixedSizeTerminal::<8usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(data_location::DataLocation::Calldata(v))),
        ))
        .boxed();

        // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
        let decimal_number_parser = choice((
            decimal_integer_parser
                .clone()
                .map(|v| Box::new(decimal_number::Choices0::DecimalInteger(v))),
            decimal_float_parser
                .clone()
                .map(|v| Box::new(decimal_number::Choices0::DecimalFloat(v))),
        ))
        .then(decimal_exponent_parser.clone().or_not())
        .map(|v| decimal_number::DecimalNumber::from_parse(v))
        .boxed();

        // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
        let double_quoted_ascii_string_literal_parser = just('"')
            .to(FixedSizeTerminal::<1usize>())
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '"' && c != '\\')
                        .to(FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .map(|v| {
                            Box::new(double_quoted_ascii_string_literal::Run::CharRepeated(v))
                        }),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(double_quoted_ascii_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('"').to(FixedSizeTerminal::<1usize>()))
            .map(|v| {
                double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral::from_parse(v)
            })
            .boxed();

        // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
        let double_quoted_unicode_string_literal_parser = just("unicode\"")
            .to(FixedSizeTerminal::<8usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '"' && c != '\\' && c != '\n' && c != '\r')
                        .to(FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .map(|v| {
                            Box::new(double_quoted_unicode_string_literal::Run::CharRepeated(v))
                        }),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(double_quoted_unicode_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('"').to(FixedSizeTerminal::<1usize>()))
            .map(|v| {
                double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral::from_parse(
                    v,
                )
            })
            .boxed();

        // «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
        let keyword_parser = choice((
            boolean_literal_parser
                .clone()
                .map(|v| Box::new(keyword::Keyword::BooleanLiteral(v))),
            fixed_bytes_type_parser
                .clone()
                .map(|v| Box::new(keyword::Keyword::FixedBytesType(v))),
            number_unit_parser
                .clone()
                .map(|v| Box::new(keyword::Keyword::NumberUnit(v))),
            reserved_keyword_parser
                .clone()
                .map(|v| Box::new(keyword::Keyword::ReservedKeyword(v))),
            signed_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::Keyword::SignedIntegerType(v))),
            unsigned_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::Keyword::UnsignedIntegerType(v))),
            choice::<_, ErrorType>((
                terminal("a").ignore_then(choice((
                    terminal("bstract").to(VariableSizeTerminal(8usize)),
                    terminal("ddress").to(VariableSizeTerminal(7usize)),
                    terminal("nonymous").to(VariableSizeTerminal(9usize)),
                    terminal("s").ignore_then(choice((
                        terminal("sembly").to(VariableSizeTerminal(8usize)),
                        empty().to(VariableSizeTerminal(2usize)),
                    ))),
                ))),
                terminal("b").ignore_then(choice((
                    terminal("ool").to(VariableSizeTerminal(4usize)),
                    terminal("reak").to(VariableSizeTerminal(5usize)),
                ))),
                terminal("c").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("lldata").to(VariableSizeTerminal(8usize)),
                        terminal("tch").to(VariableSizeTerminal(5usize)),
                    ))),
                    terminal("on").ignore_then(choice((
                        terminal("st").ignore_then(choice((
                            terminal("ant").to(VariableSizeTerminal(8usize)),
                            terminal("ructor").to(VariableSizeTerminal(11usize)),
                        ))),
                        terminal("t").ignore_then(choice((
                            terminal("inue").to(VariableSizeTerminal(8usize)),
                            terminal("ract").to(VariableSizeTerminal(8usize)),
                        ))),
                    ))),
                ))),
                terminal("d").ignore_then(choice((
                    terminal("elete").to(VariableSizeTerminal(6usize)),
                    terminal("o").to(VariableSizeTerminal(2usize)),
                ))),
                terminal("e").ignore_then(choice((
                    terminal("lse").to(VariableSizeTerminal(4usize)),
                    terminal("mit").to(VariableSizeTerminal(4usize)),
                    terminal("num").to(VariableSizeTerminal(4usize)),
                    terminal("vent").to(VariableSizeTerminal(5usize)),
                    terminal("xternal").to(VariableSizeTerminal(8usize)),
                ))),
                terminal("f").ignore_then(choice((
                    terminal("al").ignore_then(choice((
                        terminal("lback").to(VariableSizeTerminal(8usize)),
                        terminal("se").to(VariableSizeTerminal(5usize)),
                    ))),
                    terminal("ixed").to(VariableSizeTerminal(5usize)),
                    terminal("or").to(VariableSizeTerminal(3usize)),
                    terminal("unction").to(VariableSizeTerminal(8usize)),
                ))),
                terminal("hex").to(VariableSizeTerminal(3usize)),
                terminal("i").ignore_then(choice((
                    terminal("f").to(VariableSizeTerminal(2usize)),
                    terminal("m").ignore_then(choice((
                        terminal("mutable").to(VariableSizeTerminal(9usize)),
                        terminal("port").to(VariableSizeTerminal(6usize)),
                    ))),
                    terminal("n").ignore_then(choice((
                        terminal("dexed").to(VariableSizeTerminal(7usize)),
                        terminal("ter").ignore_then(choice((
                            terminal("face").to(VariableSizeTerminal(9usize)),
                            terminal("nal").to(VariableSizeTerminal(8usize)),
                        ))),
                    ))),
                    terminal("s").to(VariableSizeTerminal(2usize)),
                ))),
                terminal("library").to(VariableSizeTerminal(7usize)),
                terminal("m").ignore_then(choice((
                    terminal("apping").to(VariableSizeTerminal(7usize)),
                    terminal("emory").to(VariableSizeTerminal(6usize)),
                    terminal("odifier").to(VariableSizeTerminal(8usize)),
                ))),
                terminal("new").to(VariableSizeTerminal(3usize)),
                terminal("override").to(VariableSizeTerminal(8usize)),
                terminal("p").ignore_then(choice((
                    terminal("ayable").to(VariableSizeTerminal(7usize)),
                    terminal("r").ignore_then(choice((
                        terminal("agma").to(VariableSizeTerminal(6usize)),
                        terminal("ivate").to(VariableSizeTerminal(7usize)),
                    ))),
                    terminal("u").ignore_then(choice((
                        terminal("blic").to(VariableSizeTerminal(6usize)),
                        terminal("re").to(VariableSizeTerminal(4usize)),
                    ))),
                ))),
                terminal("re").ignore_then(choice((
                    terminal("ceive").to(VariableSizeTerminal(7usize)),
                    terminal("turn").ignore_then(choice((
                        terminal("s").to(VariableSizeTerminal(7usize)),
                        empty().to(VariableSizeTerminal(6usize)),
                    ))),
                ))),
                terminal("st").ignore_then(choice((
                    terminal("orage").to(VariableSizeTerminal(7usize)),
                    terminal("r").ignore_then(choice((
                        terminal("ing").to(VariableSizeTerminal(6usize)),
                        terminal("uct").to(VariableSizeTerminal(6usize)),
                    ))),
                ))),
                terminal("t").ignore_then(choice((
                    terminal("r").ignore_then(choice((
                        terminal("ue").to(VariableSizeTerminal(4usize)),
                        terminal("y").to(VariableSizeTerminal(3usize)),
                    ))),
                    terminal("ype").to(VariableSizeTerminal(4usize)),
                ))),
                terminal("u").ignore_then(choice((
                    terminal("fixed").to(VariableSizeTerminal(6usize)),
                    terminal("nchecked").to(VariableSizeTerminal(9usize)),
                    terminal("sing").to(VariableSizeTerminal(5usize)),
                ))),
                terminal("vi").ignore_then(choice((
                    terminal("ew").to(VariableSizeTerminal(4usize)),
                    terminal("rtual").to(VariableSizeTerminal(7usize)),
                ))),
                terminal("while").to(VariableSizeTerminal(5usize)),
            ))
            .map(|v| Box::new(keyword::Keyword::Terminal0(v))),
        ))
        .boxed();

        // ParenthesisExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
        let parenthesis_expression_parser = leading_trivia_parser
            .clone()
            .then(just('(').to(FixedSizeTerminal::<1usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                expression_parser
                    .clone()
                    .or_not()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            })
                            .then(expression_parser.clone().or_not())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(expression_repeated, comma_repeated)| {
                        parenthesis_expression::ExpressionRepeatedAndCommaRepeated {
                            expression_repeated,
                            comma_repeated,
                        }
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(')').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| parenthesis_expression::ParenthesisExpression::from_parse(v))
            .boxed();

        // PositionalArgumentList = Expression  { ',' Expression } ;
        let positional_argument_list_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(',').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(expression_parser.clone())
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|(expression_repeated, comma_repeated)| {
                positional_argument_list::PositionalArgumentList {
                    expression_repeated,
                    comma_repeated,
                }
            })
            .boxed();

        // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_ascii_string_literal_parser = just('\'')
            .to(FixedSizeTerminal::<1usize>())
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '\'' && c != '\\')
                        .to(FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .map(|v| {
                            Box::new(single_quoted_ascii_string_literal::Run::CharRepeated(v))
                        }),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(single_quoted_ascii_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('\'').to(FixedSizeTerminal::<1usize>()))
            .map(|v| {
                single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral::from_parse(v)
            })
            .boxed();

        // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_unicode_string_literal_parser = just("unicode'")
            .to(FixedSizeTerminal::<8usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '\'' && c != '\\' && c != '\n' && c != '\r')
                        .to(FixedSizeTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| VariableSizeTerminal(v.len()))
                        .map(|v| {
                            Box::new(single_quoted_unicode_string_literal::Run::CharRepeated(v))
                        }),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(single_quoted_unicode_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('\'').to(FixedSizeTerminal::<1usize>()))
            .map(|v| {
                single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral::from_parse(
                    v,
                )
            })
            .boxed();

        // UncheckedBlock = 'unchecked' Block ;
        let unchecked_block_parser = leading_trivia_parser
            .clone()
            .then(terminal("unchecked").to(FixedSizeTerminal::<9usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(block_parser.clone())
            .map(|v| unchecked_block::UncheckedBlock::from_parse(v))
            .boxed();

        // VersionPragmaSpecifier = 'solidity' 1…*{ «VersionPragmaOperator» «VersionPragmaValue» } ;
        let version_pragma_specifier_parser = leading_trivia_parser
            .clone()
            .then(terminal("solidity").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(version_pragma_operator_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        version_pragma_operator::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(version_pragma_value_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                version_pragma_value::WithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| version_pragma_specifier::Sequence0::from_parse(v))
                    .repeated()
                    .at_least(1usize),
            )
            .map(|v| version_pragma_specifier::VersionPragmaSpecifier::from_parse(v))
            .boxed();

        // YulBreakStatement = 'break' ;
        let yul_break_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("break").to(FixedSizeTerminal::<5usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .boxed();

        // YulContinueStatement = 'continue' ;
        let yul_continue_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("continue").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .boxed();

        // «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
        let yul_identifier_parser =
            difference(raw_identifier_parser.clone(), yul_keyword_parser.clone()).boxed();

        // YulLeaveStatement = 'leave' ;
        let yul_leave_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("leave").to(FixedSizeTerminal::<5usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .boxed();

        // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
        let ascii_string_literal_parser = choice((
            single_quoted_ascii_string_literal_parser.clone().map(|v| {
                Box::new(
                    ascii_string_literal::AsciiStringLiteral::SingleQuotedAsciiStringLiteral(v),
                )
            }),
            double_quoted_ascii_string_literal_parser.clone().map(|v| {
                Box::new(
                    ascii_string_literal::AsciiStringLiteral::DoubleQuotedAsciiStringLiteral(v),
                )
            }),
        ))
        .boxed();

        // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
        let assembly_flags_parser = leading_trivia_parser
            .clone()
            .then(just('(').to(FixedSizeTerminal::<1usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(double_quoted_ascii_string_literal_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        double_quoted_ascii_string_literal::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            })
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(double_quoted_ascii_string_literal_parser.clone())
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                                        double_quoted_ascii_string_literal::WithTrivia {
                                            leading_trivia,
                                            terminal,
                                            trailing_trivia,
                                        }
                                    }),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(
                        |(double_quoted_ascii_string_literal_repeated, comma_repeated)| {
                            assembly_flags::DoubleQuotedAsciiStringLiteralRepeatedAndCommaRepeated {
                                double_quoted_ascii_string_literal_repeated,
                                comma_repeated,
                            }
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(')').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| assembly_flags::AssemblyFlags::from_parse(v))
            .boxed();

        // ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
        let elementary_type_parser = choice((
            leading_trivia_parser
                .clone()
                .then(terminal("bool").to(FixedSizeTerminal::<4usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(elementary_type::ElementaryType::Bool(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("string").to(FixedSizeTerminal::<6usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(elementary_type::ElementaryType::String(v))),
            address_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::ElementaryType::AddressType(v))),
            leading_trivia_parser
                .clone()
                .then(fixed_bytes_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| fixed_bytes_type::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(elementary_type::ElementaryType::FixedBytesType(v))),
            leading_trivia_parser
                .clone()
                .then(signed_integer_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    signed_integer_type::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| Box::new(elementary_type::ElementaryType::SignedIntegerType(v))),
            leading_trivia_parser
                .clone()
                .then(unsigned_integer_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    unsigned_integer_type::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| Box::new(elementary_type::ElementaryType::UnsignedIntegerType(v))),
            leading_trivia_parser
                .clone()
                .then(signed_fixed_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    signed_fixed_type::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| Box::new(elementary_type::ElementaryType::SignedFixedType(v))),
            leading_trivia_parser
                .clone()
                .then(unsigned_fixed_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    unsigned_fixed_type::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| Box::new(elementary_type::ElementaryType::UnsignedFixedType(v))),
        ))
        .boxed();

        // «Identifier» = «RawIdentifier» - «Keyword» ;
        let identifier_parser =
            difference(raw_identifier_parser.clone(), keyword_parser.clone()).boxed();

        // «NumericLiteral» = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
        let numeric_literal_parser = choice((
            decimal_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::Choices0::DecimalNumber(v))),
            hex_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::Choices0::HexNumber(v))),
        ))
        .then(number_unit_parser.clone().or_not())
        .map(|v| numeric_literal::NumericLiteral::from_parse(v))
        .boxed();

        // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
        let unicode_string_literal_parser = choice ((single_quoted_unicode_string_literal_parser . clone () . map (| v | Box :: new (unicode_string_literal :: UnicodeStringLiteral :: SingleQuotedUnicodeStringLiteral (v))) , double_quoted_unicode_string_literal_parser . clone () . map (| v | Box :: new (unicode_string_literal :: UnicodeStringLiteral :: DoubleQuotedUnicodeStringLiteral (v))))) . boxed () ;

        // YulFunctionCall = «YulIdentifier» '(' [ YulExpression  { ',' YulExpression } ] ')' ;
        let yul_function_call_parser = leading_trivia_parser . clone () . then (yul_identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | yul_identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (just ('(') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (yul_expression_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (yul_expression_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| (yul_expression_repeated , comma_repeated) | yul_function_call :: YulExpressionRepeatedAndCommaRepeated { yul_expression_repeated , comma_repeated }) . or_not ()) . then (leading_trivia_parser . clone () . then (just (')') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | yul_function_call :: OpenParenAndYulExpressionRepeatedAndCommaRepeatedAndCloseParen :: from_parse (v))) . map (| v | yul_function_call :: YulFunctionCall :: from_parse (v)) . boxed () ;

        // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
        let yul_function_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("function").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(yul_identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        yul_identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(yul_identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                yul_identifier::WithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            })
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(just(',').to(FixedSizeTerminal::<1usize>()))
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                                        FixedSizeTerminalWithTrivia {
                                            leading_trivia,
                                            terminal,
                                            trailing_trivia,
                                        }
                                    })
                                    .then(
                                        leading_trivia_parser
                                            .clone()
                                            .then(yul_identifier_parser.clone())
                                            .then(trailing_trivia_parser.clone())
                                            .map(
                                                |((leading_trivia, terminal), trailing_trivia)| {
                                                    yul_identifier::WithTrivia {
                                                        leading_trivia,
                                                        terminal,
                                                        trailing_trivia,
                                                    }
                                                },
                                            ),
                                    )
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|(yul_identifier_repeated, comma_repeated)| {
                                yul_function_definition::Arguments {
                                    yul_identifier_repeated,
                                    comma_repeated,
                                }
                            })
                            .or_not(),
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(')').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| {
                        yul_function_definition::OpenParenAndArgumentsAndCloseParen::from_parse(v)
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("->").to(FixedSizeTerminal::<2usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(yul_identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                yul_identifier::WithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            })
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(just(',').to(FixedSizeTerminal::<1usize>()))
                                    .then(trailing_trivia_parser.clone())
                                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                                        FixedSizeTerminalWithTrivia {
                                            leading_trivia,
                                            terminal,
                                            trailing_trivia,
                                        }
                                    })
                                    .then(
                                        leading_trivia_parser
                                            .clone()
                                            .then(yul_identifier_parser.clone())
                                            .then(trailing_trivia_parser.clone())
                                            .map(
                                                |((leading_trivia, terminal), trailing_trivia)| {
                                                    yul_identifier::WithTrivia {
                                                        leading_trivia,
                                                        terminal,
                                                        trailing_trivia,
                                                    }
                                                },
                                            ),
                                    )
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|(yul_identifier_repeated, comma_repeated)| {
                                yul_function_definition::Results {
                                    yul_identifier_repeated,
                                    comma_repeated,
                                }
                            }),
                    )
                    .map(|v| yul_function_definition::Sequence0::from_parse(v))
                    .or_not(),
            )
            .then(yul_block_parser.clone())
            .map(|v| yul_function_definition::YulFunctionDefinition::from_parse(v))
            .boxed();

        // YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
        let yul_identifier_path_parser = leading_trivia_parser
            .clone()
            .then(yul_identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| yul_identifier::WithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('.').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(yul_identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                yul_identifier::WithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|(yul_identifier_repeated, period_repeated)| {
                yul_identifier_path::YulIdentifierPath {
                    yul_identifier_repeated,
                    period_repeated,
                }
            })
            .boxed();

        // ABICoderPragmaSpecifier = 'abicoder' «Identifier» ;
        let abi_coder_pragma_specifier_parser = leading_trivia_parser
            .clone()
            .then(terminal("abicoder").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .map(|v| abi_coder_pragma_specifier::AbiCoderPragmaSpecifier::from_parse(v))
            .boxed();

        // DeleteStatement = 'delete' «Identifier» ';' ;
        let delete_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("delete").to(FixedSizeTerminal::<6usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| delete_statement::DeleteStatement::from_parse(v))
            .boxed();

        // EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
        let enum_definition_parser = leading_trivia_parser . clone () . then (terminal ("enum") . to (FixedSizeTerminal :: < 4usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . then (leading_trivia_parser . clone () . then (just ('{') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (just (',') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . repeated ()) . map (repetition_mapper) . map (| (identifier_repeated , comma_repeated) | enum_definition :: IdentifierRepeatedAndCommaRepeated { identifier_repeated , comma_repeated })) . then (leading_trivia_parser . clone () . then (just ('}') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | enum_definition :: OpenBraceAndIdentifierRepeatedAndCommaRepeatedAndCloseBrace :: from_parse (v))) . map (| v | enum_definition :: EnumDefinition :: from_parse (v)) . boxed () ;

        // ExperimentalPragmaSpecifier = 'experimental' «Identifier» ;
        let experimental_pragma_specifier_parser = leading_trivia_parser
            .clone()
            .then(terminal("experimental").to(FixedSizeTerminal::<12usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .map(|v| experimental_pragma_specifier::ExperimentalPragmaSpecifier::from_parse(v))
            .boxed();

        // IdentifierPath = «Identifier»  { '.' «Identifier» } ;
        let identifier_path_parser = leading_trivia_parser
            .clone()
            .then(identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('.').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                identifier::WithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(
                |(identifier_repeated, period_repeated)| identifier_path::IdentifierPath {
                    identifier_repeated,
                    period_repeated,
                },
            )
            .boxed();

        // ImportPath = «AsciiStringLiteral» ;
        let import_path_parser = leading_trivia_parser
            .clone()
            .then(ascii_string_literal_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| ascii_string_literal::WithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .boxed();

        // NamedArgument = «Identifier» ':' Expression ;
        let named_argument_parser = leading_trivia_parser
            .clone()
            .then(identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(':').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .then(expression_parser.clone())
            .map(|v| named_argument::NamedArgument::from_parse(v))
            .boxed();

        // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
        let parameter_declaration_parser = type_name_parser
            .clone()
            .then(data_location_parser.clone().or_not())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    )
                    .or_not(),
            )
            .map(|v| parameter_declaration::ParameterDeclaration::from_parse(v))
            .boxed();

        // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
        let selected_import_parser = leading_trivia_parser
            .clone()
            .then(identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("as").to(FixedSizeTerminal::<2usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                identifier::WithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| selected_import::Sequence0::from_parse(v))
                    .or_not(),
            )
            .map(|v| selected_import::SelectedImport::from_parse(v))
            .boxed();

        // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
        let user_defined_value_type_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("type").to(FixedSizeTerminal::<4usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("is").to(FixedSizeTerminal::<2usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .then(elementary_type_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| {
                user_defined_value_type_definition::UserDefinedValueTypeDefinition::from_parse(v)
            })
            .boxed();

        // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
        let yul_literal_parser = choice((
            leading_trivia_parser
                .clone()
                .then(yul_decimal_number_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    yul_decimal_number_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| Box::new(yul_literal::YulLiteral::YulDecimalNumberLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(yul_hex_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| yul_hex_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(yul_literal::YulLiteral::YulHexLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(ascii_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    ascii_string_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| Box::new(yul_literal::YulLiteral::AsciiStringLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(boolean_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| boolean_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(yul_literal::YulLiteral::BooleanLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(hex_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    hex_string_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| Box::new(yul_literal::YulLiteral::HexStringLiteral(v))),
        ))
        .boxed();

        // MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
        let mapping_type_parser = leading_trivia_parser
            .clone()
            .then(terminal("mapping").to(FixedSizeTerminal::<7usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        choice((
                            elementary_type_parser
                                .clone()
                                .map(|v| Box::new(mapping_type::Choices1::ElementaryType(v))),
                            identifier_path_parser
                                .clone()
                                .map(|v| Box::new(mapping_type::Choices1::IdentifierPath(v))),
                        ))
                        .then(
                            leading_trivia_parser
                                .clone()
                                .then(terminal("=>").to(FixedSizeTerminal::<2usize>()))
                                .then(trailing_trivia_parser.clone())
                                .map(|((leading_trivia, terminal), trailing_trivia)| {
                                    FixedSizeTerminalWithTrivia {
                                        leading_trivia,
                                        terminal,
                                        trailing_trivia,
                                    }
                                }),
                        )
                        .then(type_name_parser.clone())
                        .map(|v| mapping_type::Sequence0::from_parse(v)),
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(')').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| mapping_type::OpenParenAndSequence0AndCloseParen::from_parse(v)),
            )
            .map(|v| mapping_type::MappingType::from_parse(v))
            .boxed();

        // NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
        let named_argument_list_parser = leading_trivia_parser
            .clone()
            .then(just('{').to(FixedSizeTerminal::<1usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                named_argument_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            })
                            .then(named_argument_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(named_argument_repeated, comma_repeated)| {
                        named_argument_list::NamedArgumentRepeatedAndCommaRepeated {
                            named_argument_repeated,
                            comma_repeated,
                        }
                    })
                    .or_not(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('}').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| named_argument_list::NamedArgumentList::from_parse(v))
            .boxed();

        // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
        let override_specifier_parser = leading_trivia_parser . clone () . then (terminal ("override") . to (FixedSizeTerminal :: < 8usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (just ('(') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (identifier_path_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (identifier_path_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| (identifier_path_repeated , comma_repeated) | override_specifier :: IdentifierPathRepeatedAndCommaRepeated { identifier_path_repeated , comma_repeated })) . then (leading_trivia_parser . clone () . then (just (')') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | override_specifier :: OpenParenAndIdentifierPathRepeatedAndCommaRepeatedAndCloseParen :: from_parse (v)) . or_not ()) . map (| v | override_specifier :: OverrideSpecifier :: from_parse (v)) . boxed () ;

        // ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
        let parameter_list_parser = leading_trivia_parser
            .clone()
            .then(just('(').to(FixedSizeTerminal::<1usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            })
                            .then(parameter_declaration_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(parameter_declaration_repeated, comma_repeated)| {
                        parameter_list::ParameterDeclarationRepeatedAndCommaRepeated {
                            parameter_declaration_repeated,
                            comma_repeated,
                        }
                    })
                    .or_not(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(')').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| parameter_list::ParameterList::from_parse(v))
            .boxed();

        // PragmaDirective = 'pragma' ( VersionPragmaSpecifier | ABICoderPragmaSpecifier | ExperimentalPragmaSpecifier ) ';' ;
        let pragma_directive_parser = leading_trivia_parser
            .clone()
            .then(terminal("pragma").to(FixedSizeTerminal::<6usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(choice((
                version_pragma_specifier_parser
                    .clone()
                    .map(|v| Box::new(pragma_directive::Choices0::VersionPragmaSpecifier(v))),
                abi_coder_pragma_specifier_parser
                    .clone()
                    .map(|v| Box::new(pragma_directive::Choices0::AbiCoderPragmaSpecifier(v))),
                experimental_pragma_specifier_parser
                    .clone()
                    .map(|v| Box::new(pragma_directive::Choices0::ExperimentalPragmaSpecifier(v))),
            )))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| pragma_directive::PragmaDirective::from_parse(v))
            .boxed();

        // SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
        let selecting_import_directive_parser = leading_trivia_parser . clone () . then (just ('{') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (selected_import_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (selected_import_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| (selected_import_repeated , comma_repeated) | selecting_import_directive :: SelectedImportRepeatedAndCommaRepeated { selected_import_repeated , comma_repeated })) . then (leading_trivia_parser . clone () . then (just ('}') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | selecting_import_directive :: OpenBraceAndSelectedImportRepeatedAndCommaRepeatedAndCloseBrace :: from_parse (v)) . then (leading_trivia_parser . clone () . then (terminal ("from") . to (FixedSizeTerminal :: < 4usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . then (import_path_parser . clone ()) . map (| v | selecting_import_directive :: SelectingImportDirective :: from_parse (v)) . boxed () ;

        // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
        let simple_import_directive_parser = import_path_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("as").to(FixedSizeTerminal::<2usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                identifier::WithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| simple_import_directive::Sequence0::from_parse(v))
                    .repeated(),
            )
            .map(|v| simple_import_directive::SimpleImportDirective::from_parse(v))
            .boxed();

        // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
        let star_import_directive_parser = leading_trivia_parser
            .clone()
            .then(just('*').to(FixedSizeTerminal::<1>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("as").to(FixedSizeTerminal::<2usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("from").to(FixedSizeTerminal::<4usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .then(import_path_parser.clone())
            .map(|v| star_import_directive::StarImportDirective::from_parse(v))
            .boxed();

        // YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
        yul_expression_parser.define(
            choice((
                yul_identifier_path_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::YulExpression::YulIdentifierPath(v))),
                yul_function_call_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::YulExpression::YulFunctionCall(v))),
                yul_literal_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::YulExpression::YulLiteral(v))),
            ))
            .boxed(),
        );

        // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
        let argument_list_parser = leading_trivia_parser
            .clone()
            .then(just('(').to(FixedSizeTerminal::<1usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                choice((
                    positional_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::Choices0::PositionalArgumentList(v))),
                    named_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::Choices0::NamedArgumentList(v))),
                ))
                .or_not(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(')').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| argument_list::ArgumentList::from_parse(v))
            .boxed();

        // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
        let catch_clause_parser = leading_trivia_parser
            .clone()
            .then(terminal("catch").to(FixedSizeTerminal::<5usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    )
                    .or_not()
                    .then(parameter_list_parser.clone())
                    .map(|v| catch_clause::Sequence0::from_parse(v))
                    .or_not(),
            )
            .then(block_parser.clone())
            .map(|v| catch_clause::CatchClause::from_parse(v))
            .boxed();

        // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
        let function_type_parser = leading_trivia_parser
            .clone()
            .then(terminal("function").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(parameter_list_parser.clone())
            .then(
                choice((
                    leading_trivia_parser
                        .clone()
                        .then(terminal("internal").to(FixedSizeTerminal::<8usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(function_type::Choices0::Internal(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("external").to(FixedSizeTerminal::<8usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(function_type::Choices0::External(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("private").to(FixedSizeTerminal::<7usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(function_type::Choices0::Private(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("public").to(FixedSizeTerminal::<6usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(function_type::Choices0::Public(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("pure").to(FixedSizeTerminal::<4usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(function_type::Choices0::Pure(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("view").to(FixedSizeTerminal::<4usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(function_type::Choices0::View(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("payable").to(FixedSizeTerminal::<7usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(function_type::Choices0::Payable(v))),
                ))
                .repeated(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("returns").to(FixedSizeTerminal::<7usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(parameter_list_parser.clone())
                    .map(|v| function_type::Sequence1::from_parse(v))
                    .or_not(),
            )
            .map(|v| function_type::FunctionType::from_parse(v))
            .boxed();

        // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
        let import_directive_parser = leading_trivia_parser
            .clone()
            .then(terminal("import").to(FixedSizeTerminal::<6usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(choice((
                simple_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::Choices0::SimpleImportDirective(v))),
                star_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::Choices0::StarImportDirective(v))),
                selecting_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::Choices0::SelectingImportDirective(v))),
            )))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| import_directive::ImportDirective::from_parse(v))
            .boxed();

        // ModifierAttribute = OverrideSpecifier | 'virtual' ;
        let modifier_attribute_parser = choice((
            override_specifier_parser
                .clone()
                .map(|v| Box::new(modifier_attribute::ModifierAttribute::OverrideSpecifier(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("virtual").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(modifier_attribute::ModifierAttribute::Virtual(v))),
        ))
        .boxed();

        // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
        let state_variable_attribute_parser = choice((
            override_specifier_parser.clone().map(|v| {
                Box::new(state_variable_attribute::StateVariableAttribute::OverrideSpecifier(v))
            }),
            leading_trivia_parser
                .clone()
                .then(terminal("constant").to(FixedSizeTerminal::<8usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| {
                    Box::new(state_variable_attribute::StateVariableAttribute::Constant(
                        v,
                    ))
                }),
            leading_trivia_parser
                .clone()
                .then(terminal("immutable").to(FixedSizeTerminal::<9usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| {
                    Box::new(state_variable_attribute::StateVariableAttribute::Immutable(
                        v,
                    ))
                }),
            leading_trivia_parser
                .clone()
                .then(terminal("internal").to(FixedSizeTerminal::<8usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| {
                    Box::new(state_variable_attribute::StateVariableAttribute::Internal(
                        v,
                    ))
                }),
            leading_trivia_parser
                .clone()
                .then(terminal("private").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(state_variable_attribute::StateVariableAttribute::Private(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("public").to(FixedSizeTerminal::<6usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(state_variable_attribute::StateVariableAttribute::Public(v))),
        ))
        .boxed();

        // YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
        let yul_assignment_statement_parser = yul_identifier_path_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(',').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(yul_identifier_path_parser.clone())
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|(yul_identifier_path_repeated, comma_repeated)| {
                yul_assignment_statement::YulIdentifierPathRepeatedAndCommaRepeated {
                    yul_identifier_path_repeated,
                    comma_repeated,
                }
            })
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal(":=").to(FixedSizeTerminal::<2usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .then(yul_expression_parser.clone())
            .map(|v| yul_assignment_statement::YulAssignmentStatement::from_parse(v))
            .boxed();

        // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
        let yul_for_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("for").to(FixedSizeTerminal::<3usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(yul_block_parser.clone())
            .then(yul_expression_parser.clone())
            .then(yul_block_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| yul_for_statement::YulForStatement::from_parse(v))
            .boxed();

        // YulIfStatement = 'if' YulExpression YulBlock ;
        let yul_if_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("if").to(FixedSizeTerminal::<2usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(yul_expression_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| yul_if_statement::YulIfStatement::from_parse(v))
            .boxed();

        // YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
        let yul_switch_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("switch").to(FixedSizeTerminal::<6usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(yul_expression_parser.clone())
            .then(
                choice((
                    leading_trivia_parser
                        .clone()
                        .then(terminal("case").to(FixedSizeTerminal::<4usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .then(yul_literal_parser.clone())
                        .map(|v| yul_switch_statement::Sequence2::from_parse(v))
                        .map(|v| Box::new(yul_switch_statement::Choices1::Sequence2(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("default").to(FixedSizeTerminal::<7usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(yul_switch_statement::Choices1::Default(v))),
                ))
                .then(yul_block_parser.clone())
                .map(|v| yul_switch_statement::Sequence0::from_parse(v))
                .repeated()
                .at_least(1usize),
            )
            .map(|v| yul_switch_statement::YulSwitchStatement::from_parse(v))
            .boxed();

        // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
        let yul_variable_declaration_parser = leading_trivia_parser
            .clone()
            .then(terminal("let").to(FixedSizeTerminal::<3usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                yul_identifier_path_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            })
                            .then(yul_identifier_path_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(yul_identifier_path_repeated, comma_repeated)| {
                        yul_variable_declaration::YulIdentifierPathRepeatedAndCommaRepeated {
                            yul_identifier_path_repeated,
                            comma_repeated,
                        }
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal(":=").to(FixedSizeTerminal::<2usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(yul_expression_parser.clone())
                    .map(|v| yul_variable_declaration::Sequence0::from_parse(v))
                    .or_not(),
            )
            .map(|v| yul_variable_declaration::YulVariableDeclaration::from_parse(v))
            .boxed();

        // EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
        let emit_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("emit").to(FixedSizeTerminal::<4usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(identifier_path_parser.clone())
            .then(argument_list_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| emit_statement::EmitStatement::from_parse(v))
            .boxed();

        // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
        let inheritance_specifier_parser = identifier_path_parser
            .clone()
            .then(argument_list_parser.clone().or_not())
            .map(|v| inheritance_specifier::InheritanceSpecifier::from_parse(v))
            .boxed();

        // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
        let modifier_invocation_parser = identifier_path_parser
            .clone()
            .then(argument_list_parser.clone().or_not())
            .map(|v| modifier_invocation::ModifierInvocation::from_parse(v))
            .boxed();

        // NewExpression = 'new' IdentifierPath ArgumentList ;
        let new_expression_parser = leading_trivia_parser
            .clone()
            .then(terminal("new").to(FixedSizeTerminal::<3usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(identifier_path_parser.clone())
            .then(argument_list_parser.clone())
            .map(|v| new_expression::NewExpression::from_parse(v))
            .boxed();

        // PayableExpression = 'payable' ArgumentList ;
        let payable_expression_parser = leading_trivia_parser
            .clone()
            .then(terminal("payable").to(FixedSizeTerminal::<7usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(argument_list_parser.clone())
            .map(|v| payable_expression::PayableExpression::from_parse(v))
            .boxed();

        // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
        let revert_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("revert").to(FixedSizeTerminal::<6usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(identifier_path_parser.clone().or_not())
            .then(argument_list_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| revert_statement::RevertStatement::from_parse(v))
            .boxed();

        // TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
        type_name_parser.define(
            choice((
                elementary_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::Choices0::ElementaryType(v))),
                function_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::Choices0::FunctionType(v))),
                mapping_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::Choices0::MappingType(v))),
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(type_name::Choices0::IdentifierPath(v))),
            ))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('[').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(expression_parser.clone().or_not())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(']').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| type_name::OpenBracketAndExpressionAndCloseBracket::from_parse(v))
                    .repeated(),
            )
            .map(|v| type_name::TypeName::from_parse(v))
            .boxed(),
        );

        // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
        let yul_statement_parser = choice((
            yul_block_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulBlock(v))),
            yul_variable_declaration_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulVariableDeclaration(v))),
            yul_function_definition_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulFunctionDefinition(v))),
            yul_assignment_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulAssignmentStatement(v))),
            yul_function_call_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulFunctionCall(v))),
            yul_if_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulIfStatement(v))),
            yul_for_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulForStatement(v))),
            yul_switch_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulSwitchStatement(v))),
            yul_leave_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulLeaveStatement(v))),
            yul_break_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulBreakStatement(v))),
            yul_continue_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulContinueStatement(v))),
        ))
        .boxed();

        // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
        let constructor_attribute_parser = choice((
            modifier_invocation_parser.clone().map(|v| {
                Box::new(constructor_attribute::ConstructorAttribute::ModifierInvocation(v))
            }),
            leading_trivia_parser
                .clone()
                .then(terminal("internal").to(FixedSizeTerminal::<8usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(constructor_attribute::ConstructorAttribute::Internal(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("payable").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(constructor_attribute::ConstructorAttribute::Payable(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("public").to(FixedSizeTerminal::<6usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(constructor_attribute::ConstructorAttribute::Public(v))),
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
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    )
                    .or_not(),
            )
            .map(|v| error_parameter::ErrorParameter::from_parse(v))
            .boxed();

        // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
        let event_parameter_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("indexed").to(FixedSizeTerminal::<7usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .or_not(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    )
                    .or_not(),
            )
            .map(|v| event_parameter::EventParameter::from_parse(v))
            .boxed();

        // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
        let fallback_function_attribute_parser = choice((
            modifier_invocation_parser.clone().map(|v| {
                Box::new(
                    fallback_function_attribute::FallbackFunctionAttribute::ModifierInvocation(v),
                )
            }),
            override_specifier_parser.clone().map(|v| {
                Box::new(
                    fallback_function_attribute::FallbackFunctionAttribute::OverrideSpecifier(v),
                )
            }),
            leading_trivia_parser
                .clone()
                .then(terminal("external").to(FixedSizeTerminal::<8usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| {
                    Box::new(fallback_function_attribute::FallbackFunctionAttribute::External(v))
                }),
            leading_trivia_parser
                .clone()
                .then(terminal("payable").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| {
                    Box::new(fallback_function_attribute::FallbackFunctionAttribute::Payable(v))
                }),
            leading_trivia_parser
                .clone()
                .then(terminal("pure").to(FixedSizeTerminal::<4usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(fallback_function_attribute::FallbackFunctionAttribute::Pure(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("view").to(FixedSizeTerminal::<4usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(fallback_function_attribute::FallbackFunctionAttribute::View(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("virtual").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| {
                    Box::new(fallback_function_attribute::FallbackFunctionAttribute::Virtual(v))
                }),
        ))
        .boxed();

        // FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
        let function_attribute_parser = choice((
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(function_attribute::FunctionAttribute::ModifierInvocation(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(function_attribute::FunctionAttribute::OverrideSpecifier(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("external").to(FixedSizeTerminal::<8usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(function_attribute::FunctionAttribute::External(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("internal").to(FixedSizeTerminal::<8usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(function_attribute::FunctionAttribute::Internal(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("payable").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(function_attribute::FunctionAttribute::Payable(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("private").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(function_attribute::FunctionAttribute::Private(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("public").to(FixedSizeTerminal::<6usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(function_attribute::FunctionAttribute::Public(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("pure").to(FixedSizeTerminal::<4usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(function_attribute::FunctionAttribute::Pure(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("view").to(FixedSizeTerminal::<4usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(function_attribute::FunctionAttribute::View(v))),
            leading_trivia_parser
                .clone()
                .then(terminal("virtual").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(function_attribute::FunctionAttribute::Virtual(v))),
        ))
        .boxed();

        // InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
        let inheritance_specifier_list_parser = leading_trivia_parser
            .clone()
            .then(terminal("is").to(FixedSizeTerminal::<2usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                inheritance_specifier_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            })
                            .then(inheritance_specifier_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|(inheritance_specifier_repeated, comma_repeated)| {
                        inheritance_specifier_list::InheritanceSpecifierRepeatedAndCommaRepeated {
                            inheritance_specifier_repeated,
                            comma_repeated,
                        }
                    }),
            )
            .map(|v| inheritance_specifier_list::InheritanceSpecifierList::from_parse(v))
            .boxed();

        // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
        let receive_function_attribute_parser = choice((
            modifier_invocation_parser.clone().map(|v| {
                Box::new(
                    receive_function_attribute::ReceiveFunctionAttribute::ModifierInvocation(v),
                )
            }),
            override_specifier_parser.clone().map(|v| {
                Box::new(receive_function_attribute::ReceiveFunctionAttribute::OverrideSpecifier(v))
            }),
            leading_trivia_parser
                .clone()
                .then(terminal("external").to(FixedSizeTerminal::<8usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| {
                    Box::new(receive_function_attribute::ReceiveFunctionAttribute::External(v))
                }),
            leading_trivia_parser
                .clone()
                .then(terminal("payable").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| {
                    Box::new(receive_function_attribute::ReceiveFunctionAttribute::Payable(v))
                }),
            leading_trivia_parser
                .clone()
                .then(terminal("virtual").to(FixedSizeTerminal::<7usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| {
                    Box::new(receive_function_attribute::ReceiveFunctionAttribute::Virtual(v))
                }),
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
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| struct_member::StructMember::from_parse(v))
            .boxed();

        // TypeExpression = 'type' '(' TypeName ')' ;
        let type_expression_parser = leading_trivia_parser
            .clone()
            .then(terminal("type").to(FixedSizeTerminal::<4usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(type_name_parser.clone())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(')').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| type_expression::OpenParenAndTypeNameAndCloseParen::from_parse(v)),
            )
            .map(|v| type_expression::TypeExpression::from_parse(v))
            .boxed();

        // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
        let using_directive_parser = leading_trivia_parser . clone () . then (terminal ("using") . to (FixedSizeTerminal :: < 5usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (choice ((identifier_path_parser . clone () . map (| v | Box :: new (using_directive :: Choices0 :: IdentifierPath (v))) , leading_trivia_parser . clone () . then (just ('{') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (identifier_path_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (identifier_path_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| (identifier_path_repeated , comma_repeated) | using_directive :: IdentifierPathRepeatedAndCommaRepeated { identifier_path_repeated , comma_repeated })) . then (leading_trivia_parser . clone () . then (just ('}') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | using_directive :: OpenBraceAndIdentifierPathRepeatedAndCommaRepeatedAndCloseBrace :: from_parse (v)) . map (| v | Box :: new (using_directive :: Choices0 :: OpenBraceAndIdentifierPathRepeatedAndCommaRepeatedAndCloseBrace (v)))))) . then (leading_trivia_parser . clone () . then (terminal ("for") . to (FixedSizeTerminal :: < 3usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . then (choice ((leading_trivia_parser . clone () . then (just ('*') . to (FixedSizeTerminal :: < 1 > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . map (| v | Box :: new (using_directive :: Choices1 :: Star (v))) , type_name_parser . clone () . map (| v | Box :: new (using_directive :: Choices1 :: TypeName (v)))))) . then (leading_trivia_parser . clone () . then (terminal ("global") . to (FixedSizeTerminal :: < 6usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . or_not ()) . then (leading_trivia_parser . clone () . then (just (';') . to (FixedSizeTerminal :: < 1 > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | using_directive :: UsingDirective :: from_parse (v)) . boxed () ;

        // YulBlock = '{' { YulStatement } '}' ;
        yul_block_parser.define(
            leading_trivia_parser
                .clone()
                .then(just('{').to(FixedSizeTerminal::<1usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .then(yul_statement_parser.clone().repeated())
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('}').to(FixedSizeTerminal::<1usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .map(|v| yul_block::YulBlock::from_parse(v))
                .boxed(),
        );

        // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
        let assembly_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("assembly").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("\"evmasm\"").to(FixedSizeTerminal::<8usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .or_not(),
            )
            .then(assembly_flags_parser.clone().or_not())
            .then(yul_block_parser.clone())
            .map(|v| assembly_statement::AssemblyStatement::from_parse(v))
            .boxed();

        // Directive = PragmaDirective | ImportDirective | UsingDirective ;
        let directive_parser = choice((
            pragma_directive_parser
                .clone()
                .map(|v| Box::new(directive::Directive::PragmaDirective(v))),
            import_directive_parser
                .clone()
                .map(|v| Box::new(directive::Directive::ImportDirective(v))),
            using_directive_parser
                .clone()
                .map(|v| Box::new(directive::Directive::UsingDirective(v))),
        ))
        .boxed();

        // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
        let error_definition_parser = leading_trivia_parser . clone () . then (terminal ("error") . to (FixedSizeTerminal :: < 5usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . then (leading_trivia_parser . clone () . then (just ('(') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (error_parameter_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (error_parameter_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| (error_parameter_repeated , comma_repeated) | error_definition :: ErrorParameterRepeatedAndCommaRepeated { error_parameter_repeated , comma_repeated }) . or_not ()) . then (leading_trivia_parser . clone () . then (just (')') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | error_definition :: OpenParenAndErrorParameterRepeatedAndCommaRepeatedAndCloseParen :: from_parse (v))) . then (leading_trivia_parser . clone () . then (just (';') . to (FixedSizeTerminal :: < 1 > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | error_definition :: ErrorDefinition :: from_parse (v)) . boxed () ;

        // EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
        let event_definition_parser = leading_trivia_parser . clone () . then (terminal ("event") . to (FixedSizeTerminal :: < 5usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . then (leading_trivia_parser . clone () . then (just ('(') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (event_parameter_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (event_parameter_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| (event_parameter_repeated , comma_repeated) | event_definition :: EventParameterRepeatedAndCommaRepeated { event_parameter_repeated , comma_repeated }) . or_not ()) . then (leading_trivia_parser . clone () . then (just (')') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | event_definition :: OpenParenAndEventParameterRepeatedAndCommaRepeatedAndCloseParen :: from_parse (v))) . then (leading_trivia_parser . clone () . then (terminal ("anonymous") . to (FixedSizeTerminal :: < 9usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . or_not ()) . then (leading_trivia_parser . clone () . then (just (';') . to (FixedSizeTerminal :: < 1 > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | event_definition :: EventDefinition :: from_parse (v)) . boxed () ;

        // PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | «NumericLiteral» | «BooleanLiteral» | «Identifier» ;
        let primary_expression_parser = choice((
            payable_expression_parser
                .clone()
                .map(|v| Box::new(primary_expression::PrimaryExpression::PayableExpression(v))),
            type_expression_parser
                .clone()
                .map(|v| Box::new(primary_expression::PrimaryExpression::TypeExpression(v))),
            new_expression_parser
                .clone()
                .map(|v| Box::new(primary_expression::PrimaryExpression::NewExpression(v))),
            parenthesis_expression_parser
                .clone()
                .map(|v| Box::new(primary_expression::PrimaryExpression::ParenthesisExpression(v))),
            array_literal_parser
                .clone()
                .map(|v| Box::new(primary_expression::PrimaryExpression::ArrayLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(ascii_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    ascii_string_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| Box::new(primary_expression::PrimaryExpression::AsciiStringLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(unicode_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    unicode_string_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| {
                    Box::new(primary_expression::PrimaryExpression::UnicodeStringLiteral(
                        v,
                    ))
                }),
            leading_trivia_parser
                .clone()
                .then(hex_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| {
                    hex_string_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    }
                })
                .map(|v| Box::new(primary_expression::PrimaryExpression::HexStringLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(numeric_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| numeric_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(primary_expression::PrimaryExpression::NumericLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(boolean_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| boolean_literal::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(primary_expression::PrimaryExpression::BooleanLiteral(v))),
            leading_trivia_parser
                .clone()
                .then(identifier_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .map(|v| Box::new(primary_expression::PrimaryExpression::Identifier(v))),
        ))
        .map(|v| Box::new(expression::Expression::PrimaryExpression(v)))
        .boxed();

        // StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
        let struct_definition_parser = leading_trivia_parser . clone () . then (terminal ("struct") . to (FixedSizeTerminal :: < 6usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . then (leading_trivia_parser . clone () . then (just ('{') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (struct_member_parser . clone () . repeated () . at_least (1usize)) . then (leading_trivia_parser . clone () . then (just ('}') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | struct_definition :: OpenBraceAndStructMemberRepeatedAndCloseBrace :: from_parse (v))) . map (| v | struct_definition :: StructDefinition :: from_parse (v)) . boxed () ;

        // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
        let index_access_expression_parser = choice ((expression_parser . clone () . then (leading_trivia_parser . clone () . then (just ('[') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (expression_parser . clone () . or_not () . then (leading_trivia_parser . clone () . then (just (':') . to (FixedSizeTerminal :: < 1 > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (expression_parser . clone () . or_not ()) . map (| v | index_access_expression :: Sequence1 :: from_parse (v)) . or_not ()) . map (| v | index_access_expression :: Sequence0 :: from_parse (v))) . then (leading_trivia_parser . clone () . then (just (']') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | index_access_expression :: OpenBracketAndSequence0AndCloseBracket :: from_parse (v))) . map (| v | index_access_expression :: Anonexpfrag3 :: from_parse (v)) . map (| v | Box :: new (expression :: Expression :: IndexAccessExpression (v))) , primary_expression_parser . clone ())) . boxed () ;

        // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
        let member_access_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('.').to(FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .then(choice((
                    leading_trivia_parser
                        .clone()
                        .then(identifier_parser.clone())
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            identifier::WithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(member_access_expression::Choices0::Identifier(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("address").to(FixedSizeTerminal::<7usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(member_access_expression::Choices0::Address(v))),
                )))
                .map(|v| member_access_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::MemberAccessExpression(v))),
            index_access_expression_parser.clone(),
        ))
        .boxed();

        // FunctionCallExpression = Expression [ '{' NamedArgument  { ',' NamedArgument } '}' ] ArgumentList ;
        let function_call_expression_parser = choice ((expression_parser . clone () . then (leading_trivia_parser . clone () . then (just ('{') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (named_argument_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (named_argument_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| (named_argument_repeated , comma_repeated) | function_call_expression :: NamedArgumentRepeatedAndCommaRepeated { named_argument_repeated , comma_repeated })) . then (leading_trivia_parser . clone () . then (just ('}') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | function_call_expression :: OpenBraceAndNamedArgumentRepeatedAndCommaRepeatedAndCloseBrace :: from_parse (v)) . or_not ()) . then (argument_list_parser . clone ()) . map (| v | function_call_expression :: Anonexpfrag4 :: from_parse (v)) . map (| v | Box :: new (expression :: Expression :: FunctionCallExpression (v))) , member_access_expression_parser . clone ())) . boxed () ;

        // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
        let unary_prefix_expression_parser = choice((
            choice((
                leading_trivia_parser
                    .clone()
                    .then(terminal("++").to(FixedSizeTerminal::<2usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(unary_prefix_expression::Choices0::PlusPlus(v))),
                leading_trivia_parser
                    .clone()
                    .then(terminal("--").to(FixedSizeTerminal::<2usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(unary_prefix_expression::Choices0::MinusMinus(v))),
                leading_trivia_parser
                    .clone()
                    .then(just('!').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(unary_prefix_expression::Choices0::Bang(v))),
                leading_trivia_parser
                    .clone()
                    .then(just('~').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(unary_prefix_expression::Choices0::Tilde(v))),
                leading_trivia_parser
                    .clone()
                    .then(just('-').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(unary_prefix_expression::Choices0::Minus(v))),
            ))
            .then(expression_parser.clone())
            .map(|v| unary_prefix_expression::Anonexpfrag3::from_parse(v))
            .map(|v| Box::new(expression::Expression::UnaryPrefixExpression(v))),
            function_call_expression_parser.clone(),
        ))
        .boxed();

        // UnarySuffixExpression = Expression ( '++' | '--' ) ;
        let unary_suffix_expression_parser = choice((
            expression_parser
                .clone()
                .then(choice((
                    leading_trivia_parser
                        .clone()
                        .then(terminal("++").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(unary_suffix_expression::Choices0::PlusPlus(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("--").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(unary_suffix_expression::Choices0::MinusMinus(v))),
                )))
                .map(|v| unary_suffix_expression::Anonexpfrag3::from_parse(v))
                .map(|v| Box::new(expression::Expression::UnarySuffixExpression(v))),
            unary_prefix_expression_parser.clone(),
        ))
        .boxed();

        // ExponentiationExpression = Expression '**' Expression ;
        let exponentiation_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(terminal("**").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .then(expression_parser.clone())
                .map(|v| exponentiation_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::ExponentiationExpression(v))),
            unary_suffix_expression_parser.clone(),
        ))
        .boxed();

        // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
        let mul_div_mod_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(
                            filter(|&c: &char| c == '*' || c == '/' || c == '%')
                                .to(FixedSizeTerminal::<1>()),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .then(expression_parser.clone())
                .map(|v| mul_div_mod_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::MulDivModExpression(v))),
            exponentiation_expression_parser.clone(),
        ))
        .boxed();

        // AddSubExpression = Expression ( '+' | '-' ) Expression ;
        let add_sub_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(filter(|&c: &char| c == '+' || c == '-').to(FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .then(expression_parser.clone())
                .map(|v| add_sub_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::AddSubExpression(v))),
            mul_div_mod_expression_parser.clone(),
        ))
        .boxed();

        // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
        let shift_expression_parser = choice((
            expression_parser
                .clone()
                .then(choice((
                    leading_trivia_parser
                        .clone()
                        .then(terminal("<<").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(shift_expression::Choices0::LessLess(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal(">>").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(shift_expression::Choices0::GreaterGreater(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal(">>>").to(FixedSizeTerminal::<3usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(shift_expression::Choices0::GreaterGreaterGreater(v))),
                )))
                .then(expression_parser.clone())
                .map(|v| shift_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::ShiftExpression(v))),
            add_sub_expression_parser.clone(),
        ))
        .boxed();

        // BitAndExpression = Expression '&' Expression ;
        let bit_and_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('&').to(FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .then(expression_parser.clone())
                .map(|v| bit_and_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::BitAndExpression(v))),
            shift_expression_parser.clone(),
        ))
        .boxed();

        // BitXOrExpression = Expression '^' Expression ;
        let bit_x_or_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('^').to(FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .then(expression_parser.clone())
                .map(|v| bit_x_or_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::BitXOrExpression(v))),
            bit_and_expression_parser.clone(),
        ))
        .boxed();

        // BitOrExpression = Expression '|' Expression ;
        let bit_or_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('|').to(FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .then(expression_parser.clone())
                .map(|v| bit_or_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::BitOrExpression(v))),
            bit_x_or_expression_parser.clone(),
        ))
        .boxed();

        // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
        let order_comparison_expression_parser = choice((
            expression_parser
                .clone()
                .then(choice((
                    leading_trivia_parser
                        .clone()
                        .then(just('<').to(FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(order_comparison_expression::Choices0::Less(v))),
                    leading_trivia_parser
                        .clone()
                        .then(just('>').to(FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(order_comparison_expression::Choices0::Greater(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("<=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(order_comparison_expression::Choices0::LessEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal(">=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(order_comparison_expression::Choices0::GreaterEqual(v))),
                )))
                .then(expression_parser.clone())
                .map(|v| order_comparison_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::OrderComparisonExpression(v))),
            bit_or_expression_parser.clone(),
        ))
        .boxed();

        // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
        let equality_comparison_expression_parser = choice((
            expression_parser
                .clone()
                .then(choice((
                    leading_trivia_parser
                        .clone()
                        .then(terminal("==").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(equality_comparison_expression::Choices0::EqualEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("!=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(equality_comparison_expression::Choices0::BangEqual(v))),
                )))
                .then(expression_parser.clone())
                .map(|v| equality_comparison_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::EqualityComparisonExpression(v))),
            order_comparison_expression_parser.clone(),
        ))
        .boxed();

        // AndExpression = Expression '&&' Expression ;
        let and_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(terminal("&&").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .then(expression_parser.clone())
                .map(|v| and_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::AndExpression(v))),
            equality_comparison_expression_parser.clone(),
        ))
        .boxed();

        // OrExpression = Expression '||' Expression ;
        let or_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(terminal("||").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .then(expression_parser.clone())
                .map(|v| or_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::OrExpression(v))),
            and_expression_parser.clone(),
        ))
        .boxed();

        // ConditionalExpression = Expression '?' Expression ':' Expression ;
        let conditional_expression_parser = choice((
            expression_parser
                .clone()
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('?').to(FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .then(expression_parser.clone())
                        .then(
                            leading_trivia_parser
                                .clone()
                                .then(just(':').to(FixedSizeTerminal::<1>()))
                                .then(trailing_trivia_parser.clone())
                                .map(|((leading_trivia, terminal), trailing_trivia)| {
                                    FixedSizeTerminalWithTrivia {
                                        leading_trivia,
                                        terminal,
                                        trailing_trivia,
                                    }
                                }),
                        )
                        .then(expression_parser.clone())
                        .map(|v| conditional_expression::Sequence0::from_parse(v)),
                )
                .map(|v| conditional_expression::Anonexpfrag3::from_parse(v))
                .map(|v| Box::new(expression::Expression::ConditionalExpression(v))),
            or_expression_parser.clone(),
        ))
        .boxed();

        // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
        let assignment_expression_parser = choice((
            expression_parser
                .clone()
                .then(choice((
                    leading_trivia_parser
                        .clone()
                        .then(just('=').to(FixedSizeTerminal::<1>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::Equal(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("|=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::PipeEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("^=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::CaretEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("&=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::AmpersandEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("<<=").to(FixedSizeTerminal::<3usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::LessLessEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal(">>=").to(FixedSizeTerminal::<3usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::GreaterGreaterEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal(">>>=").to(FixedSizeTerminal::<4usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| {
                            Box::new(assignment_expression::Choices0::GreaterGreaterGreaterEqual(
                                v,
                            ))
                        }),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("+=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::PlusEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("-=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::MinusEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("*=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::StarEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("/=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::SlashEqual(v))),
                    leading_trivia_parser
                        .clone()
                        .then(terminal("%=").to(FixedSizeTerminal::<2usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        })
                        .map(|v| Box::new(assignment_expression::Choices0::PercentEqual(v))),
                )))
                .then(expression_parser.clone())
                .map(|v| assignment_expression::Anonexpfrag4::from_parse(v))
                .map(|v| Box::new(expression::Expression::AssignmentExpression(v))),
            conditional_expression_parser.clone(),
        ))
        .boxed();

        // Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
        expression_parser.define(assignment_expression_parser.clone().boxed());

        // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
        let constant_definition_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("constant").to(FixedSizeTerminal::<8usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('=').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .then(expression_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| constant_definition::ConstantDefinition::from_parse(v))
            .boxed();

        // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
        let do_while_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("do").to(FixedSizeTerminal::<2usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(statement_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("while").to(FixedSizeTerminal::<5usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(expression_parser.clone())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(')').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| {
                        do_while_statement::OpenParenAndExpressionAndCloseParen::from_parse(v)
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| do_while_statement::DoWhileStatement::from_parse(v))
            .boxed();

        // ExpressionStatement = Expression ';' ;
        let expression_statement_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| expression_statement::ExpressionStatement::from_parse(v))
            .boxed();

        // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
        let if_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("if").to(FixedSizeTerminal::<2usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(expression_parser.clone())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(')').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| if_statement::OpenParenAndExpressionAndCloseParen::from_parse(v)),
            )
            .then(statement_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("else").to(FixedSizeTerminal::<4usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(statement_parser.clone())
                    .map(|v| if_statement::Sequence0::from_parse(v))
                    .or_not(),
            )
            .map(|v| if_statement::IfStatement::from_parse(v))
            .boxed();

        // ReturnStatement = 'return' [ Expression ] ';' ;
        let return_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("return").to(FixedSizeTerminal::<6usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(expression_parser.clone().or_not())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| return_statement::ReturnStatement::from_parse(v))
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
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('=').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(expression_parser.clone())
                    .map(|v| state_variable_declaration::Sequence0::from_parse(v))
                    .or_not(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| state_variable_declaration::StateVariableDeclaration::from_parse(v))
            .boxed();

        // TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
        let try_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("try").to(FixedSizeTerminal::<3usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(expression_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("returns").to(FixedSizeTerminal::<7usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(parameter_list_parser.clone())
                    .map(|v| try_statement::Sequence0::from_parse(v))
                    .or_not(),
            )
            .then(block_parser.clone())
            .then(catch_clause_parser.clone().repeated().at_least(1usize))
            .map(|v| try_statement::TryStatement::from_parse(v))
            .boxed();

        // TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
        let tuple_deconstruction_statement_parser = leading_trivia_parser . clone () . then (just ('(') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (type_name_parser . clone () . or_not () . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | tuple_deconstruction_statement :: Sequence0 :: from_parse (v)) . or_not () . then (leading_trivia_parser . clone () . then (just (',') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (type_name_parser . clone () . or_not () . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | tuple_deconstruction_statement :: Sequence0 :: from_parse (v)) . or_not ()) . repeated ()) . map (repetition_mapper) . map (| (sequence_0_repeated , comma_repeated) | tuple_deconstruction_statement :: Sequence0RepeatedAndCommaRepeated { sequence_0_repeated , comma_repeated }) . or_not ()) . then (leading_trivia_parser . clone () . then (just (')') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | tuple_deconstruction_statement :: OpenParenAndSequence0RepeatedAndCommaRepeatedAndCloseParen :: from_parse (v)) . then (leading_trivia_parser . clone () . then (just ('=') . to (FixedSizeTerminal :: < 1 > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . then (expression_parser . clone ()) . then (leading_trivia_parser . clone () . then (just (';') . to (FixedSizeTerminal :: < 1 > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | tuple_deconstruction_statement :: TupleDeconstructionStatement :: from_parse (v)) . boxed () ;

        // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
        let variable_declaration_statement_parser = type_name_parser
            .clone()
            .then(data_location_parser.clone().or_not())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('=').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(expression_parser.clone())
                    .map(|v| variable_declaration_statement::Sequence0::from_parse(v))
                    .or_not(),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    }),
            )
            .map(|v| variable_declaration_statement::VariableDeclarationStatement::from_parse(v))
            .boxed();

        // WhileStatement = 'while' '(' Expression ')' Statement ;
        let while_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("while").to(FixedSizeTerminal::<5usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(expression_parser.clone())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(')').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| while_statement::OpenParenAndExpressionAndCloseParen::from_parse(v)),
            )
            .then(statement_parser.clone())
            .map(|v| while_statement::WhileStatement::from_parse(v))
            .boxed();

        // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
        let simple_statement_parser = choice((
            tuple_deconstruction_statement_parser.clone().map(|v| {
                Box::new(simple_statement::SimpleStatement::TupleDeconstructionStatement(v))
            }),
            variable_declaration_statement_parser.clone().map(|v| {
                Box::new(simple_statement::SimpleStatement::VariableDeclarationStatement(v))
            }),
            expression_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::SimpleStatement::ExpressionStatement(v))),
        ))
        .boxed();

        // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
        let for_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("for").to(FixedSizeTerminal::<3usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(FixedSizeTerminal::<1usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(
                        choice((
                            simple_statement_parser
                                .clone()
                                .map(|v| Box::new(for_statement::Choices1::SimpleStatement(v))),
                            leading_trivia_parser
                                .clone()
                                .then(just(';').to(FixedSizeTerminal::<1>()))
                                .then(trailing_trivia_parser.clone())
                                .map(|((leading_trivia, terminal), trailing_trivia)| {
                                    FixedSizeTerminalWithTrivia {
                                        leading_trivia,
                                        terminal,
                                        trailing_trivia,
                                    }
                                })
                                .map(|v| Box::new(for_statement::Choices1::Semicolon(v))),
                        ))
                        .then(choice((
                            expression_statement_parser
                                .clone()
                                .map(|v| Box::new(for_statement::Choices2::ExpressionStatement(v))),
                            leading_trivia_parser
                                .clone()
                                .then(just(';').to(FixedSizeTerminal::<1>()))
                                .then(trailing_trivia_parser.clone())
                                .map(|((leading_trivia, terminal), trailing_trivia)| {
                                    FixedSizeTerminalWithTrivia {
                                        leading_trivia,
                                        terminal,
                                        trailing_trivia,
                                    }
                                })
                                .map(|v| Box::new(for_statement::Choices2::Semicolon(v))),
                        )))
                        .then(expression_parser.clone().or_not())
                        .map(|v| for_statement::Sequence0::from_parse(v)),
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(')').to(FixedSizeTerminal::<1usize>()))
                            .then(trailing_trivia_parser.clone())
                            .map(|((leading_trivia, terminal), trailing_trivia)| {
                                FixedSizeTerminalWithTrivia {
                                    leading_trivia,
                                    terminal,
                                    trailing_trivia,
                                }
                            }),
                    )
                    .map(|v| for_statement::OpenParenAndSequence0AndCloseParen::from_parse(v)),
            )
            .then(statement_parser.clone())
            .map(|v| for_statement::ForStatement::from_parse(v))
            .boxed();

        // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
        statement_parser.define(
            choice((
                block_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::Block(v))),
                simple_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::SimpleStatement(v))),
                if_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::IfStatement(v))),
                for_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::ForStatement(v))),
                while_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::WhileStatement(v))),
                do_while_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::DoWhileStatement(v))),
                continue_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::ContinueStatement(v))),
                break_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::BreakStatement(v))),
                try_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::TryStatement(v))),
                return_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::ReturnStatement(v))),
                emit_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::EmitStatement(v))),
                revert_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::RevertStatement(v))),
                delete_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::DeleteStatement(v))),
                assembly_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::AssemblyStatement(v))),
            ))
            .boxed(),
        );

        // Block = '{' { Statement | UncheckedBlock } '}' ;
        block_parser.define(
            leading_trivia_parser
                .clone()
                .then(just('{').to(FixedSizeTerminal::<1usize>()))
                .then(trailing_trivia_parser.clone())
                .map(
                    |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                        leading_trivia,
                        terminal,
                        trailing_trivia,
                    },
                )
                .then(
                    choice((
                        statement_parser
                            .clone()
                            .map(|v| Box::new(block::Choices0::Statement(v))),
                        unchecked_block_parser
                            .clone()
                            .map(|v| Box::new(block::Choices0::UncheckedBlock(v))),
                    ))
                    .repeated(),
                )
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('}').to(FixedSizeTerminal::<1usize>()))
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)| {
                            FixedSizeTerminalWithTrivia {
                                leading_trivia,
                                terminal,
                                trailing_trivia,
                            }
                        }),
                )
                .map(|v| block::Block::from_parse(v))
                .boxed(),
        );

        // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
        let constructor_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("constructor").to(FixedSizeTerminal::<11usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(parameter_list_parser.clone())
            .then(constructor_attribute_parser.clone().repeated())
            .then(block_parser.clone())
            .map(|v| constructor_definition::ConstructorDefinition::from_parse(v))
            .boxed();

        // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
        let fallback_function_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("fallback").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(parameter_list_parser.clone())
            .then(fallback_function_attribute_parser.clone().repeated())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("returns").to(FixedSizeTerminal::<7usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(parameter_list_parser.clone())
                    .map(|v| fallback_function_definition::Sequence0::from_parse(v))
                    .or_not(),
            )
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(fallback_function_definition::Choices1::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(fallback_function_definition::Choices1::Block(v))),
            )))
            .map(|v| fallback_function_definition::FallbackFunctionDefinition::from_parse(v))
            .boxed();

        // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
        let function_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("function").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    )
                    .map(|v| Box::new(function_definition::Choices0::Identifier(v))),
                leading_trivia_parser
                    .clone()
                    .then(terminal("fallback").to(FixedSizeTerminal::<8usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(function_definition::Choices0::Fallback(v))),
                leading_trivia_parser
                    .clone()
                    .then(terminal("receive").to(FixedSizeTerminal::<7usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(function_definition::Choices0::Receive(v))),
            )))
            .then(parameter_list_parser.clone())
            .then(function_attribute_parser.clone().repeated())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("returns").to(FixedSizeTerminal::<7usize>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .then(parameter_list_parser.clone())
                    .map(|v| function_definition::Sequence1::from_parse(v))
                    .or_not(),
            )
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(function_definition::Choices2::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(function_definition::Choices2::Block(v))),
            )))
            .map(|v| function_definition::FunctionDefinition::from_parse(v))
            .boxed();

        // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
        let modifier_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("modifier").to(FixedSizeTerminal::<8usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(
                        |((leading_trivia, terminal), trailing_trivia)| identifier::WithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        },
                    ),
            )
            .then(parameter_list_parser.clone().or_not())
            .then(modifier_attribute_parser.clone().repeated())
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(modifier_definition::Choices0::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(modifier_definition::Choices0::Block(v))),
            )))
            .map(|v| modifier_definition::ModifierDefinition::from_parse(v))
            .boxed();

        // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
        let receive_function_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("receive").to(FixedSizeTerminal::<7usize>()))
            .then(trailing_trivia_parser.clone())
            .map(
                |((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia {
                    leading_trivia,
                    terminal,
                    trailing_trivia,
                },
            )
            .then(parameter_list_parser.clone())
            .then(receive_function_attribute_parser.clone().repeated())
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(FixedSizeTerminal::<1>()))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| {
                        FixedSizeTerminalWithTrivia {
                            leading_trivia,
                            terminal,
                            trailing_trivia,
                        }
                    })
                    .map(|v| Box::new(receive_function_definition::Choices0::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(receive_function_definition::Choices0::Block(v))),
            )))
            .map(|v| receive_function_definition::ReceiveFunctionDefinition::from_parse(v))
            .boxed();

        // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
        let contract_body_element_parser = choice((
            using_directive_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::UsingDirective(
                    v,
                ))
            }),
            constructor_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::ConstructorDefinition(v))
            }),
            function_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::FunctionDefinition(v))
            }),
            fallback_function_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::FallbackFunctionDefinition(v))
            }),
            receive_function_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::ReceiveFunctionDefinition(v))
            }),
            modifier_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::ModifierDefinition(v))
            }),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::ContractBodyElement::StructDefinition(v))),
            enum_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::EnumDefinition(
                    v,
                ))
            }),
            user_defined_value_type_definition_parser.clone().map(|v| {
                Box::new(
                    contract_body_element::ContractBodyElement::UserDefinedValueTypeDefinition(v),
                )
            }),
            event_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::EventDefinition(
                    v,
                ))
            }),
            error_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::ErrorDefinition(
                    v,
                ))
            }),
            state_variable_declaration_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::StateVariableDeclaration(v))
            }),
        ))
        .boxed();

        // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let contract_definition_parser = leading_trivia_parser . clone () . then (terminal ("abstract") . to (FixedSizeTerminal :: < 8usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . or_not () . then (leading_trivia_parser . clone () . then (terminal ("contract") . to (FixedSizeTerminal :: < 8usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . then (inheritance_specifier_list_parser . clone () . or_not ()) . then (leading_trivia_parser . clone () . then (just ('{') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (contract_body_element_parser . clone () . repeated ()) . then (leading_trivia_parser . clone () . then (just ('}') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | contract_definition :: OpenBraceAndContractBodyElementRepeatedAndCloseBrace :: from_parse (v))) . map (| v | contract_definition :: ContractDefinition :: from_parse (v)) . boxed () ;

        // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let interface_definition_parser = leading_trivia_parser . clone () . then (terminal ("interface") . to (FixedSizeTerminal :: < 9usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . then (inheritance_specifier_list_parser . clone () . or_not ()) . then (leading_trivia_parser . clone () . then (just ('{') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (contract_body_element_parser . clone () . repeated ()) . then (leading_trivia_parser . clone () . then (just ('}') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | interface_definition :: OpenBraceAndContractBodyElementRepeatedAndCloseBrace :: from_parse (v))) . map (| v | interface_definition :: InterfaceDefinition :: from_parse (v)) . boxed () ;

        // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
        let library_definition_parser = leading_trivia_parser . clone () . then (terminal ("library") . to (FixedSizeTerminal :: < 7usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | identifier :: WithTrivia { leading_trivia , terminal , trailing_trivia })) . then (leading_trivia_parser . clone () . then (just ('{') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia }) . then (contract_body_element_parser . clone () . repeated ()) . then (leading_trivia_parser . clone () . then (just ('}') . to (FixedSizeTerminal :: < 1usize > ())) . then (trailing_trivia_parser . clone ()) . map (| ((leading_trivia , terminal) , trailing_trivia) | FixedSizeTerminalWithTrivia { leading_trivia , terminal , trailing_trivia })) . map (| v | library_definition :: OpenBraceAndContractBodyElementRepeatedAndCloseBrace :: from_parse (v))) . map (| v | library_definition :: LibraryDefinition :: from_parse (v)) . boxed () ;

        // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
        let definition_parser = choice((
            contract_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::ContractDefinition(v))),
            interface_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::InterfaceDefinition(v))),
            library_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::LibraryDefinition(v))),
            function_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::FunctionDefinition(v))),
            constant_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::ConstantDefinition(v))),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::StructDefinition(v))),
            enum_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::EnumDefinition(v))),
            user_defined_value_type_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::UserDefinedValueTypeDefinition(v))),
            error_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::ErrorDefinition(v))),
        ))
        .boxed();

        // SourceUnit = «LeadingTrivia» { Directive | Definition } «EndOfFileTrivia» ;
        let source_unit_parser = leading_trivia_parser
            .clone()
            .then(
                choice((
                    directive_parser
                        .clone()
                        .map(|v| Box::new(source_unit::Choices0::Directive(v))),
                    definition_parser
                        .clone()
                        .map(|v| Box::new(source_unit::Choices0::Definition(v))),
                ))
                .repeated(),
            )
            .then(end_of_file_trivia_parser.clone())
            .map(|v| source_unit::SourceUnit::from_parse(v))
            .boxed();

        Self {
            boolean_literal: boolean_literal_parser.then_ignore(end()).boxed(),
            decimal_integer: decimal_integer_parser.then_ignore(end()).boxed(),
            end_of_line: end_of_line_parser.then_ignore(end()).boxed(),
            fixed_bytes_type: fixed_bytes_type_parser.then_ignore(end()).boxed(),
            hex_byte_escape: hex_byte_escape_parser.then_ignore(end()).boxed(),
            hex_number: hex_number_parser.then_ignore(end()).boxed(),
            multiline_comment: multiline_comment_parser.then_ignore(end()).boxed(),
            number_unit: number_unit_parser.then_ignore(end()).boxed(),
            possibly_separated_pairs_of_hex_digits: possibly_separated_pairs_of_hex_digits_parser
                .then_ignore(end())
                .boxed(),
            raw_identifier: raw_identifier_parser.then_ignore(end()).boxed(),
            reserved_keyword: reserved_keyword_parser.then_ignore(end()).boxed(),
            signed_fixed_type: signed_fixed_type_parser.then_ignore(end()).boxed(),
            signed_integer_type: signed_integer_type_parser.then_ignore(end()).boxed(),
            single_line_comment: single_line_comment_parser.then_ignore(end()).boxed(),
            unicode_escape: unicode_escape_parser.then_ignore(end()).boxed(),
            version_pragma_operator: version_pragma_operator_parser.then_ignore(end()).boxed(),
            version_pragma_value: version_pragma_value_parser.then_ignore(end()).boxed(),
            whitespace: whitespace_parser.then_ignore(end()).boxed(),
            yul_decimal_number_literal: yul_decimal_number_literal_parser
                .then_ignore(end())
                .boxed(),
            yul_hex_literal: yul_hex_literal_parser.then_ignore(end()).boxed(),
            decimal_exponent: decimal_exponent_parser.then_ignore(end()).boxed(),
            decimal_float: decimal_float_parser.then_ignore(end()).boxed(),
            end_of_file_trivia: end_of_file_trivia_parser.then_ignore(end()).boxed(),
            escape_sequence: escape_sequence_parser.then_ignore(end()).boxed(),
            hex_string_literal: hex_string_literal_parser.then_ignore(end()).boxed(),
            leading_trivia: leading_trivia_parser.then_ignore(end()).boxed(),
            trailing_trivia: trailing_trivia_parser.then_ignore(end()).boxed(),
            unsigned_fixed_type: unsigned_fixed_type_parser.then_ignore(end()).boxed(),
            unsigned_integer_type: unsigned_integer_type_parser.then_ignore(end()).boxed(),
            yul_keyword: yul_keyword_parser.then_ignore(end()).boxed(),
            address_type: address_type_parser.then_ignore(end()).boxed(),
            array_literal: array_literal_parser.then_ignore(end()).boxed(),
            break_statement: break_statement_parser.then_ignore(end()).boxed(),
            continue_statement: continue_statement_parser.then_ignore(end()).boxed(),
            data_location: data_location_parser.then_ignore(end()).boxed(),
            decimal_number: decimal_number_parser.then_ignore(end()).boxed(),
            double_quoted_ascii_string_literal: double_quoted_ascii_string_literal_parser
                .then_ignore(end())
                .boxed(),
            double_quoted_unicode_string_literal: double_quoted_unicode_string_literal_parser
                .then_ignore(end())
                .boxed(),
            keyword: keyword_parser.then_ignore(end()).boxed(),
            parenthesis_expression: parenthesis_expression_parser.then_ignore(end()).boxed(),
            positional_argument_list: positional_argument_list_parser.then_ignore(end()).boxed(),
            single_quoted_ascii_string_literal: single_quoted_ascii_string_literal_parser
                .then_ignore(end())
                .boxed(),
            single_quoted_unicode_string_literal: single_quoted_unicode_string_literal_parser
                .then_ignore(end())
                .boxed(),
            unchecked_block: unchecked_block_parser.then_ignore(end()).boxed(),
            version_pragma_specifier: version_pragma_specifier_parser.then_ignore(end()).boxed(),
            yul_break_statement: yul_break_statement_parser.then_ignore(end()).boxed(),
            yul_continue_statement: yul_continue_statement_parser.then_ignore(end()).boxed(),
            yul_identifier: yul_identifier_parser.then_ignore(end()).boxed(),
            yul_leave_statement: yul_leave_statement_parser.then_ignore(end()).boxed(),
            ascii_string_literal: ascii_string_literal_parser.then_ignore(end()).boxed(),
            assembly_flags: assembly_flags_parser.then_ignore(end()).boxed(),
            elementary_type: elementary_type_parser.then_ignore(end()).boxed(),
            identifier: identifier_parser.then_ignore(end()).boxed(),
            numeric_literal: numeric_literal_parser.then_ignore(end()).boxed(),
            unicode_string_literal: unicode_string_literal_parser.then_ignore(end()).boxed(),
            yul_function_call: yul_function_call_parser.then_ignore(end()).boxed(),
            yul_function_definition: yul_function_definition_parser.then_ignore(end()).boxed(),
            yul_identifier_path: yul_identifier_path_parser.then_ignore(end()).boxed(),
            abi_coder_pragma_specifier: abi_coder_pragma_specifier_parser
                .then_ignore(end())
                .boxed(),
            delete_statement: delete_statement_parser.then_ignore(end()).boxed(),
            enum_definition: enum_definition_parser.then_ignore(end()).boxed(),
            experimental_pragma_specifier: experimental_pragma_specifier_parser
                .then_ignore(end())
                .boxed(),
            identifier_path: identifier_path_parser.then_ignore(end()).boxed(),
            import_path: import_path_parser.then_ignore(end()).boxed(),
            named_argument: named_argument_parser.then_ignore(end()).boxed(),
            parameter_declaration: parameter_declaration_parser.then_ignore(end()).boxed(),
            selected_import: selected_import_parser.then_ignore(end()).boxed(),
            user_defined_value_type_definition: user_defined_value_type_definition_parser
                .then_ignore(end())
                .boxed(),
            yul_literal: yul_literal_parser.then_ignore(end()).boxed(),
            mapping_type: mapping_type_parser.then_ignore(end()).boxed(),
            named_argument_list: named_argument_list_parser.then_ignore(end()).boxed(),
            override_specifier: override_specifier_parser.then_ignore(end()).boxed(),
            parameter_list: parameter_list_parser.then_ignore(end()).boxed(),
            pragma_directive: pragma_directive_parser.then_ignore(end()).boxed(),
            selecting_import_directive: selecting_import_directive_parser
                .then_ignore(end())
                .boxed(),
            simple_import_directive: simple_import_directive_parser.then_ignore(end()).boxed(),
            star_import_directive: star_import_directive_parser.then_ignore(end()).boxed(),
            yul_expression: yul_expression_parser.then_ignore(end()).boxed(),
            argument_list: argument_list_parser.then_ignore(end()).boxed(),
            catch_clause: catch_clause_parser.then_ignore(end()).boxed(),
            function_type: function_type_parser.then_ignore(end()).boxed(),
            import_directive: import_directive_parser.then_ignore(end()).boxed(),
            modifier_attribute: modifier_attribute_parser.then_ignore(end()).boxed(),
            state_variable_attribute: state_variable_attribute_parser.then_ignore(end()).boxed(),
            yul_assignment_statement: yul_assignment_statement_parser.then_ignore(end()).boxed(),
            yul_for_statement: yul_for_statement_parser.then_ignore(end()).boxed(),
            yul_if_statement: yul_if_statement_parser.then_ignore(end()).boxed(),
            yul_switch_statement: yul_switch_statement_parser.then_ignore(end()).boxed(),
            yul_variable_declaration: yul_variable_declaration_parser.then_ignore(end()).boxed(),
            emit_statement: emit_statement_parser.then_ignore(end()).boxed(),
            inheritance_specifier: inheritance_specifier_parser.then_ignore(end()).boxed(),
            modifier_invocation: modifier_invocation_parser.then_ignore(end()).boxed(),
            new_expression: new_expression_parser.then_ignore(end()).boxed(),
            payable_expression: payable_expression_parser.then_ignore(end()).boxed(),
            revert_statement: revert_statement_parser.then_ignore(end()).boxed(),
            type_name: type_name_parser.then_ignore(end()).boxed(),
            yul_statement: yul_statement_parser.then_ignore(end()).boxed(),
            constructor_attribute: constructor_attribute_parser.then_ignore(end()).boxed(),
            error_parameter: error_parameter_parser.then_ignore(end()).boxed(),
            event_parameter: event_parameter_parser.then_ignore(end()).boxed(),
            fallback_function_attribute: fallback_function_attribute_parser
                .then_ignore(end())
                .boxed(),
            function_attribute: function_attribute_parser.then_ignore(end()).boxed(),
            inheritance_specifier_list: inheritance_specifier_list_parser
                .then_ignore(end())
                .boxed(),
            receive_function_attribute: receive_function_attribute_parser
                .then_ignore(end())
                .boxed(),
            struct_member: struct_member_parser.then_ignore(end()).boxed(),
            type_expression: type_expression_parser.then_ignore(end()).boxed(),
            using_directive: using_directive_parser.then_ignore(end()).boxed(),
            yul_block: yul_block_parser.then_ignore(end()).boxed(),
            assembly_statement: assembly_statement_parser.then_ignore(end()).boxed(),
            directive: directive_parser.then_ignore(end()).boxed(),
            error_definition: error_definition_parser.then_ignore(end()).boxed(),
            event_definition: event_definition_parser.then_ignore(end()).boxed(),
            primary_expression: primary_expression_parser.then_ignore(end()).boxed(),
            struct_definition: struct_definition_parser.then_ignore(end()).boxed(),
            index_access_expression: index_access_expression_parser.then_ignore(end()).boxed(),
            member_access_expression: member_access_expression_parser.then_ignore(end()).boxed(),
            function_call_expression: function_call_expression_parser.then_ignore(end()).boxed(),
            unary_prefix_expression: unary_prefix_expression_parser.then_ignore(end()).boxed(),
            unary_suffix_expression: unary_suffix_expression_parser.then_ignore(end()).boxed(),
            exponentiation_expression: exponentiation_expression_parser.then_ignore(end()).boxed(),
            mul_div_mod_expression: mul_div_mod_expression_parser.then_ignore(end()).boxed(),
            add_sub_expression: add_sub_expression_parser.then_ignore(end()).boxed(),
            shift_expression: shift_expression_parser.then_ignore(end()).boxed(),
            bit_and_expression: bit_and_expression_parser.then_ignore(end()).boxed(),
            bit_x_or_expression: bit_x_or_expression_parser.then_ignore(end()).boxed(),
            bit_or_expression: bit_or_expression_parser.then_ignore(end()).boxed(),
            order_comparison_expression: order_comparison_expression_parser
                .then_ignore(end())
                .boxed(),
            equality_comparison_expression: equality_comparison_expression_parser
                .then_ignore(end())
                .boxed(),
            and_expression: and_expression_parser.then_ignore(end()).boxed(),
            or_expression: or_expression_parser.then_ignore(end()).boxed(),
            conditional_expression: conditional_expression_parser.then_ignore(end()).boxed(),
            assignment_expression: assignment_expression_parser.then_ignore(end()).boxed(),
            expression: expression_parser.then_ignore(end()).boxed(),
            constant_definition: constant_definition_parser.then_ignore(end()).boxed(),
            do_while_statement: do_while_statement_parser.then_ignore(end()).boxed(),
            expression_statement: expression_statement_parser.then_ignore(end()).boxed(),
            if_statement: if_statement_parser.then_ignore(end()).boxed(),
            return_statement: return_statement_parser.then_ignore(end()).boxed(),
            state_variable_declaration: state_variable_declaration_parser
                .then_ignore(end())
                .boxed(),
            try_statement: try_statement_parser.then_ignore(end()).boxed(),
            tuple_deconstruction_statement: tuple_deconstruction_statement_parser
                .then_ignore(end())
                .boxed(),
            variable_declaration_statement: variable_declaration_statement_parser
                .then_ignore(end())
                .boxed(),
            while_statement: while_statement_parser.then_ignore(end()).boxed(),
            simple_statement: simple_statement_parser.then_ignore(end()).boxed(),
            for_statement: for_statement_parser.then_ignore(end()).boxed(),
            statement: statement_parser.then_ignore(end()).boxed(),
            block: block_parser.then_ignore(end()).boxed(),
            constructor_definition: constructor_definition_parser.then_ignore(end()).boxed(),
            fallback_function_definition: fallback_function_definition_parser
                .then_ignore(end())
                .boxed(),
            function_definition: function_definition_parser.then_ignore(end()).boxed(),
            modifier_definition: modifier_definition_parser.then_ignore(end()).boxed(),
            receive_function_definition: receive_function_definition_parser
                .then_ignore(end())
                .boxed(),
            contract_body_element: contract_body_element_parser.then_ignore(end()).boxed(),
            contract_definition: contract_definition_parser.then_ignore(end()).boxed(),
            interface_definition: interface_definition_parser.then_ignore(end()).boxed(),
            library_definition: library_definition_parser.then_ignore(end()).boxed(),
            definition: definition_parser.then_ignore(end()).boxed(),
            source_unit: source_unit_parser.then_ignore(end()).boxed(),
        }
    }
}
