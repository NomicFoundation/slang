use crate::model::{Identifier, Spanned, VersionSpecifier};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use indexmap::IndexSet;
use serde::Serialize;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct FieldsErrorRecovery {
    pub terminator: Option<Spanned<Identifier>>,
    pub delimiters: Option<FieldDelimiters>,
}

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct FieldDelimiters {
    pub open: Spanned<Identifier>,
    pub close: Spanned<Identifier>,
}

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub enum Field {
    Required {
        kind: FieldKind,
    },
    Optional {
        kind: FieldKind,

        enabled: Option<Spanned<VersionSpecifier>>,
    },
}

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub enum FieldKind {
    NonTerminal {
        item: Spanned<Identifier>,
    },
    Terminal {
        items: IndexSet<Spanned<Identifier>>,
    },
}
