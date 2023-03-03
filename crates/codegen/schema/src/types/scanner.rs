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

    #[schemars(title = "DelimitedBy Expression")]
    DelimitedBy {
        open: String,
        expression: ScannerRef,
        close: String,
    },

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
        min: usize,
        max: usize,
        expression: ScannerRef,
    },

    #[schemars(title = "SeparatedBy Expression")]
    SeparatedBy {
        separator: String,
        expression: ScannerRef,
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
