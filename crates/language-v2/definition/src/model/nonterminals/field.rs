use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, VersionSpecifier};

/// Error recovery for fields, this is used by the parser to recover in case of missing or unexpected tokens.
///
/// Note: Some nonterminals have both a terminator and delimiters, ie `DoWhileStatement`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct FieldsErrorRecovery {
    /// Error recovery happens at a terminator.
    ///
    /// For example `PragmaDirective` has a `Semicolon` terminator, that could
    /// be used to recover from an wrong pragma directive like
    /// `padme solidity ^0.8.0;`
    ///  ~~~~~ -> This should be `pragma`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminator: Option<Identifier>,

    /// Error recovery happens at delimiters.
    ///
    /// For example `TupleExpression` has a `OpenParen` and `CloseParen` delimiters,
    /// that could be used to recover from wrong tuple like
    /// `(pragma, bar)`
    ///   ~~~~~ -> This is not a valid expression
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminals_matched_acceptance_threshold: Option<u8>,
}

/// A `Field` is a field of a nonterminal that can be either required or optional.
///
/// Note: `Optional` fields are versioned, `Required` fields are not.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
#[serde(tag = "type")]
pub enum Field {
    Required {
        reference: Identifier,
    },
    Optional {
        reference: Identifier,

        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<VersionSpecifier>,
    },
}
