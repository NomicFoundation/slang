pub type TreeType = Vec<Production>;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Production {
    pub name: String,
    pub expr: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Expression {
    End,
    Any,
    Repeat(Box<Expression>, RepeatCount),
    Choice(Vec<Expression>),
    Sequence(Vec<Expression>),
    DelimitedBy(Box<Expression>, char, char),
    PaddedBy(Box<Expression>, Box<Expression>),
    SeparatedBy(Box<Expression>, Box<Expression>),
    Difference(Box<Expression>, Box<Expression>),
    Chars(String),
    Identifier(String),
    CharSet(Vec<CharSetElement>, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepeatCount {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharSetElement {
    Char(char),
    Range(char, char),
}

pub fn map_char_set((neg, chars): (Option<char>, Vec<(char, Option<char>)>)) -> Expression {
    Expression::CharSet(
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
        neg.is_some(),
    )
}

pub fn map_string(chars: Vec<char>) -> Expression {
    Expression::Chars(chars.iter().collect::<String>())
}

pub fn map_production((name, expr): (String, Expression)) -> Production {
    Production { name, expr }
}

pub fn map_sequence(mut diffs: Vec<Expression>) -> Expression {
    if diffs.len() == 1 {
        diffs.pop().unwrap()
    } else {
        Expression::Sequence(diffs)
    }
}

pub fn map_choice(mut seqs: Vec<Expression>) -> Expression {
    if seqs.len() == 1 {
        seqs.pop().unwrap()
    } else {
        Expression::Choice(seqs)
    }
}

pub fn map_difference((item1, item2): (Expression, Option<Expression>)) -> Expression {
    if let Some(item2) = item2 {
        Expression::Difference(Box::new(item1), Box::new(item2))
    } else {
        item1
    }
}

pub fn map_item((h, t): (Expression, Option<char>)) -> Expression {
    match t {
        Some('?') => Expression::Repeat(Box::new(h), RepeatCount::ZeroOrOne),
        Some('*') => Expression::Repeat(Box::new(h), RepeatCount::ZeroOrMore),
        Some('+') => Expression::Repeat(Box::new(h), RepeatCount::OneOrMore),
        _ => h,
    }
}

pub fn map_identifier_in_primary(s: String) -> Expression {
    Expression::Identifier(s)
}

pub fn map_char_code_in_primary(c: char) -> Expression {
    Expression::Chars(c.to_string())
}

pub fn map_char_code(digits: Vec<char>) -> Result<char, ()> {
    let digits = digits.iter().collect::<String>();
    char::from_u32(u32::from_str_radix(digits.as_str(), 16).unwrap()).ok_or(())
}
