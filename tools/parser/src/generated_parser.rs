use super::tree_builder::*;
use chumsky::prelude::*;
pub fn create_grammar_parser() -> impl Parser<char, GrammarParserResultType, Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let char_code_parser = just("#x")
        .ignore_then(
            filter(|&c: &char| c.is_ascii_hexdigit())
                .repeated()
                .at_least(1),
        )
        .map(map_char_code)
        .unwrapped();
    let comment_parser = just("/*")
        .ignore_then(
            choice((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .repeated()
                    .at_least(1)
                    .then(none_of("*/"))
                    .ignored(),
            ))
            .repeated(),
        )
        .then(just('*').repeated().at_least(1))
        .then_ignore(just('/'))
        .ignored();
    let identifier_parser = filter(|&c: &char| c == '_' || c.is_ascii_alphabetic())
        .chain(filter(|&c: &char| c == '_' || c.is_ascii_alphanumeric()).repeated())
        .map(map_identifier);
    let string_parser = choice((
        just('\'')
            .ignore_then(filter(|&c: &char| c != '\'').repeated())
            .then_ignore(just('\'')),
        just('"')
            .ignore_then(filter(|&c: &char| c != '"').repeated())
            .then_ignore(just('"')),
    ))
    .map(map_string);
    let whitespace_parser = choice((just('\t'), just('\n'), just('\r'), just(' ')))
        .repeated()
        .at_least(1)
        .ignored();
    let char_set_char_parser = choice((char_code_parser.clone(), none_of("\t\n\r#]")));
    let s_parser = choice((whitespace_parser.clone(), comment_parser.clone()))
        .repeated()
        .ignored();
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
        char_set_parser.clone(),
        just('$').map(map_to_eof),
        just('.').map(map_to_any),
        char_code_parser.clone().map(map_char_code_in_primary),
        string_parser.clone(),
        identifier_parser
            .clone()
            .then_ignore(
                s_parser
                    .clone()
                    .ignore_then(filter(|&c: &char| c != ':'))
                    .rewind(),
            )
            .map(map_identifier_in_primary),
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
        .then_ignore(just("::="))
        .then_ignore(s_parser.clone())
        .then(expression_parser.clone());
    let grammar_parser = s_parser
        .clone()
        .ignore_then(production_parser.clone())
        .repeated()
        .then_ignore(s_parser.clone())
        .map(map_grammar);
    grammar_parser.then_ignore(end().recover_with(skip_then_retry_until([])))
}
pub fn create_expression_parser(
) -> impl Parser<char, ExpressionParserResultType, Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let char_code_parser = just("#x")
        .ignore_then(
            filter(|&c: &char| c.is_ascii_hexdigit())
                .repeated()
                .at_least(1),
        )
        .map(map_char_code)
        .unwrapped();
    let comment_parser = just("/*")
        .ignore_then(
            choice((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .repeated()
                    .at_least(1)
                    .then(none_of("*/"))
                    .ignored(),
            ))
            .repeated(),
        )
        .then(just('*').repeated().at_least(1))
        .then_ignore(just('/'))
        .ignored();
    let identifier_parser = filter(|&c: &char| c == '_' || c.is_ascii_alphabetic())
        .chain(filter(|&c: &char| c == '_' || c.is_ascii_alphanumeric()).repeated())
        .map(map_identifier);
    let string_parser = choice((
        just('\'')
            .ignore_then(filter(|&c: &char| c != '\'').repeated())
            .then_ignore(just('\'')),
        just('"')
            .ignore_then(filter(|&c: &char| c != '"').repeated())
            .then_ignore(just('"')),
    ))
    .map(map_string);
    let whitespace_parser = choice((just('\t'), just('\n'), just('\r'), just(' ')))
        .repeated()
        .at_least(1)
        .ignored();
    let char_set_char_parser = choice((char_code_parser.clone(), none_of("\t\n\r#]")));
    let s_parser = choice((whitespace_parser.clone(), comment_parser.clone()))
        .repeated()
        .ignored();
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
        char_set_parser.clone(),
        just('$').map(map_to_eof),
        just('.').map(map_to_any),
        char_code_parser.clone().map(map_char_code_in_primary),
        string_parser.clone(),
        identifier_parser
            .clone()
            .then_ignore(
                s_parser
                    .clone()
                    .ignore_then(filter(|&c: &char| c != ':'))
                    .rewind(),
            )
            .map(map_identifier_in_primary),
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

