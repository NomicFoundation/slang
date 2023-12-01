use crate::model::{Identifier, VersionSpecifier};
use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct FieldsErrorRecovery {
    pub terminator: Option<Identifier>,
    pub delimiters: Option<FieldDelimiters>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct FieldDelimiters {
    pub open: Identifier,
    pub close: Identifier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub enum Field {
    Required {
        kind: FieldKind,
    },
    Optional {
        kind: FieldKind,

        enabled: Option<VersionSpecifier>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub enum FieldKind {
    NonTerminal { item: Identifier },
    Terminal { items: IndexSet<Identifier> },
}
