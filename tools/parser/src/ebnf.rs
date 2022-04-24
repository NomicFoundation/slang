use chumsky::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Expression {
    Repeat(OccuranceCount, Box<Expression>),
    Alternate(Vec<Expression>),
    Sequence(Vec<Expression>),
    Exclusion(Box<Expression>, Box<Expression>),
    Chars(String),
    Identifier(String),
    Set(bool, Vec<SetElement>),
}

#[derive(Clone, Debug)]
pub enum OccuranceCount {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

#[derive(Clone, Debug)]
pub enum SetElement {
    Char(char),
    Range(char, char),
}

pub fn parser() -> impl Parser<char, HashMap<String, Expression>, Error = Simple<char>> {
    let whitespace = choice((just('\x09'), just('\x0A'), just('\x0D'), just('\x20'))).ignored();

    let comment = just("/*").then(take_until(just("*/"))).ignored();

    let s = whitespace.or(comment).repeated().ignored();

    let identifier = text::ident();

    let string = none_of("'")
        .repeated()
        .padded_by(just('\''))
        .or(none_of("\"").repeated().padded_by(just('"')))
        .collect::<String>()
        .map(Expression::Chars);

    let char_code = just("#x")
        .ignore_then(filter(char::is_ascii_hexdigit).repeated())
        .collect::<String>()
        .map(|digits| digits.parse::<u32>().unwrap())
        .map(|code| char::from_u32(code).ok_or(()))
        .unwrapped();

    let set_char = char_code.or(none_of("\x09\x0A\x0D#]")); /* TAB or LF or CR or '#' or ']' */

    let set = just('^')
        .or_not()
        .then(
            set_char
                .clone()
                .then(just('-').ignore_then(set_char).or_not())
                .repeated(),
        )
        .delimited_by(just('['), just(']'))
        .map(|(neg, chars)| {
            Expression::Set(
                neg.is_some(),
                chars
                    .into_iter()
                    .map(|(c1, c2)| {
                        if let Some(c2) = c2 {
                            SetElement::Range(c1, c2)
                        } else {
                            SetElement::Char(c1)
                        }
                    })
                    .collect(),
            )
        });

    let expression = recursive(|expression: Recursive<char, Expression, Simple<char>>| {
        let term = set
            .or(string)
            .or(identifier.map(Expression::Identifier))
            .or(expression.padded_by(s).delimited_by(just('('), just(')')));

        let repeat = term.then(one_of("*+?").or_not()).map(|(h, t)| match t {
            Some('?') => Expression::Repeat(OccuranceCount::ZeroOrOne, Box::new(h)),
            Some('*') => Expression::Repeat(OccuranceCount::ZeroOrMore, Box::new(h)),
            Some('+') => Expression::Repeat(OccuranceCount::OneOrMore, Box::new(h)),
            _ => h,
        });

        let exclusion = repeat
            .clone()
            .then(just('-').padded_by(s).ignore_then(repeat).or_not())
            .map(|(r1, r2)| {
                if let Some(r2) = r2 {
                    Expression::Exclusion(Box::new(r1), Box::new(r2))
                } else {
                    r1
                }
            });

        let alternate = exclusion
            .separated_by(just('|').padded_by(s))
            .map(Expression::Alternate);

        alternate.separated_by(s).map(Expression::Sequence)
    });

    let rule = identifier.then(just("::=").padded_by(s).ignore_then(expression));

    let grammar = rule.separated_by(s).padded_by(s).map(|rules| {
        let mut map = HashMap::new();
        for (i, r) in rules {
            map.insert(i, r);
        }
        map
    });

    grammar.then_ignore(end().recover_with(skip_then_retry_until([])))
}
