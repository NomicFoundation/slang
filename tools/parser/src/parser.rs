use super::tree_builder::*;
use chumsky::prelude::*;
pub fn create_expression_parser(
) -> impl Parser<char, ExpressionParserResultType, Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let any_parser = just('.').map(map_any);
    let comment_parser = just("/*")
        .ignored()
        .ignore_then(
            choice((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .ignored()
                    .ignore_then(just('*').repeated().ignored())
                    .then(none_of("*/").ignored())
                    .ignored(),
            ))
            .repeated()
            .ignored(),
        )
        .then_ignore(just('*').ignored())
        .then(just('*').repeated().ignored())
        .then_ignore(just('/').ignored())
        .ignored();
    let eof_parser = just('$').map(map_eof);
    let hex_digit_parser = choice((
        filter(|&c: &char| c.is_ascii_digit()),
        filter(|&c: &char| ('a' <= c && c <= 'f')),
        filter(|&c: &char| ('A' <= c && c <= 'F')),
    ));
    let identifier_start_parser = choice((
        just('_'),
        filter(|&c: &char| c.is_ascii_lowercase()),
        filter(|&c: &char| c.is_ascii_uppercase()),
    ));
    let whitespace_parser = choice((
        just('\t').ignored(),
        just('\n').ignored(),
        just('\r').ignored(),
        just(' ').ignored(),
    ))
    .ignored();
    let identifier_follow_parser = choice((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ));
    let s_parser = choice((
        whitespace_parser.clone().ignored(),
        comment_parser.clone().ignored(),
    ))
    .repeated()
    .ignored();
    let string_char_parser = choice((
        none_of("'\\"),
        just('\\').ignore_then(choice((
            just('\''),
            just('\\'),
            just("u{")
                .ignore_then(hex_digit_parser.clone())
                .chain(hex_digit_parser.clone().repeated())
                .then_ignore(just('}'))
                .map(map_hex_digits_to_char)
                .unwrapped(),
        ))),
    ));
    let identifier_parser = identifier_start_parser
        .clone()
        .chain(identifier_follow_parser.clone().repeated())
        .map(map_identifier);
    let single_char_string_parser = just('\'')
        .ignore_then(string_char_parser.clone())
        .then_ignore(just('\''));
    let string_parser = just('\'')
        .ignore_then(string_char_parser.clone().repeated())
        .then_ignore(just('\''))
        .map(map_string);
    let grouped_parser = just('(')
        .ignore_then(s_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(s_parser.clone())
        .then_ignore(just(')'));
    let optional_parser = just('[')
        .ignore_then(s_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(s_parser.clone())
        .then_ignore(just(']'))
        .map(map_optional);
    let repeated_parser = just('{')
        .ignore_then(s_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(s_parser.clone())
        .then_ignore(just('}'))
        .map(map_repeated);
    let char_range_parser = single_char_string_parser
        .clone()
        .then_ignore(just("…"))
        .then(single_char_string_parser.clone())
        .map(map_char_range);
    let production_reference_parser = identifier_parser.clone().map(map_production_reference);
    let primary_parser = choice((
        eof_parser.clone(),
        any_parser.clone(),
        char_range_parser.clone(),
        string_parser.clone(),
        production_reference_parser.clone(),
        grouped_parser.clone(),
        optional_parser.clone(),
        repeated_parser.clone(),
    ));
    let negation_parser = just("¬")
        .ignore_then(s_parser.clone())
        .or_not()
        .then(primary_parser.clone())
        .map(map_negation);
    let difference_parser = negation_parser
        .clone()
        .then(
            s_parser
                .clone()
                .ignore_then(just('-'))
                .ignore_then(s_parser.clone())
                .ignore_then(negation_parser.clone())
                .or_not(),
        )
        .map(map_difference);
    let sequence_parser = difference_parser
        .clone()
        .chain(
            s_parser
                .clone()
                .ignore_then(difference_parser.clone())
                .repeated(),
        )
        .map(map_sequence);
    expression_parser.define(
        sequence_parser
            .clone()
            .chain(
                s_parser
                    .clone()
                    .ignore_then(just('|'))
                    .ignore_then(s_parser.clone())
                    .ignore_then(sequence_parser.clone())
                    .repeated(),
            )
            .map(map_expression),
    );
    expression_parser.recover_with(skip_then_retry_until([]))
}
pub fn create_grammar_parser() -> impl Parser<char, GrammarParserResultType, Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let any_parser = just('.').map(map_any);
    let comment_parser = just("/*")
        .ignored()
        .ignore_then(
            choice((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .ignored()
                    .ignore_then(just('*').repeated().ignored())
                    .then(none_of("*/").ignored())
                    .ignored(),
            ))
            .repeated()
            .ignored(),
        )
        .then_ignore(just('*').ignored())
        .then(just('*').repeated().ignored())
        .then_ignore(just('/').ignored())
        .ignored();
    let eof_parser = just('$').map(map_eof);
    let hex_digit_parser = choice((
        filter(|&c: &char| c.is_ascii_digit()),
        filter(|&c: &char| ('a' <= c && c <= 'f')),
        filter(|&c: &char| ('A' <= c && c <= 'F')),
    ));
    let identifier_start_parser = choice((
        just('_'),
        filter(|&c: &char| c.is_ascii_lowercase()),
        filter(|&c: &char| c.is_ascii_uppercase()),
    ));
    let whitespace_parser = choice((
        just('\t').ignored(),
        just('\n').ignored(),
        just('\r').ignored(),
        just(' ').ignored(),
    ))
    .ignored();
    let identifier_follow_parser = choice((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ));
    let s_parser = choice((
        whitespace_parser.clone().ignored(),
        comment_parser.clone().ignored(),
    ))
    .repeated()
    .ignored();
    let string_char_parser = choice((
        none_of("'\\"),
        just('\\').ignore_then(choice((
            just('\''),
            just('\\'),
            just("u{")
                .ignore_then(hex_digit_parser.clone())
                .chain(hex_digit_parser.clone().repeated())
                .then_ignore(just('}'))
                .map(map_hex_digits_to_char)
                .unwrapped(),
        ))),
    ));
    let identifier_parser = identifier_start_parser
        .clone()
        .chain(identifier_follow_parser.clone().repeated())
        .map(map_identifier);
    let single_char_string_parser = just('\'')
        .ignore_then(string_char_parser.clone())
        .then_ignore(just('\''));
    let string_parser = just('\'')
        .ignore_then(string_char_parser.clone().repeated())
        .then_ignore(just('\''))
        .map(map_string);
    let grouped_parser = just('(')
        .ignore_then(s_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(s_parser.clone())
        .then_ignore(just(')'));
    let optional_parser = just('[')
        .ignore_then(s_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(s_parser.clone())
        .then_ignore(just(']'))
        .map(map_optional);
    let repeated_parser = just('{')
        .ignore_then(s_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(s_parser.clone())
        .then_ignore(just('}'))
        .map(map_repeated);
    let char_range_parser = single_char_string_parser
        .clone()
        .then_ignore(just("…"))
        .then(single_char_string_parser.clone())
        .map(map_char_range);
    let production_reference_parser = identifier_parser.clone().map(map_production_reference);
    let primary_parser = choice((
        eof_parser.clone(),
        any_parser.clone(),
        char_range_parser.clone(),
        string_parser.clone(),
        production_reference_parser.clone(),
        grouped_parser.clone(),
        optional_parser.clone(),
        repeated_parser.clone(),
    ));
    let negation_parser = just("¬")
        .ignore_then(s_parser.clone())
        .or_not()
        .then(primary_parser.clone())
        .map(map_negation);
    let difference_parser = negation_parser
        .clone()
        .then(
            s_parser
                .clone()
                .ignore_then(just('-'))
                .ignore_then(s_parser.clone())
                .ignore_then(negation_parser.clone())
                .or_not(),
        )
        .map(map_difference);
    let sequence_parser = difference_parser
        .clone()
        .chain(
            s_parser
                .clone()
                .ignore_then(difference_parser.clone())
                .repeated(),
        )
        .map(map_sequence);
    expression_parser.define(
        sequence_parser
            .clone()
            .chain(
                s_parser
                    .clone()
                    .ignore_then(just('|'))
                    .ignore_then(s_parser.clone())
                    .ignore_then(sequence_parser.clone())
                    .repeated(),
            )
            .map(map_expression),
    );
    let production_parser = identifier_parser
        .clone()
        .then_ignore(s_parser.clone())
        .then_ignore(just('='))
        .then_ignore(s_parser.clone())
        .then(expression_parser.clone())
        .then_ignore(s_parser.clone())
        .then_ignore(just(';'));
    let grammar_parser = s_parser
        .clone()
        .ignore_then(production_parser.clone())
        .repeated()
        .then_ignore(s_parser.clone())
        .then_ignore(end())
        .map(map_grammar);
    grammar_parser.recover_with(skip_then_retry_until([]))
}

