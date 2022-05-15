pub fn create_grammar_parser() -> impl Parser<char, GrammarParserResultType, Error = Simple<char>> {
    let mut expression_parser = Recursive::declare();
    let any_parser = just('.').map(map_any);
    let comment_parser = just("/*")
        .ignore_then(
            choice((
                filter(|&c: &char| c != '*'),
                just('*').repeated().at_least(1).then(none_of("*/")),
            ))
            .repeated(),
        )
        .then(just('*').repeated().at_least(1))
        .then_ignore(just('/'))
        .map(map_comment);
    let eof_parser = just('$').map(map_eof);
    let hex_digit_parser = choice((
        filter(|&c: &char| c.is_ascii_digit()),
        filter(|&c: &char| ('a' <= c && c <= 'f')),
        filter(|&c: &char| ('A' <= c && c <= 'F')),
    ))
    .map(map_hex_digit);
    let identifier_start_parser = choice((
        just('_'),
        filter(|&c: &char| c.is_ascii_lowercase()),
        filter(|&c: &char| c.is_ascii_uppercase()),
    ))
    .map(map_identifier_start);
    let whitespace_parser =
        choice((just('\t'), just('\n'), just('\r'), just(' '))).map(map_whitespace);
    let identifier_follow_parser = choice((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ))
    .map(map_identifier_follow);
    let s_parser = choice((whitespace_parser.clone(), comment_parser.clone()))
        .repeated()
        .ignored();
    let string_char_parser = choice((
        none_of("'\\"),
        just('\\').ignore_then(choice((
            just('\''),
            just('\\'),
            just("u{")
                .ignore_then(hex_digit_parser.clone().repeated().at_least(1))
                .then_ignore(just('}')),
        ))),
    ))
    .map(map_string_char);
    let identifier_parser = identifier_start_parser
        .clone()
        .then(identifier_follow_parser.clone().repeated())
        .map(map_identifier);
    let single_char_string_parser = just('\'')
        .ignore_then(string_char_parser.clone())
        .then_ignore(just('\''))
        .map(map_single_char_string);
    let string_parser = just('\'')
        .ignore_then(string_char_parser.clone().repeated())
        .then_ignore(just('\''))
        .map(map_string);
    let grouped_parser = just('(')
        .ignore_then(s_parser.clone())
        .then(expression_parser.clone())
        .then(s_parser.clone())
        .then_ignore(just(')'))
        .map(map_grouped);
    let optional_parser = just('[')
        .ignore_then(s_parser.clone())
        .then(expression_parser.clone())
        .then(s_parser.clone())
        .then_ignore(just(']'))
        .map(map_optional);
    let repeated_parser = just('{')
        .ignore_then(s_parser.clone())
        .then(expression_parser.clone())
        .then(s_parser.clone())
        .then_ignore(just('}'))
        .map(map_repeated);
    let char_range_parser = single_char_string_parser
        .clone()
        .then(s_parser.clone())
        .then_ignore(just("…"))
        .then(s_parser.clone())
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
    ))
    .map(map_primary);
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
                .then_ignore(just('-'))
                .then(s_parser.clone())
                .then(negation_parser.clone())
                .or_not(),
        )
        .map(map_difference);
    let sequence_parser = difference_parser
        .clone()
        .then(s_parser.clone().then(difference_parser.clone()).repeated())
        .map(map_sequence);
    expression_parser.define(
        sequence_parser
            .clone()
            .then(
                s_parser
                    .clone()
                    .then_ignore(just('|'))
                    .then(s_parser.clone())
                    .then(sequence_parser.clone())
                    .repeated(),
            )
            .map(map_expression),
    );
    let production_parser = identifier_parser
        .clone()
        .then(s_parser.clone())
        .then_ignore(just('='))
        .then(s_parser.clone())
        .then(expression_parser.clone())
        .then(s_parser.clone())
        .then_ignore(just(';'))
        .map(map_production);
    let grammar_parser = s_parser
        .clone()
        .then(production_parser.clone())
        .repeated()
        .then(s_parser.clone())
        .then_ignore(end())
        .map(map_grammar);
    grammar_parser.recover_with(skip_then_retry_until([]))
}
