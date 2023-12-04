use crate::model::{Identifier, VersionSpecifier};
use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct SeparatedItem {
    pub name: Identifier,
    pub separated: Identifier,
    pub separator: Identifier,

    pub enabled: Option<VersionSpecifier>,
}
