use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type ScannerRef = std::rc::Rc<Scanner>;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Scanner {
    #[serde(flatten)]
    pub definition: ScannerDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum ScannerDefinition {
    #[schemars(title = "Choice Expression")]
    Choice(Vec<ScannerRef>),

    #[schemars(title = "Difference Expression")]
    Difference {
        minuend: ScannerRef,
        subtrahend: ScannerRef,
    },

    #[schemars(title = "Not Expression")]
    Not(ScannerRef),

    #[schemars(title = "OneOrMore Expression")]
    OneOrMore(ScannerRef),

    #[schemars(title = "Optional Expression")]
    Optional(ScannerRef),

    #[schemars(title = "Range Expression")]
    Range { from: char, to: char },

    #[schemars(title = "Reference Expression")]
    Reference(String),

    #[schemars(title = "Repeat Expression")]
    Repeat {
        expression: ScannerRef,
        min: usize,
        max: usize,
    },

    #[schemars(title = "Sequence Expression")]
    Sequence(Vec<ScannerRef>),

    #[schemars(title = "TrailingContext Expression")]
    #[serde(rename_all = "camelCase")]
    TrailingContext {
        expression: ScannerRef,
        not_followed_by: ScannerRef,
    },

    #[schemars(title = "Terminal Expression")]
    Terminal(String),

    #[schemars(title = "ZeroOrMore Expression")]
    ZeroOrMore(ScannerRef),
}

impl ScannerDefinition {
    pub fn produces_epsilon(&self) -> bool {
        match self{
            Self::Not(_)
            | Self::Range { .. }
            | Self::Reference(_) // TODO: check if the referenced expression produces epsilon
            | Self::Terminal(_) => false,

            Self::Optional(_)
            | Self::ZeroOrMore(_)
            | Self::Repeat { min: 0, .. } => true,

            Self::Repeat { expression, .. } // min > 0
            | Self::OneOrMore(expression)
            | Self::Difference {
                minuend: expression,
                ..
            }
            | Self::TrailingContext { expression, .. } => {
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
