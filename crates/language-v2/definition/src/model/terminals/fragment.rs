use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, Scanner, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct FragmentItem {
    pub name: Identifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    pub scanner: Scanner,
}
