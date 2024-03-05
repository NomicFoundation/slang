use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct SeparatedItem {
    pub name: Identifier,
    pub reference: Identifier,
    pub separator: Identifier,

    pub enabled: Option<VersionSpecifier>,

    pub allow_empty: Option<bool>,
}
