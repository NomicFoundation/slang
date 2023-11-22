use crate::model::{Identifier, Spanned, VersionSpecifier};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use serde::Serialize;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct EnumItem {
    pub name: Spanned<Identifier>,

    pub enabled: Option<Spanned<VersionSpecifier>>,

    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct EnumVariant {
    pub name: Spanned<Identifier>,

    pub enabled: Option<Spanned<VersionSpecifier>>,

    pub reference: Spanned<Identifier>,
}
