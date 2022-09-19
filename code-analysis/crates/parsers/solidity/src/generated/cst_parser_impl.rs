// This file is generated via `cargo build`. Please don't edit by hand.

use super::cst::*;
use super::cst_parser::*;
use chumsky::prelude::*;
use chumsky::primitive::Just;
use chumsky::Parser;
#[allow(dead_code)]
fn repetition_mapper((e, es): (NodeRef, Vec<(NodeRef, NodeRef)>)) -> Vec<NodeRef> {
    let mut result = vec![e];
    for (s, e) in es.into_iter() {
        result.push(s);
        result.push(e);
    }
    result
}
#[allow(dead_code)]
fn difference<M, S>(minuend: M, subtrahend: S) -> impl Parser<char, NodeRef, Error = ErrorType>
where
    M: Clone + Parser<char, NodeRef, Error = ErrorType>,
    S: Parser<char, NodeRef, Error = ErrorType>,
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
            terminal("true").to(Node::new_token_part(TokenPartKind::True, 4usize)),
            terminal("false").to(Node::new_token_part(TokenPartKind::False, 5usize)),
        ))
        .map(|v| Node::new_rule(RuleKind::BooleanLiteral, vec![v]))
        .boxed();

        // «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
        let decimal_integer_parser = filter(|&c: &char| ('0' <= c && c <= '9'))
            .to(Node::new_anonymous_token(1))
            .then(
                just('_')
                    .to(Node::new_token_part(
                        TokenPartKind::DecimalIntegerUnderscore,
                        1,
                    ))
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none()))
                    .then(
                        filter(|&c: &char| ('0' <= c && c <= '9')).to(Node::new_anonymous_token(1)),
                    )
                    .map(|(underscore, filter_2)| {
                        Node::new_anonymous_rule(vec![underscore, filter_2])
                    })
                    .repeated()
                    .map(Node::new_anonymous_rule),
            )
            .map(|(filter_0, sequence_1_repeated)| {
                Node::new_rule(
                    RuleKind::DecimalInteger,
                    vec![filter_0, sequence_1_repeated],
                )
            })
            .boxed();

        // «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
        let end_of_line_parser = filter(|&c: &char| c == '\r' || c == '\n')
            .to(Node::new_anonymous_token(1))
            .repeated()
            .at_least(1usize)
            .map(|v| Node::new_token(TokenKind::EndOfLine, v))
            .boxed();

        // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
        let fixed_bytes_type_parser = terminal("bytes")
            .to(Node::new_token_part(TokenPartKind::Bytes, 5usize))
            .then(choice::<_, ErrorType>((
                terminal("1").ignore_then(choice((
                    terminal("0").to(Node::new_token_part(TokenPartKind::OneZero, 2usize)),
                    terminal("1").to(Node::new_token_part(TokenPartKind::OneOne, 2usize)),
                    terminal("2").to(Node::new_token_part(TokenPartKind::OneTwo, 2usize)),
                    terminal("3").to(Node::new_token_part(TokenPartKind::OneThree, 2usize)),
                    terminal("4").to(Node::new_token_part(TokenPartKind::OneFour, 2usize)),
                    terminal("5").to(Node::new_token_part(TokenPartKind::OneFive, 2usize)),
                    terminal("6").to(Node::new_token_part(TokenPartKind::OneSix, 2usize)),
                    terminal("7").to(Node::new_token_part(TokenPartKind::OneSeven, 2usize)),
                    terminal("8").to(Node::new_token_part(TokenPartKind::OneEight, 2usize)),
                    terminal("9").to(Node::new_token_part(TokenPartKind::OneNine, 2usize)),
                    empty().to(Node::new_token_part(TokenPartKind::One, 1usize)),
                ))),
                terminal("2").ignore_then(choice((
                    terminal("0").to(Node::new_token_part(TokenPartKind::TwoZero, 2usize)),
                    terminal("1").to(Node::new_token_part(TokenPartKind::TwoOne, 2usize)),
                    terminal("2").to(Node::new_token_part(TokenPartKind::TwoTwo, 2usize)),
                    terminal("3").to(Node::new_token_part(TokenPartKind::TwoThree, 2usize)),
                    terminal("4").to(Node::new_token_part(TokenPartKind::TwoFour, 2usize)),
                    terminal("5").to(Node::new_token_part(TokenPartKind::TwoFive, 2usize)),
                    terminal("6").to(Node::new_token_part(TokenPartKind::TwoSix, 2usize)),
                    terminal("7").to(Node::new_token_part(TokenPartKind::TwoSeven, 2usize)),
                    terminal("8").to(Node::new_token_part(TokenPartKind::TwoEight, 2usize)),
                    terminal("9").to(Node::new_token_part(TokenPartKind::TwoNine, 2usize)),
                    empty().to(Node::new_token_part(TokenPartKind::Two, 1usize)),
                ))),
                terminal("3").ignore_then(choice((
                    terminal("0").to(Node::new_token_part(TokenPartKind::ThreeZero, 2usize)),
                    terminal("1").to(Node::new_token_part(TokenPartKind::ThreeOne, 2usize)),
                    terminal("2").to(Node::new_token_part(TokenPartKind::ThreeTwo, 2usize)),
                    empty().to(Node::new_token_part(TokenPartKind::Three, 1usize)),
                ))),
                terminal("4").to(Node::new_token_part(TokenPartKind::Four, 1usize)),
                terminal("5").to(Node::new_token_part(TokenPartKind::Five, 1usize)),
                terminal("6").to(Node::new_token_part(TokenPartKind::Six, 1usize)),
                terminal("7").to(Node::new_token_part(TokenPartKind::Seven, 1usize)),
                terminal("8").to(Node::new_token_part(TokenPartKind::Eight, 1usize)),
                terminal("9").to(Node::new_token_part(TokenPartKind::Nine, 1usize)),
            )))
            .map(|(bytes, count)| Node::new_rule(RuleKind::FixedBytesType, vec![bytes, count]))
            .boxed();

        // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
        let hex_byte_escape_parser = just('x')
            .to(Node::new_token_part(
                TokenPartKind::HexByteEscapeLatinSmallLetterX,
                1,
            ))
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .to(Node::new_anonymous_token(1))
                .repeated()
                .exactly(2usize)
                .map(|v| Node::new_anonymous_token(v.len())),
            )
            .map(|(latin_small_letter_x, filter_0_repeated)| {
                Node::new_rule(
                    RuleKind::HexByteEscape,
                    vec![latin_small_letter_x, filter_0_repeated],
                )
            })
            .boxed();

        // «HexNumber» = '0x' «HexCharacter» { [ '_' ] «HexCharacter» } ;
        let hex_number_parser = terminal("0x")
            .to(Node::new_token_part(TokenPartKind::ZeroX, 2usize))
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .to(Node::new_anonymous_token(1))
                .then(
                    just('_')
                        .to(Node::new_token_part(TokenPartKind::HexNumberUnderscore, 1))
                        .or_not()
                        .map(|v| v.unwrap_or_else(|| Node::new_none()))
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')
                            })
                            .to(Node::new_anonymous_token(1)),
                        )
                        .map(|(underscore, filter_3)| {
                            Node::new_anonymous_rule(vec![underscore, filter_3])
                        })
                        .repeated()
                        .map(Node::new_anonymous_rule),
                )
                .map(|(filter_1, sequence_2_repeated)| {
                    Node::new_anonymous_rule(vec![filter_1, sequence_2_repeated])
                }),
            )
            .map(|(zero_x, sequence_0)| {
                Node::new_rule(RuleKind::HexNumber, vec![zero_x, sequence_0])
            })
            .boxed();

        // «MultilineComment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
        let multiline_comment_parser = just("/*")
            .to(Node::new_token_part(TokenPartKind::SlashStar, 2usize))
            .then(
                choice((
                    filter(|&c: &char| c != '*').to(Node::new_token_part(
                        TokenPartKind::MultilineCommentNotStar,
                        1,
                    )),
                    just('*')
                        .to(Node::new_token_part(TokenPartKind::MultilineCommentStar, 1))
                        .repeated()
                        .at_least(1usize)
                        .map(|v| Node::new_token(TokenKind::MultilineCommentStarRepeated, v))
                        .then(
                            filter(|&c: &char| c != '*' && c != '/')
                                .to(Node::new_anonymous_token(1)),
                        )
                        .map(|(star_repeated, filter_2)| {
                            Node::new_anonymous_rule(vec![star_repeated, filter_2])
                        }),
                ))
                .repeated()
                .map(Node::new_anonymous_rule)
                .then(
                    just('*')
                        .to(Node::new_token_part(TokenPartKind::MultilineCommentStar, 1))
                        .repeated()
                        .map(|v| {
                            if v.is_empty() {
                                Node::new_none()
                            } else {
                                Node::new_token(TokenKind::MultilineCommentStarRepeated, v)
                            }
                        }),
                )
                .map(|(choices_0_repeated, star_repeated)| {
                    Node::new_rule(
                        RuleKind::MultilineCommentContent,
                        vec![choices_0_repeated, star_repeated],
                    )
                }),
            )
            .then(just("*/").to(Node::new_token_part(TokenPartKind::StarSlash, 2usize)))
            .map(|((a, b), c)| Node::new_rule(RuleKind::MultilineComment, vec![a, b, c]))
            .boxed();

        // «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
        let number_unit_parser = choice((
            terminal("days").to(Node::new_token_part(TokenPartKind::Days, 4usize)),
            terminal("ether").to(Node::new_token_part(TokenPartKind::Ether, 5usize)),
            terminal("finney").to(Node::new_token_part(TokenPartKind::Finney, 6usize)),
            terminal("gwei").to(Node::new_token_part(TokenPartKind::Gwei, 4usize)),
            terminal("hours").to(Node::new_token_part(TokenPartKind::Hours, 5usize)),
            terminal("minutes").to(Node::new_token_part(TokenPartKind::Minutes, 7usize)),
            terminal("seconds").to(Node::new_token_part(TokenPartKind::Seconds, 7usize)),
            terminal("szabo").to(Node::new_token_part(TokenPartKind::Szabo, 5usize)),
            terminal("weeks").to(Node::new_token_part(TokenPartKind::Weeks, 5usize)),
            terminal("wei").to(Node::new_token_part(TokenPartKind::Wei, 3usize)),
            terminal("years").to(Node::new_token_part(TokenPartKind::Years, 5usize)),
        ))
        .map(|v| Node::new_rule(RuleKind::NumberUnit, vec![v]))
        .boxed();

        // «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
        let possibly_separated_pairs_of_hex_digits_parser = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        })
        .to(Node::new_anonymous_token(1))
        .repeated()
        .exactly(2usize)
        .map(|v| Node::new_anonymous_token(v.len()))
        .then(
            just('_')
                .to(Node::new_token_part(
                    TokenPartKind::PossiblySeparatedPairsOfHexDigitsUnderscore,
                    1,
                ))
                .or_not()
                .map(|v| v.unwrap_or_else(|| Node::new_none()))
                .then(
                    filter(|&c: &char| {
                        ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                    })
                    .to(Node::new_anonymous_token(1))
                    .repeated()
                    .exactly(2usize)
                    .map(|v| Node::new_anonymous_token(v.len())),
                )
                .map(|(underscore, filter_2_repeated)| {
                    Node::new_anonymous_rule(vec![underscore, filter_2_repeated])
                })
                .repeated()
                .map(Node::new_anonymous_rule),
        )
        .map(|(filter_0_repeated, sequence_1_repeated)| {
            Node::new_rule(
                RuleKind::PossiblySeparatedPairsOfHexDigits,
                vec![filter_0_repeated, sequence_1_repeated],
            )
        })
        .boxed();

        // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
        let raw_identifier_parser = filter(|&c: &char| {
            c == '_' || c == '$' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
        })
        .to(Node::new_anonymous_token(1))
        .then(
            filter(|&c: &char| {
                c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z')
                    || ('0' <= c && c <= '9')
            })
            .to(Node::new_anonymous_token(1))
            .repeated()
            .map(|v| Node::new_anonymous_token(v.len())),
        )
        .map(|(filter_0, filter_1_repeated)| {
            Node::new_rule(RuleKind::RawIdentifier, vec![filter_0, filter_1_repeated])
        })
        .boxed();

        // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
        let reserved_keyword_parser = choice::<_, ErrorType>((
            terminal("a").ignore_then(choice((
                terminal("fter").to(Node::new_token_part(TokenPartKind::After, 5usize)),
                terminal("lias").to(Node::new_token_part(TokenPartKind::Alias, 5usize)),
                terminal("pply").to(Node::new_token_part(TokenPartKind::Apply, 5usize)),
                terminal("uto").to(Node::new_token_part(TokenPartKind::Auto, 4usize)),
            ))),
            terminal("byte").to(Node::new_token_part(TokenPartKind::Byte, 4usize)),
            terminal("c").ignore_then(choice((
                terminal("ase").to(Node::new_token_part(TokenPartKind::Case, 4usize)),
                terminal("opyof").to(Node::new_token_part(TokenPartKind::Copyof, 6usize)),
            ))),
            terminal("def").ignore_then(choice((
                terminal("ault").to(Node::new_token_part(TokenPartKind::Default, 7usize)),
                terminal("ine").to(Node::new_token_part(TokenPartKind::Define, 6usize)),
            ))),
            terminal("final").to(Node::new_token_part(TokenPartKind::Final, 5usize)),
            terminal("i").ignore_then(choice((
                terminal("mplements").to(Node::new_token_part(TokenPartKind::Implements, 10usize)),
                terminal("n").ignore_then(choice((
                    terminal("line").to(Node::new_token_part(TokenPartKind::Inline, 6usize)),
                    empty().to(Node::new_token_part(TokenPartKind::In, 2usize)),
                ))),
            ))),
            terminal("let").to(Node::new_token_part(TokenPartKind::Let, 3usize)),
            terminal("m").ignore_then(choice((
                terminal("a").ignore_then(choice((
                    terminal("cro").to(Node::new_token_part(TokenPartKind::Macro, 5usize)),
                    terminal("tch").to(Node::new_token_part(TokenPartKind::Match, 5usize)),
                ))),
                terminal("utable").to(Node::new_token_part(TokenPartKind::Mutable, 7usize)),
            ))),
            terminal("null").to(Node::new_token_part(TokenPartKind::Null, 4usize)),
            terminal("of").to(Node::new_token_part(TokenPartKind::Of, 2usize)),
            terminal("p").ignore_then(choice((
                terminal("artial").to(Node::new_token_part(TokenPartKind::Partial, 7usize)),
                terminal("romise").to(Node::new_token_part(TokenPartKind::Promise, 7usize)),
            ))),
            terminal("re").ignore_then(choice((
                terminal("ference").to(Node::new_token_part(TokenPartKind::Reference, 9usize)),
                terminal("locatable").to(Node::new_token_part(TokenPartKind::Relocatable, 11usize)),
            ))),
            terminal("s").ignore_then(choice((
                terminal("ealed").to(Node::new_token_part(TokenPartKind::Sealed, 6usize)),
                terminal("izeof").to(Node::new_token_part(TokenPartKind::Sizeof, 6usize)),
                terminal("tatic").to(Node::new_token_part(TokenPartKind::Static, 6usize)),
                terminal("upports").to(Node::new_token_part(TokenPartKind::Supports, 8usize)),
                terminal("witch").to(Node::new_token_part(TokenPartKind::Switch, 6usize)),
            ))),
            terminal("type").ignore_then(choice((
                terminal("def").to(Node::new_token_part(TokenPartKind::Typedef, 7usize)),
                terminal("of").to(Node::new_token_part(TokenPartKind::Typeof, 6usize)),
            ))),
            terminal("var").to(Node::new_token_part(TokenPartKind::Var, 3usize)),
        ))
        .boxed();

        // «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
        let signed_fixed_type_parser = terminal("fixed")
            .to(Node::new_token_part(TokenPartKind::Fixed, 5usize))
            .then(
                filter(|&c: &char| ('0' <= c && c <= '9'))
                    .to(Node::new_anonymous_token(1))
                    .repeated()
                    .at_least(1usize)
                    .map(|v| Node::new_anonymous_token(v.len()))
                    .then(just('x').to(Node::new_token_part(
                        TokenPartKind::SignedFixedTypeLatinSmallLetterX,
                        1,
                    )))
                    .then(
                        filter(|&c: &char| ('0' <= c && c <= '9'))
                            .to(Node::new_anonymous_token(1))
                            .repeated()
                            .at_least(1usize)
                            .map(|v| Node::new_anonymous_token(v.len())),
                    )
                    .map(
                        |((filter_1_repeated, latin_small_letter_x), filter_2_repeated)| {
                            Node::new_anonymous_rule(vec![
                                filter_1_repeated,
                                latin_small_letter_x,
                                filter_2_repeated,
                            ])
                        },
                    )
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(|(fixed, sequence_0)| {
                Node::new_rule(RuleKind::SignedFixedType, vec![fixed, sequence_0])
            })
            .boxed();

        // «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
        let signed_integer_type_parser = terminal("int")
            .to(Node::new_token_part(TokenPartKind::Int, 3usize))
            .then(
                choice::<_, ErrorType>((
                    terminal("1").ignore_then(choice((
                        terminal("04").to(Node::new_token_part(TokenPartKind::OneZeroFour, 3usize)),
                        terminal("12").to(Node::new_token_part(TokenPartKind::OneOneTwo, 3usize)),
                        terminal("2").ignore_then(choice((
                            terminal("0")
                                .to(Node::new_token_part(TokenPartKind::OneTwoZero, 3usize)),
                            terminal("8")
                                .to(Node::new_token_part(TokenPartKind::OneTwoEight, 3usize)),
                        ))),
                        terminal("36").to(Node::new_token_part(TokenPartKind::OneThreeSix, 3usize)),
                        terminal("44").to(Node::new_token_part(TokenPartKind::OneFourFour, 3usize)),
                        terminal("52").to(Node::new_token_part(TokenPartKind::OneFiveTwo, 3usize)),
                        terminal("6").ignore_then(choice((
                            terminal("0")
                                .to(Node::new_token_part(TokenPartKind::OneSixZero, 3usize)),
                            terminal("8")
                                .to(Node::new_token_part(TokenPartKind::OneSixEight, 3usize)),
                            empty().to(Node::new_token_part(TokenPartKind::OneSix, 2usize)),
                        ))),
                        terminal("76").to(Node::new_token_part(TokenPartKind::OneSevenSix, 3usize)),
                        terminal("84")
                            .to(Node::new_token_part(TokenPartKind::OneEightFour, 3usize)),
                        terminal("92").to(Node::new_token_part(TokenPartKind::OneNineTwo, 3usize)),
                    ))),
                    terminal("2").ignore_then(choice((
                        terminal("0").ignore_then(choice((
                            terminal("0")
                                .to(Node::new_token_part(TokenPartKind::TwoZeroZero, 3usize)),
                            terminal("8")
                                .to(Node::new_token_part(TokenPartKind::TwoZeroEight, 3usize)),
                        ))),
                        terminal("16").to(Node::new_token_part(TokenPartKind::TwoOneSix, 3usize)),
                        terminal("24").to(Node::new_token_part(TokenPartKind::TwoTwoFour, 3usize)),
                        terminal("32").to(Node::new_token_part(TokenPartKind::TwoThreeTwo, 3usize)),
                        terminal("4").ignore_then(choice((
                            terminal("0")
                                .to(Node::new_token_part(TokenPartKind::TwoFourZero, 3usize)),
                            terminal("8")
                                .to(Node::new_token_part(TokenPartKind::TwoFourEight, 3usize)),
                            empty().to(Node::new_token_part(TokenPartKind::TwoFour, 2usize)),
                        ))),
                        terminal("56").to(Node::new_token_part(TokenPartKind::TwoFiveSix, 3usize)),
                    ))),
                    terminal("32").to(Node::new_token_part(TokenPartKind::ThreeTwo, 2usize)),
                    terminal("4").ignore_then(choice((
                        terminal("0").to(Node::new_token_part(TokenPartKind::FourZero, 2usize)),
                        terminal("8").to(Node::new_token_part(TokenPartKind::FourEight, 2usize)),
                    ))),
                    terminal("56").to(Node::new_token_part(TokenPartKind::FiveSix, 2usize)),
                    terminal("64").to(Node::new_token_part(TokenPartKind::SixFour, 2usize)),
                    terminal("72").to(Node::new_token_part(TokenPartKind::SevenTwo, 2usize)),
                    terminal("8").ignore_then(choice((
                        terminal("0").to(Node::new_token_part(TokenPartKind::EightZero, 2usize)),
                        terminal("8").to(Node::new_token_part(TokenPartKind::EightEight, 2usize)),
                        empty().to(Node::new_token_part(TokenPartKind::Eight, 1usize)),
                    ))),
                    terminal("96").to(Node::new_token_part(TokenPartKind::NineSix, 2usize)),
                ))
                .or_not()
                .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(|(int, byte_count)| {
                Node::new_rule(RuleKind::SignedIntegerType, vec![int, byte_count])
            })
            .boxed();

        // «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
        let single_line_comment_parser = terminal("//")
            .to(Node::new_token_part(TokenPartKind::SlashSlash, 2usize))
            .then(
                filter(|&c: &char| c != '\r' && c != '\n')
                    .to(Node::new_anonymous_token(1))
                    .repeated()
                    .map(|v| Node::new_anonymous_token(v.len())),
            )
            .map(|(slash_slash, filter_0_repeated)| {
                Node::new_rule(
                    RuleKind::SingleLineComment,
                    vec![slash_slash, filter_0_repeated],
                )
            })
            .boxed();

        // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
        let unicode_escape_parser = just('u')
            .to(Node::new_token_part(
                TokenPartKind::UnicodeEscapeLatinSmallLetterU,
                1,
            ))
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .to(Node::new_anonymous_token(1))
                .repeated()
                .exactly(4usize)
                .map(|v| Node::new_anonymous_token(v.len())),
            )
            .map(|(latin_small_letter_u, filter_0_repeated)| {
                Node::new_rule(
                    RuleKind::UnicodeEscape,
                    vec![latin_small_letter_u, filter_0_repeated],
                )
            })
            .boxed();

        // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
        let version_pragma_operator_parser = choice((
            just('^').to(Node::new_token_part(
                TokenPartKind::VersionPragmaOperatorCaret,
                1,
            )),
            just('~').to(Node::new_token_part(
                TokenPartKind::VersionPragmaOperatorTilde,
                1,
            )),
            just('=').to(Node::new_token_part(
                TokenPartKind::VersionPragmaOperatorEqual,
                1,
            )),
            just('<').to(Node::new_token_part(
                TokenPartKind::VersionPragmaOperatorLess,
                1,
            )),
            just('>').to(Node::new_token_part(
                TokenPartKind::VersionPragmaOperatorGreater,
                1,
            )),
            terminal("<=").to(Node::new_token_part(TokenPartKind::LessEqual, 2usize)),
            terminal(">=").to(Node::new_token_part(TokenPartKind::GreaterEqual, 2usize)),
        ))
        .map(|v| Node::new_rule(RuleKind::VersionPragmaOperator, vec![v]))
        .boxed();

        // «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' }  { '.' 1…*{ '0'…'9' | 'x' | 'X' | '*' } } ;
        let version_pragma_value_parser =
            filter(|&c: &char| ('0' <= c && c <= '9') || c == 'x' || c == 'X' || c == '*')
                .to(Node::new_anonymous_token(1))
                .repeated()
                .at_least(1usize)
                .map(|v| Node::new_anonymous_token(v.len()))
                .then(
                    just('.')
                        .to(Node::new_token_part(TokenPartKind::Period, 1usize))
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9') || c == 'x' || c == 'X' || c == '*'
                            })
                            .to(Node::new_anonymous_token(1))
                            .repeated()
                            .at_least(1usize)
                            .map(|v| Node::new_anonymous_token(v.len())),
                        )
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Node::new_rule(RuleKind::VersionPragmaValue, v))
                .boxed();

        // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
        let whitespace_parser = filter(|&c: &char| c == ' ' || c == '\t')
            .to(Node::new_anonymous_token(1))
            .repeated()
            .at_least(1usize)
            .map(|v| Node::new_token(TokenKind::Whitespace, v))
            .boxed();

        // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
        let yul_decimal_number_literal_parser = choice((
            just('0').to(Node::new_token_part(
                TokenPartKind::YulDecimalNumberLiteralZero,
                1,
            )),
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .to(Node::new_anonymous_token(1))
                .then(
                    filter(|&c: &char| ('0' <= c && c <= '9'))
                        .to(Node::new_anonymous_token(1))
                        .repeated()
                        .map(|v| Node::new_anonymous_token(v.len())),
                )
                .map(|(filter_1, filter_2_repeated)| {
                    Node::new_anonymous_rule(vec![filter_1, filter_2_repeated])
                }),
        ))
        .map(|v| Node::new_rule(RuleKind::YulDecimalNumberLiteral, vec![v]))
        .boxed();

        // «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
        let yul_hex_literal_parser = terminal("0x")
            .to(Node::new_token_part(TokenPartKind::ZeroX, 2usize))
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .to(Node::new_anonymous_token(1))
                .repeated()
                .at_least(1usize)
                .map(|v| Node::new_anonymous_token(v.len())),
            )
            .map(|(zero_x, filter_0_repeated)| {
                Node::new_rule(RuleKind::YulHexLiteral, vec![zero_x, filter_0_repeated])
            })
            .boxed();

        // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
        let decimal_exponent_parser = filter(|&c: &char| c == 'e' || c == 'E')
            .to(Node::new_anonymous_token(1))
            .then(
                just('-')
                    .to(Node::new_token_part(TokenPartKind::DecimalExponentMinus, 1))
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(decimal_integer_parser.clone())
            .map(|((filter_0, minus), decimal_integer)| {
                Node::new_rule(
                    RuleKind::DecimalExponent,
                    vec![filter_0, minus, decimal_integer],
                )
            })
            .boxed();

        // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
        let decimal_float_parser = decimal_integer_parser
            .clone()
            .or_not()
            .map(|v| v.unwrap_or_else(|| Node::new_none()))
            .then(just('.').to(Node::new_token_part(TokenPartKind::DecimalFloatPeriod, 1)))
            .then(decimal_integer_parser.clone())
            .map(|((decimal_integer_0_, period), decimal_integer_1_)| {
                Node::new_rule(
                    RuleKind::DecimalFloat,
                    vec![decimal_integer_0_, period, decimal_integer_1_],
                )
            })
            .boxed();

        // «EndOfFileTrivia» = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
        let end_of_file_trivia_parser = choice((
            whitespace_parser.clone(),
            multiline_comment_parser.clone(),
            single_line_comment_parser.clone(),
        ))
        .repeated()
        .map(|v| {
            if v.is_empty() {
                Node::new_none()
            } else {
                Node::new_rule(RuleKind::EndOfFileTrivia, v)
            }
        })
        .boxed();

        // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
        let escape_sequence_parser = just('\\')
            .to(Node::new_token_part(
                TokenPartKind::EscapeSequenceBackslash,
                1,
            ))
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
                .to(Node::new_anonymous_token(1)),
                hex_byte_escape_parser.clone(),
                unicode_escape_parser.clone(),
            )))
            .map(|(backslash, choices_0)| {
                Node::new_rule(RuleKind::EscapeSequence, vec![backslash, choices_0])
            })
            .boxed();

        // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
        let hex_string_literal_parser = terminal ("hex") . to (Node :: new_token_part (TokenPartKind :: Hex , 3usize)) . then (choice ((just ('"') . to (Node :: new_token_part (TokenPartKind :: DoubleQuote , 1usize)) . then (possibly_separated_pairs_of_hex_digits_parser . clone () . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (just ('"') . to (Node :: new_token_part (TokenPartKind :: DoubleQuote , 1usize))) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: HexStringLiteralDoubleQuoteAndPossiblySeparatedPairsOfHexDigitsAndDoubleQuote , vec ! [a , b , c])) , just ('\'') . to (Node :: new_token_part (TokenPartKind :: Quote , 1usize)) . then (possibly_separated_pairs_of_hex_digits_parser . clone () . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (just ('\'') . to (Node :: new_token_part (TokenPartKind :: Quote , 1usize))) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: HexStringLiteralQuoteAndPossiblySeparatedPairsOfHexDigitsAndQuote , vec ! [a , b , c]))))) . map (| (hex , choices_0) | Node :: new_rule (RuleKind :: HexStringLiteral , vec ! [hex , choices_0])) . boxed () ;

        // «LeadingTrivia» = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
        let leading_trivia_parser = choice((
            whitespace_parser.clone(),
            end_of_line_parser.clone(),
            multiline_comment_parser.clone(),
            single_line_comment_parser.clone(),
        ))
        .repeated()
        .map(|v| {
            if v.is_empty() {
                Node::new_none()
            } else {
                Node::new_rule(RuleKind::LeadingTrivia, v)
            }
        })
        .boxed();

        // «TrailingTrivia» = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
        let trailing_trivia_parser =
            choice((whitespace_parser.clone(), multiline_comment_parser.clone()))
                .repeated()
                .map(Node::new_anonymous_rule)
                .then(choice((
                    end_of_line_parser.clone(),
                    single_line_comment_parser.clone(),
                )))
                .map(|(choices_0_repeated, choices_1)| {
                    Node::new_rule(
                        RuleKind::TrailingTrivia,
                        vec![choices_0_repeated, choices_1],
                    )
                })
                .or_not()
                .map(|v| v.unwrap_or_else(|| Node::new_none()))
                .boxed();

        // «UnsignedFixedType» = 'u' «SignedFixedType» ;
        let unsigned_fixed_type_parser = just('u')
            .to(Node::new_token_part(
                TokenPartKind::UnsignedFixedTypeLatinSmallLetterU,
                1,
            ))
            .then(signed_fixed_type_parser.clone())
            .map(|(latin_small_letter_u, signed_fixed_type)| {
                Node::new_rule(
                    RuleKind::UnsignedFixedType,
                    vec![latin_small_letter_u, signed_fixed_type],
                )
            })
            .boxed();

        // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
        let unsigned_integer_type_parser = just('u')
            .to(Node::new_token_part(
                TokenPartKind::UnsignedIntegerTypeLatinSmallLetterU,
                1,
            ))
            .then(signed_integer_type_parser.clone())
            .map(|(latin_small_letter_u, signed_integer_type)| {
                Node::new_rule(
                    RuleKind::UnsignedIntegerType,
                    vec![latin_small_letter_u, signed_integer_type],
                )
            })
            .boxed();

        // «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
        let yul_keyword_parser = choice((
            boolean_literal_parser.clone(),
            choice::<_, ErrorType>((
                terminal("break").to(Node::new_token_part(TokenPartKind::Break, 5usize)),
                terminal("c").ignore_then(choice((
                    terminal("ase").to(Node::new_token_part(TokenPartKind::Case, 4usize)),
                    terminal("ontinue").to(Node::new_token_part(TokenPartKind::Continue, 8usize)),
                ))),
                terminal("default").to(Node::new_token_part(TokenPartKind::Default, 7usize)),
                terminal("f").ignore_then(choice((
                    terminal("or").to(Node::new_token_part(TokenPartKind::For, 3usize)),
                    terminal("unction").to(Node::new_token_part(TokenPartKind::Function, 8usize)),
                ))),
                terminal("hex").to(Node::new_token_part(TokenPartKind::Hex, 3usize)),
                terminal("if").to(Node::new_token_part(TokenPartKind::If, 2usize)),
                terminal("le").ignore_then(choice((
                    terminal("ave").to(Node::new_token_part(TokenPartKind::Leave, 5usize)),
                    terminal("t").to(Node::new_token_part(TokenPartKind::Let, 3usize)),
                ))),
                terminal("switch").to(Node::new_token_part(TokenPartKind::Switch, 6usize)),
            )),
        ))
        .map(|v| Node::new_rule(RuleKind::YulKeyword, vec![v]))
        .boxed();

        // AddressType = 'address' [ 'payable' ] ;
        let address_type_parser = leading_trivia_parser
            .clone()
            .then(terminal("address").to(Node::new_token_part(TokenPartKind::Address, 7usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("payable")
                            .to(Node::new_token_part(TokenPartKind::Payable, 7usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(|(address, payable)| Node::new_rule(RuleKind::AddressType, vec![address, payable]))
            .boxed();

        // ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
        let array_literal_parser = leading_trivia_parser
            .clone()
            .then(just('[').to(Node::new_token_part(TokenPartKind::OpenBracket, 1usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                expression_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(Node::new_token_part(TokenPartKind::Comma, 1usize)))
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia)
                            .then(expression_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| {
                        Node::new_rule(RuleKind::ArrayLiteralExpressionRepeatedAndCommaRepeated, v)
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(']').to(Node::new_token_part(TokenPartKind::CloseBracket, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((a, b), c)| Node::new_rule(RuleKind::ArrayLiteral, vec![a, b, c]))
            .boxed();

        // BreakStatement = 'break' ';' ;
        let break_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("break").to(Node::new_token_part(TokenPartKind::Break, 5usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::BreakStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|(r#break, semicolon)| {
                Node::new_rule(RuleKind::BreakStatement, vec![r#break, semicolon])
            })
            .boxed();

        // ContinueStatement = 'continue' ';' ;
        let continue_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("continue").to(Node::new_token_part(TokenPartKind::Continue, 8usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::ContinueStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|(r#continue, semicolon)| {
                Node::new_rule(RuleKind::ContinueStatement, vec![r#continue, semicolon])
            })
            .boxed();

        // DataLocation = 'memory' | 'storage' | 'calldata' ;
        let data_location_parser = choice((
            leading_trivia_parser
                .clone()
                .then(terminal("memory").to(Node::new_token_part(TokenPartKind::Memory, 6usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("storage").to(Node::new_token_part(TokenPartKind::Storage, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("calldata").to(Node::new_token_part(TokenPartKind::Calldata, 8usize)),
                )
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::DataLocation, vec![v]))
        .boxed();

        // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
        let decimal_number_parser =
            choice((decimal_integer_parser.clone(), decimal_float_parser.clone()))
                .then(
                    decimal_exponent_parser
                        .clone()
                        .or_not()
                        .map(|v| v.unwrap_or_else(|| Node::new_none())),
                )
                .map(|(choices_0, decimal_exponent)| {
                    Node::new_rule(RuleKind::DecimalNumber, vec![choices_0, decimal_exponent])
                })
                .boxed();

        // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
        let double_quoted_ascii_string_literal_parser = just('"')
            .to(Node::new_token_part(TokenPartKind::DoubleQuote, 1usize))
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '"' && c != '\\')
                        .to(Node::new_token_part(
                            TokenPartKind::DoubleQuotedAsciiStringLiteralChar,
                            1,
                        ))
                        .repeated()
                        .at_least(1usize)
                        .map(|v| {
                            Node::new_token(
                                TokenKind::DoubleQuotedAsciiStringLiteralCharRepeated,
                                v,
                            )
                        }),
                    escape_sequence_parser.clone(),
                ))
                .map(|v| Node::new_rule(RuleKind::DoubleQuotedAsciiStringLiteralRun, vec![v]))
                .repeated()
                .map(|v| {
                    if v.is_empty() {
                        Node::new_none()
                    } else {
                        Node::new_rule(RuleKind::DoubleQuotedAsciiStringLiteralRunRepeated, v)
                    }
                }),
            )
            .then(just('"').to(Node::new_token_part(TokenPartKind::DoubleQuote, 1usize)))
            .map(|((a, b), c)| {
                Node::new_rule(RuleKind::DoubleQuotedAsciiStringLiteral, vec![a, b, c])
            })
            .boxed();

        // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
        let double_quoted_unicode_string_literal_parser = just("unicode\"")
            .to(Node::new_token_part(
                TokenPartKind::UnicodeDoubleQuote,
                8usize,
            ))
            .then(
                choice((
                    filter(|&c: &char| c != '"' && c != '\\' && c != '\n' && c != '\r')
                        .to(Node::new_token_part(
                            TokenPartKind::DoubleQuotedUnicodeStringLiteralChar,
                            1,
                        ))
                        .repeated()
                        .at_least(1usize)
                        .map(|v| {
                            Node::new_token(
                                TokenKind::DoubleQuotedUnicodeStringLiteralCharRepeated,
                                v,
                            )
                        }),
                    escape_sequence_parser.clone(),
                ))
                .map(|v| Node::new_rule(RuleKind::DoubleQuotedUnicodeStringLiteralRun, vec![v]))
                .repeated()
                .map(|v| {
                    if v.is_empty() {
                        Node::new_none()
                    } else {
                        Node::new_rule(RuleKind::DoubleQuotedUnicodeStringLiteralRunRepeated, v)
                    }
                }),
            )
            .then(just('"').to(Node::new_token_part(TokenPartKind::DoubleQuote, 1usize)))
            .map(|((a, b), c)| {
                Node::new_rule(RuleKind::DoubleQuotedUnicodeStringLiteral, vec![a, b, c])
            })
            .boxed();

        // «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
        let keyword_parser = choice((
            boolean_literal_parser.clone(),
            fixed_bytes_type_parser.clone(),
            number_unit_parser.clone(),
            reserved_keyword_parser.clone(),
            signed_integer_type_parser.clone(),
            unsigned_integer_type_parser.clone(),
            choice::<_, ErrorType>((
                terminal("a").ignore_then(choice((
                    terminal("bstract").to(Node::new_token_part(TokenPartKind::Abstract, 8usize)),
                    terminal("ddress").to(Node::new_token_part(TokenPartKind::Address, 7usize)),
                    terminal("nonymous").to(Node::new_token_part(TokenPartKind::Anonymous, 9usize)),
                    terminal("s").ignore_then(choice((
                        terminal("sembly")
                            .to(Node::new_token_part(TokenPartKind::Assembly, 8usize)),
                        empty().to(Node::new_token_part(TokenPartKind::As, 2usize)),
                    ))),
                ))),
                terminal("b").ignore_then(choice((
                    terminal("ool").to(Node::new_token_part(TokenPartKind::Bool, 4usize)),
                    terminal("reak").to(Node::new_token_part(TokenPartKind::Break, 5usize)),
                ))),
                terminal("c").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("lldata")
                            .to(Node::new_token_part(TokenPartKind::Calldata, 8usize)),
                        terminal("tch").to(Node::new_token_part(TokenPartKind::Catch, 5usize)),
                    ))),
                    terminal("on").ignore_then(choice((
                        terminal("st").ignore_then(choice((
                            terminal("ant")
                                .to(Node::new_token_part(TokenPartKind::Constant, 8usize)),
                            terminal("ructor")
                                .to(Node::new_token_part(TokenPartKind::Constructor, 11usize)),
                        ))),
                        terminal("t").ignore_then(choice((
                            terminal("inue")
                                .to(Node::new_token_part(TokenPartKind::Continue, 8usize)),
                            terminal("ract")
                                .to(Node::new_token_part(TokenPartKind::Contract, 8usize)),
                        ))),
                    ))),
                ))),
                terminal("d").ignore_then(choice((
                    terminal("elete").to(Node::new_token_part(TokenPartKind::Delete, 6usize)),
                    terminal("o").to(Node::new_token_part(TokenPartKind::Do, 2usize)),
                ))),
                terminal("e").ignore_then(choice((
                    terminal("lse").to(Node::new_token_part(TokenPartKind::Else, 4usize)),
                    terminal("mit").to(Node::new_token_part(TokenPartKind::Emit, 4usize)),
                    terminal("num").to(Node::new_token_part(TokenPartKind::Enum, 4usize)),
                    terminal("vent").to(Node::new_token_part(TokenPartKind::Event, 5usize)),
                    terminal("xternal").to(Node::new_token_part(TokenPartKind::External, 8usize)),
                ))),
                terminal("f").ignore_then(choice((
                    terminal("al").ignore_then(choice((
                        terminal("lback").to(Node::new_token_part(TokenPartKind::Fallback, 8usize)),
                        terminal("se").to(Node::new_token_part(TokenPartKind::False, 5usize)),
                    ))),
                    terminal("ixed").to(Node::new_token_part(TokenPartKind::Fixed, 5usize)),
                    terminal("or").to(Node::new_token_part(TokenPartKind::For, 3usize)),
                    terminal("unction").to(Node::new_token_part(TokenPartKind::Function, 8usize)),
                ))),
                terminal("hex").to(Node::new_token_part(TokenPartKind::Hex, 3usize)),
                terminal("i").ignore_then(choice((
                    terminal("f").to(Node::new_token_part(TokenPartKind::If, 2usize)),
                    terminal("m").ignore_then(choice((
                        terminal("mutable")
                            .to(Node::new_token_part(TokenPartKind::Immutable, 9usize)),
                        terminal("port").to(Node::new_token_part(TokenPartKind::Import, 6usize)),
                    ))),
                    terminal("n").ignore_then(choice((
                        terminal("dexed").to(Node::new_token_part(TokenPartKind::Indexed, 7usize)),
                        terminal("ter").ignore_then(choice((
                            terminal("face")
                                .to(Node::new_token_part(TokenPartKind::Interface, 9usize)),
                            terminal("nal")
                                .to(Node::new_token_part(TokenPartKind::Internal, 8usize)),
                        ))),
                    ))),
                    terminal("s").to(Node::new_token_part(TokenPartKind::Is, 2usize)),
                ))),
                terminal("library").to(Node::new_token_part(TokenPartKind::Library, 7usize)),
                terminal("m").ignore_then(choice((
                    terminal("apping").to(Node::new_token_part(TokenPartKind::Mapping, 7usize)),
                    terminal("emory").to(Node::new_token_part(TokenPartKind::Memory, 6usize)),
                    terminal("odifier").to(Node::new_token_part(TokenPartKind::Modifier, 8usize)),
                ))),
                terminal("new").to(Node::new_token_part(TokenPartKind::New, 3usize)),
                terminal("override").to(Node::new_token_part(TokenPartKind::Override, 8usize)),
                terminal("p").ignore_then(choice((
                    terminal("ayable").to(Node::new_token_part(TokenPartKind::Payable, 7usize)),
                    terminal("r").ignore_then(choice((
                        terminal("agma").to(Node::new_token_part(TokenPartKind::Pragma, 6usize)),
                        terminal("ivate").to(Node::new_token_part(TokenPartKind::Private, 7usize)),
                    ))),
                    terminal("u").ignore_then(choice((
                        terminal("blic").to(Node::new_token_part(TokenPartKind::Public, 6usize)),
                        terminal("re").to(Node::new_token_part(TokenPartKind::Pure, 4usize)),
                    ))),
                ))),
                terminal("re").ignore_then(choice((
                    terminal("ceive").to(Node::new_token_part(TokenPartKind::Receive, 7usize)),
                    terminal("turn").ignore_then(choice((
                        terminal("s").to(Node::new_token_part(TokenPartKind::Returns, 7usize)),
                        empty().to(Node::new_token_part(TokenPartKind::Return, 6usize)),
                    ))),
                ))),
                terminal("st").ignore_then(choice((
                    terminal("orage").to(Node::new_token_part(TokenPartKind::Storage, 7usize)),
                    terminal("r").ignore_then(choice((
                        terminal("ing").to(Node::new_token_part(TokenPartKind::String, 6usize)),
                        terminal("uct").to(Node::new_token_part(TokenPartKind::Struct, 6usize)),
                    ))),
                ))),
                terminal("t").ignore_then(choice((
                    terminal("r").ignore_then(choice((
                        terminal("ue").to(Node::new_token_part(TokenPartKind::True, 4usize)),
                        terminal("y").to(Node::new_token_part(TokenPartKind::Try, 3usize)),
                    ))),
                    terminal("ype").to(Node::new_token_part(TokenPartKind::Type, 4usize)),
                ))),
                terminal("u").ignore_then(choice((
                    terminal("fixed").to(Node::new_token_part(TokenPartKind::Ufixed, 6usize)),
                    terminal("nchecked").to(Node::new_token_part(TokenPartKind::Unchecked, 9usize)),
                    terminal("sing").to(Node::new_token_part(TokenPartKind::Using, 5usize)),
                ))),
                terminal("vi").ignore_then(choice((
                    terminal("ew").to(Node::new_token_part(TokenPartKind::View, 4usize)),
                    terminal("rtual").to(Node::new_token_part(TokenPartKind::Virtual, 7usize)),
                ))),
                terminal("while").to(Node::new_token_part(TokenPartKind::While, 5usize)),
            )),
        ))
        .map(|v| Node::new_rule(RuleKind::Keyword, vec![v]))
        .boxed();

        // ParenthesisExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
        let parenthesis_expression_parser = leading_trivia_parser
            .clone()
            .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                expression_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none()))
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(Node::new_token_part(TokenPartKind::Comma, 1usize)))
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia)
                            .then(
                                expression_parser
                                    .clone()
                                    .or_not()
                                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| {
                        Node::new_rule(
                            RuleKind::ParenthesisExpressionExpressionRepeatedAndCommaRepeated,
                            v,
                        )
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(')').to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((a, b), c)| Node::new_rule(RuleKind::ParenthesisExpression, vec![a, b, c]))
            .boxed();

        // PositionalArgumentList = Expression  { ',' Expression } ;
        let positional_argument_list_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(',').to(Node::new_token_part(TokenPartKind::Comma, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(expression_parser.clone())
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Node::new_rule(RuleKind::PositionalArgumentList, v))
            .boxed();

        // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_ascii_string_literal_parser = just('\'')
            .to(Node::new_token_part(TokenPartKind::Quote, 1usize))
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '\'' && c != '\\')
                        .to(Node::new_token_part(
                            TokenPartKind::SingleQuotedAsciiStringLiteralChar,
                            1,
                        ))
                        .repeated()
                        .at_least(1usize)
                        .map(|v| {
                            Node::new_token(
                                TokenKind::SingleQuotedAsciiStringLiteralCharRepeated,
                                v,
                            )
                        }),
                    escape_sequence_parser.clone(),
                ))
                .map(|v| Node::new_rule(RuleKind::SingleQuotedAsciiStringLiteralRun, vec![v]))
                .repeated()
                .map(|v| {
                    if v.is_empty() {
                        Node::new_none()
                    } else {
                        Node::new_rule(RuleKind::SingleQuotedAsciiStringLiteralRunRepeated, v)
                    }
                }),
            )
            .then(just('\'').to(Node::new_token_part(TokenPartKind::Quote, 1usize)))
            .map(|((a, b), c)| {
                Node::new_rule(RuleKind::SingleQuotedAsciiStringLiteral, vec![a, b, c])
            })
            .boxed();

        // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_unicode_string_literal_parser = just("unicode'")
            .to(Node::new_token_part(TokenPartKind::UnicodeQuote, 8usize))
            .then(
                choice((
                    filter(|&c: &char| c != '\'' && c != '\\' && c != '\n' && c != '\r')
                        .to(Node::new_token_part(
                            TokenPartKind::SingleQuotedUnicodeStringLiteralChar,
                            1,
                        ))
                        .repeated()
                        .at_least(1usize)
                        .map(|v| {
                            Node::new_token(
                                TokenKind::SingleQuotedUnicodeStringLiteralCharRepeated,
                                v,
                            )
                        }),
                    escape_sequence_parser.clone(),
                ))
                .map(|v| Node::new_rule(RuleKind::SingleQuotedUnicodeStringLiteralRun, vec![v]))
                .repeated()
                .map(|v| {
                    if v.is_empty() {
                        Node::new_none()
                    } else {
                        Node::new_rule(RuleKind::SingleQuotedUnicodeStringLiteralRunRepeated, v)
                    }
                }),
            )
            .then(just('\'').to(Node::new_token_part(TokenPartKind::Quote, 1usize)))
            .map(|((a, b), c)| {
                Node::new_rule(RuleKind::SingleQuotedUnicodeStringLiteral, vec![a, b, c])
            })
            .boxed();

        // UncheckedBlock = 'unchecked' Block ;
        let unchecked_block_parser = leading_trivia_parser
            .clone()
            .then(terminal("unchecked").to(Node::new_token_part(TokenPartKind::Unchecked, 9usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(block_parser.clone())
            .map(|(unchecked, block)| {
                Node::new_rule(RuleKind::UncheckedBlock, vec![unchecked, block])
            })
            .boxed();

        // VersionPragmaSpecifier = 'solidity' 1…*{ «VersionPragmaOperator» «VersionPragmaValue» } ;
        let version_pragma_specifier_parser = leading_trivia_parser
            .clone()
            .then(terminal("solidity").to(Node::new_token_part(TokenPartKind::Solidity, 8usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(version_pragma_operator_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(version_pragma_value_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|(version_pragma_operator, version_pragma_value)| {
                        Node::new_anonymous_rule(vec![
                            version_pragma_operator,
                            version_pragma_value,
                        ])
                    })
                    .repeated()
                    .at_least(1usize)
                    .map(Node::new_anonymous_rule),
            )
            .map(|(solidity, sequence_0_repeated)| {
                Node::new_rule(
                    RuleKind::VersionPragmaSpecifier,
                    vec![solidity, sequence_0_repeated],
                )
            })
            .boxed();

        // YulBreakStatement = 'break' ;
        let yul_break_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("break").to(Node::new_token_part(TokenPartKind::Break, 5usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .boxed();

        // YulContinueStatement = 'continue' ;
        let yul_continue_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("continue").to(Node::new_token_part(TokenPartKind::Continue, 8usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .boxed();

        // «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
        let yul_identifier_parser =
            difference(raw_identifier_parser.clone(), yul_keyword_parser.clone()).boxed();

        // YulLeaveStatement = 'leave' ;
        let yul_leave_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("leave").to(Node::new_token_part(TokenPartKind::Leave, 5usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .boxed();

        // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
        let ascii_string_literal_parser = choice((
            single_quoted_ascii_string_literal_parser.clone(),
            double_quoted_ascii_string_literal_parser.clone(),
        ))
        .map(|v| Node::new_rule(RuleKind::AsciiStringLiteral, vec![v]))
        .boxed();

        // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
        let assembly_flags_parser = leading_trivia_parser . clone () . then (just ('(') . to (Node :: new_token_part (TokenPartKind :: OpenParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (double_quoted_ascii_string_literal_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (double_quoted_ascii_string_literal_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: AssemblyFlagsDoubleQuotedAsciiStringLiteralRepeatedAndCommaRepeated , v))) . then (leading_trivia_parser . clone () . then (just (')') . to (Node :: new_token_part (TokenPartKind :: CloseParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: AssemblyFlags , vec ! [a , b , c])) . boxed () ;

        // ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
        let elementary_type_parser = choice((
            leading_trivia_parser
                .clone()
                .then(terminal("bool").to(Node::new_token_part(TokenPartKind::Bool, 4usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("string").to(Node::new_token_part(TokenPartKind::String, 6usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            address_type_parser.clone(),
            leading_trivia_parser
                .clone()
                .then(fixed_bytes_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(signed_integer_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(unsigned_integer_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(signed_fixed_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(unsigned_fixed_type_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::ElementaryType, vec![v]))
        .boxed();

        // «Identifier» = «RawIdentifier» - «Keyword» ;
        let identifier_parser =
            difference(raw_identifier_parser.clone(), keyword_parser.clone()).boxed();

        // «NumericLiteral» = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
        let numeric_literal_parser =
            choice((decimal_number_parser.clone(), hex_number_parser.clone()))
                .then(
                    number_unit_parser
                        .clone()
                        .or_not()
                        .map(|v| v.unwrap_or_else(|| Node::new_none())),
                )
                .map(|(choices_0, number_unit)| {
                    Node::new_rule(RuleKind::NumericLiteral, vec![choices_0, number_unit])
                })
                .boxed();

        // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
        let unicode_string_literal_parser = choice((
            single_quoted_unicode_string_literal_parser.clone(),
            double_quoted_unicode_string_literal_parser.clone(),
        ))
        .map(|v| Node::new_rule(RuleKind::UnicodeStringLiteral, vec![v]))
        .boxed();

        // YulFunctionCall = «YulIdentifier» '(' [ YulExpression  { ',' YulExpression } ] ')' ;
        let yul_function_call_parser = leading_trivia_parser . clone () . then (yul_identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (just ('(') . to (Node :: new_token_part (TokenPartKind :: OpenParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (yul_expression_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (yul_expression_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: YulFunctionCallYulExpressionRepeatedAndCommaRepeated , v)) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (leading_trivia_parser . clone () . then (just (')') . to (Node :: new_token_part (TokenPartKind :: CloseParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: YulFunctionCallOpenParenAndYulExpressionRepeatedAndCommaRepeatedAndCloseParen , vec ! [a , b , c]))) . map (| (yul_identifier , open_paren_and_yul_expression_repeated_and_comma_repeated_and_close_paren) | Node :: new_rule (RuleKind :: YulFunctionCall , vec ! [yul_identifier , open_paren_and_yul_expression_repeated_and_comma_repeated_and_close_paren])) . boxed () ;

        // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
        let yul_function_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("function").to(Node::new_token_part(TokenPartKind::Function, 8usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(yul_identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(yul_identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia)
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(
                                        just(',')
                                            .to(Node::new_token_part(TokenPartKind::Comma, 1usize)),
                                    )
                                    .then(trailing_trivia_parser.clone())
                                    .map(Node::new_with_trivia)
                                    .then(
                                        leading_trivia_parser
                                            .clone()
                                            .then(yul_identifier_parser.clone())
                                            .then(trailing_trivia_parser.clone())
                                            .map(Node::new_with_trivia),
                                    )
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|v| Node::new_rule(RuleKind::YulFunctionDefinitionArguments, v))
                            .or_not()
                            .map(|v| v.unwrap_or_else(|| Node::new_none())),
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just(')')
                                    .to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| {
                        Node::new_rule(
                            RuleKind::YulFunctionDefinitionOpenParenAndArgumentsAndCloseParen,
                            vec![a, b, c],
                        )
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("->")
                            .to(Node::new_token_part(TokenPartKind::MinusGreater, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(yul_identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia)
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(
                                        just(',')
                                            .to(Node::new_token_part(TokenPartKind::Comma, 1usize)),
                                    )
                                    .then(trailing_trivia_parser.clone())
                                    .map(Node::new_with_trivia)
                                    .then(
                                        leading_trivia_parser
                                            .clone()
                                            .then(yul_identifier_parser.clone())
                                            .then(trailing_trivia_parser.clone())
                                            .map(Node::new_with_trivia),
                                    )
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|v| Node::new_rule(RuleKind::YulFunctionDefinitionResults, v)),
                    )
                    .map(|(minus_greater, results)| {
                        Node::new_anonymous_rule(vec![minus_greater, results])
                    })
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(yul_block_parser.clone())
            .map(
                |(
                    (
                        ((function, yul_identifier), open_paren_and_arguments_and_close_paren),
                        sequence_0,
                    ),
                    yul_block,
                )| {
                    Node::new_rule(
                        RuleKind::YulFunctionDefinition,
                        vec![
                            function,
                            yul_identifier,
                            open_paren_and_arguments_and_close_paren,
                            sequence_0,
                            yul_block,
                        ],
                    )
                },
            )
            .boxed();

        // YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
        let yul_identifier_path_parser = leading_trivia_parser
            .clone()
            .then(yul_identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('.').to(Node::new_token_part(TokenPartKind::Period, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(yul_identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Node::new_rule(RuleKind::YulIdentifierPath, v))
            .boxed();

        // ABICoderPragmaSpecifier = 'abicoder' «Identifier» ;
        let abi_coder_pragma_specifier_parser = leading_trivia_parser
            .clone()
            .then(terminal("abicoder").to(Node::new_token_part(TokenPartKind::Abicoder, 8usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|(abicoder, identifier)| {
                Node::new_rule(
                    RuleKind::AbiCoderPragmaSpecifier,
                    vec![abicoder, identifier],
                )
            })
            .boxed();

        // DeleteStatement = 'delete' «Identifier» ';' ;
        let delete_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("delete").to(Node::new_token_part(TokenPartKind::Delete, 6usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::DeleteStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((delete, identifier), semicolon)| {
                Node::new_rule(
                    RuleKind::DeleteStatement,
                    vec![delete, identifier, semicolon],
                )
            })
            .boxed();

        // EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
        let enum_definition_parser = leading_trivia_parser . clone () . then (terminal ("enum") . to (Node :: new_token_part (TokenPartKind :: Enum , 4usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . then (leading_trivia_parser . clone () . then (just ('{') . to (Node :: new_token_part (TokenPartKind :: OpenBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: EnumDefinitionIdentifierRepeatedAndCommaRepeated , v))) . then (leading_trivia_parser . clone () . then (just ('}') . to (Node :: new_token_part (TokenPartKind :: CloseBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: EnumDefinitionOpenBraceAndIdentifierRepeatedAndCommaRepeatedAndCloseBrace , vec ! [a , b , c]))) . map (| ((r#enum , identifier) , open_brace_and_identifier_repeated_and_comma_repeated_and_close_brace) | Node :: new_rule (RuleKind :: EnumDefinition , vec ! [r#enum , identifier , open_brace_and_identifier_repeated_and_comma_repeated_and_close_brace])) . boxed () ;

        // ExperimentalPragmaSpecifier = 'experimental' «Identifier» ;
        let experimental_pragma_specifier_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("experimental")
                    .to(Node::new_token_part(TokenPartKind::Experimental, 12usize)),
            )
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|(experimental, identifier)| {
                Node::new_rule(
                    RuleKind::ExperimentalPragmaSpecifier,
                    vec![experimental, identifier],
                )
            })
            .boxed();

        // IdentifierPath = «Identifier»  { '.' «Identifier» } ;
        let identifier_path_parser = leading_trivia_parser
            .clone()
            .then(identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('.').to(Node::new_token_part(TokenPartKind::Period, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Node::new_rule(RuleKind::IdentifierPath, v))
            .boxed();

        // ImportPath = «AsciiStringLiteral» ;
        let import_path_parser = leading_trivia_parser
            .clone()
            .then(ascii_string_literal_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .boxed();

        // NamedArgument = «Identifier» ':' Expression ;
        let named_argument_parser = leading_trivia_parser
            .clone()
            .then(identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(':').to(Node::new_token_part(TokenPartKind::NamedArgumentColon, 1)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .map(|((identifier, colon), expression)| {
                Node::new_rule(RuleKind::NamedArgument, vec![identifier, colon, expression])
            })
            .boxed();

        // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
        let parameter_declaration_parser = type_name_parser
            .clone()
            .then(
                data_location_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(|((type_name, data_location), identifier)| {
                Node::new_rule(
                    RuleKind::ParameterDeclaration,
                    vec![type_name, data_location, identifier],
                )
            })
            .boxed();

        // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
        let selected_import_parser = leading_trivia_parser
            .clone()
            .then(identifier_parser.clone())
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("as").to(Node::new_token_part(TokenPartKind::As, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|(r#as, identifier)| Node::new_anonymous_rule(vec![r#as, identifier]))
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(|(identifier, sequence_0)| {
                Node::new_rule(RuleKind::SelectedImport, vec![identifier, sequence_0])
            })
            .boxed();

        // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
        let user_defined_value_type_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("type").to(Node::new_token_part(TokenPartKind::Type, 4usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("is").to(Node::new_token_part(TokenPartKind::Is, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(elementary_type_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::UserDefinedValueTypeDefinitionSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(
                |((((r#type, identifier), is), elementary_type), semicolon)| {
                    Node::new_rule(
                        RuleKind::UserDefinedValueTypeDefinition,
                        vec![r#type, identifier, is, elementary_type, semicolon],
                    )
                },
            )
            .boxed();

        // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
        let yul_literal_parser = choice((
            leading_trivia_parser
                .clone()
                .then(yul_decimal_number_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(yul_hex_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(ascii_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(boolean_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(hex_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::YulLiteral, vec![v]))
        .boxed();

        // MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
        let mapping_type_parser = leading_trivia_parser
            .clone()
            .then(terminal("mapping").to(Node::new_token_part(TokenPartKind::Mapping, 7usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        choice((
                            elementary_type_parser.clone(),
                            identifier_path_parser.clone(),
                        ))
                        .then(
                            leading_trivia_parser
                                .clone()
                                .then(
                                    terminal("=>").to(Node::new_token_part(
                                        TokenPartKind::EqualGreater,
                                        2usize,
                                    )),
                                )
                                .then(trailing_trivia_parser.clone())
                                .map(Node::new_with_trivia),
                        )
                        .then(type_name_parser.clone())
                        .map(|((choices_1, equal_greater), type_name)| {
                            Node::new_anonymous_rule(vec![choices_1, equal_greater, type_name])
                        }),
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just(')')
                                    .to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| Node::new_anonymous_rule(vec![a, b, c])),
            )
            .map(|(mapping, open_paren_and_sequence_0_and_close_paren)| {
                Node::new_rule(
                    RuleKind::MappingType,
                    vec![mapping, open_paren_and_sequence_0_and_close_paren],
                )
            })
            .boxed();

        // NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
        let named_argument_list_parser = leading_trivia_parser
            .clone()
            .then(just('{').to(Node::new_token_part(TokenPartKind::OpenBrace, 1usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                named_argument_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(Node::new_token_part(TokenPartKind::Comma, 1usize)))
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia)
                            .then(named_argument_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| {
                        Node::new_rule(
                            RuleKind::NamedArgumentListNamedArgumentRepeatedAndCommaRepeated,
                            v,
                        )
                    })
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('}').to(Node::new_token_part(TokenPartKind::CloseBrace, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((a, b), c)| Node::new_rule(RuleKind::NamedArgumentList, vec![a, b, c]))
            .boxed();

        // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
        let override_specifier_parser = leading_trivia_parser . clone () . then (terminal ("override") . to (Node :: new_token_part (TokenPartKind :: Override , 8usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (just ('(') . to (Node :: new_token_part (TokenPartKind :: OpenParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (identifier_path_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (identifier_path_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: OverrideSpecifierIdentifierPathRepeatedAndCommaRepeated , v))) . then (leading_trivia_parser . clone () . then (just (')') . to (Node :: new_token_part (TokenPartKind :: CloseParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: OverrideSpecifierOpenParenAndIdentifierPathRepeatedAndCommaRepeatedAndCloseParen , vec ! [a , b , c])) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . map (| (r#override , open_paren_and_identifier_path_repeated_and_comma_repeated_and_close_paren) | Node :: new_rule (RuleKind :: OverrideSpecifier , vec ! [r#override , open_paren_and_identifier_path_repeated_and_comma_repeated_and_close_paren])) . boxed () ;

        // ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
        let parameter_list_parser = leading_trivia_parser
            .clone()
            .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(Node::new_token_part(TokenPartKind::Comma, 1usize)))
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia)
                            .then(parameter_declaration_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| {
                        Node::new_rule(
                            RuleKind::ParameterListParameterDeclarationRepeatedAndCommaRepeated,
                            v,
                        )
                    })
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(')').to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((a, b), c)| Node::new_rule(RuleKind::ParameterList, vec![a, b, c]))
            .boxed();

        // PragmaDirective = 'pragma' ( VersionPragmaSpecifier | ABICoderPragmaSpecifier | ExperimentalPragmaSpecifier ) ';' ;
        let pragma_directive_parser = leading_trivia_parser
            .clone()
            .then(terminal("pragma").to(Node::new_token_part(TokenPartKind::Pragma, 6usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(choice((
                version_pragma_specifier_parser.clone(),
                abi_coder_pragma_specifier_parser.clone(),
                experimental_pragma_specifier_parser.clone(),
            )))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::PragmaDirectiveSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((pragma, choices_0), semicolon)| {
                Node::new_rule(
                    RuleKind::PragmaDirective,
                    vec![pragma, choices_0, semicolon],
                )
            })
            .boxed();

        // SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
        let selecting_import_directive_parser = leading_trivia_parser . clone () . then (just ('{') . to (Node :: new_token_part (TokenPartKind :: OpenBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (selected_import_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (selected_import_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: SelectingImportDirectiveSelectedImportRepeatedAndCommaRepeated , v))) . then (leading_trivia_parser . clone () . then (just ('}') . to (Node :: new_token_part (TokenPartKind :: CloseBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: SelectingImportDirectiveOpenBraceAndSelectedImportRepeatedAndCommaRepeatedAndCloseBrace , vec ! [a , b , c])) . then (leading_trivia_parser . clone () . then (terminal ("from") . to (Node :: new_token_part (TokenPartKind :: From , 4usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . then (import_path_parser . clone ()) . map (| ((open_brace_and_selected_import_repeated_and_comma_repeated_and_close_brace , from) , import_path) | Node :: new_rule (RuleKind :: SelectingImportDirective , vec ! [open_brace_and_selected_import_repeated_and_comma_repeated_and_close_brace , from , import_path])) . boxed () ;

        // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
        let simple_import_directive_parser = import_path_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("as").to(Node::new_token_part(TokenPartKind::As, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|(r#as, identifier)| Node::new_anonymous_rule(vec![r#as, identifier]))
                    .repeated()
                    .map(Node::new_anonymous_rule),
            )
            .map(|(import_path, sequence_0_repeated)| {
                Node::new_rule(
                    RuleKind::SimpleImportDirective,
                    vec![import_path, sequence_0_repeated],
                )
            })
            .boxed();

        // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
        let star_import_directive_parser = leading_trivia_parser
            .clone()
            .then(just('*').to(Node::new_token_part(
                TokenPartKind::StarImportDirectiveStar,
                1,
            )))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("as").to(Node::new_token_part(TokenPartKind::As, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("from").to(Node::new_token_part(TokenPartKind::From, 4usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(import_path_parser.clone())
            .map(|((((star, r#as), identifier), from), import_path)| {
                Node::new_rule(
                    RuleKind::StarImportDirective,
                    vec![star, r#as, identifier, from, import_path],
                )
            })
            .boxed();

        // YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
        yul_expression_parser.define(
            choice((
                yul_identifier_path_parser.clone(),
                yul_function_call_parser.clone(),
                yul_literal_parser.clone(),
            ))
            .map(|v| Node::new_rule(RuleKind::YulExpression, vec![v]))
            .boxed(),
        );

        // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
        let argument_list_parser = leading_trivia_parser
            .clone()
            .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                choice((
                    positional_argument_list_parser.clone(),
                    named_argument_list_parser.clone(),
                ))
                .or_not()
                .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(')').to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((a, b), c)| Node::new_rule(RuleKind::ArgumentList, vec![a, b, c]))
            .boxed();

        // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
        let catch_clause_parser = leading_trivia_parser
            .clone()
            .then(terminal("catch").to(Node::new_token_part(TokenPartKind::Catch, 5usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none()))
                    .then(parameter_list_parser.clone())
                    .map(|(identifier, parameter_list)| {
                        Node::new_anonymous_rule(vec![identifier, parameter_list])
                    })
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(block_parser.clone())
            .map(|((catch, sequence_0), block)| {
                Node::new_rule(RuleKind::CatchClause, vec![catch, sequence_0, block])
            })
            .boxed();

        // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
        let function_type_parser = leading_trivia_parser
            .clone()
            .then(terminal("function").to(Node::new_token_part(TokenPartKind::Function, 8usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(parameter_list_parser.clone())
            .then(
                choice((
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal("internal")
                                .to(Node::new_token_part(TokenPartKind::Internal, 8usize)),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal("external")
                                .to(Node::new_token_part(TokenPartKind::External, 8usize)),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal("private")
                                .to(Node::new_token_part(TokenPartKind::Private, 7usize)),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal("public")
                                .to(Node::new_token_part(TokenPartKind::Public, 6usize)),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal("pure").to(Node::new_token_part(TokenPartKind::Pure, 4usize)),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal("view").to(Node::new_token_part(TokenPartKind::View, 4usize)),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal("payable")
                                .to(Node::new_token_part(TokenPartKind::Payable, 7usize)),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                ))
                .repeated()
                .map(Node::new_anonymous_rule),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("returns")
                            .to(Node::new_token_part(TokenPartKind::Returns, 7usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(parameter_list_parser.clone())
                    .map(|(returns, parameter_list)| {
                        Node::new_anonymous_rule(vec![returns, parameter_list])
                    })
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(
                |(((function, parameter_list), choices_0_repeated), sequence_1)| {
                    Node::new_rule(
                        RuleKind::FunctionType,
                        vec![function, parameter_list, choices_0_repeated, sequence_1],
                    )
                },
            )
            .boxed();

        // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
        let import_directive_parser = leading_trivia_parser
            .clone()
            .then(terminal("import").to(Node::new_token_part(TokenPartKind::Import, 6usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(choice((
                simple_import_directive_parser.clone(),
                star_import_directive_parser.clone(),
                selecting_import_directive_parser.clone(),
            )))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::ImportDirectiveSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((import, choices_0), semicolon)| {
                Node::new_rule(
                    RuleKind::ImportDirective,
                    vec![import, choices_0, semicolon],
                )
            })
            .boxed();

        // ModifierAttribute = OverrideSpecifier | 'virtual' ;
        let modifier_attribute_parser = choice((
            override_specifier_parser.clone(),
            leading_trivia_parser
                .clone()
                .then(terminal("virtual").to(Node::new_token_part(TokenPartKind::Virtual, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::ModifierAttribute, vec![v]))
        .boxed();

        // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
        let state_variable_attribute_parser = choice((
            override_specifier_parser.clone(),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("constant").to(Node::new_token_part(TokenPartKind::Constant, 8usize)),
                )
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("immutable")
                        .to(Node::new_token_part(TokenPartKind::Immutable, 9usize)),
                )
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("internal").to(Node::new_token_part(TokenPartKind::Internal, 8usize)),
                )
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("private").to(Node::new_token_part(TokenPartKind::Private, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("public").to(Node::new_token_part(TokenPartKind::Public, 6usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::StateVariableAttribute, vec![v]))
        .boxed();

        // YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
        let yul_assignment_statement_parser = yul_identifier_path_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(',').to(Node::new_token_part(TokenPartKind::Comma, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(yul_identifier_path_parser.clone())
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| {
                Node::new_rule(
                    RuleKind::YulAssignmentStatementYulIdentifierPathRepeatedAndCommaRepeated,
                    v,
                )
            })
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal(":=").to(Node::new_token_part(TokenPartKind::ColonEqual, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(yul_expression_parser.clone())
            .map(
                |(
                    (yul_identifier_path_repeated_and_comma_repeated, colon_equal),
                    yul_expression,
                )| {
                    Node::new_rule(
                        RuleKind::YulAssignmentStatement,
                        vec![
                            yul_identifier_path_repeated_and_comma_repeated,
                            colon_equal,
                            yul_expression,
                        ],
                    )
                },
            )
            .boxed();

        // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
        let yul_for_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("for").to(Node::new_token_part(TokenPartKind::For, 3usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(yul_block_parser.clone())
            .then(yul_expression_parser.clone())
            .then(yul_block_parser.clone())
            .then(yul_block_parser.clone())
            .map(
                |((((r#for, yul_block_0_), yul_expression), yul_block_1_), yul_block_2_)| {
                    Node::new_rule(
                        RuleKind::YulForStatement,
                        vec![
                            r#for,
                            yul_block_0_,
                            yul_expression,
                            yul_block_1_,
                            yul_block_2_,
                        ],
                    )
                },
            )
            .boxed();

        // YulIfStatement = 'if' YulExpression YulBlock ;
        let yul_if_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("if").to(Node::new_token_part(TokenPartKind::If, 2usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(yul_expression_parser.clone())
            .then(yul_block_parser.clone())
            .map(|((r#if, yul_expression), yul_block)| {
                Node::new_rule(
                    RuleKind::YulIfStatement,
                    vec![r#if, yul_expression, yul_block],
                )
            })
            .boxed();

        // YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
        let yul_switch_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("switch").to(Node::new_token_part(TokenPartKind::Switch, 6usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(yul_expression_parser.clone())
            .then(
                choice((
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal("case").to(Node::new_token_part(TokenPartKind::Case, 4usize)),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia)
                        .then(yul_literal_parser.clone())
                        .map(|(case, yul_literal)| {
                            Node::new_anonymous_rule(vec![case, yul_literal])
                        }),
                    leading_trivia_parser
                        .clone()
                        .then(
                            terminal("default")
                                .to(Node::new_token_part(TokenPartKind::Default, 7usize)),
                        )
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                ))
                .then(yul_block_parser.clone())
                .map(|(choices_1, yul_block)| Node::new_anonymous_rule(vec![choices_1, yul_block]))
                .repeated()
                .at_least(1usize)
                .map(Node::new_anonymous_rule),
            )
            .map(|((switch, yul_expression), sequence_0_repeated)| {
                Node::new_rule(
                    RuleKind::YulSwitchStatement,
                    vec![switch, yul_expression, sequence_0_repeated],
                )
            })
            .boxed();

        // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
        let yul_variable_declaration_parser = leading_trivia_parser . clone () . then (terminal ("let") . to (Node :: new_token_part (TokenPartKind :: Let , 3usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (yul_identifier_path_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (yul_identifier_path_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: YulVariableDeclarationYulIdentifierPathRepeatedAndCommaRepeated , v))) . then (leading_trivia_parser . clone () . then (terminal (":=") . to (Node :: new_token_part (TokenPartKind :: ColonEqual , 2usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (yul_expression_parser . clone ()) . map (| (colon_equal , yul_expression) | Node :: new_anonymous_rule (vec ! [colon_equal , yul_expression])) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . map (| ((r#let , yul_identifier_path_repeated_and_comma_repeated) , sequence_0) | Node :: new_rule (RuleKind :: YulVariableDeclaration , vec ! [r#let , yul_identifier_path_repeated_and_comma_repeated , sequence_0])) . boxed () ;

        // EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
        let emit_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("emit").to(Node::new_token_part(TokenPartKind::Emit, 4usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(identifier_path_parser.clone())
            .then(argument_list_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::EmitStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|(((emit, identifier_path), argument_list), semicolon)| {
                Node::new_rule(
                    RuleKind::EmitStatement,
                    vec![emit, identifier_path, argument_list, semicolon],
                )
            })
            .boxed();

        // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
        let inheritance_specifier_parser = identifier_path_parser
            .clone()
            .then(
                argument_list_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(|(identifier_path, argument_list)| {
                Node::new_rule(
                    RuleKind::InheritanceSpecifier,
                    vec![identifier_path, argument_list],
                )
            })
            .boxed();

        // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
        let modifier_invocation_parser = identifier_path_parser
            .clone()
            .then(
                argument_list_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(|(identifier_path, argument_list)| {
                Node::new_rule(
                    RuleKind::ModifierInvocation,
                    vec![identifier_path, argument_list],
                )
            })
            .boxed();

        // NewExpression = 'new' IdentifierPath ArgumentList ;
        let new_expression_parser = leading_trivia_parser
            .clone()
            .then(terminal("new").to(Node::new_token_part(TokenPartKind::New, 3usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(identifier_path_parser.clone())
            .then(argument_list_parser.clone())
            .map(|((new, identifier_path), argument_list)| {
                Node::new_rule(
                    RuleKind::NewExpression,
                    vec![new, identifier_path, argument_list],
                )
            })
            .boxed();

        // PayableExpression = 'payable' ArgumentList ;
        let payable_expression_parser = leading_trivia_parser
            .clone()
            .then(terminal("payable").to(Node::new_token_part(TokenPartKind::Payable, 7usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(argument_list_parser.clone())
            .map(|(payable, argument_list)| {
                Node::new_rule(RuleKind::PayableExpression, vec![payable, argument_list])
            })
            .boxed();

        // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
        let revert_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("revert").to(Node::new_token_part(TokenPartKind::Revert, 6usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                identifier_path_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(argument_list_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::RevertStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|(((revert, identifier_path), argument_list), semicolon)| {
                Node::new_rule(
                    RuleKind::RevertStatement,
                    vec![revert, identifier_path, argument_list, semicolon],
                )
            })
            .boxed();

        // TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
        type_name_parser.define(
            choice((
                elementary_type_parser.clone(),
                function_type_parser.clone(),
                mapping_type_parser.clone(),
                identifier_path_parser.clone(),
            ))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('[').to(Node::new_token_part(TokenPartKind::OpenBracket, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        expression_parser
                            .clone()
                            .or_not()
                            .map(|v| v.unwrap_or_else(|| Node::new_none())),
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just(']')
                                    .to(Node::new_token_part(TokenPartKind::CloseBracket, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| {
                        Node::new_rule(
                            RuleKind::TypeNameOpenBracketAndExpressionAndCloseBracket,
                            vec![a, b, c],
                        )
                    })
                    .repeated()
                    .map(|v| {
                        if v.is_empty() {
                            Node::new_none()
                        } else {
                            Node::new_rule(
                                RuleKind::TypeNameOpenBracketAndExpressionAndCloseBracketRepeated,
                                v,
                            )
                        }
                    }),
            )
            .map(
                |(choices_0, open_bracket_and_expression_and_close_bracket_repeated)| {
                    Node::new_rule(
                        RuleKind::TypeName,
                        vec![
                            choices_0,
                            open_bracket_and_expression_and_close_bracket_repeated,
                        ],
                    )
                },
            )
            .boxed(),
        );

        // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
        let yul_statement_parser = choice((
            yul_block_parser.clone(),
            yul_variable_declaration_parser.clone(),
            yul_function_definition_parser.clone(),
            yul_assignment_statement_parser.clone(),
            yul_function_call_parser.clone(),
            yul_if_statement_parser.clone(),
            yul_for_statement_parser.clone(),
            yul_switch_statement_parser.clone(),
            yul_leave_statement_parser.clone(),
            yul_break_statement_parser.clone(),
            yul_continue_statement_parser.clone(),
        ))
        .map(|v| Node::new_rule(RuleKind::YulStatement, vec![v]))
        .boxed();

        // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
        let constructor_attribute_parser = choice((
            modifier_invocation_parser.clone(),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("internal").to(Node::new_token_part(TokenPartKind::Internal, 8usize)),
                )
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("payable").to(Node::new_token_part(TokenPartKind::Payable, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("public").to(Node::new_token_part(TokenPartKind::Public, 6usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::ConstructorAttribute, vec![v]))
        .boxed();

        // ErrorParameter = TypeName [ «Identifier» ] ;
        let error_parameter_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(|(type_name, identifier)| {
                Node::new_rule(RuleKind::ErrorParameter, vec![type_name, identifier])
            })
            .boxed();

        // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
        let event_parameter_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("indexed")
                            .to(Node::new_token_part(TokenPartKind::Indexed, 7usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(|((type_name, indexed), identifier)| {
                Node::new_rule(
                    RuleKind::EventParameter,
                    vec![type_name, indexed, identifier],
                )
            })
            .boxed();

        // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
        let fallback_function_attribute_parser = choice((
            modifier_invocation_parser.clone(),
            override_specifier_parser.clone(),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("external").to(Node::new_token_part(TokenPartKind::External, 8usize)),
                )
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("payable").to(Node::new_token_part(TokenPartKind::Payable, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("pure").to(Node::new_token_part(TokenPartKind::Pure, 4usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("view").to(Node::new_token_part(TokenPartKind::View, 4usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("virtual").to(Node::new_token_part(TokenPartKind::Virtual, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::FallbackFunctionAttribute, vec![v]))
        .boxed();

        // FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
        let function_attribute_parser = choice((
            modifier_invocation_parser.clone(),
            override_specifier_parser.clone(),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("external").to(Node::new_token_part(TokenPartKind::External, 8usize)),
                )
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("internal").to(Node::new_token_part(TokenPartKind::Internal, 8usize)),
                )
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("payable").to(Node::new_token_part(TokenPartKind::Payable, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("private").to(Node::new_token_part(TokenPartKind::Private, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("public").to(Node::new_token_part(TokenPartKind::Public, 6usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("pure").to(Node::new_token_part(TokenPartKind::Pure, 4usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("view").to(Node::new_token_part(TokenPartKind::View, 4usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("virtual").to(Node::new_token_part(TokenPartKind::Virtual, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::FunctionAttribute, vec![v]))
        .boxed();

        // InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
        let inheritance_specifier_list_parser = leading_trivia_parser . clone () . then (terminal ("is") . to (Node :: new_token_part (TokenPartKind :: Is , 2usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (inheritance_specifier_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (inheritance_specifier_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: InheritanceSpecifierListInheritanceSpecifierRepeatedAndCommaRepeated , v))) . map (| (is , inheritance_specifier_repeated_and_comma_repeated) | Node :: new_rule (RuleKind :: InheritanceSpecifierList , vec ! [is , inheritance_specifier_repeated_and_comma_repeated])) . boxed () ;

        // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
        let receive_function_attribute_parser = choice((
            modifier_invocation_parser.clone(),
            override_specifier_parser.clone(),
            leading_trivia_parser
                .clone()
                .then(
                    terminal("external").to(Node::new_token_part(TokenPartKind::External, 8usize)),
                )
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("payable").to(Node::new_token_part(TokenPartKind::Payable, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("virtual").to(Node::new_token_part(TokenPartKind::Virtual, 7usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::ReceiveFunctionAttribute, vec![v]))
        .boxed();

        // StructMember = TypeName «Identifier» ';' ;
        let struct_member_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::StructMemberSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((type_name, identifier), semicolon)| {
                Node::new_rule(
                    RuleKind::StructMember,
                    vec![type_name, identifier, semicolon],
                )
            })
            .boxed();

        // TypeExpression = 'type' '(' TypeName ')' ;
        let type_expression_parser = leading_trivia_parser
            .clone()
            .then(terminal("type").to(Node::new_token_part(TokenPartKind::Type, 4usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(type_name_parser.clone())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just(')')
                                    .to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| {
                        Node::new_rule(
                            RuleKind::TypeExpressionOpenParenAndTypeNameAndCloseParen,
                            vec![a, b, c],
                        )
                    }),
            )
            .map(|(r#type, open_paren_and_type_name_and_close_paren)| {
                Node::new_rule(
                    RuleKind::TypeExpression,
                    vec![r#type, open_paren_and_type_name_and_close_paren],
                )
            })
            .boxed();

        // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
        let using_directive_parser = leading_trivia_parser . clone () . then (terminal ("using") . to (Node :: new_token_part (TokenPartKind :: Using , 5usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (choice ((identifier_path_parser . clone () , leading_trivia_parser . clone () . then (just ('{') . to (Node :: new_token_part (TokenPartKind :: OpenBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (identifier_path_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (identifier_path_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: UsingDirectiveIdentifierPathRepeatedAndCommaRepeated , v))) . then (leading_trivia_parser . clone () . then (just ('}') . to (Node :: new_token_part (TokenPartKind :: CloseBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: UsingDirectiveOpenBraceAndIdentifierPathRepeatedAndCommaRepeatedAndCloseBrace , vec ! [a , b , c]))))) . then (leading_trivia_parser . clone () . then (terminal ("for") . to (Node :: new_token_part (TokenPartKind :: For , 3usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . then (choice ((leading_trivia_parser . clone () . then (just ('*') . to (Node :: new_token_part (TokenPartKind :: UsingDirectiveStar , 1))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) , type_name_parser . clone ()))) . then (leading_trivia_parser . clone () . then (terminal ("global") . to (Node :: new_token_part (TokenPartKind :: Global , 6usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (leading_trivia_parser . clone () . then (just (';') . to (Node :: new_token_part (TokenPartKind :: UsingDirectiveSemicolon , 1))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| (((((using , choices_0) , r#for) , choices_1) , global) , semicolon) | Node :: new_rule (RuleKind :: UsingDirective , vec ! [using , choices_0 , r#for , choices_1 , global , semicolon])) . boxed () ;

        // YulBlock = '{' { YulStatement } '}' ;
        yul_block_parser.define(
            leading_trivia_parser
                .clone()
                .then(just('{').to(Node::new_token_part(TokenPartKind::OpenBrace, 1usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia)
                .then(yul_statement_parser.clone().repeated().map(|v| {
                    if v.is_empty() {
                        Node::new_none()
                    } else {
                        Node::new_rule(RuleKind::YulBlockYulStatementRepeated, v)
                    }
                }))
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('}').to(Node::new_token_part(TokenPartKind::CloseBrace, 1usize)))
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                )
                .map(|((a, b), c)| Node::new_rule(RuleKind::YulBlock, vec![a, b, c]))
                .boxed(),
        );

        // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
        let assembly_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("assembly").to(Node::new_token_part(TokenPartKind::Assembly, 8usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("\"evmasm\"").to(Node::new_token_part(
                        TokenPartKind::DoubleQuoteEvmasmDoubleQuote,
                        8usize,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                assembly_flags_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(yul_block_parser.clone())
            .map(
                |(((assembly, double_quote_evmasm_double_quote), assembly_flags), yul_block)| {
                    Node::new_rule(
                        RuleKind::AssemblyStatement,
                        vec![
                            assembly,
                            double_quote_evmasm_double_quote,
                            assembly_flags,
                            yul_block,
                        ],
                    )
                },
            )
            .boxed();

        // Directive = PragmaDirective | ImportDirective | UsingDirective ;
        let directive_parser = choice((
            pragma_directive_parser.clone(),
            import_directive_parser.clone(),
            using_directive_parser.clone(),
        ))
        .map(|v| Node::new_rule(RuleKind::Directive, vec![v]))
        .boxed();

        // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
        let error_definition_parser = leading_trivia_parser . clone () . then (terminal ("error") . to (Node :: new_token_part (TokenPartKind :: Error , 5usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . then (leading_trivia_parser . clone () . then (just ('(') . to (Node :: new_token_part (TokenPartKind :: OpenParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (error_parameter_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (error_parameter_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: ErrorDefinitionErrorParameterRepeatedAndCommaRepeated , v)) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (leading_trivia_parser . clone () . then (just (')') . to (Node :: new_token_part (TokenPartKind :: CloseParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: ErrorDefinitionOpenParenAndErrorParameterRepeatedAndCommaRepeatedAndCloseParen , vec ! [a , b , c]))) . then (leading_trivia_parser . clone () . then (just (';') . to (Node :: new_token_part (TokenPartKind :: ErrorDefinitionSemicolon , 1))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| (((error , identifier) , open_paren_and_error_parameter_repeated_and_comma_repeated_and_close_paren) , semicolon) | Node :: new_rule (RuleKind :: ErrorDefinition , vec ! [error , identifier , open_paren_and_error_parameter_repeated_and_comma_repeated_and_close_paren , semicolon])) . boxed () ;

        // EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
        let event_definition_parser = leading_trivia_parser . clone () . then (terminal ("event") . to (Node :: new_token_part (TokenPartKind :: Event , 5usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . then (leading_trivia_parser . clone () . then (just ('(') . to (Node :: new_token_part (TokenPartKind :: OpenParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (event_parameter_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (event_parameter_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: EventDefinitionEventParameterRepeatedAndCommaRepeated , v)) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (leading_trivia_parser . clone () . then (just (')') . to (Node :: new_token_part (TokenPartKind :: CloseParen , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: EventDefinitionOpenParenAndEventParameterRepeatedAndCommaRepeatedAndCloseParen , vec ! [a , b , c]))) . then (leading_trivia_parser . clone () . then (terminal ("anonymous") . to (Node :: new_token_part (TokenPartKind :: Anonymous , 9usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (leading_trivia_parser . clone () . then (just (';') . to (Node :: new_token_part (TokenPartKind :: EventDefinitionSemicolon , 1))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((((event , identifier) , open_paren_and_event_parameter_repeated_and_comma_repeated_and_close_paren) , anonymous) , semicolon) | Node :: new_rule (RuleKind :: EventDefinition , vec ! [event , identifier , open_paren_and_event_parameter_repeated_and_comma_repeated_and_close_paren , anonymous , semicolon])) . boxed () ;

        // PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | «NumericLiteral» | «BooleanLiteral» | «Identifier» ;
        let primary_expression_parser = choice((
            payable_expression_parser.clone(),
            type_expression_parser.clone(),
            new_expression_parser.clone(),
            parenthesis_expression_parser.clone(),
            array_literal_parser.clone(),
            leading_trivia_parser
                .clone()
                .then(ascii_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(unicode_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(hex_string_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(numeric_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(boolean_literal_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(identifier_parser.clone())
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .map(|v| Node::new_rule(RuleKind::PrimaryExpression, vec![v]))
        .boxed();

        // StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
        let struct_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("struct").to(Node::new_token_part(TokenPartKind::Struct, 6usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('{').to(Node::new_token_part(TokenPartKind::OpenBrace, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        struct_member_parser
                            .clone()
                            .repeated()
                            .at_least(1usize)
                            .map(|v| {
                                Node::new_rule(RuleKind::StructDefinitionStructMemberRepeated, v)
                            }),
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just('}')
                                    .to(Node::new_token_part(TokenPartKind::CloseBrace, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| {
                        Node::new_rule(
                            RuleKind::StructDefinitionOpenBraceAndStructMemberRepeatedAndCloseBrace,
                            vec![a, b, c],
                        )
                    }),
            )
            .map(
                |(
                    (r#struct, identifier),
                    open_brace_and_struct_member_repeated_and_close_brace,
                )| {
                    Node::new_rule(
                        RuleKind::StructDefinition,
                        vec![
                            r#struct,
                            identifier,
                            open_brace_and_struct_member_repeated_and_close_brace,
                        ],
                    )
                },
            )
            .boxed();

        // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
        let index_access_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('[').to(Node::new_token_part(TokenPartKind::OpenBracket, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        expression_parser
                            .clone()
                            .or_not()
                            .map(|v| v.unwrap_or_else(|| Node::new_none()))
                            .then(
                                leading_trivia_parser
                                    .clone()
                                    .then(just(':').to(Node::new_token_part(
                                        TokenPartKind::IndexAccessExpressionColon,
                                        1,
                                    )))
                                    .then(trailing_trivia_parser.clone())
                                    .map(Node::new_with_trivia)
                                    .then(
                                        expression_parser
                                            .clone()
                                            .or_not()
                                            .map(|v| v.unwrap_or_else(|| Node::new_none())),
                                    )
                                    .map(|(colon, expression)| {
                                        Node::new_anonymous_rule(vec![colon, expression])
                                    })
                                    .or_not()
                                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
                            )
                            .map(|(expression, sequence_1)| {
                                Node::new_anonymous_rule(vec![expression, sequence_1])
                            }),
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just(']')
                                    .to(Node::new_token_part(TokenPartKind::CloseBracket, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| Node::new_anonymous_rule(vec![a, b, c])),
            )
            .map(
                |(expression, open_bracket_and_sequence_0_and_close_bracket)| {
                    Node::new_rule(
                        RuleKind::IndexAccessExpressionAnonexpfrag3,
                        vec![expression, open_bracket_and_sequence_0_and_close_bracket],
                    )
                },
            )
            .boxed();

        // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
        let member_access_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('.').to(Node::new_token_part(
                        TokenPartKind::MemberAccessExpressionPeriod,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("address")
                            .to(Node::new_token_part(TokenPartKind::Address, 7usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )))
            .map(|((expression, period), choices_0)| {
                Node::new_rule(
                    RuleKind::MemberAccessExpressionAnonexpfrag4,
                    vec![expression, period, choices_0],
                )
            })
            .boxed();

        // FunctionCallExpression = Expression [ '{' NamedArgument  { ',' NamedArgument } '}' ] ArgumentList ;
        let function_call_expression_parser = expression_parser . clone () . then (leading_trivia_parser . clone () . then (just ('{') . to (Node :: new_token_part (TokenPartKind :: OpenBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (named_argument_parser . clone () . then (leading_trivia_parser . clone () . then (just (',') . to (Node :: new_token_part (TokenPartKind :: Comma , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (named_argument_parser . clone ()) . repeated ()) . map (repetition_mapper) . map (| v | Node :: new_rule (RuleKind :: FunctionCallExpressionNamedArgumentRepeatedAndCommaRepeated , v))) . then (leading_trivia_parser . clone () . then (just ('}') . to (Node :: new_token_part (TokenPartKind :: CloseBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: FunctionCallExpressionOpenBraceAndNamedArgumentRepeatedAndCommaRepeatedAndCloseBrace , vec ! [a , b , c])) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (argument_list_parser . clone ()) . map (| ((expression , open_brace_and_named_argument_repeated_and_comma_repeated_and_close_brace) , argument_list) | Node :: new_rule (RuleKind :: FunctionCallExpressionAnonexpfrag4 , vec ! [expression , open_brace_and_named_argument_repeated_and_comma_repeated_and_close_brace , argument_list])) . boxed () ;

        // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
        let unary_prefix_expression_parser = choice((
            leading_trivia_parser
                .clone()
                .then(terminal("++").to(Node::new_token_part(TokenPartKind::PlusPlus, 2usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(terminal("--").to(Node::new_token_part(TokenPartKind::MinusMinus, 2usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(just('!').to(Node::new_token_part(
                    TokenPartKind::UnaryPrefixExpressionBang,
                    1,
                )))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(just('~').to(Node::new_token_part(
                    TokenPartKind::UnaryPrefixExpressionTilde,
                    1,
                )))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
            leading_trivia_parser
                .clone()
                .then(just('-').to(Node::new_token_part(
                    TokenPartKind::UnaryPrefixExpressionMinus,
                    1,
                )))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia),
        ))
        .then(expression_parser.clone())
        .map(|(choices_0, expression)| {
            Node::new_rule(
                RuleKind::UnaryPrefixExpressionAnonexpfrag3,
                vec![choices_0, expression],
            )
        })
        .boxed();

        // UnarySuffixExpression = Expression ( '++' | '--' ) ;
        let unary_suffix_expression_parser = expression_parser
            .clone()
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(terminal("++").to(Node::new_token_part(TokenPartKind::PlusPlus, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("--").to(Node::new_token_part(TokenPartKind::MinusMinus, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )))
            .map(|(expression, choices_0)| {
                Node::new_rule(
                    RuleKind::UnarySuffixExpressionAnonexpfrag3,
                    vec![expression, choices_0],
                )
            })
            .boxed();

        // ExponentiationExpression = Expression '**' Expression ;
        let exponentiation_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("**").to(Node::new_token_part(TokenPartKind::StarStar, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .map(|((expression_0_, star_star), expression_1_)| {
                Node::new_rule(
                    RuleKind::ExponentiationExpressionAnonexpfrag4,
                    vec![expression_0_, star_star, expression_1_],
                )
            })
            .boxed();

        // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
        let mul_div_mod_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        filter(|&c: &char| c == '*' || c == '/' || c == '%')
                            .to(Node::new_anonymous_token(1)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .map(|((expression_0_, filter_0), expression_1_)| {
                Node::new_rule(
                    RuleKind::MulDivModExpressionAnonexpfrag4,
                    vec![expression_0_, filter_0, expression_1_],
                )
            })
            .boxed();

        // AddSubExpression = Expression ( '+' | '-' ) Expression ;
        let add_sub_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(filter(|&c: &char| c == '+' || c == '-').to(Node::new_anonymous_token(1)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .map(|((expression_0_, filter_0), expression_1_)| {
                Node::new_rule(
                    RuleKind::AddSubExpressionAnonexpfrag4,
                    vec![expression_0_, filter_0, expression_1_],
                )
            })
            .boxed();

        // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
        let shift_expression_parser = expression_parser
            .clone()
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(terminal("<<").to(Node::new_token_part(TokenPartKind::LessLess, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal(">>")
                            .to(Node::new_token_part(TokenPartKind::GreaterGreater, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(terminal(">>>").to(Node::new_token_part(
                        TokenPartKind::GreaterGreaterGreater,
                        3usize,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )))
            .then(expression_parser.clone())
            .map(|((expression_0_, choices_0), expression_1_)| {
                Node::new_rule(
                    RuleKind::ShiftExpressionAnonexpfrag4,
                    vec![expression_0_, choices_0, expression_1_],
                )
            })
            .boxed();

        // BitAndExpression = Expression '&' Expression ;
        let bit_and_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('&').to(Node::new_token_part(
                        TokenPartKind::BitAndExpressionAmpersand,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .map(|((expression_0_, ampersand), expression_1_)| {
                Node::new_rule(
                    RuleKind::BitAndExpressionAnonexpfrag4,
                    vec![expression_0_, ampersand, expression_1_],
                )
            })
            .boxed();

        // BitXOrExpression = Expression '^' Expression ;
        let bit_x_or_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('^').to(Node::new_token_part(
                        TokenPartKind::BitXOrExpressionCaret,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .map(|((expression_0_, caret), expression_1_)| {
                Node::new_rule(
                    RuleKind::BitXOrExpressionAnonexpfrag4,
                    vec![expression_0_, caret, expression_1_],
                )
            })
            .boxed();

        // BitOrExpression = Expression '|' Expression ;
        let bit_or_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('|').to(Node::new_token_part(TokenPartKind::BitOrExpressionPipe, 1)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .map(|((expression_0_, pipe), expression_1_)| {
                Node::new_rule(
                    RuleKind::BitOrExpressionAnonexpfrag4,
                    vec![expression_0_, pipe, expression_1_],
                )
            })
            .boxed();

        // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
        let order_comparison_expression_parser = expression_parser
            .clone()
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(just('<').to(Node::new_token_part(
                        TokenPartKind::OrderComparisonExpressionLess,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(just('>').to(Node::new_token_part(
                        TokenPartKind::OrderComparisonExpressionGreater,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(terminal("<=").to(Node::new_token_part(TokenPartKind::LessEqual, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal(">=")
                            .to(Node::new_token_part(TokenPartKind::GreaterEqual, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )))
            .then(expression_parser.clone())
            .map(|((expression_0_, choices_0), expression_1_)| {
                Node::new_rule(
                    RuleKind::OrderComparisonExpressionAnonexpfrag4,
                    vec![expression_0_, choices_0, expression_1_],
                )
            })
            .boxed();

        // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
        let equality_comparison_expression_parser = expression_parser
            .clone()
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("==").to(Node::new_token_part(TokenPartKind::EqualEqual, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(terminal("!=").to(Node::new_token_part(TokenPartKind::BangEqual, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )))
            .then(expression_parser.clone())
            .map(|((expression_0_, choices_0), expression_1_)| {
                Node::new_rule(
                    RuleKind::EqualityComparisonExpressionAnonexpfrag4,
                    vec![expression_0_, choices_0, expression_1_],
                )
            })
            .boxed();

        // AndExpression = Expression '&&' Expression ;
        let and_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("&&").to(Node::new_token_part(
                        TokenPartKind::AmpersandAmpersand,
                        2usize,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .map(|((expression_0_, ampersand_ampersand), expression_1_)| {
                Node::new_rule(
                    RuleKind::AndExpressionAnonexpfrag4,
                    vec![expression_0_, ampersand_ampersand, expression_1_],
                )
            })
            .boxed();

        // OrExpression = Expression '||' Expression ;
        let or_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("||").to(Node::new_token_part(TokenPartKind::PipePipe, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .map(|((expression_0_, pipe_pipe), expression_1_)| {
                Node::new_rule(
                    RuleKind::OrExpressionAnonexpfrag4,
                    vec![expression_0_, pipe_pipe, expression_1_],
                )
            })
            .boxed();

        // ConditionalExpression = Expression '?' Expression ':' Expression ;
        let conditional_expression_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('?').to(Node::new_token_part(
                        TokenPartKind::ConditionalExpressionQuestion,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(expression_parser.clone())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(':').to(Node::new_token_part(
                                TokenPartKind::ConditionalExpressionColon,
                                1,
                            )))
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .then(expression_parser.clone())
                    .map(|(((question, expression_0_), colon), expression_1_)| {
                        Node::new_anonymous_rule(vec![
                            question,
                            expression_0_,
                            colon,
                            expression_1_,
                        ])
                    }),
            )
            .map(|(expression, sequence_0)| {
                Node::new_rule(
                    RuleKind::ConditionalExpressionAnonexpfrag3,
                    vec![expression, sequence_0],
                )
            })
            .boxed();

        // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
        let assignment_expression_parser = expression_parser
            .clone()
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(just('=').to(Node::new_token_part(
                        TokenPartKind::AssignmentExpressionEqual,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(terminal("|=").to(Node::new_token_part(TokenPartKind::PipeEqual, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("^=").to(Node::new_token_part(TokenPartKind::CaretEqual, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("&=")
                            .to(Node::new_token_part(TokenPartKind::AmpersandEqual, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("<<=")
                            .to(Node::new_token_part(TokenPartKind::LessLessEqual, 3usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(terminal(">>=").to(Node::new_token_part(
                        TokenPartKind::GreaterGreaterEqual,
                        3usize,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(terminal(">>>=").to(Node::new_token_part(
                        TokenPartKind::GreaterGreaterGreaterEqual,
                        4usize,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(terminal("+=").to(Node::new_token_part(TokenPartKind::PlusEqual, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("-=").to(Node::new_token_part(TokenPartKind::MinusEqual, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(terminal("*=").to(Node::new_token_part(TokenPartKind::StarEqual, 2usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("/=").to(Node::new_token_part(TokenPartKind::SlashEqual, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("%=")
                            .to(Node::new_token_part(TokenPartKind::PercentEqual, 2usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )))
            .then(expression_parser.clone())
            .map(|((expression_0_, choices_0), expression_1_)| {
                Node::new_rule(
                    RuleKind::AssignmentExpressionAnonexpfrag4,
                    vec![expression_0_, choices_0, expression_1_],
                )
            })
            .boxed();

        // Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
        expression_parser.define(
            assignment_expression_parser
                .clone()
                .map(|v| Node::new_rule(RuleKind::Expression, vec![v]))
                .boxed(),
        );

        // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
        let constant_definition_parser = type_name_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("constant")
                            .to(Node::new_token_part(TokenPartKind::Constant, 8usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('=').to(Node::new_token_part(
                        TokenPartKind::ConstantDefinitionEqual,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::ConstantDefinitionSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(
                |(((((type_name, constant), identifier), equal), expression), semicolon)| {
                    Node::new_rule(
                        RuleKind::ConstantDefinition,
                        vec![
                            type_name, constant, identifier, equal, expression, semicolon,
                        ],
                    )
                },
            )
            .boxed();

        // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
        let do_while_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("do").to(Node::new_token_part(TokenPartKind::Do, 2usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(statement_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("while").to(Node::new_token_part(TokenPartKind::While, 5usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(expression_parser.clone())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just(')')
                                    .to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| {
                        Node::new_rule(
                            RuleKind::DoWhileStatementOpenParenAndExpressionAndCloseParen,
                            vec![a, b, c],
                        )
                    }),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::DoWhileStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(
                |(
                    (((r#do, statement), r#while), open_paren_and_expression_and_close_paren),
                    semicolon,
                )| {
                    Node::new_rule(
                        RuleKind::DoWhileStatement,
                        vec![
                            r#do,
                            statement,
                            r#while,
                            open_paren_and_expression_and_close_paren,
                            semicolon,
                        ],
                    )
                },
            )
            .boxed();

        // ExpressionStatement = Expression ';' ;
        let expression_statement_parser = expression_parser
            .clone()
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::ExpressionStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|(expression, semicolon)| {
                Node::new_rule(RuleKind::ExpressionStatement, vec![expression, semicolon])
            })
            .boxed();

        // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
        let if_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("if").to(Node::new_token_part(TokenPartKind::If, 2usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(expression_parser.clone())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just(')')
                                    .to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| {
                        Node::new_rule(
                            RuleKind::IfStatementOpenParenAndExpressionAndCloseParen,
                            vec![a, b, c],
                        )
                    }),
            )
            .then(statement_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(terminal("else").to(Node::new_token_part(TokenPartKind::Else, 4usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(statement_parser.clone())
                    .map(|(r#else, statement)| Node::new_anonymous_rule(vec![r#else, statement]))
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .map(
                |(((r#if, open_paren_and_expression_and_close_paren), statement), sequence_0)| {
                    Node::new_rule(
                        RuleKind::IfStatement,
                        vec![
                            r#if,
                            open_paren_and_expression_and_close_paren,
                            statement,
                            sequence_0,
                        ],
                    )
                },
            )
            .boxed();

        // ReturnStatement = 'return' [ Expression ] ';' ;
        let return_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("return").to(Node::new_token_part(TokenPartKind::Return, 6usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                expression_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::ReturnStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((r#return, expression), semicolon)| {
                Node::new_rule(
                    RuleKind::ReturnStatement,
                    vec![r#return, expression, semicolon],
                )
            })
            .boxed();

        // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
        let state_variable_declaration_parser = type_name_parser
            .clone()
            .then(state_variable_attribute_parser.clone().repeated().map(|v| {
                if v.is_empty() {
                    Node::new_none()
                } else {
                    Node::new_rule(
                        RuleKind::StateVariableDeclarationStateVariableAttributeRepeated,
                        v,
                    )
                }
            }))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('=').to(Node::new_token_part(
                        TokenPartKind::StateVariableDeclarationEqual,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(expression_parser.clone())
                    .map(|(equal, expression)| Node::new_anonymous_rule(vec![equal, expression]))
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::StateVariableDeclarationSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(
                |(
                    (((type_name, state_variable_attribute_repeated), identifier), sequence_0),
                    semicolon,
                )| {
                    Node::new_rule(
                        RuleKind::StateVariableDeclaration,
                        vec![
                            type_name,
                            state_variable_attribute_repeated,
                            identifier,
                            sequence_0,
                            semicolon,
                        ],
                    )
                },
            )
            .boxed();

        // TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
        let try_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("try").to(Node::new_token_part(TokenPartKind::Try, 3usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(expression_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("returns")
                            .to(Node::new_token_part(TokenPartKind::Returns, 7usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(parameter_list_parser.clone())
                    .map(|(returns, parameter_list)| {
                        Node::new_anonymous_rule(vec![returns, parameter_list])
                    })
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(block_parser.clone())
            .then(
                catch_clause_parser
                    .clone()
                    .repeated()
                    .at_least(1usize)
                    .map(|v| Node::new_rule(RuleKind::TryStatementCatchClauseRepeated, v)),
            )
            .map(
                |((((r#try, expression), sequence_0), block), catch_clause_repeated)| {
                    Node::new_rule(
                        RuleKind::TryStatement,
                        vec![r#try, expression, sequence_0, block, catch_clause_repeated],
                    )
                },
            )
            .boxed();

        // TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
        let tuple_deconstruction_statement_parser = leading_trivia_parser
            .clone()
            .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                type_name_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none()))
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(identifier_parser.clone())
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|(type_name, identifier)| {
                        Node::new_anonymous_rule(vec![type_name, identifier])
                    })
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none()))
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(just(',').to(Node::new_token_part(TokenPartKind::Comma, 1usize)))
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia)
                            .then(
                                type_name_parser
                                    .clone()
                                    .or_not()
                                    .map(|v| v.unwrap_or_else(|| Node::new_none()))
                                    .then(
                                        leading_trivia_parser
                                            .clone()
                                            .then(identifier_parser.clone())
                                            .then(trailing_trivia_parser.clone())
                                            .map(Node::new_with_trivia),
                                    )
                                    .map(|(type_name, identifier)| {
                                        Node::new_anonymous_rule(vec![type_name, identifier])
                                    })
                                    .or_not()
                                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(Node::new_anonymous_rule)
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(')').to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(|((a, b), c)| Node::new_anonymous_rule(vec![a, b, c]))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('=').to(Node::new_token_part(
                        TokenPartKind::TupleDeconstructionStatementEqual,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(expression_parser.clone())
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::TupleDeconstructionStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(
                |(
                    (
                        (
                            open_paren_and_sequence_0_repeated_and_comma_repeated_and_close_paren,
                            equal,
                        ),
                        expression,
                    ),
                    semicolon,
                )| {
                    Node::new_rule(
                        RuleKind::TupleDeconstructionStatement,
                        vec![
                            open_paren_and_sequence_0_repeated_and_comma_repeated_and_close_paren,
                            equal,
                            expression,
                            semicolon,
                        ],
                    )
                },
            )
            .boxed();

        // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
        let variable_declaration_statement_parser = type_name_parser
            .clone()
            .then(
                data_location_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('=').to(Node::new_token_part(
                        TokenPartKind::VariableDeclarationStatementEqual,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(expression_parser.clone())
                    .map(|(equal, expression)| Node::new_anonymous_rule(vec![equal, expression]))
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::VariableDeclarationStatementSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .map(
                |((((type_name, data_location), identifier), sequence_0), semicolon)| {
                    Node::new_rule(
                        RuleKind::VariableDeclarationStatement,
                        vec![type_name, data_location, identifier, sequence_0, semicolon],
                    )
                },
            )
            .boxed();

        // WhileStatement = 'while' '(' Expression ')' Statement ;
        let while_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("while").to(Node::new_token_part(TokenPartKind::While, 5usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(expression_parser.clone())
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just(')')
                                    .to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| {
                        Node::new_rule(
                            RuleKind::WhileStatementOpenParenAndExpressionAndCloseParen,
                            vec![a, b, c],
                        )
                    }),
            )
            .then(statement_parser.clone())
            .map(
                |((r#while, open_paren_and_expression_and_close_paren), statement)| {
                    Node::new_rule(
                        RuleKind::WhileStatement,
                        vec![
                            r#while,
                            open_paren_and_expression_and_close_paren,
                            statement,
                        ],
                    )
                },
            )
            .boxed();

        // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
        let simple_statement_parser = choice((
            tuple_deconstruction_statement_parser.clone(),
            variable_declaration_statement_parser.clone(),
            expression_statement_parser.clone(),
        ))
        .map(|v| Node::new_rule(RuleKind::SimpleStatement, vec![v]))
        .boxed();

        // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
        let for_statement_parser = leading_trivia_parser
            .clone()
            .then(terminal("for").to(Node::new_token_part(TokenPartKind::For, 3usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(just('(').to(Node::new_token_part(TokenPartKind::OpenParen, 1usize)))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(
                        choice((
                            simple_statement_parser.clone(),
                            leading_trivia_parser
                                .clone()
                                .then(just(';').to(Node::new_token_part(
                                    TokenPartKind::ForStatementSemicolon,
                                    1,
                                )))
                                .then(trailing_trivia_parser.clone())
                                .map(Node::new_with_trivia),
                        ))
                        .then(choice((
                            expression_statement_parser.clone(),
                            leading_trivia_parser
                                .clone()
                                .then(just(';').to(Node::new_token_part(
                                    TokenPartKind::ForStatementSemicolon,
                                    1,
                                )))
                                .then(trailing_trivia_parser.clone())
                                .map(Node::new_with_trivia),
                        )))
                        .then(
                            expression_parser
                                .clone()
                                .or_not()
                                .map(|v| v.unwrap_or_else(|| Node::new_none())),
                        )
                        .map(|((choices_1, choices_2), expression)| {
                            Node::new_anonymous_rule(vec![choices_1, choices_2, expression])
                        }),
                    )
                    .then(
                        leading_trivia_parser
                            .clone()
                            .then(
                                just(')')
                                    .to(Node::new_token_part(TokenPartKind::CloseParen, 1usize)),
                            )
                            .then(trailing_trivia_parser.clone())
                            .map(Node::new_with_trivia),
                    )
                    .map(|((a, b), c)| Node::new_anonymous_rule(vec![a, b, c])),
            )
            .then(statement_parser.clone())
            .map(
                |((r#for, open_paren_and_sequence_0_and_close_paren), statement)| {
                    Node::new_rule(
                        RuleKind::ForStatement,
                        vec![r#for, open_paren_and_sequence_0_and_close_paren, statement],
                    )
                },
            )
            .boxed();

        // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
        statement_parser.define(
            choice((
                block_parser.clone(),
                simple_statement_parser.clone(),
                if_statement_parser.clone(),
                for_statement_parser.clone(),
                while_statement_parser.clone(),
                do_while_statement_parser.clone(),
                continue_statement_parser.clone(),
                break_statement_parser.clone(),
                try_statement_parser.clone(),
                return_statement_parser.clone(),
                emit_statement_parser.clone(),
                revert_statement_parser.clone(),
                delete_statement_parser.clone(),
                assembly_statement_parser.clone(),
            ))
            .map(|v| Node::new_rule(RuleKind::Statement, vec![v]))
            .boxed(),
        );

        // Block = '{' { Statement | UncheckedBlock } '}' ;
        block_parser.define(
            leading_trivia_parser
                .clone()
                .then(just('{').to(Node::new_token_part(TokenPartKind::OpenBrace, 1usize)))
                .then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia)
                .then(
                    choice((statement_parser.clone(), unchecked_block_parser.clone()))
                        .repeated()
                        .map(Node::new_anonymous_rule),
                )
                .then(
                    leading_trivia_parser
                        .clone()
                        .then(just('}').to(Node::new_token_part(TokenPartKind::CloseBrace, 1usize)))
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia),
                )
                .map(|((a, b), c)| Node::new_rule(RuleKind::Block, vec![a, b, c]))
                .boxed(),
        );

        // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
        let constructor_definition_parser = leading_trivia_parser
            .clone()
            .then(
                terminal("constructor")
                    .to(Node::new_token_part(TokenPartKind::Constructor, 11usize)),
            )
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(parameter_list_parser.clone())
            .then(constructor_attribute_parser.clone().repeated().map(|v| {
                if v.is_empty() {
                    Node::new_none()
                } else {
                    Node::new_rule(
                        RuleKind::ConstructorDefinitionConstructorAttributeRepeated,
                        v,
                    )
                }
            }))
            .then(block_parser.clone())
            .map(
                |(((constructor, parameter_list), constructor_attribute_repeated), block)| {
                    Node::new_rule(
                        RuleKind::ConstructorDefinition,
                        vec![
                            constructor,
                            parameter_list,
                            constructor_attribute_repeated,
                            block,
                        ],
                    )
                },
            )
            .boxed();

        // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
        let fallback_function_definition_parser = leading_trivia_parser . clone () . then (terminal ("fallback") . to (Node :: new_token_part (TokenPartKind :: Fallback , 8usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (parameter_list_parser . clone ()) . then (fallback_function_attribute_parser . clone () . repeated () . map (| v | if v . is_empty () { Node :: new_none () } else { Node :: new_rule (RuleKind :: FallbackFunctionDefinitionFallbackFunctionAttributeRepeated , v) })) . then (leading_trivia_parser . clone () . then (terminal ("returns") . to (Node :: new_token_part (TokenPartKind :: Returns , 7usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (parameter_list_parser . clone ()) . map (| (returns , parameter_list) | Node :: new_anonymous_rule (vec ! [returns , parameter_list])) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (choice ((leading_trivia_parser . clone () . then (just (';') . to (Node :: new_token_part (TokenPartKind :: FallbackFunctionDefinitionSemicolon , 1))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) , block_parser . clone ()))) . map (| ((((fallback , parameter_list) , fallback_function_attribute_repeated) , sequence_0) , choices_1) | Node :: new_rule (RuleKind :: FallbackFunctionDefinition , vec ! [fallback , parameter_list , fallback_function_attribute_repeated , sequence_0 , choices_1])) . boxed () ;

        // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
        let function_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("function").to(Node::new_token_part(TokenPartKind::Function, 8usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("fallback")
                            .to(Node::new_token_part(TokenPartKind::Fallback, 8usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("receive")
                            .to(Node::new_token_part(TokenPartKind::Receive, 7usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )))
            .then(parameter_list_parser.clone())
            .then(function_attribute_parser.clone().repeated().map(|v| {
                if v.is_empty() {
                    Node::new_none()
                } else {
                    Node::new_rule(RuleKind::FunctionDefinitionFunctionAttributeRepeated, v)
                }
            }))
            .then(
                leading_trivia_parser
                    .clone()
                    .then(
                        terminal("returns")
                            .to(Node::new_token_part(TokenPartKind::Returns, 7usize)),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia)
                    .then(parameter_list_parser.clone())
                    .map(|(returns, parameter_list)| {
                        Node::new_anonymous_rule(vec![returns, parameter_list])
                    })
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::FunctionDefinitionSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                block_parser.clone(),
            )))
            .map(
                |(
                    (
                        (((function, choices_0), parameter_list), function_attribute_repeated),
                        sequence_1,
                    ),
                    choices_2,
                )| {
                    Node::new_rule(
                        RuleKind::FunctionDefinition,
                        vec![
                            function,
                            choices_0,
                            parameter_list,
                            function_attribute_repeated,
                            sequence_1,
                            choices_2,
                        ],
                    )
                },
            )
            .boxed();

        // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
        let modifier_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("modifier").to(Node::new_token_part(TokenPartKind::Modifier, 8usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(
                leading_trivia_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
            )
            .then(
                parameter_list_parser
                    .clone()
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| Node::new_none())),
            )
            .then(modifier_attribute_parser.clone().repeated().map(|v| {
                if v.is_empty() {
                    Node::new_none()
                } else {
                    Node::new_rule(RuleKind::ModifierDefinitionModifierAttributeRepeated, v)
                }
            }))
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::ModifierDefinitionSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                block_parser.clone(),
            )))
            .map(
                |(
                    (((modifier, identifier), parameter_list), modifier_attribute_repeated),
                    choices_0,
                )| {
                    Node::new_rule(
                        RuleKind::ModifierDefinition,
                        vec![
                            modifier,
                            identifier,
                            parameter_list,
                            modifier_attribute_repeated,
                            choices_0,
                        ],
                    )
                },
            )
            .boxed();

        // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
        let receive_function_definition_parser = leading_trivia_parser
            .clone()
            .then(terminal("receive").to(Node::new_token_part(TokenPartKind::Receive, 7usize)))
            .then(trailing_trivia_parser.clone())
            .map(Node::new_with_trivia)
            .then(parameter_list_parser.clone())
            .then(
                receive_function_attribute_parser
                    .clone()
                    .repeated()
                    .map(|v| {
                        if v.is_empty() {
                            Node::new_none()
                        } else {
                            Node::new_rule(
                                RuleKind::ReceiveFunctionDefinitionReceiveFunctionAttributeRepeated,
                                v,
                            )
                        }
                    }),
            )
            .then(choice((
                leading_trivia_parser
                    .clone()
                    .then(just(';').to(Node::new_token_part(
                        TokenPartKind::ReceiveFunctionDefinitionSemicolon,
                        1,
                    )))
                    .then(trailing_trivia_parser.clone())
                    .map(Node::new_with_trivia),
                block_parser.clone(),
            )))
            .map(
                |(((receive, parameter_list), receive_function_attribute_repeated), choices_0)| {
                    Node::new_rule(
                        RuleKind::ReceiveFunctionDefinition,
                        vec![
                            receive,
                            parameter_list,
                            receive_function_attribute_repeated,
                            choices_0,
                        ],
                    )
                },
            )
            .boxed();

        // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
        let contract_body_element_parser = choice((
            using_directive_parser.clone(),
            constructor_definition_parser.clone(),
            function_definition_parser.clone(),
            fallback_function_definition_parser.clone(),
            receive_function_definition_parser.clone(),
            modifier_definition_parser.clone(),
            struct_definition_parser.clone(),
            enum_definition_parser.clone(),
            user_defined_value_type_definition_parser.clone(),
            event_definition_parser.clone(),
            error_definition_parser.clone(),
            state_variable_declaration_parser.clone(),
        ))
        .map(|v| Node::new_rule(RuleKind::ContractBodyElement, vec![v]))
        .boxed();

        // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let contract_definition_parser = leading_trivia_parser . clone () . then (terminal ("abstract") . to (Node :: new_token_part (TokenPartKind :: Abstract , 8usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ())) . then (leading_trivia_parser . clone () . then (terminal ("contract") . to (Node :: new_token_part (TokenPartKind :: Contract , 8usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . then (inheritance_specifier_list_parser . clone () . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (leading_trivia_parser . clone () . then (just ('{') . to (Node :: new_token_part (TokenPartKind :: OpenBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (contract_body_element_parser . clone () . repeated () . map (| v | if v . is_empty () { Node :: new_none () } else { Node :: new_rule (RuleKind :: ContractDefinitionContractBodyElementRepeated , v) })) . then (leading_trivia_parser . clone () . then (just ('}') . to (Node :: new_token_part (TokenPartKind :: CloseBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: ContractDefinitionOpenBraceAndContractBodyElementRepeatedAndCloseBrace , vec ! [a , b , c]))) . map (| ((((r#abstract , contract) , identifier) , inheritance_specifier_list) , open_brace_and_contract_body_element_repeated_and_close_brace) | Node :: new_rule (RuleKind :: ContractDefinition , vec ! [r#abstract , contract , identifier , inheritance_specifier_list , open_brace_and_contract_body_element_repeated_and_close_brace])) . boxed () ;

        // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let interface_definition_parser = leading_trivia_parser . clone () . then (terminal ("interface") . to (Node :: new_token_part (TokenPartKind :: Interface , 9usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . then (inheritance_specifier_list_parser . clone () . or_not () . map (| v | v . unwrap_or_else (|| Node :: new_none ()))) . then (leading_trivia_parser . clone () . then (just ('{') . to (Node :: new_token_part (TokenPartKind :: OpenBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (contract_body_element_parser . clone () . repeated () . map (| v | if v . is_empty () { Node :: new_none () } else { Node :: new_rule (RuleKind :: InterfaceDefinitionContractBodyElementRepeated , v) })) . then (leading_trivia_parser . clone () . then (just ('}') . to (Node :: new_token_part (TokenPartKind :: CloseBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: InterfaceDefinitionOpenBraceAndContractBodyElementRepeatedAndCloseBrace , vec ! [a , b , c]))) . map (| (((interface , identifier) , inheritance_specifier_list) , open_brace_and_contract_body_element_repeated_and_close_brace) | Node :: new_rule (RuleKind :: InterfaceDefinition , vec ! [interface , identifier , inheritance_specifier_list , open_brace_and_contract_body_element_repeated_and_close_brace])) . boxed () ;

        // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
        let library_definition_parser = leading_trivia_parser . clone () . then (terminal ("library") . to (Node :: new_token_part (TokenPartKind :: Library , 7usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (leading_trivia_parser . clone () . then (identifier_parser . clone ()) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . then (leading_trivia_parser . clone () . then (just ('{') . to (Node :: new_token_part (TokenPartKind :: OpenBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia) . then (contract_body_element_parser . clone () . repeated () . map (| v | if v . is_empty () { Node :: new_none () } else { Node :: new_rule (RuleKind :: LibraryDefinitionContractBodyElementRepeated , v) })) . then (leading_trivia_parser . clone () . then (just ('}') . to (Node :: new_token_part (TokenPartKind :: CloseBrace , 1usize))) . then (trailing_trivia_parser . clone ()) . map (Node :: new_with_trivia)) . map (| ((a , b) , c) | Node :: new_rule (RuleKind :: LibraryDefinitionOpenBraceAndContractBodyElementRepeatedAndCloseBrace , vec ! [a , b , c]))) . map (| ((library , identifier) , open_brace_and_contract_body_element_repeated_and_close_brace) | Node :: new_rule (RuleKind :: LibraryDefinition , vec ! [library , identifier , open_brace_and_contract_body_element_repeated_and_close_brace])) . boxed () ;

        // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
        let definition_parser = choice((
            contract_definition_parser.clone(),
            interface_definition_parser.clone(),
            library_definition_parser.clone(),
            function_definition_parser.clone(),
            constant_definition_parser.clone(),
            struct_definition_parser.clone(),
            enum_definition_parser.clone(),
            user_defined_value_type_definition_parser.clone(),
            error_definition_parser.clone(),
        ))
        .map(|v| Node::new_rule(RuleKind::Definition, vec![v]))
        .boxed();

        // SourceUnit = «LeadingTrivia» { Directive | Definition } «EndOfFileTrivia» ;
        let source_unit_parser = leading_trivia_parser
            .clone()
            .then(
                choice((directive_parser.clone(), definition_parser.clone()))
                    .repeated()
                    .map(Node::new_anonymous_rule),
            )
            .then(end_of_file_trivia_parser.clone())
            .map(
                |((leading_trivia, choices_0_repeated), end_of_file_trivia)| {
                    Node::new_rule(
                        RuleKind::SourceUnit,
                        vec![leading_trivia, choices_0_repeated, end_of_file_trivia],
                    )
                },
            )
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
