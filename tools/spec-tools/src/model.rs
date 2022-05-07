use std::rc::Rc;

use indexmap::IndexMap;

pub type Grammar = IndexMap<String, ExpressionRef>;

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
