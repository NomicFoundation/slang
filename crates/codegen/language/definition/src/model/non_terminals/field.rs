use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct FieldsErrorRecovery {
    pub terminator: Option<Identifier>,
    pub delimiters: Option<FieldDelimiters>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct FieldDelimiters {
    pub open: Identifier,
    pub close: Identifier,
    /// How many tokens have to be matched to trigger the error recovery.
    /// For ambiguous syntaxes this needs to be set to at least N, where N
    /// is the token lookahead required to disambiguate the syntax.
    ///
    /// By default, we assume no lookahead (0) is required to recover from
    /// unrecognized body between delimiters, so it's always triggered.
    pub tokens_matched_acceptance_threshold: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub enum Field {
    Required {
        reference: Identifier,
    },
    Optional {
        reference: Identifier,

        enabled: Option<VersionSpecifier>,
    },
}
