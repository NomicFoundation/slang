use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, Scanner, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TokenItem {
    pub name: Identifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    /// Optional negative lookahead: rejects the token match if the remainder
    /// matches the given scanner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_followed_by: Option<Scanner>,

    pub scanner: Scanner,
}

impl TokenItem {
    pub fn is_unique(&self) -> bool {
        self.scanner.is_unique()
    }
}
