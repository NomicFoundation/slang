use std::rc::Rc;

use chumsky::prelude::*;

use super::tree_builder::*;

pub fn create_grammar_parser() -> impl Parser<char, GrammarParserResultType, Error = Simple<char>> {
    let whitespace = one_of("\x09\x0A\x0D\x20").ignored();

    let comment = just("/*").then(take_until(just("*/"))).ignored();

    let s = choice((whitespace, comment)).repeated().ignored();

    let identifier = text::ident();

    let string = choice((
        none_of("'").repeated().padded_by(just('\'')),
        none_of("\"").repeated().padded_by(just('"')),
    ))
    .map(map_string);

    let char_code = just("#x")
        .ignore_then(filter(char::is_ascii_hexdigit).repeated())
        .map(map_char_code)
        .unwrapped();

    let char_set_char = choice((
        char_code,
        none_of("\x09\x0A\x0D\x23\x5D"), /* TAB or LF or CR or '#' or ']' */
    ));

    let char_set = just('^')
        .or_not()
        .then(
            char_set_char
                .then(just('-').ignore_then(char_set_char).or_not())
                .repeated(),
        )
        .delimited_by(just('['), just(']'))
        .map(map_char_set);

    let mut expression_parser = Recursive::declare();

    let primary = choice((
        char_set,
        just('$').map(|_| Rc::new(Expression::End {})),
        just('.').map(|_| Rc::new(Expression::Any {})),
        char_code.map(map_char_code_in_primary),
        string,
        identifier
            .then_ignore(s.then(none_of(':')).rewind())
            .map(map_identifier_in_primary),
        expression_parser
            .clone()
            .padded_by(s)
            .delimited_by(just('('), just(')')),
    ));

    let item = primary.then(one_of("?*+").or_not()).map(map_item);

    let difference = item
        .clone()
        .then(just('-').padded_by(s).ignore_then(item).or_not())
        .map(map_difference);

    let sequence = difference.separated_by(s).map(map_sequence);

    expression_parser.define(
        sequence
            .separated_by(just('|').padded_by(s))
            .map(map_choice),
    );

    let production = identifier.then(just("::=").padded_by(s).ignore_then(expression_parser));

    let grammar = s
        .ignore_then(production)
        .repeated()
        .then_ignore(s)
        .map(map_grammar);

    grammar.then_ignore(end().recover_with(skip_then_retry_until([])))
}

pub fn create_expression_parser(
) -> impl Parser<char, ExpressionParserResultType, Error = Simple<char>> {
    let whitespace = one_of("\x09\x0A\x0D\x20").ignored();

    let comment = just("/*").then(take_until(just("*/"))).ignored();

    let s = choice((whitespace, comment)).repeated().ignored();

    let identifier = text::ident();

    let string = choice((
        none_of("'").repeated().padded_by(just('\'')),
        none_of("\"").repeated().padded_by(just('"')),
    ))
    .map(map_string);

    let char_code = just("#x")
        .ignore_then(filter(char::is_ascii_hexdigit).repeated())
        .map(map_char_code)
        .unwrapped();

    let char_set_char = choice((
        char_code,
        none_of("\x09\x0A\x0D\x23\x5D"), /* TAB or LF or CR or '#' or ']' */
    ));

    let char_set = just('^')
        .or_not()
        .then(
            char_set_char
                .then(just('-').ignore_then(char_set_char).or_not())
                .repeated(),
        )
        .delimited_by(just('['), just(']'))
        .map(map_char_set);

    let mut expression_parser = Recursive::declare();

    let primary = choice((
        char_set,
        just('$').map(|_| Rc::new(Expression::End {})),
        just('.').map(|_| Rc::new(Expression::Any {})),
        char_code.map(map_char_code_in_primary),
        string,
        identifier
            .then_ignore(s.then(none_of(':')).rewind())
            .map(map_identifier_in_primary),
        expression_parser
            .clone()
            .padded_by(s)
            .delimited_by(just('('), just(')')),
    ));

    let item = primary.then(one_of("?*+").or_not()).map(map_item);

    let difference = item
        .clone()
        .then(just('-').padded_by(s).ignore_then(item).or_not())
        .map(map_difference);

    let sequence = difference.separated_by(s).map(map_sequence);

    expression_parser.define(
        sequence
            .separated_by(just('|').padded_by(s))
            .map(map_choice),
    );

    expression_parser.then_ignore(end().recover_with(skip_then_retry_until([])))
}
