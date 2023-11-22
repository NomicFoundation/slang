use crate::model::{Identifier, Spanned, VersionSpecifier};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use serde::Serialize;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct RepeatedItem {
    pub name: Spanned<Identifier>,
    pub repeated: Spanned<Identifier>,

    pub enabled: Option<Spanned<VersionSpecifier>>,

    pub allow_empty: Option<Spanned<bool>>,
}
