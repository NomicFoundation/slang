use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, VersionSpecifier};

/// A `Field` of a nonterminal that can be either required or optional.
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
