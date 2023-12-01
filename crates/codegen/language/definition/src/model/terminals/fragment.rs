use crate::model::{Identifier, Scanner, VersionSpecifier};
use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct FragmentItem {
    pub name: Identifier,

    pub enabled: Option<VersionSpecifier>,

    pub scanner: Scanner,
}
