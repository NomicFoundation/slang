use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::Reference;

pub type ParserRef = std::rc::Rc<Parser>;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Parser {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten)]
    pub definition: ParserDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum ParserDefinition {
    #[schemars(title = "Choice Expression")]
    Choice(Vec<ParserRef>),

    #[schemars(title = "DelimitedBy Expression")]
    DelimitedBy {
        open: Reference,
        expression: ParserRef,
        close: Reference,
    },

    #[schemars(title = "OneOrMore Expression")]
    OneOrMore(ParserRef),

    #[schemars(title = "Optional Expression")]
    Optional(ParserRef),

    #[schemars(title = "Reference Expression")]
    Reference(String),

    #[schemars(title = "SeparatedBy Expression")]
    SeparatedBy {
        expression: ParserRef,
        separator: Reference,
    },

    #[schemars(title = "Sequence Expression")]
    Sequence(Vec<ParserRef>),

    #[schemars(title = "TerminatedBy Expression")]
    TerminatedBy {
        expression: ParserRef,
        terminator: Reference,
    },

    #[schemars(title = "ZeroOrMore Expression")]
    ZeroOrMore(ParserRef),
}
