use super::tree_builder::*;
use chumsky::prelude::*;
pub fn create_expression_parser(
) -> impl Parser<char, ExpressionParserResultType, Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let comment_parser = just("/*")
        .ignored()
        .ignore_then(
            choice((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .repeated()
                    .at_least(1)
                    .ignored()
                    .then(none_of("*/").ignored())
                    .ignored(),
            ))
            .repeated()
            .ignored(),
        )
        .then(just('*').repeated().at_least(1).ignored())
        .then_ignore(just('/').ignored())
        .ignored();
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
    .repeated()
    .at_least(1)
    .ignored();
    let char_code_parser = just("#x").ignore_then(
        hex_digit_parser
            .clone()
            .repeated()
            .at_least(1)
            .map(map_hex_digits_to_char)
            .unwrapped(),
    );
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
            one_of("'\\"),
            just("u{")
                .ignore_then(hex_digit_parser.clone().repeated().at_least(1))
                .then_ignore(just('}'))
                .map(map_hex_digits_to_char)
                .unwrapped(),
        ))),
    ));
    let char_set_char_parser = choice((char_code_parser.clone(), none_of("\t\n\r#]")));
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
    let char_range_parser = single_char_string_parser
        .clone()
        .then_ignore(just("…"))
        .then(single_char_string_parser.clone())
        .map(map_char_range);
    let char_set_parser = just('[')
        .ignore_then(just('^').or_not())
        .then(
            char_set_char_parser
                .clone()
                .then(just('-').ignore_then(char_set_char_parser.clone()).or_not())
                .repeated(),
        )
        .then_ignore(just(']'))
        .map(map_char_set);
    let primary_parser = choice((
        char_range_parser.clone(),
        char_set_parser.clone(),
        just('$').map(map_to_eof),
        just('.').map(map_to_any),
        string_parser.clone(),
        identifier_parser.clone().map(map_identifier_in_primary),
        just('(')
            .ignore_then(s_parser.clone())
            .ignore_then(expression_parser.clone())
            .then_ignore(s_parser.clone())
            .then_ignore(just(')')),
    ));
    let item_parser = primary_parser
        .clone()
        .then(choice((just('?'), just('*'), just('+'))).or_not())
        .map(map_item);
    let difference_parser = item_parser
        .clone()
        .then(
            s_parser
                .clone()
                .ignore_then(just('-'))
                .ignore_then(s_parser.clone())
                .ignore_then(item_parser.clone())
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
    expression_parser.then_ignore(end().recover_with(skip_then_retry_until([])))
}
pub fn create_grammar_parser() -> impl Parser<char, GrammarParserResultType, Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let comment_parser = just("/*")
        .ignored()
        .ignore_then(
            choice((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .repeated()
                    .at_least(1)
                    .ignored()
                    .then(none_of("*/").ignored())
                    .ignored(),
            ))
            .repeated()
            .ignored(),
        )
        .then(just('*').repeated().at_least(1).ignored())
        .then_ignore(just('/').ignored())
        .ignored();
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
    .repeated()
    .at_least(1)
    .ignored();
    let char_code_parser = just("#x").ignore_then(
        hex_digit_parser
            .clone()
            .repeated()
            .at_least(1)
            .map(map_hex_digits_to_char)
            .unwrapped(),
    );
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
            one_of("'\\"),
            just("u{")
                .ignore_then(hex_digit_parser.clone().repeated().at_least(1))
                .then_ignore(just('}'))
                .map(map_hex_digits_to_char)
                .unwrapped(),
        ))),
    ));
    let char_set_char_parser = choice((char_code_parser.clone(), none_of("\t\n\r#]")));
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
    let char_range_parser = single_char_string_parser
        .clone()
        .then_ignore(just("…"))
        .then(single_char_string_parser.clone())
        .map(map_char_range);
    let char_set_parser = just('[')
        .ignore_then(just('^').or_not())
        .then(
            char_set_char_parser
                .clone()
                .then(just('-').ignore_then(char_set_char_parser.clone()).or_not())
                .repeated(),
        )
        .then_ignore(just(']'))
        .map(map_char_set);
    let primary_parser = choice((
        char_range_parser.clone(),
        char_set_parser.clone(),
        just('$').map(map_to_eof),
        just('.').map(map_to_any),
        string_parser.clone(),
        identifier_parser.clone().map(map_identifier_in_primary),
        just('(')
            .ignore_then(s_parser.clone())
            .ignore_then(expression_parser.clone())
            .then_ignore(s_parser.clone())
            .then_ignore(just(')')),
    ));
    let item_parser = primary_parser
        .clone()
        .then(choice((just('?'), just('*'), just('+'))).or_not())
        .map(map_item);
    let difference_parser = item_parser
        .clone()
        .then(
            s_parser
                .clone()
                .ignore_then(just('-'))
                .ignore_then(s_parser.clone())
                .ignore_then(item_parser.clone())
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
    grammar_parser.then_ignore(end().recover_with(skip_then_retry_until([])))
}
