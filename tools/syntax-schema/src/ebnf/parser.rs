#[allow(unused_imports)]
use super::builder;
#[allow(unused_imports)]
use crate::chumsky::combinators::NomicParser;
use chumsky::{prelude::*, Parser};

pub type ErrorType = Simple<char>;
use crate::schema::Production;
pub type GrammarParserResultType = Vec<Production>;

pub fn create_grammar_parser() -> impl Parser<char, GrammarParserResultType, Error = ErrorType> {
    let mut expression_parser = Recursive::declare();
    let comment_parser = just("/*")
        .ignore_then(
            choice::<_, ErrorType>((
                filter(|&c: &char| c != '*').ignored().ignored(),
                just('*')
                    .repeated()
                    .at_least(1usize)
                    .then(filter(|&c: &char| (c != '*' && c != '/')))
                    .ignored()
                    .ignored(),
            ))
            .repeated(),
        )
        .then(just('*').repeated().at_least(1usize))
        .then_ignore(just('/'))
        .ignored();
    let eof_parser = just('$').map(builder::eof);
    let hex_digit_parser =
        filter(|&c: &char| c.is_ascii_digit() || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F'));
    let identifier_start_parser =
        filter(|&c: &char| c == '_' || c.is_ascii_lowercase() || c.is_ascii_uppercase());
    let number_parser = filter(|&c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1usize)
        .map(builder::number);
    let whitespace_parser =
        filter(|&c: &char| c == '\t' || c == '\n' || c == '\r' || c == ' ').ignored();
    let ignore_parser = choice::<_, ErrorType>((
        whitespace_parser.clone().ignored(),
        comment_parser.clone().ignored(),
    ))
    .repeated()
    .ignored();
    let identifier_follow_parser = choice::<_, ErrorType>((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ));
    let string_char_parser = choice::<_, ErrorType>((
        filter(|&c: &char| (c != '\'' && c != '\\')),
        just('\\').ignore_then(choice::<_, ErrorType>((
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
    ));
    let raw_identifier_parser = identifier_start_parser
        .clone()
        .chain(identifier_follow_parser.clone().repeated())
        .map(builder::raw_identifier);
    let single_char_string_parser = just('\'')
        .ignore_then(string_char_parser.clone())
        .then_ignore(just('\''));
    let string_parser = just('\'')
        .ignore_then(string_char_parser.clone().repeated())
        .then_ignore(just('\''))
        .map(builder::string);
    let grouped_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()));
    let optional_parser = just('[')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(just(']').then_ignore(ignore_parser.clone()))
        .map(builder::optional);
    let repetition_prefix_parser = choice::<_, ErrorType>((
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
    .then_ignore(just('*').then_ignore(ignore_parser.clone()));
    let repetition_separator_parser = just('/')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone());
    let identifier_parser = choice::<_, ErrorType>((
        just('«')
            .ignore_then(raw_identifier_parser.clone())
            .then_ignore(just('»'))
            .map(builder::identifier_1),
        raw_identifier_parser.clone().map(builder::identifier_2),
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
        .map(builder::char_range);
    let repeated_parser = repetition_prefix_parser
        .clone()
        .or_not()
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then(repetition_separator_parser.clone().or_not())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::repeated);
    let production_reference_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .map(builder::production_reference);
    let primary_parser = choice::<_, ErrorType>((
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
        .map(builder::negation);
    let difference_parser = negation_parser
        .clone()
        .then(
            just('-')
                .then_ignore(ignore_parser.clone())
                .ignore_then(negation_parser.clone())
                .or_not(),
        )
        .map(builder::difference);
    let sequence_parser = difference_parser
        .clone()
        .repeated()
        .at_least(1usize)
        .map(builder::sequence);
    expression_parser.define(
        sequence_parser
            .clone()
            .separated_by(just('|').then_ignore(ignore_parser.clone()))
            .at_least(1usize)
            .map(builder::expression),
    );
    let production_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then_ignore(just('=').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::production);
    let grammar_parser = ignore_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .ignore_then(production_parser.clone().repeated())
        .then_ignore(end());
    grammar_parser.recover_with(skip_then_retry_until([]))
}
