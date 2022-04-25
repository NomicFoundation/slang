use chumsky::prelude::*;

use super::ebnf_impl::*;

pub fn parser() -> impl Parser<char, Vec<Production>, Error = Simple<char>> {
    let whitespace = one_of("\x09\x0A\x0D\x20").ignored();

    let comment = just("/*").then(take_until(just("*/"))).ignored();

    let s = whitespace.or(comment).repeated().ignored();

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

    let expression = recursive(|expression: Recursive<char, Expression, Simple<char>>| {
        let primary = choice((
            char_set,
            just('$').to(Expression::End),
            just('.').to(Expression::Any),
            char_code.map(map_char_code_in_primary),
            string,
            identifier
                .then_ignore(s.then(none_of(':')).rewind())
                .map(map_identifier_in_primary),
            expression.padded_by(s).delimited_by(just('('), just(')')),
        ));

        let item = primary.then(one_of("?*+").or_not()).map(map_item);

        let difference = item
            .clone()
            .then(just('-').padded_by(s).ignore_then(item).or_not())
            .map(map_difference);

        let sequence = difference.separated_by(s).map(map_sequence);

        /* let choice = */
        sequence
            .separated_by(just('|').padded_by(s))
            .map(map_choice)
    });

    let production = identifier
        .then(just("::=").padded_by(s).ignore_then(expression))
        .map(map_production);

    let grammar = s.ignore_then(production).repeated().then_ignore(s);

    grammar.then_ignore(end().recover_with(skip_then_retry_until([])))
}
