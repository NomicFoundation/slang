use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
        open: String,
        expression: ParserRef,
        close: String,
    },

    #[schemars(title = "OneOrMore Expression")]
    OneOrMore(ParserRef),

    #[schemars(title = "Optional Expression")]
    Optional(ParserRef),

    #[schemars(title = "Reference Expression")]
    Reference(String),

    #[schemars(title = "Repeat Expression")]
    Repeat {
        min: usize,
        max: usize,
        expression: ParserRef,
    },

    #[schemars(title = "SeparatedBy Expression")]
    SeparatedBy {
        separator: String,
        expression: ParserRef,
    },

    #[schemars(title = "Sequence Expression")]
    Sequence(Vec<ParserRef>),

    #[schemars(title = "Terminal Expression")]
    Terminal(String),

    #[schemars(title = "ZeroOrMore Expression")]
    ZeroOrMore(ParserRef),
}
