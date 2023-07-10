use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::Reference;

pub type ParserRef = std::rc::Rc<Parser>;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Parser {
    #[serde(flatten)]
    pub definition: ParserDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum ParserDefinition {
    #[schemars(title = "Choice Parser")]
    Choice(Vec<ParserRef>),

    #[schemars(title = "DelimitedBy Parser")]
    DelimitedBy {
        open: Reference,
        parser: ParserRef,
        close: Reference,
    },

    #[schemars(title = "OneOrMore Parser")]
    OneOrMore(ParserRef),

    #[schemars(title = "Optional Parser")]
    Optional(ParserRef),

    #[schemars(title = "Reference Parser")]
    Reference(String),

    #[schemars(title = "SeparatedBy Parser")]
    SeparatedBy {
        parser: ParserRef,
        separator: Reference,
    },

    #[schemars(title = "Sequence Parser")]
    Sequence(Vec<ParserRef>),

    #[schemars(title = "TerminatedBy Parser")]
    TerminatedBy {
        parser: ParserRef,
        terminator: Reference,
    },

    #[schemars(title = "ZeroOrMore Parser")]
    ZeroOrMore(ParserRef),
}
