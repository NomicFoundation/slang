use std::{collections::HashMap, rc::Rc};

use indexmap::IndexMap;

pub type GrammarParserResultType = IndexMap<String, ExpressionRef>;
pub type ExpressionParserResultType = ExpressionRef;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Expression {
    End {},
    Any {},
    Repeated {
        expr: ExpressionRef,
    },
    Optional {
        expr: ExpressionRef,
    },
    Negation {
        expr: ExpressionRef,
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
    CharRange {
        start: char,
        end: char,
    },
}

pub type ExpressionRef = Rc<Expression>;

pub fn map_grammar(productions: Vec<(String, ExpressionRef)>) -> GrammarParserResultType {
    productions.into_iter().collect()
}

pub fn map_eof(_: char) -> ExpressionRef {
    Rc::new(Expression::End {})
}

pub fn map_any(_: char) -> ExpressionRef {
    Rc::new(Expression::Any {})
}

pub fn map_string(chars: Vec<char>) -> ExpressionRef {
    Rc::new(Expression::Chars {
        string: chars.iter().collect::<String>(),
    })
}

pub fn map_char_range((start, end): (char, char)) -> ExpressionRef {
    Rc::new(Expression::CharRange { start, end })
}

pub fn map_identifier(chars: Vec<char>) -> String {
    chars.iter().collect()
}

pub fn map_sequence(mut diffs: Vec<ExpressionRef>) -> ExpressionRef {
    if diffs.len() == 1 {
        diffs.pop().unwrap()
    } else {
        Rc::new(Expression::Sequence { exprs: diffs })
    }
}

pub fn map_expression(mut seqs: Vec<ExpressionRef>) -> ExpressionRef {
    if seqs.len() == 1 {
        seqs.pop().unwrap()
    } else {
        Rc::new(Expression::Choice { exprs: seqs })
    }
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

pub fn map_negation((negation, expr): (Option<()>, ExpressionRef)) -> ExpressionRef {
    if negation.is_some() {
        Rc::new(Expression::Negation { expr })
    } else {
        expr
    }
}

pub fn map_optional(expr: ExpressionRef) -> ExpressionRef {
    Rc::new(Expression::Optional { expr })
}

pub fn map_repeated(expr: ExpressionRef) -> ExpressionRef {
    Rc::new(Expression::Repeated { expr })
}

pub fn map_production_reference(name: String) -> ExpressionRef {
    Rc::new(Expression::Identifier { name })
}

pub fn map_hex_digits_to_char(digits: Vec<char>) -> Result<char, ()> {
    let digits = digits.iter().collect::<String>();
    char::from_u32(u32::from_str_radix(digits.as_str(), 16).unwrap()).ok_or(())
}
