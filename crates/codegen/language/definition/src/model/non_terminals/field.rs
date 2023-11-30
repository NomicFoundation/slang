use crate::model::{Identifier, VersionSpecifier};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct FieldsErrorRecovery {
    pub terminator: Option<Identifier>,
    pub delimiters: Option<FieldDelimiters>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct FieldDelimiters {
    pub open: Identifier,
    pub close: Identifier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
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
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub enum FieldKind {
    NonTerminal { item: Identifier },
    Terminal { items: IndexSet<Identifier> },
}
