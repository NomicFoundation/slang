use std::{collections::HashMap, rc::Rc};

pub type GrammarParserResultType = HashMap<String, ExpressionRef>;
pub type ExpressionParserResultType = ExpressionRef;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Expression {
    End {},
    Any {},
    Repeat {
        expr: ExpressionRef,
        count: RepeatCount,
    },
    Choice {
        exprs: Vec<ExpressionRef>,
    },
    Sequence {
        exprs: Vec<ExpressionRef>,
    },
    Difference {
        minuend: ExpressionRef,
        subtrahend: ExpressionRef,
    },
    Chars {
        string: String,
    },
    Identifier {
        name: String,
    },
    CharSet {
        elements: Vec<CharSetElement>,
        negated: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum RepeatCount {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum CharSetElement {
    Char(char),
    Range(char, char),
}

pub type ExpressionRef = Rc<Expression>;

pub fn map_grammar(productions: Vec<(String, ExpressionRef)>) -> GrammarParserResultType {
    productions.into_iter().collect()
}

pub fn map_to_eof(_: char) -> ExpressionRef {
    Rc::new(Expression::End {})
}

pub fn map_to_any(_: char) -> ExpressionRef {
    Rc::new(Expression::Any {})
}

pub fn map_char_set((neg, chars): (Option<char>, Vec<(char, Option<char>)>)) -> ExpressionRef {
    Rc::new(Expression::CharSet {
        elements: chars
            .into_iter()
            .map(|(c1, c2)| {
                if let Some(c2) = c2 {
                    CharSetElement::Range(c1, c2)
                } else {
                    CharSetElement::Char(c1)
                }
            })
            .collect(),
        negated: neg.is_some(),
    })
}

pub fn map_string(chars: Vec<char>) -> ExpressionRef {
    Rc::new(Expression::Chars {
        string: chars.iter().collect::<String>(),
    })
}

pub fn map_sequence((head, tail): (ExpressionRef, Vec<ExpressionRef>)) -> ExpressionRef {
    // if diffs.len() == 1 {
    //     diffs.pop().unwrap()
    // } else {
    //     Rc::new(Expression::Sequence { exprs: diffs })
    // }
    Rc::new(Expression::Any {})
}

pub fn map_expression((head, tail): (ExpressionRef, Vec<ExpressionRef>)) -> ExpressionRef {
    // if seqs.len() == 1 {
    //     seqs.pop().unwrap()
    // } else {
    //     Rc::new(Expression::Choice { exprs: seqs })
    // }
    Rc::new(Expression::Any {})
}

pub fn map_difference((item1, item2): (ExpressionRef, Option<ExpressionRef>)) -> ExpressionRef {
    if let Some(item2) = item2 {
        Rc::new(Expression::Difference {
            minuend: item1,
            subtrahend: item2,
        })
    } else {
        item1
    }
}

pub fn map_item((h, t): (ExpressionRef, Option<char>)) -> ExpressionRef {
    match t {
        Some('?') => Rc::new(Expression::Repeat {
            expr: h,
            count: RepeatCount::ZeroOrOne,
        }),
        Some('*') => Rc::new(Expression::Repeat {
            expr: h,
            count: RepeatCount::ZeroOrMore,
        }),
        Some('+') => Rc::new(Expression::Repeat {
            expr: h,
            count: RepeatCount::OneOrMore,
        }),
        _ => h,
    }
}

pub fn map_identifier_in_primary((head, tail): (char, Vec<char>)) -> ExpressionRef {
    Rc::new(Expression::Identifier { name: "".into() })
}

pub fn map_char_code_in_primary(c: char) -> ExpressionRef {
    Rc::new(Expression::Chars {
        string: c.to_string(),
    })
}

pub fn map_char_code(digits: Vec<char>) -> Result<char, ()> {
    let digits = digits.iter().collect::<String>();
    char::from_u32(u32::from_str_radix(digits.as_str(), 16).unwrap()).ok_or(())
}
