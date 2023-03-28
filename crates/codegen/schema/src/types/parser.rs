use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::production::Reference;

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

    #[schemars(title = "Repeat Expression")]
    Repeat {
        expression: ParserRef,
        min: usize,
        max: usize,
    },

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

impl ParserDefinition {
    pub fn produces_epsilon(&self) -> bool {
        match self {
            Self::TerminatedBy { .. } // TODO: check if the expression && referenced expression both produce epsilon
            | Self::DelimitedBy {..} // TODO: check if the referenced delimiters && expression all produce epsilon
            | Self::Reference(_) => false, // TODO: check if the referenced expression produces epsilon

            Self::Optional(_)
            | Self::ZeroOrMore(_)
            | Self::Repeat { min: 0, .. } => true,

            Self::SeparatedBy { expression, .. } 
            | Self::Repeat { expression, .. } // min > 0
            | Self::OneOrMore(expression) => {
                expression.definition.produces_epsilon()
            }

            Self::Choice(expressions) => {
                expressions.iter().any(|e| e.definition.produces_epsilon())
            }

            Self::Sequence(expressions) => {
                expressions.iter().all(|e| e.definition.produces_epsilon())
            }
        }
    }
}
