use crate::model::{Identifier, VersionSpecifier};
use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct EnumItem {
    pub name: Identifier,

    pub enabled: Option<VersionSpecifier>,

    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct EnumVariant {
    pub name: Identifier,

    pub enabled: Option<VersionSpecifier>,

    pub reference: Identifier,
}
