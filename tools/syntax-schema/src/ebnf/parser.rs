use chumsky::{prelude::*, Parser};

use super::builder::*;
use crate::schema::Production;
pub type GrammarParserResultType = Vec<Production>;

pub fn create_grammar_parser() -> impl Parser<char, GrammarParserResultType, Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let comment_parser = just("/*")
        .ignore_then(
            choice((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .repeated()
                    .at_least(1usize)
                    .then(none_of("*/"))
                    .ignored(),
            ))
            .repeated(),
        )
        .then(just('*').repeated().at_least(1usize))
        .then_ignore(just('/'))
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
    let number_parser = filter(|&c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1usize)
        .map(map_number);
    let whitespace_parser = choice((just('\t'), just('\n'), just('\r'), just(' '))).ignored();
    let ignore_parser = choice((whitespace_parser.clone(), comment_parser.clone()))
        .repeated()
        .ignored();
    let identifier_follow_parser = choice((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ));
    let string_char_parser = choice((
        none_of("'\\"),
        just('\\').ignore_then(choice((
            just('\''),
            just('\\'),
            just("u{")
                .ignore_then(
                    hex_digit_parser
                        .clone()
                        .repeated()
                        .at_least(1usize)
                        .at_most(6usize)
                        .map(map_hex_digits_to_char)
                        .unwrapped(),
                )
                .then_ignore(just('}')),
        ))),
    ));
    let raw_identifier_parser = identifier_start_parser
        .clone()
        .chain(identifier_follow_parser.clone().repeated())
        .map(map_raw_identifier);
    let single_char_string_parser = just('\'')
        .ignore_then(string_char_parser.clone())
        .then_ignore(just('\''));
    let string_parser = just('\'')
        .ignore_then(string_char_parser.clone().repeated())
        .then_ignore(just('\''))
        .map(map_string);
    let grouped_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()));
    let optional_parser = just('[')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(just(']').then_ignore(ignore_parser.clone()))
        .map(map_optional);
    let repetition_prefix_parser = choice((
        number_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .then(
                just('…')
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(
                        number_parser
                            .clone()
                            .then_ignore(ignore_parser.clone())
                            .or_not(),
                    )
                    .or_not(),
            )
            .map(map_repetition_prefix_1),
        just('…')
            .then_ignore(ignore_parser.clone())
            .ignore_then(number_parser.clone().then_ignore(ignore_parser.clone()))
            .map(map_repetition_prefix_2),
    ))
    .then_ignore(just('*').then_ignore(ignore_parser.clone()));
    let repetition_separator_parser = just('/')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone());
    let identifier_parser = choice((
        just('«')
            .ignore_then(raw_identifier_parser.clone())
            .then_ignore(just('»'))
            .map(map_identifier_1),
        raw_identifier_parser.clone().map(map_identifier_2),
    ));
    let char_range_parser = single_char_string_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then_ignore(just('…').then_ignore(ignore_parser.clone()))
        .then(
            single_char_string_parser
                .clone()
                .then_ignore(ignore_parser.clone()),
        )
        .map(map_char_range);
    let repeated_parser = repetition_prefix_parser
        .clone()
        .or_not()
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then(repetition_separator_parser.clone().or_not())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(map_repeated);
    let production_reference_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .map(map_production_reference);
    let primary_parser = choice((
        production_reference_parser.clone(),
        grouped_parser.clone(),
        optional_parser.clone(),
        repeated_parser.clone(),
        char_range_parser.clone(),
        eof_parser.clone().then_ignore(ignore_parser.clone()),
        string_parser.clone().then_ignore(ignore_parser.clone()),
    ));
    let negation_parser = just('¬')
        .then_ignore(ignore_parser.clone())
        .or_not()
        .then(primary_parser.clone())
        .map(map_negation);
    let difference_parser = negation_parser
        .clone()
        .then(
            just('-')
                .then_ignore(ignore_parser.clone())
                .ignore_then(negation_parser.clone())
                .or_not(),
        )
        .map(map_difference);
    let sequence_parser = difference_parser
        .clone()
        .repeated()
        .at_least(1usize)
        .map(map_sequence);
    expression_parser.define(
        sequence_parser
            .clone()
            .separated_by(just('|').then_ignore(ignore_parser.clone()))
            .at_least(1usize)
            .map(map_expression),
    );
    let production_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then_ignore(just('=').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(map_production);
    let grammar_parser = ignore_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .ignore_then(production_parser.clone().repeated())
        .then_ignore(end());
    grammar_parser.recover_with(skip_then_retry_until([]))
}
