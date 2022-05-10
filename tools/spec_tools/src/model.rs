use std::rc::Rc;

use serde::Serialize;

pub type Grammar = Vec<Production>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Production {
    pub name: String,
    pub expr: ExpressionRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Expression {
    #[serde(default)]
    #[serde(skip_serializing_if = "ChumskyConfig::is_default")]
    pub config: ChumskyConfig,
    // #[serde(flatten)]
    pub ebnf: EBNF,
}

impl Expression {
    pub fn ref_from_ebnf(ebnf: EBNF) -> ExpressionRef {
        Rc::new(Self {
            config: Default::default(),
            ebnf,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[allow(dead_code)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum EBNF {
    End,
    Any,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ChumskyConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub ignore: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub nomap: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub unwrap: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub chain: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookahead: Option<ExpressionRef>,
}

impl ChumskyConfig {
    pub fn is_default(&self) -> bool {
        !self.ignore
            && !self.nomap
            && self.map.is_none()
            && !self.unwrap
            && !self.chain
            && self.lookahead.is_none()
    }
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
