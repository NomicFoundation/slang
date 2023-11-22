use crate::model::{Identifier, Scanner, Spanned, VersionSpecifier};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use serde::Serialize;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct FragmentItem {
    pub name: Spanned<Identifier>,

    pub enabled: Option<Spanned<VersionSpecifier>>,

    pub scanner: Scanner,
}
