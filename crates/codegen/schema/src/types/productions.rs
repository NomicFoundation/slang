use std::rc::Rc;

use indexmap::IndexMap;
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

pub type ProductionRef = Rc<Production>;

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Production {
    pub name: String,
    pub kind: ProductionKind,
    #[serde(flatten)]
    pub versioning: ProductionVersioning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub enum ProductionKind {
    Rule,
    Trivia,
    Token,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum ProductionVersioning {
    Unversioned(ExpressionRef),
    Versioned(IndexMap<Version, ExpressionRef>),
}

pub type ExpressionRef = Rc<Expression>;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Expression {
    #[serde(default, flatten)]
    pub config: ExpressionConfig,
    #[serde(flatten)]
    pub parser: ExpressionParser,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ExpressionConfig {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub parser_type: Option<ParserType>,
    #[serde(default)]
    pub lookahead: Option<ExpressionRef>,
    #[serde(default)]
    pub associativity: Option<ExpressionAssociativity>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Hash)]
#[serde(deny_unknown_fields)]
pub enum ParserType {
    Precedence,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Hash)]
#[serde(deny_unknown_fields)]
pub enum ExpressionAssociativity {
    Left,
    Right,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum ExpressionParser {
    #[schemars(title = "Choice Expression")]
    Choice(Vec<ExpressionRef>),

    #[schemars(title = "DelimitedBy Expression")]
    DelimitedBy {
        open: String,
        expression: ExpressionRef,
        close: String,
    },

    #[schemars(title = "Difference Expression")]
    Difference {
        minuend: ExpressionRef,
        subtrahend: ExpressionRef,
    },

    #[schemars(title = "Not Expression")]
    Not(ExpressionRef),

    #[schemars(title = "OneOrMore Expression")]
    OneOrMore(ExpressionRef),

    #[schemars(title = "Optional Expression")]
    Optional(ExpressionRef),

    #[schemars(title = "Range Expression")]
    Range { from: char, to: char },

    #[schemars(title = "Reference Expression")]
    Reference(String),

    #[schemars(title = "Repeat Expression")]
    Repeat {
        min: usize,
        max: usize,
        expression: ExpressionRef,
    },

    #[schemars(title = "SeparatedBy Expression")]
    SeparatedBy {
        separator: String,
        expression: ExpressionRef,
    },
    #[schemars(title = "Sequence Expression")]
    Sequence(Vec<ExpressionRef>),

    #[schemars(title = "Terminal Expression")]
    Terminal(String),

    #[schemars(title = "ZeroOrMore Expression")]
    ZeroOrMore(ExpressionRef),
}
