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
    Repeat(RepeatCount, Box<Expression>),
    Choice(Vec<Expression>),
    Sequence(Vec<Expression>),
    Difference(Box<Expression>, Box<Expression>),
    Chars(String),
    Identifier(String),
    CharSet(bool, Vec<CharSetElement>),
}

#[derive(Clone, Debug)]
pub enum RepeatCount {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

#[derive(Clone, Debug)]
pub enum CharSetElement {
    Char(char),
    Range(char, char),
}

pub fn parser() -> impl Parser<char, Vec<Production>, Error = Simple<char>> {
    let whitespace = one_of("\x09\x0A\x0D\x20").ignored();

    let comment = just("/*").then(take_until(just("*/"))).ignored();

    let s = whitespace.or(comment).repeated().ignored();

    let identifier = text::ident();

    let string = none_of("'")
        .repeated()
        .padded_by(just('\''))
        .or(none_of("\"").repeated().padded_by(just('"')))
        .map(map_string);

    let char_code = just("#x")
        .ignore_then(filter(char::is_ascii_hexdigit).repeated())
        .map(map_char_code)
        .unwrapped();

    let char_set_char = char_code.or(none_of("\x09\x0A\x0D\x23\x5D")); /* TAB or LF or CR or '#' or ']' */

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
                .map(Expression::Identifier),
            expression.padded_by(s).delimited_by(just('('), just(')')),
        ));

        let item = primary.then(one_of("?*+").or_not()).map(map_primary);

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

fn map_char_set((neg, chars): (Option<char>, Vec<(char, Option<char>)>)) -> Expression {
    Expression::CharSet(
        neg.is_some(),
        chars
            .into_iter()
            .map(|(c1, c2)| {
                if let Some(c2) = c2 {
                    CharSetElement::Range(c1, c2)
                } else {
                    CharSetElement::Char(c1)
                }
            })
            .collect(),
    )
}

fn map_string(chars: Vec<char>) -> Expression {
    Expression::Chars(chars.iter().collect::<String>())
}

fn map_production((name, expr): (String, Expression)) -> Production {
    Production { name, expr }
}

fn map_sequence(mut diffs: Vec<Expression>) -> Expression {
    if diffs.len() == 1 {
        diffs.pop().unwrap()
    } else {
        Expression::Sequence(diffs)
    }
}

fn map_choice(mut seqs: Vec<Expression>) -> Expression {
    if seqs.len() == 1 {
        seqs.pop().unwrap()
    } else {
        Expression::Choice(seqs)
    }
}

fn map_difference((item1, item2): (Expression, Option<Expression>)) -> Expression {
    if let Some(item2) = item2 {
        Expression::Difference(Box::new(item1), Box::new(item2))
    } else {
        item1
    }
}

fn map_primary((h, t): (Expression, Option<char>)) -> Expression {
    match t {
        Some('?') => Expression::Repeat(RepeatCount::ZeroOrOne, Box::new(h)),
        Some('*') => Expression::Repeat(RepeatCount::ZeroOrMore, Box::new(h)),
        Some('+') => Expression::Repeat(RepeatCount::OneOrMore, Box::new(h)),
        _ => h,
    }
}

fn map_char_code_in_primary(c: char) -> Expression {
    Expression::Chars(c.to_string())
}

fn map_char_code(digits: Vec<char>) -> Result<char, ()> {
    let digits = digits.iter().collect::<String>();
    char::from_u32(u32::from_str_radix(digits.as_str(), 16).unwrap()).ok_or(())
}
