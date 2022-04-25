use chumsky::prelude::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Production {
    name: String,
    expr: Expression,
}

#[derive(Clone, Debug)]
pub enum Expression {
    End,
    Any,
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

pub fn parser() -> impl Parser<char, Vec<Production>, Error = Simple<char>> {
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
        .map(|digits| u32::from_str_radix(digits.as_str(), 16).unwrap())
        .map(|code| char::from_u32(code).ok_or(()))
        .unwrapped();

    let set_char = char_code.or(none_of("\x09\x0A\x0D\x23\x5D")); /* TAB or LF or CR or '#' or ']' */

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
        let primary = set
            .or(just('$').to(Expression::End))
            .or(just('.').to(Expression::Any))
            .or(char_code.map(|c: char| Expression::Chars(c.to_string())))
            .or(string)
            .or(identifier
                .then_ignore(s.then(none_of(':')).rewind())
                .map(Expression::Identifier))
            .or(expression.padded_by(s).delimited_by(just('('), just(')')));

        let item = primary.then(one_of("+*?").or_not()).map(|(h, t)| match t {
            Some('?') => Expression::Repeat(OccuranceCount::ZeroOrOne, Box::new(h)),
            Some('*') => Expression::Repeat(OccuranceCount::ZeroOrMore, Box::new(h)),
            Some('+') => Expression::Repeat(OccuranceCount::OneOrMore, Box::new(h)),
            _ => h,
        });

        let difference = item
            .clone()
            .then(just('-').padded_by(s).ignore_then(item).or_not())
            .map(|(item1, item2)| {
                if let Some(item2) = item2 {
                    Expression::Exclusion(Box::new(item1), Box::new(item2))
                } else {
                    item1
                }
            });

        let sequence = difference.separated_by(s).map(|mut diffs| {
            if diffs.len() == 1 {
                diffs.pop().unwrap()
            } else {
                Expression::Sequence(diffs)
            }
        });

        /*let choice = */
        sequence
            .separated_by(just('|').padded_by(s))
            .map(|mut seqs| {
                if seqs.len() == 1 {
                    seqs.pop().unwrap()
                } else {
                    Expression::Alternate(seqs)
                }
            })
    });

    let production = identifier
        .then(just("::=").padded_by(s).ignore_then(expression))
        .map(|(name, expr)| Production { name, expr });

    let grammar = s.ignore_then(production).repeated().then_ignore(s);

    grammar.then_ignore(end().recover_with(skip_then_retry_until([])))
}
