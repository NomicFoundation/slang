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
    #[schemars(title = "Choice Scanner")]
    Choice(Vec<ScannerRef>),

    #[schemars(title = "Difference Scanner")]
    Difference {
        minuend: ScannerRef,
        subtrahend: ScannerRef,
    },

    #[schemars(title = "Not Scanner")]
    Not(ScannerRef),

    #[schemars(title = "OneOrMore Scanner")]
    OneOrMore(ScannerRef),

    #[schemars(title = "Optional Scanner")]
    Optional(ScannerRef),

    #[schemars(title = "Range Scanner")]
    Range { from: char, to: char },

    #[schemars(title = "Reference Scanner")]
    Reference(String),

    #[schemars(title = "Sequence Scanner")]
    Sequence(Vec<ScannerRef>),

    #[schemars(title = "TrailingContext Scanner")]
    #[serde(rename_all = "camelCase")]
    TrailingContext {
        scanner: ScannerRef,
        not_followed_by: ScannerRef,
    },

    #[schemars(title = "Terminal Scanner")]
    Terminal(String),

    #[schemars(title = "ZeroOrMore Scanner")]
    ZeroOrMore(ScannerRef),
}
