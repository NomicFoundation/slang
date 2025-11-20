use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, Scanner, VersionSpecifier};

/// Fragments are scanner definitions that are inlined into the lexer and parser.
///
/// They don't produce any tokens, but they are used in other tokens for commong scanners.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct FragmentItem {
    pub name: Identifier,

    /// Although fragments don't produce tokens, this allows us to check
    /// fragments are actually referenced within the language definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    pub scanner: Scanner,
}
