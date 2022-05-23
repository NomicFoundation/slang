use chumsky::{prelude::*, Parser};

use super::builder;
use crate::schema::Production;
pub type GrammarParserResultType = Vec<Production>;

pub fn create_grammar_parser() -> impl Parser<char, GrammarParserResultType, Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let comment_parser = just("/*")
        .ignore_then(
            choice::<_, Simple<char>>((
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
        .ignored()
        .boxed();
    let eof_parser = just('$').map(builder::eof).boxed();
    let hex_digit_parser = choice::<_, Simple<char>>((
        filter(|&c: &char| c.is_ascii_digit()),
        filter(|&c: &char| ('a' <= c && c <= 'f')),
        filter(|&c: &char| ('A' <= c && c <= 'F')),
    ))
    .boxed();
    let identifier_start_parser = choice::<_, Simple<char>>((
        just('_'),
        filter(|&c: &char| c.is_ascii_lowercase()),
        filter(|&c: &char| c.is_ascii_uppercase()),
    ))
    .boxed();
    let number_parser = filter(|&c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1usize)
        .map(builder::number)
        .boxed();
    let whitespace_parser =
        choice::<_, Simple<char>>((just('\t'), just('\n'), just('\r'), just(' ')))
            .ignored()
            .boxed();
    let ignore_parser =
        choice::<_, Simple<char>>((whitespace_parser.clone(), comment_parser.clone()))
            .repeated()
            .ignored()
            .boxed();
    let identifier_follow_parser = choice::<_, Simple<char>>((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ))
    .boxed();
    let string_char_parser = choice::<_, Simple<char>>((
        none_of("'\\"),
        just('\\').ignore_then(choice::<_, Simple<char>>((
            just('\''),
            just('\\'),
            just("u{")
                .ignore_then(
                    hex_digit_parser
                        .clone()
                        .repeated()
                        .at_least(1usize)
                        .at_most(6usize)
                        .map(builder::hex_digits_to_char)
                        .unwrapped(),
                )
                .then_ignore(just('}')),
        ))),
    ))
    .boxed();
    let raw_identifier_parser = identifier_start_parser
        .clone()
        .chain(identifier_follow_parser.clone().repeated())
        .map(builder::raw_identifier)
        .boxed();
    let single_char_string_parser = just('\'')
        .ignore_then(string_char_parser.clone())
        .then_ignore(just('\''))
        .boxed();
    let string_parser = just('\'')
        .ignore_then(string_char_parser.clone().repeated())
        .then_ignore(just('\''))
        .map(builder::string)
        .boxed();
    let grouped_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .boxed();
    let optional_parser = just('[')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(just(']').then_ignore(ignore_parser.clone()))
        .map(builder::optional)
        .boxed();
    let repetition_prefix_parser = choice::<_, Simple<char>>((
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
            .map(builder::repetition_prefix_1),
        just('…')
            .then_ignore(ignore_parser.clone())
            .ignore_then(number_parser.clone().then_ignore(ignore_parser.clone()))
            .map(builder::repetition_prefix_2),
    ))
    .then_ignore(just('*').then_ignore(ignore_parser.clone()))
    .boxed();
    let repetition_separator_parser = just('/')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .boxed();
    let identifier_parser = choice::<_, Simple<char>>((
        just('«')
            .ignore_then(raw_identifier_parser.clone())
            .then_ignore(just('»'))
            .map(builder::identifier_1),
        raw_identifier_parser.clone().map(builder::identifier_2),
    ))
    .boxed();
    let char_range_parser = single_char_string_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then_ignore(just('…').then_ignore(ignore_parser.clone()))
        .then(
            single_char_string_parser
                .clone()
                .then_ignore(ignore_parser.clone()),
        )
        .map(builder::char_range)
        .boxed();
    let repeated_parser = repetition_prefix_parser
        .clone()
        .or_not()
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then(repetition_separator_parser.clone().or_not())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::repeated)
        .boxed();
    let production_reference_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .map(builder::production_reference)
        .boxed();
    let primary_parser = choice::<_, Simple<char>>((
        production_reference_parser.clone(),
        grouped_parser.clone(),
        optional_parser.clone(),
        repeated_parser.clone(),
        char_range_parser.clone(),
        eof_parser.clone().then_ignore(ignore_parser.clone()),
        string_parser.clone().then_ignore(ignore_parser.clone()),
    ))
    .boxed();
    let negation_parser = just('¬')
        .then_ignore(ignore_parser.clone())
        .or_not()
        .then(primary_parser.clone())
        .map(builder::negation)
        .boxed();
    let difference_parser = negation_parser
        .clone()
        .then(
            just('-')
                .then_ignore(ignore_parser.clone())
                .ignore_then(negation_parser.clone())
                .or_not(),
        )
        .map(builder::difference)
        .boxed();
    let sequence_parser = difference_parser
        .clone()
        .repeated()
        .at_least(1usize)
        .map(builder::sequence)
        .boxed();
    expression_parser.define(
        sequence_parser
            .clone()
            .separated_by(just('|').then_ignore(ignore_parser.clone()))
            .at_least(1usize)
            .map(builder::expression)
            .boxed(),
    );
    let production_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then_ignore(just('=').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::production)
        .boxed();
    let grammar_parser = ignore_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .ignore_then(production_parser.clone().repeated())
        .then_ignore(end())
        .boxed();
    grammar_parser.recover_with(skip_then_retry_until([]))
}
