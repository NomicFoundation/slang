use chumsky::prelude::*;
pub fn create_expression_parser() -> impl Parser<char, (), Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let char_code_parser = just("#x")
        .then(
            filter(|&c: &char| c.is_ascii_hexdigit())
                .repeated()
                .at_least(1),
        )
        .ignored();
    let comment_parser = just("/*")
        .then(
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
        .then(just('/'))
        .ignored();
    let identifier_parser = filter(|&c: &char| c == '_' || c.is_ascii_alphabetic())
        .then(filter(|&c: &char| c == '_' || c.is_ascii_alphanumeric()).repeated())
        .ignored();
    let string_parser = choice((
        just('\'')
            .then(filter(|&c: &char| c != '\'').repeated())
            .then(just('\'')),
        just('"')
            .then(filter(|&c: &char| c != '"').repeated())
            .then(just('"')),
    ))
    .ignored();
    let whitespace_parser = choice((just('\t'), just('\n'), just('\r'), just(' ')))
        .repeated()
        .ignored();
    let char_set_char_parser =
        choice((char_code_parser.clone(), none_of("\t\n\r#]").ignored())).ignored();
    let s_parser = choice((whitespace_parser.clone(), comment_parser.clone()))
        .repeated()
        .ignored();
    let char_set_parser = just('[')
        .then(just('^').or_not())
        .then(
            char_set_char_parser
                .clone()
                .then(just('-').then(char_set_char_parser.clone()).or_not())
                .repeated(),
        )
        .then(just(']'))
        .ignored();
    let primary_parser = choice((
        char_set_parser.clone(),
        just('$').ignored(),
        just('.').ignored(),
        char_code_parser.clone().ignored(),
        string_parser.clone(),
        identifier_parser
            .clone()
            .then_ignore(s_parser.clone().then(filter(|&c: &char| c != ':')).rewind())
            .ignored(),
        just('(')
            .then(s_parser.clone())
            .then(expression_parser.clone())
            .then(s_parser.clone())
            .then(just(')'))
            .ignored(),
    ))
    .ignored();
    let item_parser = primary_parser
        .clone()
        .then(choice((just('?'), just('*'), just('+'))).or_not())
        .ignored();
    let difference_parser = item_parser
        .clone()
        .then(
            s_parser
                .clone()
                .then(just('-'))
                .then(s_parser.clone())
                .then(item_parser.clone())
                .or_not(),
        )
        .ignored();
    let sequence_parser = difference_parser
        .clone()
        .then(s_parser.clone().then(difference_parser.clone()).repeated())
        .ignored();
    expression_parser.define(
        sequence_parser
            .clone()
            .then(
                s_parser
                    .clone()
                    .then(just('|'))
                    .then(s_parser.clone())
                    .then(sequence_parser.clone())
                    .repeated(),
            )
            .ignored(),
    );
    expression_parser.then_ignore(end().recover_with(skip_then_retry_until([])))
}

