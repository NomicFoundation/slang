use std::rc::Rc;

use indexmap::IndexMap;

pub type Grammar = IndexMap<String, ExpressionRef>;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Expression {
    End {
        config: ChumskyConfig,
    },
    Any {
        config: ChumskyConfig,
    },
    Repeated {
        config: ChumskyConfig,
        expr: ExpressionRef,
    },
    Optional {
        config: ChumskyConfig,
        expr: ExpressionRef,
    },
    Negation {
        config: ChumskyConfig,
        expr: ExpressionRef,
    },
    Choice {
        config: ChumskyConfig,
        exprs: Vec<ExpressionRef>,
    },
    Sequence {
        config: ChumskyConfig,
        exprs: Vec<ExpressionRef>,
    },
    Difference {
        config: ChumskyConfig,
        minuend: ExpressionRef,
        subtrahend: ExpressionRef,
    },
    Chars {
        config: ChumskyConfig,
        string: String,
    },
    Identifier {
        config: ChumskyConfig,
        name: String,
    },
    CharRange {
        config: ChumskyConfig,
        start: char,
        end: char,
    },
}

pub type ExpressionRef = Rc<Expression>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChumskyConfig {
    pub ignore: bool,
    pub nomap: bool,
    pub map: Option<String>,
    pub unwrap: bool,
    pub chain: bool,
    pub lookahead: Option<ExpressionRef>,
}

impl Default for ChumskyConfig {
    fn default() -> Self {
        Self {
            ignore: false,
            nomap: false,
            map: None,
            unwrap: false,
            chain: false,
            lookahead: None,
        }
    }
}
