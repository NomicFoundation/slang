use crate::model::{Identifier, Scanner, VersionSpecifier};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct FragmentItem {
    pub name: Identifier,

    pub enabled: Option<VersionSpecifier>,

    pub scanner: Scanner,
}
